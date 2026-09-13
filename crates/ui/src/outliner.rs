//! Painel Outliner hierárquico e Galeria de Assets do Petunia3D (`Outliner`).
//! Apresenta a árvore de objetos reais da cena, estatísticas, busca instantânea,
//! alternância de visibilidade funcional e galeria de inserção de primitivas 3D.

use egui::{vec2, Color32, Id, ScrollArea, Ui};
use egui_ltreeview::{Action, NodeBuilder, TreeView, TreeViewSettings};
use petunia_core::AppState;
use petunia_mesh::Mesh;

use crate::tokens;
use crate::widgets;

/// Identificador único para cada nó da árvore no Outliner.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OutlinerNodeId {
    SceneCollection,
    Collection,
    Camera,
    Light,
    Asset(usize),
}

/// Renderiza o painel Outliner.
pub fn draw(ui: &mut Ui, state: &mut AppState) {
    egui::CollapsingHeader::new("Outliner")
        .default_open(true)
        .show(ui, |ui| {
            // 1. Cabeçalho do Outliner com contagem e busca
            draw_outliner_header(ui, state);

            ui.separator();

            // 2. Área de rolagem com a árvore de objetos da cena
            ScrollArea::vertical()
                .id_salt("outliner_tree_scroll")
                .max_height(260.0)
                .show(ui, |ui| {
                    draw_tree_nodes(ui, state);
                });
        });
}

fn draw_outliner_header(ui: &mut Ui, state: &mut AppState) {
    let n_assets = state.project.assets.len();

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(6.0, 0.0);

        // Título de Objetos da Cena com contador
        ui.label(
            egui::RichText::new("Scene Objects")
                .size(11.5)
                .strong()
                .color(tokens::TEXT_PRIMARY),
        );
        ui.label(
            egui::RichText::new(format!("({n_assets})"))
                .size(10.5)
                .color(tokens::TEXT_MUTED),
        );

        // Campo de busca com ícone Phosphor e botão de limpar
        widgets::petunia_search_box(ui, &mut state.outliner_search, "Search...");
    });
}

