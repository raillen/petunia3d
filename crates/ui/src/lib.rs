//! Petunia3D UI (egui): header com pílulas de workspace, toolbar,
//! Asset Library, painéis por workspace, viewport com overlays e status.
//!
//! Regra de borrow: rótulos `state.t()` (owned String) são pré-calculados
//! antes dos closures; mutações via índices, nunca com iterator vivo.

use petunia_core::Projection;
use petunia_core::{AppState, EditMode, ModuleRegistry, RefAxis, Workspace};
use petunia_mesh::Mesh;
use petunia_module_model::ToolRegistry;
use petunia_project::{export, format};

pub mod camera_controls;
mod cutting;
pub mod gizmo;
pub mod icons;
#[cfg(test)]
mod modal_tests;
mod modal_viewport;
mod tool_fields;
mod viewport_interaction;

pub struct UiAction {
    pub quit: bool,
}

impl UiAction {
    pub fn none() -> Self {
        Self { quit: false }
    }
}

pub fn draw(
    ctx: &egui::Context,
    state: &mut AppState,
    tools: &ToolRegistry,
    registry: &mut ModuleRegistry,
    action: &mut UiAction,
) {
    top_bar(ctx, state, action);
    status_bar(ctx, state, tools);
    left_toolbar(ctx, state, tools);
    right_panel(ctx, state, tools, registry);
    viewport(ctx, state);
}

// ---------------------------------------------------------------- top bar

fn top_bar(ctx: &egui::Context, state: &mut AppState, action: &mut UiAction) {
    egui::TopBottomPanel::top("top").show(ctx, |ui| {
        ui.add_enabled_ui(!state.is_interacting(), |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.strong("Petunia3D");
                ui.separator();
                ui.menu_button(state.t("menu.file"), |ui| {
                    for (key, operation) in [
                        ("file.new", 0),
                        ("file.open_project", 1),
                        ("file.save", 2),
                        ("file.save_as", 3),
                        ("file.import_obj", 4),
                        ("file.quit", 5),
                    ] {
                        if ui.button(state.t(key)).clicked() {
                            match operation {
                                0 => new_project(state),
                                1 => open_project_dialog(state),
                                2 => save_project_dialog(state, false),
                                3 => save_project_dialog(state, true),
                                4 => import_obj_dialog(state),
                                _ => action.quit = true,
                            }
                            ui.close();
                        }
                    }
                });
                ui.menu_button(state.t("menu.edit"), |ui| {
                    if ui
                        .add_enabled(
                            state.undo.can_undo(),
                            egui::Button::new(state.t("edit.undo")),
                        )
                        .clicked()
                    {
                        state.undo();
                        ui.close();
                    }
                    if ui
                        .add_enabled(
                            state.undo.can_redo(),
                            egui::Button::new(state.t("edit.redo")),
                        )
                        .clicked()
                    {
                        state.redo();
                        ui.close();
                    }
                });
                ui.menu_button(
                    if state.i18n.lang == "en" {
                        "Display"
                    } else {
                        "Exibição"
                    },
                    |ui| {
                        for (shading, key) in [
                            (petunia_render::Shading::Solid, "shading.solid"),
                            (petunia_render::Shading::Smooth, "shading.smooth"),
                            (petunia_render::Shading::Unlit, "shading.unlit"),
                            (petunia_render::Shading::Wireframe, "shading.wire"),
                        ] {
                            if ui
                                .selectable_label(state.shading == shading, state.t(key))
                                .clicked()
                            {
                                state.shading = shading;
                                state.mark_dirty();
                                ui.close();
                            }
                        }
                        let label = state.t("shading.textured");
                        if ui.checkbox(&mut state.textured, label).changed() {
                            state.mark_dirty();
                        }
                        ui.checkbox(&mut state.show_perf, "Performance");
                        ui.separator();
                        for lang in petunia_config::I18n::available() {
                            if ui
                                .selectable_label(state.i18n.lang == lang, &lang)
                                .clicked()
                            {
                                state.i18n.set_lang(&lang);
                                state.mark_dirty();
                            }
                        }
                    },
                );
                if ui.button(state.t("menu.help")).clicked() {
                    state.show_help = !state.show_help;
                }
            });
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                for workspace in Workspace::all() {
                    if ui
                        .selectable_label(state.workspace == workspace, state.t(workspace.key()))
                        .clicked()
                    {
                        state.workspace = workspace;
                        state.mark_dirty();
                    }
                }
                ui.separator();
                let current = if state.mode == EditMode::Object {
                    state.t("mode.object")
                } else {
                    match state.select_mode {
                        petunia_core::SelectMode::Vertex => if state.i18n.lang == "en" {
                            "Vertices · 1"
                        } else {
                            "Vértices · 1"
                        }
                        .to_owned(),
                        petunia_core::SelectMode::Edge => if state.i18n.lang == "en" {
                            "Edges · 2"
                        } else {
                            "Arestas · 2"
                        }
                        .to_owned(),
                        petunia_core::SelectMode::Face => "Faces · 3".to_owned(),
                    }
                };
                egui::ComboBox::from_id_salt("selection.level")
                    .selected_text(current)
                    .width(112.0)
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(
                                state.mode == EditMode::Object,
                                format!("{} · 4", state.t("mode.object")),
                            )
                            .clicked()
                        {
                            state.mode = EditMode::Object;
                            state.mark_dirty();
                        }
                        for (mode, label) in [
                            (
                                petunia_core::SelectMode::Vertex,
                                if state.i18n.lang == "en" {
                                    "Vertices · 1"
                                } else {
                                    "Vértices · 1"
                                },
                            ),
                            (
                                petunia_core::SelectMode::Edge,
                                if state.i18n.lang == "en" {
                                    "Edges · 2"
                                } else {
                                    "Arestas · 2"
                                },
                            ),
                            (petunia_core::SelectMode::Face, "Faces · 3"),
                        ] {
                            if ui
                                .selectable_label(
                                    state.mode == EditMode::Edit && state.select_mode == mode,
                                    label,
                                )
                                .clicked()
                            {
                                state.mode = EditMode::Edit;
                                state.select_mode = mode;
                                state.sync_selection();
                            }
                        }
                    });
            });
            ui.separator();
            camera_controls::draw(ui, state);
        });
    });
}

