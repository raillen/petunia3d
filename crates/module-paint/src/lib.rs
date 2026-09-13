//! module-paint — workspace PAINT (§11): vertex paint + canvas 2D.
//! Brush/soft/fill/eyedropper operam em vértices; o canvas alimenta o
//! preview texturizado (albedo). Layers: baseline futuro — V1 usa 1 layer.

use petunia_core::{AppState, Module};

#[derive(Default)]
pub struct PaintModule {
    pub canvas_tex: Option<egui::TextureHandle>,
}

impl PaintModule {
    pub fn new() -> Self {
        Self { canvas_tex: None }
    }

    /// Preenche seleção (ou tudo) com a cor atual. Retorna nº de verts.
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
            state.undo.checkpoint("fill", &before);
            state.emit_mesh_changed();
        }
        n
    }

    /// Eyedropper: copia a cor do vértice para o pincel.
    pub fn eyedrop_vertex(state: &mut AppState, vi: usize) {
        if let Some(o) = state.project.assets.get(state.project.active) {
            if let Some(v) = o.mesh.verts.get(vi) {
                state.paint_color = v.color;
                state.mark_dirty();
            }
        }
    }

    /// Registra cor na palette (recentes, máx 16) e sincroniza com o projeto.
    pub fn push_palette(state: &mut AppState, c: [f32; 3]) {
        state
            .palette
            .retain(|&x| (x[0] - c[0]).abs() + (x[1] - c[1]).abs() + (x[2] - c[2]).abs() > 1e-3);
        state.palette.insert(0, c);
        state.palette.truncate(16);
        state.project.palette = state.palette.clone();
    }

    /// Substitui a paleta atual e sincroniza com o projeto.
    pub fn set_palette(state: &mut AppState, pal: Vec<[f32; 3]>) {
        state.palette = pal.clone();
        state.project.palette = pal;
        state.mark_dirty();
    }

    /// Abre diálogo para importar paleta (.hex ou .gpl).
    pub fn import_palette_dialog(state: &mut AppState) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Palette (*.hex, *.gpl)", &["hex", "gpl"])
            .pick_file()
        {
            if let Ok(content) = std::fs::read_to_string(&path) {
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                let colors = if ext.eq_ignore_ascii_case("gpl") {
                    petunia_project::palette::import_gpl(&content)
                } else {
                    petunia_project::palette::import_hex(&content)
                };
                if !colors.is_empty() {
                    let count = colors.len();
                    Self::set_palette(state, colors);
                    state.set_status(format!("imported {count} colors"));
                } else {
                    state.set_status("no valid colors found in palette file".to_string());
                }
            }
        }
    }

    /// Abre diálogo para exportar paleta (.gpl).
    pub fn export_palette_dialog(state: &mut AppState) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("GIMP Palette (*.gpl)", &["gpl"])
            .set_file_name("palette.gpl")
            .save_file()
        {
            let gpl = petunia_project::palette::export_gpl("Petunia Palette", &state.palette);
            if std::fs::write(&path, gpl).is_ok() {
                state.set_status(format!("exported palette to {}", path.display()));
            }
        }
    }

    // ---- canvas 2D ----

    pub fn has_canvas(state: &AppState) -> bool {
        state
            .project
            .assets
            .get(state.project.active)
            .map(|o| o.texture.is_some())
            .unwrap_or(true)
    }

    pub fn ensure_canvas(state: &mut AppState) {
        if let Some(o) = state.project.active_mut() {
            if o.texture.is_none() {
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
                state.mark_dirty();
            }
        }
    }

    pub fn canvas_brush(state: &mut AppState, x: u32, y: u32, erase: bool) {
        Self::ensure_canvas(state);
        let (col, r) = if erase {
            ([0, 0, 0, 0], state.canvas_brush)
        } else {
            let c = state.paint_color;
            (
                [
                    (c[0] * 255.0) as u8,
                    (c[1] * 255.0) as u8,
                    (c[2] * 255.0) as u8,
                    255,
                ],
                state.canvas_brush,
            )
        };
        if let Some(o) = state.project.active_mut() {
            if let Some(cv) = o.texture.as_mut() {
                let r = r as i32;
                for dy in -r..=r {
                    for dx in -r..=r {
                        if dx * dx + dy * dy <= r * r {
                            cv.set(
                                (x as i32 + dx).max(0) as u32,
                                (y as i32 + dy).max(0) as u32,
                                col,
                            );
                        }
                    }
                }
                state.mark_dirty();
            }
        }
    }

    pub fn canvas_fill(state: &mut AppState) {
        Self::ensure_canvas(state);
        let c = state.paint_color;
        let col = [
            (c[0] * 255.0) as u8,
            (c[1] * 255.0) as u8,
            (c[2] * 255.0) as u8,
            255,
        ];
        if let Some(o) = state.project.active_mut() {
            if let Some(cv) = o.texture.as_mut() {
                cv.fill(col);
                state.mark_dirty();
            }
        }
    }
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

