//! Contexto do inspector MODEL: o que inspecionar + quais abas mostrar.
//!
//! Fonte única da decisão "seleção → contexto → abas" (Shapr3D: seleção reduz
//! informação; Spline: inspector contextual; Blockbench: composição por
//! workspace — aqui só MODEL usa este módulo, os demais mantêm seus painéis).
//!
//! Modelo de seleção honesto: Petunia tem UM ativo por vez (sem
//! multisseleção de objetos — `export_selected` é só do diálogo de export).
//! Não existe contexto "multi-objetos"; há seleção de componentes (muitos
//! vértices/arestas/faces do ativo), com valores derivados únicos.

use petunia_core::{AppState, EditMode};
use uuid::Uuid;

/// Abas textuais do inspector (sem rail de ícones: texto > memorização).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectorTab {
    Object,
    Modify,
    Material,
    Selection,
}

impl InspectorTab {
    pub fn key(self) -> &'static str {
        match self {
            InspectorTab::Object => "inspector.tab_object",
            InspectorTab::Modify => "inspector.tab_modify",
            InspectorTab::Material => "inspector.tab_material",
            InspectorTab::Selection => "inspector.tab_selection",
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            InspectorTab::Object => "object",
            InspectorTab::Modify => "modify",
            InspectorTab::Material => "material",
            InspectorTab::Selection => "selection",
        }
    }

    /// Converte aba armazenada (inclui legados `tool`/`modifiers`/`data`).
    pub fn from_id(id: &str) -> Option<Self> {
        match id {
            "object" => Some(InspectorTab::Object),
            "modify" => Some(InspectorTab::Modify),
            "material" => Some(InspectorTab::Material),
            "selection" => Some(InspectorTab::Selection),
            _ => None,
        }
    }
}

/// Contexto inspecionado (resolvido da seleção real a cada frame).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InspectorContext {
    /// Nenhum asset na cena.
    EmptyScene,
    /// Um objeto mesh (fixado ou ativo).
    MeshObject {
        asset: Uuid,
        pinned: bool,
    },
    /// Seleção de componentes no modo de edição.
    ComponentSelection {
        verts: usize,
        edges: usize,
        faces: usize,
    },
    Annotation(Uuid),
    Measurement(Uuid),
}

impl InspectorContext {
    /// Resolve o contexto a partir do estado real (puro, testável).
    pub fn resolve(state: &AppState) -> Self {
        if let Some(id) = state.selected_annotation
            && state.project.annotations.iter().any(|a| a.id == id)
        {
            return InspectorContext::Annotation(id);
        }
        if let Some(id) = state.selected_measurement
            && state.project.measurements.iter().any(|m| m.id == id)
        {
            return InspectorContext::Measurement(id);
        }
        if state.project.assets.is_empty() {
            return InspectorContext::EmptyScene;
        }
        // Componentes selecionados no modo de edição têm contexto próprio.
        if state.mode == EditMode::Edit && !state.selection.is_empty() {
            return InspectorContext::ComponentSelection {
                verts: state.selection.verts.len(),
                edges: state.selection.edges.len(),
                faces: state.selection.faces.len(),
            };
        }
        // Fixado (se existir) ou ativo; índice sempre pinçado.
        let pinned_idx = state
            .ui
            .inspector_pinned
            .and_then(|id| state.project.assets.iter().position(|a| a.id == id));
        let idx = pinned_idx.unwrap_or_else(|| {
            state
                .project
                .active
                .min(state.project.assets.len().saturating_sub(1))
        });
        match state.project.assets.get(idx) {
            Some(asset) => InspectorContext::MeshObject {
                asset: asset.id,
                pinned: pinned_idx.is_some(),
            },
            None => InspectorContext::EmptyScene,
        }
    }

    /// Abas do contexto (vazio = sem abas).
    pub fn tabs(&self) -> &'static [InspectorTab] {
        match self {
            InspectorContext::EmptyScene => &[],
            InspectorContext::MeshObject { .. } => &[
                InspectorTab::Object,
                InspectorTab::Modify,
                InspectorTab::Material,
            ],
            InspectorContext::ComponentSelection { .. } => &[
                InspectorTab::Selection,
                InspectorTab::Modify,
                InspectorTab::Material,
            ],
            InspectorContext::Annotation(_) | InspectorContext::Measurement(_) => &[],
        }
    }

    /// Aba padrão do contexto.
    pub fn default_tab(&self) -> Option<InspectorTab> {
        match self {
            InspectorContext::EmptyScene => None,
            InspectorContext::MeshObject { .. } => Some(InspectorTab::Object),
            InspectorContext::ComponentSelection { .. } => Some(InspectorTab::Selection),
            InspectorContext::Annotation(_) | InspectorContext::Measurement(_) => None,
        }
    }

    /// Higieniza a aba armazenada para o contexto (legados/desconhecidos caem
    /// no padrão do contexto; contextos sem abas retornam `None`).
    pub fn sanitize_tab(&self, stored: &str) -> Option<InspectorTab> {
        match InspectorTab::from_id(stored) {
            Some(tab) if self.tabs().contains(&tab) => Some(tab),
            _ => self.default_tab(),
        }
    }
}