pub fn new_project(state: &mut AppState) {
    state.project = petunia_project::Project::new();
    state.palette = state.project.palette.clone();
    state.undo.clear();
    state.refs.clear();
    state.uv_selected.clear();
    state.project_path = None;
    state.sync_selection();
    state.set_status("new".to_string());
}

pub fn open_project_dialog(state: &mut AppState) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("Petunia", &["petunia"])
        .pick_file()
    {
        match format::load(&path) {
            Ok(p) => {
                state.palette = p.palette.clone();
                state.project = p;
                state.undo.clear();
                state.uv_selected.clear();
                state.project_path = Some(path.to_string_lossy().to_string());
                state.events.emit(petunia_core::AppEvent::ProjectLoaded);
                state.sync_selection();
                state.set_status(format!("open {}", path.display()));
            }
            Err(e) => state.set_status(format!("open err: {e}")),
        }
    }
}

pub fn save_project_dialog(state: &mut AppState, save_as: bool) {
    state.project.palette = state.palette.clone();
    let path = if !save_as {
        state.project_path.clone().map(std::path::PathBuf::from)
    } else {
        None
    };
    let path = path.or_else(|| {
        rfd::FileDialog::new()
            .add_filter("Petunia", &["petunia"])
            .set_file_name("project.petunia")
            .save_file()
    });
    if let Some(path) = path {
        match format::save(&state.project, &path) {
            Ok(()) => {
                state.project_path = Some(path.to_string_lossy().to_string());
                state.set_status(format!("saved {}", path.display()));
            }
            Err(e) => state.set_status(format!("save err: {e}")),
        }
    }
}

