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

/// Domínio unificado de interação e seleção (P3D-015 / P3D-016).
/// Elimina a divisão artificial entre Object Mode e Edit Mode na UX.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SelectionDomain {
    #[default]
    Object,
    Vertex,
    Edge,
    Face,
}

impl SelectionDomain {
    pub fn is_component(&self) -> bool {
        matches!(self, Self::Vertex | Self::Edge | Self::Face)
    }

    pub fn as_select_mode(&self) -> Option<SelectMode> {
        match self {
            Self::Vertex => Some(SelectMode::Vertex),
            Self::Edge => Some(SelectMode::Edge),
            Self::Face => Some(SelectMode::Face),
            Self::Object => None,
        }
    }

    pub fn from_select_mode(mode: SelectMode) -> Self {
        match mode {
            SelectMode::Vertex => Self::Vertex,
            SelectMode::Edge => Self::Edge,
            SelectMode::Face => Self::Face,
        }
    }

    pub fn key(&self) -> &'static str {
        match self {
            Self::Object => "modes.object",
            Self::Vertex => "modes.vertex",
            Self::Edge => "modes.edge",
            Self::Face => "modes.face",
        }
    }

    pub fn shortcut(&self) -> &'static str {
        match self {
            Self::Object => "0",
            Self::Vertex => "1",
            Self::Edge => "2",
            Self::Face => "3",
        }
    }
}

/// Workspaces V1: MODEL / PAINT / UV / ANIMATE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Workspace {
    #[default]
    Model,
    Paint,
    Uv,
    Animate,
}

impl Workspace {
    pub fn all() -> [Workspace; 4] {
        [
            Workspace::Model,
            Workspace::Paint,
            Workspace::Uv,
            Workspace::Animate,
        ]
    }
    pub fn key(&self) -> &'static str {
        match self {
            Workspace::Model => "ws.model",
            Workspace::Paint => "ws.paint",
            Workspace::Uv => "ws.uv",
            Workspace::Animate => "ws.animate",
        }
    }
}

/// Seleção atual (sincronizada entre viewport e UV via eventos).
#[derive(Debug, Clone, Default)]
pub struct Selection {
    pub asset: Option<Uuid>,
    pub verts: Vec<u32>,
    pub faces: Vec<usize>,
    pub edges: Vec<(u32, u32)>,
}

impl Selection {
    pub fn clear(&mut self) {
        self.verts.clear();
        self.faces.clear();
        self.edges.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.verts.is_empty() && self.faces.is_empty() && self.edges.is_empty()
    }
}
