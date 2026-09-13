//! Painel de Propriedades do Petunia3D (`Properties Panel`).
//! Contém a barra vertical de abas canônicas (Tool, Render, Output, Object, Modifiers, Data, Material)
//! e formulários sanfonados com fidelidade estética ao Blender.svg.

use egui::{vec2, Color32, Context, ScrollArea, Ui};
use petunia_core::{
    AppState, DeleteSelectionCmd, DuplicateSelectionCmd, ModuleRegistry, Workspace,
};
use petunia_module_model::ToolRegistry;
use uuid::Uuid;

use crate::icon_registry::{IconRegistry, PetuniaIcon};
use crate::tokens;
use crate::tool_fields;
use crate::widgets::{self, PetuniaPropertyTabButton};

/// Renderiza o painel de propriedades completo com abas e seções sanfonadas.
pub fn draw(
    ctx: &Context,
    ui: &mut Ui,
    state: &mut AppState,
    tools: &ToolRegistry,
    registry: &mut ModuleRegistry,
) {
    // 1. Barra de abas de propriedades (Tool, Render, Object, Modifiers, etc.)
    draw_property_tabs(ui, state);

    ui.separator();

    // 2. Área principal com os controles da aba selecionada
    ScrollArea::vertical()
        .id_salt("properties_content_scroll")
        .show(ui, |ui| {
            let fields = state.workspace == Workspace::Model && tool_fields::draw(ui, state);
            if fields && state.active_tool == "transform" {
                ui.add_enabled_ui(!state.is_interacting(), |ui| {
                    ui.horizontal_wrapped(|ui| {
                        if ui.button(state.t("actions.duplicate")).clicked() {
                            let _ = state.dispatch(&DuplicateSelectionCmd);
                        }
                        if ui.button(state.t("actions.delete")).clicked() {
                            let _ = state.dispatch(&DeleteSelectionCmd);
                        }
                    });
                });
            }

            ui.add_enabled_ui(!state.is_interacting(), |ui| {
                match state.workspace {
                    Workspace::Model => draw_active_tab_content(ctx, ui, state, tools, fields),
                    Workspace::Paint => {
                        if let Some(module) = registry.get_mut("paint") {
                            if let Some(paint) = module
                                .as_any_mut()
                                .downcast_mut::<petunia_module_paint::PaintModule>(
                            ) {
                                paint.ui(ctx, ui, state);
                            }
                        }
                    }
                    Workspace::Uv => {
                        if let Some(module) = registry.get_mut("uv") {
                            if let Some(uv) = module
                                .as_any_mut()
                                .downcast_mut::<petunia_module_uv::UvModule>()
                            {
                                uv.ui(ctx, ui, state);
                            }
                        }
                    }
                    Workspace::Animate => {
                        egui::CollapsingHeader::new("Animation Properties")
                            .default_open(true)
                            .show(ui, |ui| {
                                ui.label(format!("Current Frame: {}", state.ui.timeline_frame));
                                ui.label(format!(
                                    "Range: {}..={}",
                                    state.ui.timeline_start, state.ui.timeline_end
                                ));
                            });
                    }
                }

                if state.ui.show_help {
                    egui::CollapsingHeader::new(state.t("ui.help"))
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.label(state.t("help.body"));
                        });
                }
            });
        });
}

fn draw_property_tabs(ui: &mut Ui, state: &mut AppState) {
    let tabs: [(&str, PetuniaIcon, &str); 5] = [
        ("tool", PetuniaIcon::PropTool, "Active Tool & Settings"),
        (
            "object",
            PetuniaIcon::PropObject,
            "Object Transform & Properties",
        ),
        (
            "modifiers",
            PetuniaIcon::PropModifiers,
            "Modifiers & Geometry Tools",
        ),
        (
            "data",
            PetuniaIcon::PropData,
            "Mesh Data & Reference Images",
        ),
        (
            "material",
            PetuniaIcon::PropMaterial,
            "Material & Surface Color",
        ),
    ];

    egui::Frame::new()
        .fill(tokens::BG_PANEL_HEADER)
        .corner_radius(tokens::RADIUS_CONTAINER)
        .inner_margin(egui::Margin::symmetric(4, 3))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = vec2(4.0, 0.0);
                for (tab_id, icon, hint) in tabs {
                    let is_active = state.ui.properties_tab == tab_id;
                    if PetuniaPropertyTabButton::new(icon, is_active)
                        .accent_color(tokens::ACCENT_BLUE)
                        .tooltip(hint)
                        .show(ui)
                        .clicked()
                    {
                        state.ui.properties_tab = tab_id.to_string();
                        state.mark_dirty();
                    }
                }
            });
        });
}

