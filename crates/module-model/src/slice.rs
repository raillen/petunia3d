use glam::Vec3;
use petunia_core::AppState;

use super::Tool;

#[derive(Default)]
pub struct SliceTool;

impl Tool for SliceTool {
    fn id(&self) -> &'static str {
        "slice"
    }
    fn label_key(&self) -> &'static str {
        "tools.slice"
    }
    fn hint_key(&self) -> &'static str {
        "hints.slice"
    }
    fn icon(&self) -> &'static str {
        "✂"
    }
    fn shortcut(&self) -> &'static str {
        "Shift+K"
    }
    fn ui(&self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        let l_title = state.t("tools.slice");
        let l_go = state.t("actions.slice");
        ui.label(l_title);

        ui.horizontal(|ui| {
            if ui.button("Slice X").clicked() {
                Self::apply_slice(state, Vec3::X, false);
            }
            if ui.button("Slice Y").clicked() {
                Self::apply_slice(state, Vec3::Y, false);
            }
            if ui.button("Slice Z").clicked() {
                Self::apply_slice(state, Vec3::Z, false);
            }
        });

        if ui.button(format!("{l_go} (Cap)")).clicked() {
            let cam_dir = state.camera.forward();
            Self::apply_slice(state, cam_dir, true);
        }
    }
    fn on_activate(&self, state: &mut AppState) {
        state.set_status(state.t("hints.slice"));
    }
}

impl SliceTool {
    fn apply_slice(state: &mut AppState, normal: Vec3, cap: bool) {
        state.checkpoint("slice");
        let center = if let Some(m) = state.project.active_mesh() {
            Vec3::from(m.selection_center())
        } else {
            Vec3::ZERO
        };

        if let Some(m) = state.project.active_mesh_mut() {
            m.slice_plane(center, normal, cap);
        }
        state.sync_selection();
        state.emit_mesh_changed();
    }
}
