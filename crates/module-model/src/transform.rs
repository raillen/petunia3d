use petunia_core::AppState;

use super::Tool;

#[derive(Default)]
pub struct TransformTool;

impl Tool for TransformTool {
    fn id(&self) -> &'static str {
        "transform"
    }
    fn label_key(&self) -> &'static str {
        "tools.transform"
    }
    fn hint_key(&self) -> &'static str {
        "hints.transform"
    }
    fn icon(&self) -> &'static str {
        "✥"
    }
    fn shortcut(&self) -> &'static str {
        "G"
    }
    fn on_activate(&self, state: &mut AppState) {
        state.pending_modal = Some(petunia_core::ModalKind::Move);
        state.mark_dirty();
    }
}

impl TransformTool {
    pub fn apply_move(state: &mut AppState) {
        let d = state.transform_delta;
        state.checkpoint("move");
        if let Some(m) = state.project.active_mesh_mut() {
            m.translate_selected(d);
        }
        state.transform_delta = [0.0; 3];
        state.set_status(format!("move {d:?}"));
        state.sync_selection();
        state.emit_mesh_changed();
    }

    pub fn apply_scale(state: &mut AppState) {
        let s = state.transform_scale;
        state.checkpoint("scale");
        if let Some(m) = state.project.active_mesh_mut() {
            let c = m.selection_center();
            m.scale_selected(s, c);
        }
        state.transform_scale = 1.0;
        state.sync_selection();
        state.emit_mesh_changed();
    }

    pub fn apply_duplicate(state: &mut AppState) {
        state.checkpoint("duplicate");
        if let Some(m) = state.project.active_mesh_mut() {
            m.duplicate_selected();
        }
        state.sync_selection();
        state.emit_mesh_changed();
    }

    pub fn apply_delete(state: &mut AppState) {
        state.checkpoint("delete");
        if let Some(m) = state.project.active_mesh_mut() {
            m.delete_selected();
        }
        state.sync_selection();
        state.emit_mesh_changed();
    }
}
