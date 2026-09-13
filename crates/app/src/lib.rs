//! Petunia3D app: núcleo `Core` + backends wgpu/OpenGL + render-on-demand.
//! Módulos concretos vivem aqui; eventos são despachados a eles por frame.

use std::sync::Arc;

use petunia_core::{
    AppState, ClearSelectionCmd, DeleteSelectionCmd, DuplicateSelectionCmd, EditMode,
    InvertSelectionCmd, ModuleRegistry, SelectAllCmd, SelectMode,
};
use petunia_module_assets::AssetsModule;
use petunia_module_model::ToolRegistry;
use petunia_module_paint::PaintModule;
use petunia_module_uv::UvModule;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode as WKey, PhysicalKey};
use winit::window::{Window, WindowId};

use petunia_core::events::AppEvent;

// ---------------------------------------------------------------- Core

pub struct Core {
    pub state: AppState,
    pub tools: ToolRegistry,
    pub registry: ModuleRegistry,
    pub autosave: petunia_core::AutosaveService,
    pub recent_projects: petunia_core::RecentProjects,
    pub pending_recovery: Option<petunia_core::RecoveryInfo>,
    pub mmb_down: bool,
    pub shift_down: bool,
    pub ctrl_down: bool,
    pub alt_down: bool,
    pub last_mouse: Option<(f64, f64)>,
    pub save_requested: bool,
}

impl Default for Core {
    fn default() -> Self {
        Self::new()
    }
}

impl Core {
    pub fn new() -> Self {
        let lang = std::env::var("PETUNIA_LANG")
            .or_else(|_| std::env::var("SIMPLE3D_LANG"))
            .unwrap_or_else(|_| "pt-BR".to_string());
        let lang = if ["en", "pt-BR"].contains(&lang.as_str()) {
            lang
        } else {
            "pt-BR".to_string()
        };

        let pending_recovery = petunia_core::AutosaveService::detect_recovery(None);
        let now_secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let _ = petunia_core::AutosaveService::create_session_lock(None, "Untitled", now_secs);

        Self {
            state: AppState::new(&lang),
            tools: ToolRegistry::with_defaults(),
            registry: {
                let mut r = ModuleRegistry::new();
                r.register(PaintModule::new());
                r.register(UvModule::new());
                r.register(AssetsModule::new());
                r
            },
            autosave: petunia_core::AutosaveService::default(),
            recent_projects: petunia_core::RecentProjects::default(),
            pending_recovery,
            mmb_down: false,
            shift_down: false,
            ctrl_down: false,
            alt_down: false,
            last_mouse: None,
            save_requested: false,
        }
    }

    /// Executa o tick periódico do autosave (P3D-002).
    pub fn tick_autosave(&mut self) {
        let now_secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let is_dirty = self.state.is_document_dirty();
        let proj_path = self
            .state
            .project
            .project_path
            .as_deref()
            .map(std::path::Path::new);
        if let Some(res) =
            self.autosave
                .tick(now_secs, is_dirty, &self.state.project.project, proj_path)
        {
            match res {
                Ok(path) => {
                    let filename = path
                        .file_name()
                        .and_then(|f| f.to_str())
                        .unwrap_or("snapshot");
                    self.state.set_status(format!("autosave: {filename}"));
                }
                Err(err) => {
                    self.state.set_status(format!("autosave err: {err}"));
                }
            }
        }
    }

    /// Despacha eventos acumulados aos módulos via registry.
    pub fn dispatch_events(&mut self) {
        for ev in self.state.events.drain() {
            match &ev {
                petunia_core::AppEvent::RequestImportPalette => {
                    if let Some(path) = petunia_ui::file_dialog_service::pick_palette_import_file()
                    {
                        if let Err(e) =
                            petunia_core::ProjectService::import_palette(&mut self.state, &path)
                        {
                            self.state.set_status(format!("import palette err: {e}"));
                        }
                    }
                }
                petunia_core::AppEvent::RequestExportPalette => {
                    if let Some(path) =
                        petunia_ui::file_dialog_service::pick_palette_export_file("palette.gpl")
                    {
                        if let Err(e) = petunia_core::ProjectService::export_palette(
                            &self.state.project.palette,
                            "Petunia Palette",
                            &path,
                        ) {
                            self.state.set_status(format!("export palette err: {e}"));
                        }
                    }
                }
                _ => {}
            }
            self.registry.dispatch(&ev, &mut self.state);
        }
    }

    /// Presets de diagnóstico headless (screenshots): PETUNIA_DEMO=1 monta
    /// cena rica; WS/VIEW/SHADE/TOOL/PROFILE ajustam o estado inicial.
    pub fn apply_dev_presets(&mut self) {
        use petunia_core::{RefAxis, ViewPreset, Workspace};
        if std::env::var_os("PETUNIA_DEMO").is_some() {
            self.build_demo_scene();
        }
        if let Ok(ws) = std::env::var("PETUNIA_WS") {
            self.state.workspace = match ws.to_lowercase().as_str() {
                "paint" | "pintura" => Workspace::Paint,
                "uv" => Workspace::Uv,
                "animate" | "anim" => Workspace::Animate,
                _ => Workspace::Model,
            };
        }
        if let Ok(v) = std::env::var("PETUNIA_VIEW") {
            self.state
                .camera
                .set_preset(match v.to_lowercase().as_str() {
                    "front" => ViewPreset::Front,
                    "back" => ViewPreset::Back,
                    "left" => ViewPreset::Left,
                    "right" => ViewPreset::Right,
                    "top" => ViewPreset::Top,
                    "bottom" => ViewPreset::Bottom,
                    _ => ViewPreset::Persp,
                });
        }
        if let Ok(s) = std::env::var("PETUNIA_SHADE") {
            match s.to_lowercase().as_str() {
                "wire" => self.state.shading = petunia_render::Shading::Wireframe,
                "smooth" => self.state.shading = petunia_render::Shading::Smooth,
                "unlit" => self.state.shading = petunia_render::Shading::Unlit,
                "textured" => self.state.textured = true,
                _ => {}
            }
        }
        if let Ok(t) = std::env::var("PETUNIA_TOOL") {
            if self.tools.get(&t).is_some() {
                self.state.active_tool = t;
            }
        }
        if std::env::var_os("PETUNIA_PROFILE").is_some() {
            self.state.camera.set_preset(ViewPreset::Front);
            petunia_module_model::draw_profile::profile_capture_frame(&mut self.state);
            self.state.profile.points = vec![[-1.5, -1.5], [1.5, -1.5], [1.5, 1.5], [-1.5, 1.5]];
            self.state.profile.closed = true;
            self.state.active_tool = "draw_profile".to_string();
            let _ = RefAxis::Front;
        }
        self.state.mark_dirty();
    }

    /// Cena demo: vaso (revolve) + cápsula + referência procedural +
    /// canvas com círculo + textura ligada.
    fn build_demo_scene(&mut self) {
        use petunia_mesh::Mesh;
        let s = &mut self.state;
        if let Ok(mut vase) = Mesh::revolve(
            &[[0.0, 0.0], [0.8, 0.1], [1.0, 1.0], [0.6, 2.0], [0.0, 2.2]],
            14,
        ) {
            for v in &mut vase.verts {
                v.pos[0] -= 3.2;
            }
            s.project.add("Vase", vase);
        }
        let mut cap = Mesh::capsule(10, 0.5, 2.2);
        for v in &mut cap.verts {
            v.pos[0] += 3.2;
        }
        s.project.add("Capsule", cap);
        // referência procedural (gradiente + mira) — exercita o pipeline
        let (w, h) = (256u32, 256u32);
        let mut rgba = vec![0u8; (w * h * 4) as usize];
        for y in 0..h {
            for x in 0..w {
                let i = ((y * w + x) * 4) as usize;
                rgba[i] = (x as f32 / w as f32 * 255.0) as u8;
                rgba[i + 1] = (y as f32 / h as f32 * 255.0) as u8;
                rgba[i + 2] = 128;
                rgba[i + 3] = 255;
            }
        }
        for k in 0..w {
            let i = ((128 * w + k) * 4) as usize;
            rgba[i] = 255;
            rgba[i + 1] = 0;
            rgba[i + 2] = 0;
        }
        for k in 0..h {
            let i = ((k * w + 128) * 4) as usize;
            rgba[i] = 255;
            rgba[i + 1] = 0;
            rgba[i + 2] = 0;
        }
        let mut r = petunia_core::ReferenceImage::from_rgba("demo-ref".into(), w, h, rgba);
        r.axis = petunia_core::RefAxis::Front;
        s.project.refs.push(r);
        // canvas com círculo no CUBO (asset 0, central) + preview texturizado
        s.project.active = 0;
        PaintModule::ensure_canvas(s);
        if let Some(o) = s.project.active_mut() {
            if let Some(cv) = o.texture.as_mut() {
                for y in 0..cv.h {
                    for x in 0..cv.w {
                        let dx = x as i32 - 128;
                        let dy = y as i32 - 128;
                        if dx * dx + dy * dy < 60 * 60 {
                            cv.set(x, y, [220, 40, 40, 255]);
                        }
                    }
                }
            }
        }
        s.textured = true;
        s.render.canvas_dirty = true;
        s.sync_selection();
        s.mark_dirty();
    }

