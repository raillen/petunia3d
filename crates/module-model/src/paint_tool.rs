//! Paint tool (MODEL workspace): entra no modo pintura de vértice.

use petunia_core::{AppState, EditMode};

use super::Tool;

#[derive(Default)]
pub struct PaintTool;
impl Tool for PaintTool {
    fn id(&self) -> &'static str {
        "paint"
    }
    fn label_key(&self) -> &'static str {
        "tools.paint"
    }
    fn hint_key(&self) -> &'static str {
        "hints.paint"
    }
    fn icon(&self) -> &'static str {
        "🖌"
    }
    fn shortcut(&self) -> &'static str {
        "B"
    }
    fn on_activate(&self, state: &mut AppState) {
        state.mode = EditMode::TexturePaint;
        state.mark_dirty();
    }
}