fn draw_active_tab_content(
    ctx: &Context,
    ui: &mut Ui,
    state: &mut AppState,
    tools: &ToolRegistry,
    fields: bool,
) {
    match state.ui.properties_tab.as_str() {
        "tool" => draw_tab_tool(ctx, ui, state, tools, fields),
        "modifiers" => draw_tab_modifiers(ui, state),
        "data" => draw_tab_data(ui, state),
        "material" => draw_tab_material(ui, state),
        _ => draw_tab_object(ui, state),
    }
}

fn draw_tab_tool(
    ctx: &Context,
    ui: &mut Ui,
    state: &mut AppState,
    tools: &ToolRegistry,
    fields: bool,
) {
    if !fields {
        let active_id = state.active_tool.clone();
        egui::CollapsingHeader::new(format!("Active Tool: {active_id}"))
            .default_open(true)
            .show(ui, |ui| {
                if let Some(tool) = tools.get(&active_id) {
                    tool.ui(ctx, ui, state);
                } else {
                    ui.label(state.t("ui.no_tool"));
                }
            });
    }
}

fn draw_tab_object(ui: &mut Ui, state: &mut AppState) {
    if let Some(ann_id) = state.selected_annotation {
        draw_tab_annotation(ui, state, ann_id);
        return;
    }
    if let Some(meas_id) = state.selected_measurement {
        draw_tab_measurement(ui, state, meas_id);
        return;
    }

    if state.project.assets.is_empty() {
        ui.label("No active object in scene");
        return;
    }

    let idx = state.project.active.min(state.project.assets.len() - 1);

    // Identidade do Objeto
    let asset_name = state.project.assets[idx].name.clone();
    let visible = state.project.assets[idx].visible;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(6.0, 0.0);
        let (icon_rect, _) = ui.allocate_exact_size(vec2(16.0, 16.0), egui::Sense::hover());
        IconRegistry::paint(
            ui.ctx(),
            ui.painter(),
            &PetuniaIcon::ObjectMesh,
            icon_rect,
            tokens::ACCENT_BLUE,
        );
        ui.label(egui::RichText::new(&asset_name).strong().size(12.0));

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let (rect, resp) = ui.allocate_exact_size(vec2(20.0, 20.0), egui::Sense::click());
            if ui.is_rect_visible(rect) {
                let fill = if resp.hovered() {
                    tokens::BG_SURFACE_HOVER
                } else {
                    Color32::TRANSPARENT
                };
                ui.painter().rect_filled(rect, tokens::RADIUS_CONTROL, fill);
                let (icon, fg) = if visible {
                    (PetuniaIcon::Eye, tokens::TEXT_PRIMARY)
                } else {
                    (PetuniaIcon::EyeHidden, tokens::TEXT_MUTED)
                };
                let icon_rect = egui::Rect::from_center_size(rect.center(), vec2(14.0, 14.0));
                IconRegistry::paint(ui.ctx(), ui.painter(), &icon, icon_rect, fg);
            }
            if resp
                .on_hover_text(if visible {
                    "Hide Object in 3D Viewport"
                } else {
                    "Show Object in 3D Viewport"
                })
                .clicked()
            {
                if let Some(o) = state.project.assets.get_mut(idx) {
                    o.visible = !visible;
                }
                state.mark_dirty();
            }
        });
    });

    ui.add_space(4.0);

    // Seção de Transform (Location, Rotation, Scale)
    egui::CollapsingHeader::new("Transform")
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Name")
                        .size(11.0)
                        .color(tokens::TEXT_SECONDARY),
                );
                if let Some(o) = state.project.assets.get_mut(idx) {
                    ui.text_edit_singleline(&mut o.name);
                }
            });

            ui.add_space(4.0);

            // Location X, Y, Z com cores semânticas nos rótulos
            ui.label(egui::RichText::new("Location").strong().size(11.0));
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("X").color(tokens::AXIS_X).strong());
                ui.add(egui::DragValue::new(&mut state.transform_delta[0]).speed(0.05));
                ui.label(egui::RichText::new("Y").color(tokens::AXIS_Y).strong());
                ui.add(egui::DragValue::new(&mut state.transform_delta[1]).speed(0.05));
                ui.label(egui::RichText::new("Z").color(tokens::AXIS_Z).strong());
                ui.add(egui::DragValue::new(&mut state.transform_delta[2]).speed(0.05));
            });

            ui.add_space(2.0);

            // Rotation X, Y, Z (graus)
            ui.label(egui::RichText::new("Rotation").strong().size(11.0));
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("X").color(tokens::AXIS_X).strong());
                ui.add(
                    egui::DragValue::new(&mut state.transform_rotation[0])
                        .speed(1.0)
                        .suffix("°"),
                );
                ui.label(egui::RichText::new("Y").color(tokens::AXIS_Y).strong());
                ui.add(
                    egui::DragValue::new(&mut state.transform_rotation[1])
                        .speed(1.0)
                        .suffix("°"),
                );
                ui.label(egui::RichText::new("Z").color(tokens::AXIS_Z).strong());
                ui.add(
                    egui::DragValue::new(&mut state.transform_rotation[2])
                        .speed(1.0)
                        .suffix("°"),
                );
            });

            ui.add_space(2.0);

            // Scale
            ui.label(egui::RichText::new("Scale").strong().size(11.0));
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Uniform")
                        .size(11.0)
                        .color(tokens::TEXT_SECONDARY),
                );
                ui.add(
                    egui::DragValue::new(&mut state.transform_scale)
                        .speed(0.02)
                        .range(0.01..=100.0),
                );
            });

            ui.separator();

            ui.horizontal(|ui| {
                if widgets::petunia_action_button(
                    ui,
                    Some(PetuniaIcon::Duplicate),
                    "Duplicate · Shift+D",
                    false,
                )
                .clicked()
                {
                    let _ = state.dispatch(&DuplicateSelectionCmd);
                }

                if widgets::petunia_action_button(ui, Some(PetuniaIcon::Delete), "Delete · X", true)
                    .clicked()
                {
                    let _ = state.dispatch(&DeleteSelectionCmd);
                }
            });
        });
}

