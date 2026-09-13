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
    fn on_activate(&self, state: &mut AppState) {
        state.pending_modal = Some(petunia_core::ModalKind::Extrude);
        state.mark_dirty();
    }
}

impl ExtrudeTool {
    pub fn apply(state: &mut AppState) {
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
