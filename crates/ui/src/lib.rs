//! Petunia3D UI (egui): header com pílulas de workspace, toolbar,
//! Asset Library, painéis por workspace, viewport com overlays e status.
//!
//! Regra de borrow: rótulos `state.t()` (owned String) são pré-calculados
//! antes dos closures; mutações via índices, nunca com iterator vivo.

use petunia_core::Projection;
use petunia_core::{AppState, ModuleRegistry, ProjectService, RefAxis};
use petunia_module_model::ToolRegistry;
use petunia_project::export;

pub mod annotation;
pub mod app_icons;
pub mod asset_browser;
pub mod asset_library_drawer;
pub mod camera_controls;
pub mod command_palette;
pub mod contextual_shelf;
mod cutting;
pub mod file_dialog_service;
pub mod gizmo;
pub mod icon_registry;
pub mod icons;
pub mod main_header;
pub mod measurement;
#[cfg(test)]
mod modal_tests;
mod modal_viewport;
pub mod modules_ui;
pub mod nav_gizmo;
pub mod outliner;
pub mod properties_panel;
pub mod recovery_dialog;
pub mod reference_manager;
pub mod settings_modal;
pub mod status_bar;
pub mod tiles_workspace;
pub mod timeline;
pub mod tokens;
mod tool_fields;
pub mod toolbar;
pub mod transform_gizmo_integration;
pub mod viewport_bar;
mod viewport_interaction;
pub mod widgets;

pub use recovery_dialog::{draw as draw_recovery_dialog, RecoveryAction};
pub use tokens::apply_theme_to_egui;

pub fn rect_to_logical(r: egui::Rect) -> petunia_core::viewport::LogicalRect {
    petunia_core::viewport::LogicalRect::from_min_max([r.min.x, r.min.y], [r.max.x, r.max.y])
}

pub fn logical_to_rect(r: petunia_core::viewport::LogicalRect) -> egui::Rect {
    egui::Rect::from_min_max(
        egui::pos2(r.left(), r.top()),
        egui::pos2(r.right(), r.bottom()),
    )
}

static REF_TEXTURES: std::sync::Mutex<
    Option<std::collections::HashMap<String, egui::TextureHandle>>,
> = std::sync::Mutex::new(None);

pub fn get_ref_texture(
    ctx: &egui::Context,
    img: &petunia_core::ReferenceImage,
) -> egui::TextureHandle {
    let mut lock = REF_TEXTURES.lock().unwrap();
    let map = lock.get_or_insert_with(std::collections::HashMap::new);
    if let Some(handle) = map.get(&img.name) {
        return handle.clone();
    }
    let color_img = egui::ColorImage::from_rgba_unmultiplied(
        [img.width as usize, img.height as usize],
        &img.rgba,
    );
    let handle = ctx.load_texture(&img.name, color_img, egui::TextureOptions::LINEAR);
    map.insert(img.name.clone(), handle.clone());
    handle
}

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
    main_header::draw(ctx, state, action);
    status_bar::draw(ctx, state, tools);
    asset_browser::draw(ctx, state);
    toolbar::draw(ctx, state, tools);
    right_panel(ctx, state, tools, registry);
    viewport_bar_panel(ctx, state);
    viewport(ctx, state);
    asset_library_drawer::draw(ctx, state);
    settings_modal::draw(ctx, state);
    command_palette::draw(ctx, state);
    reference_manager::draw(ctx, state);
}

fn viewport_bar_panel(ctx: &egui::Context, state: &mut AppState) {
    egui::TopBottomPanel::top("viewport_context_bar")
        .default_height(tokens::VIEWPORT_BAR_HEIGHT)
        .height_range(tokens::VIEWPORT_BAR_HEIGHT..=tokens::VIEWPORT_BAR_MAX_HEIGHT)
        .resizable(true)
        .frame(
            egui::Frame::new()
                .fill(tokens::BG_PANEL_HEADER)
                .stroke(tokens::stroke_border())
                .inner_margin(egui::Margin::symmetric(6, 2)),
        )
        .show(ctx, |ui| {
            ui.add_enabled_ui(!state.is_interacting(), |ui| {
                viewport_bar::draw(ui, state);
            });
        });
}

