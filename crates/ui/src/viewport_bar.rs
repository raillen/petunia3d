//! Barra de contexto superior do Viewport 3D (`3D View Bar`).
//! Organizada em 6 clusters semânticos coerentes:
//! 1. Modo de Trabalho (`[ Object Mode ▾ ]` ou `[ Edit Mode ▾ ]`) com alvos contextuais (`⬝ Vértice`, `╱ Aresta`, `▨ Face`) exibidos apenas em Edit Mode;
//! 2. Menus Rápidos (`👁 View ▾`, `▢ Select ▾`, `➕ Add+ ▾`, e menu contextual de `Objeto` / `Malha`);
//! 3. Orientação de Transformação e Ponto de Pivô;
//! 4. Snapping Magnético e Edição Proporcional;
//! 5. Diagnóstico de Cena (Overlays e X-Ray);
//! 6. 4 Modos de Sombreamento no estilo esférico canônico do Blender.

use egui::{vec2, Color32, CornerRadius, Ui};
use petunia_core::{AppState, EditMode, Projection, SelectMode};
use petunia_mesh::Mesh;
use petunia_render::Shading;

use crate::tokens;

/// Renderiza a barra de contexto horizontal do Viewport 3D.
pub fn draw(ui: &mut Ui, state: &mut AppState) {
    // Interceptação defensiva de atalhos globais de modo se nenhum campo de texto estiver focado
    handle_keyboard_shortcuts(ui, state);

    ui.horizontal_centered(|ui| {
        ui.spacing_mut().item_spacing = vec2(6.0, 0.0);

        // CLUSTER 1: Seletor de Modo e Alvos de Seleção Contextuais (Apenas em Edit Mode)
        draw_mode_and_targets_cluster(ui, state);

        ui.add_space(2.0);
        ui.separator();
        ui.add_space(2.0);

        // CLUSTER 2: Menus Rápidos com Ícones (View, Select, Add+, Objeto/Malha)
        draw_viewport_actions_cluster(ui, state);

        ui.add_space(2.0);
        ui.separator();
        ui.add_space(2.0);

        // CLUSTER 3: Orientação e Ponto de Pivô
        draw_transform_cluster(ui, state);

        ui.add_space(2.0);
        ui.separator();
        ui.add_space(2.0);

        // CLUSTER 4: Snapping Magnético e Edição Proporcional
        draw_snap_and_prop_cluster(ui, state);

        // CLUSTER 5 & 6: Controles do lado direito (Overlays, X-Ray e 4 Esferas de Sombreamento)
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // CLUSTER 6: 4 Modos de Sombreamento no Estilo Canônico do Blender
            draw_shading_spheres_cluster(ui, state);

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            // CLUSTER 5: Diagnóstico de Visualização (Overlays e X-Ray)
            draw_display_toggles_cluster(ui, state);
        });
    });
}

/// Trata atalhos de teclado (Tab, 1, 2, 3, 0) diretamente no egui para máxima responsividade.
fn handle_keyboard_shortcuts(ui: &mut Ui, state: &mut AppState) {
    if ui.ctx().wants_keyboard_input() {
        return;
    }

    // Tab: Alterna entre Object Mode e Edit Mode
    if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Tab)) {
        state.mode = match state.mode {
            EditMode::Object => EditMode::Edit,
            _ => EditMode::Object,
        };
        state.sync_selection();
        state.mark_dirty();
    }

    // 0: Modo Objeto
    if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Num0)) {
        state.mode = EditMode::Object;
        state.active_tool = "select".into();
        state.mark_dirty();
    }

    // 1: Vértice (em Edit Mode)
    if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Num1)) {
        state.mode = EditMode::Edit;
        state.select_mode = SelectMode::Vertex;
        state.sync_selection();
        state.mark_dirty();
    }

    // 2: Aresta (em Edit Mode)
    if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Num2)) {
        state.mode = EditMode::Edit;
        state.select_mode = SelectMode::Edge;
        state.sync_selection();
        state.mark_dirty();
    }

    // 3: Face (em Edit Mode)
    if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Num3)) {
        state.mode = EditMode::Edit;
        state.select_mode = SelectMode::Face;
        state.sync_selection();
        state.mark_dirty();
    }
}

