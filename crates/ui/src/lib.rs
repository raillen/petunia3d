//! Petunia3D UI (egui): header com pílulas de workspace, toolbar,
//! Asset Library, painéis por workspace, viewport com overlays e status.
//!
//! Regra de borrow: rótulos `state.t()` (owned String) são pré-calculados
//! antes dos closures; mutações via índices, nunca com iterator vivo.

use petunia_core::Projection;
use petunia_core::{AppState, ModuleRegistry, ProjectService, RefAxis, Workspace};
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
#[cfg(feature = "devtools")]
pub mod devtools;
pub mod file_dialog_service;
pub mod flex_layout;
pub mod gizmo;
#[cfg(feature = "help-markdown")]
pub mod help_markdown;
pub mod icon_provider;
pub mod icon_registry;
pub mod icons;
pub mod image_kit;
pub mod inbox_bridge;
#[cfg(feature = "keymap-capture")]
pub mod key_capture;
pub mod main_header;
pub mod measurement;
#[cfg(test)]
mod modal_tests;
mod modal_viewport;
pub mod modules_ui;
pub mod nav_gizmo;
pub mod outliner;
#[cfg(feature = "palette-autocomplete")]
pub mod palette_complete;
pub mod properties_panel;
pub mod recovery_dialog;
pub mod reference_manager;
pub mod regions;
pub mod settings_modal;
pub mod status_bar;
pub mod tiles_workspace;
pub mod timeline;
pub mod tokens;
mod tool_fields;
pub mod toolbar;
pub mod transform_gizmo_integration;
pub mod twill_bridge;
pub mod viewport_bar;
mod viewport_interaction;
pub mod widgets;
pub mod workspaces;

pub use recovery_dialog::{RecoveryAction, draw as draw_recovery_dialog};
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
    ui: &mut egui::Ui,
    state: &mut AppState,
    tools: &ToolRegistry,
    registry: &mut ModuleRegistry,
    action: &mut UiAction,
) {
    let theme_id_key = egui::Id::new("petunia_applied_theme_id");
    let needs_theme_update = ui.ctx().data(|d| {
        d.get_temp::<String>(theme_id_key)
            .map(|id| id != state.ui.active_theme_id)
            .unwrap_or(true)
    });
    if needs_theme_update {
        let theme_reg = petunia_config::ThemeRegistry::global();
        if let Some(t) = theme_reg.get_theme(&state.ui.active_theme_id) {
            tokens::apply_theme_to_egui(t, ui.ctx());
            ui.ctx().data_mut(|d| {
                d.insert_temp(theme_id_key, state.ui.active_theme_id.clone());
            });
        }
    }

    icon_registry::IconRegistry::ensure_fonts(ui.ctx());
    ui.ctx().data_mut(|d| {
        d.insert_temp(
            egui::Id::new("petunia_active_icon_pack"),
            state.ui.active_icon_pack_id.clone(),
        );
    });
    // Wave 2: reseta as regiões do shell; cada painel registra a sua ao desenhar.
    regions::reset(ui.ctx());

    main_header::draw(ui, state, action);
    // A status bar é o ÚNICO `Panel::bottom` do shell: dois painéis bottom
    // empilhados deslocam o segundo em ~11px. A Timeline do Animate vive como
    // faixa fixa dentro da área central (ver `animate_workspace_center`).
    status_bar::draw(ui, state, tools);
    asset_browser::draw(ui, state);
    toolbar::draw(ui, state, tools);
    right_panel(ui, state, tools, registry);
    viewport_bar_panel(ui, state);
    viewport(ui, state);
    let ctx = ui.ctx().clone();
    asset_library_drawer::draw(&ctx, state);
    settings_modal::draw(&ctx, state);
    command_palette::draw(&ctx, state);
    reference_manager::draw(&ctx, state);
    file_dialog_service::draw(&ctx, state);
}

