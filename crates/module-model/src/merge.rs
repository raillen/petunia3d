use petunia_core::AppState;

use super::Tool;

#[derive(Default)]
pub struct MergeTool;
impl Tool for MergeTool {
    fn id(&self) -> &'static str {
        "merge"
    }
    fn label_key(&self) -> &'static str {
        "tools.merge"
    }
    fn hint_key(&self) -> &'static str {
        "hints.merge"
    }
    fn icon(&self) -> &'static str {
        "⛭"
    }
    fn shortcut(&self) -> &'static str {
        "M"
    }
    fn ui(&self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        let l_title = state.t("tools.merge");
        let l_go = state.t("actions.merge_center");
        ui.label(l_title);
        if ui.button(l_go).clicked() {
            Self::apply(state);
        }
    }
    fn on_activate(&self, state: &mut AppState) {
        state.set_status(state.t("hints.merge"));
        state.mark_dirty();
    }
}

impl MergeTool {
    fn apply(state: &mut AppState) {
        state.checkpoint("merge");
        if let Some(m) = state.project.active_mesh_mut() {
            m.merge_center();
        }
        state.sync_selection();
        state.emit_mesh_changed();
    }
}
