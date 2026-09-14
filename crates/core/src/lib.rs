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
pub mod modal_feedback;
pub mod module;
pub mod picking;
pub mod project_service;
pub mod proportional;
pub mod queries;
pub mod recent_projects;
pub mod selection;
pub mod snap;
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
    CommandDispatcher, CommandError, CommandMetadata, CommandPaletteItem, CycleSelectionDomainCmd,
    DeleteAssetCmd, DeleteSelectionCmd, DuplicateAssetCmd, DuplicateSelectionCmd, ExportGlbCmd,
    ExportObjCmd, ExtrudeIndividualCmd, ExtrudeSelectedCmd, FlipDiagonalCmd, FlipNormalsCmd,
    FrameSelectionCmd, ImportObjCmd, InsetFacesCmd, InstantiateAssetCmd, InvertSelectionCmd,
    MergeCenterCmd, NewProjectCmd, PrimitiveKind, RedoCmd, ResetCameraCmd, RevolveCmd,
    SaveActiveAsAssetCmd, SaveProjectAsCmd, SaveProjectCmd, SelectAllCmd, SelectLinkedCmd,
    SeparateSelectionCmd, SetAssetCollectionCmd, SetSelectionDomainCmd, SubdivideSelectionCmd,
    ToggleCollectionLockCmd, ToggleCollectionVisibilityCmd, ToggleCommandPaletteCmd, ToggleHelpCmd,
    ToggleLockAssetCmd, ToggleProjectionCmd, ToggleSettingsCmd, ToggleVisibilityAssetCmd,
    ToggleWireframeCmd, ToggleXRayCmd, UndoCmd,
};
pub use project_service::{sanitize_filename, ProjectService, ProjectServiceError};

pub use camera::{Camera, Projection, ViewPreset};
pub use events::{AppEvent, EventBus};
pub use modal_feedback::ToolFeedback;
pub use module::{Module, ModuleRegistry};
pub use proportional::{calculate_falloff_weight, ProportionalFalloff, ProportionalSettings};
pub use selection::{SelectMode, Selection, SelectionDomain, Workspace};
pub use snap::{
    snap_point, snap_point_to_edges, snap_point_to_faces, snap_point_to_grid,
    snap_point_to_increment, snap_point_to_vertices, SnapElement, SnapQuery, SnapResult,
    SnapSettings, SnapTarget,
};
pub use state::{
    AnnotationItem, AnnotationStroke, AppState, DomainState, EditMode, EditorSession, Measurement,
    MeasurementItem, PivotPoint, ProfileState, ProjectState, RefAxis, ReferenceImage,
    RenderResources, RenderStats, ToolState, TransformOrientation, UiState,
};
pub use viewport::{
    unproject_cursor_or_vertex_snap, unproject_to_surface_or_cursor_plane, LogicalRect,
    PhysicalViewport,
};

pub use modal::{ModalConstraint, ModalError, ModalKind, ModalOp};

#[cfg(test)]
mod preview_tests;
