use petunia_core::AppState;

use super::Tool;

/// Mirror simples e previsível (§8.11): eixo + weld no plano.
#[derive(Default)]
pub struct MirrorTool;
impl Tool for MirrorTool {
    fn id(&self) -> &'static str {
        "mirror"
    }
    fn label_key(&self) -> &'static str {
        "tools.mirror"
    }
    fn hint_key(&self) -> &'static str {
        "hints.mirror"
    }
    fn icon(&self) -> &'static str {
        "◧"
    }
    fn shortcut(&self) -> &'static str {
        "Ctrl+M"
    }
    fn ui(&self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        let l_title = state.t("tools.mirror");
        let l_go = state.t("actions.mirror");
        let l_weld = state.t("actions.weld_eps");
        ui.label(l_title);
        ui.horizontal(|ui| {
            for (ax, name) in [(0, "X"), (1, "Y"), (2, "Z")] {
                if ui.selectable_label(state.mirror_axis == ax, name).clicked() {
                    state.mirror_axis = ax;
                    state.mark_dirty();
                }
            }
        });
        if ui
            .add(egui::Slider::new(&mut state.mirror_weld, 0.0..=0.05).text(l_weld))
            .changed()
        {
            state.mark_dirty();
        }
        if ui.button(l_go).clicked() {
            Self::apply(state);
        }
    }
    fn on_activate(&self, state: &mut AppState) {
        state.set_status(state.t("hints.mirror"));
        state.mark_dirty();
    }
}

impl MirrorTool {
    fn apply(state: &mut AppState) {
        let (ax, weld) = (state.mirror_axis, state.mirror_weld);
        state.checkpoint("mirror");
        if let Some(m) = state.project.active_mesh_mut() {
            m.mirror(ax, weld);
        }
        state.sync_selection();
        state.emit_mesh_changed();
    }
}
