//! Barra de contexto superior do Viewport 3D (`3D View Bar`).
//! Organizada em 7 clusters semânticos coerentes:
//! 1. Seletor de Modo (`Object` vs `Edit`) com dropdown estilizado e ícone vetorial;
//! 2. Alvos de Seleção de Malha segmentados (`Vértice`, `Aresta`, `Face`) exibidos exclusivamente em Edit Mode;
//! 3. Menus Rápidos Padronizados (`View ▾`, `Select ▾`, `Add ▾`, `Mesh ▾` / `Object ▾`) com atalhos dinâmicos;
//! 4. Orientação de Transformação, Ponto de Pivô e Travamento de Eixos [X][Y][Z];
//! 5. Auxiliares de Edição (Snapping Magnético e Edição Proporcional com ícones canônicos);
//! 6. Diagnóstico de Cena (Overlays e X-Ray com ícones vetoriais dedicados);
//! 7. 4 Esferas de Sombreamento no estilo canônico do Blender (Wireframe, Solid, Material, Rendered).

use egui::{pos2, vec2, Color32, CornerRadius, Rect, Ui};
use petunia_core::{
    AddPrimitiveCmd, AppState, ClearSelectionCmd, DeleteAssetCmd, DuplicateAssetCmd, EditMode,
    InvertSelectionCmd, MergeCenterCmd, PrimitiveKind, SelectAllCmd, SelectMode,
    SubdivideSelectionCmd,
};
use petunia_render::Shading;

use crate::icon_registry::{IconRegistry, PetuniaIcon};
use crate::tokens;
use crate::widgets::{petunia_menu_separator, PetuniaMenuItem};

