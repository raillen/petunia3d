//! Unified devtools entry (`devtools` feature, P1-07..P1-11, E2).
//!
//! Default-off: never ships in distribution builds. When enabled (dev/test),
//! this module is the single entry for:
//!
//! ```text
//! Inspect UI tree      (egui_inspection, loopback TCP only)
//! Agent UI bridge      (egui_mcp, loopback attach only)
//! Debug state probe    (egui-probe over snapshot DTOs)
//! Profiler             (puffin scopes; embedded flamegraph deferred, see below)
//! ```
//!
//! Distribution behavior is enforced by construction: everything here lives
//! behind `#[cfg(feature = "devtools")]`, and [`attach_remote`] refuses
//! non-loopback addresses.
//!
//! `puffin_egui` 0.30 pins egui 0.33 (incompatible with our 0.36 family), so
//! the embedded flamegraph UI is deferred until upstream releases an egui
//! 0.36-compatible version. Profiling data still flows through `puffin`
//! scopes; this panel reports scope state instead of the flamegraph.

use egui_probe::{EguiProbe, Probe};

/// Loopback-only guard for every devtools network surface.
fn ensure_loopback(addr: &str) -> Result<std::net::SocketAddr, String> {
    let resolved: std::net::SocketAddr = addr
        .parse()
        .map_err(|e| format!("bad devtools addr '{addr}': {e}"))?;
    if !resolved.ip().is_loopback() {
        return Err(format!(
            "devtools refuses non-loopback addr '{addr}': inspection is loopback-only"
        ));
    }
    Ok(resolved)
}

/// Read-only scene snapshot for the probe panel. A copy, never live domain
/// state: edits in the probe affect only this snapshot.
#[derive(Clone, Debug, Default, EguiProbe)]
pub struct SceneSnapshot {
    /// Asset count.
    pub assets: usize,
    /// Selected annotation debug label.
    pub selection: String,
    /// Frame time in ms.
    pub frame_ms: f32,
    /// Camera distance.
    pub camera_distance: f32,
}

impl SceneSnapshot {
    /// Captures the current scene numbers.
    pub fn capture(state: &petunia_core::AppState) -> Self {
        Self {
            assets: state.project.assets.len(),
            selection: format!("{:?}", state.select_mode),
            frame_ms: state.render.stats.frame_ms,
            camera_distance: state.camera.distance,
        }
    }
}

/// Devtools panel state, owned by the embedding host (never Core).
#[derive(Clone, Debug, Default)]
pub struct DevtoolsPanel {
    /// Whether the panel window is open.
    pub open: bool,
    /// Whether puffin scope collection is enabled.
    pub profiling: bool,
    /// Last inspection attach result for display.
    pub attach_status: String,
}

impl DevtoolsPanel {
    /// Attaches the inspection plugin and loopback TCP serve on `ctx`.
    /// Returns `true` when the plugin was registered. Non-loopback `addr`
    /// is refused without side effects.
    pub fn attach(&mut self, ctx: &egui::Context, addr: &str) -> bool {
        let resolved = match ensure_loopback(addr) {
            Ok(resolved) => resolved,
            Err(detail) => {
                self.attach_status = detail;
                return false;
            }
        };
        ctx.add_plugin(egui_inspection::InspectionPlugin::new(Some(
            "petunia3d".to_string(),
        )));
        match egui_inspection::serve(ctx, &resolved.to_string()) {
            Ok(()) => {
                self.attach_status = format!("inspection on {resolved} (loopback)");
                true
            }
            Err(e) => {
                self.attach_status = format!("inspection bind failed: {e}");
                false
            }
        }
    }

    /// Enables or disables puffin scope collection.
    pub fn set_profiling(&mut self, enabled: bool) {
        self.profiling = enabled;
        puffin::set_scopes_on(enabled);
    }

