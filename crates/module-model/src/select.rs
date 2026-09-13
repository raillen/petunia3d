use petunia_core::{AppState, ClearSelectionCmd, InvertSelectionCmd, SelectAllCmd, SelectMode};

use super::Tool;

#[derive(Default)]
pub struct SelectTool;
impl Tool for SelectTool {
    fn id(&self) -> &'static str {
        "select"
    }
    fn label_key(&self) -> &'static str {
        "tools.select"
    }
    fn hint_key(&self) -> &'static str {
        "hints.select"
    }
    fn icon(&self) -> &'static str {
        "⬒"
    }
    fn shortcut(&self) -> &'static str {
        "1"
    }
    fn ui(&self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        let l_title = state.t("tools.select");
        let l_v = state.t("modes.vertex");
        let l_e = state.t("modes.edge");
        let l_f = state.t("modes.face");
        let l_all = state.t("actions.select_all");
        let l_none = state.t("actions.deselect");
        ui.label(l_title);
        ui.horizontal(|ui| {
            if ui
                .selectable_label(state.select_mode == SelectMode::Vertex, l_v)
                .clicked()
            {
                state.select_mode = SelectMode::Vertex;
                state.mark_dirty();
            }
            if ui
                .selectable_label(state.select_mode == SelectMode::Edge, l_e)
                .clicked()
            {
                state.select_mode = SelectMode::Edge;
                if let Some(m) = state.project.active_mesh_mut() {
                    m.sync_edge_selection_from_verts();
                }
                state.mark_dirty();
            }
            if ui
                .selectable_label(state.select_mode == SelectMode::Face, l_f)
                .clicked()
            {
                state.select_mode = SelectMode::Face;
                if let Some(m) = state.project.active_mesh_mut() {
                    m.sync_face_selection_from_verts();
                }
                state.mark_dirty();
            }
        });
        let l_invert = state.t("actions.invert");
        let l_linked = state.t("actions.select_linked");
        ui.horizontal(|ui| {
            if ui.button(l_all).clicked() {
                let _ = state.dispatch(&SelectAllCmd);
            }
            if ui.button(l_none).clicked() {
                let _ = state.dispatch(&ClearSelectionCmd);
            }
        });
        ui.horizontal(|ui| {
            if ui.button(l_invert).clicked() {
                let _ = state.dispatch(&InvertSelectionCmd);
            }
            if ui.button(l_linked).clicked() {
                if let Some(m) = state.project.active_mesh_mut() {
                    m.select_linked();
                }
                state.sync_selection();
            }
        });
    }
}
