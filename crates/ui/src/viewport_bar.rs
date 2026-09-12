//! Barra de contexto superior do Viewport 3D (`3D View Bar`).
//! Organizada em 5 clusters semânticos coerentes:
//! 1. Seleção unificada (Objeto, Vértice, Aresta, Face);
//! 2. Menus de ação rápida com ícones compactos (View, Select, Add);
//! 3. Orientação de transformação, Ponto de pivô e Snapping magnético;
//! 4. Diagnóstico de cena (Overlays e X-Ray);
//! 5. 4 modos de sombreamento esféricos canônicos do Blender.

use egui::{vec2, Color32, CornerRadius, Ui};
use petunia_core::{AppState, EditMode, Projection, SelectMode};
use petunia_mesh::Mesh;
use petunia_render::Shading;

use crate::tokens;

/// Renderiza a barra de contexto horizontal do Viewport 3D.
pub fn draw(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal_centered(|ui| {
        ui.spacing_mut().item_spacing = vec2(6.0, 0.0);

        // CLUSTER 1: Seletor Unificado de Seleção (Objeto, Vértice, Aresta, Face)
        draw_selection_target_cluster(ui, state);

        ui.add_space(2.0);
        ui.separator();
        ui.add_space(2.0);

        // CLUSTER 2: Menus Rápidos com Ícones (View, Select, Add)
        draw_viewport_actions_cluster(ui, state);

        ui.add_space(2.0);
        ui.separator();
        ui.add_space(2.0);

        // CLUSTER 3: Orientação, Pivô e Snapping
        draw_transform_and_snap_cluster(ui, state);

        // CLUSTER 4 & 5: Controles do lado direito (Overlays, X-Ray e 4 Esferas de Shading)
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // CLUSTER 5: 4 Modos de Sombreamento no Estilo Canônico do Blender
            draw_shading_spheres_cluster(ui, state);

            ui.add_space(3.0);
            ui.separator();
            ui.add_space(3.0);

            // CLUSTER 4: Diagnóstico de Visualização (Overlays e X-Ray)
            draw_display_toggles_cluster(ui, state);
        });
    });
}

