//! Advanced responsive sublayout pilot (`egui_taffy`, P1-15).
//!
//! BASELINE CANDIDATE, not a migration: `egui_tiles` stays the controlled
//! macro-layout engine. This adapter pilots Flex row layout for one dense
//! sublayout (inspector key/value rows) behind [`flex_key_value_row`].
//! Native `ui.horizontal` remains the default everywhere else; broader
//! adoption needs an ADR proving complexity/performance wins.

/// Renders a dense key/value row with Flex layout: key left, value
/// right-aligned, gap handled by the flex container.
///
/// Returns the value response for interaction (click, hover, tooltip).
pub fn flex_key_value_row(
    ui: &mut egui::Ui,
    id: impl Into<egui::Id>,
    key: &str,
    value: &str,
) -> egui::Response {
    use egui_taffy::{TuiBuilderLogic, taffy};
    let mut out = None;
    egui_taffy::tui(ui, id)
        .reserve_available_width()
        .style(taffy::Style {
            flex_direction: taffy::FlexDirection::Row,
            justify_content: Some(taffy::JustifyContent::SpaceBetween),
            align_items: Some(taffy::AlignItems::Center),
            gap: taffy::Size {
                width: taffy::LengthPercentage::length(8.0),
                height: taffy::LengthPercentage::length(0.0),
            },
            ..Default::default()
        })
        .show(|tui| {
            tui.ui(|ui| {
                ui.label(
                    egui::RichText::new(key)
                        .size(11.0)
                        .color(crate::tokens::TEXT_SECONDARY),
                );
            });
            let response = tui.ui(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(value).size(11.0).strong());
                })
                .response
            });
            out = Some(response);
        });
    out.expect("taffy row always produces a value response")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flex_row_renders_key_and_value_without_panic() {
        let ctx = egui::Context::default();
        ctx.run_ui(egui::RawInput::default(), |ui| {
            // Multiple frames: taffy layout converges (sizing pass first).
            for n in 0..3 {
                let response = flex_key_value_row(ui, format!("taffy-pilot-{n}"), "Tris", "1_024");
                assert!(response.rect.width() >= 0.0);
            }
        })
        .textures_delta
        .clear();
    }
}