    pub fn on_mouse_input(&mut self, pressed: bool, button: MouseButton) {
        if button == MouseButton::Middle {
            self.mmb_down = pressed;
        }
        self.state.mark_dirty();
    }

    pub fn on_cursor_moved(&mut self, x: f64, y: f64) {
        if self.mmb_down {
            if let Some((lx, ly)) = self.last_mouse {
                let dx = (x - lx) as f32;
                let dy = (y - ly) as f32;
                if self.shift_down {
                    self.state.camera.pan(dx, dy);
                } else {
                    self.state.camera.orbit(dx, dy);
                }
            }
        }
        self.last_mouse = Some((x, y));
    }

    pub fn on_wheel(&mut self, y: f32) {
        self.state.camera.zoom(y);
        self.state.mark_dirty();
    }

    pub fn on_modifiers(&mut self, shift: bool, ctrl: bool, alt: bool) {
        self.shift_down = shift;
        self.ctrl_down = ctrl;
        self.alt_down = alt;
    }

    fn mods(&self) -> petunia_config::keybinds::Mods2 {
        petunia_config::keybinds::Mods2 {
            ctrl: self.ctrl_down,
            shift: self.shift_down,
            alt: self.alt_down,
        }
    }

    /// Teclado via keybinds configuráveis (assets/keybinds/*.toml).
    pub fn on_key(&mut self, physical: PhysicalKey) {
        if self.state.is_interacting() {
            return; // A sessão modal consome teclado no viewport egui.
        }
        if let PhysicalKey::Code(key) = physical {
            use petunia_core::ViewPreset;
            let preset = match (key, self.ctrl_down) {
                (WKey::Numpad1, false) => Some(ViewPreset::Front),
                (WKey::Numpad1, true) => Some(ViewPreset::Back),
                (WKey::Numpad3, false) => Some(ViewPreset::Right),
                (WKey::Numpad3, true) => Some(ViewPreset::Left),
                (WKey::Numpad7, false) => Some(ViewPreset::Top),
                (WKey::Numpad7, true) => Some(ViewPreset::Bottom),
                _ => None,
            };
            if let Some(preset) = preset {
                self.state.camera_frame = None;
                self.state.camera.set_preset(preset);
                self.state.mark_dirty();
                return;
            }
            if key == WKey::Numpad5 {
                self.state.camera_frame = None;
                self.state.camera.toggle_projection();
                self.state.mark_dirty();
                return;
            }
            if key == WKey::Numpad9 {
                self.state.camera_frame = None;
                self.state.camera.opposite_view();
                self.state.mark_dirty();
                return;
            }
            if key == WKey::NumpadDecimal {
                petunia_ui::frame_selection(&mut self.state);
                return;
            }
        }
        // Escape: cancela último ponto do perfil
        if physical == PhysicalKey::Code(WKey::Escape) {
            if self.state.active_tool == "draw_profile" && !self.state.profile.points.is_empty() {
                self.state.profile.points.pop();
                self.state.profile.closed = false;
                self.state.mark_dirty();
            }
            return;
        }
        if (self.state.workspace == petunia_core::Workspace::Paint
            || self.state.mode == EditMode::TexturePaint)
            && matches!(
                physical,
                PhysicalKey::Code(WKey::KeyF | WKey::KeyG | WKey::KeyB)
            )
            && !self.ctrl_down
        {
            return; // Brush radius and eyedropper are contextual viewport input.
        }
        // Tab: alterna entre modo Objeto e modo de Edição de malha
        if physical == PhysicalKey::Code(WKey::Tab) && !self.ctrl_down {
            self.state.mode = match self.state.mode {
                EditMode::Object => EditMode::Edit,
                _ => EditMode::Object,
            };
            self.state.mark_dirty();
            return;
        }
        // Tecla 0: Seleção de Objeto
        if physical == PhysicalKey::Code(WKey::Digit0) && !self.ctrl_down && !self.shift_down {
            self.state.mode = EditMode::Object;
            self.state.active_tool = "select".into();
            self.state.mark_dirty();
            return;
        }
        // Alt+Z: modo Raio-X
        if physical == PhysicalKey::Code(WKey::KeyZ) && self.alt_down {
            self.state.show_xray = !self.state.show_xray;
            self.state.mark_dirty();
            return;
        }
        // NumpadDivide / Slash: Isolar objeto ativo (Local View)
        if matches!(
            physical,
            PhysicalKey::Code(WKey::NumpadDivide | WKey::Slash)
        ) && !self.ctrl_down
        {
            self.state.toggle_isolate();
            return;
        }
        let mods = self.mods();
        let ck = to_config_key(physical);
        let action = ck
            .and_then(|k| self.state.ui.keybinds.find(k, mods))
            .unwrap_or("");
        let action = action.to_string();
        match action.as_str() {
            "global.undo" => {
                self.state.undo();
            }
            "global.redo" => {
                self.state.redo();
            }
            "global.save_project" => {
                self.save_requested = true;
            }
            "global.toggle_wireframe" => {
                self.state.shading = match self.state.shading {
                    petunia_render::Shading::Wireframe => petunia_render::Shading::Solid,
                    _ => petunia_render::Shading::Wireframe,
                };
                self.state.mark_dirty();
            }
            "global.help" => {
                self.state.ui.show_help = !self.state.ui.show_help;
                self.state.mark_dirty();
            }
            "global.command_palette" => {
                self.state.ui.show_command_palette = !self.state.ui.show_command_palette;
                if self.state.ui.show_command_palette {
                    self.state.ui.command_palette_query.clear();
                    self.state.ui.command_palette_selected_index = 0;
                }
                self.state.mark_dirty();
            }
            "global.settings" => {
                self.state.ui.show_settings = !self.state.ui.show_settings;
                self.state.mark_dirty();
            }
            "global.toggle_projection" => {
                self.state.camera_frame = None;
                self.state.camera.toggle_projection();
                self.state.mark_dirty();
            }
            "global.reset_camera" => {
                self.state.camera_frame = None;
                self.state.camera.reset();
                self.state.mark_dirty();
            }
            "global.cycle_mode" => {
                self.state.mode = match self.state.mode {
                    EditMode::Object => EditMode::Edit,
                    _ => EditMode::Object,
                };
                self.state.mark_dirty();
            }
            "model.select_vertex" => self.set_tool("select", Some(SelectMode::Vertex)),
            "model.select_edge" => self.set_tool("select", Some(SelectMode::Edge)),
            "model.select_face" => self.set_tool("select", Some(SelectMode::Face)),
            "model.select_object" => {
                self.state.mode = EditMode::Object;
                self.state.active_tool = "select".into();
                self.state.mark_dirty();
            }
            "model.frame_selection" => petunia_ui::frame_selection(&mut self.state),
            "model.transform" => self.set_tool("transform", None),
            "model.rotate" | "model.scale" => {
                self.state.pending_modal = Some(if action == "model.rotate" {
                    petunia_core::modal::ModalKind::Rotate
                } else {
                    petunia_core::modal::ModalKind::Scale
                });
                self.state.active_tool = "transform".into();
                self.state.mark_dirty();
            }
            "model.primitives" => self.set_tool("primitives", None),
            "model.draw_profile" => self.set_tool("draw_profile", None),
            "model.merge" => self.set_tool("merge", None),
            "model.inset" => self.set_tool("inset", None),
            "model.bevel" => self.set_tool("bevel", None),
            "model.subdivide" => self.set_tool("subdivide", None),
            "model.mirror" => self.set_tool("mirror", None),
            "model.push_pull" => self.set_tool("pushpull", None),
            "model.extrude" => {
                self.set_tool("extrude", None);
            }
            "model.extrude_individual" => {
                let dist = if self.state.extrude_dist == 0.0 {
                    0.5
                } else {
                    self.state.extrude_dist
                };
                let _ = self
                    .state
                    .dispatch(&petunia_core::ExtrudeIndividualCmd { dist });
            }
            "model.flip_diagonal" => {
                let _ = self.state.dispatch(&petunia_core::FlipDiagonalCmd);
            }
            "model.revolve" => {
                let _ = self.state.dispatch(&petunia_core::RevolveCmd::default());
            }
            "model.delete" => {
                let _ = self.state.dispatch(&DeleteSelectionCmd);
            }
            "model.duplicate" => {
                let _ = self.state.dispatch(&DuplicateSelectionCmd);
            }
            "model.slice" => self.set_tool("slice", None),
            "model.knife" | "model.loop_cut" => {
                self.state.active_tool = if action == "model.knife" {
                    "knife"
                } else {
                    "loop_cut"
                }
                .into();
                self.state.mark_dirty();
            }
            "model.connect" => self.set_tool("connect", None),
            "model.dissolve" => self.set_tool("dissolve", None),
            "model.select_all" => {
                let _ = self.state.dispatch(&SelectAllCmd);
            }
            "model.deselect_all" => {
                let _ = self.state.dispatch(&ClearSelectionCmd);
            }
            "model.invert_selection" => {
                let _ = self.state.dispatch(&InvertSelectionCmd);
            }
            "model.select_linked" => {
                let _ = self.state.dispatch(&petunia_core::SelectLinkedCmd);
            }
            "paint.paint" => self.set_tool("paint", None),
            _ => {}
        }
    }