fn draw_tab_annotation(ui: &mut Ui, state: &mut AppState, ann_id: Uuid) {
    let Some(ann_idx) = state
        .project
        .annotations
        .iter()
        .position(|a| a.id == ann_id)
    else {
        ui.label(egui::RichText::new("Anotação selecionada não encontrada").italics());
        return;
    };

    let is_locked = state.project.annotations[ann_idx].locked || state.project.annotations_locked;

    // 1. Identidade da Anotação (Header ciano)
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(6.0, 0.0);
        let (icon_rect, _) = ui.allocate_exact_size(vec2(16.0, 16.0), egui::Sense::hover());
        IconRegistry::paint(
            ui.ctx(),
            ui.painter(),
            &PetuniaIcon::Annotate,
            icon_rect,
            Color32::from_rgb(0, 210, 211),
        );
        ui.label(
            egui::RichText::new("Annotation")
                .color(Color32::from_rgb(0, 210, 211))
                .strong(),
        );
    });

    let mut name = state.project.annotations[ann_idx].name.clone();
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Name")
                .size(11.0)
                .color(tokens::TEXT_SECONDARY),
        );
        let name_resp = ui.add(egui::TextEdit::singleline(&mut name).hint_text("Annotation Name"));
        if name_resp.changed() {
            state.project.annotations[ann_idx].name = name;
            state.mark_dirty();
        }
        if name_resp.lost_focus() {
            state.checkpoint("rename annotation");
        }
    });

    ui.add_space(4.0);

    // 2. Visibilidade e Bloqueio
    ui.horizontal(|ui| {
        let mut vis = state.project.annotations[ann_idx].visible;
        if ui.checkbox(&mut vis, "Visible").changed() {
            state.checkpoint("toggle annotation visibility");
            state.project.annotations[ann_idx].visible = vis;
            state.mark_dirty();
        }

        let mut locked = state.project.annotations[ann_idx].locked;
        if ui.checkbox(&mut locked, "Locked").changed() {
            state.checkpoint("toggle annotation lock");
            state.project.annotations[ann_idx].locked = locked;
            state.mark_dirty();
        }
    });

    // Subgrupo dentro da collection de Anotações
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Subgroup:")
                .size(11.0)
                .color(tokens::TEXT_MUTED),
        );
        let current_group = state.project.annotations[ann_idx].group.clone();
        let current_label = current_group.as_deref().unwrap_or("(Root / No subgroup)");

        let groups = state.project.annotation_groups.clone();
        egui::ComboBox::from_id_salt("annotation_group_selector")
            .selected_text(current_label)
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(current_group.is_none(), "(Root / No subgroup)")
                    .clicked()
                {
                    state.checkpoint("move annotation to root");
                    state.project.annotations[ann_idx].group = None;
                    state.mark_dirty();
                }
                for grp in groups {
                    let is_sel = current_group.as_deref() == Some(&grp);
                    if ui.selectable_label(is_sel, &grp).clicked() {
                        state.checkpoint("change annotation group");
                        state.project.annotations[ann_idx].group = Some(grp);
                        state.mark_dirty();
                    }
                }
            });
    });

    ui.add_space(4.0);

    // 3. Aparência do Traço
    egui::CollapsingHeader::new("Stroke Style")
        .default_open(true)
        .show(ui, |ui| {
            let ann = &state.project.annotations[ann_idx];
            let mut color = ann
                .strokes
                .first()
                .map(|s| s.color)
                .unwrap_or([0.0, 0.74, 0.83, 1.0]);
            let mut width = ann.strokes.first().map(|s| s.width).unwrap_or(2.0);

            let mut color_changed = false;
            let mut width_changed = false;

            ui.horizontal(|ui| {
                ui.label("Color:");
                let resp = ui.color_edit_button_rgba_unmultiplied(&mut color);
                if resp.changed() {
                    color_changed = true;
                }
                if resp.drag_stopped() {
                    state.checkpoint("change annotation color");
                }
            });

            ui.horizontal(|ui| {
                ui.label("Width:");
                let resp = ui.add(egui::Slider::new(&mut width, 0.5..=10.0).suffix(" px"));
                if resp.changed() {
                    width_changed = true;
                }
                if resp.drag_stopped() {
                    state.checkpoint("change annotation width");
                }
            });

            if color_changed {
                for s in &mut state.project.annotations[ann_idx].strokes {
                    s.color = color;
                }
                state.mark_dirty();
            }
            if width_changed {
                for s in &mut state.project.annotations[ann_idx].strokes {
                    s.width = width;
                }
                state.mark_dirty();
            }
        });

    ui.add_space(4.0);

    // 4. Seção de Transformação (Location, Rotation, Scale)
    egui::CollapsingHeader::new("Transform")
        .default_open(true)
        .show(ui, |ui| {
            if is_locked {
                ui.label(
                    egui::RichText::new("Annotation locked against transformations")
                        .italics()
                        .color(tokens::TEXT_MUTED),
                );
            }

            ui.add_enabled_ui(!is_locked, |ui| {
                let mut trans = state.project.annotations[ann_idx].translation;
                let mut rot = state.project.annotations[ann_idx].rotation;
                let mut scale = state.project.annotations[ann_idx].scale;

                let mut changed = false;
                let mut stopped = false;

                // Location X, Y, Z
                ui.label(egui::RichText::new("Location").strong().size(11.0));
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("X").color(tokens::AXIS_X).strong());
                    let r0 = ui.add(egui::DragValue::new(&mut trans[0]).speed(0.05));
                    ui.label(egui::RichText::new("Y").color(tokens::AXIS_Y).strong());
                    let r1 = ui.add(egui::DragValue::new(&mut trans[1]).speed(0.05));
                    ui.label(egui::RichText::new("Z").color(tokens::AXIS_Z).strong());
                    let r2 = ui.add(egui::DragValue::new(&mut trans[2]).speed(0.05));

                    if r0.changed() || r1.changed() || r2.changed() {
                        changed = true;
                    }
                    if r0.drag_stopped() || r1.drag_stopped() || r2.drag_stopped() {
                        stopped = true;
                    }
                });

                ui.add_space(2.0);

                // Rotation X, Y, Z (graus)
                ui.label(egui::RichText::new("Rotation").strong().size(11.0));
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("X").color(tokens::AXIS_X).strong());
                    let r0 = ui.add(egui::DragValue::new(&mut rot[0]).speed(1.0).suffix("°"));
                    ui.label(egui::RichText::new("Y").color(tokens::AXIS_Y).strong());
                    let r1 = ui.add(egui::DragValue::new(&mut rot[1]).speed(1.0).suffix("°"));
                    ui.label(egui::RichText::new("Z").color(tokens::AXIS_Z).strong());
                    let r2 = ui.add(egui::DragValue::new(&mut rot[2]).speed(1.0).suffix("°"));

                    if r0.changed() || r1.changed() || r2.changed() {
                        changed = true;
                    }
                    if r0.drag_stopped() || r1.drag_stopped() || r2.drag_stopped() {
                        stopped = true;
                    }
                });

                ui.add_space(2.0);

                // Scale X, Y, Z
                ui.label(egui::RichText::new("Scale").strong().size(11.0));
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("X").color(tokens::AXIS_X).strong());
                    let r0 = ui.add(
                        egui::DragValue::new(&mut scale[0])
                            .speed(0.02)
                            .range(0.01..=100.0),
                    );
                    ui.label(egui::RichText::new("Y").color(tokens::AXIS_Y).strong());
                    let r1 = ui.add(
                        egui::DragValue::new(&mut scale[1])
                            .speed(0.02)
                            .range(0.01..=100.0),
                    );
                    ui.label(egui::RichText::new("Z").color(tokens::AXIS_Z).strong());
                    let r2 = ui.add(
                        egui::DragValue::new(&mut scale[2])
                            .speed(0.02)
                            .range(0.01..=100.0),
                    );

                    if r0.changed() || r1.changed() || r2.changed() {
                        changed = true;
                    }
                    if r0.drag_stopped() || r1.drag_stopped() || r2.drag_stopped() {
                        stopped = true;
                    }
                });

                ui.add_space(4.0);

                if widgets::petunia_action_button(ui, None, "Reset Transform", false).clicked() {
                    state.checkpoint("reset annotation transform");
                    state.project.annotations[ann_idx].translation = [0.0, 0.0, 0.0];
                    state.project.annotations[ann_idx].rotation = [0.0, 0.0, 0.0];
                    state.project.annotations[ann_idx].scale = [1.0, 1.0, 1.0];
                    state.mark_dirty();
                }

                if changed {
                    state.project.annotations[ann_idx].translation = trans;
                    state.project.annotations[ann_idx].rotation = rot;
                    state.project.annotations[ann_idx].scale = scale;
                    state.mark_dirty();
                }
                if stopped {
                    state.checkpoint("transform annotation");
                }
            });
        });

    ui.add_space(8.0);
    ui.separator();

    // 5. Ações (Deletar Anotação)
    ui.horizontal(|ui| {
        if widgets::petunia_action_button(ui, Some(PetuniaIcon::Delete), "Delete Annotation", true)
            .clicked()
        {
            state.checkpoint("delete annotation");
            state.project.remove_annotation(ann_id);
            state.selected_annotation = None;
            state.mark_dirty();
        }
    });
}

