//! Seleção e workspaces (pílulas da UI).

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SelectMode {
    #[default]
    Vertex,
    Edge,
    Face,
}

/// Workspaces V1: MODEL / PAINT / UV / EXPORT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Workspace {
    #[default]
    Model,
    Paint,
    Uv,
    Export,
}

impl Workspace {
    pub fn all() -> [Workspace; 4] {
        [
            Workspace::Model,
            Workspace::Paint,
            Workspace::Uv,
            Workspace::Export,
        ]
    }
    pub fn key(&self) -> &'static str {
        match self {
            Workspace::Model => "ws.model",
            Workspace::Paint => "ws.paint",
            Workspace::Uv => "ws.uv",
            Workspace::Export => "ws.export",
        }
    }
}

/// Seleção atual (sincronizada entre viewport e UV via eventos).
#[derive(Debug, Clone, Default)]
pub struct Selection {
    pub asset: Option<Uuid>,
    pub verts: Vec<u32>,
    pub faces: Vec<usize>,
}

impl Selection {
    pub fn clear(&mut self) {
        self.verts.clear();
        self.faces.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.verts.is_empty() && self.faces.is_empty()
    }
}
