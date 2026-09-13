use super::Tool;

#[derive(Default)]
pub struct SelectTool;

impl Tool for SelectTool {
    fn id(&self) -> &'static str {
        "select"
    }
    fn label_key(&self) -> &'static str {
        "tools.select"
    }
    fn hint_key(&self) -> &'static str {
        "hints.select"
    }
    fn icon(&self) -> &'static str {
        "⬒"
    }
    fn shortcut(&self) -> &'static str {
        "1"
    }
}
