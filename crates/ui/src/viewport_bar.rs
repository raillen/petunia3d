//! Barra de contexto superior do Viewport 3D (`3D View Bar`).
//! Implementa os controles de modo (Object/Edit), seleção de componente (1/2/3),
//! orientação de transformação, snapping, proporcional e modos de shading canônicos do Blender.svg.

use egui::{vec2, Ui};
use petunia_core::{AppState, EditMode, Projection, SelectMode};
use petunia_render::Shading;

use crate::tokens;

/// Renderiza a barra de contexto horizontal do Viewport 3D.
pub fn draw(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(4.0, 0.0);

        // 1. Seletor de Modo (Object / Edit)
        draw_mode_selector(ui, state);

        ui.add_space(2.0);

        // 2. Seleção de Componentes (Vértice / Aresta / Face) no Modo de Edição
        if state.mode == EditMode::Edit {
            draw_component_selectors(ui, state);
            ui.separator();
        }

        // 3. Menus contextuais do Viewport (View, Select, Add, Mesh)
        draw_viewport_menus(ui, state);

        ui.separator();

        // 4. Orientação de Transformação (Global, Local, etc.)
        draw_transform_orientation(ui, state);

        // 5. Ponto de Pivô
        draw_pivot_point(ui, state);

        // 6. Snapping e Edição Proporcional
        draw_snap_and_proportional(ui, state);

        // 7. Controles do lado direito (Overlays, X-Ray e 4 Esferas de Shading)
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            draw_shading_spheres(ui, state);
            ui.separator();
            draw_display_toggles(ui, state);
        });
    });
}

fn draw_mode_selector(ui: &mut Ui, state: &mut AppState) {
    let (mode_label, mode_color) = match state.mode {
        EditMode::Object => ("Object Mode", tokens::MODE_OBJECT),
        EditMode::Edit => ("Edit Mode", tokens::MODE_EDIT),
        EditMode::TexturePaint => ("Paint Mode", tokens::MODE_PAINT),
    };

    let _ = mode_color;
    egui::ComboBox::from_id_salt("viewport_mode_select")
        .selected_text(
            egui::RichText::new(mode_label)
                .size(11.5)
                .color(tokens::TEXT_ACTIVE),
        )
        .show_ui(ui, |ui| {
            if ui
                .selectable_label(state.mode == EditMode::Object, "Object Mode · Tab")
                .clicked()
            {
                state.mode = EditMode::Object;
                state.mark_dirty();
            }
            if ui
                .selectable_label(state.mode == EditMode::Edit, "Edit Mode · Tab")
                .clicked()
            {
                state.mode = EditMode::Edit;
                state.mark_dirty();
            }
            if ui
                .selectable_label(state.mode == EditMode::TexturePaint, "Paint Mode")
                .clicked()
            {
                state.mode = EditMode::TexturePaint;
                state.mark_dirty();
            }
        });
}

