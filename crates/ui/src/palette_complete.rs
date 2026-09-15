//! Command-palette completion (`egui_autocomplete`, P1-18).
//!
//! OPTIONAL, behind the `palette-autocomplete` feature. The widget only
//! completes text: it returns [`CommandIds`](petunia_core::command) search
//! items owned by Petunia, and command semantics stay in the dispatcher.

use egui_autocomplete::AutoCompleteTextEdit;

/// Search field with dropdown completion over Petunia command ids.
///
/// Returns the text-field response. Selecting a suggestion fills `query`;
/// execution still goes through the regular palette path.
pub fn command_search(
    ui: &mut egui::Ui,
    id: egui::Id,
    query: &mut String,
    command_ids: &[String],
    hint: &str,
) -> egui::Response {
    let hint = hint.to_string();
    ui.add(
        AutoCompleteTextEdit::new(query, command_ids)
            .max_suggestions(8)
            .highlight_matches(true)
            .set_text_edit_properties(move |edit| edit.hint_text(hint).id(id)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completion_field_renders_without_panic() {
        let ctx = egui::Context::default();
        let mut query = "ext".to_string();
        let ids = vec![
            "model.extrude".to_string(),
            "model.extrude_individual".to_string(),
            "global.undo".to_string(),
        ];
        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                let _ = command_search(
                    ui,
                    egui::Id::new("palette-test"),
                    &mut query,
                    &ids,
                    "Type a command…",
                );
            });
        })
        .textures_delta
        .clear();
        assert_eq!(query, "ext");
    }
}