fn import_obj_dialog(state: &mut AppState) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("OBJ", &["obj"])
        .pick_file()
    {
        match std::fs::read_to_string(&path) {
            Ok(text) => {
                let mesh = Mesh::from_obj(&text);
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("mesh")
                    .to_string();
                state.checkpoint("import obj");
                state.project.add(&name, mesh);
                state.sync_selection();
                state.emit_mesh_changed();
                state.set_status(format!("import {}", path.display()));
            }
            Err(e) => state.set_status(format!("import err: {e}")),
        }
    }
}

pub fn frame_selection(state: &mut AppState) {
    if let Some(o) = state.project.assets.get(state.project.active) {
        let mut c = glam::Vec3::ZERO;
        let mut n = 0;
        let mut r: f32 = 0.0;
        for v in &o.mesh.verts {
            if v.selected {
                c += v.vec();
                n += 1;
            }
        }
        if n == 0 {
            for v in &o.mesh.verts {
                c += v.vec();
            }
            n = o.mesh.verts.len().max(1);
        }
        c /= n as f32;
        let has_selection = o.mesh.verts.iter().any(|v| v.selected);
        for v in &o.mesh.verts {
            if !has_selection || v.selected {
                r = r.max((v.vec() - c).length());
            }
        }
        let mut goal = state.camera.clone();
        goal.frame(c, r.max(0.05));
        state.camera_frame = Some((state.camera.clone(), goal, 0.0));
        state.mark_dirty();
    }
}

// ------------------------------------------------------- toolbar esquerda

fn left_toolbar(ctx: &egui::Context, state: &mut AppState, tools: &ToolRegistry) {
    let want: &[&str] = match state.workspace {
        Workspace::Model => &[
            "select",
            "transform",
            "rotate",
            "scale",
            "primitives",
            "extrude",
            "inset",
            "bevel",
            "pushpull",
            "loop_cut",
            "knife",
            "slice",
            "subdivide",
            "draw_profile",
            "connect",
            "dissolve",
            "mirror",
            "merge",
        ],
        Workspace::Paint => &["paint"],
        Workspace::Uv => &["select"],
        Workspace::Export => &[],
    };
    let compact = ctx.screen_rect().width() < 1000.0;
    egui::SidePanel::left("toolbar")
        .exact_width(if compact { 52.0 } else { 152.0 })
        .resizable(false)
        .show(ctx, |ui| {
            ui.add_enabled_ui(!state.is_interacting(), |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("tools.scroll")
                    .show(ui, |ui| {
                        for &id in want {
                            let extra = match id {
                                "rotate" => Some((
                                    if state.i18n.lang == "en" {
                                        "Rotate"
                                    } else {
                                        "Rotacionar"
                                    },
                                    "R",
                                )),
                                "scale" => Some((
                                    if state.i18n.lang == "en" {
                                        "Scale"
                                    } else {
                                        "Escalar"
                                    },
                                    "S",
                                )),
                                "loop_cut" => Some(("Loop cut", "Ctrl+R")),
                                "knife" => Some((
                                    if state.i18n.lang == "en" {
                                        "Knife"
                                    } else {
                                        "Faca"
                                    },
                                    "K",
                                )),
                                _ => None,
                            };
                            let (label, hint) = if let Some((label, key)) = extra {
                                (label.to_owned(), format!("{label} · {key}"))
                            } else if let Some(tool) = tools.get(id) {
                                (
                                    state.t(tool.label_key()),
                                    format!("{} · {}", state.t(tool.hint_key()), tool.shortcut()),
                                )
                            } else {
                                continue;
                            };
                            let kind = match id {
                                "rotate" => petunia_core::ModalKind::Rotate,
                                "scale" => petunia_core::ModalKind::Scale,
                                _ => petunia_core::ModalKind::Move,
                            };
                            let active = if matches!(id, "transform" | "rotate" | "scale") {
                                state.active_tool == "transform" && state.gizmo_mode == kind
                            } else {
                                state.active_tool == id
                            };
                            if icons::tool_button(ui, id, &label, active, compact)
                                .on_hover_text(hint)
                                .clicked()
                            {
                                if matches!(id, "transform" | "rotate" | "scale") {
                                    state.active_tool = "transform".into();
                                    state.gizmo_mode = kind;
                                } else {
                                    state.active_tool = id.into();
                                }
                                // Toolbar selects a tool so its editable properties are immediately available.
                                // Keyboard shortcuts continue to launch direct mouse sessions.
                                state.pending_modal = None;
                                state.mark_dirty();
                            }
                        }
                    });
            });
        });
}

