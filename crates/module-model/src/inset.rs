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
    fn ui(&self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        let l_title = state.t("tools.inset");
        let l_factor = state.t("actions.factor");
        let l_go = state.t("actions.inset");
        ui.label(l_title);
        if ui
            .add(egui::Slider::new(&mut state.inset_factor, 0.0..=0.9).text(l_factor))
            .changed()
        {
            state.mark_dirty();
        }
        if ui.button(l_go).clicked() {
            let f = state.inset_factor;
            state.checkpoint("inset");
            if let Some(m) = state.project.active_mesh_mut() {
                m.inset_selected(f);
            }
            state.sync_selection();
            state.emit_mesh_changed();
        }
    }
    fn on_activate(&self, state: &mut AppState) {
        state.pending_modal = Some(petunia_core::ModalKind::Inset);
        state.mark_dirty();
    }
}