    fn set_tool(&mut self, id: &str, select: Option<SelectMode>) {
        self.state.active_tool = id.to_string();
        if let Some(sm) = select {
            self.state.mode = EditMode::Edit;
            self.state.select_mode = sm;
            match sm {
                SelectMode::Edge => {
                    if let Some(m) = self.state.project.active_mesh_mut() {
                        m.sync_edge_selection_from_verts();
                    }
                }
                SelectMode::Face => {
                    if let Some(m) = self.state.project.active_mesh_mut() {
                        m.sync_face_selection_from_verts();
                    }
                }
                SelectMode::Vertex => {}
            }
        }
        if id == "paint" {
            self.state.mode = EditMode::TexturePaint;
        }
        if let Some(t) = self.tools.get(id) {
            t.on_activate(&mut self.state);
        }
        self.state
            .events
            .emit(AppEvent::ToolActivated(id.to_string()));
        self.state.mark_dirty();
    }
}

fn to_config_key(p: PhysicalKey) -> Option<petunia_config::keybinds::winit_keys::KeyCode> {
    use petunia_config::keybinds::winit_keys::KeyCode as C;
    let c = match p {
        PhysicalKey::Code(WKey::KeyA) => C::KeyA,
        PhysicalKey::Code(WKey::KeyB) => C::KeyB,
        PhysicalKey::Code(WKey::KeyC) => C::KeyC,
        PhysicalKey::Code(WKey::KeyD) => C::KeyD,
        PhysicalKey::Code(WKey::KeyE) => C::KeyE,
        PhysicalKey::Code(WKey::KeyF) => C::KeyF,
        PhysicalKey::Code(WKey::KeyG) => C::KeyG,
        PhysicalKey::Code(WKey::KeyH) => C::KeyH,
        PhysicalKey::Code(WKey::KeyI) => C::KeyI,
        PhysicalKey::Code(WKey::KeyJ) => C::KeyJ,
        PhysicalKey::Code(WKey::KeyK) => C::KeyK,
        PhysicalKey::Code(WKey::KeyL) => C::KeyL,
        PhysicalKey::Code(WKey::KeyM) => C::KeyM,
        PhysicalKey::Code(WKey::KeyN) => C::KeyN,
        PhysicalKey::Code(WKey::KeyO) => C::KeyO,
        PhysicalKey::Code(WKey::KeyP) => C::KeyP,
        PhysicalKey::Code(WKey::KeyQ) => C::KeyQ,
        PhysicalKey::Code(WKey::KeyR) => C::KeyR,
        PhysicalKey::Code(WKey::KeyS) => C::KeyS,
        PhysicalKey::Code(WKey::KeyT) => C::KeyT,
        PhysicalKey::Code(WKey::KeyU) => C::KeyU,
        PhysicalKey::Code(WKey::KeyV) => C::KeyV,
        PhysicalKey::Code(WKey::KeyW) => C::KeyW,
        PhysicalKey::Code(WKey::KeyX) => C::KeyX,
        PhysicalKey::Code(WKey::KeyY) => C::KeyY,
        PhysicalKey::Code(WKey::KeyZ) => C::KeyZ,
        PhysicalKey::Code(WKey::Digit0) => C::Digit0,
        PhysicalKey::Code(WKey::Digit1) => C::Digit1,
        PhysicalKey::Code(WKey::Digit2) => C::Digit2,
        PhysicalKey::Code(WKey::Digit3) => C::Digit3,
        PhysicalKey::Code(WKey::Digit4) => C::Digit4,
        PhysicalKey::Code(WKey::Digit5) => C::Digit5,
        PhysicalKey::Code(WKey::Digit6) => C::Digit6,
        PhysicalKey::Code(WKey::Digit7) => C::Digit7,
        PhysicalKey::Code(WKey::Digit8) => C::Digit8,
        PhysicalKey::Code(WKey::Digit9) => C::Digit9,
        PhysicalKey::Code(WKey::F1) => C::F1,
        PhysicalKey::Code(WKey::F2) => C::F2,
        PhysicalKey::Code(WKey::F3) => C::F3,
        PhysicalKey::Code(WKey::F4) => C::F4,
        PhysicalKey::Code(WKey::F5) => C::F5,
        PhysicalKey::Code(WKey::F6) => C::F6,
        PhysicalKey::Code(WKey::F7) => C::F7,
        PhysicalKey::Code(WKey::F8) => C::F8,
        PhysicalKey::Code(WKey::F9) => C::F9,
        PhysicalKey::Code(WKey::F10) => C::F10,
        PhysicalKey::Code(WKey::F11) => C::F11,
        PhysicalKey::Code(WKey::F12) => C::F12,
        PhysicalKey::Code(WKey::Tab) => C::Tab,
        PhysicalKey::Code(WKey::Space) => C::Space,
        PhysicalKey::Code(WKey::Delete) => C::Delete,
        PhysicalKey::Code(WKey::Backspace) => C::Backspace,
        PhysicalKey::Code(WKey::Home) => C::Home,
        PhysicalKey::Code(WKey::End) => C::End,
        PhysicalKey::Code(WKey::Escape) => C::Escape,
        PhysicalKey::Code(WKey::Enter) => C::Enter,
        _ => return None,
    };
    Some(c)
}

