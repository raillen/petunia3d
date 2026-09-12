use petunia_core::AppState;

use super::Tool;

#[derive(Default)]
pub struct ExtrudeTool;
impl Tool for ExtrudeTool {
    fn id(&self) -> &'static str {
        "extrude"
    }
    fn label_key(&self) -> &'static str {
        "tools.extrude"
    }
    fn hint_key(&self) -> &'static str {
        "hints.extrude"
    }
    fn icon(&self) -> &'static str {
        "⬆"
    }
    fn shortcut(&self) -> &'static str {
        "E"
    }
    fn ui(&self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        let l_title = state.t("tools.extrude");
        let l_dist = state.t("actions.distance");
        let l_go = state.t("actions.extrude");
        ui.label(l_title);
        if ui
            .add(egui::Slider::new(&mut state.extrude_dist, -2.0..=2.0).text(l_dist))
            .changed()
        {
            state.mark_dirty();
        }
        if ui.button(l_go).clicked() {
            let d = state.extrude_dist;
            state.checkpoint("extrude");
            if let Some(m) = state.project.active_mesh_mut() {
                m.extrude_selected(d);
            }
            state.set_status(format!("extrude {d}"));
            state.sync_selection();
            state.emit_mesh_changed();
        }
    }
    fn on_activate(&self, state: &mut AppState) {
        state.pending_modal = Some(petunia_core::ModalKind::Extrude);
        state.mark_dirty();
    }
}