pub fn right_panel(
    ctx: &egui::Context,
    state: &mut AppState,
    tools: &ToolRegistry,
    registry: &mut ModuleRegistry,
) {
    let max_width = (ctx.screen_rect().width() * 0.45).clamp(240.0, 420.0);
    egui::SidePanel::right("props")
        .default_width(tokens::PROPERTIES_DEFAULT_WIDTH)
        .width_range(220.0..=max_width)
        .frame(
            egui::Frame::new()
                .fill(tokens::BG_PANEL)
                .stroke(tokens::stroke_border())
                .inner_margin(egui::Margin::symmetric(6, 4)),
        )
        .show(ctx, |ui| {
            outliner::draw(ui, state);
            ui.separator();
            properties_panel::draw(ctx, ui, state, tools, registry);
        });
}

pub fn new_project(state: &mut AppState) {
    ProjectService::new_project(state);
}

pub fn open_project_dialog(state: &mut AppState) {
    if let Some(path) = file_dialog_service::pick_project_file() {
        if let Err(e) = ProjectService::load_project(state, &path) {
            state.set_status(format!("open err: {e}"));
        }
    }
}

pub fn save_project_dialog(state: &mut AppState, save_as: bool) {
    let path = if !save_as {
        state
            .project
            .project_path
            .clone()
            .map(std::path::PathBuf::from)
    } else {
        None
    };
    let path = path.or_else(|| file_dialog_service::pick_save_project_file("project.petunia"));
    if let Some(path) = path {
        if let Err(e) = ProjectService::save_project(state, &path) {
            state.set_status(format!("save err: {e}"));
        }
    }
}

pub fn import_obj_dialog(state: &mut AppState) {
    if let Some(path) = file_dialog_service::pick_obj_file() {
        if let Err(e) = ProjectService::import_obj(state, &path) {
            state.set_status(format!("import err: {e}"));
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

pub fn refs_section(ui: &mut egui::Ui, state: &mut AppState) {
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
                if let Some(path) = file_dialog_service::pick_image_file() {
                    match load_image_rgba(&path) {
                        Ok((w, h, rgba)) => {
                            let name = path
                                .file_name()
                                .and_then(|s| s.to_str())
                                .unwrap_or("ref")
                                .to_string();
                            ProjectService::add_reference_image(state, name, w, h, rgba);
                        }
                        Err(e) => state.set_status(format!("ref err: {e}")),
                    }
                }
            }
            let mut rm: Option<usize> = None;
            let n = state.project.refs.len();
            for i in 0..n {
                let (tex_id, aspect) = {
                    let r = &state.project.refs[i];
                    let tex = get_ref_texture(&ctx, r);
                    (Some(tex.id()), r.height as f32 / r.width.max(1) as f32)
                };
                {
                    let r = &mut state.project.refs[i];
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut r.visible, "");
                        let mut lock = r.locked;
                        if ui.checkbox(&mut lock, "Lock").changed() {
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
                    let r = &mut state.project.refs[i];
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
                state.project.refs.remove(i);
                state.mark_dirty();
            }
        });
}

pub fn export_section(ui: &mut egui::Ui, state: &mut AppState) {
    let l_exp = state.t("export.title");
    let l_fmt = state.t("export.format");
    let l_report = state.t("export.report");
    let l_go = state.t("export.go");
    egui::CollapsingHeader::new(l_exp)
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(l_fmt);
                if ui
                    .selectable_label(!state.project.export_gltf, "OBJ")
                    .clicked()
                {
                    state.project.export_gltf = false;
                }
                if ui
                    .selectable_label(state.project.export_gltf, "GLB")
                    .clicked()
                {
                    state.project.export_gltf = true;
                }
            });
            // multi-select
            let mut sel_ids = state.project.export_selected.clone();
            if sel_ids.is_empty() {
                sel_ids = state.project.assets.iter().map(|a| a.id).collect();
            }
            let mut new_sel_ids = Vec::new();
            let mut new_sel_indices = Vec::new();
            let asset_items: Vec<(usize, uuid::Uuid, String)> = state
                .project
                .assets
                .iter()
                .enumerate()
                .map(|(i, a)| (i, a.id, a.name.clone()))
                .collect();
            let mut changed = false;
            for (i, asset_id, name) in asset_items {
                let mut on = sel_ids.contains(&asset_id);
                if ui.checkbox(&mut on, &name).changed() {
                    changed = true;
                }
                if on {
                    new_sel_ids.push(asset_id);
                    new_sel_indices.push(i);
                }
            }
            if changed {
                state.mark_dirty();
            }
            state.project.export_selected = new_sel_ids;
            ui.separator();
            ui.label(l_report);
            for line in
                export::export_report(&state.project, &new_sel_indices, state.project.export_gltf)
            {
                ui.small(line);
            }
            if ui.button(l_go).clicked() {
                export_dialog(state, &new_sel_indices);
            }
        });
}