/// Pick no viewport: draw-profile / paint (+Alt eyedropper) / seleção.
pub fn handle_pick(core: &mut Core, nx: f32, ny: f32) {
    if core.state.active_tool == "draw_profile" {
        petunia_module_model::draw_profile::profile_add_point(&mut core.state, nx, ny);
        return;
    }
    let (origin, dir) = core.state.camera.ray(nx, ny);
    let paint_mode = core.state.mode == EditMode::TexturePaint || core.state.active_tool == "paint";
    if paint_mode && core.alt_down {
        if let Some((vi, _)) = core.state.pick_vertex(origin, dir) {
            PaintModule::eyedrop_vertex(&mut core.state, vi);
            core.state.set_status(core.state.t("paint.picked"));
        }
        return;
    }
    if paint_mode {
        // ponto real na superfície (face) cai melhor que vértice p/ pintar
        let p = core
            .state
            .project
            .assets
            .get(core.state.project.active)
            .and_then(|o| o.mesh.ray_hit(origin, dir))
            .map(|(_, _, p)| p)
            .or_else(|| core.state.pick_vertex(origin, dir).map(|(_, p)| p));
        if let Some(p) = p {
            core.state.paint_at(p);
        }
        return;
    }
    use petunia_core::picking::{pick_mesh, PickComponent};
    let viewport = core
        .state
        .ui
        .viewport_rect
        .map(|r| glam::Vec2::new(r.width(), r.height()))
        .unwrap_or(glam::Vec2::new(800.0, 600.0))
        * core.state.ui.viewport_pixels_per_point;
    let mode = if core.state.mode == EditMode::Object {
        SelectMode::Face
    } else {
        core.state.select_mode
    };
    let hit = core.state.project.active_mesh().and_then(|mesh| {
        pick_mesh(
            mesh,
            &core.state.camera,
            viewport,
            glam::Vec2::new(nx, ny),
            mode,
            core.state.shading == petunia_render::Shading::Wireframe || core.state.show_xray,
        )
    });
    if let Some(mesh) = core.state.project.active_mesh_mut() {
        if core.state.session.mode == EditMode::Object {
            if hit.is_some() {
                mesh.select_all();
            } else if !core.shift_down {
                mesh.deselect_all();
            }
        } else {
            // Shift toggles; a plain click replaces selection without ambiguous fallbacks.
            let was_selected = hit.is_some_and(|h| match h.component {
                PickComponent::Vertex(i) => mesh.verts.get(i).is_some_and(|v| v.selected),
                PickComponent::Edge(a, b) => mesh.selected_edges.contains(&(a.min(b), a.max(b))),
                PickComponent::Face(i) => mesh.faces.get(i).is_some_and(|f| f.selected),
            });
            if !core.shift_down {
                mesh.deselect_all();
            }
            let selected = !core.shift_down || !was_selected;
            if let Some(hit) = hit {
                match hit.component {
                    PickComponent::Vertex(i) => {
                        if let Some(v) = mesh.verts.get_mut(i) {
                            v.selected = selected;
                        }
                        mesh.sync_face_selection_from_verts();
                    }
                    PickComponent::Edge(a, b) => {
                        let edge = (a.min(b), a.max(b));
                        if selected {
                            mesh.selected_edges.insert(edge);
                        } else {
                            mesh.selected_edges.remove(&edge);
                        }
                        for v in &mut mesh.verts {
                            v.selected = false;
                        }
                        for &(a, b) in &mesh.selected_edges {
                            for i in [a, b] {
                                if let Some(v) = mesh.verts.get_mut(i as usize) {
                                    v.selected = true;
                                }
                            }
                        }
                        mesh.sync_face_selection_from_verts();
                    }
                    PickComponent::Face(i) => {
                        if let Some(f) = mesh.faces.get_mut(i) {
                            f.selected = selected;
                        }
                        mesh.sync_vert_selection_from_faces();
                    }
                }
            }
        }
    }
    core.state.sync_selection();
}

// ---------------------------------------------------------------------------
// Backend wgpu
// ---------------------------------------------------------------------------

struct WgpuGfx {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    renderer3d: petunia_render_wgpu::Renderer,
    egui_ctx: egui::Context,
    egui_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
}

struct WgpuApp {
    core: Core,
    gfx: Option<WgpuGfx>,
    fps_acc: f32,
    fps_n: u32,
}

impl WgpuApp {
    fn new() -> Self {
        Self {
            core: Core::new(),
            gfx: None,
            fps_acc: 0.0,
            fps_n: 0,
        }
    }

    fn init_gfx(&mut self, event_loop: &ActiveEventLoop) -> Result<(), String> {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("Petunia3D"))
                .map_err(|error| format!("wgpu window: {error}"))?,
        );
        let size = window.inner_size();
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance
            .create_surface(Arc::clone(&window))
            .map_err(|error| format!("wgpu surface: {error}"))?;
        let adapter = pollster::block_on(pick_adapter(&instance, &surface))?;
        let info = adapter.get_info();
        self.core.state.render.backend_name = format!("wgpu {:?} {}", info.backend, info.name);
        eprintln!(
            "petunia3d: GPU: {} ({:?} via {:?}, driver {})",
            info.name, info.device_type, info.backend, info.driver_info
        );
        let (device, queue) = pollster::block_on(pick_device(&adapter))?;

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .or_else(|| caps.formats.first().copied())
            .ok_or_else(|| "wgpu: surface exposes no texture format".to_owned())?;
        let alpha_mode = caps
            .alpha_modes
            .first()
            .copied()
            .ok_or_else(|| "wgpu: surface exposes no alpha mode".to_owned())?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let renderer3d = petunia_render_wgpu::Renderer::new(&device, format);

        let egui_ctx = egui::Context::default();
        petunia_ui::apply_theme_to_egui(&petunia_config::Theme::load(), &egui_ctx);
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &*window,
            None,
            None,
            None,
        );
        let egui_renderer = egui_wgpu::Renderer::new(&device, format, None, 1, true);

        self.core.apply_dev_presets();
        self.gfx = Some(WgpuGfx {
            window,
            surface,
            device,
            queue,
            config,
            renderer3d,
            egui_ctx,
            egui_state,
            egui_renderer,
        });
        self.core.state.mark_dirty();
        Ok(())
    }

    fn redraw(&mut self) {
        let t0 = std::time::Instant::now();
        let Some(gfx) = self.gfx.as_mut() else { return };

        if let Some(rect) = self.core.state.ui.viewport_rect {
            if rect.width() > 1.0 && rect.height() > 1.0 {
                self.core.state.camera.aspect = rect.width() / rect.height();
            }
        } else {
            let s = gfx.window.inner_size();
            self.core.state.camera.aspect = s.width as f32 / s.height.max(1) as f32;
        }

        let raw_input = gfx.egui_state.take_egui_input(&gfx.window);
        let mut quit = false;
        let full_output = gfx.egui_ctx.run(raw_input, |ctx| {
            let mut act = petunia_ui::UiAction::none();
            petunia_ui::draw(
                ctx,
                &mut self.core.state,
                &self.core.tools,
                &mut self.core.registry,
                &mut act,
            );
            if let Some(ref info) = self.core.pending_recovery {
                if let Some(rec_act) =
                    petunia_ui::draw_recovery_dialog(ctx, &mut self.core.state, info)
                {
                    match rec_act {
                        petunia_ui::RecoveryAction::Recover
                        | petunia_ui::RecoveryAction::OpenSaved => {
                            self.core.pending_recovery = None;
                        }
                        petunia_ui::RecoveryAction::Discard => {
                            let _ = petunia_core::AutosaveService::discard_recovery(
                                info.main_project_path.as_deref(),
                            );
                            self.core.pending_recovery = None;
                        }
                    }
                }
            }
            quit = act.quit;
        });
        gfx.egui_state
            .handle_platform_output(&gfx.window, full_output.platform_output.clone());
        self.core.dispatch_events();
        self.core.tick_autosave();

        if let Some((nx, ny)) = self.core.state.ui.pending_pick.take() {
            handle_pick(&mut self.core, nx, ny);
        }
        if self.core.save_requested {
            self.core.save_requested = false;
            petunia_ui::save_project_dialog(&mut self.core.state, false);
        }

        gfx.renderer3d.update(
            &gfx.device,
            &gfx.queue,
            &self.core.state.project,
            &self.core.state.project.refs,
            &self.core.state.camera,
            self.core.state.shading,
            self.core.state.show_xray,
            self.core.state.show_triangulation,
        );
        gfx.renderer3d
            .upload_ref_pixels(&gfx.queue, &self.core.state.project.refs);

        let paint_jobs = gfx
            .egui_ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);
        let screen_desc = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [gfx.config.width, gfx.config.height],
            pixels_per_point: full_output.pixels_per_point,
        };

        let frame = match gfx.surface.get_current_texture() {
            Ok(f) => f,
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                gfx.surface.configure(&gfx.device, &gfx.config);
                return;
            }
            Err(wgpu::SurfaceError::OutOfMemory) => std::process::exit(1),
            Err(_) => return,
        };
        let view = frame.texture.create_view(&Default::default());

        gfx.renderer3d
            .resize(&gfx.device, gfx.config.width, gfx.config.height);

        let mut encoder = gfx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("petunia-encoder"),
            });

        {
            let Some(depth) = gfx.renderer3d.depth_view() else {
                eprintln!("petunia3d: render skipped: depth buffer unavailable");
                return;
            };
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("petunia-3d"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.117,
                            g: 0.117,
                            b: 0.133,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            if let Some(viewport) = petunia_core::viewport::PhysicalViewport::from_logical(
                self.core.state.ui.viewport_rect,
                self.core.state.ui.viewport_pixels_per_point,
                gfx.config.width,
                gfx.config.height,
            ) {
                pass.set_viewport(
                    viewport.x as f32,
                    viewport.y as f32,
                    viewport.width as f32,
                    viewport.height as f32,
                    0.0,
                    1.0,
                );
                pass.set_scissor_rect(viewport.x, viewport.y, viewport.width, viewport.height);
                gfx.renderer3d
                    .render(&mut pass, &self.core.state.project.refs);
            }
        }

        for (id, delta) in &full_output.textures_delta.set {
            gfx.egui_renderer
                .update_texture(&gfx.device, &gfx.queue, *id, delta);
        }
        let user_cmds = gfx.egui_renderer.update_buffers(
            &gfx.device,
            &gfx.queue,
            &mut encoder,
            &paint_jobs,
            &screen_desc,
        );
        {
            let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("petunia-egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            gfx.egui_renderer
                .render(&mut pass.forget_lifetime(), &paint_jobs, &screen_desc);
        }
        for id in &full_output.textures_delta.free {
            gfx.egui_renderer.free_texture(id);
        }

        gfx.queue
            .submit(user_cmds.into_iter().chain([encoder.finish()]));
        frame.present();

        self.update_stats(t0);
        if quit {
            let proj_path = self
                .core
                .state
                .project
                .project_path
                .as_deref()
                .map(std::path::Path::new);
            petunia_core::AutosaveService::remove_session_lock(proj_path);
            std::process::exit(0);
        }
    }

    fn update_stats(&mut self, t0: std::time::Instant) {
        let ms = t0.elapsed().as_secs_f32() * 1000.0;
        self.fps_acc += ms;
        self.fps_n += 1;
        let (v, t) = self.core.state.project.totals();
        // estimativa honesta de draws: malha+arestas por asset + refs + grid
        let draws = self
            .core
            .state
            .project
            .assets
            .iter()
            .filter(|a| a.visible)
            .count()
            * 2
            + self
                .core
                .state
                .project
                .refs
                .iter()
                .filter(|r| r.visible)
                .count()
            + 1;
        self.core.state.render.stats.tris = t;
        self.core.state.render.stats.verts = v;
        self.core.state.render.stats.draws = draws;
        if self.fps_acc >= 500.0 && self.fps_n > 0 {
            self.core.state.render.stats.fps = 1000.0 * self.fps_n as f32 / self.fps_acc;
            self.core.state.render.stats.frame_ms = self.fps_acc / self.fps_n as f32;
            self.fps_acc = 0.0;
            self.fps_n = 0;
        }
    }
}