/// Cluster 1: Seletor de Modo (`Object` vs `Edit`) + Alvos de Seleção (`⬝ Vértice`, `╱ Aresta`, `▨ Face`)
/// exibidos exclusivamente no modo de edição.
fn draw_mode_and_targets_cluster(ui: &mut Ui, state: &mut AppState) {
    let mode_text = match state.mode {
        EditMode::Object => "🧊 Object Mode ▾",
        EditMode::Edit => "🕸 Edit Mode ▾",
        _ => "Modo ▾",
    };

    ui.menu_button(
        egui::RichText::new(mode_text)
            .strong()
            .size(11.0)
            .color(tokens::TEXT_PRIMARY),
        |ui| {
            if ui.button("🧊 Object Mode · Tab").clicked() {
                state.mode = EditMode::Object;
                state.active_tool = "select".into();
                state.mark_dirty();
                ui.close();
            }
            if ui.button("🕸 Edit Mode · Tab").clicked() {
                state.mode = EditMode::Edit;
                state.sync_selection();
                state.mark_dirty();
                ui.close();
            }
        },
    );

    // Botões de alvo de seleção: visíveis EXCLUSIVAMENTE em modo de edição
    if state.mode == EditMode::Edit {
        ui.add_space(2.0);
        let targets = [
            (SelectMode::Vertex, "⬝ Vértice", "1", "Seleção de Vértices"),
            (SelectMode::Edge, "╱ Aresta", "2", "Seleção de Arestas"),
            (SelectMode::Face, "▨ Face", "3", "Seleção de Faces"),
        ];

        for (mode, label, shortcut, hint) in targets {
            let is_active = state.select_mode == mode;
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
                .on_hover_text(format!("{hint} · [{shortcut}]"))
                .clicked()
            {
                state.select_mode = mode;
                state.sync_selection();
                state.mark_dirty();
            }
        }
    }
}