pub fn export_active_or_all(state: &mut AppState, glb: bool) {
    let sel = state.project.export_selected_indices();
    state.project.export_gltf = glb;
    export_dialog(state, &sel);
}

fn export_dialog(state: &mut AppState, sel: &[usize]) {
    if sel.is_empty() {
        state.set_status(state.t("export.empty"));
        return;
    }
    if state.project.export_gltf {
        if let Some(path) = file_dialog_service::pick_export_glb_file("assets.glb") {
            match ProjectService::export_glb(state, sel, &path) {
                Ok(()) => state.set_status(format!("export {}", path.display())),
                Err(e) => state.set_status(format!("export err: {e}")),
            }
        }
    } else if let Some(dir) = file_dialog_service::pick_folder() {
        match ProjectService::export_all_obj_to_dir(state, sel, &dir) {
            Ok(n) => state.set_status(format!("export: {n} OBJ")),
            Err(e) => state.set_status(format!("export err: {e}")),
        }
    }
}

// ------------------------------------------------------------- viewport

fn viewport(ctx: &egui::Context, state: &mut AppState) {
    // FUNDO TRANSPARENTE: o 3D é desenhado por baixo (wgpu/GL) e o egui
    // compõe por cima. Um fill opaco aqui ESCONDE a cena inteira.
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(egui::Color32::TRANSPARENT))
        .show(ctx, |ui| {
            let rect = ui.available_rect_before_wrap();
            state.ui.viewport_rect = Some(rect_to_logical(rect));
            state.ui.viewport_pixels_per_point = ctx.pixels_per_point();
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

            // Barra contextual horizontal flutuante na base da viewport
            let shelf_rect = contextual_shelf::draw(ui, state, rect);
            let pointer_on_shelf = shelf_rect.is_some_and(|sr| {
                ui.input(|i| {
                    i.pointer
                        .interact_pos()
                        .or(i.pointer.hover_pos())
                        .is_some_and(|pos| sr.contains(pos))
                })
            });

            if pointer_on_shelf {
                state.ui.box_select_start = None;
                return;
            }
            if resp.drag_started_by(egui::PointerButton::Primary) {
                if let Some(pos) = resp.interact_pointer_pos() {
                    state.ui.box_select_start = Some([pos.x, pos.y]);
                }
            }
            if resp.dragged_by(egui::PointerButton::Primary) {
                if let (Some(start), Some(curr)) =
                    (state.ui.box_select_start, resp.interact_pointer_pos())
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
                if let (Some(start), Some(curr)) = (
                    state.ui.box_select_start.take(),
                    resp.interact_pointer_pos(),
                ) {
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
                        let vp = state.session.camera.view_proj().to_cols_array();
                        let _ = state.dispatch(&petunia_core::BoxSelectCmd {
                            p0,
                            p1,
                            view_proj: vp,
                            add: shift,
                        });
                    }
                }
            }
            if resp.clicked() {
                state.ui.box_select_start = None;
                if let Some(pos) = resp.interact_pointer_pos() {
                    let nx = ((pos.x - rect.min.x) / rect.width().max(1.0)) * 2.0 - 1.0;
                    let ny = 1.0 - ((pos.y - rect.min.y) / rect.height().max(1.0)) * 2.0;
                    state.ui.pending_pick = Some((nx, ny));
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

pub fn load_image_rgba(path: &std::path::Path) -> Result<(u32, u32, Vec<u8>), String> {
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

/// Diálogo para selecionar e adicionar uma imagem de referência à cena.
pub fn pick_and_add_reference_image(state: &mut AppState) {
    if let Some(path) = file_dialog_service::pick_image_file() {
        match load_image_rgba(&path) {
            Ok((w, h, rgba)) => {
                let name = path
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("ref")
                    .to_string();
                ProjectService::add_reference_image(state, name.clone(), w, h, rgba);
                state.set_status(format!("Imagem de referência '{name}' adicionada"));
            }
            Err(e) => state.set_status(format!("Erro ao carregar imagem: {e}")),
        }
    }
}

#[cfg(test)]
mod paint_tests;

#[cfg(test)]
mod cutting_tests;
