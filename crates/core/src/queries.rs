//! Application API: Queries semânticas e DTOs com identificadores estáveis (`Uuid`).
//! (§Gauntlet G9 / Remediação de F-010).
//!
//! Isola as consultas de leitura da interface gráfica e clientes externos
//! (como CLI, C++, C#) através de contratos imutáveis, eliminando o acoplamento
//! a índices brutos `usize` sujeitos a descompasso após mutações.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::selection::SelectMode;
use crate::state::{AppState, EditMode};

/// Resumo de um objeto/asset na cena com identificador estável e dados de leitura imutáveis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneObjectDto {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
    pub visible: bool,
    pub locked: bool,
    pub collection: Option<String>,
    pub vert_count: usize,
    pub face_count: usize,
    pub base_color: [f32; 3],
}

/// DTO contendo a hierarquia completa da cena para apresentação em Outliners ou clientes remotos.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneHierarchyDto {
    pub assets: Vec<SceneObjectDto>,
    pub collections: Vec<String>,
    pub total_verts: usize,
    pub total_faces: usize,
    pub active_id: Option<Uuid>,
}

/// DTO detalhando o estado atual de seleção sem expor ponteiros ou índices efêmeros.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectionDetailsDto {
    pub active_asset_id: Option<Uuid>,
    pub active_asset_name: Option<String>,
    pub selected_verts_count: usize,
    pub selected_faces_count: usize,
    pub selected_edges_count: usize,
    pub select_mode: SelectMode,
    pub edit_mode: EditMode,
    pub selection_center: Option<[f32; 3]>,
}

/// DTO contendo o estado da ferramenta ativa e telemetria de interação.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolStatusDto {
    pub active_tool: String,
    pub is_modal_active: bool,
    pub modal_kind: Option<String>,
    pub has_cut_session: bool,
    pub has_pointer_session: bool,
    pub status: String,
}

impl AppState {
    /// Consulta imutável da hierarquia completa de objetos e coleções da cena.
    pub fn query_scene_hierarchy(&self) -> SceneHierarchyDto {
        let active_id = self.project.active().map(|a| a.id);
        let mut total_verts = 0;
        let mut total_faces = 0;

        let assets = self
            .project
            .assets
            .iter()
            .map(|a| {
                let v = a.mesh.vert_count();
                let f = a.mesh.faces.len();
                total_verts += v;
                total_faces += f;
                SceneObjectDto {
                    id: a.id,
                    name: a.name.clone(),
                    is_active: Some(a.id) == active_id,
                    visible: a.visible,
                    locked: a.locked,
                    collection: a.collection.clone(),
                    vert_count: v,
                    face_count: f,
                    base_color: a.base_color,
                }
            })
            .collect();

        SceneHierarchyDto {
            assets,
            collections: self.project.collections.clone(),
            total_verts,
            total_faces,
            active_id,
        }
    }

    /// Consulta imutável do resumo de seleção do objeto ativo.
    pub fn query_selection_details(&self) -> SelectionDetailsDto {
        let active_asset = self.project.active();
        let (v_count, f_count, e_count, center) = if let Some(a) = active_asset {
            let v = a.mesh.verts.iter().filter(|v| v.selected).count();
            let f = a.mesh.faces.iter().filter(|f| f.selected).count();
            let e = a.mesh.selected_edges.len();
            let c = if v > 0 || f > 0 || e > 0 {
                Some(a.mesh.selection_center())
            } else {
                None
            };
            (v, f, e, c)
        } else {
            (0, 0, 0, None)
        };

        SelectionDetailsDto {
            active_asset_id: active_asset.map(|a| a.id),
            active_asset_name: active_asset.map(|a| a.name.clone()),
            selected_verts_count: v_count,
            selected_faces_count: f_count,
            selected_edges_count: e_count,
            select_mode: self.select_mode,
            edit_mode: self.mode,
            selection_center: center,
        }
    }

    /// Consulta imutável do status de ferramenta e sessões de modal.
    pub fn query_tool_status(&self) -> ToolStatusDto {
        ToolStatusDto {
            active_tool: self.active_tool.clone(),
            is_modal_active: self.modal.is_some(),
            modal_kind: self.modal.as_ref().map(|m| format!("{:?}", m.kind)),
            has_cut_session: self.cut_session.is_some(),
            has_pointer_session: self.pointer_session.is_some(),
            status: self.ui.status.clone(),
        }
    }

    /// Ativa o asset correspondente ao identificador estável `Uuid`.
    /// Retorna `true` se encontrado e selecionado, ou `false` se não existir.
    pub fn set_active_asset_by_id(&mut self, id: Uuid) -> bool {
        if let Some(idx) = self.project.find(id) {
            if self.project.active != idx {
                self.project.active = idx;
                self.sync_selection();
                self.mark_dirty();
            }
            true
        } else {
            false
        }
    }

    /// Remove asset pelo identificador permanente `Uuid` sem perigo de deslocamento de índice.
    pub fn delete_asset_by_id(&mut self, id: Uuid) -> bool {
        if let Some(idx) = self.project.find(id) {
            self.checkpoint("delete asset");
            self.project.remove(idx);
            self.sync_selection();
            self.emit_mesh_changed();
            self.mark_dirty();
            true
        } else {
            false
        }
    }

    /// Localiza referência imutável de asset por UUID.
    pub fn find_asset_by_id(&self, id: Uuid) -> Option<&petunia_project::Asset> {
        self.project.find(id).and_then(|idx| self.project.assets.get(idx))
    }

    /// Localiza referência mutável de asset por UUID.
    pub fn find_asset_by_id_mut(&mut self, id: Uuid) -> Option<&mut petunia_project::Asset> {
        if let Some(idx) = self.project.find(id) {
            self.project.assets.get_mut(idx)
        } else {
            None
        }
    }
}