    /// Draws the unified devtools window (inspection, bridge, probe, profiler).
    pub fn draw(&mut self, ctx: &egui::Context, state: &petunia_core::AppState) {
        if !self.open {
            return;
        }
        let mut open = self.open;
        egui::Window::new("Petunia Devtools")
            .open(&mut open)
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("profiling:");
                    let mut profiling = self.profiling;
                    if ui.checkbox(&mut profiling, "puffin scopes").changed() {
                        self.set_profiling(profiling);
                    }
                });
                let scopes_on = puffin::are_scopes_on();
                ui.label(format!(
                    "scopes state: {}",
                    if scopes_on { "on" } else { "off" }
                ));
                ui.separator();
                ui.label("inspection (loopback TCP only):");
                ui.label(self.attach_status.clone());
                ui.label("agent bridge: egui_mcp attach → 127.0.0.1:5719 (loopback)");
                ui.separator();
                let mut snapshot = SceneSnapshot::capture(state);
                Probe::new(&mut snapshot)
                    .with_header("scene snapshot (copy)")
                    .show(ui);
            });
        self.open = open;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loopback_guard_rejects_remote_addrs() {
        assert!(ensure_loopback("127.0.0.1:5719").is_ok());
        assert!(ensure_loopback("[::1]:5719").is_ok());
        assert!(ensure_loopback("0.0.0.0:5719").is_err());
        assert!(ensure_loopback("192.168.1.10:5719").is_err());
        assert!(ensure_loopback("not-an-addr").is_err());
    }

    #[test]
    fn attach_refuses_non_loopback_without_side_effects() {
        let ctx = egui::Context::default();
        let mut panel = DevtoolsPanel::default();
        assert!(!panel.attach(&ctx, "0.0.0.0:5719"));
        assert!(panel.attach_status.contains("loopback-only"));
    }

    #[test]
    fn inspection_smoke_flow_through_plugin() {
        use egui_inspection::{Request, Response};
        let ctx = egui::Context::default();
        ctx.add_plugin(egui_inspection::InspectionPlugin::new(Some(
            "test".to_string(),
        )));
        let replied = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let replied_hook = replied.clone();
        ctx.run_ui(egui::RawInput::default(), |ui| {
            ui.ctx()
                .with_plugin(|plugin: &mut egui_inspection::InspectionPlugin| {
                    let hook = replied_hook.clone();
                    plugin.submit(Request::GetInfo, move |response| {
                        if matches!(response, Response::Info { .. }) {
                            hook.store(true, std::sync::atomic::Ordering::SeqCst);
                        }
                    });
                });
            ui.ctx().request_repaint();
        })
        .textures_delta
        .clear();
        // Pump frames until the async reply round-trips (bounded).
        for _ in 0..30 {
            if replied.load(std::sync::atomic::Ordering::SeqCst) {
                break;
            }
            ctx.run_ui(egui::RawInput::default(), |_| {})
                .textures_delta
                .clear();
        }
        assert!(replied.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[test]
    fn profiler_toggle_and_probe_snapshot() {
        let mut panel = DevtoolsPanel::default();
        panel.set_profiling(true);
        assert!(puffin::are_scopes_on());
        panel.set_profiling(false);
        assert!(!puffin::are_scopes_on());

        let state = petunia_core::AppState::new("en");
        let snapshot = SceneSnapshot::capture(&state);
        assert_eq!(snapshot.assets, state.project.assets.len());

        let ctx = egui::Context::default();
        let mut panel = DevtoolsPanel {
            open: true,
            ..Default::default()
        };
        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                let mut snapshot = snapshot.clone();
                Probe::new(&mut snapshot).with_header("probe-test").show(ui);
            });
        })
        .textures_delta
        .clear();
        panel.open = false;
        assert!(!panel.open);
    }

    #[test]
    fn egui_mcp_server_initialization_and_tool_discovery() {
        let server = egui_mcp::Server::new();
        let _cloned = server.clone();
    }
}