async fn pick_adapter(
    instance: &wgpu::Instance,
    surface: &wgpu::Surface<'_>,
) -> Result<wgpu::Adapter, String> {
    let attempts = [
        (
            "high-performance",
            wgpu::PowerPreference::HighPerformance,
            true,
            false,
        ),
        ("low-power", wgpu::PowerPreference::LowPower, true, false),
        ("qualquer", wgpu::PowerPreference::None, true, false),
        ("software", wgpu::PowerPreference::None, true, true),
    ];
    for (name, power, with_surface, fallback) in attempts {
        let req = wgpu::RequestAdapterOptions {
            power_preference: power,
            compatible_surface: if with_surface { Some(surface) } else { None },
            force_fallback_adapter: fallback,
        };
        match instance.request_adapter(&req).await {
            Ok(a) => {
                if !with_surface && !a.is_surface_supported(surface) {
                    let i = a.get_info();
                    eprintln!(
                        "petunia3d: adapter {:?} ({}) sem suporte à janela, tentando próximo",
                        i.backend, i.name
                    );
                    continue;
                }
                eprintln!("petunia3d: adapter via tentativa '{name}'");
                return Ok(a);
            }
            Err(e) => eprintln!("petunia3d: tentativa '{name}' falhou: {e:?}"),
        }
    }
    Err("No compatible wgpu adapter. Use SIMPLE3D_BACKEND=gl or PETUNIA_BACKEND=gl.".to_owned())
}

async fn pick_device(adapter: &wgpu::Adapter) -> Result<(wgpu::Device, wgpu::Queue), String> {
    let mk = |limits: wgpu::Limits| wgpu::DeviceDescriptor {
        label: Some("petunia3d"),
        required_features: wgpu::Features::empty(),
        required_limits: limits,
        memory_hints: Default::default(),
        trace: Default::default(),
    };
    match adapter.request_device(&mk(wgpu::Limits::default())).await {
        Ok(d) => Ok(d),
        Err(e) => {
            eprintln!("petunia3d: limits padrão recusados ({e}), usando downlevel");
            adapter
                .request_device(&mk(wgpu::Limits::downlevel_defaults()))
                .await
                .map_err(|error| {
                    format!("wgpu device creation failed with downlevel limits: {error}")
                })
        }
    }
}

impl ApplicationHandler for WgpuApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.gfx.is_none() {
            if let Err(error) = self.init_gfx(event_loop) {
                self.core.state.set_status(error);
                event_loop.exit();
                return;
            }
            if let Some(g) = self.gfx.as_ref() {
                g.window.request_redraw();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(gfx) = self.gfx.as_mut() else { return };

        let is_shortcut_key = matches!(
            &event,
            WindowEvent::KeyboardInput {
                event: winit::event::KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(
                        WKey::Tab
                            | WKey::Digit0
                            | WKey::Digit1
                            | WKey::Digit2
                            | WKey::Digit3
                            | WKey::Digit4
                    ),
                    ..
                },
                ..
            }
        );

