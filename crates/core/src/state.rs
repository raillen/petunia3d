//! Estado global do editor (vive em `core`, sem conhecer backends).
//! Renderers/módulos/UI operam sobre este estado + eventos.

use std::collections::HashSet;

use glam::Vec3;
use petunia_commands::UndoStack;
use petunia_config::{I18n, Keybinds};
use petunia_project::Project;
use petunia_render::Shading;

use super::camera::Camera;
use super::events::{AppEvent, EventBus};
use super::selection::{SelectMode, Selection, Workspace};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefAxis {
    Front,
    Back,
    Left,
    Right,
    Side,
    Top,
    Bottom,
}

impl RefAxis {
    pub fn key(&self) -> &'static str {
        match self {
            RefAxis::Front => "refs.front",
            RefAxis::Back => "refs.back",
            RefAxis::Left => "refs.left",
            RefAxis::Right | RefAxis::Side => "refs.right",
            RefAxis::Top => "refs.top",
            RefAxis::Bottom => "refs.bottom",
        }
    }
}

/// Imagem de referência: pixels RGBA + posicionamento no espaço.
pub struct ReferenceImage {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    pub texture: Option<egui::TextureHandle>,
    pub axis: RefAxis,
    pub offset: f32,
    pub size: f32,
    pub opacity: f32,
    pub visible: bool,
    pub locked: bool,
    pub rotation: f32,
    pub xray: bool,
}

impl ReferenceImage {
    pub fn from_rgba(name: String, width: u32, height: u32, rgba: Vec<u8>) -> Self {
        Self {
            name,
            width,
            height,
            rgba,
            texture: None,
            axis: RefAxis::Front,
            offset: -3.0,
            size: 4.0,
            opacity: 0.6,
            visible: true,
            locked: false,
            rotation: 0.0,
            xray: false,
        }
    }

    pub fn ensure_texture(&mut self, ctx: &egui::Context) {
        if self.texture.is_none() {
            let img = egui::ColorImage::from_rgba_unmultiplied(
                [self.width as usize, self.height as usize],
                &self.rgba,
            );
            self.texture = Some(ctx.load_texture(&self.name, img, egui::TextureOptions::LINEAR));
        }
    }
}

/// Perfil 2D do Draw Profile (spec §9), num frame right/up/origin capturado
/// ao ativar a ferramenta numa vista ortográfica.
#[derive(Debug, Clone, Default)]
pub struct ProfileState {
    pub points: Vec<[f32; 2]>,
    pub right: [f32; 3],
    pub up: [f32; 3],
    pub origin: [f32; 3],
    pub normal: [f32; 3],
    pub closed: bool,
    pub depth: f32,
    pub revolve_segments: u32,
    pub snap: bool,
}

impl ProfileState {
    pub fn clear(&mut self) {
        *self = Self {
            depth: self.depth,
            revolve_segments: self.revolve_segments,
            ..Default::default()
        };
        if self.depth == 0.0 {
            self.depth = 1.0;
        }
        if self.revolve_segments == 0 {
            self.revolve_segments = 12;
        }
    }
    pub fn to_3d(&self, i: usize) -> Vec3 {
        let p = self.points[i];
        Vec3::from(self.origin) + Vec3::from(self.right) * p[0] + Vec3::from(self.up) * p[1]
    }
}

/// Estatísticas do último frame (overlay §45).
#[derive(Debug, Clone, Copy, Default)]
pub struct RenderStats {
    pub fps: f32,
    pub frame_ms: f32,
    pub tris: usize,
    pub verts: usize,
    pub draws: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EditMode {
    #[default]
    Object,
    Edit,
    TexturePaint,
}

impl EditMode {
    pub fn key(&self) -> &'static str {
        match self {
            EditMode::Object => "modes.object",
            EditMode::Edit => "modes.edit",
            EditMode::TexturePaint => "modes.paint",
        }
    }
}

