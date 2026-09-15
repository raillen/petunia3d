use petunia_core::AppState;

use super::Tool;

/// Bevel simples (§8.8): chanfro em arestas manifold selecionadas.
#[derive(Default)]
pub struct BevelTool;

impl Tool for BevelTool {
    fn id(&self) -> &'static str {
        "bevel"
    }
    fn label_key(&self) -> &'static str {
        "tools.bevel"
    }
    fn hint_key(&self) -> &'static str {
        "hints.bevel"
    }
    fn icon(&self) -> &'static str {
        "◐"
    }
    fn shortcut(&self) -> &'static str {
        "Ctrl+B"
    }
    fn on_activate(&self, state: &mut AppState) {
        state.pending_modal = Some(petunia_core::ModalKind::Bevel);
        state.mark_dirty();
    }
}

impl BevelTool {
    pub fn apply(state: &mut AppState) {
        let amt = state.bevel_amount;
        let segs = state.bevel_segments.clamp(1, 4);
        state.checkpoint("bevel");
        let (ok, skipped) = state
            .project
            .active_mesh_mut()
            .map(|m| {
                if segs > 1 {
                    m.bevel_selected_segments(amt, segs)
                } else {
                    m.bevel_selected(amt)
                }
            })
            .unwrap_or((0, 0));
        if skipped > 0 {
            state.set_status(format!(
                "bevel: {ok} ok, {skipped} ignoradas (não-manifold)"
            ));
        } else {
            state.set_status(format!("bevel: {ok} arestas"));
        }
        state.sync_selection();
        state.emit_mesh_changed();
    }
}
