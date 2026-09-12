use petunia_core::AppState;

use super::Tool;

#[derive(Default)]
pub struct SubdivideTool;
impl Tool for SubdivideTool {
    fn id(&self) -> &'static str {
        "subdivide"
    }
    fn label_key(&self) -> &'static str {
        "tools.subdivide"
    }
    fn hint_key(&self) -> &'static str {
        "hints.subdivide"
    }
    fn icon(&self) -> &'static str {
        "⊞"
    }
    fn shortcut(&self) -> &'static str {
        "W"
    }
    fn ui(&self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        let l_title = state.t("tools.subdivide");
        let l_sub = state.t("actions.subdivide");
        let l_tri = state.t("actions.triangulate");
        ui.label(l_title);
        if ui.button(l_sub).clicked() {
            state.checkpoint("subdivide");
            if let Some(m) = state.project.active_mesh_mut() {
                m.subdivide_selected();
            }
            state.sync_selection();
            state.emit_mesh_changed();
        }
        if ui.button(l_tri).clicked() {
            state.checkpoint("triangulate");
            if let Some(m) = state.project.active_mesh_mut() {
                m.triangulate();
            }
            state.sync_selection();
            state.emit_mesh_changed();
        }
    }
    fn on_activate(&self, state: &mut AppState) {
        state.set_status(state.t("hints.subdivide"));
        state.mark_dirty();
    }
}