// ---------------------------------------------------------- painel direito

fn right_panel(
    ctx: &egui::Context,
    state: &mut AppState,
    tools: &ToolRegistry,
    registry: &mut ModuleRegistry,
) {
    let max_width = (ctx.screen_rect().width() * 0.43).clamp(210.0, 400.0);
    egui::SidePanel::right("props")
        .default_width(260.0)
        .width_range(210.0..=max_width)
        .show(ctx, |ui| {
            ui.heading(state.t("ui.properties"));
            ui.separator();
            egui::ScrollArea::vertical()
                .id_salt("properties.scroll")
                .show(ui, |ui| {
                    let fields =
                        state.workspace == Workspace::Model && tool_fields::draw(ui, state);
                    ui.add_enabled_ui(!state.is_interacting(), |ui| {
                        match state.workspace {
                            Workspace::Model => model_section(ctx, ui, state, tools, fields),
                            Workspace::Paint => {
                                if let Some(module) = registry.get_mut("paint") {
                                    module.ui(ctx, ui, state);
                                }
                            }
                            Workspace::Uv => {
                                if let Some(module) = registry.get_mut("uv") {
                                    module.ui(ctx, ui, state);
                                }
                            }
                            Workspace::Export => export_section(ui, state),
                        }
                        ui.separator();
                        if let Some(module) = registry.get_mut("assets") {
                            module.ui(ctx, ui, state);
                        }
                        if state.show_help {
                            egui::CollapsingHeader::new(state.t("ui.help"))
                                .default_open(true)
                                .show(ui, |ui| {
                                    ui.label(state.t("help.body"));
                                });
                        }
                    });
                });
        });
}

fn model_section(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    state: &mut AppState,
    tools: &ToolRegistry,
    fields: bool,
) {
    let active_id = state.active_tool.clone();
    let l_tool = state.t("ui.active_tool");
    let l_no_tool = state.t("ui.no_tool");
    if fields && active_id == "transform" {
        ui.horizontal_wrapped(|ui| {
            if ui.button(state.t("actions.duplicate")).clicked() {
                state.checkpoint("duplicate");
                if let Some(mesh) = state.project.active_mesh_mut() {
                    mesh.duplicate_selected();
                }
                state.sync_selection();
                state.emit_mesh_changed();
            }
            if ui.button(state.t("actions.delete")).clicked() {
                state.checkpoint("delete");
                if let Some(mesh) = state.project.active_mesh_mut() {
                    mesh.delete_selected();
                }
                state.sync_selection();
                state.emit_mesh_changed();
            }
        });
    }
    if !fields {
        egui::CollapsingHeader::new(l_tool)
            .default_open(true)
            .show(ui, |ui| {
                if let Some(tool) = tools.get(&active_id) {
                    tool.ui(ctx, ui, state);
                } else {
                    ui.label(l_no_tool);
                }
            });
    }
    let l_props = state.t("ui.properties");
    let l_name = state.t("props.name");
    let l_verts = state.t("props.verts");
    let l_faces = state.t("props.faces");
    egui::CollapsingHeader::new(l_props)
        .default_open(false)
        .show(ui, |ui| {
            if state.project.assets.is_empty() {
                return;
            }
            let idx = state.project.active.min(state.project.assets.len() - 1);
            let (vc, fc) = {
                let o = &state.project.assets[idx];
                (o.mesh.vert_count(), o.mesh.tri_count())
            };
            ui.horizontal(|ui| {
                ui.label(l_name);
                if let Some(o) = state.project.assets.get_mut(idx) {
                    ui.text_edit_singleline(&mut o.name);
                }
            });
            ui.label(format!("{l_verts}: {vc}  {l_faces}: {fc}"));
            let mut c = state.project.assets[idx].base_color;
            if ui.color_edit_button_rgb(&mut c).changed() {
                state.checkpoint("base color");
                if let Some(o) = state.project.assets.get_mut(idx) {
                    o.base_color = c;
                    for v in &mut o.mesh.verts {
                        if !v.selected {
                            v.color = c;
                        }
                    }
                }
                state.emit_mesh_changed();
                state.mark_dirty();
            }
        });

    refs_section(ui, state);
}