impl PaintModule {
    pub fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        // vertex paint rápido
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
                let pal = state.palette.clone();
                ui.horizontal_wrapped(|ui| {
                    for col in pal {
                        let (r, gg, b) = (
                            (col[0] * 255.0) as u8,
                            (col[1] * 255.0) as u8,
                            (col[2] * 255.0) as u8,
                        );
                        if ui
                            .color_edit_button_srgba(&mut egui::Color32::from_rgb(r, gg, b))
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
                        PaintModule::import_palette_dialog(state);
                    }
                    if ui
                        .small_button("Export")
                        .on_hover_text("Export palette to .gpl")
                        .clicked()
                    {
                        PaintModule::export_palette_dialog(state);
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
                // R2: criação passiva do canvas também é checkpoint (era un-undoable)
                if !PaintModule::has_canvas(state) {
                    state.checkpoint("canvas new");
                    PaintModule::ensure_canvas(state);
                }
                if state.canvas_dirty {
                    if let Some(o) = state.project.assets.get(state.project.active) {
                        if let Some(cv) = &o.texture {
                            let img = egui::ColorImage::from_rgba_unmultiplied(
                                [cv.w as usize, cv.h as usize],
                                &cv.pixels,
                            );
                            self.canvas_tex = Some(ctx.load_texture(
                                "canvas",
                                img,
                                egui::TextureOptions::NEAREST,
                            ));
                        }
                    }
                    state.canvas_dirty = false;
                }
                ui.horizontal(|ui| {
                    if ui.button(l_brush).clicked() {
                        state.canvas_brush = state.canvas_brush.max(1);
                    }
                    if ui.button(l_eraser).clicked() {
                        // borracha = pincel com erase via Ctrl (simplificação documentada)
                        state.set_status(state.t("paint.eraser_hint"));
                    }
                    if ui.button(l_cfill).clicked() {
                        state.checkpoint("canvas fill");
                        PaintModule::canvas_fill(state);
                        state.canvas_dirty = true;
                    }
                    if ui.button(l_clear).clicked() {
                        state.checkpoint("canvas clear");
                        if let Some(o) = state.project.active_mut() {
                            if let Some(cv) = o.texture.as_mut() {
                                cv.fill([0, 0, 0, 0]);
                            }
                        }
                        state.canvas_dirty = true;
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
                    state.canvas_dirty = true;
                    state.mark_dirty();
                }
                if let Some(tex) = self.canvas_tex.clone() {
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
                        egui::Color32::WHITE,
                    );
                    // grade sutil p/ pixel art
                    if scale >= 3.0 {
                        let p = ui.painter_at(rect);
                        let step = scale;
                        let mut x = rect.min.x;
                        while x <= rect.max.x {
                            p.line_segment(
                                [egui::pos2(x, rect.min.y), egui::pos2(x, rect.max.y)],
                                egui::Stroke::new(0.5_f32, egui::Color32::from_black_alpha(40)),
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
                            state.canvas_dirty = true;
                        }
                    }
                    if resp.drag_stopped() {
                        state.emit_mesh_changed();
                    }
                }
                ui.small(state.t("paint.canvas_hint"));
            });
    }

    // ------------------------------------------------------------------ UV
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_set_palette_sync() {
        let mut state = AppState::new("en");
        let initial_len = state.palette.len();
        assert_eq!(state.project.palette.len(), initial_len);

        PaintModule::push_palette(&mut state, [0.5, 0.5, 0.5]);
        assert_eq!(state.palette[0], [0.5, 0.5, 0.5]);
        assert_eq!(state.project.palette[0], [0.5, 0.5, 0.5]);

        let p8 = petunia_project::preset_pico8();
        PaintModule::set_palette(&mut state, p8.clone());
        assert_eq!(state.palette.len(), 16);
        assert_eq!(state.project.palette.len(), 16);
        assert_eq!(state.palette, p8);
        assert_eq!(state.project.palette, p8);
    }
}