fn draw_tab_measurement(ui: &mut Ui, state: &mut AppState, meas_id: Uuid) {
    let Some(meas_idx) = state
        .project
        .measurements
        .iter()
        .position(|m| m.id == meas_id)
    else {
        ui.label(egui::RichText::new("Measurement not found").italics());
        return;
    };

    let meas = &state.project.measurements[meas_idx];
    let distance = meas.distance;
    let start = meas.start;
    let end = meas.end;
    let deltas = meas.deltas();
    let mut name = meas.name.clone();

    // 1. Identidade da Medição (Header amarelo)
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(6.0, 0.0);
        let (icon_rect, _) = ui.allocate_exact_size(vec2(16.0, 16.0), egui::Sense::hover());
        IconRegistry::paint(
            ui.ctx(),
            ui.painter(),
            &PetuniaIcon::Measure,
            icon_rect,
            Color32::from_rgb(0xfe, 0xca, 0x57),
        );
        ui.label(
            egui::RichText::new("Measurement")
                .color(Color32::from_rgb(0xfe, 0xca, 0x57))
                .strong(),
        );
    });

    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Name")
                .size(11.0)
                .color(tokens::TEXT_SECONDARY),
        );
        let name_resp = ui.add(egui::TextEdit::singleline(&mut name).hint_text("Measurement Name"));
        if name_resp.changed() {
            state.project.measurements[meas_idx].name = name;
            state.mark_dirty();
        }
        if name_resp.lost_focus() {
            state.checkpoint("rename measurement");
        }
    });

    ui.add_space(4.0);

    // 2. Opções: estritamente apenas ocultar ou deletar
    ui.horizontal(|ui| {
        let mut vis = state.project.measurements[meas_idx].visible;
        if ui.checkbox(&mut vis, "Visible").changed() {
            state.checkpoint("toggle measurement visibility");
            state.project.measurements[meas_idx].visible = vis;
            state.mark_dirty();
        }
    });

    ui.add_space(4.0);

    // 3. Leituras de Medição (Readouts)
    egui::CollapsingHeader::new("Measurement Values")
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Total Distance:").strong());
                ui.label(
                    egui::RichText::new(format!("{distance:.3} m"))
                        .color(Color32::from_rgb(0xfe, 0xca, 0x57))
                        .strong(),
                );
            });

            ui.add_space(2.0);

            ui.label(
                egui::RichText::new("Cartesian Deltas (|Δ|)")
                    .size(11.0)
                    .color(tokens::TEXT_MUTED),
            );
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("ΔX").color(tokens::AXIS_X).strong());
                ui.label(format!("{:.3} m", deltas[0]));
                ui.label(egui::RichText::new("ΔY").color(tokens::AXIS_Y).strong());
                ui.label(format!("{:.3} m", deltas[1]));
                ui.label(egui::RichText::new("ΔZ").color(tokens::AXIS_Z).strong());
                ui.label(format!("{:.3} m", deltas[2]));
            });

            ui.add_space(2.0);

            ui.label(
                egui::RichText::new("Coordinates")
                    .size(11.0)
                    .color(tokens::TEXT_MUTED),
            );
            ui.label(format!(
                "Start: ({:.2}, {:.2}, {:.2})",
                start[0], start[1], start[2]
            ));
            ui.label(format!(
                "End:   ({:.2}, {:.2}, {:.2})",
                end[0], end[1], end[2]
            ));
        });

    ui.add_space(8.0);
    ui.separator();

    // 4. Ações: estritamente deletar
    ui.horizontal(|ui| {
        if widgets::petunia_action_button(ui, Some(PetuniaIcon::Delete), "Delete Measurement", true)
            .clicked()
        {
            state.checkpoint("delete measurement");
            state.project.remove_measurement(meas_id);
            state.selected_measurement = None;
            state.mark_dirty();
        }
    });
}