fn viewport_bar_panel(ui: &mut egui::Ui, state: &mut AppState) {
    let bar = egui::Panel::top("viewport_context_bar")
        .default_size(tokens::VIEWPORT_BAR_HEIGHT)
        .size_range(tokens::VIEWPORT_BAR_HEIGHT..=tokens::VIEWPORT_BAR_MAX_HEIGHT)
        .resizable(true)
        .frame(
            egui::Frame::new()
                .fill(tokens::bg_panel_header(state))
                .stroke(tokens::stroke_border_dyn(state))
                .inner_margin(egui::Margin::symmetric(6, 2)),
        )
        .show(ui, |ui| {
            ui.add_enabled_ui(!state.is_interacting(), |ui| {
                viewport_bar::draw(ui, state);
            });
        });
    regions::record(
        ui.ctx(),
        regions::RegionSlot::ViewportToolbar,
        bar.response.rect,
    );
}

/// Dock direito: Outliner e Inspector como painéis independentes (Wave 2).
///
/// Substitui o empilhamento único por split vertical com divisor arrastável,
/// colapso independente e rolagem própria por seção. A posição do divisor e os
/// colapsos vivem em `UiState` (dono único, §3.2) e sobrevivem à troca de
/// workspace. O destacamento do Inspector continua opcional, mas não é mais o
/// único alívio para o espremedimento.
pub fn right_panel(
    ui: &mut egui::Ui,
    state: &mut AppState,
    tools: &ToolRegistry,
    registry: &mut ModuleRegistry,
) {
    puffin::profile_function!();
    let was_detached = state.ui.inspector_detached;
    let max_width = (ui.ctx().viewport_rect().width() * 0.45).clamp(240.0, 420.0);
    let dock = egui::Panel::right("props")
        .default_size(tokens::PROPERTIES_DEFAULT_WIDTH)
        .size_range(220.0..=max_width)
        .frame(
            egui::Frame::new()
                .fill(tokens::bg_panel(state))
                .stroke(tokens::stroke_border_dyn(state))
                .inner_margin(egui::Margin::symmetric(6, 4)),
        )
        .show(ui, |ui| {
            // Wave 2: zera o espaçamento vertical entre alocações do dock para que
            // as seções somem exatamente à altura do painel (sem invadir a status
            // bar). Interiores restauram o espaçamento padrão na sua raiz.
            let dock_spacing = ui.spacing().item_spacing;
            ui.spacing_mut().item_spacing.y = 0.0;
            if was_detached {
                // Aviso primeiro (altura própria), Outliner preenche o restante.
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = dock_spacing;
                    ui.label(
                        egui::RichText::new("Inspector Flutuante")
                            .size(11.0)
                            .color(tokens::TEXT_MUTED),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .button(egui::RichText::new("Reancorar").size(11.0))
                            .on_hover_text("Reancorar o Painel de Propriedades na barra lateral")
                            .clicked()
                        {
                            state.ui.inspector_detached = false;
                            state.mark_dirty();
                        }
                    });
                });
                ui.separator();
                let rest = ui.available_rect_before_wrap();
                let body = ui.allocate_ui_with_layout(
                    rest.size(),
                    egui::Layout::top_down_justified(egui::Align::LEFT),
                    |ui| {
                        ui.spacing_mut().item_spacing = dock_spacing;
                        egui::ScrollArea::vertical()
                            .id_salt("dock_outliner_full_scroll")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                outliner::draw_body(ui, state);
                            });
                    },
                );
                regions::record(
                    ui.ctx(),
                    regions::RegionSlot::RightOutliner,
                    body.response.rect,
                );
            } else {
                draw_split_dock(ui, state, tools, registry, dock_spacing);
            }
            // Restaura o espaçamento do painel (higiene; nada mais aloca abaixo).
            ui.spacing_mut().item_spacing = dock_spacing;
        });
    regions::record(ui.ctx(), regions::RegionSlot::RightDock, dock.response.rect);

    if was_detached && state.ui.inspector_detached {
        let mut is_open = true;
        let ctx = ui.ctx().clone();
        // Wave 2 (§9.4): máximo nunca excede a viewport útil.
        let screen_rect = ctx.viewport_rect();
        let avail_w = (screen_rect.width() - 24.0).clamp(240.0, 640.0);
        let avail_h = (screen_rect.height() - 24.0).clamp(240.0, 760.0);
        let win = egui::Window::new("Properties Inspector")
            .open(&mut is_open)
            .default_size([280.0, 420.0])
            .min_width(220.0_f32.min(avail_w))
            .min_height(200.0_f32.min(avail_h))
            .max_size(egui::vec2(avail_w, avail_h))
            .frame(
                egui::Frame::new()
                    .fill(tokens::bg_panel(state))
                    .stroke(tokens::stroke_border_dyn(state))
                    .inner_margin(egui::Margin::symmetric(6, 4)),
            )
            .show(&ctx, |ui| {
                ui.push_id("detached_inspector", |ui| {
                    properties_panel::draw(ui, state, tools, registry);
                });
            });
        if let Some(win) = win {
            regions::record(&ctx, regions::RegionSlot::RightInspector, win.response.rect);
        }
        if !is_open {
            state.ui.inspector_detached = false;
            state.mark_dirty();
        }
    }
}

