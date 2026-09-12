//! Responsive camera controls shared by the viewport header.
//! Camera math belongs to core; this adapter cancels stale framing animations.

use petunia_core::{AppState, Projection, ViewPreset};

const VIEWS: [(ViewPreset, &str, &str); 6] = [
    (ViewPreset::Front, "camera.front", "Numpad 1"),
    (ViewPreset::Back, "camera.back", "Ctrl + Numpad 1"),
    (ViewPreset::Right, "camera.right", "Numpad 3"),
    (ViewPreset::Left, "camera.left", "Ctrl + Numpad 3"),
    (ViewPreset::Top, "camera.top", "Numpad 7"),
    (ViewPreset::Bottom, "camera.bottom", "Ctrl + Numpad 7"),
];

/// Draws a wrapping row without a horizontal scrollbar or hidden camera actions.
pub fn draw(ui: &mut egui::Ui, state: &mut AppState) {
    ui.add_enabled_ui(!state.is_interacting(), |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 6.0);
            for (projection, label) in [
                (Projection::Perspective, "camera.perspective"),
                (Projection::Ortho, "camera.orthographic"),
            ] {
                if ui
                    .selectable_label(state.camera.proj == projection, state.t(label))
                    .on_hover_text(format!(
                        "{}\nO · Numpad 5",
                        state.t("camera.projection_hint")
                    ))
                    .clicked()
                {
                    state.camera_frame = None;
                    state.camera.set_projection(projection);
                    state.mark_dirty();
                }
            }
            let current = state.camera.view_preset();
            let current_label = VIEWS
                .iter()
                .find(|(view, _, _)| Some(*view) == current)
                .map_or_else(|| state.t("camera.free"), |(_, label, _)| state.t(label));
            egui::ComboBox::from_id_salt("camera.view")
                .width(116.0)
                .selected_text(format!("{}: {current_label}", state.t("camera.view")))
                .show_ui(ui, |ui| {
                    for (view, label, shortcut) in VIEWS {
                        if ui
                            .selectable_label(current == Some(view), state.t(label))
                            .on_hover_text(shortcut)
                            .clicked()
                        {
                            state.camera_frame = None;
                            state.camera.set_preset(view);
                            state.mark_dirty();
                        }
                    }
                });
            ui.label(state.t("camera.visible_height"))
                .on_hover_text(state.t("camera.height_hint"));
            let mut height = state.camera.visible_height();
            let speed = (height * 0.005).max(0.001);
            if ui
                .add(
                    egui::DragValue::new(&mut height)
                        .range(0.1..=100_000.0)
                        .speed(speed)
                        .max_decimals(3)
                        .suffix(" m"),
                )
                .on_hover_text(state.t("camera.height_hint"))
                .changed()
            {
                state.camera_frame = None;
                state.camera.set_visible_height(height);
                state.mark_dirty();
            }
            if ui
                .button(state.t("view.frame"))
                .on_hover_text(state.t("camera.frame_hint"))
                .clicked()
            {
                crate::frame_selection(state);
            }
            if ui
                .button(state.t("camera.reset"))
                .on_hover_text(state.t("camera.reset_hint"))
                .clicked()
            {
                state.camera_frame = None;
                state.camera.reset();
                state.mark_dirty();
            }
        });
    });
}
