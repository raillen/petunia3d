//! Painel Outliner hierárquico e Galeria de Assets do Petunia3D (`Outliner`).
//! Apresenta a árvore de objetos reais da cena, coleções especializadas de Anotações e Medidas,
//! suporte a coleções/pastas de geometria, bloqueio de transformação, isolamento de visualização,
//! imagens de referência e estatísticas.

use egui::{vec2, Color32, Id, ScrollArea, Ui};
use egui_ltreeview::{Action, NodeBuilder, TreeView, TreeViewSettings};
use petunia_core::{AnnotationItem, AppState};
use petunia_mesh::Mesh;
use uuid::Uuid;

use crate::tokens;
use crate::widgets;

/// Identificador único para cada nó da árvore no Outliner.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OutlinerNodeId {
    AnnotationCollection,
    AnnotationSubgroup(String),
    Annotation(Uuid),
    MeasurementCollection,
    Measurement(Uuid),
    SceneCollection,
    Collection(String),
    Camera,
    Light,
    Asset(usize),
    ReferenceImages,
    ReferenceImage(usize),
}

/// Renderiza o painel Outliner.
pub fn draw(ui: &mut Ui, state: &mut AppState) {
    egui::CollapsingHeader::new("Outliner")
        .default_open(true)
        .show(ui, |ui| {
            // 1. Cabeçalho do Outliner com contagem, ações e busca
            draw_outliner_header(ui, state);

            ui.separator();

            // 2. Área de rolagem com a árvore hierárquica completa
            ScrollArea::vertical()
                .id_salt("outliner_tree_scroll")
                .max_height(280.0)
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

        // Botão de Isolar Objeto
        let is_iso = state.isolate_active;
        let iso_btn = egui::Button::new(
            egui::RichText::new(if is_iso {
                "⌖ Isolar [Ativo]"
            } else {
                "⌖ Isolar"
            })
            .size(10.0)
            .color(if is_iso {
                Color32::WHITE
            } else {
                tokens::TEXT_SECONDARY
            }),
        )
        .fill(if is_iso {
            tokens::ACCENT_BLUE
        } else {
            tokens::BG_SURFACE
        });
        if ui
            .add(iso_btn)
            .on_hover_text(if is_iso {
                "Isolar Ativo (Numpad /): Restaurar visibilidade de todos os objetos"
            } else {
                "Isolar Objeto Ativo (Numpad /): Esconder todos os outros e focar no selecionado"
            })
            .clicked()
        {
            state.toggle_isolate();
        }

        // Botão de Nova Coleção de Malha
        let new_col_btn = egui::Button::new(
            egui::RichText::new("📁+ Pasta")
                .size(10.0)
                .color(tokens::TEXT_SECONDARY),
        )
        .fill(tokens::BG_SURFACE);
        if ui
            .add(new_col_btn)
            .on_hover_text("Criar nova Pasta/Coleção para organizar modelos")
            .clicked()
        {
            let mut num = state.project.collections.len() + 1;
            let mut name = format!("Coleção {num}");
            while state.project.collections.contains(&name) {
                num += 1;
                name = format!("Coleção {num}");
            }
            state.project.add_collection(&name);
            state.mark_dirty();
        }
    });

    ui.add_space(2.0);

    // Campo de busca com ícone e botão de limpar
    widgets::petunia_search_box(ui, &mut state.outliner_search, "Search...");
}