fn draw_component_selectors(ui: &mut Ui, state: &mut AppState) {
    let components = [
        (SelectMode::Vertex, "Vertex", "1", "Vértices · 1"),
        (SelectMode::Edge, "Edge", "2", "Arestas · 2"),
        (SelectMode::Face, "Face", "3", "Faces · 3"),
    ];

    for (mode, tag, shortcut, hint) in components {
        let is_active = state.select_mode == mode;
        let (bg, fg) = if is_active {
            (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
        } else {
            (tokens::BG_SURFACE, tokens::TEXT_SECONDARY)
        };

        let btn = egui::Button::new(
            egui::RichText::new(format!("{tag} [{shortcut}]"))
                .size(10.5)
                .color(fg),
        )
        .fill(bg)
        .corner_radius(tokens::RADIUS_CONTROL);

        if ui.add(btn).on_hover_text(hint).clicked() {
            state.select_mode = mode;
            state.sync_selection();
        }
    }
}

fn draw_viewport_menus(ui: &mut Ui, state: &mut AppState) {
    ui.menu_button("View", |ui| {
        if ui.button("Frame Selected · Numpad .").clicked() {
            crate::frame_selection(state);
            ui.close();
        }
        ui.separator();
        if ui.button("Perspective / Ortho · Numpad 5").clicked() {
            state.camera.proj = match state.camera.proj {
                Projection::Perspective => Projection::Ortho,
                Projection::Ortho => Projection::Perspective,
            };
            state.mark_dirty();
            ui.close();
        }
        if ui.button("Front · Numpad 1").clicked() {
            state.camera.set_preset(petunia_core::ViewPreset::Front);
            state.mark_dirty();
            ui.close();
        }
        if ui.button("Right · Numpad 3").clicked() {
            state.camera.set_preset(petunia_core::ViewPreset::Right);
            state.mark_dirty();
            ui.close();
        }
        if ui.button("Top · Numpad 7").clicked() {
            state.camera.set_preset(petunia_core::ViewPreset::Top);
            state.mark_dirty();
            ui.close();
        }
    });

    ui.menu_button("Select", |ui| {
        if ui.button("Select All · A").clicked() {
            if let Some(m) = state.project.active_mesh_mut() {
                m.select_all();
            }
            state.sync_selection();
            ui.close();
        }
        if ui.button("Deselect All · Alt+A").clicked() {
            if let Some(m) = state.project.active_mesh_mut() {
                m.deselect_all();
            }
            state.sync_selection();
            ui.close();
        }
        if ui.button("Invert Selection · Ctrl+I").clicked() {
            if let Some(m) = state.project.active_mesh_mut() {
                m.invert_selection();
            }
            state.sync_selection();
            ui.close();
        }
    });

    ui.menu_button("Add", |ui| {
        if ui.button("Cube").clicked() {
            state.active_tool = "primitives".into();
            state.mark_dirty();
            ui.close();
        }
        if ui.button("Cylinder").clicked() {
            state.active_tool = "primitives".into();
            state.mark_dirty();
            ui.close();
        }
    });
}

fn draw_transform_orientation(ui: &mut Ui, state: &mut AppState) {
    egui::ComboBox::from_id_salt("transform_orientation")
        .selected_text(
            egui::RichText::new(&state.transform_orientation)
                .size(11.0)
                .color(tokens::TEXT_PRIMARY),
        )
        .width(68.0)
        .show_ui(ui, |ui| {
            for orient in ["Global", "Local", "Normal", "Gimbal", "View", "Cursor"] {
                if ui
                    .selectable_label(state.transform_orientation == orient, orient)
                    .clicked()
                {
                    state.transform_orientation = orient.to_string();
                    state.mark_dirty();
                }
            }
        });
}

fn draw_pivot_point(ui: &mut Ui, state: &mut AppState) {
    egui::ComboBox::from_id_salt("pivot_point")
        .selected_text(
            egui::RichText::new(&state.pivot_point)
                .size(11.0)
                .color(tokens::TEXT_PRIMARY),
        )
        .width(96.0)
        .show_ui(ui, |ui| {
            for pivot in [
                "Bounding Box",
                "3D Cursor",
                "Individual Origins",
                "Median Point",
                "Active Element",
            ] {
                if ui
                    .selectable_label(state.pivot_point == pivot, pivot)
                    .clicked()
                {
                    state.pivot_point = pivot.to_string();
                    state.mark_dirty();
                }
            }
        });
}

fn draw_snap_and_proportional(ui: &mut Ui, state: &mut AppState) {
    // Botão Snap (Ímã)
    let snap_bg = if state.snap_enabled {
        tokens::ACCENT_BLUE
    } else {
        tokens::BG_SURFACE
    };
    let snap_btn = egui::Button::new(
        egui::RichText::new("🧲 Snap")
            .size(10.5)
            .color(tokens::TEXT_ACTIVE),
    )
    .fill(snap_bg)
    .corner_radius(tokens::RADIUS_CONTROL);

    if ui
        .add(snap_btn)
        .on_hover_text("Snapping Magnético · Shift+Tab")
        .clicked()
    {
        state.snap_enabled = !state.snap_enabled;
        state.mark_dirty();
    }

    // Botão Edição Proporcional
    let prop_bg = if state.proportional_editing {
        tokens::ACCENT_BLUE
    } else {
        tokens::BG_SURFACE
    };
    let prop_btn = egui::Button::new(
        egui::RichText::new("◎ Prop")
            .size(10.5)
            .color(tokens::TEXT_ACTIVE),
    )
    .fill(prop_bg)
    .corner_radius(tokens::RADIUS_CONTROL);

    if ui
        .add(prop_btn)
        .on_hover_text("Edição Proporcional · O")
        .clicked()
    {
        state.proportional_editing = !state.proportional_editing;
        state.mark_dirty();
    }
}

fn draw_display_toggles(ui: &mut Ui, state: &mut AppState) {
    // Toggle Overlays (Grid, Eixos, Cursor)
    let ov_bg = if state.show_overlays {
        tokens::ACCENT_BLUE
    } else {
        tokens::BG_SURFACE
    };
    let ov_btn = egui::Button::new(
        egui::RichText::new("Overlays")
            .size(10.5)
            .color(tokens::TEXT_ACTIVE),
    )
    .fill(ov_bg)
    .corner_radius(tokens::RADIUS_CONTROL);

    if ui
        .add(ov_btn)
        .on_hover_text("Alternar Exibição de Overlays")
        .clicked()
    {
        state.show_overlays = !state.show_overlays;
        state.mark_dirty();
    }

    // Toggle X-Ray
    let xray_bg = if state.show_xray {
        tokens::ACCENT_BLUE
    } else {
        tokens::BG_SURFACE
    };
    let xray_btn = egui::Button::new(
        egui::RichText::new("X-Ray")
            .size(10.5)
            .color(tokens::TEXT_ACTIVE),
    )
    .fill(xray_bg)
    .corner_radius(tokens::RADIUS_CONTROL);

    if ui
        .add(xray_btn)
        .on_hover_text("Modo Raio-X / Transparência · Alt+Z")
        .clicked()
    {
        state.show_xray = !state.show_xray;
        state.mark_dirty();
    }
}

fn draw_shading_spheres(ui: &mut Ui, state: &mut AppState) {
    let modes = [
        (Shading::Wireframe, "Wire", "Wireframe Shading · Z 4"),
        (Shading::Solid, "Solid", "Solid Shading · Z 6"),
        (Shading::Smooth, "Material", "Material Preview · Z 2"),
        (Shading::Unlit, "Render", "Rendered View · Z 8"),
    ];

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(2.0, 0.0);
        for (shading, label, hint) in modes {
            let is_active = state.shading == shading;
            let (bg, fg) = if is_active {
                (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
            } else {
                (tokens::BG_SURFACE, tokens::TEXT_SECONDARY)
            };

            let btn = egui::Button::new(egui::RichText::new(label).size(10.5).color(fg))
                .fill(bg)
                .corner_radius(tokens::RADIUS_CONTROL);

            if ui.add(btn).on_hover_text(hint).clicked() {
                state.shading = shading;
                state.mark_dirty();
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_bar_renders_without_panic() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw(ui, &mut state);
            });
        });
    }
}