        let resp = gfx.egui_state.on_window_event(&gfx.window, &event);
        if resp.repaint {
            gfx.window.request_redraw();
        }
        if resp.consumed && (!is_shortcut_key || gfx.egui_ctx.wants_keyboard_input()) {
            if matches!(event, WindowEvent::RedrawRequested) {
                self.redraw();
            }
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                let PhysicalSize { width, height } = size;
                if width > 0 && height > 0 {
                    gfx.config.width = width;
                    gfx.config.height = height;
                    gfx.surface.configure(&gfx.device, &gfx.config);
                }
                gfx.window.request_redraw();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.core
                    .on_mouse_input(state == ElementState::Pressed, button);
                gfx.window.request_redraw();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let y = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => y * 40.0,
                    winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32,
                };
                let _ = y; // O viewport processa scroll com foco e modalidade.
                gfx.window.request_redraw();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.core.last_mouse = Some((position.x, position.y));
                gfx.window.request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    self.core.on_key(event.physical_key);
                    gfx.window.request_redraw();
                }
            }
            WindowEvent::ModifiersChanged(m) => {
                let s = m.state();
                self.core
                    .on_modifiers(s.shift_key(), s.control_key(), s.alt_key());
            }
            WindowEvent::RedrawRequested => self.redraw(),
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _id: winit::event::DeviceId,
        _event: DeviceEvent,
    ) {
    }

    /// Render-on-demand (§33): só redesenha se algo mudou.
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let spin = std::env::var_os("SIMPLE3D_SPIN").is_some();
        if spin || self.core.state.consume_dirty() {
            if let Some(g) = self.gfx.as_ref() {
                g.window.request_redraw();
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Backend OpenGL puro (glow + glutin)
// ---------------------------------------------------------------------------

struct GlGfx {
    gl_window: petunia_render_gl::GlWindow,
    egui_ctx: egui::Context,
    egui_state: egui_winit::State,
    egui_painter: egui_glow::Painter,
    gl_renderer: petunia_render_gl::GlRenderer,
}

struct GlApp {
    core: Core,
    gfx: Option<GlGfx>,
    fps_acc: f32,
    fps_n: u32,
    shot_path: Option<String>,
    shot_frames: u32,
    shot_taken: u32,
}

impl GlApp {
    fn new() -> Self {
        Self {
            core: Core::new(),
            gfx: None,
            fps_acc: 0.0,
            fps_n: 0,
            shot_path: std::env::var("PETUNIA_SCREENSHOT").ok(),
            shot_frames: std::env::var("PETUNIA_SHOT_FRAMES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            shot_taken: 0,
        }
    }

    fn init_gfx(&mut self, event_loop: &ActiveEventLoop) -> Result<(), String> {
        // Todo conhecimento GL vive em render-gl (§27); aqui só UI + loop.
        // PETUNIA_SHOT_SIZE=WxH fixa a janela no modo screenshot.
        let attrs = if let Some((w, h)) = std::env::var("PETUNIA_SHOT_SIZE").ok().and_then(|s| {
            let (w, h) = s.split_once('x')?;
            Some((w.parse::<u32>().ok()?, h.parse::<u32>().ok()?))
        }) {
            Window::default_attributes()
                .with_title("Petunia3D (OpenGL)")
                .with_inner_size(winit::dpi::LogicalSize::new(w, h))
        } else {
            Window::default_attributes().with_title("Petunia3D (OpenGL)")
        };
        let gl_window = petunia_render_gl::GlWindow::create(event_loop, attrs)?;
        let caps = gl_window.caps();
        self.core.state.render.backend_name =
            format!("OpenGL {} ({})", caps.gl_version, caps.renderer);
        eprintln!("petunia3d: OpenGL: {} | {}", caps.gl_version, caps.renderer);

        let egui_ctx = egui::Context::default();
        petunia_ui::apply_theme_to_egui(&petunia_config::Theme::load(), &egui_ctx);
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            gl_window.window(),
            None,
            None,
            None,
        );
        let egui_painter = egui_glow::Painter::new(Arc::clone(gl_window.gl()), "", None, false)
            .map_err(|error| format!("OpenGL UI painter: {error}"))?;
        let gl_renderer = petunia_render_gl::GlRenderer::new(Arc::clone(gl_window.gl()))?;

        self.core.apply_dev_presets();
        self.gfx = Some(GlGfx {
            gl_window,
            egui_ctx,
            egui_state,
            egui_painter,
            gl_renderer,
        });
        self.core.state.mark_dirty();
        Ok(())
    }

    fn redraw(&mut self) {
        let t0 = std::time::Instant::now();
        let Some(g) = self.gfx.as_mut() else { return };

        if let Some(rect) = self.core.state.ui.viewport_rect {
            if rect.width() > 1.0 && rect.height() > 1.0 {
                self.core.state.camera.aspect = rect.width() / rect.height();
            }
        } else {
            let s = g.gl_window.window().inner_size();
            self.core.state.camera.aspect = s.width as f32 / s.height.max(1) as f32;
        }
        let (w, h) = g.gl_window.size();

        let raw_input = g.egui_state.take_egui_input(g.gl_window.window());
        let mut quit = false;
        let full_output = g.egui_ctx.run(raw_input, |ctx| {
            let mut act = petunia_ui::UiAction::none();
            petunia_ui::draw(
                ctx,
                &mut self.core.state,
                &self.core.tools,
                &mut self.core.registry,
                &mut act,
            );
            if let Some(ref info) = self.core.pending_recovery {
                if let Some(rec_act) =
                    petunia_ui::draw_recovery_dialog(ctx, &mut self.core.state, info)
                {
                    match rec_act {
                        petunia_ui::RecoveryAction::Recover
                        | petunia_ui::RecoveryAction::OpenSaved => {
                            self.core.pending_recovery = None;
                        }
                        petunia_ui::RecoveryAction::Discard => {
                            let _ = petunia_core::AutosaveService::discard_recovery(
                                info.main_project_path.as_deref(),
                            );
                            self.core.pending_recovery = None;
                        }
                    }
                }
            }
            quit = act.quit;
        });
        g.egui_state
            .handle_platform_output(g.gl_window.window(), full_output.platform_output.clone());
        self.core.dispatch_events();
        self.core.tick_autosave();

        if let Some((nx, ny)) = self.core.state.ui.pending_pick.take() {
            handle_pick(&mut self.core, nx, ny);
        }
        if self.core.save_requested {
            self.core.save_requested = false;
            petunia_ui::save_project_dialog(&mut self.core.state, false);
        }

        let paint_jobs = g
            .egui_ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);
        let t_draw = std::time::Instant::now();
        g.gl_renderer.draw(&self.core.state, w, h);
        let t_paint = std::time::Instant::now();
        g.egui_painter.paint_and_update_textures(
            [w, h],
            full_output.pixels_per_point,
            &paint_jobs,
            &full_output.textures_delta,
        );
        let t_swap = std::time::Instant::now();
        // modo screenshot: captura o framebuffer e sai (verificação headless)
        if let Some(path) = self.shot_path.clone() {
            self.shot_taken += 1;
            if self.shot_taken >= self.shot_frames {
                capture_screenshot(g, &path, w, h);
                std::process::exit(0);
            }
        }
        g.gl_window.swap();
        let t_end = std::time::Instant::now();
        if std::env::var_os("SIMPLE3D_PHASES").is_some() && self.fps_n < 5 {
            eprintln!(
                "phases ui={:.1}ms draw={:.1}ms paint={:.1}ms swap={:.1}ms",
                (t_draw - t0).as_secs_f32() * 1000.0,
                (t_paint - t_draw).as_secs_f32() * 1000.0,
                (t_swap - t_paint).as_secs_f32() * 1000.0,
                (t_end - t_swap).as_secs_f32() * 1000.0,
            );
        }

        let ms = t0.elapsed().as_secs_f32() * 1000.0;
        self.fps_acc += ms;
        self.fps_n += 1;
        let (v, t) = self.core.state.project.totals();
        self.core.state.render.stats.tris = t;
        self.core.state.render.stats.verts = v;
        self.core.state.render.stats.draws = self
            .core
            .state
            .project
            .assets
            .iter()
            .filter(|a| a.visible)
            .count()
            * 2
            + self
                .core
                .state
                .project
                .refs
                .iter()
                .filter(|r| r.visible)
                .count()
            + 1;
        if self.fps_acc >= 500.0 && self.fps_n > 0 {
            self.core.state.render.stats.fps = 1000.0 * self.fps_n as f32 / self.fps_acc;
            self.core.state.render.stats.frame_ms = self.fps_acc / self.fps_n as f32;
            self.fps_acc = 0.0;
            self.fps_n = 0;
        }
        if std::env::var_os("SIMPLE3D_STATS").is_some() {
            eprintln!(
                "petunia3d: {:.0} fps {:.1} ms",
                self.core.state.render.stats.fps, self.core.state.render.stats.frame_ms
            );
        }

        if quit {
            let proj_path = self
                .core
                .state
                .project
                .project_path
                .as_deref()
                .map(std::path::Path::new);
            petunia_core::AutosaveService::remove_session_lock(proj_path);
            std::process::exit(0);
        }
    }
}

impl ApplicationHandler for GlApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.gfx.is_none() {
            if let Err(error) = self.init_gfx(event_loop) {
                self.core.state.set_status(error);
                event_loop.exit();
                return;
            }
            if let Some(g) = self.gfx.as_ref() {
                g.gl_window.window().request_redraw();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(g) = self.gfx.as_mut() else { return };

        let is_shortcut_key = matches!(
            &event,
            WindowEvent::KeyboardInput {
                event: winit::event::KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(
                        WKey::Tab
                            | WKey::Digit0
                            | WKey::Digit1
                            | WKey::Digit2
                            | WKey::Digit3
                            | WKey::Digit4
                    ),
                    ..
                },
                ..
            }
        );

        let resp = g.egui_state.on_window_event(g.gl_window.window(), &event);
        if resp.repaint {
            g.gl_window.window().request_redraw();
        }
        if resp.consumed && (!is_shortcut_key || g.egui_ctx.wants_keyboard_input()) {
            if matches!(event, WindowEvent::RedrawRequested) {
                self.redraw();
            }
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                g.gl_window.resize(size.width, size.height);
                g.gl_window.window().request_redraw();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.core
                    .on_mouse_input(state == ElementState::Pressed, button);
                g.gl_window.window().request_redraw();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let y = match delta {
                    winit::event::MouseScrollDelta::LineDelta(_, y) => y * 40.0,
                    winit::event::MouseScrollDelta::PixelDelta(p) => p.y as f32,
                };
                let _ = y; // O viewport processa scroll com foco e modalidade.
                g.gl_window.window().request_redraw();
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.core.last_mouse = Some((position.x, position.y));
                g.gl_window.window().request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    self.core.on_key(event.physical_key);
                    g.gl_window.window().request_redraw();
                }
            }
            WindowEvent::ModifiersChanged(m) => {
                let s = m.state();
                self.core
                    .on_modifiers(s.shift_key(), s.control_key(), s.alt_key());
            }
            WindowEvent::RedrawRequested => self.redraw(),
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _id: winit::event::DeviceId,
        _event: DeviceEvent,
    ) {
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let spin = std::env::var_os("SIMPLE3D_SPIN").is_some() || self.shot_path.is_some();
        if spin || self.core.state.consume_dirty() {
            if let Some(g) = self.gfx.as_ref() {
                g.gl_window.window().request_redraw();
            }
        }
    }
}