/// Índice do asset fixado, se ainda existir (pin nunca retém inválido).
pub fn pinned_asset_idx(state: &AppState) -> Option<usize> {
    state
        .ui
        .inspector_pinned
        .and_then(|id| state.project.assets.iter().position(|a| a.id == id))
}

/// Índice inspecionado: fixado (válido) ou ativo pinçado.
pub fn inspected_asset_idx(state: &AppState) -> Option<usize> {
    if state.project.assets.is_empty() {
        return None;
    }
    Some(pinned_asset_idx(state).unwrap_or_else(|| {
        state
            .project
            .active
            .min(state.project.assets.len().saturating_sub(1))
    }))
}

/// Alterna o pin entre "fixar ativo" e "soltar".
pub fn toggle_pin(state: &mut AppState) {
    if state.ui.inspector_pinned.is_some() {
        state.ui.inspector_pinned = None;
    } else if let Some(idx) = inspected_asset_idx(state)
        && let Some(asset) = state.project.assets.get(idx)
    {
        state.ui.inspector_pinned = Some(asset.id);
    }
    state.mark_dirty();
}

#[cfg(test)]
mod tests {
    use super::*;
    use petunia_core::Workspace;

    fn state_with_cube() -> AppState {
        let mut state = AppState::new("en");
        assert!(!state.project.assets.is_empty());
        state.workspace = Workspace::Model;
        state
    }

    #[test]
    fn empty_scene_resolves_without_tabs() {
        let mut state = AppState::new("en");
        state.project.assets.clear();
        let ctx = InspectorContext::resolve(&state);
        assert_eq!(ctx, InspectorContext::EmptyScene);
        assert!(ctx.tabs().is_empty());
        assert_eq!(ctx.default_tab(), None);
        assert_eq!(ctx.sanitize_tab("object"), None);
    }

    #[test]
    fn mesh_object_tabs_and_defaults() {
        let state = state_with_cube();
        let ctx = InspectorContext::resolve(&state);
        assert!(matches!(
            ctx,
            InspectorContext::MeshObject { pinned: false, .. }
        ));
        assert_eq!(
            ctx.tabs(),
            &[
                InspectorTab::Object,
                InspectorTab::Modify,
                InspectorTab::Material
            ]
        );
        assert_eq!(ctx.default_tab(), Some(InspectorTab::Object));
        assert_eq!(ctx.sanitize_tab("material"), Some(InspectorTab::Material));
    }

    #[test]
    fn legacy_tabs_fall_back_to_default() {
        let state = state_with_cube();
        let ctx = InspectorContext::resolve(&state);
        for legacy in ["tool", "modifiers", "data"] {
            assert_eq!(
                ctx.sanitize_tab(legacy),
                Some(InspectorTab::Object),
                "{legacy} deveria cair no padrão"
            );
        }
        assert_eq!(ctx.sanitize_tab("bogus"), Some(InspectorTab::Object));
    }

    #[test]
    fn pin_and_unpin_follow_and_release() {
        let mut state = state_with_cube();
        let id = state.project.assets[state.project.active].id;
        toggle_pin(&mut state);
        assert_eq!(state.ui.inspector_pinned, Some(id));
        assert_eq!(pinned_asset_idx(&state), Some(state.project.active));
        let ctx = InspectorContext::resolve(&state);
        assert!(matches!(
            ctx,
            InspectorContext::MeshObject { pinned: true, .. }
        ));
        toggle_pin(&mut state);
        assert_eq!(state.ui.inspector_pinned, None);
    }

    #[test]
    fn deleted_pin_falls_back_gracefully() {
        let mut state = state_with_cube();
        toggle_pin(&mut state);
        let pinned = state.ui.inspector_pinned.expect("pin ativo");
        state.project.assets.retain(|a| a.id != pinned);
        // Pin obsoleto é ignorado (nunca retém inválido); há fallback.
        assert_eq!(pinned_asset_idx(&state), None);
        if state.project.assets.is_empty() {
            assert_eq!(
                InspectorContext::resolve(&state),
                InspectorContext::EmptyScene
            );
        } else {
            assert!(matches!(
                InspectorContext::resolve(&state),
                InspectorContext::MeshObject { pinned: false, .. }
            ));
        }
    }

    #[test]
    fn component_selection_context_in_edit_mode() {
        let mut state = state_with_cube();
        state.mode = EditMode::Edit;
        state.selection.verts = vec![0, 1, 2];
        let ctx = InspectorContext::resolve(&state);
        assert_eq!(
            ctx,
            InspectorContext::ComponentSelection {
                verts: 3,
                edges: 0,
                faces: 0
            }
        );
        assert_eq!(
            ctx.tabs(),
            &[
                InspectorTab::Selection,
                InspectorTab::Modify,
                InspectorTab::Material
            ]
        );
        // Fora do modo de edição, mesmos vértices não viram contexto.
        state.mode = petunia_core::EditMode::Object;
        assert!(matches!(
            InspectorContext::resolve(&state),
            InspectorContext::MeshObject { .. }
        ));
    }
}