/// Renderiza a barra de contexto horizontal do Viewport 3D.
pub fn draw(ui: &mut Ui, state: &mut AppState) {
    // Interceptação defensiva de atalhos globais de modo se nenhum campo de texto estiver focado
    handle_keyboard_shortcuts(ui, state);

    ui.horizontal_centered(|ui| {
        ui.spacing_mut().item_spacing = vec2(6.0, 0.0);

        // CLUSTER 1 & 2: Seletor de Modo e Alvos de Seleção Segmentados (Apenas em Edit Mode)
        draw_mode_and_targets_cluster(ui, state);

        ui.add_space(2.0);
        ui.separator();
        ui.add_space(2.0);

        // CLUSTER 3: Menus Rápidos Padronizados com Ícones (View, Select, Add, Objeto/Malha)
        draw_viewport_actions_cluster(ui, state);

        ui.add_space(2.0);
        ui.separator();
        ui.add_space(2.0);

        // CLUSTER 4: Orientação, Ponto de Pivô e Travamento de Eixos
        draw_transform_cluster(ui, state);

        ui.add_space(2.0);
        ui.separator();
        ui.add_space(2.0);

        // CLUSTER 5: Snapping Magnético e Edição Proporcional
        draw_snap_and_prop_cluster(ui, state);

        // CLUSTERS 6 & 7: Controles do lado direito (Overlays, X-Ray e 4 Esferas de Sombreamento)
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // CLUSTER 7: 4 Modos de Sombreamento no Estilo Canônico do Blender
            draw_shading_spheres_cluster(ui, state);

            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            // CLUSTER 6: Diagnóstico de Visualização (Overlays e X-Ray)
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

/// Cluster 1 & 2: Seletor de Modo (`Object` vs `Edit`) + Alvos de Seleção Segmentados
/// exibidos exclusivamente no modo de edição.
fn draw_mode_and_targets_cluster(ui: &mut Ui, state: &mut AppState) {
    let (mode_icon, mode_label) = match state.mode {
        EditMode::Object => (PetuniaIcon::ModeObject, state.t("modes.object")),
        EditMode::Edit => (PetuniaIcon::ModeEdit, state.t("modes.edit")),
        _ => (PetuniaIcon::ModeObject, "Mode".to_string()),
    };

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(4.0, 0.0);

        let (rect, _) = ui.allocate_exact_size(vec2(16.0, 16.0), egui::Sense::hover());
        if ui.is_rect_visible(rect) {
            IconRegistry::paint(
                ui.ctx(),
                ui.painter(),
                &mode_icon,
                rect,
                tokens::TEXT_PRIMARY,
            );
        }

        ui.menu_button(
            egui::RichText::new(format!("{mode_label} ▾"))
                .strong()
                .size(11.0)
                .color(tokens::TEXT_PRIMARY),
            |ui| {
                let sc_obj = state
                    .ui
                    .keybinds
                    .shortcut_for("model.select_object")
                    .unwrap_or_else(|| "0".into());
                let sc_edit = state
                    .ui
                    .keybinds
                    .shortcut_for("global.cycle_mode")
                    .unwrap_or_else(|| "Tab".into());

                if PetuniaMenuItem::new(&state.t("modes.object"))
                    .icon(PetuniaIcon::ModeObject)
                    .shortcut(Some(&sc_obj))
                    .show(ui)
                    .clicked()
                {
                    state.mode = EditMode::Object;
                    state.active_tool = "select".into();
                    state.mark_dirty();
                    ui.close();
                }
                if PetuniaMenuItem::new(&state.t("modes.edit"))
                    .icon(PetuniaIcon::ModeEdit)
                    .shortcut(Some(&sc_edit))
                    .show(ui)
                    .clicked()
                {
                    state.mode = EditMode::Edit;
                    state.sync_selection();
                    state.mark_dirty();
                    ui.close();
                }
            },
        );
    });

    // Botões de alvo de seleção segmentados: visíveis EXCLUSIVAMENTE em modo de edição
    if state.mode == EditMode::Edit {
        ui.add_space(2.0);
        let targets = [
            (
                SelectMode::Vertex,
                PetuniaIcon::SelectVertex,
                "1",
                state.t("modes.vertex"),
            ),
            (
                SelectMode::Edge,
                PetuniaIcon::SelectEdge,
                "2",
                state.t("modes.edge"),
            ),
            (
                SelectMode::Face,
                PetuniaIcon::SelectFace,
                "3",
                state.t("modes.face"),
            ),
        ];

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = vec2(2.0, 0.0);
            for (mode, icon, shortcut, name) in targets {
                let is_active = state.select_mode == mode;
                let (bg, fg) = if is_active {
                    (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
                } else {
                    (tokens::BG_SURFACE, tokens::TEXT_SECONDARY)
                };

                let (rect, resp) = ui.allocate_exact_size(vec2(24.0, 22.0), egui::Sense::click());
                if ui.is_rect_visible(rect) {
                    let painter = ui.painter();
                    let fill = if is_active {
                        bg
                    } else if resp.hovered() {
                        tokens::BG_SURFACE_HOVER
                    } else {
                        bg
                    };
                    painter.rect_filled(rect, tokens::RADIUS_CONTROL, fill);

                    let icon_rect = Rect::from_center_size(rect.center(), vec2(16.0, 16.0));
                    IconRegistry::paint(ui.ctx(), painter, &icon, icon_rect, fg);
                }

                if resp
                    .on_hover_text(format!("{name} · [{shortcut}]"))
                    .clicked()
                {
                    state.select_mode = mode;
                    state.sync_selection();
                    state.mark_dirty();
                }
            }
        });
    }
}

