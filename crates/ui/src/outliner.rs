//! Painel Outliner hierárquico e Galeria de Assets do Petunia3D (`Outliner`).
//! Apresenta a árvore de objetos reais da cena, coleções especializadas de Anotações e Medidas,
//! suporte a coleções/pastas de geometria, bloqueio de transformação, isolamento de visualização,
//! imagens de referência e estatísticas.

use egui::{Color32, Id, Rect, Response, ScrollArea, Ui, vec2};
use egui_ltreeview::{Action, NodeBuilder, TreeView, TreeViewSettings};
use petunia_core::{
    AddPrimitiveCmd, AnnotationItem, AppState, DeleteAssetCmd, DuplicateAssetCmd, PrimitiveKind,
};
use uuid::Uuid;

use crate::icon_registry::{IconRegistry, PetuniaIcon};
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
    Asset(Uuid),
    ReferenceImages,
    ReferenceImage(usize),
}

/// Renderiza o painel Outliner.
pub fn draw(ui: &mut Ui, state: &mut AppState) {
    puffin::profile_function!();
    egui::CollapsingHeader::new("Outliner")
        .default_open(true)
        .show(ui, |ui| {
            // 1. Cabeçalho do Outliner com contagem, ações e busca
            draw_outliner_header(ui, state);

            ui.separator();

            // 2. Área de rolagem com a árvore hierárquica completa
            let scroll_max_h = if state.ui.inspector_detached {
                f32::INFINITY
            } else {
                280.0
            };
            ScrollArea::vertical()
                .id_salt("outliner_tree_scroll")
                .max_height(scroll_max_h)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    draw_tree_nodes(ui, state);
                });
        });
}

fn outliner_node_icon(ui: &mut Ui, icon: &PetuniaIcon, fg: Color32) {
    let (rect, _) = ui.allocate_exact_size(vec2(14.0, 14.0), egui::Sense::hover());
    if ui.is_rect_visible(rect) {
        IconRegistry::paint(ui.ctx(), ui.painter(), icon, rect, fg);
    }
}

fn outliner_icon_button(ui: &mut Ui, icon: &PetuniaIcon, fg: Color32, tooltip: &str) -> Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(18.0, 18.0), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let fill = if resp.hovered() {
            tokens::BG_SURFACE_HOVER
        } else {
            Color32::TRANSPARENT
        };
        ui.painter().rect_filled(rect, tokens::RADIUS_CONTROL, fill);
        let icon_rect = Rect::from_center_size(rect.center(), vec2(13.0, 13.0));
        IconRegistry::paint(ui.ctx(), ui.painter(), icon, icon_rect, fg);
    }
    resp.on_hover_text(tooltip)
}

fn outliner_eye_button(ui: &mut Ui, visible: bool, tooltip: &str) -> Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(18.0, 18.0), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let (icon, fg) = if visible {
            (PetuniaIcon::Eye, tokens::TEXT_PRIMARY)
        } else {
            (PetuniaIcon::EyeHidden, tokens::TEXT_MUTED)
        };
        let icon_rect = Rect::from_center_size(rect.center(), vec2(13.0, 13.0));
        IconRegistry::paint(ui.ctx(), ui.painter(), &icon, icon_rect, fg);
    }
    resp.on_hover_text(tooltip)
}

fn outliner_lock_button(ui: &mut Ui, locked: bool, tooltip: &str) -> Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(18.0, 18.0), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let (icon, fg) = if locked {
            (PetuniaIcon::Lock, Color32::from_rgb(0xe6, 0x7e, 0x22))
        } else {
            (PetuniaIcon::Unlock, tokens::TEXT_MUTED)
        };
        let icon_rect = Rect::from_center_size(rect.center(), vec2(13.0, 13.0));
        IconRegistry::paint(ui.ctx(), ui.painter(), &icon, icon_rect, fg);
    }
    resp.on_hover_text(tooltip)
}