fn refs_section(ui: &mut egui::Ui, state: &mut AppState) {
    let l_refs = state.t("ui.refs");
    let l_load = state.t("refs.load");
    let l_offset = state.t("refs.offset");
    let l_size = state.t("refs.size");
    let l_opacity = state.t("refs.opacity");
    let l_rotation = state.t("refs.rotation");
    let l_xray = state.t("refs.xray");
    let l_front = state.t(RefAxis::Front.key());
    let l_back = state.t(RefAxis::Back.key());
    let l_left = state.t(RefAxis::Left.key());
    let l_right = state.t(RefAxis::Right.key());
    let l_top = state.t(RefAxis::Top.key());
    let l_bottom = state.t(RefAxis::Bottom.key());
    let ctx = ui.ctx().clone();
    egui::CollapsingHeader::new(l_refs)
        .default_open(false)
        .show(ui, |ui| {
            if ui.button(l_load).clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("image", &["png", "jpg", "jpeg"])
                    .pick_file()
                {
                    match load_image_rgba(&path) {
                        Ok((w, h, rgba)) => {
                            let name = path
                                .file_name()
                                .and_then(|s| s.to_str())
                                .unwrap_or("ref")
                                .to_string();
                            state
                                .refs
                                .push(petunia_core::ReferenceImage::from_rgba(name, w, h, rgba));
                            state.set_status(format!("ref {}", path.display()));
                            state.mark_dirty();
                        }
                        Err(e) => state.set_status(format!("ref err: {e}")),
                    }
                }
            }
            for r in state.refs.iter_mut() {
                r.ensure_texture(&ctx);
            }
            let mut rm: Option<usize> = None;
            let n = state.refs.len();
            for i in 0..n {
                let (tex_id, aspect) = {
                    let r = &state.refs[i];
                    (
                        r.texture.as_ref().map(|t| t.id()),
                        r.height as f32 / r.width.max(1) as f32,
                    )
                };
                {
                    let r = &mut state.refs[i];
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut r.visible, "");
                        let mut lock = r.locked;
                        if ui.checkbox(&mut lock, "🔒").changed() {
                            r.locked = lock;
                        }
                        ui.label(&r.name);
                        if ui.small_button("✕").clicked() {
                            rm = Some(i);
                        }
                    });
                }
                if let Some(tid) = tex_id {
                    ui.image((tid, egui::vec2(230.0, 230.0 * aspect)));
                }
                {
                    let r = &mut state.refs[i];
                    ui.horizontal(|ui| {
                        if ui
                            .selectable_label(r.axis == RefAxis::Front, format!("F {l_front}"))
                            .clicked()
                        {
                            r.axis = RefAxis::Front;
                        }
                        if ui
                            .selectable_label(r.axis == RefAxis::Back, format!("B {l_back}"))
                            .clicked()
                        {
                            r.axis = RefAxis::Back;
                        }
                        if ui
                            .selectable_label(r.axis == RefAxis::Left, format!("L {l_left}"))
                            .clicked()
                        {
                            r.axis = RefAxis::Left;
                        }
                    });
                    ui.horizontal(|ui| {
                        if ui
                            .selectable_label(
                                r.axis == RefAxis::Right || r.axis == RefAxis::Side,
                                format!("R {l_right}"),
                            )
                            .clicked()
                        {
                            r.axis = RefAxis::Right;
                        }
                        if ui
                            .selectable_label(r.axis == RefAxis::Top, format!("T {l_top}"))
                            .clicked()
                        {
                            r.axis = RefAxis::Top;
                        }
                        if ui
                            .selectable_label(r.axis == RefAxis::Bottom, format!("D {l_bottom}"))
                            .clicked()
                        {
                            r.axis = RefAxis::Bottom;
                        }
                    });
                    let mut ch = false;
                    ch |= ui
                        .add(egui::Slider::new(&mut r.offset, -10.0..=10.0).text(&l_offset))
                        .changed();
                    ch |= ui
                        .add(egui::Slider::new(&mut r.size, 0.5..=12.0).text(&l_size))
                        .changed();
                    ch |= ui
                        .add(egui::Slider::new(&mut r.opacity, 0.05..=1.0).text(&l_opacity))
                        .changed();
                    ch |= ui
                        .add(
                            egui::Slider::new(&mut r.rotation, -180.0..=180.0)
                                .suffix("°")
                                .text(&l_rotation),
                        )
                        .changed();
                    ch |= ui.checkbox(&mut r.xray, &l_xray).changed();
                    if ch {
                        state.mark_dirty();
                    }
                }
                ui.separator();
            }
            if let Some(i) = rm {
                state.refs.remove(i);
                state.mark_dirty();
            }
        });
}

