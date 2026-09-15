//! module-paint — workspace PAINT (P3D-055 a P3D-062, P3D-132).
//!
//! Motor de pintura 2D e 3D por projeção UV:
//! - Pincéis: Pixel Brush rígido (P3D-056), Soft Brush com atenuação suave (P3D-057),
//!   Borracha (P3D-058), Flood Fill (P3D-059), Conta-gotas / Eyedropper (P3D-060).
//! - Camadas de pintura e modos de mesclagem (P3D-061).
//! - Pintura direta sobre malha 3D via coordenadas baricêntricas e UV (P3D-062).
//! - Isolamento de seleção / Paint Masks (P3D-132).
//! - Sincronização direta com o modelo canônico de Material (P3D-050).

use std::collections::VecDeque;

use glam::Vec3;
use petunia_core::{AppState, Module};
use petunia_project::Canvas;
use serde::{Deserialize, Serialize};

/// Tipo de pincel ativo no motor de pintura (P3D-056 a P3D-060).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BrushType {
    /// Pincel rígido com pixels exatos sem anti-aliasing (P3D-056).
    #[default]
    Pixel,
    /// Pincel com atenuação radial suave (P3D-057).
    Soft,
    /// Borracha que atenua ou remove o canal alfa (P3D-058).
    Eraser,
    /// Balde de preenchimento flood-fill por tolerância (P3D-059).
    Fill,
    /// Amostrador de cor / conta-gotas (P3D-060).
    Eyedropper,
}

/// Modo de mesclagem de camadas de pintura (P3D-061).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LayerBlendMode {
    #[default]
    Normal,
    Multiply,
    Add,
    Screen,
}

/// Efeitos não-destrutivos sobre texturas / camadas (P3D-134).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum PaintEffect {
    /// Pixelização com tamanho de bloco especificado (P3D-134).
    Pixelate { cell_size: u32 },
    /// Quantização / posterização de tons por canal de cor (P3D-134).
    Posterize { levels: u8 },
    /// Inversão de cores RGB.
    Invert,
}

/// Decalque / projeção 2D parametrizada e reposicionável (P3D-133).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DecalLayer {
    pub image: Canvas,
    /// Centro da projeção no espaço UV [0.0..1.0]
    pub center_uv: [f32; 2],
    /// Escala relativa da estampa no espaço UV [0.0..1.0]
    pub scale_uv: [f32; 2],
    /// Rotação do decalque em radianos
    pub rotation_rad: f32,
}

impl DecalLayer {
    pub fn new(image: Canvas, center_uv: [f32; 2], scale_uv: [f32; 2], rotation_rad: f32) -> Self {
        Self {
            image,
            center_uv,
            scale_uv,
            rotation_rad,
        }
    }
}

/// Conteúdo específico da camada (Raster, Decal ou Efeito).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LayerKind {
    /// Camada de pintura raster comum com canvas próprio (P3D-061).
    Raster(Canvas),
    /// Camada de decalque / estampa projetada sobre o UV (P3D-133).
    Decal(DecalLayer),
    /// Camada de efeito não-destrutivo aplicada sobre a composição inferior (P3D-134).
    Effect(PaintEffect),
}

/// Camada de pintura unificada com suporte a raster, decalques e efeitos (P3D-061, P3D-133, P3D-134).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaintLayer {
    pub id: uuid::Uuid,
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
    pub blend: LayerBlendMode,
    pub kind: LayerKind,
}

impl PaintLayer {
    pub fn new(name: impl Into<String>, w: u32, h: u32, fill: [u8; 4]) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: name.into(),
            visible: true,
            opacity: 1.0,
            blend: LayerBlendMode::Normal,
            kind: LayerKind::Raster(Canvas::new(w, h, fill)),
        }
    }

    pub fn new_raster(name: impl Into<String>, canvas: Canvas) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: name.into(),
            visible: true,
            opacity: 1.0,
            blend: LayerBlendMode::Normal,
            kind: LayerKind::Raster(canvas),
        }
    }

    pub fn new_decal(name: impl Into<String>, decal: DecalLayer) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: name.into(),
            visible: true,
            opacity: 1.0,
            blend: LayerBlendMode::Normal,
            kind: LayerKind::Decal(decal),
        }
    }

    pub fn new_effect(name: impl Into<String>, effect: PaintEffect) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: name.into(),
            visible: true,
            opacity: 1.0,
            blend: LayerBlendMode::Normal,
            kind: LayerKind::Effect(effect),
        }
    }

    pub fn canvas(&self) -> Option<&Canvas> {
        match &self.kind {
            LayerKind::Raster(c) => Some(c),
            LayerKind::Decal(d) => Some(&d.image),
            LayerKind::Effect(_) => None,
        }
    }

    pub fn canvas_mut(&mut self) -> Option<&mut Canvas> {
        match &mut self.kind {
            LayerKind::Raster(c) => Some(c),
            LayerKind::Decal(d) => Some(&mut d.image),
            LayerKind::Effect(_) => None,
        }
    }
}

