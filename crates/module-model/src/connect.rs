use petunia_core::AppState;

use super::Tool;

#[derive(Default)]
pub struct ConnectTool;

impl Tool for ConnectTool {
    fn id(&self) -> &'static str {
        "connect"
    }
    fn label_key(&self) -> &'static str {
        "tools.connect"
    }
    fn hint_key(&self) -> &'static str {
        "hints.connect"
    }
    fn icon(&self) -> &'static str {
        "☍"
    }
    fn shortcut(&self) -> &'static str {
        "Ctrl+J"
    }
    fn ui(&self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        let l_title = state.t("tools.connect");
        let l_go = state.t("actions.connect");
        ui.label(l_title);
        if ui.button(l_go).clicked() {
            Self::apply(state);
        }
    }
    fn on_activate(&self, state: &mut AppState) {
        state.set_status(state.t("hints.connect"));
    }
}

impl ConnectTool {
    fn apply(state: &mut AppState) {
        let sel_faces: Vec<usize> = if let Some(m) = state.project.active_mesh() {
            m.faces
                .iter()
                .enumerate()
                .filter(|(_, f)| f.selected)
                .map(|(i, _)| i)
                .collect()
        } else {
            Vec::new()
        };

        if sel_faces.len() == 2 {
            state.checkpoint("connect");
            if let Some(m) = state.project.active_mesh_mut() {
                match m.connect_loops(sel_faces[0], sel_faces[1]) {
                    Ok(_) => state.set_status(state.t("status.connected")),
                    Err(e) => state.set_status(format!("{}: {e}", state.t("status.connect_err"))),
                }
            }
            state.sync_selection();
            state.emit_mesh_changed();
        } else {
            state.set_status(state.t("status.connect_need_2"));
        }
    }
}