fn export_section(ui: &mut egui::Ui, state: &mut AppState) {
    let l_exp = state.t("export.title");
    let l_fmt = state.t("export.format");
    let l_report = state.t("export.report");
    let l_go = state.t("export.go");
    egui::CollapsingHeader::new(l_exp)
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(l_fmt);
                if ui.selectable_label(!state.export_gltf, "OBJ").clicked() {
                    state.export_gltf = false;
                }
                if ui.selectable_label(state.export_gltf, "GLB").clicked() {
                    state.export_gltf = true;
                }
            });
            // multi-select
            let mut sel = state.export_selected.clone();
            if sel.is_empty() {
                sel = (0..state.project.assets.len()).collect();
            }
            let mut new_sel = Vec::new();
            let names: Vec<String> = state
                .project
                .assets
                .iter()
                .map(|a| a.name.clone())
                .collect();
            for (i, name) in names.iter().enumerate() {
                let mut on = sel.contains(&i);
                if ui.checkbox(&mut on, name).changed() {
                    state.mark_dirty();
                }
                if on {
                    new_sel.push(i);
                }
            }
            state.export_selected = new_sel.clone();
            ui.separator();
            ui.label(l_report);
            for line in export::export_report(&state.project, &new_sel, state.export_gltf) {
                ui.small(line);
            }
            if ui.button(l_go).clicked() {
                export_dialog(state, &new_sel);
            }
        });
}

fn export_dialog(state: &mut AppState, sel: &[usize]) {
    if sel.is_empty() {
        state.set_status(state.t("export.empty"));
        return;
    }
    if state.export_gltf {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("glTF", &["glb"])
            .set_file_name("assets.glb")
            .save_file()
        {
            match export::export_gltf(&state.project, sel) {
                Ok(bytes) => match std::fs::write(&path, bytes) {
                    Ok(()) => state.set_status(format!("export {}", path.display())),
                    Err(e) => state.set_status(format!("export err: {e}")),
                },
                Err(e) => state.set_status(format!("export err: {e}")),
            }
        }
    } else if let Some(dir) = rfd::FileDialog::new().pick_folder() {
        let mut n = 0;
        for &i in sel {
            if let Some(a) = state.project.assets.get(i) {
                let path = dir.join(format!("{}.obj", sanitize(&a.name)));
                if std::fs::write(&path, export::export_obj(a)).is_ok() {
                    n += 1;
                }
            }
        }
        state.set_status(format!("export: {n} OBJ"));
    }
}