/// Cabeçalho slim de seção do dock com colapso explícito.
fn dock_section_header(ui: &mut egui::Ui, title: String, collapsed: bool, tooltip: String) -> bool {
    let mut clicked = false;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(4.0, 0.0);
        let glyph = if collapsed { "+" } else { "–" };
        if ui.small_button(glyph).on_hover_text(&tooltip).clicked() {
            clicked = true;
        }
        if ui
            .selectable_label(!collapsed, egui::RichText::new(title).size(11.0).strong())
            .on_hover_text(&tooltip)
            .clicked()
        {
            clicked = true;
        }
    });
    clicked
}

fn draw_split_dock(
    ui: &mut egui::Ui,
    state: &mut AppState,
    tools: &ToolRegistry,
    registry: &mut ModuleRegistry,
    interior_spacing: egui::Vec2,
) {
    let avail = ui.available_rect_before_wrap();
    let total_h = avail.height();
    let total_w = avail.width();
    let (out_h, insp_h) = regions::split_heights(
        total_h,
        state.ui.right_dock_split,
        state.ui.outliner_collapsed,
        state.ui.inspector_collapsed,
    );

    // 1. Outliner (topo)
    let out_title = state.t("ui.outliner");
    let out_tip = state.t(if state.ui.outliner_collapsed {
        "ui.expand"
    } else {
        "ui.collapse"
    });
    let out_resp = ui.allocate_ui_with_layout(
        egui::vec2(total_w, out_h),
        egui::Layout::top_down_justified(egui::Align::LEFT),
        |ui| {
            // Interiores usam espaçamento normal; o empilhamento do dock usa 0.
            ui.spacing_mut().item_spacing = interior_spacing;
            if dock_section_header(ui, out_title, state.ui.outliner_collapsed, out_tip) {
                state.ui.outliner_collapsed = !state.ui.outliner_collapsed;
                state.mark_dirty();
            }
            if !state.ui.outliner_collapsed {
                egui::ScrollArea::vertical()
                    .id_salt("dock_outliner_scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        outliner::draw_body(ui, state);
                    });
            }
        },
    );
    regions::record(
        ui.ctx(),
        regions::RegionSlot::RightOutliner,
        out_resp.response.rect,
    );

    // 2. Divisor arrastável
    let (sep_rect, sep_resp) = ui.allocate_exact_size(
        egui::vec2(total_w, regions::DOCK_SEPARATOR_H),
        egui::Sense::drag(),
    );
    let sep_fill = if sep_resp.dragged() {
        tokens::ACCENT_BLUE
    } else if sep_resp.hovered() {
        tokens::BG_SURFACE_HOVER
    } else {
        egui::Color32::TRANSPARENT
    };
    ui.painter().rect_filled(sep_rect, 2.0, sep_fill);
    let mid_y = sep_rect.center().y;
    ui.painter().line_segment(
        [
            egui::pos2(sep_rect.min.x + 10.0, mid_y),
            egui::pos2(sep_rect.max.x - 10.0, mid_y),
        ],
        egui::Stroke::new(1.0_f32, tokens::BORDER_SUBTLE),
    );
    let split_tip = state.t("ui.dock_split_hint");
    let sep_resp = sep_resp.on_hover_text(split_tip);
    if sep_resp.hovered() || sep_resp.dragged() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
    }
    if sep_resp.dragged()
        && let Some(pos) = ui.input(|i| i.pointer.hover_pos().or(i.pointer.interact_pos()))
    {
        let new_split = ((pos.y - avail.min.y) / total_h.max(1.0)).clamp(0.25, 0.75);
        if (new_split - state.ui.right_dock_split).abs() > f32::EPSILON {
            state.ui.right_dock_split = new_split;
            // Arrastar reabre ambas as seções: o gesto declara intenção de ver as duas.
            state.ui.outliner_collapsed = false;
            state.ui.inspector_collapsed = false;
            state.mark_dirty();
        }
    }

    // 3. Inspector (base)
    let insp_title = state.t("ui.properties");
    let insp_tip = state.t(if state.ui.inspector_collapsed {
        "ui.expand"
    } else {
        "ui.collapse"
    });
    let insp_resp = ui.allocate_ui_with_layout(
        egui::vec2(total_w, insp_h.max(regions::DOCK_HEADER_H)),
        egui::Layout::top_down_justified(egui::Align::LEFT),
        |ui| {
            ui.spacing_mut().item_spacing = interior_spacing;
            if dock_section_header(ui, insp_title, state.ui.inspector_collapsed, insp_tip) {
                state.ui.inspector_collapsed = !state.ui.inspector_collapsed;
                state.mark_dirty();
            }
            if !state.ui.inspector_collapsed {
                properties_panel::draw(ui, state, tools, registry);
            }
        },
    );
    regions::record(
        ui.ctx(),
        regions::RegionSlot::RightInspector,
        insp_resp.response.rect,
    );
}

