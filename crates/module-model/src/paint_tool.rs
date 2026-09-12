//! Paint tool (MODEL workspace): entra no modo pintura de vértice.

use petunia_core::{AppState, EditMode};

use super::Tool;

#[derive(Default)]
pub struct PaintTool;
impl Tool for PaintTool {
    fn id(&self) -> &'static str {
        "paint"
    }
    fn label_key(&self) -> &'static str {
        "tools.paint"
    }
    fn hint_key(&self) -> &'static str {
        "hints.paint"
    }
    fn icon(&self) -> &'static str {
        "🖌"
    }
    fn shortcut(&self) -> &'static str {
        "B"
    }
    fn ui(&self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        let l_title = state.t("tools.paint");
        let l_radius = state.t("paint.radius");
        let l_strength = state.t("paint.strength");
        let l_hint = state.t("hints.paint");
        ui.label(l_title);
        let mut c = state.paint_color;
        if ui.color_edit_button_rgb(&mut c).changed() {
            state.checkpoint("brush color");
            state.paint_color = c;
            if let Some(o) = state.project.active_mut() {
                o.base_color = c;
            }
            state.mark_dirty();
        }
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
        ui.label(l_hint);
    }
    fn on_activate(&self, state: &mut AppState) {
        state.mode = EditMode::TexturePaint;
        state.mark_dirty();
    }
}
