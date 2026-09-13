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
    fn on_activate(&self, state: &mut AppState) {
        state.set_status(state.t("hints.merge"));
        state.mark_dirty();
    }
}

impl MergeTool {
    pub fn apply(state: &mut AppState) {
        state.checkpoint("merge");
        if let Some(m) = state.project.active_mesh_mut() {
            m.merge_center();
        }
        state.sync_selection();
        state.emit_mesh_changed();
    }
}