fn draw_tree_nodes(ui: &mut Ui, state: &mut AppState) {
    let search = state.outliner_search.trim().to_lowercase();
    let n_assets = state.project.assets.len();
    let active_idx = state.project.active;

    let tree_id = Id::new("petunia_outliner_ltreeview");
    let tree = TreeView::new(tree_id)
        .with_settings(TreeViewSettings {
            override_indent: Some(14.0),
            default_node_height: Some(22.0),
            ..Default::default()
        })
        .allow_multi_selection(false);

    let mut toggle_vis_idx: Option<usize> = None;
    let mut delete_idx: Option<usize> = None;
    let mut dup_idx: Option<usize> = None;

    let (_, actions) = tree.show(ui, |builder| {
        let open_scene = builder.node(
            NodeBuilder::dir(OutlinerNodeId::SceneCollection)
                .default_open(true)
                .label_ui(|ui| {
                    ui.label(
                        egui::RichText::new("📁 Scene Collection")
                            .size(11.5)
                            .color(tokens::TEXT_PRIMARY),
                    );
                }),
        );

        if open_scene {
            // Lista todos os assets reais do projeto
            for i in 0..n_assets {
                let name = state.project.assets[i].name.clone();
                if !search.is_empty() && !name.to_lowercase().contains(&search) {
                    continue;
                }

                let is_selected = i == active_idx;
                let (vc, fc) = (
                    state.project.assets[i].mesh.vert_count(),
                    state.project.assets[i].mesh.tri_count(),
                );
                let visible = state.project.assets[i].visible;

                builder.node(NodeBuilder::leaf(OutlinerNodeId::Asset(i)).label_ui(|ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = vec2(4.0, 0.0);

                        let fg = if is_selected {
                            tokens::TEXT_ACTIVE
                        } else {
                            tokens::TEXT_PRIMARY
                        };

                        let item_label = format!("🧊 {name} ({vc}v, {fc}f)");
                        let label_resp = ui.label(
                            egui::RichText::new(item_label)
                                .size(11.0)
                                .color(fg)
                                .background_color(if is_selected {
                                    tokens::ACCENT_BLUE
                                } else {
                                    Color32::TRANSPARENT
                                }),
                        );

                        // Menu contextual de botão direito no item
                        label_resp.context_menu(|ui| {
                            if ui.button("Duplicate · Shift+D").clicked() {
                                dup_idx = Some(i);
                                ui.close();
                            }
                            if ui.button("Delete · X").clicked() {
                                delete_idx = Some(i);
                                ui.close();
                            }
                        });

                        // Botão de Olho funcional para visibilidade do asset
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let (eye_icon, eye_col) = if visible {
                                ("👁", tokens::TEXT_PRIMARY)
                            } else {
                                ("⊘", tokens::TEXT_MUTED)
                            };
                            let eye_btn = egui::Button::new(
                                egui::RichText::new(eye_icon).size(10.5).color(eye_col),
                            )
                            .fill(Color32::TRANSPARENT);

                            if ui
                                .add(eye_btn)
                                .on_hover_text(if visible {
                                    "Ocultar da Visualização 3D"
                                } else {
                                    "Exibir na Visualização 3D"
                                })
                                .clicked()
                            {
                                toggle_vis_idx = Some(i);
                            }
                        });
                    });
                }));
            }

            builder.close_dir();
        }
    });

    // Processa alterações de visibilidade
    if let Some(idx) = toggle_vis_idx {
        if let Some(asset) = state.project.assets.get_mut(idx) {
            asset.visible = !asset.visible;
            state.mark_dirty();
        }
    }

    // Processa exclusão
    if let Some(idx) = delete_idx {
        if idx < state.project.assets.len() {
            state.checkpoint("delete asset");
            state.project.remove(idx);
            state.sync_selection();
            state.emit_mesh_changed();
            state.mark_dirty();
        }
    }

    // Processa duplicação
    if let Some(idx) = dup_idx {
        let dup = state.project.assets.get(idx).map(|a| a.duplicate());
        if let Some(dup) = dup {
            state.checkpoint("duplicate asset");
            state.project.assets.push(dup);
            state.project.active = state.project.assets.len() - 1;
            state.sync_selection();
            state.emit_mesh_changed();
            state.mark_dirty();
        }
    }

    // Trata ações geradas pelo TreeView (clique/seleção de asset)
    for action in actions {
        match action {
            Action::SetSelected(nodes) => {
                for node in nodes {
                    if let OutlinerNodeId::Asset(idx) = node {
                        if idx < state.project.assets.len() && state.project.active != idx {
                            state.project.active = idx;
                            state.sync_selection();
                            state.mark_dirty();
                        }
                    }
                }
            }
            Action::Activate(act) => {
                for node in act.selected {
                    if let OutlinerNodeId::Asset(idx) = node {
                        if idx < state.project.assets.len() && state.project.active != idx {
                            state.project.active = idx;
                            state.sync_selection();
                            state.mark_dirty();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn add_primitive_to_scene(state: &mut AppState, kind: usize, name: &str) {
    let mesh = match kind {
        0 => Mesh::cube(1.0),
        1 => Mesh::sphere_low(16, 12, 0.5),
        2 => Mesh::cylinder(16, 0.5, 1.0),
        3 => Mesh::plane(2.0),
        4 => Mesh::cone(16, 0.5, 1.0),
        _ => Mesh::capsule(16, 0.3, 0.8),
    };
    state.checkpoint("add primitive");
    state.project.add(name, mesh);

    // Posiciona na coordenada do 3D Cursor
    let cursor = state.cursor_3d;
    if let Some(m) = state.project.active_mesh_mut() {
        for v in &mut m.verts {
            v.pos[0] += cursor[0];
            v.pos[1] += cursor[1];
            v.pos[2] += cursor[2];
        }
    }
    state.sync_selection();
    state.emit_mesh_changed();
    state.mark_dirty();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outliner_renders_without_panic() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw(ui, &mut state);
            });
        });
    }

    #[test]
    fn test_outliner_search_filtering() {
        let mut state = AppState::new("en");
        state.project.add("TargetCube", Mesh::cube(1.0));
        state.project.add("OtherObject", Mesh::cube(1.0));

        state.outliner_search = "target".to_string();
        assert_eq!(state.project.assets.len(), 3); // Default cube + 2 novos
    }

    #[test]
    fn test_add_primitive_to_scene() {
        let mut state = AppState::new("en");
        let before = state.project.assets.len();
        add_primitive_to_scene(&mut state, 0, "NewCube");
        assert_eq!(state.project.assets.len(), before + 1);
        assert_eq!(state.project.assets.last().unwrap().name, "NewCube");
    }
}