fn draw_tree_nodes(ui: &mut Ui, state: &mut AppState) {
    let search = state.outliner_search.trim().to_lowercase();
    let n_assets = state.project.assets.len();
    let active_idx = state.project.active;
    let collections = state.project.collections.clone();

    let tree_id = Id::new("petunia_outliner_ltreeview");
    let tree = TreeView::new(tree_id)
        .with_settings(TreeViewSettings {
            override_indent: Some(14.0),
            default_node_height: Some(22.0),
            ..Default::default()
        })
        .allow_multi_selection(false);

    // Ações para meshes
    let mut toggle_vis_idx: Option<usize> = None;
    let mut toggle_lock_idx: Option<usize> = None;
    let mut delete_idx: Option<usize> = None;
    let mut dup_idx: Option<usize> = None;
    let mut isolate_idx: Option<usize> = None;
    let mut move_to_col: Option<(usize, Option<String>)> = None;

    let mut toggle_col_vis: Option<String> = None;
    let mut toggle_col_lock: Option<String> = None;
    let mut delete_col: Option<String> = None;
    let mut rename_col: Option<(String, String)> = None;

    // Ações para referências
    let mut ref_toggle_vis: Option<usize> = None;
    let mut ref_toggle_xray: Option<usize> = None;
    let mut ref_delete: Option<usize> = None;

    // Ações para anotações
    let mut toggle_ann_vis: Option<Uuid> = None;
    let mut toggle_ann_lock: Option<Uuid> = None;
    let mut delete_ann: Option<Uuid> = None;
    let mut dup_ann: Option<Uuid> = None;
    let mut ann_move_to_group: Option<(Uuid, Option<String>)> = None;
    let mut toggle_all_ann_vis = false;
    let mut toggle_all_ann_lock = false;
    let mut clear_all_ann = false;
    let mut add_ann_subgroup: Option<String> = None;
    let mut delete_ann_subgroup: Option<String> = None;

    // Ações para medidas
    let mut toggle_meas_vis: Option<Uuid> = None;
    let mut delete_meas: Option<Uuid> = None;
    let mut toggle_all_meas_vis = false;
    let mut clear_all_meas = false;

    let show_ann_collection = !state.project.annotations.is_empty()
        || !state.project.annotation_groups.is_empty()
        || state.active_tool == "annotate";
    let show_meas_collection =
        !state.project.measurements.is_empty() || state.active_tool == "measure";

    let (_, actions) = tree.show(ui, |builder| {
        // =========================================================================
        // SEÇÃO 1: 📝 ANOTAÇÕES (Acima das existentes, tipo dedicado, cor ciano)
        // =========================================================================
        if show_ann_collection {
            let n_anns = state.project.annotations.len();
            let all_ann_vis = state.project.annotations_visible;
            let all_ann_lock = state.project.annotations_locked;

            let open_ann = builder.node(
                NodeBuilder::dir(OutlinerNodeId::AnnotationCollection)
                    .default_open(true)
                    .label_ui(|ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = vec2(4.0, 0.0);
                            let col_resp = ui.label(
                                egui::RichText::new(format!("📝 Anotações ({n_anns})"))
                                    .size(11.5)
                                    .strong()
                                    .color(Color32::from_rgb(0, 210, 211)),
                            );

                            col_resp.context_menu(|ui| {
                                if ui.button("📁+ Novo Subgrupo...").clicked() {
                                    let mut num = state.project.annotation_groups.len() + 1;
                                    let mut name = format!("Subgrupo {num}");
                                    while state.project.annotation_groups.contains(&name) {
                                        num += 1;
                                        name = format!("Subgrupo {num}");
                                    }
                                    add_ann_subgroup = Some(name);
                                    ui.close();
                                }
                                ui.separator();
                                if ui.button("👁 Alternar Visibilidade da Coleção").clicked() {
                                    toggle_all_ann_vis = true;
                                    ui.close();
                                }
                                if ui.button("🔒 Alternar Bloqueio da Coleção").clicked() {
                                    toggle_all_ann_lock = true;
                                    ui.close();
                                }
                                ui.separator();
                                if ui.button("🗑 Limpar Todas as Anotações").clicked() {
                                    clear_all_ann = true;
                                    ui.close();
                                }
                            });

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    // 1. Visibilidade de toda a coleção de anotações
                                    let (eye_icon, eye_col) = if all_ann_vis {
                                        ("👁", tokens::TEXT_PRIMARY)
                                    } else {
                                        ("⊘", tokens::TEXT_MUTED)
                                    };
                                    if ui
                                        .add(
                                            egui::Button::new(
                                                egui::RichText::new(eye_icon)
                                                    .size(10.5)
                                                    .color(eye_col),
                                            )
                                            .fill(Color32::TRANSPARENT),
                                        )
                                        .on_hover_text("Ocultar/Exibir todas as anotações")
                                        .clicked()
                                    {
                                        toggle_all_ann_vis = true;
                                    }

                                    // 2. Bloqueio de toda a coleção de anotações
                                    let (lock_icon, lock_col) = if all_ann_lock {
                                        ("🔒", Color32::from_rgb(0xe6, 0x7e, 0x22))
                                    } else {
                                        ("🔓", tokens::TEXT_MUTED)
                                    };
                                    if ui
                                        .add(
                                            egui::Button::new(
                                                egui::RichText::new(lock_icon)
                                                    .size(10.5)
                                                    .color(lock_col),
                                            )
                                            .fill(Color32::TRANSPARENT),
                                        )
                                        .on_hover_text("Bloquear/Desbloquear todas as anotações")
                                        .clicked()
                                    {
                                        toggle_all_ann_lock = true;
                                    }
                                },
                            );
                        });
                    }),
            );

            if open_ann {
                // Subgrupos internos de anotações
                for group_name in &state.project.annotation_groups.clone() {
                    let anns_in_group: Vec<AnnotationItem> = state
                        .project
                        .annotations
                        .iter()
                        .filter(|a| a.group.as_deref() == Some(group_name))
                        .cloned()
                        .collect();

                    let group_for_closure = group_name.clone();
                    let count = anns_in_group.len();

                    let open_group = builder.node(
                        NodeBuilder::dir(OutlinerNodeId::AnnotationSubgroup(group_name.clone()))
                            .default_open(true)
                            .label_ui(|ui| {
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing = vec2(4.0, 0.0);
                                    let resp = ui.label(
                                        egui::RichText::new(format!(
                                            "📁 {group_for_closure} ({count})"
                                        ))
                                        .size(11.0)
                                        .color(Color32::from_rgb(0, 180, 180)),
                                    );
                                    resp.context_menu(|ui| {
                                        if ui.button("🗑 Excluir Subgrupo").clicked() {
                                            delete_ann_subgroup = Some(group_for_closure.clone());
                                            ui.close();
                                        }
                                    });
                                });
                            }),
                    );

                    if open_group {
                        for ann in anns_in_group {
                            let is_selected = state.selected_annotation == Some(ann.id);
                            let ann_id = ann.id;
                            let name = ann.name.clone();
                            let visible = ann.visible;
                            let locked = ann.locked;

                            builder.node(
                                NodeBuilder::leaf(OutlinerNodeId::Annotation(ann.id)).label_ui(
                                    |ui| {
                                        ui.horizontal(|ui| {
                                            ui.spacing_mut().item_spacing = vec2(4.0, 0.0);
                                            let fg = if is_selected {
                                                tokens::TEXT_ACTIVE
                                            } else {
                                                Color32::from_rgb(0, 210, 211)
                                            };

                                            let label_resp = ui.label(
                                                egui::RichText::new(format!("✏ {name}"))
                                                    .size(11.0)
                                                    .color(fg)
                                                    .background_color(if is_selected {
                                                        Color32::from_rgb(0, 100, 120)
                                                    } else {
                                                        Color32::TRANSPARENT
                                                    }),
                                            );

                                            label_resp.context_menu(|ui| {
                                                ui.menu_button(
                                                    "📁 Agrupar em Subgrupo ▾",
                                                    |ui| {
                                                        if ui.button("Nenhum (Raiz)").clicked() {
                                                            ann_move_to_group =
                                                                Some((ann_id, None));
                                                            ui.close();
                                                        }
                                                        for g in &state.project.annotation_groups {
                                                            if ui
                                                                .button(format!("📁 {g}"))
                                                                .clicked()
                                                            {
                                                                ann_move_to_group =
                                                                    Some((ann_id, Some(g.clone())));
                                                                ui.close();
                                                            }
                                                        }
                                                    },
                                                );
                                                ui.separator();
                                                let lock_txt = if locked {
                                                    "🔓 Desbloquear"
                                                } else {
                                                    "🔒 Bloquear"
                                                };
                                                if ui.button(lock_txt).clicked() {
                                                    toggle_ann_lock = Some(ann_id);
                                                    ui.close();
                                                }
                                                if ui.button("📋 Duplicar").clicked() {
                                                    dup_ann = Some(ann_id);
                                                    ui.close();
                                                }
                                                if ui.button("🗑 Deletar · Delete").clicked() {
                                                    delete_ann = Some(ann_id);
                                                    ui.close();
                                                }
                                            });

                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    // Visibilidade
                                                    let (eye_icon, eye_col) = if visible {
                                                        ("👁", tokens::TEXT_PRIMARY)
                                                    } else {
                                                        ("⊘", tokens::TEXT_MUTED)
                                                    };
                                                    if ui
                                                        .add(
                                                            egui::Button::new(
                                                                egui::RichText::new(eye_icon)
                                                                    .size(10.5)
                                                                    .color(eye_col),
                                                            )
                                                            .fill(Color32::TRANSPARENT),
                                                        )
                                                        .clicked()
                                                    {
                                                        toggle_ann_vis = Some(ann_id);
                                                    }

                                                    // Bloqueio
                                                    let (lock_icon, lock_col) = if locked {
                                                        ("🔒", Color32::from_rgb(0xe6, 0x7e, 0x22))
                                                    } else {
                                                        ("🔓", tokens::TEXT_MUTED)
                                                    };
                                                    if ui
                                                        .add(
                                                            egui::Button::new(
                                                                egui::RichText::new(lock_icon)
                                                                    .size(10.5)
                                                                    .color(lock_col),
                                                            )
                                                            .fill(Color32::TRANSPARENT),
                                                        )
                                                        .clicked()
                                                    {
                                                        toggle_ann_lock = Some(ann_id);
                                                    }
                                                },
                                            );
                                        });
                                    },
                                ),
                            );
                        }
                        builder.close_dir();
                    }
                }

                // Anotações na raiz da coleção (sem subgrupo)
                for ann in state
                    .project
                    .annotations
                    .iter()
                    .filter(|a| a.group.is_none())
                    .cloned()
                {
                    let is_selected = state.selected_annotation == Some(ann.id);
                    let ann_id = ann.id;
                    let name = ann.name.clone();
                    let visible = ann.visible;
                    let locked = ann.locked;

                    builder.node(
                        NodeBuilder::leaf(OutlinerNodeId::Annotation(ann.id)).label_ui(|ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing = vec2(4.0, 0.0);
                                let fg = if is_selected {
                                    tokens::TEXT_ACTIVE
                                } else {
                                    Color32::from_rgb(0, 210, 211)
                                };

                                let label_resp = ui.label(
                                    egui::RichText::new(format!("✏ {name}"))
                                        .size(11.0)
                                        .color(fg)
                                        .background_color(if is_selected {
                                            Color32::from_rgb(0, 100, 120)
                                        } else {
                                            Color32::TRANSPARENT
                                        }),
                                );

                                label_resp.context_menu(|ui| {
                                    if !state.project.annotation_groups.is_empty() {
                                        ui.menu_button("📁 Agrupar em Subgrupo ▾", |ui| {
                                            for g in &state.project.annotation_groups {
                                                if ui.button(format!("📁 {g}")).clicked() {
                                                    ann_move_to_group =
                                                        Some((ann_id, Some(g.clone())));
                                                    ui.close();
                                                }
                                            }
                                        });
                                        ui.separator();
                                    }
                                    let lock_txt = if locked {
                                        "🔓 Desbloquear"
                                    } else {
                                        "🔒 Bloquear"
                                    };
                                    if ui.button(lock_txt).clicked() {
                                        toggle_ann_lock = Some(ann_id);
                                        ui.close();
                                    }
                                    if ui.button("📋 Duplicar").clicked() {
                                        dup_ann = Some(ann_id);
                                        ui.close();
                                    }
                                    if ui.button("🗑 Deletar · Delete").clicked() {
                                        delete_ann = Some(ann_id);
                                        ui.close();
                                    }
                                });

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        // Visibilidade
                                        let (eye_icon, eye_col) = if visible {
                                            ("👁", tokens::TEXT_PRIMARY)
                                        } else {
                                            ("⊘", tokens::TEXT_MUTED)
                                        };
                                        if ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new(eye_icon)
                                                        .size(10.5)
                                                        .color(eye_col),
                                                )
                                                .fill(Color32::TRANSPARENT),
                                            )
                                            .clicked()
                                        {
                                            toggle_ann_vis = Some(ann_id);
                                        }

                                        // Bloqueio
                                        let (lock_icon, lock_col) = if locked {
                                            ("🔒", Color32::from_rgb(0xe6, 0x7e, 0x22))
                                        } else {
                                            ("🔓", tokens::TEXT_MUTED)
                                        };
                                        if ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new(lock_icon)
                                                        .size(10.5)
                                                        .color(lock_col),
                                                )
                                                .fill(Color32::TRANSPARENT),
                                            )
                                            .clicked()
                                        {
                                            toggle_ann_lock = Some(ann_id);
                                        }
                                    },
                                );
                            });
                        }),
                    );
                }

                builder.close_dir();
            }
        }

        // =========================================================================
        // SEÇÃO 2: 📏 MEDIDAS (Acima das de malha, tipo dedicado, cor amarela)
        // =========================================================================
        if show_meas_collection {
            let n_meas = state.project.measurements.len();
            let all_meas_vis = state.project.measurements_visible;

            let open_meas = builder.node(
                NodeBuilder::dir(OutlinerNodeId::MeasurementCollection)
                    .default_open(true)
                    .label_ui(|ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = vec2(4.0, 0.0);
                            let col_resp = ui.label(
                                egui::RichText::new(format!("📏 Medidas ({n_meas})"))
                                    .size(11.5)
                                    .strong()
                                    .color(Color32::from_rgb(254, 202, 87)),
                            );

                            col_resp.context_menu(|ui| {
                                if ui.button("👁 Alternar Visibilidade da Coleção").clicked() {
                                    toggle_all_meas_vis = true;
                                    ui.close();
                                }
                                ui.separator();
                                if ui.button("🗑 Limpar Todas as Medidas").clicked() {
                                    clear_all_meas = true;
                                    ui.close();
                                }
                            });

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    let (eye_icon, eye_col) = if all_meas_vis {
                                        ("👁", tokens::TEXT_PRIMARY)
                                    } else {
                                        ("⊘", tokens::TEXT_MUTED)
                                    };
                                    if ui
                                        .add(
                                            egui::Button::new(
                                                egui::RichText::new(eye_icon)
                                                    .size(10.5)
                                                    .color(eye_col),
                                            )
                                            .fill(Color32::TRANSPARENT),
                                        )
                                        .on_hover_text("Ocultar/Exibir todas as medidas")
                                        .clicked()
                                    {
                                        toggle_all_meas_vis = true;
                                    }
                                },
                            );
                        });
                    }),
            );

            if open_meas {
                for m in state.project.measurements.clone() {
                    let m_id = m.id;
                    let is_selected = state.selected_measurement == Some(m_id);
                    let name = m.name.clone();
                    let dist = m.distance;
                    let visible = m.visible;

                    builder.node(
                        NodeBuilder::leaf(OutlinerNodeId::Measurement(m_id)).label_ui(|ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing = vec2(4.0, 0.0);
                                let fg = if is_selected {
                                    tokens::TEXT_ACTIVE
                                } else {
                                    Color32::from_rgb(254, 202, 87)
                                };

                                let label_resp = ui.label(
                                    egui::RichText::new(format!("📏 {name} ({dist:.2}m)"))
                                        .size(11.0)
                                        .color(fg)
                                        .background_color(if is_selected {
                                            Color32::from_rgb(120, 100, 20)
                                        } else {
                                            Color32::TRANSPARENT
                                        }),
                                );

                                label_resp.context_menu(|ui| {
                                    if ui.button("🗑 Deletar · Delete").clicked() {
                                        delete_meas = Some(m_id);
                                        ui.close();
                                    }
                                });

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        // Visibilidade (único controle além de deletar)
                                        let (eye_icon, eye_col) = if visible {
                                            ("👁", tokens::TEXT_PRIMARY)
                                        } else {
                                            ("⊘", tokens::TEXT_MUTED)
                                        };
                                        if ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new(eye_icon)
                                                        .size(10.5)
                                                        .color(eye_col),
                                                )
                                                .fill(Color32::TRANSPARENT),
                                            )
                                            .clicked()
                                        {
                                            toggle_meas_vis = Some(m_id);
                                        }

                                        // Botão explícito de deletar
                                        if ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new("🗑")
                                                        .size(10.0)
                                                        .color(tokens::TEXT_MUTED),
                                                )
                                                .fill(Color32::TRANSPARENT),
                                            )
                                            .on_hover_text("Excluir medida")
                                            .clicked()
                                        {
                                            delete_meas = Some(m_id);
                                        }
                                    },
                                );
                            });
                        }),
                    );
                }

                builder.close_dir();
            }
        }

        // =========================================================================
        // SEÇÃO 3: 📁 SCENE COLLECTION (Malhas 3D e Coleções de Objetos)
        // =========================================================================
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
            // 1. Coleções personalizadas de geometria
            for col_name in &collections {
                let assets_in_col: Vec<usize> = (0..n_assets)
                    .filter(|&i| state.project.assets[i].collection.as_deref() == Some(col_name))
                    .collect();

                let all_visible = !assets_in_col.is_empty()
                    && assets_in_col
                        .iter()
                        .all(|&i| state.project.assets[i].visible);
                let all_locked = !assets_in_col.is_empty()
                    && assets_in_col
                        .iter()
                        .all(|&i| state.project.assets[i].locked);

                let col_for_closure = col_name.clone();
                let count = assets_in_col.len();

                let open_col = builder.node(
                    NodeBuilder::dir(OutlinerNodeId::Collection(col_name.clone()))
                        .default_open(true)
                        .label_ui(|ui| {
                            let rename_id = egui::Id::new("petunia_renaming_col");
                            let is_renaming = ui.data(|d| {
                                d.get_temp::<String>(rename_id).as_deref() == Some(&col_for_closure)
                            });

                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing = vec2(4.0, 0.0);

                                if is_renaming {
                                    let mut edit_buf = ui.data(|d| {
                                        d.get_temp::<String>(egui::Id::new("petunia_rename_buf"))
                                            .unwrap_or_else(|| col_for_closure.clone())
                                    });
                                    let resp = ui.text_edit_singleline(&mut edit_buf);
                                    if resp.lost_focus()
                                        || ui.input(|i| i.key_pressed(egui::Key::Enter))
                                    {
                                        if edit_buf.trim() != col_for_closure
                                            && !edit_buf.trim().is_empty()
                                        {
                                            rename_col = Some((
                                                col_for_closure.clone(),
                                                edit_buf.trim().to_string(),
                                            ));
                                        }
                                        ui.data_mut(|d| d.remove_temp::<String>(rename_id));
                                        ui.data_mut(|d| {
                                            d.remove_temp::<String>(egui::Id::new(
                                                "petunia_rename_buf",
                                            ))
                                        });
                                    } else {
                                        ui.data_mut(|d| {
                                            d.insert_temp(
                                                egui::Id::new("petunia_rename_buf"),
                                                edit_buf,
                                            )
                                        });
                                    }
                                } else {
                                    let col_resp = ui.label(
                                        egui::RichText::new(format!(
                                            "📁 {col_for_closure} ({count})"
                                        ))
                                        .size(11.0)
                                        .color(tokens::TEXT_PRIMARY),
                                    );
                                    col_resp.context_menu(|ui| {
                                        if ui.button("✏ Renomear Pasta").clicked() {
                                            ui.data_mut(|d| {
                                                d.insert_temp(rename_id, col_for_closure.clone());
                                                d.insert_temp(
                                                    egui::Id::new("petunia_rename_buf"),
                                                    col_for_closure.clone(),
                                                );
                                            });
                                            ui.close();
                                        }
                                        if ui.button("🗑 Excluir Pasta").clicked() {
                                            delete_col = Some(col_for_closure.clone());
                                            ui.close();
                                        }
                                        ui.separator();
                                        if ui.button("👁 Alternar Visibilidade da Pasta").clicked()
                                        {
                                            toggle_col_vis = Some(col_for_closure.clone());
                                            ui.close();
                                        }
                                        if ui.button("🔒 Alternar Bloqueio da Pasta").clicked() {
                                            toggle_col_lock = Some(col_for_closure.clone());
                                            ui.close();
                                        }
                                    });
                                }

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        // Visibilidade coletiva
                                        let (eye_icon, eye_col) = if all_visible {
                                            ("👁", tokens::TEXT_PRIMARY)
                                        } else {
                                            ("⊘", tokens::TEXT_MUTED)
                                        };
                                        if ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new(eye_icon)
                                                        .size(10.5)
                                                        .color(eye_col),
                                                )
                                                .fill(Color32::TRANSPARENT),
                                            )
                                            .on_hover_text(
                                                "Alternar visibilidade de todos nesta pasta",
                                            )
                                            .clicked()
                                        {
                                            toggle_col_vis = Some(col_for_closure.clone());
                                        }

                                        // Bloqueio coletivo
                                        let (lock_icon, lock_col) = if all_locked {
                                            ("🔒", Color32::from_rgb(0xe6, 0x7e, 0x22))
                                        } else {
                                            ("🔓", tokens::TEXT_MUTED)
                                        };
                                        if ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new(lock_icon)
                                                        .size(10.5)
                                                        .color(lock_col),
                                                )
                                                .fill(Color32::TRANSPARENT),
                                            )
                                            .on_hover_text("Alternar bloqueio de todos nesta pasta")
                                            .clicked()
                                        {
                                            toggle_col_lock = Some(col_for_closure.clone());
                                        }
                                    },
                                );
                            });
                        }),
                );

                if open_col {
                    for i in assets_in_col {
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
                        let locked = state.project.assets[i].locked;

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

                                label_resp.context_menu(|ui| {
                                    ui.menu_button("📁 Mover para Coleção ▾", |ui| {
                                        if ui.button("📁 Nenhuma (Raiz)").clicked() {
                                            move_to_col = Some((i, None));
                                            ui.close();
                                        }
                                        ui.separator();
                                        for col in &collections {
                                            let is_cur =
                                                state.project.assets[i].collection.as_deref()
                                                    == Some(col);
                                            let label = if is_cur {
                                                format!("✓ 📁 {col}")
                                            } else {
                                                format!("📁 {col}")
                                            };
                                            if ui.button(label).clicked() {
                                                move_to_col = Some((i, Some(col.clone())));
                                                ui.close();
                                            }
                                        }
                                    });
                                    ui.separator();
                                    let lock_txt = if locked {
                                        "🔓 Desbloquear Objeto"
                                    } else {
                                        "🔒 Bloquear Objeto"
                                    };
                                    if ui.button(lock_txt).clicked() {
                                        toggle_lock_idx = Some(i);
                                        ui.close();
                                    }
                                    let iso_txt = if state.isolate_active && is_selected {
                                        "⌖ Desativar Isolar"
                                    } else {
                                        "⌖ Isolar Este Objeto (Numpad /)"
                                    };
                                    if ui.button(iso_txt).clicked() {
                                        isolate_idx = Some(i);
                                        ui.close();
                                    }
                                    ui.separator();
                                    if ui.button("Duplicate · Shift+D").clicked() {
                                        dup_idx = Some(i);
                                        ui.close();
                                    }
                                    if ui.button("Delete · X").clicked() {
                                        delete_idx = Some(i);
                                        ui.close();
                                    }
                                });

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        // 1. Visibilidade
                                        let (eye_icon, eye_col) = if visible {
                                            ("👁", tokens::TEXT_PRIMARY)
                                        } else {
                                            ("⊘", tokens::TEXT_MUTED)
                                        };
                                        if ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new(eye_icon)
                                                        .size(10.5)
                                                        .color(eye_col),
                                                )
                                                .fill(Color32::TRANSPARENT),
                                            )
                                            .on_hover_text(if visible {
                                                "Ocultar da Visualização 3D"
                                            } else {
                                                "Exibir na Visualização 3D"
                                            })
                                            .clicked()
                                        {
                                            toggle_vis_idx = Some(i);
                                        }

                                        // 2. Bloqueio
                                        let (lock_icon, lock_col) = if locked {
                                            ("🔒", Color32::from_rgb(0xe6, 0x7e, 0x22))
                                        } else {
                                            ("🔓", tokens::TEXT_MUTED)
                                        };
                                        if ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new(lock_icon)
                                                        .size(10.5)
                                                        .color(lock_col),
                                                )
                                                .fill(Color32::TRANSPARENT),
                                            )
                                            .on_hover_text(if locked {
                                                "Desbloquear Objeto (está fixado)"
                                            } else {
                                                "Bloquear Objeto (impede movimentação)"
                                            })
                                            .clicked()
                                        {
                                            toggle_lock_idx = Some(i);
                                        }
                                    },
                                );
                            });
                        }));
                    }
                    builder.close_dir();
                }
            }

            // 2. Objetos na raiz da cena (sem coleção ou com coleção inexistente)
            for i in 0..n_assets {
                let has_valid_collection = state.project.assets[i]
                    .collection
                    .as_ref()
                    .is_some_and(|c| collections.contains(c));
                if has_valid_collection {
                    continue;
                }

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
                let locked = state.project.assets[i].locked;

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

                        label_resp.context_menu(|ui| {
                            if !collections.is_empty() {
                                ui.menu_button("📁 Mover para Coleção ▾", |ui| {
                                    for col in &collections {
                                        if ui.button(format!("📁 {col}")).clicked() {
                                            move_to_col = Some((i, Some(col.clone())));
                                            ui.close();
                                        }
                                    }
                                });
                                ui.separator();
                            }
                            let lock_txt = if locked {
                                "🔓 Desbloquear Objeto"
                            } else {
                                "🔒 Bloquear Objeto"
                            };
                            if ui.button(lock_txt).clicked() {
                                toggle_lock_idx = Some(i);
                                ui.close();
                            }
                            let iso_txt = if state.isolate_active && is_selected {
                                "⌖ Desativar Isolar"
                            } else {
                                "⌖ Isolar Este Objeto (Numpad /)"
                            };
                            if ui.button(iso_txt).clicked() {
                                isolate_idx = Some(i);
                                ui.close();
                            }
                            ui.separator();
                            if ui.button("Duplicate · Shift+D").clicked() {
                                dup_idx = Some(i);
                                ui.close();
                            }
                            if ui.button("Delete · X").clicked() {
                                delete_idx = Some(i);
                                ui.close();
                            }
                        });

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // 1. Visibilidade
                            let (eye_icon, eye_col) = if visible {
                                ("👁", tokens::TEXT_PRIMARY)
                            } else {
                                ("⊘", tokens::TEXT_MUTED)
                            };
                            if ui
                                .add(
                                    egui::Button::new(
                                        egui::RichText::new(eye_icon).size(10.5).color(eye_col),
                                    )
                                    .fill(Color32::TRANSPARENT),
                                )
                                .on_hover_text(if visible {
                                    "Ocultar da Visualização 3D"
                                } else {
                                    "Exibir na Visualização 3D"
                                })
                                .clicked()
                            {
                                toggle_vis_idx = Some(i);
                            }

                            // 2. Bloqueio
                            let (lock_icon, lock_col) = if locked {
                                ("🔒", Color32::from_rgb(0xe6, 0x7e, 0x22))
                            } else {
                                ("🔓", tokens::TEXT_MUTED)
                            };
                            if ui
                                .add(
                                    egui::Button::new(
                                        egui::RichText::new(lock_icon).size(10.5).color(lock_col),
                                    )
                                    .fill(Color32::TRANSPARENT),
                                )
                                .on_hover_text(if locked {
                                    "Desbloquear Objeto (está fixado)"
                                } else {
                                    "Bloquear Objeto (impede movimentação)"
                                })
                                .clicked()
                            {
                                toggle_lock_idx = Some(i);
                            }
                        });
                    });
                }));
            }

            builder.close_dir();
        }

        // =========================================================================
        // SEÇÃO 4: 🖼 IMAGENS DE REFERÊNCIA
        // =========================================================================
        if !state.refs.is_empty() {
            let open_refs = builder.node(
                NodeBuilder::dir(OutlinerNodeId::ReferenceImages)
                    .default_open(true)
                    .label_ui(|ui| {
                        ui.label(
                            egui::RichText::new(format!(
                                "🖼 Imagens de Referência ({})",
                                state.refs.len()
                            ))
                            .size(11.5)
                            .color(tokens::TEXT_PRIMARY),
                        );
                    }),
            );

            if open_refs {
                for r_idx in 0..state.refs.len() {
                    let r_vis = state.refs[r_idx].visible;
                    let r_xray = state.refs[r_idx].xray;
                    let r_axis = format!("{:?}", state.refs[r_idx].axis);
                    let r_dim = format!("{}x{}", state.refs[r_idx].width, state.refs[r_idx].height);

                    builder.node(
                        NodeBuilder::leaf(OutlinerNodeId::ReferenceImage(r_idx)).label_ui(|ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing = vec2(4.0, 0.0);
                                let item_label = format!("🖼 {r_axis} ({r_dim})");
                                let label_resp = ui.label(
                                    egui::RichText::new(item_label)
                                        .size(11.0)
                                        .color(tokens::TEXT_PRIMARY),
                                );
                                label_resp.context_menu(|ui| {
                                    let xray_txt = if r_xray {
                                        "⚡ Desativar Raio-X"
                                    } else {
                                        "⚡ Ativar Raio-X"
                                    };
                                    if ui.button(xray_txt).clicked() {
                                        ref_toggle_xray = Some(r_idx);
                                        ui.close();
                                    }
                                    if ui.button("🗑 Remover Imagem").clicked() {
                                        ref_delete = Some(r_idx);
                                        ui.close();
                                    }
                                });

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        // Visibilidade
                                        let (eye_icon, eye_col) = if r_vis {
                                            ("👁", tokens::TEXT_PRIMARY)
                                        } else {
                                            ("⊘", tokens::TEXT_MUTED)
                                        };
                                        if ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new(eye_icon)
                                                        .size(10.5)
                                                        .color(eye_col),
                                                )
                                                .fill(Color32::TRANSPARENT),
                                            )
                                            .on_hover_text(if r_vis {
                                                "Ocultar Imagem de Referência"
                                            } else {
                                                "Exibir Imagem de Referência"
                                            })
                                            .clicked()
                                        {
                                            ref_toggle_vis = Some(r_idx);
                                        }

                                        // Raio-X
                                        let (xray_txt, xray_col) = if r_xray {
                                            ("⚡", tokens::ACCENT_BLUE)
                                        } else {
                                            ("⚡", tokens::TEXT_MUTED)
                                        };
                                        if ui
                                            .add(
                                                egui::Button::new(
                                                    egui::RichText::new(xray_txt)
                                                        .size(10.5)
                                                        .color(xray_col),
                                                )
                                                .fill(Color32::TRANSPARENT),
                                            )
                                            .on_hover_text(if r_xray {
                                                "Raio-X Ativo (visível sobre malhas)"
                                            } else {
                                                "Ativar Raio-X"
                                            })
                                            .clicked()
                                        {
                                            ref_toggle_xray = Some(r_idx);
                                        }
                                    },
                                );
                            });
                        }),
                    );
                }
                builder.close_dir();
            }
        }
    });

    // =========================================================================
    // DISPATCHING DE AÇÕES APÓS TREEVIEW
    // =========================================================================

    // Ações de anotações
    if let Some(ann_id) = toggle_ann_vis {
        if let Some(ann) = state
            .project
            .annotations
            .iter_mut()
            .find(|a| a.id == ann_id)
        {
            ann.visible = !ann.visible;
            state.mark_dirty();
        }
    }
    if let Some(ann_id) = toggle_ann_lock {
        if let Some(ann) = state
            .project
            .annotations
            .iter_mut()
            .find(|a| a.id == ann_id)
        {
            ann.locked = !ann.locked;
            state.mark_dirty();
        }
    }
    if let Some(ann_id) = delete_ann {
        state.checkpoint("delete annotation");
        state.project.remove_annotation(ann_id);
        if state.selected_annotation == Some(ann_id) {
            state.selected_annotation = None;
        }
        state.mark_dirty();
    }
    if let Some(ann_id) = dup_ann {
        let dup = state
            .project
            .annotations
            .iter()
            .find(|a| a.id == ann_id)
            .cloned();
        if let Some(mut dup) = dup {
            state.checkpoint("duplicate annotation");
            dup.id = Uuid::new_v4();
            dup.name = format!("{} (cópia)", dup.name);
            let new_id = dup.id;
            state.project.add_annotation(dup);
            state.selected_annotation = Some(new_id);
            state.mark_dirty();
        }
    }
    if let Some((ann_id, grp)) = ann_move_to_group {
        if let Some(ann) = state
            .project
            .annotations
            .iter_mut()
            .find(|a| a.id == ann_id)
        {
            ann.group = grp;
            state.mark_dirty();
        }
    }
    if toggle_all_ann_vis {
        state.project.annotations_visible = !state.project.annotations_visible;
        state.mark_dirty();
    }
    if toggle_all_ann_lock {
        state.project.annotations_locked = !state.project.annotations_locked;
        state.mark_dirty();
    }
    if clear_all_ann && !state.project.annotations.is_empty() {
        state.checkpoint("clear annotations");
        state.project.annotations.clear();
        state.selected_annotation = None;
        state.mark_dirty();
    }
    if let Some(subgroup_name) = add_ann_subgroup {
        state.project.add_annotation_group(&subgroup_name);
        state.mark_dirty();
    }
    if let Some(subgroup_name) = delete_ann_subgroup {
        state.project.remove_annotation_group(&subgroup_name);
        state.mark_dirty();
    }

    // Ações de medidas
    if let Some(meas_id) = toggle_meas_vis {
        if let Some(m) = state
            .project
            .measurements
            .iter_mut()
            .find(|m| m.id == meas_id)
        {
            m.visible = !m.visible;
            state.mark_dirty();
        }
    }
    if let Some(meas_id) = delete_meas {
        state.checkpoint("delete measurement");
        state.project.remove_measurement(meas_id);
        if state.selected_measurement == Some(meas_id) {
            state.selected_measurement = None;
        }
        state.mark_dirty();
    }
    if toggle_all_meas_vis {
        state.project.measurements_visible = !state.project.measurements_visible;
        state.mark_dirty();
    }
    if clear_all_meas && !state.project.measurements.is_empty() {
        state.checkpoint("clear measurements");
        state.project.measurements.clear();
        state.selected_measurement = None;
        state.mark_dirty();
    }

    // Ações de meshes
    if let Some(idx) = toggle_vis_idx {
        if let Some(asset) = state.project.assets.get_mut(idx) {
            asset.visible = !asset.visible;
            state.mark_dirty();
        }
    }
    if let Some(idx) = toggle_lock_idx {
        if let Some(asset) = state.project.assets.get_mut(idx) {
            asset.locked = !asset.locked;
            state.mark_dirty();
        }
    }
    if let Some(idx) = isolate_idx {
        if idx < state.project.assets.len() {
            state.project.active = idx;
            state.toggle_isolate();
        }
    }
    if let Some((idx, col)) = move_to_col {
        if let Some(asset) = state.project.assets.get_mut(idx) {
            asset.collection = col;
            state.mark_dirty();
        }
    }
    if let Some(col) = toggle_col_vis {
        let all_vis = state
            .project
            .assets
            .iter()
            .filter(|a| a.collection.as_deref() == Some(&col))
            .all(|a| a.visible);
        for a in &mut state.project.assets {
            if a.collection.as_deref() == Some(&col) {
                a.visible = !all_vis;
            }
        }
        state.mark_dirty();
    }
    if let Some(col) = toggle_col_lock {
        let all_locked = state
            .project
            .assets
            .iter()
            .filter(|a| a.collection.as_deref() == Some(&col))
            .all(|a| a.locked);
        for a in &mut state.project.assets {
            if a.collection.as_deref() == Some(&col) {
                a.locked = !all_locked;
            }
        }
        state.mark_dirty();
    }
    if let Some(col) = delete_col {
        state.project.remove_collection(&col);
        state.mark_dirty();
    }
    if let Some((old_name, new_name)) = rename_col {
        if !new_name.trim().is_empty() && !state.project.collections.contains(&new_name) {
            for c in &mut state.project.collections {
                if *c == old_name {
                    *c = new_name.clone();
                }
            }
            for a in &mut state.project.assets {
                if a.collection.as_deref() == Some(&old_name) {
                    a.collection = Some(new_name.clone());
                }
            }
            state.mark_dirty();
        }
    }

    // Ações de referências
    if let Some(idx) = ref_toggle_vis {
        if let Some(r) = state.refs.get_mut(idx) {
            r.visible = !r.visible;
            state.mark_dirty();
        }
    }
    if let Some(idx) = ref_toggle_xray {
        if let Some(r) = state.refs.get_mut(idx) {
            r.xray = !r.xray;
            state.mark_dirty();
        }
    }
    if let Some(idx) = ref_delete {
        if idx < state.refs.len() {
            state.refs.remove(idx);
            state.mark_dirty();
        }
    }
    if let Some(idx) = delete_idx {
        if idx < state.project.assets.len() {
            state.checkpoint("delete asset");
            state.project.remove(idx);
            state.sync_selection();
            state.emit_mesh_changed();
            state.mark_dirty();
        }
    }
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

    // Trata seleção pelo TreeView
    for action in actions {
        match action {
            Action::SetSelected(nodes) => {
                for node in nodes {
                    match node {
                        OutlinerNodeId::Asset(idx) => {
                            if idx < state.project.assets.len() && state.project.active != idx {
                                state.project.active = idx;
                                state.selected_annotation = None;
                                state.selected_measurement = None;
                                state.sync_selection();
                                state.mark_dirty();
                            }
                        }
                        OutlinerNodeId::Annotation(id) => {
                            state.selected_annotation = Some(id);
                            state.selected_measurement = None;
                            state.mark_dirty();
                        }
                        OutlinerNodeId::Measurement(id) => {
                            state.selected_measurement = Some(id);
                            state.selected_annotation = None;
                            state.mark_dirty();
                        }
                        _ => {}
                    }
                }
            }
            Action::Activate(act) => {
                for node in act.selected {
                    match node {
                        OutlinerNodeId::Asset(idx) => {
                            if idx < state.project.assets.len() && state.project.active != idx {
                                state.project.active = idx;
                                state.selected_annotation = None;
                                state.selected_measurement = None;
                                state.sync_selection();
                                state.mark_dirty();
                            }
                        }
                        OutlinerNodeId::Annotation(id) => {
                            state.selected_annotation = Some(id);
                            state.selected_measurement = None;
                            state.mark_dirty();
                        }
                        OutlinerNodeId::Measurement(id) => {
                            state.selected_measurement = Some(id);
                            state.selected_annotation = None;
                            state.mark_dirty();
                        }
                        _ => {}
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
    use petunia_core::MeasurementItem;

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
    fn test_outliner_renders_with_collections_and_locks() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");
        state.project.add_collection("Props");
        state.project.add("Table", Mesh::cube(1.0));
        state.project.assets.last_mut().unwrap().collection = Some("Props".to_string());
        state.project.assets.last_mut().unwrap().locked = true;

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw(ui, &mut state);
            });
        });

        assert_eq!(state.project.collections.len(), 1);
        assert!(state.project.assets.last().unwrap().locked);
    }

    #[test]
    fn test_outliner_renders_annotation_and_measurement_collections() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");
        state
            .project
            .add_annotation(AnnotationItem::new("Note 1", vec![]));
        state.project.add_measurement(MeasurementItem::new(
            "Dist 1",
            [0.0, 0.0, 0.0],
            [1.0, 1.0, 1.0],
            1.73,
        ));

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw(ui, &mut state);
            });
        });

        assert_eq!(state.project.annotations.len(), 1);
        assert_eq!(state.project.measurements.len(), 1);
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