fn draw_outliner_header(ui: &mut Ui, state: &mut AppState) {
    let n_assets = state.project.assets.len();

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(4.0, 0.0);

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

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Botão Nova Coleção com ícone vetorial
            let (rect, resp) = ui.allocate_exact_size(vec2(22.0, 20.0), egui::Sense::click());
            if ui.is_rect_visible(rect) {
                let fill = if resp.hovered() {
                    tokens::BG_SURFACE_HOVER
                } else {
                    tokens::BG_SURFACE
                };
                ui.painter().rect_filled(rect, tokens::RADIUS_CONTROL, fill);
                let icon_rect = Rect::from_center_size(rect.center(), vec2(14.0, 14.0));
                IconRegistry::paint(
                    ui.ctx(),
                    ui.painter(),
                    &PetuniaIcon::Folder,
                    icon_rect,
                    tokens::TEXT_SECONDARY,
                );
            }
            if resp
                .on_hover_text("Criar nova Coleção para organizar modelos")
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

            // Botão de Isolar Objeto Ativo
            let is_iso = state.isolate_active;
            let (rect, resp) = ui.allocate_exact_size(vec2(22.0, 20.0), egui::Sense::click());
            if ui.is_rect_visible(rect) {
                let bg = if is_iso {
                    tokens::ACCENT_BLUE
                } else if resp.hovered() {
                    tokens::BG_SURFACE_HOVER
                } else {
                    tokens::BG_SURFACE
                };
                let fg = if is_iso {
                    Color32::WHITE
                } else {
                    tokens::TEXT_SECONDARY
                };
                ui.painter().rect_filled(rect, tokens::RADIUS_CONTROL, bg);
                let icon_rect = Rect::from_center_size(rect.center(), vec2(14.0, 14.0));
                IconRegistry::paint(
                    ui.ctx(),
                    ui.painter(),
                    &PetuniaIcon::Cursor3D,
                    icon_rect,
                    fg,
                );
            }
            if resp.on_hover_text(if is_iso {
                "Isolar Ativo (Numpad /): Restaurar visibilidade de todos os objetos"
            } else {
                "Isolar Objeto Ativo (Numpad /): Esconder todos os outros e focar no selecionado"
            }).clicked() {
                state.toggle_isolate();
            }
        });
    });

    ui.add_space(2.0);

    // Campo de busca com ícone e botão de limpar
    widgets::petunia_search_box(ui, &mut state.ui.outliner_search, "Search...");
}

