//! Floating Tool Properties owned by the viewport, not the Object Inspector.
//!
//! This is the transitional convergence boundary: interactive modal tools
//! use the canonical `tool_fields`, while non-modal modeling tools reuse
//! their existing controls in a viewport-local card until every tool has
//! a descriptor-backed session.

use egui::{Align2, RichText, Ui, Vec2};
use petunia_core::{AppState, Workspace};

pub fn draw(ui: &mut Ui, state: &mut AppState, viewport: egui::Rect) {
    if state.workspace != Workspace::Model || state.is_active_locked() {
        return;
    }
    let active = state.active_tool.clone();
    let show = state.modal.is_some()
        || state.pending_modal.is_some()
        || matches!(
            active.as_str(),
            "primitives"
                | "extrude"
                | "inset"
                | "bevel"
                | "pushpull"
                | "slice"
                | "subdivide"
                | "draw_profile"
                | "merge"
                | "connect"
                | "dissolve"
                | "revolve"
        );
    if !show {
        return;
    }

    let title = state
        .modal
        .as_ref()
        .map(|modal| modal.kind.label().to_string())
        .unwrap_or_else(|| {
            let translated = state.t(&format!("tools.{active}"));
            if translated == format!("tools.{active}") {
                active.clone()
            } else {
                translated
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
                ui.set_max_width(286.0);
                ui.label(RichText::new(&title).strong().size(12.0));
                ui.separator();

                if state.modal.is_some() {
                    crate::tool_fields::draw(ui, state);
                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button(state.t("actions.apply")).clicked() {
                            state.commit_modal();
                        }
                        if ui.button(state.t("actions.cancel")).clicked() {
                            state.cancel_modal();
                        }
                    });
                } else {
                    crate::modules_ui::model_ui::draw_tool_panel(ui, state, &active);
                }
            });
        });
}
