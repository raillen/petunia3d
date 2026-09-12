//! Painel de Propriedades do Petunia3D (`Properties Panel`).
//! Contém a barra vertical de abas canônicas (Tool, Render, Output, Object, Modifiers, Data, Material)
//! e formulários sanfonados com fidelidade estética ao Blender.svg.

use egui::{vec2, Context, ScrollArea, Ui};
use petunia_core::{AppState, ModuleRegistry, Workspace};
use petunia_module_model::ToolRegistry;

use crate::tokens;
use crate::tool_fields;

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
                            state.checkpoint("duplicate");
                            if let Some(mesh) = state.project.active_mesh_mut() {
                                mesh.duplicate_selected();
                            }
                            state.sync_selection();
                            state.emit_mesh_changed();
                        }
                        if ui.button(state.t("actions.delete")).clicked() {
                            state.checkpoint("delete");
                            if let Some(mesh) = state.project.active_mesh_mut() {
                                mesh.delete_selected();
                            }
                            state.sync_selection();
                            state.emit_mesh_changed();
                        }
                    });
                });
            }

            ui.add_enabled_ui(!state.is_interacting(), |ui| {
                match state.workspace {
                    Workspace::Model => draw_active_tab_content(ctx, ui, state, tools, fields),
                    Workspace::Paint => {
                        if let Some(module) = registry.get_mut("paint") {
                            module.ui(ctx, ui, state);
                        }
                    }
                    Workspace::Uv => {
                        if let Some(module) = registry.get_mut("uv") {
                            module.ui(ctx, ui, state);
                        }
                    }
                    Workspace::Export => crate::export_section(ui, state),
                }

                ui.separator();
                if let Some(module) = registry.get_mut("assets") {
                    module.ui(ctx, ui, state);
                }

                if state.show_help {
                    egui::CollapsingHeader::new(state.t("ui.help"))
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.label(state.t("help.body"));
                        });
                }
            });
        });
}

fn draw_property_tabs(ui: &mut Ui, state: &mut AppState) {
    let tabs = [
        ("tool", "🔧", "Active Tool & Workspace Settings"),
        ("render", "📷", "Render Properties"),
        ("output", "🖨", "Output & Export Properties"),
        ("scene", "🎬", "Scene Properties"),
        ("world", "🌐", "World Environment"),
        ("object", "📦", "Object Transform & Properties"),
        ("modifiers", "⚡", "Modifier Stack"),
        ("data", "📐", "Mesh Data & References"),
        ("material", "🎨", "Material & Surface Shading"),
    ];

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(3.0, 3.0);
        for (tab_id, icon, hint) in tabs {
            let is_active = state.properties_tab == tab_id;
            let (bg, fg) = if is_active {
                (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
            } else {
                (tokens::BG_SURFACE, tokens::TEXT_SECONDARY)
            };

            let btn = egui::Button::new(egui::RichText::new(icon).size(12.5).color(fg))
                .fill(bg)
                .min_size(vec2(24.0, 24.0))
                .corner_radius(tokens::RADIUS_CONTROL);

            if ui.add(btn).on_hover_text(hint).clicked() {
                state.properties_tab = tab_id.to_string();
                state.mark_dirty();
            }
        }
    });
}

fn draw_active_tab_content(
    ctx: &Context,
    ui: &mut Ui,
    state: &mut AppState,
    tools: &ToolRegistry,
    fields: bool,
) {
    match state.properties_tab.as_str() {
        "tool" => draw_tab_tool(ctx, ui, state, tools, fields),
        "render" => draw_tab_render(ui, state),
        "output" => crate::export_section(ui, state),
        "scene" => draw_tab_scene(ui, state),
        "world" => draw_tab_world(ui, state),
        "object" => draw_tab_object(ui, state),
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
    if state.project.assets.is_empty() {
        ui.label("No active object in scene");
        return;
    }

    let idx = state.project.active.min(state.project.assets.len() - 1);

    // Seção de Transform (Location, Rotation, Scale)
    egui::CollapsingHeader::new("Transform")
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                if let Some(o) = state.project.assets.get_mut(idx) {
                    ui.text_edit_singleline(&mut o.name);
                }
            });

            ui.add_space(4.0);

            // Location X, Y, Z com cores semânticas nos rótulos
            ui.label(egui::RichText::new("Location").strong());
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("X").color(tokens::AXIS_X).strong());
                ui.add(egui::DragValue::new(&mut state.transform_delta[0]).speed(0.05));
                ui.label(egui::RichText::new("Y").color(tokens::AXIS_Y).strong());
                ui.add(egui::DragValue::new(&mut state.transform_delta[1]).speed(0.05));
                ui.label(egui::RichText::new("Z").color(tokens::AXIS_Z).strong());
                ui.add(egui::DragValue::new(&mut state.transform_delta[2]).speed(0.05));
            });

            // Scale
            ui.label(egui::RichText::new("Scale").strong());
            ui.horizontal(|ui| {
                ui.label("XYZ");
                ui.add(
                    egui::DragValue::new(&mut state.transform_scale)
                        .speed(0.02)
                        .range(0.01..=100.0),
                );
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Duplicate · Shift+D").clicked() {
                    state.checkpoint("duplicate");
                    if let Some(mesh) = state.project.active_mesh_mut() {
                        mesh.duplicate_selected();
                    }
                    state.sync_selection();
                    state.emit_mesh_changed();
                }
                if ui.button("Delete · X").clicked() {
                    state.checkpoint("delete");
                    if let Some(mesh) = state.project.active_mesh_mut() {
                        mesh.delete_selected();
                    }
                    state.sync_selection();
                    state.emit_mesh_changed();
                }
            });
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
        egui::CollapsingHeader::new("Geometry Statistics")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!("Vertices: {}", mesh.vert_count()));
                ui.label(format!("Triangles: {}", mesh.tri_count()));
                ui.label(format!("Faces: {}", mesh.faces.len()));
            });
    }

    crate::refs_section(ui, state);
}

fn draw_tab_material(ui: &mut Ui, state: &mut AppState) {
    if state.project.assets.is_empty() {
        return;
    }

    let idx = state.project.active.min(state.project.assets.len() - 1);
    egui::CollapsingHeader::new("Surface Material")
        .default_open(true)
        .show(ui, |ui| {
            let mut c = state.project.assets[idx].base_color;
            ui.horizontal(|ui| {
                ui.label("Base Color");
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
        });
}

fn draw_tab_render(ui: &mut Ui, state: &mut AppState) {
    egui::CollapsingHeader::new("Render Engine")
        .default_open(true)
        .show(ui, |ui| {
            ui.label(format!("Backend: {}", state.backend_name));
            ui.checkbox(&mut state.show_perf, "Display Performance Overlay");
        });
}

fn draw_tab_scene(ui: &mut Ui, _state: &mut AppState) {
    egui::CollapsingHeader::new("Scene Settings")
        .default_open(true)
        .show(ui, |ui| {
            ui.label("Unit System: Metric");
            ui.label("Gravity: -9.81 m/s² Z");
        });
}

fn draw_tab_world(ui: &mut Ui, state: &mut AppState) {
    egui::CollapsingHeader::new("World Environment")
        .default_open(true)
        .show(ui, |ui| {
            ui.checkbox(&mut state.textured, "Enable Textures & Lighting");
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
}
