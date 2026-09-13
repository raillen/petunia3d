use petunia_core::AppState;

use super::Tool;

#[derive(Default)]
pub struct InsetTool;

impl Tool for InsetTool {
    fn id(&self) -> &'static str {
        "inset"
    }
    fn label_key(&self) -> &'static str {
        "tools.inset"
    }
    fn hint_key(&self) -> &'static str {
        "hints.inset"
    }
    fn icon(&self) -> &'static str {
        "◫"
    }
    fn shortcut(&self) -> &'static str {
        "I"
    }
    fn on_activate(&self, state: &mut AppState) {
        state.pending_modal = Some(petunia_core::ModalKind::Inset);
        state.mark_dirty();
    }
}

impl InsetTool {
    pub fn apply(state: &mut AppState) {
        let f = state.inset_factor;
        state.checkpoint("inset");
        if let Some(m) = state.project.active_mesh_mut() {
            m.inset_selected(f);
        }
        state.sync_selection();
        state.emit_mesh_changed();
    }
}
