use petunia_core::AppState;

use super::Tool;

/// Push/Pull (§8.7): empurra/puxa faces ao longo da normal, resposta imediata.
#[derive(Default)]
pub struct PushPullTool;

impl Tool for PushPullTool {
    fn id(&self) -> &'static str {
        "pushpull"
    }
    fn label_key(&self) -> &'static str {
        "tools.pushpull"
    }
    fn hint_key(&self) -> &'static str {
        "hints.pushpull"
    }
    fn icon(&self) -> &'static str {
        "⇅"
    }
    fn shortcut(&self) -> &'static str {
        "P"
    }
    fn on_activate(&self, state: &mut AppState) {
        state.pending_modal = Some(petunia_core::ModalKind::PushPull);
        state.mark_dirty();
    }
}

impl PushPullTool {
    pub fn apply(state: &mut AppState) {
        let d = state.push_dist;
        state.checkpoint("push/pull");
        if let Some(m) = state.project.active_mesh_mut() {
            m.push_pull(d);
        }
        state.sync_selection();
        state.emit_mesh_changed();
    }
}
