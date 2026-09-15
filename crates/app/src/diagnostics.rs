//! Diagnostics sink (`tracing-subscriber`, P0-10).
//!
//! Configures filtering, format and output centrally for the whole desktop
//! application. Production defaults stay quiet and useful (`warn` for noisy
//! wgpu/hal targets, `info` for `petunia`); developers get verbosity through
//! `RUST_LOG` without recompiling business logic.

/// Initializes the global `tracing` subscriber.
///
/// Safe to call more than once (later calls are no-ops): tests and embedding
/// hosts may already have installed a subscriber.
pub fn init_diagnostics() {
    use tracing_subscriber::{EnvFilter, fmt};
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("info,petunia=info,wgpu_hal=off,wgpu_core=off,egui_glow=off")
    });
    let _ = fmt()
        .with_env_filter(filter)
        .with_target(true)
        .without_time()
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_is_idempotent_and_quiet_by_default() {
        init_diagnostics();
        // Second call must not panic (global subscriber already set).
        init_diagnostics();
        petunia_core::log_event(
            petunia_core::DiagnosticCategory::Commands,
            "test.diagnostics",
            "sink smoke",
        );
    }
}