// ---------------------------------------------------------------------------

/// Captura o framebuffer atual em PNG (glReadPixels + flip vertical).
fn capture_screenshot(g: &GlGfx, path: &str, w: u32, h: u32) {
    use egui_glow::glow::{HasContext as _, PixelPackData};
    let gl = g.gl_window.gl();
    let mut buf = vec![0u8; (w * h * 4) as usize];
    unsafe {
        gl.read_pixels(
            0,
            0,
            w as i32,
            h as i32,
            egui_glow::glow::RGBA,
            egui_glow::glow::UNSIGNED_BYTE,
            PixelPackData::Slice(Some(&mut buf)),
        );
    }
    match image::RgbaImage::from_raw(w, h, buf) {
        Some(img) => {
            let img = image::imageops::flip_vertical(&img);
            match img.save(path) {
                Ok(()) => eprintln!("petunia3d: screenshot {path} ({w}x{h})"),
                Err(e) => eprintln!("petunia3d: screenshot err: {e}"),
            }
        }
        None => eprintln!("petunia3d: screenshot: buffer inválido"),
    }
}

/// Smoke test scriptado (headless): exercita domínio + módulos + projeto
/// sem janela. Falha com `Err` legível. Uso: `petunia3d --smoke-test`.
pub fn smoke_test() -> anyhow::Result<()> {
    use anyhow::{ensure, Context as _};
    use petunia_mesh::Mesh;

    let mut core = Core::new();
    let s = &mut core.state;

    // primitivas
    s.checkpoint("add");
    s.project.add("Cube2", Mesh::cube(2.0));
    s.project.add("Capsule", Mesh::capsule(8, 0.5, 2.0));
    s.project.add("Sphere", Mesh::sphere_low(8, 5, 1.0));
    ensure!(s.project.assets.len() == 4, "assets após add");
    eprintln!("SMOKE primitives ok ({} assets)", s.project.assets.len());

    // seleção + ops com undo/redo
    s.project
        .active_mesh_mut()
        .context("mesh ativa")?
        .select_all();
    s.checkpoint("extrude");
    s.project.active_mesh_mut().unwrap().extrude_selected(0.4);
    s.checkpoint("inset");
    s.project.active_mesh_mut().unwrap().inset_selected(0.2);
    s.checkpoint("subdivide");
    s.project.active_mesh_mut().unwrap().subdivide_selected();
    // Bevel has a stricter supported topology than arbitrary subdivided spheres.
    let mut bevel_fixture = petunia_mesh::Mesh::cube(2.0);
    bevel_fixture.selected_edges.insert((0, 1));
    let (ok, skipped) = bevel_fixture.bevel_selected(0.05);
    ensure!(ok == 1 && skipped == 0, "bevel fixture");
    let (topology, defects) = petunia_mesh::HalfEdgeMesh::from_mesh(&bevel_fixture);
    ensure!(
        defects.is_empty() && topology.is_closed_manifold(),
        "bevel closed manifold"
    );
    s.checkpoint("mirror");
    s.project.active_mesh_mut().unwrap().mirror(0, 0.001);
    s.checkpoint("pushpull");
    s.project.active_mesh_mut().unwrap().push_pull(0.2);
    // novas operações de geometria da Gauntlet
    let m = s.project.active_mesh_mut().unwrap();
    m.slice_plane(glam::Vec3::ZERO, glam::Vec3::Y, true);
    m.flip_normals();
    m.recalculate_normals();
    let edge_for_dissolve = {
        m.faces.iter().find_map(|f| {
            let k = f.verts.len();
            (0..k).find_map(|i| {
                let a = f.verts[i];
                let b = f.verts[(i + 1) % k];
                if m.edge_faces(a, b).len() == 2 {
                    Some((a, b))
                } else {
                    None
                }
            })
        })
    };
    if let Some(edge) = edge_for_dissolve {
        m.selected_edges.insert(edge);
        m.dissolve_selected();
    }
    let rep = m.validate_topology();
    ensure!(
        rep.isolated_vertex_count == 0,
        "topology isolated vertices cleaned"
    );

    // sweep
    let mut sweep_mesh = petunia_mesh::Mesh::default();
    let sweep_prof = [[-0.2, -0.2], [0.2, -0.2], [0.2, 0.2], [-0.2, 0.2]];
    let path = [
        glam::vec3(0.0, 0.0, 0.0),
        glam::vec3(0.0, 1.0, 0.5),
        glam::vec3(1.0, 2.0, 1.0),
    ];
    sweep_mesh
        .sweep(&sweep_prof, &path, true)
        .map_err(|e| anyhow::anyhow!(e))?;
    ensure!(sweep_mesh.tri_count() > 0, "sweep gerou geometria");

    let (v, f) = s.project.totals();
    ensure!(v > 8 && f > 6, "malha cresceu v={v} f={f}");
    eprintln!("SMOKE ops ok (v={v} f={f})");

    ensure!(s.undo(), "undo 1");
    ensure!(s.undo(), "undo 2");
    ensure!(s.redo(), "redo 1");
    eprintln!("SMOKE undo/redo ok");

    // draw profile -> extrude + revolve (nível domínio)
    let square = [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]];
    let pm = Mesh::from_polygon(&square, 0.5).map_err(|e| anyhow::anyhow!(e))?;
    ensure!(pm.tri_count() >= 2, "polígono triangulou");
    s.project.add("Profile", pm);
    let prof = [[0.0, 0.0], [1.0, 0.0], [1.0, 2.0], [0.0, 2.0]];
    let rm = Mesh::revolve(&prof, 10).map_err(|e| anyhow::anyhow!(e))?;
    ensure!(!rm.faces.is_empty(), "revolve gerou faces");
    s.project.add("Revolved", rm);
    eprintln!("SMOKE profile ok");

    // uv + paint
    s.project.active_mesh_mut().unwrap().project_planar();
    UvModule::move_selected(s, 0.1, 0.0);
    ensure!(
        s.project
            .active_mesh_mut()
            .unwrap()
            .faces
            .iter()
            .all(|f| f.uv.len() == f.verts.len()),
        "uv invariante"
    );
    PaintModule::fill_selection(s);
    PaintModule::ensure_canvas(s);
    PaintModule::canvas_brush(s, 10, 10, false);
    PaintModule::canvas_fill(s);
    eprintln!("SMOKE uv/paint ok");

    // projeto save/load + export
    let dir = std::env::temp_dir();
    let pp = dir.join("petunia_smoke.petunia");
    petunia_project::format::save(&s.project, &pp).context("save")?;
    let q = petunia_project::format::load(&pp).context("load")?;
    ensure!(q.assets.len() == s.project.assets.len(), "roundtrip assets");
    let glb = petunia_project::export::export_gltf(&s.project, &[0]).context("glb")?;
    ensure!(glb.starts_with(b"glTF"), "magic glb");
    let gp = dir.join("petunia_smoke.glb");
    std::fs::write(&gp, &glb).context("write glb")?;
    eprintln!(
        "SMOKE project ok ({} assets, glb {} bytes)",
        q.assets.len(),
        glb.len()
    );
    let _ = std::fs::remove_file(&pp);
    let _ = std::fs::remove_file(&gp);

    // eventos fluem?
    s.emit_mesh_changed();
    core.dispatch_events();
    eprintln!("SMOKE OK");
    Ok(())
}

async fn probe_wgpu() -> bool {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    for power in [
        wgpu::PowerPreference::HighPerformance,
        wgpu::PowerPreference::LowPower,
    ] {
        if instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: power,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .is_ok()
        {
            return true;
        }
    }
    instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::None,
            compatible_surface: None,
            force_fallback_adapter: true,
        })
        .await
        .is_ok()
}

/// Ponto de entrada chamado pelo binário.
pub fn run() {
    if let Err(error) = run_event_loop() {
        eprintln!("petunia3d: {error}");
        std::process::exit(1);
    }
}

