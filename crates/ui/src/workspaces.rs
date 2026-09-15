//! Perfis de composição por workspace (Wave 3 — §6).
//!
//! Fonte única da verdade sobre o que muda quando o usuário troca MODEL /
//! PAINT / UV / ANIMATE. Workspaces compartilham projeto, seleção e undo;
//! só a composição do shell muda: paleta de ferramentas, centro, painel
//! inferior, aba padrão do inspector e overlays da viewport.

use petunia_core::Workspace;

/// Paleta da toolbar esquerda por workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolPaletteKind {
    /// Modelagem: seleção, transform, inspeção + malha (Edit).
    Modeling,
    /// Pintura: pincel, apagador, conta-gotas.
    Paint,
    /// UV: seleção + projeção.
    Uv,
    /// Animação: seleção + transform de pose.
    Animation,
}

/// Composição da área central por workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CenterKind {
    /// Viewport 3D único em toda a área central.
    Viewport3D,
    /// Editor UV 2D + prévia 3D lado a lado (ou alternados em janela estreita).
    UvSplit,
}

/// Painel inferior fixo por workspace (além da overlay flutuante).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BottomPaneKind {
    None,
    Timeline,
}

/// Perfil de layout de um workspace.
pub struct WorkspaceLayoutProfile {
    pub id: Workspace,
    pub left_tools: ToolPaletteKind,
    pub center: CenterKind,
    pub bottom: BottomPaneKind,
    /// Aba do inspector quando o workspace usa abas persistentes (Model).
    pub default_inspector_tab: &'static str,
    /// Shelf contextual flutuante sobre a viewport.
    pub overlay_shelf: bool,
}

const PROFILES: [WorkspaceLayoutProfile; 4] = [
    WorkspaceLayoutProfile {
        id: Workspace::Model,
        left_tools: ToolPaletteKind::Modeling,
        center: CenterKind::Viewport3D,
        bottom: BottomPaneKind::None,
        default_inspector_tab: "object",
        overlay_shelf: true,
    },
    WorkspaceLayoutProfile {
        id: Workspace::Paint,
        left_tools: ToolPaletteKind::Paint,
        center: CenterKind::Viewport3D,
        bottom: BottomPaneKind::None,
        default_inspector_tab: "object",
        overlay_shelf: true,
    },
    WorkspaceLayoutProfile {
        id: Workspace::Uv,
        left_tools: ToolPaletteKind::Uv,
        center: CenterKind::UvSplit,
        bottom: BottomPaneKind::None,
        default_inspector_tab: "object",
        overlay_shelf: false,
    },
    WorkspaceLayoutProfile {
        id: Workspace::Animate,
        left_tools: ToolPaletteKind::Animation,
        center: CenterKind::Viewport3D,
        bottom: BottomPaneKind::Timeline,
        default_inspector_tab: "object",
        overlay_shelf: true,
    },
];

/// Perfil canônico do workspace (sempre existe: 4 workspaces, 4 perfis).
pub fn profile_for(workspace: Workspace) -> &'static WorkspaceLayoutProfile {
    PROFILES
        .iter()
        .find(|p| p.id == workspace)
        .expect("workspace sem perfil de layout")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_workspace_has_a_profile() {
        for ws in Workspace::all() {
            assert_eq!(profile_for(ws).id, ws);
        }
    }

    #[test]
    fn uv_is_the_only_split_center() {
        for ws in Workspace::all() {
            assert_eq!(
                profile_for(ws).center == CenterKind::UvSplit,
                ws == Workspace::Uv
            );
        }
    }

    #[test]
    fn animate_is_the_only_timeline_bottom() {
        for ws in Workspace::all() {
            assert_eq!(
                profile_for(ws).bottom == BottomPaneKind::Timeline,
                ws == Workspace::Animate
            );
        }
    }

    #[test]
    fn tool_palettes_are_distinct_per_workspace() {
        let kinds: Vec<ToolPaletteKind> = Workspace::all()
            .iter()
            .map(|ws| profile_for(*ws).left_tools)
            .collect();
        let mut sorted = kinds.clone();
        sorted.sort_by_key(|k| *k as u8);
        sorted.dedup_by_key(|k| *k as u8);
        assert_eq!(sorted.len(), 4);
    }
}
