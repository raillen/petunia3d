//! Painel de Pintura 2D e Vertex Paint (`paint_ui`).
//! Renderiza controles de cores, paleta, sliders de pincel e canvas interativo de textura.

use egui::{Color32, Ui};
use petunia_core::AppState;
use petunia_module_paint::PaintModule;

pub fn draw_paint_panel(
    ui: &mut Ui,
    state: &mut AppState,
    canvas_tex: &mut Option<egui::TextureHandle>,
) {
    let l_vpaint = state.t("paint.vertex");
    let l_fill = state.t("paint.fill_sel");
    let l_pick = state.t("paint.pick");
    let l_radius = state.t("paint.radius");
    let l_strength = state.t("paint.strength");

    egui::CollapsingHeader::new(l_vpaint)
        .default_open(true)
        .show(ui, |ui| {
            let mut c = state.paint_color;
            if ui.color_edit_button_rgb(&mut c).changed() {
                state.paint_color = c;
                PaintModule::push_palette(state, c);
                state.mark_dirty();
            }
            // palette
            let pal = state.project.palette.clone();
            ui.horizontal_wrapped(|ui| {
                for col in pal {
                    let (r, gg, b) = (
                        (col[0] * 255.0) as u8,
                        (col[1] * 255.0) as u8,
                        (col[2] * 255.0) as u8,
                    );
                    if ui
                        .color_edit_button_srgba(&mut Color32::from_rgb(r, gg, b))
                        .clicked()
                    {
                        state.paint_color = col;
                    }
                }
            });
            ui.horizontal_wrapped(|ui| {
                let add_label = state.t("paint.palette_add");
                if ui
                    .small_button(&add_label)
                    .on_hover_text(state.t("paint.palette_add_tip"))
                    .clicked()
                {
                    let c = state.paint_color;
                    PaintModule::push_palette(state, c);
                }
                let clear_label = state.t("paint.palette_clear");
                if ui.small_button(&clear_label).clicked() {
                    PaintModule::set_palette(state, Vec::new());
                }
                // Marcas registradas: sem tradução.
                if ui.small_button("PICO-8").clicked() {
                    PaintModule::set_palette(state, petunia_project::preset_pico8());
                    let msg = state.t("paint.palette_loaded_pico8");
                    state.set_status(&msg);
                }
                if ui.small_button("GameBoy").clicked() {
                    PaintModule::set_palette(state, petunia_project::preset_gameboy());
                    let msg = state.t("paint.palette_loaded_gameboy");
                    state.set_status(&msg);
                }
                let import_label = state.t("paint.palette_import");
                if ui
                    .small_button(&import_label)
                    .on_hover_text(state.t("paint.palette_import_tip"))
                    .clicked()
                {
                    state
                        .events
                        .emit(petunia_core::AppEvent::RequestImportPalette);
                }
                let export_label = state.t("paint.palette_export");
                if ui
                    .small_button(&export_label)
                    .on_hover_text(state.t("paint.palette_export_tip"))
                    .clicked()
                {
                    state
                        .events
                        .emit(petunia_core::AppEvent::RequestExportPalette);
                }
            });
            if ui
                .add(egui::Slider::new(&mut state.paint_radius, 0.1..=3.0).text(l_radius))
                .changed()
            {
                state.mark_dirty();
            }
            if ui
                .add(egui::Slider::new(&mut state.paint_strength, 0.05..=1.0).text(l_strength))
                .changed()
            {
                state.mark_dirty();
            }
            ui.horizontal(|ui| {
                if ui.button(l_fill).clicked() {
                    let n = PaintModule::fill_selection(state);
                    let msg = state.t("paint.fill_done").replace("{n}", &n.to_string());
                    state.set_status(&msg);
                    state.sync_selection();
                }
                if ui.button(l_pick).clicked() {
                    state.set_status(state.t("paint.pick_hint"));
                }
            });
        });

    // canvas 2D (albedo)
    let l_canvas = state.t("paint.canvas");
    let l_cfill = state.t("paint.fill");
    let l_clear = state.t("paint.clear");
    let l_new = state.t("paint.new_canvas");
    egui::CollapsingHeader::new(l_canvas)
        .default_open(true)
        .show(ui, |ui| {
            if !PaintModule::has_canvas(state) {
                state.checkpoint("canvas new");
                PaintModule::ensure_canvas(state);
            }
            if state.render.canvas_dirty {
                if let Some(o) = state.project.assets.get(state.project.active)
                    && let Some(cv) = &o.texture
                {
                    let img = egui::ColorImage::from_rgba_unmultiplied(
                        [cv.w as usize, cv.h as usize],
                        &cv.pixels,
                    );
                    *canvas_tex = Some(ui.ctx().load_texture(
                        "canvas",
                        img,
                        egui::TextureOptions::NEAREST,
                    ));
                }
                state.render.canvas_dirty = false;
            }
            // Pincéis em linhas de largura total (uma por pincel): `horizontal`
            // aninhado dentro de `horizontal_wrapped` nunca quebra linha e os
            // últimos pincéis transbordavam invisíveis além do painel.
            // `PetuniaToolbarButton` (não compacto) dá o mesmo contrato visual
            // da toolbar: ícone+rótulo, seleção, hover e anel de foco.
            let brushes = [
                (
                    0,
                    "paint.brush_pixel",
                    crate::icon_registry::PetuniaIcon::PaintBrush,
                ),
                (
                    1,
                    "paint.brush_soft",
                    crate::icon_registry::PetuniaIcon::PaintBrush,
                ),
                (
                    2,
                    "paint.brush_eraser",
                    crate::icon_registry::PetuniaIcon::PaintEraser,
                ),
                (
                    3,
                    "paint.brush_fill",
                    crate::icon_registry::PetuniaIcon::PaintFill,
                ),
                (
                    4,
                    "paint.brush_picker",
                    crate::icon_registry::PetuniaIcon::PaintPicker,
                ),
                (
                    5,
                    "paint.brush_line",
                    crate::icon_registry::PetuniaIcon::PaintLine,
                ),
                (
                    6,
                    "paint.brush_rect",
                    crate::icon_registry::PetuniaIcon::PaintRect,
                ),
            ];
            for (kind, key, icon) in brushes {
                let label = state.t(key);
                if crate::widgets::PetuniaToolbarButton::new(icon, &label)
                    .selected(state.paint_brush_kind == kind)
                    .compact(false)
                    .show(ui)
                    .clicked()
                {
                    state.paint_brush_kind = kind;
                }
            }

            let isolate_label = state.t("paint.isolate_faces");
            ui.checkbox(&mut state.paint_isolate_selection, isolate_label)
                .on_hover_text(state.t("paint.isolate_faces_tip"));
            let grid_label = state.t("paint.pixel_grid");
            ui.checkbox(&mut state.paint_pixel_grid, grid_label)
                .on_hover_text(state.t("paint.pixel_grid_tip"));

            // Canal alvo (V1: Albedo; resto desabilitado com motivo — P3D-062).
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(state.t("paint.channel"))
                        .size(11.0)
                        .color(crate::tokens::TEXT_SECONDARY),
                );
                egui::ComboBox::from_id_salt("paint_channel_combo")
                    .selected_text(state.t("paint.channel_albedo"))
                    .show_ui(ui, |ui| {
                        let albedo_label = state.t("paint.channel_albedo");
                        ui.selectable_value(
                            &mut state.paint_channel,
                            petunia_project::TextureChannel::Albedo,
                            albedo_label,
                        );
                        for key in [
                            "paint.channel_normal",
                            "paint.channel_roughness",
                            "paint.channel_metallic",
                            "paint.channel_emission",
                            "paint.channel_height",
                        ] {
                            let label = state.t(key);
                            ui.add_enabled_ui(false, |ui| {
                                let _ = ui.selectable_label(false, label);
                            });
                        }
                        ui.small(state.t("paint.channel_locked_tip"));
                    });
            });

            draw_layers_panel(ui, state);

            ui.horizontal(|ui| {
                if ui.button(l_cfill).clicked() {
                    state.checkpoint("canvas fill");
                    PaintModule::canvas_fill(state);
                    state.render.canvas_dirty = true;
                }
                if ui.button(l_clear).clicked() {
                    state.checkpoint("canvas clear");
                    PaintModule::canvas_clear(state);
                    state.mark_dirty();
                }
            });
            let l_size = state.t("paint.size");
            if ui
                .add(egui::Slider::new(&mut state.canvas_brush, 1..=32).text(l_size))
                .changed()
            {
                state.mark_dirty();
            }
            if ui.button(l_new).clicked() {
                state.checkpoint("canvas new");
                if let Some(o) = state.project.active_mut() {
                    let c = o.base_color;
                    o.texture = Some(petunia_project::Canvas::new(
                        256,
                        256,
                        [
                            (c[0] * 255.0) as u8,
                            (c[1] * 255.0) as u8,
                            (c[2] * 255.0) as u8,
                            255,
                        ],
                    ));
                    // Pilha recomeça junto (representação única, sem divergir).
                    o.paint_stack = None;
                }
                PaintModule::ensure_stack(state);
                PaintModule::composite_active(state);
                state.mark_dirty();
            }
            // Resize explícito (operação com checkpoint; mantém conteúdo).
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    egui::RichText::new(state.t("paint.canvas_size"))
                        .size(11.0)
                        .color(crate::tokens::TEXT_SECONDARY),
                );
                let cur = state
                    .project
                    .assets
                    .get(state.project.active)
                    .and_then(|o| o.texture.as_ref())
                    .map(|c| (c.w, c.h))
                    .unwrap_or((256, 256));
                for side in [64u32, 128, 256, 512, 1024] {
                    let sel = cur == (side, side);
                    if ui
                        .selectable_label(sel, format!("{side}"))
                        .on_hover_text(state.t("paint.canvas_resize"))
                        .clicked()
                        && !sel
                    {
                        state.checkpoint("canvas resize");
                        PaintModule::resize_canvas(state, side, side);
                    }
                }
            });
            if let Some(tex) = canvas_tex.as_ref() {
                let avail = ui.available_width().min(300.0);
                let (cw, ch) = state
                    .project
                    .assets
                    .get(state.project.active)
                    .and_then(|o| o.texture.as_ref())
                    .map(|c| (c.w as f32, c.h as f32))
                    .unwrap_or((256.0, 256.0));
                let scale = avail / cw;
                let size = egui::vec2(avail, ch * scale);
                let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());
                ui.painter_at(rect).image(
                    tex.id(),
                    rect,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    Color32::WHITE,
                );
                // grade sutil p/ pixel art (contextual: zoom + preferência).
                if state.paint_pixel_grid && scale >= 3.0 {
                    let p = ui.painter_at(rect);
                    let step = scale;
                    let mut x = rect.min.x;
                    while x <= rect.max.x {
                        p.line_segment(
                            [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                            egui::Stroke::new(0.5_f32, Color32::from_black_alpha(40)),
                        );
                        x += step;
                    }
                }
                let erase = ui.input(|i| i.modifiers.ctrl);
                // Formas (Line/Rectangle): arrastar define, soltar confirma.
                // Início/fim vivem em temp-data (efêmero, cap. 29).
                let shape_id = egui::Id::new("paint.shape_drag");
                let brush_type = if erase {
                    petunia_module_paint::BrushType::Eraser
                } else {
                    match state.paint_brush_kind {
                        1 => petunia_module_paint::BrushType::Soft,
                        2 => petunia_module_paint::BrushType::Eraser,
                        3 => petunia_module_paint::BrushType::Fill,
                        4 => petunia_module_paint::BrushType::Eyedropper,
                        5 => petunia_module_paint::BrushType::Line,
                        6 => petunia_module_paint::BrushType::Rectangle,
                        _ => petunia_module_paint::BrushType::Pixel,
                    }
                };
                let is_shape = matches!(
                    brush_type,
                    petunia_module_paint::BrushType::Line
                        | petunia_module_paint::BrushType::Rectangle
                );
                if is_shape {
                    if resp.drag_started()
                        && let Some(pos) = resp.interact_pointer_pos()
                    {
                        let px = (((pos.x - rect.min.x) / scale) as u32).min(cw as u32 - 1);
                        let py = (((pos.y - rect.min.y) / scale) as u32).min(ch as u32 - 1);
                        state.checkpoint("canvas shape");
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(shape_id, (px, py, px, py)));
                    }
                    if resp.dragged()
                        && let Some(pos) = resp.interact_pointer_pos()
                    {
                        let px = (((pos.x - rect.min.x) / scale) as u32).min(cw as u32 - 1);
                        let py = (((pos.y - rect.min.y) / scale) as u32).min(ch as u32 - 1);
                        ui.ctx().data_mut(|d| {
                            if let Some(start) = d.get_temp::<(u32, u32, u32, u32)>(shape_id) {
                                d.insert_temp(shape_id, (start.0, start.1, px, py));
                            }
                        });
                    }
                    // Preview da forma sobre o canvas.
                    if let Some((x0, y0, x1, y1)) = ui
                        .ctx()
                        .data(|d| d.get_temp::<(u32, u32, u32, u32)>(shape_id))
                    {
                        let to_screen = |(x, y): (u32, u32)| {
                            egui::pos2(rect.min.x + x as f32 * scale, rect.min.y + y as f32 * scale)
                        };
                        let p = ui.painter_at(rect);
                        let stroke =
                            egui::Stroke::new(1.5_f32, egui::Color32::WHITE.gamma_multiply(0.9));
                        match brush_type {
                            petunia_module_paint::BrushType::Rectangle => {
                                p.rect_stroke(
                                    egui::Rect::from_two_pos(
                                        to_screen((x0, y0)),
                                        to_screen((x1, y1)),
                                    ),
                                    0.0,
                                    stroke,
                                    egui::StrokeKind::Outside,
                                );
                            }
                            _ => {
                                p.line_segment([to_screen((x0, y0)), to_screen((x1, y1))], stroke);
                            }
                        }
                    }
                    if resp.drag_stopped()
                        && let Some((x0, y0, x1, y1)) = ui
                            .ctx()
                            .data(|d| d.get_temp::<(u32, u32, u32, u32)>(shape_id))
                    {
                        ui.ctx()
                            .data_mut(|d| d.remove::<(u32, u32, u32, u32)>(shape_id));
                        PaintModule::commit_shape(
                            state,
                            petunia_module_paint::ShapeStroke {
                                x0,
                                y0,
                                x1,
                                y1,
                                brush: brush_type,
                                color: [
                                    (state.paint_color[0] * 255.0) as u8,
                                    (state.paint_color[1] * 255.0) as u8,
                                    (state.paint_color[2] * 255.0) as u8,
                                    255,
                                ],
                                strength: state.paint_strength,
                            },
                        );
                        state.emit_mesh_changed();
                    }
                } else if (resp.dragged() || resp.clicked())
                    && let Some(pos) = resp.interact_pointer_pos()
                {
                    let px = (((pos.x - rect.min.x) / scale) as u32).min(cw as u32 - 1);
                    let py = (((pos.y - rect.min.y) / scale) as u32).min(ch as u32 - 1);
                    if resp.drag_started() {
                        state.checkpoint("canvas paint");
                    }
                    PaintModule::canvas_brush_advanced(
                        state,
                        px,
                        py,
                        brush_type,
                        state.canvas_brush,
                        state.paint_strength,
                    );
                    state.render.canvas_dirty = true;
                }
                if resp.drag_stopped() {
                    state.emit_mesh_changed();
                }
            }
            ui.small(state.t("paint.canvas_hint"));
        });
}

