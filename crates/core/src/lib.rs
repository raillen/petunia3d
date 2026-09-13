//! Petunia3D core neutro: tipos compartilhados, eventos e contrato de módulo.
//! Features conhecem estas abstrações — nunca umas às outras (§22).

pub mod camera;
pub mod command;
pub mod events;
pub mod loop_cut;
pub mod mesh_preview;
pub mod modal;
pub mod module;
pub mod picking;
pub mod selection;
pub mod state;
pub mod viewport;

pub use command::{
    AddPrimitiveCmd, ClearSelectionCmd, Command, CommandDispatcher, CommandError, DeleteAssetCmd,
    DeleteSelectionCmd, DuplicateAssetCmd, DuplicateSelectionCmd, InvertSelectionCmd,
    PrimitiveKind, SelectAllCmd,
};

pub use camera::{Camera, Projection, ViewPreset};
pub use events::{AppEvent, EventBus};
pub use module::{Module, ModuleRegistry};
pub use selection::{SelectMode, Selection, Workspace};
pub use state::{
    AnnotationItem, AnnotationStroke, AppState, EditMode, Measurement, MeasurementItem,
    ProfileState, RefAxis, ReferenceImage, RenderStats,
};
pub use viewport::{LogicalRect, PhysicalViewport};

pub use modal::{ModalConstraint, ModalError, ModalKind, ModalOp};

#[cfg(test)]
mod preview_tests;