/// Mistura dois pixels com modo de mesclagem e opacidade.
pub fn blend_pixels(dst: [u8; 4], src: [u8; 4], opacity: f32, mode: LayerBlendMode) -> [u8; 4] {
    let alpha = (src[3] as f32 / 255.0) * opacity.clamp(0.0, 1.0);
    if alpha <= 0.0 {
        return dst;
    }

    let (sr, sg, sb) = (src[0] as f32, src[1] as f32, src[2] as f32);
    let (dr, dg, db) = (dst[0] as f32, dst[1] as f32, dst[2] as f32);

    let (mr, mg, mb) = match mode {
        LayerBlendMode::Normal => (sr, sg, sb),
        LayerBlendMode::Multiply => (sr * dr / 255.0, sg * dg / 255.0, sb * db / 255.0),
        LayerBlendMode::Add => (
            (sr + dr).min(255.0),
            (sg + dg).min(255.0),
            (sb + db).min(255.0),
        ),
        LayerBlendMode::Screen => (
            255.0 - ((255.0 - sr) * (255.0 - dr) / 255.0),
            255.0 - ((255.0 - sg) * (255.0 - dg) / 255.0),
            255.0 - ((255.0 - sb) * (255.0 - db) / 255.0),
        ),
    };

    let out_r = (dr * (1.0 - alpha) + mr * alpha).round().clamp(0.0, 255.0) as u8;
    let out_g = (dg * (1.0 - alpha) + mg * alpha).round().clamp(0.0, 255.0) as u8;
    let out_b = (db * (1.0 - alpha) + mb * alpha).round().clamp(0.0, 255.0) as u8;
    let out_a = (dst[3] as f32 * (1.0 - alpha) + src[3] as f32 * alpha)
        .round()
        .clamp(0.0, 255.0) as u8;

    [out_r, out_g, out_b, out_a]
}

/// Pilha unificada de camadas de pintura, decalques e efeitos (P3D-061, P3D-133, P3D-134).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PaintLayerStack {
    pub layers: Vec<PaintLayer>,
    pub active_layer: usize,
}

