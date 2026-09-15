use petunia_core::{AppState, SymmetrizeCmd};

use super::Tool;

/// Symmetrize previsível (§8.11): copia +eixo para −eixo (ou o inverso) e
/// solda a costura. Ativação nunca muta geometria nem histórico.
#[derive(Default)]
pub struct SymmetrizeTool;

impl Tool for SymmetrizeTool {
    fn id(&self) -> &'static str {
        "symmetrize"
    }
    fn label_key(&self) -> &'static str {
        "tools.symmetrize"
    }
    fn hint_key(&self) -> &'static str {
        "hints.symmetrize"
    }
    fn icon(&self) -> &'static str {
        "⇄"
    }
    fn shortcut(&self) -> &'static str {
        "Alt+M"
    }
    fn on_activate(&self, state: &mut AppState) {
        state.set_status(state.t("hints.symmetrize"));
        state.mark_dirty();
    }
}

impl SymmetrizeTool {
    pub fn apply(state: &mut AppState) {
        let cmd = SymmetrizeCmd {
            axis: state.symmetrize_axis,
            positive_to_negative: state.symmetrize_pos_to_neg,
            eps: state.mirror_weld,
        };
        if let Err(err) = state.dispatch(&cmd) {
            state.set_status(format!("symmetrize: {err}"));
        }
    }
}
