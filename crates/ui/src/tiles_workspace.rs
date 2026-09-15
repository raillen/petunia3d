//! Sistema de ancoragem e painéis dinâmicos do Petunia3D com `egui_tiles`.
//!
//! Implementa a macroestrutura modular inspirada no Blender/Cinema4D, com suporte
//! a drag-and-drop de abas, divisão e redimensionamento, estilizado estritamente
//! conforme os tokens visuais do Petunia Design System.

use egui::{Color32, Stroke, Style, Ui, Visuals, WidgetText};
use egui_tiles::{Behavior, TabState, TileId, Tiles, Tree, UiResponse};
use petunia_core::{AppState, ModuleRegistry};
use petunia_module_model::ToolRegistry;
use serde::{Deserialize, Serialize};

use crate::{outliner, properties_panel, timeline, tokens};

/// Identificador dos tipos de painéis disponíveis na área de trabalho.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PetuniaPane {
    Viewport,
    Outliner,
    Properties,
    Timeline,
    AssetBrowser,
}

/// Comportamento personalizado do `egui_tiles` integrado ao Petunia Design System.
pub struct PetuniaTilesBehavior<'a> {
    pub state: &'a mut AppState,
    pub tools: Option<&'a ToolRegistry>,
    pub registry: Option<&'a mut ModuleRegistry>,
}

impl<'a> PetuniaTilesBehavior<'a> {
    pub fn new(state: &'a mut AppState) -> Self {
        Self {
            state,
            tools: None,
            registry: None,
        }
    }

    pub fn with_registries(
        state: &'a mut AppState,
        tools: &'a ToolRegistry,
        registry: &'a mut ModuleRegistry,
    ) -> Self {
        Self {
            state,
            tools: Some(tools),
            registry: Some(registry),
        }
    }
}

impl<'a> Behavior<PetuniaPane> for PetuniaTilesBehavior<'a> {
    fn tab_title_for_pane(&mut self, pane: &PetuniaPane) -> WidgetText {
        match pane {
            PetuniaPane::Viewport => "3D Viewport".into(),
            PetuniaPane::Outliner => "Outliner".into(),
            PetuniaPane::Properties => "Properties".into(),
            PetuniaPane::Timeline => "Timeline".into(),
            PetuniaPane::AssetBrowser => "Asset Browser".into(),
        }
    }

    fn pane_ui(&mut self, ui: &mut Ui, _tile_id: TileId, pane: &mut PetuniaPane) -> UiResponse {
        match pane {
            PetuniaPane::Viewport => {
                // O viewport 3D real é renderizado pelo pipeline GPU (wgpu/glow).
                // Desenhamos a área reservada de visualização de alto contraste.
                ui.centered_and_justified(|ui| {
                    ui.label(
                        egui::RichText::new("3D Viewport Canvas")
                            .color(tokens::TEXT_SECONDARY)
                            .size(13.0),
                    );
                });
            }
            PetuniaPane::Outliner => {
                outliner::draw(ui, self.state);
            }
            PetuniaPane::Properties => {
                if let (Some(tools), Some(registry)) = (self.tools, self.registry.as_deref_mut()) {
                    properties_panel::draw(ui, self.state, tools, registry);
                } else {
                    let default_tools = ToolRegistry::new();
                    let mut default_registry = ModuleRegistry::new();
                    properties_panel::draw(ui, self.state, &default_tools, &mut default_registry);
                }
            }
            PetuniaPane::Timeline => {
                timeline::draw(ui, self.state);
            }
            PetuniaPane::AssetBrowser => {
                ui.heading("Asset Browser");
                ui.label("Coleção de materiais, primitivas e modelos pré-fabricados.");
            }
        }

        UiResponse::None
    }

    // Customizações estéticas Petunia Dark Theme
    fn tab_bar_color(&self, _visuals: &Visuals) -> Color32 {
        tokens::BG_PANEL
    }

    fn tab_bg_color(
        &self,
        _visuals: &Visuals,
        _tiles: &Tiles<PetuniaPane>,
        _tile_id: TileId,
        state: &TabState,
    ) -> Color32 {
        if state.active {
            tokens::BG_SURFACE
        } else {
            tokens::BG_PANEL
        }
    }

    fn tab_text_color(
        &self,
        _visuals: &Visuals,
        _tiles: &Tiles<PetuniaPane>,
        _tile_id: TileId,
        state: &TabState,
    ) -> Color32 {
        if state.active {
            tokens::TEXT_ACTIVE
        } else {
            tokens::TEXT_SECONDARY
        }
    }

    fn tab_bar_hline_stroke(&self, _visuals: &Visuals) -> Stroke {
        Stroke::new(1.0_f32, tokens::BORDER_SUBTLE)
    }

    fn resize_stroke(&self, _style: &Style, _resize_state: egui_tiles::ResizeState) -> Stroke {
        Stroke::new(1.5_f32, tokens::BORDER_SUBTLE)
    }

    fn drag_preview_stroke(&self, _visuals: &Visuals) -> Stroke {
        Stroke::new(1.5_f32, tokens::ACCENT_BLUE)
    }

    fn drag_preview_color(&self, _visuals: &Visuals) -> Color32 {
        Color32::from_rgba_unmultiplied(68, 142, 247, 40)
    }

    fn is_tab_closable(&self, _tiles: &Tiles<PetuniaPane>, _tile_id: TileId) -> bool {
        true
    }
}

/// Cria uma árvore canônica de tiles correspondente ao layout golden reference (Blender.svg).
pub fn create_canonical_tree() -> Tree<PetuniaPane> {
    let mut tiles = Tiles::default();

    let viewport = tiles.insert_pane(PetuniaPane::Viewport);
    let outliner = tiles.insert_pane(PetuniaPane::Outliner);
    let properties = tiles.insert_pane(PetuniaPane::Properties);

    // Barra lateral direita: Outliner em cima, Propriedades embaixo
    let right_sidebar = tiles.insert_vertical_tile(vec![outliner, properties]);

    // Raiz: Divisão horizontal entre Viewport 3D e Barra Lateral Direita
    let root = tiles.insert_horizontal_tile(vec![viewport, right_sidebar]);

    Tree::new("petunia_workspace_tree", root, tiles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_tree_structure() {
        let tree = create_canonical_tree();
        assert!(tree.root().is_some());
    }

    #[test]
    fn test_pane_titles() {
        let mut state = AppState::new("en");
        let mut behavior = PetuniaTilesBehavior::new(&mut state);

        assert_eq!(
            behavior.tab_title_for_pane(&PetuniaPane::Viewport).text(),
            "3D Viewport"
        );
        assert_eq!(
            behavior.tab_title_for_pane(&PetuniaPane::Outliner).text(),
            "Outliner"
        );
        assert_eq!(
            behavior.tab_title_for_pane(&PetuniaPane::Properties).text(),
            "Properties"
        );
        assert_eq!(
            behavior.tab_title_for_pane(&PetuniaPane::Timeline).text(),
            "Timeline"
        );
    }

    #[test]
    fn test_tiles_tree_renders_without_panic() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");
        let mut tree = create_canonical_tree();

        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                let mut behavior = PetuniaTilesBehavior::new(&mut state);
                tree.ui(&mut behavior, ui);
            });
        })
        .textures_delta
        .clear();
    }
}