pub fn new_project(state: &mut AppState) {
    ProjectService::new_project(state);
}

pub fn open_project_dialog(_state: &mut AppState) {
    file_dialog_service::open_project_in_canvas();
}

pub fn save_project_dialog(state: &mut AppState, save_as: bool) {
    if !save_as && let Some(ref path) = state.project.project_path {
        let p = std::path::PathBuf::from(path);
        if let Err(e) = ProjectService::save_project(state, &p) {
            state.set_status(format!("save err: {e}"));
        }
    } else {
        file_dialog_service::save_project_as_in_canvas();
    }
}

pub fn import_obj_dialog(_state: &mut AppState) {
    file_dialog_service::import_obj_in_canvas();
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
            if ui.button(l_load).clicked()
                && let Some(path) = file_dialog_service::pick_image_file()
            {
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

fn viewport(ui: &mut egui::Ui, state: &mut AppState) {
    puffin::profile_function!();
    // FUNDO TRANSPARENTE: o 3D é desenhado por baixo (wgpu/GL) e o egui
    // compõe por cima. Um fill opaco aqui ESCONDE a cena inteira.
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(egui::Color32::TRANSPARENT))
        .show(ui, |ui| {
            if state.workspace == Workspace::Uv {
                uv_workspace_center(ui, state);
            } else if state.workspace == Workspace::Animate
                && workspaces::profile_for(state.workspace).bottom
                    == workspaces::BottomPaneKind::Timeline
            {
                animate_workspace_center(ui, state);
            } else {
                let rect = ui.available_rect_before_wrap();
                viewport_3d(ui, state, rect);
            }
        });
}

/// Centro do workspace Animate (Wave 3 — §6.4): viewport 3D + faixa real de
/// Timeline abaixo. Faixa com altura fixa canônica; a viewport 3D registra o
/// retângulo restante (a superfície GPU segue `viewport_rect`).
fn animate_workspace_center(ui: &mut egui::Ui, state: &mut AppState) {
    puffin::profile_function!();
    let total = ui.available_rect_before_wrap();
    let strip_h = tokens::TIMELINE_HEIGHT;
    let vp_rect = egui::Rect::from_min_max(
        total.min,
        egui::pos2(total.max.x, (total.max.y - strip_h).max(total.min.y)),
    );
    viewport_3d(ui, state, vp_rect);
    // Faixa posicionada explicitamente (não via cursor): soma exata ao total.
    let strip_rect = egui::Rect::from_min_max(egui::pos2(total.min.x, vp_rect.max.y), total.max);
    regions::record(ui.ctx(), regions::RegionSlot::BottomDock, strip_rect);
    ui.painter()
        .rect_filled(strip_rect, 0.0, tokens::bg_panel(state));
    ui.painter().rect_stroke(
        strip_rect,
        0.0,
        tokens::stroke_border_dyn(state),
        egui::StrokeKind::Inside,
    );
    let inner = strip_rect.shrink2(egui::vec2(8.0, 4.0));
    let mut strip_ui = ui.new_child(egui::UiBuilder::new().max_rect(inner));
    timeline::draw_contents(&mut strip_ui, state);
}

/// Centro do workspace UV (Wave 3 — §6.3): editor UV 2D + prévia 3D.
///
/// Janela larga: lado a lado. Janela estreita (<760px): alternador, nunca os
/// dois esmagados. A prévia 3D registra o `viewport_rect` que posiciona a
/// superfície GPU — nenhum acoplamento novo com o app.
fn uv_workspace_center(ui: &mut egui::Ui, state: &mut AppState) {
    puffin::profile_function!();
    let total = ui.available_rect_before_wrap();
    if total.width() < 760.0 {
        ui.horizontal(|ui| {
            let editor_label = state.t("uv.title");
            let preview_label = state.t("uv.preview_3d");
            if ui
                .selectable_label(!state.ui.uv_show_preview, editor_label)
                .clicked()
            {
                state.ui.uv_show_preview = false;
                state.mark_dirty();
            }
            if ui
                .selectable_label(state.ui.uv_show_preview, preview_label)
                .clicked()
            {
                state.ui.uv_show_preview = true;
                state.mark_dirty();
            }
        });
        ui.separator();
        if state.ui.uv_show_preview {
            viewport_3d(ui, state, ui.available_rect_before_wrap());
        } else {
            let pane = ui.available_rect_before_wrap();
            regions::record(ui.ctx(), regions::RegionSlot::UvEditor, pane);
            egui::ScrollArea::vertical()
                .id_salt("uv_editor_narrow_scroll")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    modules_ui::uv_ui::draw_uv_panel(ui, state);
                });
        }
    } else {
        let gap = ui.spacing().item_spacing.x;
        let left_w = (total.width() * 0.45).clamp(320.0, 560.0);
        let left = ui.allocate_ui_with_layout(
            egui::vec2(left_w, total.height()),
            egui::Layout::top_down_justified(egui::Align::LEFT),
            |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("uv_editor_scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        modules_ui::uv_ui::draw_uv_panel(ui, state);
                    });
            },
        );
        regions::record(ui.ctx(), regions::RegionSlot::UvEditor, left.response.rect);
        let right_rect = egui::Rect::from_min_max(
            egui::pos2(left.response.rect.max.x + gap, total.min.y),
            total.max,
        );
        viewport_3d(ui, state, right_rect);
    }
}

