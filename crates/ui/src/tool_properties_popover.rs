//! Floating Tool Properties owned by the viewport, not the Object Inspector.

use egui::{Align2, RichText, Ui, Vec2};
use petunia_core::{AppState, Workspace};

pub fn draw(ui: &mut Ui, state: &mut AppState, viewport: egui::Rect) {
    if state.workspace != Workspace::Model
        || state.is_active_locked()
        || !crate::modeling_tool_properties::supports(state)
    {
        return;
    }

    let active = state.active_tool.clone();
    let title = state
        .modal
        .as_ref()
        .map(|modal| crate::tool_fields::kind_label(state, modal.kind))
        .or_else(|| {
            state
                .pending_modal
                .map(|kind| crate::tool_fields::kind_label(state, kind))
        })
        .unwrap_or_else(|| {
            if active == "transform" {
                state.t("tool_properties.universal_transform")
            } else {
                let key = format!("tools.{active}");
                let translated = state.t(&key);
                if translated == key {
                    active.clone()
                } else {
                    translated
                }
            }
        });

    let pos = viewport.left_top() + Vec2::new(18.0, 18.0);
    egui::Area::new(egui::Id::new("viewport.tool_properties"))
        .order(egui::Order::Foreground)
        .fixed_pos(pos)
        .pivot(Align2::LEFT_TOP)
        .show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_min_width(220.0);
                ui.set_max_width(300.0);
                ui.label(RichText::new(&title).strong().size(12.0));
                ui.small(state.t("tool_properties.title"));
                ui.separator();
                crate::modeling_tool_properties::draw(ui, state);
            });
        });
}
