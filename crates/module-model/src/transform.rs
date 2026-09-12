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
    fn ui(&self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        let l_title = state.t("tools.transform");
        let l_move = state.t("actions.move");
        let l_scale = state.t("actions.scale");
        let l_apply = state.t("actions.apply_scale");
        let l_dup = state.t("actions.duplicate");
        let l_del = state.t("actions.delete");
        ui.label(l_title);
        let mut moved = false;
        moved |= ui
            .add(egui::Slider::new(&mut state.transform_delta[0], -3.0..=3.0).text("X"))
            .changed();
        moved |= ui
            .add(egui::Slider::new(&mut state.transform_delta[1], -3.0..=3.0).text("Y"))
            .changed();
        moved |= ui
            .add(egui::Slider::new(&mut state.transform_delta[2], -3.0..=3.0).text("Z"))
            .changed();
        if moved {
            state.mark_dirty();
        }
        if ui.button(l_move).clicked() {
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
        ui.add(egui::Slider::new(&mut state.transform_scale, 0.1..=3.0).text(l_scale));
        if ui.button(l_apply).clicked() {
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
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button(l_dup).clicked() {
                state.checkpoint("duplicate");
                if let Some(m) = state.project.active_mesh_mut() {
                    m.duplicate_selected();
                }
                state.sync_selection();
                state.emit_mesh_changed();
            }
            if ui.button(l_del).clicked() {
                state.checkpoint("delete");
                if let Some(m) = state.project.active_mesh_mut() {
                    m.delete_selected();
                }
                state.sync_selection();
                state.emit_mesh_changed();
            }
        });
    }
}