fn viewport_3d(ui: &mut egui::Ui, state: &mut AppState, rect: egui::Rect) {
    puffin::profile_function!();
    {
        let ctx = ui.ctx().clone();
        regions::record(&ctx, regions::RegionSlot::Viewport, rect);
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
        if viewport_interaction::draw(&ctx, state, rect, &p, &resp) {
            return;
        }

        // Barra contextual horizontal flutuante na base da viewport.
        // Posicionada dentro do `rect` da viewport (nunca da tela global).
        let shelf_rect = contextual_shelf::draw(ui, state, rect);
        if let Some(shelf) = shelf_rect {
            regions::record(&ctx, regions::RegionSlot::Shelf, shelf);
        }
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
        if resp.drag_started_by(egui::PointerButton::Primary)
            && let Some(pos) = resp.interact_pointer_pos()
        {
            state.ui.box_select_start = Some([pos.x, pos.y]);
        }
        if resp.dragged_by(egui::PointerButton::Primary)
            && let (Some(start), Some(curr)) =
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
        if resp.drag_stopped_by(egui::PointerButton::Primary)
            && let (Some(start), Some(curr)) = (
                state.ui.box_select_start.take(),
                resp.interact_pointer_pos(),
            )
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
                let vp = state.session.camera.view_proj().to_cols_array();
                let _ = state.dispatch(&petunia_core::BoxSelectCmd {
                    p0,
                    p1,
                    view_proj: vp,
                    add: shift,
                });
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
    }
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

/// Diálogo para selecionar e adicionar uma imagem de referência à cena via in-canvas picker.
pub fn pick_and_add_reference_image(_state: &mut AppState) {
    file_dialog_service::open_reference_image_dialog(None);
}

#[cfg(test)]
mod paint_tests;

#[cfg(test)]
mod cutting_tests;
