//! Narrow `eframe` host boundary (P0-16).
//!
//! Production desktop hosting stays manual (`WgpuApp` + `GlApp`) because a
//! single `eframe::App` cannot cleanly represent the dual-backend invariant
//! (WGPU primary + OpenGL fallback behind renderer contracts). This module is
//! the sanctioned `eframe` integration point: [`PetuniaEframeApp`] owns the
//! desktop lifecycle through `eframe`, keeps Core unaware of the host, and
//! reuses the exact same [`petunia_ui::draw`] frontend. Use it for the
//! `eframe` smoke/dev path; the manual host remains the distribution path.

use crate::Core;

/// `eframe` host for Petunia3D. Construct with [`PetuniaEframeApp::new`],
/// run with `eframe::run_native` from a thin binary shim.
pub struct PetuniaEframeApp {
    core: Core,
    pending_quit: bool,
}

impl PetuniaEframeApp {
    /// Creates the host with a fresh [`Core`] (dev presets applied).
    pub fn new() -> Self {
        let mut core = Core::new();
        core.apply_dev_presets();
        Self {
            core,
            pending_quit: false,
        }
    }

    /// Whether the UI requested quit on the last frame.
    pub fn pending_quit(&self) -> bool {
        self.pending_quit
    }

    /// Read-only access to the owned [`Core`] for tests and shims.
    pub fn core(&self) -> &Core {
        &self.core
    }
}

impl Default for PetuniaEframeApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for PetuniaEframeApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut action = petunia_ui::UiAction::none();
        petunia_ui::draw(
            ui,
            &mut self.core.state,
            &self.core.tools,
            &mut self.core.registry,
            &mut action,
        );
        self.core.dispatch_events();
        self.core.tick_autosave();
        self.core.tick_watcher();
        self.pending_quit = action.quit;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_constructs_with_fresh_core() {
        let app = PetuniaEframeApp::new();
        assert!(!app.pending_quit());
        // Fresh core carries the default starter scene (never an empty project).
        assert!(!app.core().state.project.assets.is_empty());
    }
}