fn sanitize(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

// --------------------------------------------------------------- status

fn status_bar(ctx: &egui::Context, state: &mut AppState, tools: &ToolRegistry) {
    let (verts, faces) = state
        .project
        .active_mesh()
        .map(|m| (m.verts.len(), m.faces.len()))
        .unwrap_or((0, 0));
    let hint = tools
        .get(&state.active_tool)
        .map(|t| state.t(t.hint_key()))
        .unwrap_or_else(|| state.t("hints.select"));
    let status = if state.status.is_empty() {
        hint
    } else {
        state.status.clone()
    };
    egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui
                .add_enabled(
                    !state.is_interacting() && state.undo.can_undo(),
                    egui::Button::new(state.t("edit.undo")),
                )
                .on_hover_text(format!(
                    "Ctrl+Z · {}",
                    state.undo.undo_label().unwrap_or("")
                ))
                .clicked()
            {
                state.undo();
            }
            if ui
                .add_enabled(
                    !state.is_interacting() && state.undo.can_redo(),
                    egui::Button::new(state.t("edit.redo")),
                )
                .on_hover_text(format!(
                    "Ctrl+Shift+Z · {}",
                    state.undo.redo_label().unwrap_or("")
                ))
                .clicked()
            {
                state.redo();
            }
            ui.separator();
            ui.monospace(format!("{verts}v  {faces}f"))
                .on_hover_text(format!(
                    "{}\n{:.1} ms · {} draws",
                    state.backend_name, state.stats.frame_ms, state.stats.draws
                ));
            ui.separator();
            ui.add(egui::Label::new(&status).truncate())
                .on_hover_text(&status);
        });
    });
}

// ------------------------------------------------------------- viewport

fn viewport(ctx: &egui::Context, state: &mut AppState) {
    // FUNDO TRANSPARENTE: o 3D é desenhado por baixo (wgpu/GL) e o egui
    // compõe por cima. Um fill opaco aqui ESCONDE a cena inteira.
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(egui::Color32::TRANSPARENT))
        .show(ctx, |ui| {
            let rect = ui.available_rect_before_wrap();
            state.viewport_rect = Some(rect);
            state.viewport_pixels_per_point = ctx.pixels_per_point();
            state.camera.aspect = rect.width() / rect.height().max(1.0);
            let p = ui.painter_at(rect);
            // mira central
            p.circle_stroke(
                rect.center(),
                4.0,
                egui::Stroke::new(1.0_f32, egui::Color32::from_gray(120)),
            );
            // overlay do Draw Profile
            if !state.profile.points.is_empty() {
                draw_profile_overlay(&p, rect, state);
            }
            // indicador de vista ortográfica
            if state.camera.proj == Projection::Ortho {
                p.text(
                    rect.min + egui::vec2(8.0, 8.0),
                    egui::Align2::LEFT_TOP,
                    "ORTHO",
                    egui::FontId::monospace(11.0),
                    egui::Color32::LIGHT_BLUE,
                );
            }
            let resp = ui.allocate_rect(rect, egui::Sense::click_and_drag());
            if viewport_interaction::draw(ctx, state, rect, &p, &resp) {
                return;
            }
            if resp.drag_started_by(egui::PointerButton::Primary) {
                if let Some(pos) = resp.interact_pointer_pos() {
                    state.box_select_start = Some([pos.x, pos.y]);
                }
            }
            if resp.dragged_by(egui::PointerButton::Primary) {
                if let (Some(start), Some(curr)) =
                    (state.box_select_start, resp.interact_pointer_pos())
                {
                    let r = egui::Rect::from_two_pos(egui::pos2(start[0], start[1]), curr);
                    p.rect_filled(
                        r,
                        0.0,
                        egui::Color32::from_rgba_unmultiplied(255, 160, 40, 40),
                    );
                    p.rect_stroke(
                        r,
                        0.0,
                        egui::Stroke::new(1.0f32, egui::Color32::from_rgb(255, 160, 40)),
                        egui::StrokeKind::Outside,
                    );
                    state.mark_dirty();
                }
            }
            if resp.drag_stopped_by(egui::PointerButton::Primary) {
                if let (Some(start), Some(curr)) =
                    (state.box_select_start.take(), resp.interact_pointer_pos())
                {
                    let dx = (curr.x - start[0]).abs();
                    let dy = (curr.y - start[1]).abs();
                    if dx > 8.0 || dy > 8.0 {
                        let to_ndc = |pos: egui::Pos2| -> [f32; 2] {
                            let nx = ((pos.x - rect.min.x) / rect.width().max(1.0)) * 2.0 - 1.0;
                            let ny = 1.0 - ((pos.y - rect.min.y) / rect.height().max(1.0)) * 2.0;
                            [nx, ny]
                        };
                        let p0 = to_ndc(egui::pos2(start[0], start[1]));
                        let p1 = to_ndc(curr);
                        let shift = ui.input(|i| i.modifiers.shift);
                        if let Some(m) = state.project.active_mesh_mut() {
                            m.box_select(p0, p1, &state.camera.view_proj().to_cols_array(), shift);
                        }
                        state.sync_selection();
                        state.mark_dirty();
                    }
                }
            }
            if resp.clicked() {
                state.box_select_start = None;
                if let Some(pos) = resp.interact_pointer_pos() {
                    let nx = ((pos.x - rect.min.x) / rect.width().max(1.0)) * 2.0 - 1.0;
                    let ny = 1.0 - ((pos.y - rect.min.y) / rect.height().max(1.0)) * 2.0;
                    state.pending_pick = Some((nx, ny));
                    state.mark_dirty();
                }
            }
        });
}