/// Cluster 3: Menus rápidos padronizados com ícones (View, Select, Add, Objeto/Malha).
fn draw_viewport_actions_cluster(ui: &mut Ui, state: &mut AppState) {
    let visuals = ui.visuals_mut();
    visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
    visuals.widgets.hovered.weak_bg_fill = tokens::BG_SURFACE_HOVER;
    visuals.widgets.active.weak_bg_fill = tokens::ACCENT_BLUE;

    // Menu View
    ui.menu_button("View ▾", |ui| {
        let sc_frame = state
            .ui
            .keybinds
            .shortcut_for("view.frame_selection")
            .unwrap_or_else(|| "F".into());
        if PetuniaMenuItem::new(&state.t("view.frame"))
            .shortcut(Some(&sc_frame))
            .show(ui)
            .clicked()
        {
            state.frame_selection();
            ui.close();
        }

        let sc_frame_all = state
            .ui
            .keybinds
            .shortcut_for("view.frame_all")
            .unwrap_or_else(|| "Home".into());
        if PetuniaMenuItem::new("Frame All")
            .shortcut(Some(&sc_frame_all))
            .show(ui)
            .clicked()
        {
            state.frame_all();
            ui.close();
        }

        let sc_reset = state
            .ui
            .keybinds
            .shortcut_for("view.reset_camera")
            .unwrap_or_else(|| "Shift+Home".into());
        if PetuniaMenuItem::new("Reset Camera")
            .shortcut(Some(&sc_reset))
            .show(ui)
            .clicked()
        {
            state.camera_frame = None;
            state.camera.reset();
            state.mark_dirty();
            ui.close();
        }

        petunia_menu_separator(ui);

        let sc_proj = state
            .ui
            .keybinds
            .shortcut_for("view.toggle_projection")
            .unwrap_or_else(|| "O".into());
        if PetuniaMenuItem::new(&state.t("camera.projection"))
            .shortcut(Some(&sc_proj))
            .show(ui)
            .clicked()
        {
            state.camera.toggle_projection();
            state.mark_dirty();
            ui.close();
        }

        petunia_menu_separator(ui);

        if PetuniaMenuItem::new(&state.t("camera.front"))
            .shortcut(Some("Numpad 1"))
            .show(ui)
            .clicked()
        {
            state.camera.set_preset(petunia_core::ViewPreset::Front);
            state.mark_dirty();
            ui.close();
        }
        if PetuniaMenuItem::new("Back")
            .shortcut(Some("Ctrl+Numpad 1"))
            .show(ui)
            .clicked()
        {
            state.camera.set_preset(petunia_core::ViewPreset::Back);
            state.mark_dirty();
            ui.close();
        }
        if PetuniaMenuItem::new(&state.t("camera.right"))
            .shortcut(Some("Numpad 3"))
            .show(ui)
            .clicked()
        {
            state.camera.set_preset(petunia_core::ViewPreset::Right);
            state.mark_dirty();
            ui.close();
        }
        if PetuniaMenuItem::new("Left")
            .shortcut(Some("Ctrl+Numpad 3"))
            .show(ui)
            .clicked()
        {
            state.camera.set_preset(petunia_core::ViewPreset::Left);
            state.mark_dirty();
            ui.close();
        }
        if PetuniaMenuItem::new(&state.t("camera.top"))
            .shortcut(Some("Numpad 7"))
            .show(ui)
            .clicked()
        {
            state.camera.set_preset(petunia_core::ViewPreset::Top);
            state.mark_dirty();
            ui.close();
        }
        if PetuniaMenuItem::new("Bottom")
            .shortcut(Some("Ctrl+Numpad 7"))
            .show(ui)
            .clicked()
        {
            state.camera.set_preset(petunia_core::ViewPreset::Bottom);
            state.mark_dirty();
            ui.close();
        }

        petunia_menu_separator(ui);

        ui.menu_button("Isometric ▾", |ui| {
            if PetuniaMenuItem::new("Isometric NE").show(ui).clicked() {
                state
                    .camera
                    .set_preset(petunia_core::ViewPreset::IsometricNE);
                state.mark_dirty();
                ui.close();
            }
            if PetuniaMenuItem::new("Isometric NW").show(ui).clicked() {
                state
                    .camera
                    .set_preset(petunia_core::ViewPreset::IsometricNW);
                state.mark_dirty();
                ui.close();
            }
            if PetuniaMenuItem::new("Isometric SE").show(ui).clicked() {
                state
                    .camera
                    .set_preset(petunia_core::ViewPreset::IsometricSE);
                state.mark_dirty();
                ui.close();
            }
            if PetuniaMenuItem::new("Isometric SW").show(ui).clicked() {
                state
                    .camera
                    .set_preset(petunia_core::ViewPreset::IsometricSW);
                state.mark_dirty();
                ui.close();
            }
        });

        petunia_menu_separator(ui);

        let sc_ref = state
            .ui
            .keybinds
            .shortcut_for("window.reference_manager")
            .unwrap_or_else(|| "Shift+R".into());
        if PetuniaMenuItem::new("Gerenciador de Referências...")
            .icon(PetuniaIcon::ReferenceImage)
            .shortcut(Some(&sc_ref))
            .show(ui)
            .clicked()
        {
            state.ui.show_reference_manager = true;
            state.mark_dirty();
            ui.close();
        }
    });

    // Menu Select
    ui.menu_button("Select ▾", |ui| {
        if PetuniaMenuItem::new(&state.t("actions.select_all"))
            .shortcut(Some("A"))
            .show(ui)
            .clicked()
        {
            let _ = state.dispatch(&SelectAllCmd);
            ui.close();
        }
        if PetuniaMenuItem::new(&state.t("actions.deselect"))
            .shortcut(Some("Alt+A"))
            .show(ui)
            .clicked()
        {
            let _ = state.dispatch(&ClearSelectionCmd);
            ui.close();
        }
        if PetuniaMenuItem::new(&state.t("actions.invert"))
            .shortcut(Some("Ctrl+I"))
            .show(ui)
            .clicked()
        {
            let _ = state.dispatch(&InvertSelectionCmd);
            ui.close();
        }
    });

    // Menu Add
    let mut spawn_kind: Option<PrimitiveKind> = None;
    ui.menu_button("Add ▾", |ui| {
        if PetuniaMenuItem::new(&state.t("prims.cube"))
            .icon(PetuniaIcon::AddPrimitive)
            .show(ui)
            .clicked()
        {
            spawn_kind = Some(PrimitiveKind::Cube);
            ui.close();
        }
        if PetuniaMenuItem::new(&state.t("prims.sphere"))
            .icon(PetuniaIcon::AddPrimitive)
            .show(ui)
            .clicked()
        {
            spawn_kind = Some(PrimitiveKind::Sphere);
            ui.close();
        }
        if PetuniaMenuItem::new(&state.t("prims.cylinder"))
            .icon(PetuniaIcon::AddPrimitive)
            .show(ui)
            .clicked()
        {
            spawn_kind = Some(PrimitiveKind::Cylinder);
            ui.close();
        }
        if PetuniaMenuItem::new(&state.t("prims.plane"))
            .icon(PetuniaIcon::AddPrimitive)
            .show(ui)
            .clicked()
        {
            spawn_kind = Some(PrimitiveKind::Plane);
            ui.close();
        }
        if PetuniaMenuItem::new(&state.t("prims.cone"))
            .icon(PetuniaIcon::AddPrimitive)
            .show(ui)
            .clicked()
        {
            spawn_kind = Some(PrimitiveKind::Cone);
            ui.close();
        }
        petunia_menu_separator(ui);
        if PetuniaMenuItem::new(&state.t("ui.refs"))
            .icon(PetuniaIcon::ReferenceImage)
            .shortcut(Some("Shift+R"))
            .show(ui)
            .clicked()
        {
            state.ui.show_reference_manager = true;
            state.mark_dirty();
            ui.close();
        }
    });

    if let Some(kind) = spawn_kind {
        let _ = state.dispatch(&AddPrimitiveCmd {
            kind,
            name: None,
            at_cursor: true,
        });
    }

    // Menu Contextual: Objeto (em Object Mode) ou Malha (em Edit Mode)
    if state.mode == EditMode::Object {
        ui.menu_button("Object ▾", |ui| {
            let sc_dup = state
                .ui
                .keybinds
                .shortcut_for("model.duplicate")
                .unwrap_or_else(|| "Shift+D".into());
            if PetuniaMenuItem::new(&state.t("actions.duplicate"))
                .icon(PetuniaIcon::Duplicate)
                .shortcut(Some(&sc_dup))
                .show(ui)
                .clicked()
            {
                let _ = state.dispatch(&DuplicateAssetCmd { asset_index: None });
                ui.close();
            }

            let sc_del = state
                .ui
                .keybinds
                .shortcut_for("model.delete")
                .unwrap_or_else(|| "Delete".into());
            if PetuniaMenuItem::new(&state.t("actions.delete"))
                .icon(PetuniaIcon::Delete)
                .shortcut(Some(&sc_del))
                .show(ui)
                .clicked()
            {
                let _ = state.dispatch(&DeleteAssetCmd { asset_index: None });
                ui.close();
            }
            petunia_menu_separator(ui);
            if PetuniaMenuItem::new("Cursor to World Origin")
                .icon(PetuniaIcon::Cursor3D)
                .show(ui)
                .clicked()
            {
                state.cursor_3d = [0.0, 0.0, 0.0];
                state.mark_dirty();
                ui.close();
            }
        });
    } else {
        ui.menu_button("Mesh ▾", |ui| {
            let sc_ext = state
                .ui
                .keybinds
                .shortcut_for("model.extrude")
                .unwrap_or_else(|| "E".into());
            if PetuniaMenuItem::new(&state.t("tools.extrude"))
                .icon(PetuniaIcon::Extrude)
                .shortcut(Some(&sc_ext))
                .show(ui)
                .clicked()
            {
                state.active_tool = "extrude".into();
                state.mark_dirty();
                ui.close();
            }

            let sc_ext_ind = state
                .ui
                .keybinds
                .shortcut_for("model.extrude_individual")
                .unwrap_or_else(|| "Alt+E".into());
            if PetuniaMenuItem::new("Extrude Individual")
                .icon(PetuniaIcon::Extrude)
                .shortcut(Some(&sc_ext_ind))
                .show(ui)
                .clicked()
            {
                let dist = if state.extrude_dist == 0.0 {
                    0.5
                } else {
                    state.extrude_dist
                };
                let _ = state.dispatch(&petunia_core::ExtrudeIndividualCmd { dist });
                ui.close();
            }

            let sc_ins = state
                .ui
                .keybinds
                .shortcut_for("model.inset")
                .unwrap_or_else(|| "I".into());
            if PetuniaMenuItem::new(&state.t("tools.inset"))
                .icon(PetuniaIcon::Inset)
                .shortcut(Some(&sc_ins))
                .show(ui)
                .clicked()
            {
                state.active_tool = "inset".into();
                state.mark_dirty();
                ui.close();
            }

            let sc_bev = state
                .ui
                .keybinds
                .shortcut_for("model.bevel")
                .unwrap_or_else(|| "Ctrl+B".into());
            if PetuniaMenuItem::new(&state.t("tools.bevel"))
                .icon(PetuniaIcon::Bevel)
                .shortcut(Some(&sc_bev))
                .show(ui)
                .clicked()
            {
                state.active_tool = "bevel".into();
                state.mark_dirty();
                ui.close();
            }

            if PetuniaMenuItem::new("Loop Cut")
                .icon(PetuniaIcon::LoopCut)
                .shortcut(Some("Ctrl+R"))
                .show(ui)
                .clicked()
            {
                state.active_tool = "loop_cut".into();
                state.mark_dirty();
                ui.close();
            }

            if PetuniaMenuItem::new("Knife")
                .icon(PetuniaIcon::Knife)
                .shortcut(Some("K"))
                .show(ui)
                .clicked()
            {
                state.active_tool = "knife".into();
                state.mark_dirty();
                ui.close();
            }

            petunia_menu_separator(ui);

            if PetuniaMenuItem::new(&state.t("actions.subdivide"))
                .icon(PetuniaIcon::Subdivide)
                .show(ui)
                .clicked()
            {
                let _ = state.dispatch(&SubdivideSelectionCmd);
                ui.close();
            }

            if PetuniaMenuItem::new(&state.t("actions.merge_center"))
                .icon(PetuniaIcon::Custom("merge"))
                .show(ui)
                .clicked()
            {
                let _ = state.dispatch(&MergeCenterCmd);
                ui.close();
            }

            if PetuniaMenuItem::new("Flip Diagonal")
                .icon(PetuniaIcon::Custom("flip_diagonal"))
                .show(ui)
                .clicked()
            {
                let _ = state.dispatch(&petunia_core::FlipDiagonalCmd);
                ui.close();
            }

            if PetuniaMenuItem::new("Revolve Selection")
                .icon(PetuniaIcon::Custom("revolve"))
                .show(ui)
                .clicked()
            {
                let _ = state.dispatch(&petunia_core::RevolveCmd::default());
                ui.close();
            }
        });
    }
}