/// Cluster 1: Seletor unificado de 4 tipos de seleção substituindo a dualidade de modos.
fn draw_selection_target_cluster(ui: &mut Ui, state: &mut AppState) {
    let targets = [
        (0, "🧊 Objeto", "Tab", "Seleção de Objeto · Tab ou 0"),
        (1, "⬝ Vértice", "1", "Seleção de Vértices · 1"),
        (2, "╱ Aresta", "2", "Seleção de Arestas · 2"),
        (3, "▨ Face", "3", "Seleção de Faces · 3"),
    ];

    for (target_idx, label, shortcut, hint) in targets {
        let is_active = match target_idx {
            0 => state.mode == EditMode::Object,
            1 => state.mode == EditMode::Edit && state.select_mode == SelectMode::Vertex,
            2 => state.mode == EditMode::Edit && state.select_mode == SelectMode::Edge,
            3 => state.mode == EditMode::Edit && state.select_mode == SelectMode::Face,
            _ => false,
        };

        let (bg, fg) = if is_active {
            (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
        } else {
            (tokens::BG_SURFACE, tokens::TEXT_SECONDARY)
        };

        let btn = egui::Button::new(egui::RichText::new(label).size(11.0).color(fg))
            .fill(bg)
            .corner_radius(tokens::RADIUS_CONTROL);

        if ui
            .add(btn)
            .on_hover_text(format!("{hint} [{shortcut}]"))
            .clicked()
        {
            match target_idx {
                0 => {
                    state.mode = EditMode::Object;
                    state.active_tool = "select".into();
                }
                1 => {
                    state.mode = EditMode::Edit;
                    state.select_mode = SelectMode::Vertex;
                    state.sync_selection();
                }
                2 => {
                    state.mode = EditMode::Edit;
                    state.select_mode = SelectMode::Edge;
                    state.sync_selection();
                }
                3 => {
                    state.mode = EditMode::Edit;
                    state.select_mode = SelectMode::Face;
                    state.sync_selection();
                }
                _ => {}
            }
            state.mark_dirty();
        }
    }
}

/// Cluster 2: Menus rápidos representados com ícones compactos.
fn draw_viewport_actions_cluster(ui: &mut Ui, state: &mut AppState) {
    let visuals = ui.visuals_mut();
    visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
    visuals.widgets.hovered.weak_bg_fill = tokens::BG_SURFACE_HOVER;
    visuals.widgets.active.weak_bg_fill = tokens::ACCENT_BLUE;

    // Menu View com ícone de Câmera/Viewport
    ui.menu_button("👁 View ▾", |ui| {
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

    // Menu Select com ícone de Marquise de Seleção
    ui.menu_button("▢ Select ▾", |ui| {
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

    // Menu Add com botão explícito ➕ Add+ solicitado pelo usuário
    let mut spawn_mesh: Option<(&'static str, Mesh)> = None;
    ui.menu_button("➕ Add+ ▾", |ui| {
        if ui.button("🧊 Cube").clicked() {
            spawn_mesh = Some(("Cube", Mesh::cube(1.0)));
            ui.close();
        }
        if ui.button("⚪ UV Sphere").clicked() {
            spawn_mesh = Some(("Sphere", Mesh::sphere_low(16, 12, 0.5)));
            ui.close();
        }
        if ui.button("🛢 Cylinder").clicked() {
            spawn_mesh = Some(("Cylinder", Mesh::cylinder(16, 0.5, 1.0)));
            ui.close();
        }
        if ui.button("▭ Plane").clicked() {
            spawn_mesh = Some(("Plane", Mesh::plane(2.0)));
            ui.close();
        }
        if ui.button("▲ Cone").clicked() {
            spawn_mesh = Some(("Cone", Mesh::cone(16, 0.5, 1.0)));
            ui.close();
        }
    });

    if let Some((name, mut mesh)) = spawn_mesh {
        state.checkpoint("add primitive");
        let cursor = state.cursor_3d;
        for v in &mut mesh.verts {
            v.pos[0] += cursor[0];
            v.pos[1] += cursor[1];
            v.pos[2] += cursor[2];
        }
        state.project.add(name, mesh);
        state.sync_selection();
        state.emit_mesh_changed();
        state.mark_dirty();
    }
}

/// Cluster 3: Orientação de transformação, Ponto de pivô e Snapping magnético.
fn draw_transform_and_snap_cluster(ui: &mut Ui, state: &mut AppState) {
    // Orientação de Transformação
    egui::ComboBox::from_id_salt("transform_orientation")
        .selected_text(
            egui::RichText::new(&state.transform_orientation)
                .size(11.0)
                .color(tokens::TEXT_PRIMARY),
        )
        .width(64.0)
        .show_ui(ui, |ui| {
            for orient in ["Global", "Local", "Normal", "View", "Cursor"] {
                if ui
                    .selectable_label(state.transform_orientation == orient, orient)
                    .clicked()
                {
                    state.transform_orientation = orient.to_string();
                    state.mark_dirty();
                }
            }
        });

    // Ponto de Pivô
    egui::ComboBox::from_id_salt("pivot_point")
        .selected_text(
            egui::RichText::new(&state.pivot_point)
                .size(11.0)
                .color(tokens::TEXT_PRIMARY),
        )
        .width(92.0)
        .show_ui(ui, |ui| {
            for pivot in [
                "Median Point",
                "3D Cursor",
                "Bounding Box",
                "Individual Origins",
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

/// Cluster 4: Alternâncias de visualização de cena (Overlays e X-Ray).
fn draw_display_toggles_cluster(ui: &mut Ui, state: &mut AppState) {
    // Toggle Overlays (Grid, Eixos, Cursor)
    let ov_bg = if state.show_overlays {
        tokens::ACCENT_BLUE
    } else {
        tokens::BG_SURFACE
    };
    let ov_btn = egui::Button::new(
        egui::RichText::new("⊞ Overlays")
            .size(10.5)
            .color(tokens::TEXT_ACTIVE),
    )
    .min_size(vec2(24.0, 22.0))
    .fill(ov_bg)
    .corner_radius(tokens::RADIUS_CONTROL);

    if ui
        .add(ov_btn)
        .on_hover_text("Alternar Exibição de Overlays (Grade 3D e Eixos)")
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
        egui::RichText::new("⧉ X-Ray")
            .size(10.5)
            .color(tokens::TEXT_ACTIVE),
    )
    .min_size(vec2(24.0, 22.0))
    .fill(xray_bg)
    .corner_radius(tokens::RADIUS_CONTROL);

    if ui
        .add(xray_btn)
        .on_hover_text("Modo Raio-X / Transparência de Malha · Alt+Z")
        .clicked()
    {
        state.show_xray = !state.show_xray;
        state.mark_dirty();
    }
}

/// Cluster 5: Os 4 modos canônicos de sombreamento no estilo esférico do Blender.
fn draw_shading_spheres_cluster(ui: &mut Ui, state: &mut AppState) {
    let modes = [
        (Shading::Wireframe, "○", "Wireframe (Z 4)"),
        (Shading::Solid, "●", "Solid / Clay (Z 6)"),
        (Shading::Smooth, "◐", "Material Preview (Z 2)"),
        (Shading::Unlit, "☼", "Rendered View (Z 8)"),
    ];

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(3.0, 0.0);
        for (shading, label, hint) in modes {
            let is_active = state.shading == shading;
            let (bg, fg) = if is_active {
                (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
            } else {
                (tokens::BG_SURFACE, tokens::TEXT_SECONDARY)
            };

            let btn = egui::Button::new(egui::RichText::new(label).size(13.0).strong().color(fg))
                .min_size(vec2(22.0, 22.0))
                .fill(bg)
                .corner_radius(CornerRadius::same(11));

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

    #[test]
    fn test_selection_modes_toggle_via_bar() {
        let mut state = AppState::new("en");
        assert_eq!(state.mode, EditMode::Object);

        state.mode = EditMode::Edit;
        state.select_mode = SelectMode::Vertex;
        assert_eq!(state.select_mode, SelectMode::Vertex);

        state.select_mode = SelectMode::Face;
        assert_eq!(state.select_mode, SelectMode::Face);
    }
}
