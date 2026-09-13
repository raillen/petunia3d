//! Painel de Pintura 2D e Vertex Paint (`paint_ui`).
//! Renderiza controles de cores, paleta, sliders de pincel e canvas interativo de textura.

use egui::{Color32, Context, Ui};
use petunia_core::AppState;
use petunia_module_paint::PaintModule;

pub fn draw_paint_panel(
    ctx: &Context,
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
                if ui
                    .small_button("+ Add")
                    .on_hover_text("Add current color to palette")
                    .clicked()
                {
                    let c = state.paint_color;
                    PaintModule::push_palette(state, c);
                }
                if ui.small_button("Clear").clicked() {
                    PaintModule::set_palette(state, Vec::new());
                }
                if ui.small_button("PICO-8").clicked() {
                    PaintModule::set_palette(state, petunia_project::preset_pico8());
                    state.set_status("Loaded PICO-8 palette");
                }
                if ui.small_button("GameBoy").clicked() {
                    PaintModule::set_palette(state, petunia_project::preset_gameboy());
                    state.set_status("Loaded Game Boy palette");
                }
                if ui
                    .small_button("Import")
                    .on_hover_text("Import .hex or .gpl palette")
                    .clicked()
                {
                    state
                        .events
                        .emit(petunia_core::AppEvent::RequestImportPalette);
                }
                if ui
                    .small_button("Export")
                    .on_hover_text("Export palette to .gpl")
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
                    state.set_status(format!("fill {n}"));
                    state.sync_selection();
                }
                if ui.button(l_pick).clicked() {
                    state.set_status(state.t("paint.pick_hint"));
                }
            });
        });

    // canvas 2D (albedo)
    let l_canvas = state.t("paint.canvas");
    let l_brush = state.t("paint.brush");
    let l_eraser = state.t("paint.eraser");
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
                if let Some(o) = state.project.assets.get(state.project.active) {
                    if let Some(cv) = &o.texture {
                        let img = egui::ColorImage::from_rgba_unmultiplied(
                            [cv.w as usize, cv.h as usize],
                            &cv.pixels,
                        );
                        *canvas_tex =
                            Some(ctx.load_texture("canvas", img, egui::TextureOptions::NEAREST));
                    }
                }
                state.render.canvas_dirty = false;
            }
            ui.horizontal(|ui| {
                if ui.button(l_brush).clicked() {
                    state.canvas_brush = state.canvas_brush.max(1);
                }
                if ui.button(l_eraser).clicked() {
                    state.set_status(state.t("paint.eraser_hint"));
                }
                if ui.button(l_cfill).clicked() {
                    state.checkpoint("canvas fill");
                    PaintModule::canvas_fill(state);
                    state.render.canvas_dirty = true;
                }
                if ui.button(l_clear).clicked() {
                    state.checkpoint("canvas clear");
                    if let Some(o) = state.project.active_mut() {
                        if let Some(cv) = o.texture.as_mut() {
                            cv.fill([0, 0, 0, 0]);
                        }
                    }
                    state.render.canvas_dirty = true;
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
                }
                state.render.canvas_dirty = true;
                state.mark_dirty();
            }
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
                // grade sutil p/ pixel art
                if scale >= 3.0 {
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
                if resp.dragged() || resp.clicked() {
                    if let Some(pos) = resp.interact_pointer_pos() {
                        let px = (((pos.x - rect.min.x) / scale) as u32).min(cw as u32 - 1);
                        let py = (((pos.y - rect.min.y) / scale) as u32).min(ch as u32 - 1);
                        if resp.drag_started() {
                            state.checkpoint("canvas paint");
                        }
                        PaintModule::canvas_brush(state, px, py, erase);
                        state.render.canvas_dirty = true;
                    }
                }
                if resp.drag_stopped() {
                    state.emit_mesh_changed();
                }
            }
            ui.small(state.t("paint.canvas_hint"));
        });
}