/// Cluster 2: Menus rápidos com ícones (View, Select, Add+, Objeto/Malha).
fn draw_viewport_actions_cluster(ui: &mut Ui, state: &mut AppState) {
    let visuals = ui.visuals_mut();
    visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
    visuals.widgets.hovered.weak_bg_fill = tokens::BG_SURFACE_HOVER;
    visuals.widgets.active.weak_bg_fill = tokens::ACCENT_BLUE;

    // Menu View com ícone
    ui.menu_button("👁 View ▾", |ui| {
        if ui.button("Centralizar Seleção · Numpad .").clicked() {
            crate::frame_selection(state);
            ui.close();
        }
        ui.separator();
        if ui
            .button("Alternar Perspectiva / Ortho · Numpad 5")
            .clicked()
        {
            state.camera.proj = match state.camera.proj {
                Projection::Perspective => Projection::Ortho,
                Projection::Ortho => Projection::Perspective,
            };
            state.mark_dirty();
            ui.close();
        }
        if ui.button("Frente (Front) · Numpad 1").clicked() {
            state.camera.set_preset(petunia_core::ViewPreset::Front);
            state.mark_dirty();
            ui.close();
        }
        if ui.button("Direita (Right) · Numpad 3").clicked() {
            state.camera.set_preset(petunia_core::ViewPreset::Right);
            state.mark_dirty();
            ui.close();
        }
        if ui.button("Topo (Top) · Numpad 7").clicked() {
            state.camera.set_preset(petunia_core::ViewPreset::Top);
            state.mark_dirty();
            ui.close();
        }
    });

    // Menu Select com ícone
    ui.menu_button("▢ Select ▾", |ui| {
        if ui.button("Selecionar Tudo · A").clicked() {
            if let Some(m) = state.project.active_mesh_mut() {
                m.select_all();
            }
            state.sync_selection();
            ui.close();
        }
        if ui.button("Desmarcar Tudo · Alt+A").clicked() {
            if let Some(m) = state.project.active_mesh_mut() {
                m.deselect_all();
            }
            state.sync_selection();
            ui.close();
        }
        if ui.button("Inverter Seleção · Ctrl+I").clicked() {
            if let Some(m) = state.project.active_mesh_mut() {
                m.invert_selection();
            }
            state.sync_selection();
            ui.close();
        }
    });

    // Menu Add+ explícito conforme solicitado pelo usuário
    let mut spawn_mesh: Option<(&'static str, Mesh)> = None;
    ui.menu_button("➕ Add+ ▾", |ui| {
        if ui.button("🧊 Cubo").clicked() {
            spawn_mesh = Some(("Cube", Mesh::cube(1.0)));
            ui.close();
        }
        if ui.button("⚪ Esfera UV").clicked() {
            spawn_mesh = Some(("Sphere", Mesh::sphere_low(16, 12, 0.5)));
            ui.close();
        }
        if ui.button("🛢 Cilindro").clicked() {
            spawn_mesh = Some(("Cylinder", Mesh::cylinder(16, 0.5, 1.0)));
            ui.close();
        }
        if ui.button("▭ Plano").clicked() {
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

    // Menu Contextual: Objeto (em Object Mode) ou Malha (em Edit Mode)
    if state.mode == EditMode::Object {
        ui.menu_button("🧊 Object ▾", |ui| {
            if ui.button("📋 Duplicar Objeto · Shift+D").clicked() {
                if let Some(active) = state.project.active() {
                    let dup = active.duplicate();
                    state.checkpoint("duplicate object");
                    state.project.assets.push(dup);
                    state.project.active = state.project.assets.len() - 1;
                    state.sync_selection();
                    state.emit_mesh_changed();
                    state.mark_dirty();
                }
                ui.close();
            }
            if ui.button("🗑 Deletar Objeto · Delete").clicked() {
                if state.project.assets.len() > 1 {
                    let idx = state.project.active;
                    state.checkpoint("delete object");
                    state.project.assets.remove(idx);
                    state.project.active = state.project.active.min(state.project.assets.len() - 1);
                    state.sync_selection();
                    state.emit_mesh_changed();
                    state.mark_dirty();
                }
                ui.close();
            }
            ui.separator();
            if ui.button("🎯 Cursor para a Origem").clicked() {
                state.cursor_3d = [0.0, 0.0, 0.0];
                state.mark_dirty();
                ui.close();
            }
        });
    } else {
        ui.menu_button("🕸 Mesh ▾", |ui| {
            if ui.button("Extrusão (Extrude) · E").clicked() {
                state.active_tool = "extrude".into();
                state.mark_dirty();
                ui.close();
            }
            if ui.button("Inserção (Inset) · I").clicked() {
                state.active_tool = "inset".into();
                state.mark_dirty();
                ui.close();
            }
            if ui.button("Chanfro (Bevel) · Ctrl+B").clicked() {
                state.active_tool = "bevel".into();
                state.mark_dirty();
                ui.close();
            }
            if ui.button("Corte em Anel (Loop Cut) · Ctrl+R").clicked() {
                state.active_tool = "loop_cut".into();
                state.mark_dirty();
                ui.close();
            }
            if ui.button("Faca Topológica (Knife) · K").clicked() {
                state.active_tool = "knife".into();
                state.mark_dirty();
                ui.close();
            }
            ui.separator();
            if ui.button("Subdividir Seleção").clicked() {
                state.checkpoint("subdivide");
                if let Some(m) = state.project.active_mesh_mut() {
                    m.subdivide_selected();
                }
                state.sync_selection();
                state.emit_mesh_changed();
                ui.close();
            }
            if ui.button("Fundir no Centro (Merge)").clicked() {
                state.checkpoint("merge");
                if let Some(m) = state.project.active_mesh_mut() {
                    m.merge_center();
                }
                state.sync_selection();
                state.emit_mesh_changed();
                ui.close();
            }
        });
    }
}

/// Cluster 3: Orientação de transformação e Ponto de pivô.
fn draw_transform_cluster(ui: &mut Ui, state: &mut AppState) {
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
}

/// Cluster 4: Snapping magnético e Edição proporcional.
fn draw_snap_and_prop_cluster(ui: &mut Ui, state: &mut AppState) {
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

/// Cluster 5: Alternâncias de visualização de cena (Overlays e X-Ray).
fn draw_display_toggles_cluster(ui: &mut Ui, state: &mut AppState) {
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

/// Cluster 6: Os 4 modos canônicos de sombreamento no estilo esférico do Blender.
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

    #[test]
    fn test_mode_and_target_separation() {
        let mut state = AppState::new("en");
        // Em Object Mode, alvos de seleção de malha não devem ser alterados
        assert_eq!(state.mode, EditMode::Object);

        state.mode = EditMode::Edit;
        assert_eq!(state.mode, EditMode::Edit);
        state.select_mode = SelectMode::Edge;
        assert_eq!(state.select_mode, SelectMode::Edge);
    }
}
