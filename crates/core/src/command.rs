//! Sistema de comandos do Petunia3D (Command Pattern & Dispatcher).
//! Encapsula operações de edição de malha, assets, seleção e histórico (Undo/Redo)
//! de forma desacoplada da interface gráfica e pronta para execução headless.

use petunia_mesh::Mesh;
use petunia_project::Asset;

use crate::state::{AppState, EditMode};

/// Taxonomia de erros de comandos da aplicação.
#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum CommandError {
    #[error("Nenhum asset ativo selecionado")]
    NoActiveAsset,
    #[error("Índice de asset inválido: {0}")]
    InvalidAssetIndex(usize),
    #[error("Comando não suportado no modo de edição atual: {0:?}")]
    InvalidMode(EditMode),
    #[error("Nenhum elemento selecionado")]
    EmptySelection,
    #[error("Erro ao executar comando: {0}")]
    Execution(String),
}

/// Trait central de comando executável contra o estado do editor (`AppState`).
pub trait Command: Send + Sync {
    /// Rótulo legível para telemetria e pilha de histórico (Undo/Redo).
    fn label(&self) -> &'static str;

    /// Executa o comando contra o estado do editor.
    fn execute(&self, state: &mut AppState) -> Result<(), CommandError>;

    /// Indica se a operação altera dados do projeto e exige gravação prévia de checkpoint no UndoStack.
    fn is_destructive(&self) -> bool {
        true
    }
}

use std::collections::HashMap;

/// Despachante central de comandos com auto-checkpointing de Undo/Redo,
/// sincronização de eventos e registro opcional por identificador de texto.
#[derive(Default)]
pub struct CommandDispatcher {
    registry: HashMap<String, Box<dyn Command>>,
}

impl CommandDispatcher {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }

    pub fn register(&mut self, id: impl Into<String>, cmd: Box<dyn Command>) {
        self.registry.insert(id.into(), cmd);
    }

    pub fn execute(&self, id: &str, state: &mut AppState) -> Result<(), CommandError> {
        let cmd = self
            .registry
            .get(id)
            .ok_or_else(|| CommandError::Execution(format!("Comando '{id}' não registrado")))?;
        Self::dispatch(state, cmd.as_ref())
    }

    pub fn dispatch(state: &mut AppState, cmd: &dyn Command) -> Result<(), CommandError> {
        if cmd.is_destructive() {
            state.checkpoint(cmd.label());
        }
        let res = cmd.execute(state);
        if res.is_ok() {
            if cmd.is_destructive() {
                state.mark_document_dirty();
            }
            state.sync_selection();
            state.emit_mesh_changed();
            state.mark_dirty();
        }
        res
    }
}

// -------------------------------------------------------------------------------------------------
// Comandos Canônicos
// -------------------------------------------------------------------------------------------------

/// Tipos de primitivas geométricas tridimensionais suportadas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveKind {
    Cube,
    Sphere,
    Cylinder,
    Plane,
    Cone,
    Capsule,
}

impl PrimitiveKind {
    pub fn default_name(&self) -> &'static str {
        match self {
            Self::Cube => "Cube",
            Self::Sphere => "Sphere",
            Self::Cylinder => "Cylinder",
            Self::Plane => "Plane",
            Self::Cone => "Cone",
            Self::Capsule => "Capsule",
        }
    }

    pub fn generate_mesh(&self) -> Mesh {
        match self {
            Self::Cube => Mesh::cube(1.0),
            Self::Sphere => Mesh::sphere_low(16, 12, 0.5),
            Self::Cylinder => Mesh::cylinder(16, 0.5, 1.0),
            Self::Plane => Mesh::plane(2.0),
            Self::Cone => Mesh::cone(16, 0.5, 1.0),
            Self::Capsule => Mesh::capsule(16, 0.3, 0.8),
        }
    }
}

/// Comando para inserção de primitiva geométrica na cena.
#[derive(Debug, Clone)]
pub struct AddPrimitiveCmd {
    pub kind: PrimitiveKind,
    pub name: Option<String>,
    pub at_cursor: bool,
}

impl AddPrimitiveCmd {
    pub fn new(kind: PrimitiveKind) -> Self {
        Self {
            kind,
            name: None,
            at_cursor: true,
        }
    }

    pub fn cube(at_cursor: bool) -> Self {
        Self {
            kind: PrimitiveKind::Cube,
            name: None,
            at_cursor,
        }
    }
}

