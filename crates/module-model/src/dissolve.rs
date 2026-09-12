use petunia_core::AppState;

use super::Tool;

#[derive(Default)]
pub struct DissolveTool;

impl Tool for DissolveTool {
    fn id(&self) -> &'static str {
        "dissolve"
    }
    fn label_key(&self) -> &'static str {
        "tools.dissolve"
    }
    fn hint_key(&self) -> &'static str {
        "hints.dissolve"
    }
    fn icon(&self) -> &'static str {
        "⌫"
    }
    fn shortcut(&self) -> &'static str {
        "X"
    }
    fn ui(&self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        let l_title = state.t("tools.dissolve");
        let l_go = state.t("actions.dissolve");
        ui.label(l_title);
        if ui.button(l_go).clicked() {
            Self::apply(state);
        }
    }
    fn on_activate(&self, state: &mut AppState) {
        state.set_status(state.t("hints.dissolve"));
    }
}

impl DissolveTool {
    fn apply(state: &mut AppState) {
        state.checkpoint("dissolve");
        if let Some(m) = state.project.active_mesh_mut() {
            m.dissolve_selected();
        }
        state.sync_selection();
        state.emit_mesh_changed();
    }
}
