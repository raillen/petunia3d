//! EventBus simples: módulos reagem a `AppEvent` sem se conhecerem.
//!
//! ```text
//! module-model → MeshChanged → EventBus → module-uv, module-assets
//! ```

use uuid::Uuid;

use super::selection::Selection;

#[derive(Debug, Clone)]
pub enum AppEvent {
    MeshChanged { asset_id: Uuid },
    SelectionChanged(Selection),
    ToolActivated(String),
    ActiveAssetChanged { asset_id: Uuid },
    TextureChanged { asset_id: Uuid },
    ProjectLoaded,
    ProjectSaving,
}

/// Fila drenada uma vez por frame pelo App.
#[derive(Debug, Default)]
pub struct EventBus {
    queue: Vec<AppEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn emit(&mut self, ev: AppEvent) {
        self.queue.push(ev);
    }
    pub fn drain(&mut self) -> Vec<AppEvent> {
        std::mem::take(&mut self.queue)
    }
    pub fn pending(&self) -> usize {
        self.queue.len()
    }
}
