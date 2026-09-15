//! In-app help and release-notes rendering (`egui_commonmark`, P1-17).
//!
//! OPTIONAL, behind the `help-markdown` feature. Markdown is presentation
//! only: source strings stay Petunia-owned translation keys, and no Markdown
//! renderer type leaks outside this adapter.

use egui_commonmark::{CommonMarkCache, CommonMarkViewer};

/// Renders a Markdown help document inside the current UI.
/// The render cache is frame-local: help bodies carry no images, so no
/// persistent cache is needed.
pub fn render_help(ui: &mut egui::Ui, markdown: &str) {
    let mut cache = CommonMarkCache::default();
    CommonMarkViewer::new().show(ui, &mut cache, markdown);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_renders_without_panic() {
        let ctx = egui::Context::default();
        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                render_help(
                    ui,
                    "# Help\n\n- **Bold** item\n- `code` item\n\n[link](https://example.com)",
                );
            });
        })
        .textures_delta
        .clear();
    }
}