/// Cluster 4: Orientação de transformação, Ponto de pivô e Travamento de Eixos.
fn draw_transform_cluster(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(3.0, 0.0);

        let (rect, _) = ui.allocate_exact_size(vec2(14.0, 14.0), egui::Sense::hover());
        if ui.is_rect_visible(rect) {
            IconRegistry::paint(
                ui.ctx(),
                ui.painter(),
                &PetuniaIcon::OrientationGlobal,
                rect,
                tokens::TEXT_SECONDARY,
            );
        }

        egui::ComboBox::from_id_salt("transform_orientation")
            .selected_text(
                egui::RichText::new(&state.transform_orientation)
                    .size(11.0)
                    .color(tokens::TEXT_PRIMARY),
            )
            .width(62.0)
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

        ui.add_space(2.0);

        let (rect, _) = ui.allocate_exact_size(vec2(14.0, 14.0), egui::Sense::hover());
        if ui.is_rect_visible(rect) {
            IconRegistry::paint(
                ui.ctx(),
                ui.painter(),
                &PetuniaIcon::PivotMedian,
                rect,
                tokens::TEXT_SECONDARY,
            );
        }

        egui::ComboBox::from_id_salt("pivot_point")
            .selected_text(
                egui::RichText::new(&state.pivot_point)
                    .size(11.0)
                    .color(tokens::TEXT_PRIMARY),
            )
            .width(88.0)
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
    });

    ui.add_space(2.0);

    // Controles e Indicador Visual de Travamento de Eixos na Barra do Viewport
    draw_axis_lock_controls(ui, state);
}