fn run_event_loop() -> Result<(), String> {
    // Logs: RUST_LOG=wgpu_hal=debug diagnostica backend que falhou.
    // Sem RUST_LOG, os ERRORs internos da sonda wgpu/EGL (normais em
    // máquina sem Vulkan) são silenciados para não assustar.
    {
        let mut b = env_logger::Builder::from_default_env();
        if std::env::var_os("RUST_LOG").is_none() {
            b.filter_module("wgpu_hal", log::LevelFilter::Off);
            b.filter_module("wgpu_core", log::LevelFilter::Off);
            b.filter_module("egui_glow", log::LevelFilter::Off);
        }
        if let Err(error) = b.try_init() {
            eprintln!("petunia3d: logger initialization: {error}");
        }
    }
    let forced = std::env::var("PETUNIA_BACKEND")
        .or_else(|_| std::env::var("SIMPLE3D_BACKEND"))
        .unwrap_or_default();
    let use_gl = match forced.as_str() {
        "gl" | "opengl" => true,
        "wgpu" => false,
        _ => {
            if forced.is_empty() {
                eprintln!("petunia3d: procurando GPU via wgpu...");
            } else {
                eprintln!("petunia3d: backend '{forced}' desconhecido, detectando...");
            }
            !pollster::block_on(probe_wgpu())
        }
    };
    let event_loop = EventLoop::new().map_err(|error| format!("Event loop: {error}"))?;
    // Render-on-demand: Wait + redraw explícito (sem loop contínuo em idle).
    // SIMPLE3D_SPIN=1 volta ao loop contínuo (diagnóstico/benchmark).
    if std::env::var_os("SIMPLE3D_SPIN").is_some() {
        event_loop.set_control_flow(ControlFlow::Poll);
    } else {
        event_loop.set_control_flow(ControlFlow::Wait);
    }
    if use_gl {
        eprintln!("petunia3d: backend OpenGL puro (glow).");
        let mut app = GlApp::new();
        event_loop
            .run_app(&mut app)
            .map_err(|error| format!("Application event loop: {error}"))?;
        if app.gfx.is_none() {
            return Err(app.core.state.ui.status.clone());
        }
    } else {
        eprintln!("petunia3d: backend wgpu.");
        let mut app = WgpuApp::new();
        event_loop
            .run_app(&mut app)
            .map_err(|error| format!("Application event loop: {error}"))?;
        if app.gfx.is_none() {
            return Err(app.core.state.ui.status.clone());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use petunia_core::SelectMode;

    /// NDC aproximado do centro da face frontal do cubo default.
    /// (câmera default: yaw 0.7/pitch 0.5 — o centro da tela atinge o cubo)
    #[test]
    fn pick_center_selects_face() {
        let mut core = Core::new();
        core.state.select_mode = SelectMode::Face;
        handle_pick(&mut core, 0.0, 0.0);
        let m = core.state.project.active_mesh_mut().unwrap();
        assert!(
            m.selected_face_count() >= 1,
            "clique no centro deve selecionar face"
        );
    }

    #[test]
    fn pick_empty_space_deselects() {
        let mut core = Core::new();
        core.state.project.active_mesh_mut().unwrap().select_all();
        // canto longe do cubo
        handle_pick(&mut core, 0.99, 0.99);
        let m = core.state.project.active_mesh_mut().unwrap();
        assert_eq!(m.selected_face_count(), 0);
        assert_eq!(m.selected_vert_count(), 0);
    }

    #[test]
    fn key_extrude_starts_preview_without_mutating() {
        use winit::keyboard::KeyCode as WKey;
        let mut core = Core::new();
        core.state.project.active_mesh_mut().unwrap().select_all();
        let before = core.state.project.active_mesh_mut().unwrap().faces.len();
        core.on_key(PhysicalKey::Code(WKey::KeyE));
        let after = core.state.project.active_mesh_mut().unwrap().faces.len();
        assert_eq!(after, before, "E apenas inicia interação");
        assert_eq!(
            core.state.pending_modal,
            Some(petunia_core::ModalKind::Extrude)
        );
        assert!(!core.state.project.undo.can_undo());
    }

    #[test]
    fn key_ctrl_z_undoes() {
        let mut core = Core::new();
        core.state.mode = EditMode::Edit;
        let mesh = core.state.project.active_mesh_mut().unwrap();
        mesh.deselect_all();
        mesh.faces[0].selected = true;
        mesh.sync_vert_selection_from_faces();
        let before = mesh.faces.len();
        core.state
            .begin_modal(petunia_core::ModalKind::Extrude)
            .unwrap();
        core.state.update_modal(glam::Vec3::ZERO, 0.5).unwrap();
        core.state.commit_modal();
        assert!(core.state.project.active_mesh().unwrap().faces.len() > before);
        core.ctrl_down = true;
        core.on_key(PhysicalKey::Code(WKey::KeyZ));
        assert_eq!(
            core.state.project.active_mesh().unwrap().faces.len(),
            before
        );
    }

    #[test]
    fn draw_profile_click_adds_point_in_front_view() {
        use petunia_core::ViewPreset;
        let mut core = Core::new();
        core.state.camera.set_preset(ViewPreset::Front);
        core.set_tool("draw_profile", None);
        assert!(core.state.profile.points.is_empty());
        handle_pick(&mut core, 0.0, 0.0);
        assert_eq!(core.state.profile.points.len(), 1);
        handle_pick(&mut core, 0.5, 0.5);
        assert_eq!(core.state.profile.points.len(), 2);
    }

    #[test]
    fn test_tab_and_selection_keys() {
        use winit::keyboard::KeyCode as WKey;
        let mut core = Core::new();
        assert_eq!(core.state.mode, EditMode::Object);

        // Tab -> alterna para Edit
        core.on_key(PhysicalKey::Code(WKey::Tab));
        assert_eq!(core.state.mode, EditMode::Edit);

        // Tab -> alterna de volta para Object
        core.on_key(PhysicalKey::Code(WKey::Tab));
        assert_eq!(core.state.mode, EditMode::Object);

        // Tecla 1 -> alterna para Edit + Vertex
        core.on_key(PhysicalKey::Code(WKey::Digit1));
        assert_eq!(core.state.mode, EditMode::Edit);
        assert_eq!(core.state.select_mode, SelectMode::Vertex);

        // Tecla 2 -> alterna para Edit + Edge
        core.on_key(PhysicalKey::Code(WKey::Digit2));
        assert_eq!(core.state.mode, EditMode::Edit);
        assert_eq!(core.state.select_mode, SelectMode::Edge);

        // Tecla 3 -> alterna para Edit + Face
        core.on_key(PhysicalKey::Code(WKey::Digit3));
        assert_eq!(core.state.mode, EditMode::Edit);
        assert_eq!(core.state.select_mode, SelectMode::Face);

        // Tecla 0 -> alterna para Object
        core.on_key(PhysicalKey::Code(WKey::Digit0));
        assert_eq!(core.state.mode, EditMode::Object);
    }
}

#[cfg(test)]
mod camera_shortcut_tests {
    use super::*;
    use petunia_core::{Projection, ViewPreset};

    #[test]
    fn numpad_and_letter_toggle_preserve_scale_and_cancel_frame_animation() {
        let mut core = Core::new();
        core.state.ui.keybinds = petunia_config::keybinds::Keybinds::defaults();
        let height = core.state.camera.visible_height();
        for key in [WKey::Numpad5, WKey::KeyO] {
            let before = core.state.camera.proj;
            core.state.camera_frame =
                Some((core.state.camera.clone(), core.state.camera.clone(), 0.0));
            core.on_key(PhysicalKey::Code(key));
            assert_ne!(core.state.camera.proj, before);
            assert!((core.state.camera.visible_height() - height).abs() < 1e-5);
            assert!(core.state.camera_frame.is_none());
        }
    }

    #[test]
    fn numpad_with_ctrl_exposes_all_six_orthographic_views() {
        let mut core = Core::new();
        for (key, ctrl, view) in [
            (WKey::Numpad1, false, ViewPreset::Front),
            (WKey::Numpad1, true, ViewPreset::Back),
            (WKey::Numpad3, false, ViewPreset::Right),
            (WKey::Numpad3, true, ViewPreset::Left),
            (WKey::Numpad7, false, ViewPreset::Top),
            (WKey::Numpad7, true, ViewPreset::Bottom),
        ] {
            core.ctrl_down = ctrl;
            core.on_key(PhysicalKey::Code(key));
            assert_eq!(core.state.camera.proj, Projection::Ortho);
            assert_eq!(core.state.camera.view_preset(), Some(view));
        }
    }
}