pub struct AppState {
    pub project: Project,
    pub undo: UndoStack<Project>,
    pub events: EventBus,
    pub selection: Selection,
    pub mode: EditMode,
    pub workspace: Workspace,
    pub select_mode: SelectMode,
    pub shading: Shading,
    pub textured: bool,
    pub active_tool: String,
    pub camera: Camera,
    pub camera_frame: Option<(Camera, Camera, f32)>,
    pub gizmo_mode: crate::ModalKind,
    pub i18n: I18n,
    pub keybinds: Keybinds,
    pub refs: Vec<ReferenceImage>,
    pub profile: ProfileState,
    pub uv_selected: HashSet<usize>,
    pub paint_color: [f32; 3],
    pub paint_radius: f32,
    pub paint_strength: f32,
    pub paint_stroke: Option<Project>,
    pub mesh_preview: Option<crate::mesh_preview::MeshPreview>,
    pub palette: Vec<[f32; 3]>,
    pub canvas_brush: u32,
    pub transform_delta: [f32; 3],
    pub transform_scale: f32,
    pub extrude_dist: f32,
    pub inset_factor: f32,
    pub bevel_amount: f32,
    pub mirror_axis: usize,
    pub mirror_weld: f32,
    pub push_dist: f32,
    pub status: String,
    pub stats: RenderStats,
    pub viewport_rect: Option<egui::Rect>,
    pub viewport_pixels_per_point: f32,
    pub pending_pick: Option<(f32, f32)>,
    pub modal: Option<crate::modal::ModalOp>,
    pub pending_modal: Option<crate::modal::ModalKind>,
    pub box_select_start: Option<[f32; 2]>,
    pub show_help: bool,
    pub show_perf: bool,
    pub dirty: bool,
    pub project_path: Option<String>,
    /// Nome do backend ativo (wgpu xxx / OpenGL) p/ status bar.
    pub backend_name: String,
    /// Textura egui do canvas do asset ativo (painel PAINT).
    pub canvas_tex: Option<egui::TextureHandle>,
    pub canvas_dirty: bool,
    /// Seleção múltipla p/ export em lote (índices de assets).
    pub export_selected: Vec<usize>,
    /// Posição do 3D Cursor no espaço de mundo.
    pub cursor_3d: [f32; 3],
    /// Posição de abertura do menu contextual (RMB) no viewport.
    pub context_menu_pos: Option<[f32; 2]>,
    /// Formato do export: false = OBJ (pasta), true = GLB (arquivo).
    pub export_gltf: bool,
    /// Termo de busca no painel Outliner.
    pub outliner_search: String,
    /// Aba ativa no painel Properties (tool, render, object, modifiers, data, material).
    pub properties_tab: String,
    /// Frame atual da timeline de animação.
    pub timeline_frame: i32,
    /// Frame inicial do intervalo da timeline.
    pub timeline_start: i32,
    /// Frame final do intervalo da timeline.
    pub timeline_end: i32,
    /// Estado de reprodução da timeline.
    pub timeline_playing: bool,
    /// Snapping magnético ativado no viewport.
    pub snap_enabled: bool,
    /// Edição proporcional ativada no viewport.
    pub proportional_editing: bool,
    /// Orientação de transformação ativa ("Global", "Local", etc.).
    pub transform_orientation: String,
    /// Ponto de pivô ativo ("Median Point", "3D Cursor", etc.).
    pub pivot_point: String,
    /// Exibição de overlays no viewport (grid, eixos, 3d cursor).
    pub show_overlays: bool,
    /// Modo de raio-x / transparência no viewport.
    pub show_xray: bool,
    /// Medições ativas na cena (ferramenta Measure).
    pub measurements: Vec<Measurement>,
    pub active_measurement: Option<Measurement>,
    /// Anotações e rascunhos livres na cena (ferramenta Annotate).
    pub annotations: Vec<AnnotationStroke>,
    pub active_annotation: Option<AnnotationStroke>,
    /// Modal de configurações ativado.
    pub show_settings: bool,
    pub settings_tab: String,
    pub show_asset_library: bool,
    /// Painel retrátil de navegação de assets (lado esquerdo).
    pub show_asset_browser: bool,
    /// Tema ativo ("petunia-dark", "petunia-light", "petunia-capuccino", "petunia-tokyo-nights").
    pub active_theme_id: String,
    /// Pacote de ícones ativo ("tabler", "iconoir", "phosphor", "lucide").
    pub active_icon_pack_id: String,
    /// Perfil de atalhos ativo ("petunia-default", "blender-like", etc.).
    pub active_keymap_id: String,
}