fn draw_axis_lock_controls(ui: &mut Ui, state: &mut AppState) {
    let any_locked = state.is_axis_locked(0) || state.is_axis_locked(1) || state.is_axis_locked(2);
    let lock_icon = if any_locked {
        PetuniaIcon::Lock
    } else {
        PetuniaIcon::Unlock
    };
    let lock_col = if any_locked {
        tokens::TEXT_ACTIVE
    } else {
        tokens::TEXT_MUTED
    };

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(3.0, 0.0);

        let (rect, _) = ui.allocate_exact_size(vec2(14.0, 18.0), egui::Sense::hover());
        if ui.is_rect_visible(rect) {
            let icon_rect = Rect::from_center_size(rect.center(), vec2(13.0, 13.0));
            IconRegistry::paint(ui.ctx(), ui.painter(), &lock_icon, icon_rect, lock_col);
        }

        for (axis_idx, label, color) in [
            (0, "X", tokens::AXIS_X),
            (1, "Y", tokens::AXIS_Y),
            (2, "Z", tokens::AXIS_Z),
        ] {
            let is_locked = state.is_axis_locked(axis_idx);
            let btn = if is_locked {
                egui::Button::new(
                    egui::RichText::new(label)
                        .strong()
                        .size(10.5)
                        .color(Color32::WHITE),
                )
                .fill(color)
                .corner_radius(tokens::RADIUS_CONTROL)
            } else {
                egui::Button::new(egui::RichText::new(label).size(10.5).color(color))
                    .fill(tokens::BG_SURFACE)
                    .corner_radius(tokens::RADIUS_CONTROL)
            };

            let tooltip = if is_locked {
                format!("Eixo {label} travado · Clique para destravar (ou atalho {label})")
            } else {
                format!("Travar eixo {label} na edição · Atalho {label}")
            };

            if ui.add(btn).on_hover_text(tooltip).clicked() {
                state.toggle_axis_lock(axis_idx);
            }
        }

        // Se houver restrição ativa, exibir badge estilizado
        if let Some((label, rgb)) = state.active_axis_constraint_label() {
            ui.add_space(2.0);
            let badge_bg = Color32::from_rgb(rgb[0], rgb[1], rgb[2]);
            let (rect, _) = ui.allocate_exact_size(
                vec2(20.0 + (label.len() as f32) * 6.5, 18.0),
                egui::Sense::hover(),
            );
            ui.painter()
                .rect_filled(rect, tokens::RADIUS_CONTROL, badge_bg);

            let icon_rect =
                Rect::from_min_size(pos2(rect.min.x + 3.0, rect.min.y + 2.5), vec2(13.0, 13.0));
            IconRegistry::paint(
                ui.ctx(),
                ui.painter(),
                &PetuniaIcon::Lock,
                icon_rect,
                Color32::WHITE,
            );

            ui.painter().text(
                pos2(rect.min.x + 18.0, rect.center().y),
                egui::Align2::LEFT_CENTER,
                label,
                egui::FontId::monospace(10.0),
                Color32::WHITE,
            );
        }
    });
}