/// Nome padrão de camada nova, localizado (`{n}` = ordinal da pilha).
fn default_layer_name(state: &AppState, n: usize) -> String {
    state
        .t("paint.layer_default_name")
        .replace("{n}", &n.to_string())
}

/// Painel de camadas (P3D-061): lista compacta com ativa/visibilidade/
/// opacidade, add/delete/rename/reorder. Tudo com checkpoint próprio.
fn draw_layers_panel(ui: &mut Ui, state: &mut AppState) {
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(state.t("paint.layers"))
                .strong()
                .size(11.5),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .small_button("+")
                .on_hover_text(state.t("paint.layer_new"))
                .clicked()
            {
                state.checkpoint("layer add");
                PaintModule::ensure_stack(state);
                let active = state.project.active;
                let (w, h) = state
                    .project
                    .assets
                    .get(active)
                    .and_then(|o| o.texture.as_ref())
                    .map(|c| (c.w, c.h))
                    .unwrap_or((256, 256));
                let next_n = state
                    .project
                    .assets
                    .get(active)
                    .and_then(|o| o.paint_stack.as_ref())
                    .map(|s| s.layers.len() + 1)
                    .unwrap_or(2);
                // Nome resolvido antes do borrow mutável (i18n fora do empréstimo).
                let name = default_layer_name(state, next_n);
                if let Some(o) = state.project.assets.get_mut(active)
                    && let Some(stack) = o.paint_stack.as_mut()
                {
                    stack.add_layer(petunia_project::PaintLayer::new(name, w, h, [0, 0, 0, 0]));
                }
                PaintModule::composite_active(state);
            }
        });
    });

    PaintModule::ensure_stack(state);
    // Snapshot p/ desenhar sem borrow vivo; ações aplicadas depois.
    let rows: Vec<(uuid::Uuid, String, bool, f32)> = state
        .project
        .assets
        .get(state.project.active)
        .and_then(|o| o.paint_stack.as_ref())
        .map(|stack| {
            stack
                .layers
                .iter()
                .map(|l| (l.id, l.name.clone(), l.visible, l.opacity))
                .collect()
        })
        .unwrap_or_default();
    let active = state
        .project
        .assets
        .get(state.project.active)
        .and_then(|o| o.paint_stack.as_ref())
        .and_then(|s| s.active())
        .map(|l| l.id);

    enum LayerAction {
        Activate(uuid::Uuid),
        ToggleVisible(uuid::Uuid),
        Opacity(uuid::Uuid, f32),
        Rename(uuid::Uuid, String),
        MoveUp(usize),
        MoveDown(usize),
        Delete(uuid::Uuid),
    }
    let mut actions: Vec<LayerAction> = Vec::new();

    if rows.is_empty() {
        ui.small(state.t("paint.layer_empty"));
        return;
    }
    for (idx, (id, name, visible, opacity)) in rows.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(4.0, 0.0);
            let mut vis = *visible;
            if ui.checkbox(&mut vis, "").clicked() {
                actions.push(LayerAction::ToggleVisible(*id));
            }
            let sel = active == Some(*id);
            if ui.selectable_label(sel, name).clicked() {
                actions.push(LayerAction::Activate(*id));
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let can_delete = rows.len() > 1;
                let del_tip = state.t("paint.layer_delete");
                let down_tip = state.t("paint.layer_down");
                let up_tip = state.t("paint.layer_up");
                if ui
                    .add_enabled(can_delete, egui::Button::new("×"))
                    .on_hover_text(&del_tip)
                    .clicked()
                {
                    actions.push(LayerAction::Delete(*id));
                }
                if crate::widgets::chevron_toggle_dir(
                    ui,
                    &down_tip,
                    crate::widgets::ChevronDir::Down,
                )
                .clicked()
                {
                    actions.push(LayerAction::MoveDown(idx));
                }
                if crate::widgets::chevron_toggle_dir(ui, &up_tip, crate::widgets::ChevronDir::Up)
                    .clicked()
                {
                    actions.push(LayerAction::MoveUp(idx));
                }
            });
        });
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(4.0, 0.0);
            let mut op = *opacity;
            let resp = ui.add(
                egui::Slider::new(&mut op, 0.0..=1.0)
                    .text(state.t("paint.layer_opacity"))
                    .show_value(false),
            );
            if resp.drag_started() {
                state.checkpoint("layer opacity");
            }
            if resp.changed() {
                actions.push(LayerAction::Opacity(*id, op));
            }
            let mut new_name = name.clone();
            let resp = ui.add(
                egui::TextEdit::singleline(&mut new_name)
                    .desired_width(90.0)
                    .hint_text(name.clone()),
            );
            if resp.lost_focus() && new_name.trim() != name && !new_name.trim().is_empty() {
                actions.push(LayerAction::Rename(*id, new_name.trim().to_string()));
            }
        });
    }

    if actions.is_empty() {
        return;
    }
    // Aplica com 1 checkpoint por gesto (opacidade já carimbou no início).
    let needs_checkpoint = actions
        .iter()
        .any(|a| !matches!(a, LayerAction::Opacity(_, _)));
    if needs_checkpoint {
        state.checkpoint("layer edit");
    }
    for action in actions {
        let active = state.project.active;
        let Some(o) = state.project.assets.get_mut(active) else {
            continue;
        };
        let Some(stack) = o.paint_stack.as_mut() else {
            continue;
        };
        match action {
            LayerAction::Activate(id) => {
                stack.set_active(id);
            }
            LayerAction::ToggleVisible(id) => {
                if let Some(l) = stack.layers.iter_mut().find(|l| l.id == id) {
                    l.visible = !l.visible;
                }
            }
            LayerAction::Opacity(id, op) => {
                if let Some(l) = stack.layers.iter_mut().find(|l| l.id == id) {
                    l.opacity = op.clamp(0.0, 1.0);
                }
            }
            LayerAction::Rename(id, name) => {
                if let Some(l) = stack.layers.iter_mut().find(|l| l.id == id) {
                    l.name = name;
                }
            }
            LayerAction::MoveUp(idx) => {
                if idx > 0 {
                    stack.move_layer(idx, idx - 1);
                }
            }
            LayerAction::MoveDown(idx) => {
                stack.move_layer(idx, idx + 1);
            }
            LayerAction::Delete(id) => {
                stack.remove_layer(id);
            }
        }
    }
    PaintModule::composite_active(state);
}