fn draw_profile_overlay(p: &egui::Painter, rect: egui::Rect, state: &AppState) {
    let to_screen = |w: glam::Vec3| {
        let ndc = state.camera.project_ndc(w);
        egui::pos2(
            rect.min.x + (ndc.x * 0.5 + 0.5) * rect.width(),
            rect.min.y + (1.0 - (ndc.y * 0.5 + 0.5)) * rect.height(),
        )
    };
    let pts: Vec<egui::Pos2> = (0..state.profile.points.len())
        .map(|i| to_screen(state.profile.to_3d(i)))
        .collect();
    for w in pts.windows(2) {
        p.line_segment(
            [w[0], w[1]],
            egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(255, 170, 60)),
        );
    }
    if state.profile.closed && pts.len() >= 3 {
        p.line_segment(
            [pts[pts.len() - 1], pts[0]],
            egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(255, 170, 60)),
        );
        // preview da triangulação
        if let Ok(tris) = petunia_mesh::triangulate::ear_clip(&state.profile.points) {
            for t in tris {
                for (a, b) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
                    if a < pts.len() && b < pts.len() {
                        p.line_segment(
                            [pts[a], pts[b]],
                            egui::Stroke::new(0.75_f32, egui::Color32::from_rgb(90, 200, 255)),
                        );
                    }
                }
            }
        }
    }
    for q in &pts {
        p.circle_filled(*q, 3.5, egui::Color32::from_rgb(255, 170, 60));
    }
}

fn load_image_rgba(path: &std::path::Path) -> Result<(u32, u32, Vec<u8>), String> {
    let img = image::open(path).map_err(|e| e.to_string())?;
    let mut rgba = img.to_rgba8();
    const MAX: u32 = 2048;
    if rgba.width() > MAX || rgba.height() > MAX {
        let (w, h) = (rgba.width(), rgba.height());
        let s = (MAX as f32 / w.max(h) as f32).min(1.0);
        rgba = image::imageops::resize(
            &rgba,
            ((w as f32 * s) as u32).max(1),
            ((h as f32 * s) as u32).max(1),
            image::imageops::FilterType::Lanczos3,
        );
    }
    Ok((rgba.width(), rgba.height(), rgba.into_raw()))
}

#[cfg(test)]
mod paint_tests;

#[cfg(test)]
mod cutting_tests;
