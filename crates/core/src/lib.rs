//! Petunia3D core neutro: tipos compartilhados, eventos e contrato de módulo.
//! Features conhecem estas abstrações — nunca umas às outras (§22).

pub mod camera;
pub mod command;
pub mod cutting_session;
pub mod docs;
pub mod events;
pub mod loop_cut;
pub mod mesh_preview;
pub mod modal;
pub mod module;
pub mod picking;
pub mod project_service;
pub mod queries;
pub mod recent_projects;
pub mod selection;
pub mod state;
pub mod viewport;

pub use cutting_session::CutSession;
pub use docs::DocsTopic;
pub use petunia_project::{
    AssetSummary, AutosaveConfig, AutosaveService, ModelLibraryQuery, ModelLibraryService,
    ModelLibrarySort, RecoveryInfo, SessionLockInfo,
};
pub use queries::{SceneHierarchyDto, SceneObjectDto, SelectionDetailsDto, ToolStatusDto};
pub use recent_projects::{RecentProjectEntry, RecentProjects};

pub use command::{
    AddPrimitiveCmd, BevelCmd, BoxSelectCmd, ClearSelectionCmd, Command, CommandCategory,
    CommandDispatcher, CommandError, CommandMetadata, CommandPaletteItem, DeleteAssetCmd,
    DeleteSelectionCmd, DuplicateAssetCmd, DuplicateSelectionCmd, ExportGlbCmd, ExportObjCmd,
    ExtrudeIndividualCmd, ExtrudeSelectedCmd, FlipDiagonalCmd, FlipNormalsCmd, FrameSelectionCmd,
    ImportObjCmd, InsetFacesCmd, InvertSelectionCmd, MergeCenterCmd, NewProjectCmd, PrimitiveKind,
    RedoCmd, ResetCameraCmd, RevolveCmd, SaveActiveAsAssetCmd, SaveProjectAsCmd, SaveProjectCmd,
    SelectAllCmd, SelectLinkedCmd, SetAssetCollectionCmd, SubdivideSelectionCmd,
    ToggleCollectionLockCmd, ToggleCollectionVisibilityCmd, ToggleCommandPaletteCmd, ToggleHelpCmd,
    ToggleLockAssetCmd, ToggleProjectionCmd, ToggleSettingsCmd, ToggleVisibilityAssetCmd,
    ToggleWireframeCmd, ToggleXRayCmd, UndoCmd,
};
pub use project_service::{sanitize_filename, ProjectService, ProjectServiceError};

pub use camera::{Camera, Projection, ViewPreset};
pub use events::{AppEvent, EventBus};
pub use module::{Module, ModuleRegistry};
pub use selection::{SelectMode, Selection, Workspace};
pub use state::{
    AnnotationItem, AnnotationStroke, AppState, DomainState, EditMode, EditorSession, Measurement,
    MeasurementItem, ProfileState, ProjectState, RefAxis, ReferenceImage, RenderResources,
    RenderStats, ToolState, UiState,
};
pub use viewport::{
    unproject_cursor_or_vertex_snap, unproject_to_surface_or_cursor_plane, LogicalRect,
    PhysicalViewport,
};

pub use modal::{ModalConstraint, ModalError, ModalKind, ModalOp};

#[cfg(test)]
mod preview_tests;