/// Cluster 5: Snapping magnético e Edição proporcional com ícones canônicos.
fn draw_snap_and_prop_cluster(ui: &mut Ui, state: &mut AppState) {
    // Botão Snap (Ímã Vetorial)
    let (rect, resp) = ui.allocate_exact_size(vec2(26.0, 22.0), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let is_active = state.snap_enabled;
        let bg = if is_active {
            tokens::ACCENT_BLUE
        } else if resp.hovered() {
            tokens::BG_SURFACE_HOVER
        } else {
            tokens::BG_SURFACE
        };
        let fg = if is_active {
            tokens::TEXT_ACTIVE
        } else {
            tokens::TEXT_SECONDARY
        };
        ui.painter().rect_filled(rect, tokens::RADIUS_CONTROL, bg);
        let icon_rect = Rect::from_center_size(rect.center(), vec2(16.0, 16.0));
        IconRegistry::paint(
            ui.ctx(),
            ui.painter(),
            &PetuniaIcon::SnapMagnet,
            icon_rect,
            fg,
        );
    }
    if resp
        .on_hover_text("Snapping Magnético · Shift+Tab")
        .clicked()
    {
        state.snap_enabled = !state.snap_enabled;
        state.mark_dirty();
    }

    // Botão Edição Proporcional (Ícone Vetorial Círculos Concêntricos - Nunca 'Prop')
    let (rect, resp) = ui.allocate_exact_size(vec2(26.0, 22.0), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let is_active = state.proportional_editing;
        let bg = if is_active {
            tokens::ACCENT_BLUE
        } else if resp.hovered() {
            tokens::BG_SURFACE_HOVER
        } else {
            tokens::BG_SURFACE
        };
        let fg = if is_active {
            tokens::TEXT_ACTIVE
        } else {
            tokens::TEXT_SECONDARY
        };
        ui.painter().rect_filled(rect, tokens::RADIUS_CONTROL, bg);
        let icon_rect = Rect::from_center_size(rect.center(), vec2(16.0, 16.0));
        IconRegistry::paint(
            ui.ctx(),
            ui.painter(),
            &PetuniaIcon::ProportionalEditing,
            icon_rect,
            fg,
        );
    }
    if resp.on_hover_text("Edição Proporcional · O").clicked() {
        state.proportional_editing = !state.proportional_editing;
        state.mark_dirty();
    }
}