/// Medição interativa em espaço 3D (ferramenta Measure).
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Measurement {
    pub start: [f32; 3],
    pub end: [f32; 3],
    pub distance: f32,
}

impl Measurement {
    pub fn new(start: [f32; 3], end: [f32; 3]) -> Self {
        let dx = end[0] - start[0];
        let dy = end[1] - start[1];
        let dz = end[2] - start[2];
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();
        Self {
            start,
            end,
            distance,
        }
    }

    pub fn deltas(&self) -> [f32; 3] {
        [
            (self.end[0] - self.start[0]).abs(),
            (self.end[1] - self.start[1]).abs(),
            (self.end[2] - self.start[2]).abs(),
        ]
    }
}

/// Traço de anotação livre em espaço 3D ou tela (ferramenta Annotate).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnnotationStroke {
    pub points: Vec<[f32; 3]>,
    pub color: [f32; 4],
    pub width: f32,
}

impl Default for AnnotationStroke {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            color: [0.0, 0.74, 0.83, 1.0], // Ciano característico do Blender
            width: 2.0,
        }
    }
}

impl AppState {
    pub fn new(lang: &str) -> Self {
        Self {
            project: Project::new(),
            undo: UndoStack::new(),
            events: EventBus::new(),
            selection: Selection::default(),
            mode: EditMode::Object,
            workspace: Workspace::Model,
            select_mode: SelectMode::Vertex,
            shading: Shading::Solid,
            textured: false,
            active_tool: "select".to_string(),
            camera: Camera::default(),
            camera_frame: None,
            gizmo_mode: crate::ModalKind::Move,
            i18n: I18n::load(lang),
            keybinds: Keybinds::load(),
            refs: Vec::new(),
            profile: ProfileState {
                depth: 1.0,
                revolve_segments: 12,
                ..Default::default()
            },
            uv_selected: HashSet::new(),
            paint_color: [1.0, 0.2, 0.2],
            paint_radius: 0.8,
            paint_strength: 1.0,
            paint_stroke: None,
            mesh_preview: None,
            palette: vec![
                [1.0, 0.2, 0.2],
                [1.0, 0.8, 0.2],
                [0.2, 0.8, 0.3],
                [0.3, 0.5, 1.0],
            ],
            canvas_brush: 4,
            transform_delta: [0.0; 3],
            transform_scale: 1.0,
            extrude_dist: 0.5,
            inset_factor: 0.3,
            bevel_amount: 0.15,
            mirror_axis: 0,
            mirror_weld: 0.001,
            push_dist: 0.5,
            status: String::new(),
            stats: RenderStats::default(),
            viewport_rect: None,
            viewport_pixels_per_point: 1.0,
            pending_pick: None,
            modal: None,
            pending_modal: None,
            box_select_start: None,
            show_help: false,
            show_perf: false,
            dirty: true,
            project_path: None,
            backend_name: String::new(),
            canvas_tex: None,
            canvas_dirty: true,
            export_selected: Vec::new(),
            cursor_3d: [0.0, 0.0, 0.0],
            context_menu_pos: None,
            export_gltf: true,
            outliner_search: String::new(),
            properties_tab: "object".to_string(),
            timeline_frame: 1,
            timeline_start: 1,
            timeline_end: 250,
            timeline_playing: false,
            snap_enabled: false,
            proportional_editing: false,
            transform_orientation: "Global".to_string(),
            pivot_point: "Median Point".to_string(),
            show_overlays: true,
            show_xray: false,
            measurements: Vec::new(),
            active_measurement: None,
            annotations: Vec::new(),
            active_annotation: None,
            show_settings: false,
            settings_tab: "appearance".to_string(),
            show_asset_library: false,
            show_asset_browser: false,
            active_theme_id: "petunia-dark".to_string(),
            active_icon_pack_id: "tabler".to_string(),
            active_keymap_id: "petunia-default".to_string(),
        }
    }

    pub fn scene_tris(&self) -> usize {
        self.project.assets.iter().map(|a| a.mesh.tri_count()).sum()
    }

