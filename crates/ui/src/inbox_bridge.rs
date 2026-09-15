//! Worker-to-UI bridge (`egui_inbox`, P1-16, BASELINE CANDIDATE).
//!
//! Controlled worker results (thumbnails, export preprocessing, validation
//! reports) cross into egui through [`UiBridge`]: the worker posts through a
//! cloneable sender, the UI drains on its frame and requests repaint. The UI
//! never owns domain concurrency; the domain never owns egui handles.

/// Bounded bridge from one producer to the UI frame loop.
#[derive(Debug)]
pub struct UiBridge<T> {
    inbox: egui_inbox::UiInbox<T>,
}

impl<T> Default for UiBridge<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> UiBridge<T> {
    /// Creates a bridge not yet bound to a repaint context.
    pub fn new() -> Self {
        Self {
            inbox: egui_inbox::UiInbox::new(),
        }
    }

    /// Cloneable sender for hand-off to a worker thread or callback.
    pub fn sender(&self) -> egui_inbox::UiInboxSender<T> {
        self.inbox.sender()
    }

    /// Drains buffered messages on the UI thread, requesting repaint while
    /// any arrive. Returns everything posted since the last drain.
    pub fn drain(&self, ui: &egui::Ui) -> Vec<T> {
        self.inbox.read(ui).collect()
    }

    /// Drains without a repaint context (tests, headless pumps).
    pub fn drain_headless(&self) -> Vec<T> {
        self.inbox.read_without_ctx().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worker_post_reaches_ui_drain() {
        let bridge = UiBridge::new();
        let sender = bridge.sender();
        sender.send("thumb-ready".to_string()).unwrap();
        sender.send("export-done".to_string()).unwrap();
        assert_eq!(
            bridge.drain_headless(),
            vec!["thumb-ready".to_string(), "export-done".to_string()]
        );
        assert!(bridge.drain_headless().is_empty());
    }

    #[test]
    fn drain_inside_frame_requests_repaint() {
        let bridge: UiBridge<String> = UiBridge::new();
        bridge.sender().send("ping".to_string()).unwrap();
        let ctx = egui::Context::default();
        ctx.run_ui(egui::RawInput::default(), |ui| {
            let messages = bridge.drain(ui);
            assert_eq!(messages, vec!["ping".to_string()]);
        })
        .textures_delta
        .clear();
    }
}