/// Cluster 6: Alternâncias de visualização de cena (Overlays e X-Ray com ícones vetoriais).
fn draw_display_toggles_cluster(ui: &mut Ui, state: &mut AppState) {
    // Overlays (Segmented Toggle + Popover Dropdown, P3D-010)
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(1.0, 0.0);

        let (rect, resp) = ui.allocate_exact_size(vec2(24.0, 22.0), egui::Sense::click());
        if ui.is_rect_visible(rect) {
            let is_active = state.show_overlays;
            let bg = if is_active {
                tokens::ACCENT_BLUE
            } else if resp.hovered() {
                tokens::BG_SURFACE_HOVER
            } else {
                tokens::BG_SURFACE
            };
            let fg = if is_active {
                tokens::TEXT_ACTIVE
            } else {
                tokens::TEXT_SECONDARY
            };
            ui.painter().rect_filled(
                rect,
                CornerRadius {
                    nw: 4,
                    sw: 4,
                    ne: 0,
                    se: 0,
                },
                bg,
            );
            let icon_rect = Rect::from_center_size(rect.center(), vec2(15.0, 15.0));
            IconRegistry::paint(
                ui.ctx(),
                ui.painter(),
                &PetuniaIcon::Overlays,
                icon_rect,
                fg,
            );
        }
        if resp
            .on_hover_text("Alternar Exibição de Overlays")
            .clicked()
        {
            state.show_overlays = !state.show_overlays;
            state.mark_dirty();
        }

        ui.menu_button("▾", |ui| {
            ui.set_min_width(210.0);
            ui.label(
                egui::RichText::new("Opções de Overlay")
                    .strong()
                    .size(12.0)
                    .color(tokens::TEXT_PRIMARY),
            );
            ui.separator();

            let mut dirty = false;
            if ui
                .checkbox(&mut state.show_grid, "Grade 3D (Grid)")
                .changed()
            {
                dirty = true;
            }
            if ui
                .checkbox(&mut state.show_axes, "Eixos Mundiais (Axes)")
                .changed()
            {
                dirty = true;
            }
            if ui.checkbox(&mut state.show_cursor, "Cursor 3D").changed() {
                dirty = true;
            }
            if ui
                .checkbox(
                    &mut state.show_wireframe_overlay,
                    "Aramado (Wireframe Overlay)",
                )
                .changed()
            {
                dirty = true;
            }
            if ui
                .checkbox(&mut state.show_triangulation, "Triangulação (Diagonais)")
                .changed()
            {
                dirty = true;
            }
            if ui
                .checkbox(&mut state.show_nav_hud, "Navigation HUD (Orientação)")
                .changed()
            {
                dirty = true;
            }

            ui.separator();

            if PetuniaMenuItem::new("Gerenciador de Referências...")
                .icon(PetuniaIcon::ReferenceImage)
                .shortcut(Some("Shift+R"))
                .show(ui)
                .clicked()
            {
                state.ui.show_reference_manager = true;
                dirty = true;
                ui.close();
            }

            if dirty {
                state.mark_dirty();
            }
        });
    });

    // X-Ray
    let (rect, resp) = ui.allocate_exact_size(vec2(26.0, 22.0), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let is_active = state.show_xray;
        let bg = if is_active {
            tokens::ACCENT_BLUE
        } else if resp.hovered() {
            tokens::BG_SURFACE_HOVER
        } else {
            tokens::BG_SURFACE
        };
        let fg = if is_active {
            tokens::TEXT_ACTIVE
        } else {
            tokens::TEXT_SECONDARY
        };
        ui.painter().rect_filled(rect, tokens::RADIUS_CONTROL, bg);
        let icon_rect = Rect::from_center_size(rect.center(), vec2(16.0, 16.0));
        IconRegistry::paint(ui.ctx(), ui.painter(), &PetuniaIcon::XRay, icon_rect, fg);
    }
    if resp
        .on_hover_text("Modo Raio-X / Transparência de Malha · Alt+Z")
        .clicked()
    {
        state.show_xray = !state.show_xray;
        state.mark_dirty();
    }

    // Triangulação
    let (rect, resp) = ui.allocate_exact_size(vec2(26.0, 22.0), egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let is_active = state.show_triangulation;
        let bg = if is_active {
            tokens::ACCENT_BLUE
        } else if resp.hovered() {
            tokens::BG_SURFACE_HOVER
        } else {
            tokens::BG_SURFACE
        };
        let fg = if is_active {
            tokens::TEXT_ACTIVE
        } else {
            tokens::TEXT_SECONDARY
        };
        ui.painter().rect_filled(rect, tokens::RADIUS_CONTROL, bg);
        let r = rect.shrink(5.0);
        ui.painter().line_segment(
            [pos2(r.left(), r.bottom()), pos2(r.right(), r.top())],
            egui::Stroke::new(1.5_f32, fg),
        );
        ui.painter().rect_stroke(
            r,
            1.0,
            egui::Stroke::new(1.0_f32, fg.gamma_multiply(0.5)),
            egui::StrokeKind::Inside,
        );
    }
    if resp
        .on_hover_text("Inspeção de Triangulação (Diagonais Internas de Quads/N-gons)")
        .clicked()
    {
        state.show_triangulation = !state.show_triangulation;
        state.mark_dirty();
    }
}

