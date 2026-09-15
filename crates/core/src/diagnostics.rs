//! Structured diagnostics boundary (`tracing`, P0-09).
//!
//! Domain code emits typed [`DiagnosticEvent`]s through this module instead of
//! ad-hoc logging. The sink (formatting, filtering, output) is configured once
//! by the application host (`petunia_app::diagnostics`, P0-10). Domain errors
//! stay typed; logs never replace user-facing diagnostics.

/// Stable diagnostic category. Categories are part of the product contract:
/// adding a variant is additive, renaming one is breaking.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DiagnosticCategory {
    /// Mesh/geometry validation and repair.
    Geometry,
    /// Project save/load, import/export, recovery.
    ProjectIo,
    /// Undo/redo command dispatch.
    Commands,
    /// Background jobs (validation, thumbnails, export preprocessing).
    Jobs,
    /// Plugin/MCP boundary activity.
    Plugins,
}

impl DiagnosticCategory {
    /// Stable snake_case name used as the `tracing` target suffix.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Geometry => "geometry",
            Self::ProjectIo => "project_io",
            Self::Commands => "commands",
            Self::Jobs => "jobs",
            Self::Plugins => "plugins",
        }
    }
}

/// A structured diagnostic event. Fields are fixed so sinks and tests can rely
/// on them; free-form text goes in `detail`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DiagnosticEvent {
    /// Stable category for filtering.
    pub category: DiagnosticCategory,
    /// Short machine-readable code, e.g. `"mesh.non_finite_vertex"`.
    pub code: &'static str,
    /// Human-readable detail (never shown as product UI by itself).
    pub detail: String,
}

impl DiagnosticEvent {
    /// Builds an event. `detail` accepts any displayable value.
    pub fn new(
        category: DiagnosticCategory,
        code: &'static str,
        detail: impl std::fmt::Display,
    ) -> Self {
        Self {
            category,
            code,
            detail: detail.to_string(),
        }
    }

    /// Emits the event through `tracing` at INFO level with a stable target.
    pub fn emit(&self) {
        tracing::info!(
            target: "petunia",
            category = self.category.as_str(),
            code = self.code,
            "{}",
            self.detail
        );
    }

    /// Emits the event through `tracing` at WARN level.
    pub fn emit_warn(&self) {
        tracing::warn!(
            target: "petunia",
            category = self.category.as_str(),
            code = self.code,
            "{}",
            self.detail
        );
    }
}

/// Convenience helper for one-shot INFO diagnostics without building a struct.
pub fn log_event(category: DiagnosticCategory, code: &'static str, detail: impl std::fmt::Display) {
    DiagnosticEvent::new(category, code, detail).emit();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_names_are_stable() {
        assert_eq!(DiagnosticCategory::Geometry.as_str(), "geometry");
        assert_eq!(DiagnosticCategory::ProjectIo.as_str(), "project_io");
        assert_eq!(DiagnosticCategory::Commands.as_str(), "commands");
        assert_eq!(DiagnosticCategory::Jobs.as_str(), "jobs");
        assert_eq!(DiagnosticCategory::Plugins.as_str(), "plugins");
    }

    #[test]
    fn emit_does_not_panic_without_subscriber() {
        // No global subscriber installed in this test: `tracing` must no-op.
        let event = DiagnosticEvent::new(
            DiagnosticCategory::Geometry,
            "test.emit",
            "diagnostics boundary smoke",
        );
        event.emit();
        event.emit_warn();
        log_event(DiagnosticCategory::Jobs, "test.helper", "helper smoke");
    }
}