impl PaintLayerStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_layer(&mut self, layer: PaintLayer) -> uuid::Uuid {
        let id = layer.id;
        self.layers.push(layer);
        self.active_layer = self.layers.len() - 1;
        id
    }

    pub fn remove_layer(&mut self, id: uuid::Uuid) -> bool {
        if let Some(pos) = self.layers.iter().position(|l| l.id == id) {
            self.layers.remove(pos);
            if self.active_layer >= self.layers.len() && !self.layers.is_empty() {
                self.active_layer = self.layers.len() - 1;
            }
            true
        } else {
            false
        }
    }

    pub fn move_layer(&mut self, from: usize, to: usize) -> bool {
        if from < self.layers.len() && to < self.layers.len() && from != to {
            let l = self.layers.remove(from);
            self.layers.insert(to, l);
            self.active_layer = to;
            true
        } else {
            false
        }
    }

    /// Executa a composição determinística de todas as camadas sobre o canvas base.
    pub fn composite(&self, base: &mut Canvas) {
        for layer in &self.layers {
            if !layer.visible || layer.opacity <= 0.0 {
                continue;
            }

            match &layer.kind {
                LayerKind::Raster(canvas) => {
                    let w = base.w.min(canvas.w);
                    let h = base.h.min(canvas.h);
                    for y in 0..h {
                        for x in 0..w {
                            if let (Some(dst), Some(src)) = (base.get(x, y), canvas.get(x, y)) {
                                let blended = blend_pixels(dst, src, layer.opacity, layer.blend);
                                base.set(x, y, blended);
                            }
                        }
                    }
                }
                LayerKind::Decal(decal) => {
                    if decal.scale_uv[0].abs() < 1e-5 || decal.scale_uv[1].abs() < 1e-5 {
                        continue;
                    }
                    let w = base.w;
                    let h = base.h;
                    let cos_rot = (-decal.rotation_rad).cos();
                    let sin_rot = (-decal.rotation_rad).sin();

                    for y in 0..h {
                        for x in 0..w {
                            let u = (x as f32 + 0.5) / w as f32;
                            let v = (y as f32 + 0.5) / h as f32;

                            let dx = u - decal.center_uv[0];
                            let dy = v - decal.center_uv[1];

                            let rx = dx * cos_rot - dy * sin_rot;
                            let ry = dx * sin_rot + dy * cos_rot;

                            let decal_u = rx / decal.scale_uv[0] + 0.5;
                            let decal_v = ry / decal.scale_uv[1] + 0.5;

                            if (0.0..=1.0).contains(&decal_u) && (0.0..=1.0).contains(&decal_v) {
                                let sx = (decal_u * decal.image.w as f32)
                                    .clamp(0.0, decal.image.w as f32 - 1.0)
                                    as u32;
                                let sy = (decal_v * decal.image.h as f32)
                                    .clamp(0.0, decal.image.h as f32 - 1.0)
                                    as u32;

                                if let (Some(dst), Some(src)) =
                                    (base.get(x, y), decal.image.get(sx, sy))
                                {
                                    let blended =
                                        blend_pixels(dst, src, layer.opacity, layer.blend);
                                    base.set(x, y, blended);
                                }
                            }
                        }
                    }
                }
                LayerKind::Effect(effect) => {
                    let w = base.w;
                    let h = base.h;
                    match effect {
                        PaintEffect::Pixelate { cell_size } => {
                            let step = (*cell_size).max(1);
                            for y_block in (0..h).step_by(step as usize) {
                                for x_block in (0..w).step_by(step as usize) {
                                    if let Some(sample) = base.get(x_block, y_block) {
                                        for dy in 0..step {
                                            for dx in 0..step {
                                                let px = x_block + dx;
                                                let py = y_block + dy;
                                                if px < w
                                                    && py < h
                                                    && let Some(current) = base.get(px, py)
                                                {
                                                    let blended = blend_pixels(
                                                        current,
                                                        sample,
                                                        layer.opacity,
                                                        LayerBlendMode::Normal,
                                                    );
                                                    base.set(px, py, blended);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        PaintEffect::Posterize { levels } => {
                            let n = (*levels).max(2) as f32;
                            let step = 255.0 / (n - 1.0);
                            for y in 0..h {
                                for x in 0..w {
                                    if let Some(c) = base.get(x, y) {
                                        let pr = (((c[0] as f32 / 255.0 * (n - 1.0)).round())
                                            * step)
                                            .clamp(0.0, 255.0)
                                            as u8;
                                        let pg = (((c[1] as f32 / 255.0 * (n - 1.0)).round())
                                            * step)
                                            .clamp(0.0, 255.0)
                                            as u8;
                                        let pb = (((c[2] as f32 / 255.0 * (n - 1.0)).round())
                                            * step)
                                            .clamp(0.0, 255.0)
                                            as u8;
                                        let quant = [pr, pg, pb, c[3]];
                                        let blended = blend_pixels(
                                            c,
                                            quant,
                                            layer.opacity,
                                            LayerBlendMode::Normal,
                                        );
                                        base.set(x, y, blended);
                                    }
                                }
                            }
                        }
                        PaintEffect::Invert => {
                            for y in 0..h {
                                for x in 0..w {
                                    if let Some(c) = base.get(x, y) {
                                        let inv = [255 - c[0], 255 - c[1], 255 - c[2], c[3]];
                                        let blended = blend_pixels(
                                            c,
                                            inv,
                                            layer.opacity,
                                            LayerBlendMode::Normal,
                                        );
                                        base.set(x, y, blended);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Default)]
pub struct PaintModule;

impl PaintModule {
    pub fn new() -> Self {
        Self
    }

    /// Preenche a seleção de vértices (ou toda a malha se nada estiver selecionado) com a cor atual.
    pub fn fill_selection(state: &mut AppState) -> usize {
        let before = state.project.clone();
        let col = state.paint_color;
        let mut n = 0;
        if let Some(m) = state.project.active_mesh_mut() {
            let any = m.verts.iter().any(|v| v.selected);
            for v in &mut m.verts {
                if v.selected || !any {
                    v.color = col;
                    n += 1;
                }
            }
        }
        if n > 0 {
            state.project.undo.checkpoint("fill", &before);
            state.emit_mesh_changed();
        }
        n
    }

    /// Eyedropper: copia a cor do vértice para o pincel.
    pub fn eyedrop_vertex(state: &mut AppState, vi: usize) {
        if let Some(o) = state.project.assets.get(state.project.active)
            && let Some(v) = o.mesh.verts.get(vi)
        {
            state.paint_color = v.color;
            state.mark_dirty();
        }
    }

    /// Registra cor na palette (recentes, máx 16) no ProjectState.
    pub fn push_palette(state: &mut AppState, c: [f32; 3]) {
        state
            .project
            .palette
            .retain(|&x| (x[0] - c[0]).abs() + (x[1] - c[1]).abs() + (x[2] - c[2]).abs() > 1e-3);
        state.project.palette.insert(0, c);
        state.project.palette.truncate(16);
    }

    /// Substitui a paleta atual no ProjectState.
    pub fn set_palette(state: &mut AppState, pal: Vec<[f32; 3]>) {
        state.project.palette = pal;
        state.mark_dirty();
    }

    /// Importa paleta a partir de arquivo (.hex ou .gpl) usando o ProjectService.
    pub fn import_palette_file(
        state: &mut AppState,
        path: &std::path::Path,
    ) -> Result<usize, petunia_core::ProjectServiceError> {
        petunia_core::ProjectService::import_palette(state, path)
    }

    /// Exporta a paleta atual para arquivo (.gpl) usando o ProjectService.
    pub fn export_palette_file(
        state: &AppState,
        path: &std::path::Path,
    ) -> Result<(), petunia_core::ProjectServiceError> {
        petunia_core::ProjectService::export_palette(
            &state.project.palette,
            "Petunia Palette",
            path,
        )
    }

    // ---- Canvas 2D & Material Texture Synchronization (P3D-050, P3D-055) ----

    pub fn has_canvas(state: &AppState) -> bool {
        if let Some(o) = state.project.assets.get(state.project.active) {
            if o.texture.is_some() {
                return true;
            }
            if let Some(mat) = o.material(&state.project.project) {
                return mat.albedo_texture.is_some();
            }
        }
        false
    }

    /// Garante que o canvas existe no asset e no material associado.
    pub fn ensure_canvas(state: &mut AppState) {
        let active_idx = state.project.active;
        let mut base_col = [0.75, 0.75, 0.78];
        let mut mat_id = None;

        if let Some(o) = state.project.assets.get(active_idx) {
            base_col = o.base_color;
            mat_id = o.material_id;
        }

        let fill_rgba = [
            (base_col[0] * 255.0) as u8,
            (base_col[1] * 255.0) as u8,
            (base_col[2] * 255.0) as u8,
            255,
        ];

        // Sincroniza no Material associado se houver
        if let Some(mid) = mat_id
            && let Some(mat) = state.project.project.get_material_mut(mid)
            && mat.albedo_texture.is_none()
        {
            mat.albedo_texture = Some(Canvas::new(256, 256, fill_rgba));
        }

        // Sincroniza no Asset
        if let Some(o) = state.project.active_mut()
            && o.texture.is_none()
        {
            o.texture = Some(Canvas::new(256, 256, fill_rgba));
            state.mark_dirty();
        }
    }

    /// Aplica carimbo do pincel rígido pixel-perfect (P3D-056).
    pub fn stamp_pixel_brush(canvas: &mut Canvas, cx: u32, cy: u32, radius: u32, color: [u8; 4]) {
        let r = radius as i32;
        let r2 = r * r;
        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy <= r2 {
                    let px = (cx as i32 + dx).max(0) as u32;
                    let py = (cy as i32 + dy).max(0) as u32;
                    canvas.set(px, py, color);
                }
            }
        }
    }

    /// Aplica carimbo do pincel suave com atenuação quadrática (P3D-057).
    pub fn stamp_soft_brush(
        canvas: &mut Canvas,
        cx: u32,
        cy: u32,
        radius: u32,
        color: [u8; 4],
        strength: f32,
    ) {
        let r = radius as i32;
        let r_f = radius as f32;
        let str_k = strength.clamp(0.0, 1.0);

        for dy in -r..=r {
            for dx in -r..=r {
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                if dist <= r_f {
                    let factor = (1.0 - (dist / r_f)).powi(2) * str_k;
                    let px = (cx as i32 + dx).max(0) as u32;
                    let py = (cy as i32 + dy).max(0) as u32;

                    if let Some(bg) = canvas.get(px, py) {
                        let blended = [
                            (bg[0] as f32 * (1.0 - factor) + color[0] as f32 * factor) as u8,
                            (bg[1] as f32 * (1.0 - factor) + color[1] as f32 * factor) as u8,
                            (bg[2] as f32 * (1.0 - factor) + color[2] as f32 * factor) as u8,
                            (bg[3] as f32 * (1.0 - factor) + color[3] as f32 * factor) as u8,
                        ];
                        canvas.set(px, py, blended);
                    }
                }
            }
        }
    }

    /// Aplica carimbo de borracha atenuando ou removendo o alfa (P3D-058).
    pub fn stamp_eraser(canvas: &mut Canvas, cx: u32, cy: u32, radius: u32, strength: f32) {
        let r = radius as i32;
        let r_f = radius as f32;
        let str_k = strength.clamp(0.0, 1.0);

        for dy in -r..=r {
            for dx in -r..=r {
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                if dist <= r_f {
                    let factor = (1.0 - (dist / r_f)) * str_k;
                    let px = (cx as i32 + dx).max(0) as u32;
                    let py = (cy as i32 + dy).max(0) as u32;

                    if let Some(mut bg) = canvas.get(px, py) {
                        bg[3] = (bg[3] as f32 * (1.0 - factor).max(0.0)) as u8;
                        canvas.set(px, py, bg);
                    }
                }
            }
        }
    }

    /// Preenchimento flood-fill 4-conectado com tolerância de cor (P3D-059).
    pub fn flood_fill(
        canvas: &mut Canvas,
        start_x: u32,
        start_y: u32,
        new_color: [u8; 4],
        tolerance: u8,
    ) {
        if start_x >= canvas.w || start_y >= canvas.h {
            return;
        }
        let target_color = match canvas.get(start_x, start_y) {
            Some(c) => c,
            None => return,
        };

        if colors_match(target_color, new_color, 0) {
            return;
        }

        let mut queue = VecDeque::new();
        let mut visited = vec![false; (canvas.w * canvas.h) as usize];

        queue.push_back((start_x, start_y));
        let idx = (start_y * canvas.w + start_x) as usize;
        visited[idx] = true;

        let w = canvas.w as i32;
        let h = canvas.h as i32;

        while let Some((x, y)) = queue.pop_front() {
            canvas.set(x, y, new_color);

            let neighbors = [
                (x as i32 + 1, y as i32),
                (x as i32 - 1, y as i32),
                (x as i32, y as i32 + 1),
                (x as i32, y as i32 - 1),
            ];

            for (nx, ny) in neighbors {
                if nx >= 0 && nx < w && ny >= 0 && ny < h {
                    let ux = nx as u32;
                    let uy = ny as u32;
                    let uidx = (uy * canvas.w + ux) as usize;
                    if !visited[uidx] {
                        visited[uidx] = true;
                        if let Some(col) = canvas.get(ux, uy)
                            && colors_match(col, target_color, tolerance)
                        {
                            queue.push_back((ux, uy));
                        }
                    }
                }
            }
        }
    }

    /// Pinta no canvas 2D com suporte aos 5 tipos de pincéis.
    pub fn canvas_brush_advanced(
        state: &mut AppState,
        x: u32,
        y: u32,
        brush: BrushType,
        radius: u32,
        strength: f32,
    ) {
        Self::ensure_canvas(state);
        let color = [
            (state.paint_color[0] * 255.0) as u8,
            (state.paint_color[1] * 255.0) as u8,
            (state.paint_color[2] * 255.0) as u8,
            255,
        ];

        let active_idx = state.project.active;
        let mat_id = state
            .project
            .assets
            .get(active_idx)
            .and_then(|a| a.material_id);

        if let Some(o) = state.project.active_mut()
            && let Some(cv) = o.texture.as_mut()
        {
            match brush {
                BrushType::Pixel => Self::stamp_pixel_brush(cv, x, y, radius, color),
                BrushType::Soft => Self::stamp_soft_brush(cv, x, y, radius, color, strength),
                BrushType::Eraser => Self::stamp_eraser(cv, x, y, radius, strength),
                BrushType::Fill => Self::flood_fill(cv, x, y, color, 16),
                BrushType::Eyedropper => {
                    if let Some(c) = cv.get(x, y) {
                        state.paint_color = [
                            c[0] as f32 / 255.0,
                            c[1] as f32 / 255.0,
                            c[2] as f32 / 255.0,
                        ];
                    }
                }
            }
        }

        let cloned_cv = state
            .project
            .assets
            .get(active_idx)
            .and_then(|o| o.texture.clone());

        // Sincroniza de volta com o canal Albedo do Material (P3D-050, P3D-051)
        if let (Some(mid), Some(src_cv)) = (mat_id, cloned_cv)
            && let Some(mat) = state.project.project.get_material_mut(mid)
        {
            mat.albedo_texture = Some(src_cv);
        }

        state.render.canvas_dirty = true;
        state.mark_dirty();
    }

    /// Wrapper compatível com API legado.
    pub fn canvas_brush(state: &mut AppState, x: u32, y: u32, erase: bool) {
        let brush = if erase {
            BrushType::Eraser
        } else {
            BrushType::Pixel
        };
        Self::canvas_brush_advanced(state, x, y, brush, state.canvas_brush, 1.0);
    }

    /// Preenche todo o canvas ativo com a cor selecionada.
    pub fn canvas_fill(state: &mut AppState) {
        Self::ensure_canvas(state);
        let color = [
            (state.paint_color[0] * 255.0) as u8,
            (state.paint_color[1] * 255.0) as u8,
            (state.paint_color[2] * 255.0) as u8,
            255,
        ];

        let active_idx = state.project.active;
        let mat_id = state
            .project
            .assets
            .get(active_idx)
            .and_then(|a| a.material_id);

        if let Some(o) = state.project.active_mut()
            && let Some(cv) = o.texture.as_mut()
        {
            cv.fill(color);
        }

        if let Some(mid) = mat_id
            && let Some(mat) = state.project.project.get_material_mut(mid)
            && let Some(cv) = mat.albedo_texture.as_mut()
        {
            cv.fill(color);
        }

        state.render.canvas_dirty = true;
        state.mark_dirty();
    }

    // ---- Pintura 3D Direta sobre Malha via UV (P3D-062, P3D-132) ----

    /// Pinta na textura 2D do modelo projetando o ponto de impacto 3D nas coordenadas UV da face.
    pub fn paint_mesh_3d(
        state: &mut AppState,
        face_idx: usize,
        hit_pos: Vec3,
        brush: BrushType,
        radius: u32,
        strength: f32,
        isolate_selection: bool,
    ) -> bool {
        let (uv, should_paint) = {
            let active_asset = match state.project.assets.get(state.project.active) {
                Some(a) => a,
                None => return false,
            };
            let mesh = &active_asset.mesh;

            let face = match mesh.faces.get(face_idx) {
                Some(f) => f,
                None => return false,
            };

            // P3D-132: Paint Masks / Face & Selection Isolation
            if isolate_selection {
                let has_selected_faces = mesh.faces.iter().any(|f| f.selected);
                if has_selected_faces && !face.selected {
                    return false;
                }
            }

            let m = face.verts.len();
            if m < 3 || face.uv.len() < m {
                return false;
            }

            let mut best_uv = None;
            // Decompõe face em triângulos fan a partir do vértice 0
            for i in 1..m - 1 {
                let idx0 = face.verts[0] as usize;
                let idx1 = face.verts[i] as usize;
                let idx2 = face.verts[i + 1] as usize;

                if idx0 < mesh.verts.len() && idx1 < mesh.verts.len() && idx2 < mesh.verts.len() {
                    let v0 = mesh.verts[idx0].vec();
                    let v1 = mesh.verts[idx1].vec();
                    let v2 = mesh.verts[idx2].vec();

                    let uv0 = face.uv[0];
                    let uv1 = face.uv[i];
                    let uv2 = face.uv[i + 1];

                    if let Some(interpolated) = barycentric_uv(hit_pos, v0, v1, v2, uv0, uv1, uv2) {
                        best_uv = Some(interpolated);
                        break;
                    }
                }
            }

            (best_uv, true)
        };

        if !should_paint {
            return false;
        }

        if let Some(uv) = uv {
            Self::ensure_canvas(state);
            let (w, h) = if let Some(o) = state.project.assets.get(state.project.active)
                && let Some(cv) = &o.texture
            {
                (cv.w, cv.h)
            } else {
                (256, 256)
            };

            let u = uv[0].rem_euclid(1.0);
            let v = uv[1].rem_euclid(1.0);
            let px = ((u * w as f32) as u32).min(w.saturating_sub(1));
            let py = (((1.0 - v) * h as f32) as u32).min(h.saturating_sub(1));

            Self::canvas_brush_advanced(state, px, py, brush, radius, strength);
            true
        } else {
            false
        }
    }
}

/// Calcula interpolação baricêntrica de coordenadas UV para um ponto dentro de um triângulo 3D.
pub fn barycentric_uv(
    p: Vec3,
    a: Vec3,
    b: Vec3,
    c: Vec3,
    uv_a: [f32; 2],
    uv_b: [f32; 2],
    uv_c: [f32; 2],
) -> Option<[f32; 2]> {
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;

    let d00 = v0.dot(v0);
    let d01 = v0.dot(v1);
    let d11 = v1.dot(v1);
    let d20 = v2.dot(v0);
    let d21 = v2.dot(v1);

    let denom = d00 * d11 - d01 * d01;
    if denom.abs() < 1e-8 {
        return None;
    }

    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

    // Tolerância para pontos na borda ou levemente fora do triângulo
    let eps = -0.05;
    if u >= eps && v >= eps && w >= eps {
        let interpolated_u = u * uv_a[0] + v * uv_b[0] + w * uv_c[0];
        let interpolated_v = u * uv_a[1] + v * uv_b[1] + w * uv_c[1];
        Some([interpolated_u, interpolated_v])
    } else {
        None
    }
}

fn colors_match(a: [u8; 4], b: [u8; 4], tol: u8) -> bool {
    let t = tol as i16;
    (a[0] as i16 - b[0] as i16).abs() <= t
        && (a[1] as i16 - b[1] as i16).abs() <= t
        && (a[2] as i16 - b[2] as i16).abs() <= t
        && (a[3] as i16 - b[3] as i16).abs() <= t
}

impl Module for PaintModule {
    fn id(&self) -> &'static str {
        "paint"
    }

    fn as_any(&self) -> &(dyn std::any::Any + 'static) {
        self
    }

    fn as_any_mut(&mut self) -> &mut (dyn std::any::Any + 'static) {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_set_palette_sync() {
        let mut state = AppState::new("en");

        PaintModule::push_palette(&mut state, [0.5, 0.5, 0.5]);
        assert_eq!(state.project.palette[0], [0.5, 0.5, 0.5]);

        let p8 = petunia_project::preset_pico8();
        PaintModule::set_palette(&mut state, p8.clone());
        assert_eq!(state.project.palette.len(), 16);
        assert_eq!(state.project.palette, p8);
    }

    #[test]
    fn test_pixel_brush_and_canvas_stamp() {
        let mut cv = Canvas::new(8, 8, [0, 0, 0, 255]);
        PaintModule::stamp_pixel_brush(&mut cv, 4, 4, 1, [255, 255, 255, 255]);

        assert_eq!(cv.get(4, 4), Some([255, 255, 255, 255]));
        assert_eq!(cv.get(4, 3), Some([255, 255, 255, 255]));
        assert_eq!(cv.get(4, 5), Some([255, 255, 255, 255]));
        assert_eq!(cv.get(0, 0), Some([0, 0, 0, 255]));
    }

    #[test]
    fn test_eraser_reduces_alpha() {
        let mut cv = Canvas::new(8, 8, [100, 100, 100, 255]);
        PaintModule::stamp_eraser(&mut cv, 4, 4, 2, 1.0);

        let center = cv.get(4, 4).unwrap();
        assert_eq!(center[3], 0);
    }

    #[test]
    fn test_flood_fill_changes_connected_region() {
        let mut cv = Canvas::new(4, 4, [0, 0, 0, 255]);
        cv.set(1, 0, [255, 0, 0, 255]); // barreira
        cv.set(1, 1, [255, 0, 0, 255]);
        cv.set(1, 2, [255, 0, 0, 255]);
        cv.set(1, 3, [255, 0, 0, 255]);

        PaintModule::flood_fill(&mut cv, 0, 0, [0, 255, 0, 255], 0);

        assert_eq!(cv.get(0, 0), Some([0, 255, 0, 255]));
        assert_eq!(cv.get(0, 3), Some([0, 255, 0, 255]));
        // Do outro lado da barreira deve permanecer intacto
        assert_eq!(cv.get(2, 0), Some([0, 0, 0, 255]));
    }

    #[test]
    fn test_barycentric_uv_interpolation() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(2.0, 0.0, 0.0);
        let c = Vec3::new(0.0, 2.0, 0.0);

        let uv_a = [0.0, 0.0];
        let uv_b = [1.0, 0.0];
        let uv_c = [0.0, 1.0];

        // Centro do triângulo
        let p = Vec3::new(0.5, 0.5, 0.0);
        let uv = barycentric_uv(p, a, b, c, uv_a, uv_b, uv_c).expect("ponto dentro do triângulo");
        assert!((uv[0] - 0.25).abs() < 1e-4);
        assert!((uv[1] - 0.25).abs() < 1e-4);
    }

    #[test]
    fn test_paint_mesh_3d_hit_and_material_sync() {
        let mut state = AppState::new("en");
        state.paint_color = [0.2, 0.8, 0.4];

        // Face frontal do cubo padrão
        let hit_pos = Vec3::new(0.0, 0.0, 1.0);
        let painted =
            PaintModule::paint_mesh_3d(&mut state, 0, hit_pos, BrushType::Pixel, 2, 1.0, false);

        assert!(painted);
        assert!(PaintModule::has_canvas(&state));

        // Sincronização verificada no Asset e no Material ativo
        let asset = state.project.active().unwrap();
        assert!(asset.texture.is_some());
        let mat = state.project.active_material().unwrap();
        assert!(mat.albedo_texture.is_some());
    }

    #[test]
    fn test_paint_mask_selection_isolation() {
        let mut state = AppState::new("en");

        // Seleciona apenas a face 1
        if let Some(m) = state.project.active_mesh_mut() {
            m.faces[1].selected = true;
        }

        // Tenta pintar na face 0 com isolamento ativado -> deve ser rejeitado (P3D-132)
        let hit_pos = Vec3::new(0.0, 0.0, 1.0);
        let painted = PaintModule::paint_mesh_3d(
            &mut state,
            0,
            hit_pos,
            BrushType::Pixel,
            2,
            1.0,
            true, // isolate_selection = true
        );
        assert!(!painted);

        // Tenta pintar na face 1 com isolamento ativado -> deve ser aceito
        let painted =
            PaintModule::paint_mesh_3d(&mut state, 1, hit_pos, BrushType::Pixel, 2, 1.0, true);
        assert!(painted);
    }

    #[test]
    fn test_paint_layer_stack_composition() {
        let mut base = Canvas::new(8, 8, [0, 0, 0, 255]);
        let mut stack = PaintLayerStack::new();

        // Camada 1: Vermelho com 50% de opacidade
        let mut layer1 = PaintLayer::new("Layer 1", 8, 8, [255, 0, 0, 255]);
        layer1.opacity = 0.5;
        stack.add_layer(layer1);

        stack.composite(&mut base);
        let px = base.get(0, 0).unwrap();
        // 0 * 0.5 + 255 * 0.5 = 128 (aprox)
        assert!((px[0] as i32 - 128).abs() <= 2);
    }

    #[test]
    fn test_decal_layer_projection() {
        let mut base = Canvas::new(16, 16, [0, 0, 0, 255]);
        let mut stack = PaintLayerStack::new();

        // Decalque 4x4 totalmente amarelo
        let decal_img = Canvas::new(4, 4, [255, 255, 0, 255]);
        // Posicionado no centro do UV (0.5, 0.5) ocupando 50% do UV (0.5, 0.5)
        let decal = DecalLayer::new(decal_img, [0.5, 0.5], [0.5, 0.5], 0.0);
        stack.add_layer(PaintLayer::new_decal("Sticker Decal", decal));

        stack.composite(&mut base);

        // O centro (8, 8) deve ser amarelo
        let center = base.get(8, 8).unwrap();
        assert_eq!(center, [255, 255, 0, 255]);

        // Os cantos externos (0, 0) devem permanecer pretos (sem cobertura do decal)
        let corner = base.get(0, 0).unwrap();
        assert_eq!(corner, [0, 0, 0, 255]);
    }

    #[test]
    fn test_paint_effects_pixelate_and_posterize() {
        // Teste do efeito Pixelate
        let mut base = Canvas::new(4, 4, [0, 0, 0, 255]);
        base.set(0, 0, [200, 100, 50, 255]);
        let mut stack = PaintLayerStack::new();
        stack.add_layer(PaintLayer::new_effect(
            "Pixelate 2x2",
            PaintEffect::Pixelate { cell_size: 2 },
        ));
        stack.composite(&mut base);

        // O pixel vizinho no bloco 2x2 deve ter adotado a cor do topo do bloco
        assert_eq!(base.get(1, 1), Some([200, 100, 50, 255]));

        // Teste do efeito Posterize
        let mut base_post = Canvas::new(2, 2, [120, 130, 140, 255]);
        let mut stack_post = PaintLayerStack::new();
        stack_post.add_layer(PaintLayer::new_effect(
            "Posterize 2 levels",
            PaintEffect::Posterize { levels: 2 },
        ));
        stack_post.composite(&mut base_post);
        let px = base_post.get(0, 0).unwrap();
        // Com 2 níveis, valores ~128 são quantizados para 0 ou 255
        assert!(px[0] == 0 || px[0] == 255);

        // Teste do efeito Invert
        let mut base_inv = Canvas::new(2, 2, [255, 0, 100, 255]);
        let mut stack_inv = PaintLayerStack::new();
        stack_inv.add_layer(PaintLayer::new_effect("Invert", PaintEffect::Invert));
        stack_inv.composite(&mut base_inv);
        assert_eq!(base_inv.get(0, 0), Some([0, 255, 155, 255]));
    }
}