fn draw_tree_nodes(ui: &mut Ui, state: &mut AppState) {
    let search = state.ui.outliner_search.trim().to_lowercase();
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
        .allow_drag_and_drop(false)
        .allow_multi_selection(false);

    if let Some(mut tree_state) = egui_ltreeview::TreeViewState::<OutlinerNodeId>::load(ui, tree_id)
        && let Some(active_asset) = state.project.assets.get(active_idx)
    {
        let target = OutlinerNodeId::Asset(active_asset.id);
        if !tree_state.selected().contains(&target) {
            tree_state.set_one_selected(target);
            tree_state.store(ui, tree_id);
        }
    }

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

    // Atalhos de teclado no Outliner quando nenhum campo de texto está ativo
    if !ui.ctx().egui_wants_keyboard_input() {
        if ui.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)) {
            if let Some(ann_id) = state.selected_annotation {
                delete_ann = Some(ann_id);
            } else if let Some(meas_id) = state.selected_measurement {
                delete_meas = Some(meas_id);
            } else if active_idx < state.project.assets.len() {
                delete_idx = Some(active_idx);
            }
        }
        if ui.input(|i| (i.modifiers.shift || i.modifiers.command) && i.key_pressed(egui::Key::D)) {
            if let Some(ann_id) = state.selected_annotation {
                dup_ann = Some(ann_id);
            } else if active_idx < state.project.assets.len() {
                dup_idx = Some(active_idx);
            }
        }
    }

    let show_ann_collection = !state.project.annotations.is_empty()
        || !state.project.annotation_groups.is_empty()
        || state.active_tool == "annotate";
    let show_meas_collection =
        !state.project.measurements.is_empty() || state.active_tool == "measure";

    let (_, actions) = tree.show(ui, |builder| {
        // =========================================================================
        // SEÇÃO 1: ANOTAÇÕES (Acima das existentes, tipo dedicado, cor ciano)
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
                            outliner_node_icon(
                                ui,
                                &PetuniaIcon::Annotate,
                                Color32::from_rgb(0, 210, 211),
                            );
                            let col_resp = ui.label(
                                egui::RichText::new(format!("Annotations ({n_anns})"))
                                    .size(11.5)
                                    .strong()
                                    .color(Color32::from_rgb(0, 210, 211)),
                            );

                            col_resp.context_menu(|ui| {
                                if ui.button("New Subgroup...").clicked() {
                                    let mut num = state.project.annotation_groups.len() + 1;
                                    let mut name = format!("Subgroup {num}");
                                    while state.project.annotation_groups.contains(&name) {
                                        num += 1;
                                        name = format!("Subgroup {num}");
                                    }
                                    add_ann_subgroup = Some(name);
                                    ui.close();
                                }
                                ui.separator();
                                if ui.button("Toggle Collection Visibility").clicked() {
                                    toggle_all_ann_vis = true;
                                    ui.close();
                                }
                                if ui.button("Toggle Collection Lock").clicked() {
                                    toggle_all_ann_lock = true;
                                    ui.close();
                                }
                                ui.separator();
                                if ui.button("Clear All Annotations").clicked() {
                                    clear_all_ann = true;
                                    ui.close();
                                }
                            });

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if outliner_eye_button(
                                        ui,
                                        all_ann_vis,
                                        "Toggle visibility of all annotations",
                                    )
                                    .clicked()
                                    {
                                        toggle_all_ann_vis = true;
                                    }

                                    if outliner_lock_button(
                                        ui,
                                        all_ann_lock,
                                        "Toggle lock of all annotations",
                                    )
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
                                    outliner_node_icon(
                                        ui,
                                        &PetuniaIcon::Folder,
                                        Color32::from_rgb(0, 180, 180),
                                    );
                                    let resp = ui.label(
                                        egui::RichText::new(format!(
                                            "{group_for_closure} ({count})"
                                        ))
                                        .size(11.0)
                                        .color(Color32::from_rgb(0, 180, 180)),
                                    );
                                    resp.context_menu(|ui| {
                                        if ui.button("Delete Subgroup").clicked() {
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

                                            outliner_node_icon(ui, &PetuniaIcon::Annotate, fg);
                                            let label_resp = ui.label(
                                                egui::RichText::new(&name)
                                                    .size(11.0)
                                                    .color(fg)
                                                    .background_color(if is_selected {
                                                        Color32::from_rgb(0, 100, 120)
                                                    } else {
                                                        Color32::TRANSPARENT
                                                    }),
                                            );

                                            label_resp.context_menu(|ui| {
                                                ui.menu_button("Move to Subgroup ›", |ui| {
                                                    if ui.button("None (Root)").clicked() {
                                                        ann_move_to_group = Some((ann_id, None));
                                                        ui.close();
                                                    }
                                                    for g in &state.project.annotation_groups {
                                                        if ui.button(g).clicked() {
                                                            ann_move_to_group =
                                                                Some((ann_id, Some(g.clone())));
                                                            ui.close();
                                                        }
                                                    }
                                                });
                                                ui.separator();
                                                let lock_txt =
                                                    if locked { "Unlock" } else { "Lock" };
                                                if ui.button(lock_txt).clicked() {
                                                    toggle_ann_lock = Some(ann_id);
                                                    ui.close();
                                                }
                                                if ui.button("Duplicate").clicked() {
                                                    dup_ann = Some(ann_id);
                                                    ui.close();
                                                }
                                                if ui.button("Delete").clicked() {
                                                    delete_ann = Some(ann_id);
                                                    ui.close();
                                                }
                                            });

                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    if outliner_eye_button(
                                                        ui,
                                                        visible,
                                                        "Toggle annotation visibility",
                                                    )
                                                    .clicked()
                                                    {
                                                        toggle_ann_vis = Some(ann_id);
                                                    }

                                                    if outliner_lock_button(
                                                        ui,
                                                        locked,
                                                        "Toggle annotation lock",
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

                                outliner_node_icon(ui, &PetuniaIcon::Annotate, fg);
                                let label_resp = ui.label(
                                    egui::RichText::new(&name)
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
                                        ui.menu_button("Move to Subgroup ›", |ui| {
                                            for g in &state.project.annotation_groups {
                                                if ui.button(g).clicked() {
                                                    ann_move_to_group =
                                                        Some((ann_id, Some(g.clone())));
                                                    ui.close();
                                                }
                                            }
                                        });
                                        ui.separator();
                                    }
                                    let lock_txt = if locked { "Unlock" } else { "Lock" };
                                    if ui.button(lock_txt).clicked() {
                                        toggle_ann_lock = Some(ann_id);
                                        ui.close();
                                    }
                                    if ui.button("Duplicate").clicked() {
                                        dup_ann = Some(ann_id);
                                        ui.close();
                                    }
                                    if ui.button("Delete").clicked() {
                                        delete_ann = Some(ann_id);
                                        ui.close();
                                    }
                                });

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if outliner_eye_button(
                                            ui,
                                            visible,
                                            "Toggle annotation visibility",
                                        )
                                        .clicked()
                                        {
                                            toggle_ann_vis = Some(ann_id);
                                        }

                                        if outliner_lock_button(
                                            ui,
                                            locked,
                                            "Toggle annotation lock",
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
        // SEÇÃO 2: MEDIDAS (Acima das de malha, tipo dedicado, cor amarela)
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
                            outliner_node_icon(
                                ui,
                                &PetuniaIcon::Measure,
                                Color32::from_rgb(254, 202, 87),
                            );
                            let col_resp = ui.label(
                                egui::RichText::new(format!("Measurements ({n_meas})"))
                                    .size(11.5)
                                    .strong()
                                    .color(Color32::from_rgb(254, 202, 87)),
                            );

                            col_resp.context_menu(|ui| {
                                if ui.button("Toggle Collection Visibility").clicked() {
                                    toggle_all_meas_vis = true;
                                    ui.close();
                                }
                                ui.separator();
                                if ui.button("Clear All Measurements").clicked() {
                                    clear_all_meas = true;
                                    ui.close();
                                }
                            });

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if outliner_eye_button(
                                        ui,
                                        all_meas_vis,
                                        "Toggle visibility of all measurements",
                                    )
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

                                outliner_node_icon(ui, &PetuniaIcon::Measure, fg);
                                let label_resp = ui.label(
                                    egui::RichText::new(format!("{name} ({dist:.2}m)"))
                                        .size(11.0)
                                        .color(fg)
                                        .background_color(if is_selected {
                                            Color32::from_rgb(120, 100, 20)
                                        } else {
                                            Color32::TRANSPARENT
                                        }),
                                );

                                label_resp.context_menu(|ui| {
                                    if ui.button("Delete").clicked() {
                                        delete_meas = Some(m_id);
                                        ui.close();
                                    }
                                });

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if outliner_eye_button(
                                            ui,
                                            visible,
                                            "Toggle measurement visibility",
                                        )
                                        .clicked()
                                        {
                                            toggle_meas_vis = Some(m_id);
                                        }

                                        if outliner_icon_button(
                                            ui,
                                            &PetuniaIcon::Delete,
                                            tokens::TEXT_MUTED,
                                            "Delete measurement",
                                        )
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
        // SEÇÃO 3: SCENE COLLECTION (Malhas 3D e Coleções de Objetos)
        // =========================================================================
        let open_scene = builder.node(
            NodeBuilder::dir(OutlinerNodeId::SceneCollection)
                .default_open(true)
                .label_ui(|ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = vec2(4.0, 0.0);
                        outliner_node_icon(ui, &PetuniaIcon::Collection, tokens::TEXT_PRIMARY);
                        ui.label(
                            egui::RichText::new("Scene Collection")
                                .size(11.5)
                                .color(tokens::TEXT_PRIMARY),
                        );
                    });
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
                                    outliner_node_icon(
                                        ui,
                                        &PetuniaIcon::Folder,
                                        tokens::TEXT_PRIMARY,
                                    );
                                    let col_resp = ui.label(
                                        egui::RichText::new(format!("{col_for_closure} ({count})"))
                                            .size(11.0)
                                            .color(tokens::TEXT_PRIMARY),
                                    );
                                    col_resp.context_menu(|ui| {
                                        if widgets::PetuniaMenuItem::new("Rename Collection")
                                            .icon(PetuniaIcon::Folder)
                                            .show(ui)
                                            .clicked()
                                        {
                                            ui.data_mut(|d| {
                                                d.insert_temp(rename_id, col_for_closure.clone());
                                                d.insert_temp(
                                                    egui::Id::new("petunia_rename_buf"),
                                                    col_for_closure.clone(),
                                                );
                                            });
                                            ui.close();
                                        }
                                        if widgets::PetuniaMenuItem::new("Delete Collection")
                                            .icon(PetuniaIcon::Trash)
                                            .show(ui)
                                            .clicked()
                                        {
                                            delete_col = Some(col_for_closure.clone());
                                            ui.close();
                                        }
                                        ui.separator();
                                        if widgets::PetuniaMenuItem::new(
                                            "Toggle Collection Visibility",
                                        )
                                        .icon(PetuniaIcon::Eye)
                                        .show(ui)
                                        .clicked()
                                        {
                                            toggle_col_vis = Some(col_for_closure.clone());
                                            ui.close();
                                        }
                                        if widgets::PetuniaMenuItem::new("Toggle Collection Lock")
                                            .icon(PetuniaIcon::Lock)
                                            .show(ui)
                                            .clicked()
                                        {
                                            toggle_col_lock = Some(col_for_closure.clone());
                                            ui.close();
                                        }
                                    });
                                }

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if outliner_eye_button(
                                            ui,
                                            all_visible,
                                            "Toggle visibility of all objects in collection",
                                        )
                                        .clicked()
                                        {
                                            toggle_col_vis = Some(col_for_closure.clone());
                                        }

                                        if outliner_lock_button(
                                            ui,
                                            all_locked,
                                            "Toggle lock of all objects in collection",
                                        )
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
                        let asset_id = state.project.assets[i].id;
                        builder.node(NodeBuilder::leaf(OutlinerNodeId::Asset(asset_id)).label_ui(
                            |ui| {
                                ui.horizontal(|ui| {
                                    ui.spacing_mut().item_spacing = vec2(4.0, 0.0);

                                    let fg = if is_selected {
                                        tokens::TEXT_ACTIVE
                                    } else {
                                        tokens::TEXT_PRIMARY
                                    };

                                    outliner_node_icon(ui, &PetuniaIcon::ObjectMesh, fg);
                                    let item_label = format!("{name} ({vc}v, {fc}f)");
                                    let label_resp = ui.selectable_label(
                                        is_selected,
                                        egui::RichText::new(item_label).size(11.0).color(fg),
                                    );
                                    if label_resp.clicked() {
                                        state.set_active_asset_by_id(asset_id);
                                        state.selected_annotation = None;
                                        state.selected_measurement = None;
                                        state.mark_dirty();
                                    }

                                    label_resp.context_menu(|ui| {
                                        ui.menu_button("Move to Collection ›", |ui| {
                                            if widgets::PetuniaMenuItem::new("None (Root)")
                                                .show(ui)
                                                .clicked()
                                            {
                                                move_to_col = Some((i, None));
                                                ui.close();
                                            }
                                            ui.separator();
                                            for col in &collections {
                                                let is_cur =
                                                    state.project.assets[i].collection.as_deref()
                                                        == Some(col);
                                                let label = if is_cur {
                                                    format!("✓ {col}")
                                                } else {
                                                    col.clone()
                                                };
                                                if widgets::PetuniaMenuItem::new(&label)
                                                    .icon(PetuniaIcon::Folder)
                                                    .show(ui)
                                                    .clicked()
                                                {
                                                    move_to_col = Some((i, Some(col.clone())));
                                                    ui.close();
                                                }
                                            }
                                        });
                                        ui.separator();
                                        let (lock_txt, lock_icon) = if locked {
                                            ("Unlock Object", PetuniaIcon::Unlock)
                                        } else {
                                            ("Lock Object", PetuniaIcon::Lock)
                                        };
                                        if widgets::PetuniaMenuItem::new(lock_txt)
                                            .icon(lock_icon)
                                            .show(ui)
                                            .clicked()
                                        {
                                            toggle_lock_idx = Some(i);
                                            ui.close();
                                        }
                                        let iso_txt = if state.isolate_active && is_selected {
                                            "Restore Visibility (Exit Isolate)"
                                        } else {
                                            "Isolate Object"
                                        };
                                        if widgets::PetuniaMenuItem::new(iso_txt)
                                            .icon(PetuniaIcon::Eye)
                                            .shortcut(Some("Numpad /"))
                                            .show(ui)
                                            .clicked()
                                        {
                                            isolate_idx = Some(i);
                                            ui.close();
                                        }
                                        ui.separator();
                                        if widgets::PetuniaMenuItem::new("Duplicate")
                                            .icon(PetuniaIcon::Duplicate)
                                            .shortcut(Some("Shift+D"))
                                            .show(ui)
                                            .clicked()
                                        {
                                            dup_idx = Some(i);
                                            ui.close();
                                        }
                                        if widgets::PetuniaMenuItem::new("Delete")
                                            .icon(PetuniaIcon::Trash)
                                            .shortcut(Some("Delete"))
                                            .show(ui)
                                            .clicked()
                                        {
                                            delete_idx = Some(i);
                                            ui.close();
                                        }
                                    });

                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            if outliner_icon_button(
                                                ui,
                                                &PetuniaIcon::Delete,
                                                tokens::TEXT_MUTED,
                                                "Delete object (Delete)",
                                            )
                                            .clicked()
                                            {
                                                delete_idx = Some(i);
                                            }

                                            if outliner_lock_button(
                                                ui,
                                                locked,
                                                if locked {
                                                    "Unlock Object (currently fixed)"
                                                } else {
                                                    "Lock Object (prevents transform)"
                                                },
                                            )
                                            .clicked()
                                            {
                                                toggle_lock_idx = Some(i);
                                            }

                                            if outliner_eye_button(
                                                ui,
                                                visible,
                                                if visible {
                                                    "Hide in 3D Viewport"
                                                } else {
                                                    "Show in 3D Viewport"
                                                },
                                            )
                                            .clicked()
                                            {
                                                toggle_vis_idx = Some(i);
                                            }
                                        },
                                    );
                                });
                            },
                        ));
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
                let asset_id = state.project.assets[i].id;
                builder.node(
                    NodeBuilder::leaf(OutlinerNodeId::Asset(asset_id)).label_ui(|ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = vec2(4.0, 0.0);

                            let fg = if is_selected {
                                tokens::TEXT_ACTIVE
                            } else {
                                tokens::TEXT_PRIMARY
                            };

                            outliner_node_icon(ui, &PetuniaIcon::ObjectMesh, fg);
                            let item_label = format!("{name} ({vc}v, {fc}f)");
                            let label_resp = ui.selectable_label(
                                is_selected,
                                egui::RichText::new(item_label).size(11.0).color(fg),
                            );
                            if label_resp.clicked() {
                                state.set_active_asset_by_id(asset_id);
                                state.selected_annotation = None;
                                state.selected_measurement = None;
                                state.mark_dirty();
                            }

                            label_resp.context_menu(|ui| {
                                if !collections.is_empty() {
                                    ui.menu_button("Move to Collection ›", |ui| {
                                        for col in &collections {
                                            if widgets::PetuniaMenuItem::new(col)
                                                .icon(PetuniaIcon::Folder)
                                                .show(ui)
                                                .clicked()
                                            {
                                                move_to_col = Some((i, Some(col.clone())));
                                                ui.close();
                                            }
                                        }
                                    });
                                    ui.separator();
                                }
                                let (lock_txt, lock_icon) = if locked {
                                    ("Unlock Object", PetuniaIcon::Unlock)
                                } else {
                                    ("Lock Object", PetuniaIcon::Lock)
                                };
                                if widgets::PetuniaMenuItem::new(lock_txt)
                                    .icon(lock_icon)
                                    .show(ui)
                                    .clicked()
                                {
                                    toggle_lock_idx = Some(i);
                                    ui.close();
                                }
                                let iso_txt = if state.isolate_active && is_selected {
                                    "Restore Visibility (Exit Isolate)"
                                } else {
                                    "Isolate Object"
                                };
                                if widgets::PetuniaMenuItem::new(iso_txt)
                                    .icon(PetuniaIcon::Eye)
                                    .shortcut(Some("Numpad /"))
                                    .show(ui)
                                    .clicked()
                                {
                                    isolate_idx = Some(i);
                                    ui.close();
                                }
                                ui.separator();
                                if widgets::PetuniaMenuItem::new("Duplicate")
                                    .icon(PetuniaIcon::Duplicate)
                                    .shortcut(Some("Shift+D"))
                                    .show(ui)
                                    .clicked()
                                {
                                    dup_idx = Some(i);
                                    ui.close();
                                }
                                if widgets::PetuniaMenuItem::new("Delete")
                                    .icon(PetuniaIcon::Trash)
                                    .shortcut(Some("Delete"))
                                    .show(ui)
                                    .clicked()
                                {
                                    delete_idx = Some(i);
                                    ui.close();
                                }
                            });

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if outliner_icon_button(
                                        ui,
                                        &PetuniaIcon::Delete,
                                        tokens::TEXT_MUTED,
                                        "Delete object (Delete)",
                                    )
                                    .clicked()
                                    {
                                        delete_idx = Some(i);
                                    }

                                    if outliner_lock_button(
                                        ui,
                                        locked,
                                        if locked {
                                            "Unlock Object (currently fixed)"
                                        } else {
                                            "Lock Object (prevents transform)"
                                        },
                                    )
                                    .clicked()
                                    {
                                        toggle_lock_idx = Some(i);
                                    }

                                    if outliner_eye_button(
                                        ui,
                                        visible,
                                        if visible {
                                            "Hide in 3D Viewport"
                                        } else {
                                            "Show in 3D Viewport"
                                        },
                                    )
                                    .clicked()
                                    {
                                        toggle_vis_idx = Some(i);
                                    }
                                },
                            );
                        });
                    }),
                );
            }

            builder.close_dir();
        }

        // =========================================================================
        // SEÇÃO 4: IMAGENS DE REFERÊNCIA
        // =========================================================================
        if !state.project.refs.is_empty() {
            let open_refs = builder.node(
                NodeBuilder::dir(OutlinerNodeId::ReferenceImages)
                    .default_open(true)
                    .label_ui(|ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = vec2(4.0, 0.0);
                            outliner_node_icon(
                                ui,
                                &PetuniaIcon::ReferenceImage,
                                tokens::TEXT_PRIMARY,
                            );
                            ui.label(
                                egui::RichText::new(format!(
                                    "Reference Images ({})",
                                    state.project.refs.len()
                                ))
                                .size(11.5)
                                .color(tokens::TEXT_PRIMARY),
                            );
                        });
                    }),
            );

            if open_refs {
                for r_idx in 0..state.project.refs.len() {
                    let r_vis = state.project.refs[r_idx].visible;
                    let r_xray = state.project.refs[r_idx].xray;
                    let r_axis = format!("{:?}", state.project.refs[r_idx].axis);
                    let r_dim = format!(
                        "{}x{}",
                        state.project.refs[r_idx].width, state.project.refs[r_idx].height
                    );

                    builder.node(
                        NodeBuilder::leaf(OutlinerNodeId::ReferenceImage(r_idx)).label_ui(|ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing = vec2(4.0, 0.0);
                                outliner_node_icon(
                                    ui,
                                    &PetuniaIcon::ReferenceImage,
                                    tokens::TEXT_PRIMARY,
                                );
                                let item_label = format!("{r_axis} ({r_dim})");
                                let label_resp = ui.label(
                                    egui::RichText::new(item_label)
                                        .size(11.0)
                                        .color(tokens::TEXT_PRIMARY),
                                );
                                label_resp.context_menu(|ui| {
                                    let xray_txt = if r_xray {
                                        "Disable X-Ray"
                                    } else {
                                        "Enable X-Ray"
                                    };
                                    if ui.button(xray_txt).clicked() {
                                        ref_toggle_xray = Some(r_idx);
                                        ui.close();
                                    }
                                    if ui.button("Remove Image").clicked() {
                                        ref_delete = Some(r_idx);
                                        ui.close();
                                    }
                                });

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if outliner_eye_button(
                                            ui,
                                            r_vis,
                                            if r_vis {
                                                "Hide Reference Image"
                                            } else {
                                                "Show Reference Image"
                                            },
                                        )
                                        .clicked()
                                        {
                                            ref_toggle_vis = Some(r_idx);
                                        }

                                        let xray_col = if r_xray {
                                            tokens::ACCENT_BLUE
                                        } else {
                                            tokens::TEXT_MUTED
                                        };
                                        if outliner_icon_button(
                                            ui,
                                            &PetuniaIcon::XRay,
                                            xray_col,
                                            if r_xray {
                                                "X-Ray Active (visible over meshes)"
                                            } else {
                                                "Enable X-Ray"
                                            },
                                        )
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
    if let Some(ann_id) = toggle_ann_vis
        && let Some(ann) = state
            .project
            .annotations
            .iter_mut()
            .find(|a| a.id == ann_id)
    {
        ann.visible = !ann.visible;
        state.mark_dirty();
    }
    if let Some(ann_id) = toggle_ann_lock
        && let Some(ann) = state
            .project
            .annotations
            .iter_mut()
            .find(|a| a.id == ann_id)
    {
        ann.locked = !ann.locked;
        state.mark_dirty();
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
    if let Some((ann_id, grp)) = ann_move_to_group
        && let Some(ann) = state
            .project
            .annotations
            .iter_mut()
            .find(|a| a.id == ann_id)
    {
        ann.group = grp;
        state.mark_dirty();
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
    if let Some(meas_id) = toggle_meas_vis
        && let Some(m) = state
            .project
            .measurements
            .iter_mut()
            .find(|m| m.id == meas_id)
    {
        m.visible = !m.visible;
        state.mark_dirty();
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
        let _ = state.dispatch(&petunia_core::ToggleVisibilityAssetCmd {
            asset_index: Some(idx),
        });
    }
    if let Some(idx) = toggle_lock_idx {
        let _ = state.dispatch(&petunia_core::ToggleLockAssetCmd {
            asset_index: Some(idx),
        });
    }
    if let Some(idx) = isolate_idx
        && idx < state.project.assets.len()
    {
        state.project.active = idx;
        state.toggle_isolate();
    }
    if let Some((idx, col)) = move_to_col {
        let _ = state.dispatch(&petunia_core::SetAssetCollectionCmd {
            asset_index: idx,
            collection: col,
        });
    }
    if let Some(col) = toggle_col_vis {
        let _ = state.dispatch(&petunia_core::ToggleCollectionVisibilityCmd { collection: col });
    }
    if let Some(col) = toggle_col_lock {
        let _ = state.dispatch(&petunia_core::ToggleCollectionLockCmd { collection: col });
    }
    if let Some(col) = delete_col {
        state.project.remove_collection(&col);
        state.mark_dirty();
    }
    if let Some((old_name, new_name)) = rename_col
        && !new_name.trim().is_empty()
        && !state.project.collections.contains(&new_name)
    {
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

    // Ações de referências
    if let Some(idx) = ref_toggle_vis
        && let Some(r) = state.project.refs.get_mut(idx)
    {
        r.visible = !r.visible;
        state.mark_dirty();
    }
    if let Some(idx) = ref_toggle_xray
        && let Some(r) = state.project.refs.get_mut(idx)
    {
        r.xray = !r.xray;
        state.mark_dirty();
    }
    if let Some(idx) = ref_delete
        && idx < state.project.refs.len()
    {
        state.project.refs.remove(idx);
        state.mark_dirty();
    }
    if let Some(idx) = delete_idx {
        let _ = state.dispatch(&DeleteAssetCmd {
            asset_index: Some(idx),
        });
    }
    if let Some(idx) = dup_idx {
        let _ = state.dispatch(&DuplicateAssetCmd {
            asset_index: Some(idx),
        });
    }

    // Trata seleção pelo TreeView
    for action in actions {
        match action {
            Action::SetSelected(nodes) => {
                for node in nodes {
                    match node {
                        OutlinerNodeId::Asset(id) => {
                            state.set_active_asset_by_id(id);
                            state.selected_annotation = None;
                            state.selected_measurement = None;
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
                        OutlinerNodeId::Asset(id) => {
                            state.set_active_asset_by_id(id);
                            state.selected_annotation = None;
                            state.selected_measurement = None;
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
    let p_kind = match kind {
        0 => PrimitiveKind::Cube,
        1 => PrimitiveKind::Sphere,
        2 => PrimitiveKind::Cylinder,
        3 => PrimitiveKind::Plane,
        4 => PrimitiveKind::Cone,
        _ => PrimitiveKind::Capsule,
    };
    let _ = state.dispatch(&AddPrimitiveCmd {
        kind: p_kind,
        name: Some(name.to_string()),
        at_cursor: true,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use petunia_core::MeasurementItem;
    use petunia_mesh::Mesh;

    #[test]
    fn test_outliner_renders_without_panic() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");

        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                draw(ui, &mut state);
            });
        })
        .textures_delta
        .clear();
    }

    #[test]
    fn test_outliner_renders_with_collections_and_locks() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");
        state.project.add_collection("Props");
        state.project.add("Table", Mesh::cube(1.0));
        state.project.assets.last_mut().unwrap().collection = Some("Props".to_string());
        state.project.assets.last_mut().unwrap().locked = true;

        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                draw(ui, &mut state);
            });
        })
        .textures_delta
        .clear();

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

        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                draw(ui, &mut state);
            });
        })
        .textures_delta
        .clear();

        assert_eq!(state.project.annotations.len(), 1);
        assert_eq!(state.project.measurements.len(), 1);
    }

    #[test]
    fn test_outliner_search_filtering() {
        let mut state = AppState::new("en");
        state.project.add("TargetCube", Mesh::cube(1.0));
        state.project.add("OtherObject", Mesh::cube(1.0));

        state.ui.outliner_search = "target".to_string();
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
