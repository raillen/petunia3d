//! Pilha de camadas de pintura (P3D-061, P3D-133, P3D-134).
//!
//! Vive no crate de projeto porque é **dado persistente do asset**
//! (serialize + undo via snapshot do `Project`), não estado de UI.
//! `module-paint` re-exporta estes tipos e implementa as operações.
//!
//! Modelo V1: camadas Raster ordenadas com visibility/opacity/active,
//! composição determinística alpha-normal sobre o canvas base. Decal e
//! Effect existem como dados (pós-V1: P3D-133/134) e participam da
//! composição, mas a UI V1 só cria/opera Raster.

use serde::{Deserialize, Serialize};

use crate::Canvas;

/// Modo de mesclagem de camadas de pintura (P3D-061).
///
/// V1 usa `Normal`; demais modos existem para compatibilidade de
/// serialização e testes, sem UI dedicada.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LayerBlendMode {
    #[default]
    Normal,
    Multiply,
    Add,
    Screen,
}

/// Efeitos não-destrutivos sobre texturas / camadas (P3D-134, pós-V1).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum PaintEffect {
    /// Pixelização com tamanho de bloco especificado (P3D-134).
    Pixelate { cell_size: u32 },
    /// Quantização / posterização de tons por canal de cor (P3D-134).
    Posterize { levels: u8 },
    /// Inversão de cores RGB.
    Invert,
}

/// Decalque / projeção 2D parametrizada e reposicionável (P3D-133, pós-V1).
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
    /// Camada de decalque / estampa projetada sobre o UV (P3D-133, pós-V1).
    Decal(DecalLayer),
    /// Camada de efeito não-destrutivo aplicada sobre a composição inferior (P3D-134, pós-V1).
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

    /// Só camadas Raster aceitam pinceladas (decal/efeito são pós-V1).
    pub fn is_paintable(&self) -> bool {
        matches!(&self.kind, LayerKind::Raster(_))
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

    /// Pilha inicial V1: uma camada Raster sobre o canvas base.
    pub fn with_base(name: impl Into<String>, canvas: Canvas) -> Self {
        let mut stack = Self::new();
        stack.add_layer(PaintLayer::new_raster(name, canvas));
        stack
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

    pub fn active(&self) -> Option<&PaintLayer> {
        self.layers.get(self.active_layer)
    }

    pub fn active_mut(&mut self) -> Option<&mut PaintLayer> {
        self.layers.get_mut(self.active_layer)
    }

    pub fn set_active(&mut self, id: uuid::Uuid) -> bool {
        if let Some(pos) = self.layers.iter().position(|l| l.id == id) {
            self.active_layer = pos;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_stack_composition_is_deterministic() {
        let mut base = Canvas::new(8, 8, [0, 0, 0, 255]);
        let mut stack = PaintLayerStack::new();
        let mut layer1 = PaintLayer::new("Layer 1", 8, 8, [255, 0, 0, 255]);
        layer1.opacity = 0.5;
        stack.add_layer(layer1);
        stack.composite(&mut base);
        let px = base.get(0, 0).unwrap();
        assert!((px[0] as i32 - 128).abs() <= 2);
    }

    #[test]
    fn active_layer_tracking_survives_remove_and_move() {
        let mut stack = PaintLayerStack::new();
        let a = stack.add_layer(PaintLayer::new("A", 4, 4, [255, 0, 0, 255]));
        let b = stack.add_layer(PaintLayer::new("B", 4, 4, [0, 255, 0, 255]));
        assert!(stack.set_active(a));
        assert_eq!(stack.active().map(|l| l.id), Some(a));
        assert!(stack.move_layer(0, 1));
        assert_eq!(stack.active().map(|l| l.id), Some(a));
        assert!(stack.remove_layer(b));
        assert_eq!(stack.layers.len(), 1);
    }
}