impl Command for AddPrimitiveCmd {
    fn label(&self) -> &'static str {
        "add primitive"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let mut mesh = self.kind.generate_mesh();
        if self.at_cursor {
            let cursor = state.cursor_3d;
            for v in &mut mesh.verts {
                v.pos[0] += cursor[0];
                v.pos[1] += cursor[1];
                v.pos[2] += cursor[2];
            }
        }
        let name = self
            .name
            .clone()
            .unwrap_or_else(|| self.kind.default_name().to_string());
        state.project.add(&name, mesh);
        state.set_status(format!("Added {}", name));
        Ok(())
    }
}

/// Comando para duplicação do asset ativo ou especificado.
#[derive(Debug, Clone, Default)]
pub struct DuplicateAssetCmd {
    pub asset_index: Option<usize>,
}

impl Command for DuplicateAssetCmd {
    fn label(&self) -> &'static str {
        "duplicate asset"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let idx = self.asset_index.unwrap_or(state.project.active);
        let Some(asset) = state.project.assets.get(idx) else {
            return Err(CommandError::InvalidAssetIndex(idx));
        };

        let copy = asset.duplicate();
        let copy_name = copy.name.clone();
        state.project.assets.push(copy);
        state.project.active = state.project.assets.len() - 1;
        state.set_status(format!("Duplicated {}", copy_name));
        Ok(())
    }
}

/// Comando para exclusão segura de um asset da cena.
#[derive(Debug, Clone, Default)]
pub struct DeleteAssetCmd {
    pub asset_index: Option<usize>,
}

impl Command for DeleteAssetCmd {
    fn label(&self) -> &'static str {
        "delete asset"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let idx = self.asset_index.unwrap_or(state.project.active);
        if idx >= state.project.assets.len() {
            return Err(CommandError::InvalidAssetIndex(idx));
        }

        state.project.assets.remove(idx);
        if state.project.assets.is_empty() {
            state
                .project
                .assets
                .push(Asset::new("Cube", Mesh::cube(2.0)));
        }
        state.project.active = state.project.active.min(state.project.assets.len() - 1);
        state.set_status("Asset deleted");
        Ok(())
    }
}

/// Comando contextual para deletar a seleção (sub-elementos em Edit Mode ou asset em Object Mode).
#[derive(Debug, Clone, Default)]
pub struct DeleteSelectionCmd;

impl Command for DeleteSelectionCmd {
    fn label(&self) -> &'static str {
        "delete"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        if state.mode == EditMode::Object {
            let cmd = DeleteAssetCmd { asset_index: None };
            return cmd.execute(state);
        }

        let Some(mesh) = state.project.active_mesh_mut() else {
            return Err(CommandError::NoActiveAsset);
        };
        mesh.delete_selected();
        state.set_status("Deleted selection");
        Ok(())
    }
}

/// Comando para duplicar a geometria selecionada dentro da malha ativa em Edit Mode.
#[derive(Debug, Clone, Default)]
pub struct DuplicateSelectionCmd;

impl Command for DuplicateSelectionCmd {
    fn label(&self) -> &'static str {
        "duplicate"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        if state.mode == EditMode::Object {
            let cmd = DuplicateAssetCmd { asset_index: None };
            return cmd.execute(state);
        }

        let Some(mesh) = state.project.active_mesh_mut() else {
            return Err(CommandError::NoActiveAsset);
        };
        mesh.duplicate_selected();
        state.set_status("Duplicated selection");
        Ok(())
    }
}

/// Comando para selecionar todos os elementos da malha ativa.
#[derive(Debug, Clone, Default)]
pub struct SelectAllCmd;

impl Command for SelectAllCmd {
    fn label(&self) -> &'static str {
        "select all"
    }

    fn is_destructive(&self) -> bool {
        false
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let Some(mesh) = state.project.active_mesh_mut() else {
            return Err(CommandError::NoActiveAsset);
        };
        mesh.select_all();
        state.sync_selection();
        Ok(())
    }
}

/// Comando para limpar toda a seleção de elementos da malha ativa.
#[derive(Debug, Clone, Default)]
pub struct ClearSelectionCmd;

impl Command for ClearSelectionCmd {
    fn label(&self) -> &'static str {
        "clear selection"
    }

    fn is_destructive(&self) -> bool {
        false
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let Some(mesh) = state.project.active_mesh_mut() else {
            return Err(CommandError::NoActiveAsset);
        };
        mesh.deselect_all();
        state.sync_selection();
        Ok(())
    }
}

/// Comando para inverter a seleção de elementos da malha ativa.
#[derive(Debug, Clone, Default)]
pub struct InvertSelectionCmd;

impl Command for InvertSelectionCmd {
    fn label(&self) -> &'static str {
        "invert selection"
    }

    fn is_destructive(&self) -> bool {
        false
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let Some(mesh) = state.project.active_mesh_mut() else {
            return Err(CommandError::NoActiveAsset);
        };
        mesh.invert_selection();
        state.sync_selection();
        Ok(())
    }
}