fn draw_tab_modifiers(ui: &mut Ui, state: &mut AppState) {
    egui::CollapsingHeader::new("Modifier Stack")
        .default_open(true)
        .show(ui, |ui| {
            ui.menu_button("+ Add Modifier", |ui| {
                if ui.button("Bevel").clicked() {
                    state.active_tool = "bevel".into();
                    ui.close();
                }
                if ui.button("Mirror").clicked() {
                    state.active_tool = "mirror".into();
                    ui.close();
                }
                if ui.button("Subdivision Surface").clicked() {
                    state.active_tool = "subdivide".into();
                    ui.close();
                }
            });

            ui.separator();
            ui.label("No active modifiers in stack.");
        });
}

fn draw_tab_data(ui: &mut Ui, state: &mut AppState) {
    if let Some(mesh) = state.project.active_mesh() {
        egui::CollapsingHeader::new("Estatísticas de Geometria")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!("Vértices: {}", mesh.vert_count()));
                ui.label(format!("Triângulos: {}", mesh.tri_count()));
                ui.label(format!("Faces: {}", mesh.faces.len()));
            });
    }

    crate::refs_section(ui, state);
}

fn draw_tab_material(ui: &mut Ui, state: &mut AppState) {
    if state.project.assets.is_empty() {
        ui.label("Nenhum modelo ativo para material");
        return;
    }

    let idx = state.project.active.min(state.project.assets.len() - 1);
    egui::CollapsingHeader::new("Material & Cores")
        .default_open(true)
        .show(ui, |ui| {
            let mut c = state.project.assets[idx].base_color;
            ui.horizontal(|ui| {
                ui.label("Cor Base:");
                if ui.color_edit_button_rgb(&mut c).changed() {
                    state.checkpoint("base color");
                    if let Some(o) = state.project.assets.get_mut(idx) {
                        o.base_color = c;
                        for v in &mut o.mesh.verts {
                            if !v.selected {
                                v.color = c;
                            }
                        }
                    }
                    state.emit_mesh_changed();
                    state.mark_dirty();
                }
            });

            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("Paleta do Projeto:")
                    .size(11.0)
                    .color(tokens::TEXT_MUTED),
            );
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = vec2(4.0, 4.0);
                let palette_colors = state.project.palette.clone();
                for (pal_idx, pal_col) in palette_colors.iter().enumerate() {
                    let color32 = egui::Color32::from_rgb(
                        (pal_col[0] * 255.0) as u8,
                        (pal_col[1] * 255.0) as u8,
                        (pal_col[2] * 255.0) as u8,
                    );
                    let (rect, resp) =
                        ui.allocate_exact_size(vec2(18.0, 18.0), egui::Sense::click());
                    ui.painter()
                        .rect_filled(rect, tokens::RADIUS_CONTROL, color32);
                    ui.painter().rect_stroke(
                        rect,
                        tokens::RADIUS_CONTROL,
                        tokens::stroke_border(),
                        egui::StrokeKind::Outside,
                    );
                    if resp
                        .on_hover_text(format!("Aplicar cor {pal_idx}"))
                        .clicked()
                    {
                        state.checkpoint("apply palette color");
                        if let Some(o) = state.project.assets.get_mut(idx) {
                            o.base_color = *pal_col;
                            for v in &mut o.mesh.verts {
                                if !v.selected {
                                    v.color = *pal_col;
                                }
                            }
                        }
                        state.emit_mesh_changed();
                        state.mark_dirty();
                    }
                }
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properties_panel_renders_without_panic() {
        let ctx = Context::default();
        let mut state = AppState::new("en");
        let tools = ToolRegistry::default();
        let mut registry = ModuleRegistry::new();

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw(ctx, ui, &mut state, &tools, &mut registry);
            });
        });
    }

    #[test]
    fn test_properties_panel_renders_annotation_inspector() {
        let ctx = Context::default();
        let mut state = AppState::new("en");
        let tools = ToolRegistry::default();
        let mut registry = ModuleRegistry::new();

        let stroke = petunia_core::AnnotationStroke {
            points: vec![[0.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
            color: [0.0, 0.74, 0.83, 1.0],
            width: 3.0,
        };
        let mut ann = petunia_core::AnnotationItem::new("TestNote", vec![stroke]);
        ann.translation = [1.0, 2.0, 3.0];
        ann.rotation = [45.0, 0.0, 0.0];
        ann.scale = [2.0, 2.0, 2.0];
        let ann_id = ann.id;
        state.project.add_annotation(ann);
        state.selected_annotation = Some(ann_id);
        state.ui.properties_tab = "object".to_string();

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw(ctx, ui, &mut state, &tools, &mut registry);
            });
        });

        assert_eq!(state.project.annotations.len(), 1);
        assert_eq!(state.project.annotations[0].translation, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_properties_panel_renders_measurement_inspector() {
        let ctx = Context::default();
        let mut state = AppState::new("en");
        let tools = ToolRegistry::default();
        let mut registry = ModuleRegistry::new();

        let meas =
            petunia_core::MeasurementItem::new("TestMeas", [0.0, 0.0, 0.0], [3.0, 4.0, 0.0], 5.0);
        let meas_id = meas.id;
        state.project.add_measurement(meas);
        state.selected_measurement = Some(meas_id);
        state.ui.properties_tab = "object".to_string();

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw(ctx, ui, &mut state, &tools, &mut registry);
            });
        });

        assert_eq!(state.project.measurements.len(), 1);
        assert_eq!(state.project.measurements[0].distance, 5.0);
    }
}