    pub fn scene_verts(&self) -> usize {
        self.project.assets.iter().map(|a| a.mesh.verts.len()).sum()
    }

    pub fn t(&self, key: &str) -> String {
        self.i18n.t(key)
    }

    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status = msg.into();
    }

    /// Marca para render-on-demand (§33).
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }
    pub fn consume_dirty(&mut self) -> bool {
        std::mem::replace(&mut self.dirty, false)
    }

    /// Checkpoint de undo ANTES de mutar o projeto + evento.
    pub fn checkpoint(&mut self, label: &str) {
        let snap = self.project.clone();
        self.undo.checkpoint(label, &snap);
        self.mark_dirty();
    }

    pub fn undo(&mut self) -> bool {
        if self.mesh_preview.is_some() {
            self.finish_mesh_preview(true);
            return true;
        }
        if self.paint_stroke.is_some() {
            self.finish_paint_stroke(true);
            return true;
        }
        if self.cancel_modal() {
            return true;
        }
        let cur = self.project.clone();
        if let Some(prev) = self.undo.undo(cur) {
            self.palette = prev.palette.clone();
            self.project = prev;
            self.sync_selection();
            self.uv_selected.clear();
            self.mark_dirty();
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self) -> bool {
        if self.mesh_preview.is_some() {
            self.finish_mesh_preview(true);
            return true;
        }
        if self.paint_stroke.is_some() {
            self.finish_paint_stroke(true);
            return true;
        }
        if self.cancel_modal() {
            return true;
        }
        let cur = self.project.clone();
        if let Some(next) = self.undo.redo(cur) {
            self.palette = next.palette.clone();
            self.project = next;
            self.sync_selection();
            self.uv_selected.clear();
            self.mark_dirty();
            true
        } else {
            false
        }
    }

    pub fn emit_mesh_changed(&mut self) {
        let id = self.project.assets.get(self.project.active).map(|a| a.id);
        if let Some(asset_id) = id {
            self.events.emit(AppEvent::MeshChanged { asset_id });
        }
        self.mark_dirty();
    }

    pub fn sync_selection(&mut self) {
        let mut sel = Selection::default();
        if let Some(a) = self.project.assets.get(self.project.active) {
            sel.asset = Some(a.id);
            sel.verts = a
                .mesh
                .verts
                .iter()
                .enumerate()
                .filter(|(_, v)| v.selected)
                .map(|(i, _)| i as u32)
                .collect();
            sel.faces = a
                .mesh
                .faces
                .iter()
                .enumerate()
                .filter(|(_, f)| f.selected)
                .map(|(i, _)| i)
                .collect();
        }
        self.selection = sel.clone();
        self.events.emit(AppEvent::SelectionChanged(sel));
        self.mark_dirty();
    }

    /// Pinta vértices próximos do ponto 3D (vertex paint).
    pub fn paint_at(&mut self, center: Vec3) {
        let before = self.paint_stroke.is_none().then(|| self.project.clone());
        let (mut n, col, r, k) = (
            0,
            self.paint_color,
            self.paint_radius,
            self.paint_strength.clamp(0.0, 1.0),
        );
        let r2 = r * r;
        if let Some(obj) = self.project.active_mut() {
            for v in &mut obj.mesh.verts {
                let d2 = (v.vec() - center).length_squared();
                if d2 <= r2 {
                    for (ch, cc) in v.color.iter_mut().zip(col.iter()) {
                        *ch = *ch * (1.0 - k) + cc * k;
                    }
                    n += 1;
                }
            }
        }
        if n > 0 {
            if let Some(before) = before {
                self.undo.checkpoint("paint", &before);
            }
            let id = self.project.assets.get(self.project.active).map(|a| a.id);
            if let Some(asset_id) = id {
                self.events.emit(AppEvent::TextureChanged { asset_id });
            }
            self.status = format!("paint: {n} verts");
            self.mark_dirty();
        }
    }

    pub fn begin_paint_stroke(&mut self) {
        if self.modal.is_some() || self.mesh_preview.is_some() {
            return;
        }
        if self.paint_stroke.is_none() {
            self.paint_stroke = Some(self.project.clone());
        }
    }

    pub fn finish_paint_stroke(&mut self, cancel: bool) {
        let Some(original) = self.paint_stroke.take() else {
            return;
        };
        if cancel {
            self.project = original;
        } else {
            let changed = original
                .active_mesh()
                .zip(self.project.active_mesh())
                .is_some_and(|(a, b)| {
                    a.verts
                        .iter()
                        .zip(&b.verts)
                        .any(|(a, b)| a.color != b.color)
                });
            if changed {
                self.undo.checkpoint("Paint stroke", &original);
            }
        }
        self.emit_mesh_changed();
    }

    /// Vértice mais próximo do raio (pick em perspectiva e ortográfica).
    pub fn pick_vertex(&self, origin: Vec3, dir: Vec3) -> Option<(usize, Vec3)> {
        let obj = self.project.assets.get(self.project.active)?;
        let mut best: Option<(usize, f32, Vec3)> = None;
        for (i, v) in obj.mesh.verts.iter().enumerate() {
            let p = v.vec();
            let to = p - origin;
            let t = to.dot(dir);
            if t < 0.0 {
                continue;
            }
            let proj = origin + dir * t;
            let d = (p - proj).length();
            let tol = 0.12 * (1.0 + t * 0.15);
            if d < tol && best.map(|(_, bt, _)| t < bt).unwrap_or(true) {
                best = Some((i, t, p));
            }
        }
        best.map(|(i, _, p)| (i, p))
    }

    /// Aresta mais próxima do raio (modo Edge).
    pub fn pick_edge(&self, origin: Vec3, dir: Vec3) -> Option<((u32, u32), Vec3)> {
        let obj = self.project.assets.get(self.project.active)?;
        let mut best: Option<((u32, u32), f32, f32, Vec3)> = None;
        for (a, b) in obj.mesh.edges_unique() {
            let pa = obj.mesh.verts[a as usize].vec();
            let pb = obj.mesh.verts[b as usize].vec();
            // ponto do segmento mais próximo do raio (amostragem + refinamento)
            let mut bd = f32::MAX;
            let mut bp = pa;
            for k in 0..=8 {
                let p = pa.lerp(pb, k as f32 / 8.0);
                let t = (p - origin).dot(dir);
                if t < 0.0 {
                    continue;
                }
                let d = (p - (origin + dir * t)).length();
                if d < bd {
                    bd = d;
                    bp = p;
                }
            }
            let t = (bp - origin).dot(dir);
            let tol = 0.15 * (1.0 + t.max(0.0) * 0.15);
            if bd < tol && best.map(|(_, bt, _, _)| t < bt).unwrap_or(true) {
                best = Some(((a, b), t, bd, bp));
            }
        }
        best.map(|(e, _, _, p)| (e, p))
    }

    /// Salva o objeto ativo atual como um novo asset permanente na biblioteca do projeto.
    pub fn save_active_as_asset(&mut self) -> bool {
        if let Some(active_asset) = self.project.active() {
            let mut cloned = active_asset.duplicate();
            cloned.name = format!("{} (Asset)", active_asset.name);
            let name = cloned.name.clone();
            self.checkpoint("save asset");
            self.project.assets.push(cloned);
            self.set_status(format!("Asset '{}' salvo na biblioteca do projeto", name));
            self.mark_dirty();
            return true;
        }
        false
    }

    /// Cria uma nova instância de um asset da biblioteca na posição do 3D Cursor.
    pub fn instantiate_asset_at_cursor(&mut self, asset_index: usize) -> bool {
        if let Some(asset) = self.project.assets.get(asset_index) {
            let mut new_asset = asset.duplicate();
            let cursor = self.cursor_3d;
            for v in &mut new_asset.mesh.verts {
                v.pos[0] += cursor[0];
                v.pos[1] += cursor[1];
                v.pos[2] += cursor[2];
            }
            let name = new_asset.name.clone();
            self.checkpoint("instantiate asset");
            self.project.assets.push(new_asset);
            self.project.active = self.project.assets.len() - 1;
            self.sync_selection();
            self.emit_mesh_changed();
            self.set_status(format!("Asset '{}' instanciado na cena", name));
            self.mark_dirty();
            return true;
        }
        false
    }
}