/// Comando para subdividir a geometria selecionada na malha ativa.
#[derive(Debug, Clone, Default)]
pub struct SubdivideSelectionCmd;

impl Command for SubdivideSelectionCmd {
    fn label(&self) -> &'static str {
        "subdivide"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let Some(mesh) = state.project.active_mesh_mut() else {
            return Err(CommandError::NoActiveAsset);
        };
        mesh.subdivide_selected();
        state.set_status("Subdivided selection");
        Ok(())
    }
}

/// Comando para fundir elementos selecionados no centro na malha ativa.
#[derive(Debug, Clone, Default)]
pub struct MergeCenterCmd;

impl Command for MergeCenterCmd {
    fn label(&self) -> &'static str {
        "merge"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let Some(mesh) = state.project.active_mesh_mut() else {
            return Err(CommandError::NoActiveAsset);
        };
        mesh.merge_center();
        state.set_status("Merged selection at center");
        Ok(())
    }
}

/// Comando para inverter a orientação das normais da malha ativa.
#[derive(Debug, Clone, Default)]
pub struct FlipNormalsCmd;

impl Command for FlipNormalsCmd {
    fn label(&self) -> &'static str {
        "flip_normals"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let Some(mesh) = state.project.active_mesh_mut() else {
            return Err(CommandError::NoActiveAsset);
        };
        mesh.flip_normals();
        state.set_status("Flipped normals");
        Ok(())
    }
}

/// Comando para inverter a diagonal de triangulação interna de quads selecionados
/// ou executar edge-flip em aresta compartilhada por triângulos.
#[derive(Debug, Clone, Default)]
pub struct FlipDiagonalCmd;

impl Command for FlipDiagonalCmd {
    fn label(&self) -> &'static str {
        "flip diagonal"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let Some(mesh) = state.project.active_mesh_mut() else {
            return Err(CommandError::NoActiveAsset);
        };
        if mesh.flip_diagonal() {
            state.set_status("Flipped quad diagonal / triangle edge");
            Ok(())
        } else {
            Err(CommandError::Execution(
                "No selected quad or edge suitable for diagonal flip".into(),
            ))
        }
    }
}

/// Comando para rotacionar/revolver perfil selecionado ao redor de um eixo coordenado.
#[derive(Debug, Clone)]
pub struct RevolveCmd {
    pub segments: u32,
    pub angle_deg: f32,
    pub axis: usize,
    pub center: [f32; 3],
}

impl Default for RevolveCmd {
    fn default() -> Self {
        Self {
            segments: 16,
            angle_deg: 360.0,
            axis: 1, // Eixo Y
            center: [0.0, 0.0, 0.0],
        }
    }
}

impl Command for RevolveCmd {
    fn label(&self) -> &'static str {
        "revolve"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let Some(mesh) = state.project.active_mesh_mut() else {
            return Err(CommandError::NoActiveAsset);
        };
        if mesh.revolve_selection(self.segments, self.angle_deg, self.axis, self.center) {
            state.set_status(format!(
                "Revolved selection ({} segments, {:.0}°)",
                self.segments, self.angle_deg
            ));
            Ok(())
        } else {
            Err(CommandError::Execution(
                "Failed to revolve selection: require selected connected edges".into(),
            ))
        }
    }
}

/// Comando para extrudar faces selecionadas individualmente (desacopladas).
#[derive(Debug, Clone)]
pub struct ExtrudeIndividualCmd {
    pub dist: f32,
}

impl Default for ExtrudeIndividualCmd {
    fn default() -> Self {
        Self { dist: 0.0 }
    }
}

impl Command for ExtrudeIndividualCmd {
    fn label(&self) -> &'static str {
        "extrude individual"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let Some(mesh) = state.project.active_mesh_mut() else {
            return Err(CommandError::NoActiveAsset);
        };
        if !mesh.faces.iter().any(|f| f.selected) {
            return Err(CommandError::EmptySelection);
        }
        mesh.extrude_individual(self.dist);
        state.set_status(format!("Extruded individual faces ({:.2})", self.dist));
        Ok(())
    }
}

/// Comando para selecionar todos os elementos conectados à seleção atual (Select Linked).
#[derive(Debug, Clone, Default)]
pub struct SelectLinkedCmd;

impl Command for SelectLinkedCmd {
    fn label(&self) -> &'static str {
        "select linked"
    }

    fn is_destructive(&self) -> bool {
        false
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let Some(mesh) = state.project.active_mesh_mut() else {
            return Err(CommandError::NoActiveAsset);
        };
        mesh.select_linked();
        state.sync_selection();
        Ok(())
    }
}

