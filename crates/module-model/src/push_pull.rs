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
    fn ui(&self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        let l_title = state.t("tools.pushpull");
        let l_dist = state.t("actions.distance");
        let l_go = state.t("actions.pushpull");
        ui.label(l_title);
        if ui
            .add(egui::Slider::new(&mut state.push_dist, -2.0..=2.0).text(l_dist))
            .changed()
        {
            state.mark_dirty();
        }
        if ui.button(l_go).clicked() {
            let d = state.push_dist;
            state.checkpoint("push/pull");
            if let Some(m) = state.project.active_mesh_mut() {
                m.push_pull(d);
            }
            state.sync_selection();
            state.emit_mesh_changed();
        }
    }
    fn on_activate(&self, state: &mut AppState) {
        state.pending_modal = Some(petunia_core::ModalKind::PushPull);
        state.mark_dirty();
    }
}