/// Cluster 7: Os 4 modos canônicos de sombreamento no estilo esférico do Blender com ícones vetoriais.
fn draw_shading_spheres_cluster(ui: &mut Ui, state: &mut AppState) {
    let modes = [
        (
            Shading::Wireframe,
            PetuniaIcon::ShadingWireframe,
            "Wireframe (Z 4)",
        ),
        (
            Shading::Solid,
            PetuniaIcon::ShadingSolid,
            "Solid / Clay (Z 6)",
        ),
        (
            Shading::Smooth,
            PetuniaIcon::ShadingMaterial,
            "Material Preview (Z 2)",
        ),
        (
            Shading::Unlit,
            PetuniaIcon::ShadingRendered,
            "Rendered View (Z 8)",
        ),
    ];

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(3.0, 0.0);
        for (shading, icon, hint) in modes {
            let is_active = state.shading == shading;
            let (rect, resp) = ui.allocate_exact_size(vec2(22.0, 22.0), egui::Sense::click());
            if ui.is_rect_visible(rect) {
                let bg = if is_active {
                    tokens::ACCENT_BLUE
                } else if resp.hovered() {
                    tokens::BG_SURFACE_HOVER
                } else {
                    tokens::BG_SURFACE
                };
                let fg = if is_active {
                    tokens::TEXT_ACTIVE
                } else {
                    tokens::TEXT_SECONDARY
                };
                ui.painter().rect_filled(rect, CornerRadius::same(11), bg);
                let icon_rect = Rect::from_center_size(rect.center(), vec2(16.0, 16.0));
                IconRegistry::paint(ui.ctx(), ui.painter(), &icon, icon_rect, fg);
            }
            if resp.on_hover_text(hint).clicked() {
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

    #[test]
    fn test_axis_lock_controls_render_and_badge() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");

        // 1. Renderizar com todos eixos livres
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw_axis_lock_controls(ui, &mut state);
            });
        });

        // 2. Travar eixo X e verificar renderização do badge
        state.toggle_axis_lock(0);
        assert!(state.is_axis_locked(0));
        assert_eq!(
            state.active_axis_constraint_label(),
            Some(("Eixo X", [235, 75, 75]))
        );

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw_axis_lock_controls(ui, &mut state);
            });
        });

        // 3. Travar eixo Z formando plano XZ e verificar badge
        state.toggle_axis_lock(2);
        assert_eq!(
            state.active_axis_constraint_label(),
            Some(("Plano XZ", [142, 68, 173]))
        );

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw_axis_lock_controls(ui, &mut state);
            });
        });
    }
}