/// Comando para seleção por área (retângulo 2D projetado via matriz de visão/projeção).
#[derive(Debug, Clone)]
pub struct BoxSelectCmd {
    pub p0: [f32; 2],
    pub p1: [f32; 2],
    pub view_proj: [f32; 16],
    pub add: bool,
}

impl Command for BoxSelectCmd {
    fn label(&self) -> &'static str {
        "box select"
    }

    fn is_destructive(&self) -> bool {
        false
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let Some(mesh) = state.project.active_mesh_mut() else {
            return Err(CommandError::NoActiveAsset);
        };
        mesh.box_select(self.p0, self.p1, &self.view_proj, self.add);
        state.sync_selection();
        Ok(())
    }
}

/// Comando para alternar o bloqueio (lock) do asset ativo ou especificado.
#[derive(Debug, Clone, Default)]
pub struct ToggleLockAssetCmd {
    pub asset_index: Option<usize>,
}

impl Command for ToggleLockAssetCmd {
    fn label(&self) -> &'static str {
        "toggle lock"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let idx = self.asset_index.unwrap_or(state.project.active);
        let Some(asset) = state.project.assets.get_mut(idx) else {
            return Err(CommandError::InvalidAssetIndex(idx));
        };
        asset.locked = !asset.locked;
        let name = asset.name.clone();
        let locked = asset.locked;
        state.set_status(if locked {
            format!("Locked {}", name)
        } else {
            format!("Unlocked {}", name)
        });
        Ok(())
    }
}

/// Comando para alternar a visibilidade do asset ativo ou especificado.
#[derive(Debug, Clone, Default)]
pub struct ToggleVisibilityAssetCmd {
    pub asset_index: Option<usize>,
}

impl Command for ToggleVisibilityAssetCmd {
    fn label(&self) -> &'static str {
        "toggle visibility"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let idx = self.asset_index.unwrap_or(state.project.active);
        let Some(asset) = state.project.assets.get_mut(idx) else {
            return Err(CommandError::InvalidAssetIndex(idx));
        };
        asset.visible = !asset.visible;
        let name = asset.name.clone();
        let visible = asset.visible;
        state.set_status(if visible {
            format!("Showed {}", name)
        } else {
            format!("Hid {}", name)
        });
        Ok(())
    }
}

/// Comando para definir a coleção organizadora de um asset.
#[derive(Debug, Clone)]
pub struct SetAssetCollectionCmd {
    pub asset_index: usize,
    pub collection: Option<String>,
}

impl Command for SetAssetCollectionCmd {
    fn label(&self) -> &'static str {
        "set collection"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let Some(asset) = state.project.assets.get_mut(self.asset_index) else {
            return Err(CommandError::InvalidAssetIndex(self.asset_index));
        };
        asset.collection = self.collection.clone();
        let name = asset.name.clone();
        if let Some(ref col) = self.collection {
            state.set_status(format!("Moved {} to collection {}", name, col));
        } else {
            state.set_status(format!("Removed {} from collections", name));
        }
        Ok(())
    }
}

/// Comando para alternar a visibilidade de todos os assets de uma coleção.
#[derive(Debug, Clone)]
pub struct ToggleCollectionVisibilityCmd {
    pub collection: String,
}

impl Command for ToggleCollectionVisibilityCmd {
    fn label(&self) -> &'static str {
        "toggle collection visibility"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let all_vis = state
            .project
            .assets
            .iter()
            .filter(|a| a.collection.as_deref() == Some(&self.collection))
            .all(|a| a.visible);
        for a in &mut state.project.assets {
            if a.collection.as_deref() == Some(&self.collection) {
                a.visible = !all_vis;
            }
        }
        state.set_status(format!(
            "Collection {}: {}",
            self.collection,
            if !all_vis { "visible" } else { "hidden" }
        ));
        Ok(())
    }
}

/// Comando para alternar o bloqueio (lock) de todos os assets de uma coleção.
#[derive(Debug, Clone)]
pub struct ToggleCollectionLockCmd {
    pub collection: String,
}

impl Command for ToggleCollectionLockCmd {
    fn label(&self) -> &'static str {
        "toggle collection lock"
    }

    fn execute(&self, state: &mut AppState) -> Result<(), CommandError> {
        let all_locked = state
            .project
            .assets
            .iter()
            .filter(|a| a.collection.as_deref() == Some(&self.collection))
            .all(|a| a.locked);
        for a in &mut state.project.assets {
            if a.collection.as_deref() == Some(&self.collection) {
                a.locked = !all_locked;
            }
        }
        state.set_status(format!(
            "Collection {}: {}",
            self.collection,
            if !all_locked { "locked" } else { "unlocked" }
        ));
        Ok(())
    }
}
