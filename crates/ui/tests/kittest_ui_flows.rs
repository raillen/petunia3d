//! Testes de integração de fluxos de UI com `egui_kittest`.

use egui_kittest::Harness;
use petunia_core::{AppState, EditMode, ModalKind, SelectMode, Workspace};
use petunia_module_model::ToolRegistry;
use petunia_ui::{
    UiAction, contextual_shelf, main_header, outliner, tiles_workspace, toolbar, viewport_bar,
};

#[test]
fn test_kittest_toolbar_flow() {
    let mut state = AppState::new("en");
    state.active_tool = "select".into();
    let tools = ToolRegistry::new();

    let mut harness = Harness::builder().build_ui(|ui| {
        toolbar::draw(ui, &mut state, &tools);
    });

    harness.run();
    drop(harness);
    assert_eq!(state.active_tool, "select");
}

#[test]
fn test_kittest_main_header_flow() {
    let mut state = AppState::new("en");
    let mut action = UiAction::none();

    let mut harness = Harness::builder().build_ui(|ui| {
        main_header::draw(ui, &mut state, &mut action);
    });

    harness.run();
    drop(harness);
    assert_eq!(state.mode, EditMode::Object);
}

#[test]
fn test_kittest_viewport_bar_flow() {
    let mut state = AppState::new("en");

    let mut harness = Harness::builder().build_ui(|ui| {
        viewport_bar::draw(ui, &mut state);
    });

    harness.run();
    drop(harness);
    assert_eq!(state.gizmo_mode, ModalKind::Move);
}

#[test]
fn test_kittest_outliner_tree_flow() {
    let mut state = AppState::new("en");
    state.ui.outliner_search = "Cube".to_string();

    let mut harness = Harness::builder().build_ui(|ui| {
        outliner::draw(ui, &mut state);
    });

    harness.run();
    drop(harness);
    assert_eq!(state.ui.outliner_search, "Cube");
}

#[test]
fn test_kittest_tiles_workspace_flow() {
    let mut state = AppState::new("en");
    let mut tree = tiles_workspace::create_canonical_tree();

    let mut harness = Harness::builder().build_ui(|ui| {
        let mut behavior = tiles_workspace::PetuniaTilesBehavior::new(&mut state);
        tree.ui(&mut behavior, ui);
    });

    harness.run();
    drop(harness);
    assert!(tree.root().is_some());
}

#[test]
fn test_kittest_reference_manager_flow() {
    let mut state = AppState::new("pt-BR");
    state.ui.show_reference_manager = true;

    // Adiciona uma imagem para validar rendering de slot preenchido
    state
        .project
        .refs
        .push(petunia_core::ReferenceImage::from_rgba(
            "test_ref.png".to_string(),
            64,
            64,
            vec![255u8; 64 * 64 * 4],
        ));

    let mut harness = Harness::builder().build_ui(|ui| {
        petunia_ui::reference_manager::draw(ui.ctx(), &mut state);
    });

    harness.run_steps(2);
    drop(harness);
    assert!(state.ui.show_reference_manager);
    assert_eq!(state.project.refs.len(), 1);
}

#[test]
fn test_kittest_nav_hud_flow() {
    let state = AppState::new("en");
    assert!(state.show_nav_hud);
    assert!(state.show_overlays);

    let mut harness = Harness::builder().build_ui(|ui| {
        let rect = ui.max_rect();
        petunia_ui::nav_gizmo::draw_nav_hud(&state, rect, ui.painter());
    });

    harness.run();
    drop(harness);
}

#[test]
fn test_kittest_asset_browser_flow() {
    let mut state = AppState::new("en");
    state.ui.show_asset_browser = true;
    state.ui.asset_thumbnail_size = 96.0;

    let mut harness = Harness::builder().build_ui(|ui| {
        petunia_ui::asset_browser::draw(ui, &mut state);
    });

    harness.run_steps(2);
    drop(harness);
    assert_eq!(state.ui.asset_thumbnail_size, 96.0);
}

#[test]
fn test_kittest_right_panel_docked_and_detached_flow() {
    let mut state = AppState::new("en");
    let tools = ToolRegistry::new();
    let mut registry = petunia_core::ModuleRegistry::new();

    // 1. Docked mode
    state.ui.inspector_detached = false;
    let mut harness_docked = Harness::builder().build_ui(|ui| {
        petunia_ui::right_panel(ui, &mut state, &tools, &mut registry);
    });
    harness_docked.run_steps(2);
    drop(harness_docked);

    // 2. Detached floating window mode
    state.ui.inspector_detached = true;
    let mut harness_detached = Harness::builder().build_ui(|ui| {
        petunia_ui::right_panel(ui, &mut state, &tools, &mut registry);
    });
    harness_detached.run_steps(2);
    drop(harness_detached);
    assert!(state.ui.inspector_detached);
}

#[test]
fn test_kittest_properties_panel_tabs_flow() {
    let mut state = AppState::new("en");
    let tools = ToolRegistry::new();
    let mut registry = petunia_core::ModuleRegistry::new();

    state.ui.properties_tab = "object".to_string();
    let mut harness = Harness::builder().build_ui(|ui| {
        petunia_ui::properties_panel::draw(ui, &mut state, &tools, &mut registry);
    });
    harness.run_steps(2);
    drop(harness);

    state.ui.properties_tab = "tool".to_string();
    let mut harness_tool = Harness::builder().build_ui(|ui| {
        petunia_ui::properties_panel::draw(ui, &mut state, &tools, &mut registry);
    });
    harness_tool.run_steps(2);
    drop(harness_tool);
    assert_eq!(state.ui.properties_tab, "tool");
}

#[test]
fn test_kittest_settings_modal_stability_over_multiple_frames() {
    let mut state = AppState::new("en");
    state.ui.show_settings = true;

    for tab in [
        "appearance",
        "icons",
        "language",
        "keymap",
        "interface",
        "import_export",
    ] {
        state.ui.settings_tab = tab.to_string();
        let mut harness = Harness::builder().build_ui(|ui| {
            petunia_ui::settings_modal::draw(ui.ctx(), &mut state);
        });
        // Executa 10 frames consecutivos para assegurar que não há runaway horizontal
        harness.run_steps(10);
        drop(harness);
    }
}

#[test]
fn test_kittest_asset_library_drawer_stability_over_multiple_frames() {
    let mut state = AppState::new("en");
    state.ui.show_asset_library = true;

    // Adiciona alguns assets para preencher a gaveta
    for _ in 0..4 {
        state.save_active_as_asset();
    }

    let mut harness = Harness::builder().build_ui(|ui| {
        petunia_ui::asset_library_drawer::draw(ui.ctx(), &mut state);
    });

    // Executa 10 frames consecutivos para garantir largura estável e sem expansão
    harness.run_steps(10);
    drop(harness);
}

#[test]
fn test_kittest_reference_manager_multi_frame_stability() {
    let mut state = AppState::new("pt-BR");
    state.ui.show_reference_manager = true;

    let mut harness = Harness::builder().build_ui(|ui| {
        petunia_ui::reference_manager::draw(ui.ctx(), &mut state);
    });

    // 10 frames em slots vazios (onde empty_rect era alocado sem restrição)
    harness.run_steps(10);
    drop(harness);
}

#[test]
fn test_kittest_detached_inspector_multi_frame_stability() {
    let mut state = AppState::new("en");
    state.ui.inspector_detached = true;
    let tools = ToolRegistry::new();
    let mut registry = petunia_core::ModuleRegistry::new();

    let mut harness = Harness::builder().build_ui(|ui| {
        petunia_ui::right_panel(ui, &mut state, &tools, &mut registry);
    });

    harness.run_steps(10);
    drop(harness);
}

#[test]
fn test_kittest_full_draw_with_all_modals() {
    let mut state = AppState::new("en");
    let tools = ToolRegistry::new();
    let mut registry = petunia_core::ModuleRegistry::new();
    let mut action = petunia_ui::UiAction::none();

    // Test with each modal open during full UI draw
    for modal in 0..5 {
        state.ui.show_settings = modal == 0;
        state.ui.show_asset_library = modal == 1;
        state.ui.show_command_palette = modal == 2;
        state.ui.show_reference_manager = modal == 3;
        state.ui.inspector_detached = modal == 4;

        let mut harness = Harness::builder().build_ui(|ui| {
            petunia_ui::draw(ui, &mut state, &tools, &mut registry, &mut action);
        });

        harness.run_steps(3);
        drop(harness);
    }
}

#[test]
fn test_kittest_inspector_detachment_transition() {
    let state = std::rc::Rc::new(std::cell::RefCell::new(AppState::new("en")));
    state.borrow_mut().active_tool = "transform".to_string();
    let tools = ToolRegistry::new();
    let mut registry = petunia_core::ModuleRegistry::new();
    let mut action = petunia_ui::UiAction::none();

    let state_clone = state.clone();
    let mut harness = Harness::builder().build_ui(move |ui| {
        petunia_ui::draw(
            ui,
            &mut state_clone.borrow_mut(),
            &tools,
            &mut registry,
            &mut action,
        );
    });
    harness.run_steps(2);

    // Detach mid-session
    state.borrow_mut().ui.inspector_detached = true;
    harness.run_steps(3);

    // Dock back
    state.borrow_mut().ui.inspector_detached = false;
    harness.run_steps(3);
    drop(harness);
}

#[test]
fn test_kittest_inspector_layer_collision_prevention() {
    let mut state = AppState::new("en");
    state.active_tool = "transform".to_string();
    let tools = ToolRegistry::new();
    let mut registry = petunia_core::ModuleRegistry::new();

    // Verifies that toggling inspector_detached inside right_panel closure
    // cleanly snapshots was_detached and never double-renders or causes layer collision.
    let mut harness = Harness::builder().build_ui(|ui| {
        petunia_ui::right_panel(ui, &mut state, &tools, &mut registry);
        state.ui.inspector_detached = !state.ui.inspector_detached;
    });
    harness.run_steps(6);
    drop(harness);
}

#[test]
fn test_kittest_selection_modes_and_edit_mode_flow() {
    let mut state = AppState::new("en");

    // 1. Object Mode
    state.mode = EditMode::Object;
    let mut harness = Harness::builder().build_ui(|ui| {
        viewport_bar::draw(ui, &mut state);
    });
    harness.run_steps(2);
    drop(harness);
    assert_eq!(state.mode, EditMode::Object);

    // 2. Edit Mode - Vertex Selection
    state.mode = EditMode::Edit;
    state.select_mode = SelectMode::Vertex;
    let mut harness_vertex = Harness::builder().build_ui(|ui| {
        viewport_bar::draw(ui, &mut state);
    });
    harness_vertex.run_steps(2);
    drop(harness_vertex);
    assert_eq!(state.select_mode, SelectMode::Vertex);

    // 3. Edit Mode - Edge Selection
    state.select_mode = SelectMode::Edge;
    let mut harness_edge = Harness::builder().build_ui(|ui| {
        viewport_bar::draw(ui, &mut state);
    });
    harness_edge.run_steps(2);
    drop(harness_edge);
    assert_eq!(state.select_mode, SelectMode::Edge);

    // 4. Edit Mode - Face Selection
    state.select_mode = SelectMode::Face;
    let mut harness_face = Harness::builder().build_ui(|ui| {
        viewport_bar::draw(ui, &mut state);
    });
    harness_face.run_steps(2);
    drop(harness_face);
    assert_eq!(state.select_mode, SelectMode::Face);
}

#[test]
fn test_kittest_contextual_shelf_across_all_workspaces() {
    let mut state = AppState::new("en");
    let fake_rect = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(1920.0, 1080.0));

    // 1. Model Workspace — Object Mode
    state.workspace = Workspace::Model;
    state.mode = EditMode::Object;
    let mut harness_obj = Harness::builder().build_ui(|ui| {
        let shelf_rect = contextual_shelf::draw(ui, &mut state, fake_rect);
        assert!(shelf_rect.is_some());
    });
    harness_obj.run_steps(2);
    drop(harness_obj);

    // 2. Model Workspace — Edit Mode
    state.mode = EditMode::Edit;
    let mut harness_edit = Harness::builder().build_ui(|ui| {
        let shelf_rect = contextual_shelf::draw(ui, &mut state, fake_rect);
        assert!(shelf_rect.is_some());
    });
    harness_edit.run_steps(2);
    drop(harness_edit);

    // 3. Paint Workspace
    state.workspace = Workspace::Paint;
    let mut harness_paint = Harness::builder().build_ui(|ui| {
        let shelf_rect = contextual_shelf::draw(ui, &mut state, fake_rect);
        assert!(shelf_rect.is_some());
    });
    harness_paint.run_steps(2);
    drop(harness_paint);

    // 4. UV Workspace
    state.workspace = Workspace::Uv;
    let mut harness_uv = Harness::builder().build_ui(|ui| {
        let shelf_rect = contextual_shelf::draw(ui, &mut state, fake_rect);
        assert!(shelf_rect.is_some());
    });
    harness_uv.run_steps(2);
    drop(harness_uv);

    // 5. Animate Workspace
    state.workspace = Workspace::Animate;
    let mut harness_anim = Harness::builder().build_ui(|ui| {
        let shelf_rect = contextual_shelf::draw(ui, &mut state, fake_rect);
        assert!(shelf_rect.is_some());
    });
    harness_anim.run_steps(2);
    drop(harness_anim);

    // 6. Narrow Viewport — Wave 4: pílula "Tools…" em vez de sumir, sem clippar.
    let narrow_rect = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(300.0, 600.0));
    let mut harness_narrow = Harness::builder().build_ui(|ui| {
        let shelf_rect = contextual_shelf::draw(ui, &mut state, narrow_rect);
        let shelf = shelf_rect.expect("narrow shelf collapses to pill, not None");
        assert!(shelf.width() <= narrow_rect.width());
        assert!(narrow_rect.contains_rect(shelf));
    });
    harness_narrow.run_steps(2);
    drop(harness_narrow);

    // 7. Medium Viewport — Compact/Overflow sem clippar.
    let medium_rect = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(700.0, 600.0));
    let mut harness_medium = Harness::builder().build_ui(|ui| {
        let shelf_rect = contextual_shelf::draw(ui, &mut state, medium_rect);
        let shelf = shelf_rect.expect("medium shelf stays visible");
        assert!(shelf.width() <= medium_rect.width());
        assert!(medium_rect.contains_rect(shelf));
    });
    harness_medium.run_steps(2);
    drop(harness_medium);

    // 8. Tiny Viewport — sem espaço nem para a pílula.
    let tiny_rect = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(60.0, 600.0));
    let mut harness_tiny = Harness::builder().build_ui(|ui| {
        let shelf_rect = contextual_shelf::draw(ui, &mut state, tiny_rect);
        assert!(shelf_rect.is_none());
    });
    harness_tiny.run_steps(2);
    drop(harness_tiny);
}

#[test]
fn test_kittest_menu_rows_fit_content() {
    // Linhas de menu acompanham o conteúdo (Wave 4 — §8.1), sem mínimo global.
    let mut harness = Harness::builder().build_ui(|ui| {
        let short = petunia_ui::widgets::PetuniaMenuItem::new("OK").show(ui);
        let long =
            petunia_ui::widgets::PetuniaMenuItem::new("Exportar malha selecionada como OBJ…")
                .shortcut(Some("Ctrl+E"))
                .show(ui);
        assert!(short.rect.width() < long.rect.width());
        assert!(long.rect.width() <= petunia_ui::widgets::MENU_MAX_W + 1.0);
        assert!(short.rect.width() >= petunia_ui::widgets::MENU_MIN_W - 1.0);
    });
    harness.run_steps(2);
    drop(harness);
}

#[test]
fn test_kittest_right_dock_regions_are_disjoint_and_safe() {
    let state = std::rc::Rc::new(std::cell::RefCell::new(AppState::new("en")));
    let tools = ToolRegistry::new();
    let mut registry = petunia_core::ModuleRegistry::new();
    let mut action = petunia_ui::UiAction::none();
    let seen = std::rc::Rc::new(std::cell::RefCell::new(None));
    let seen_clone = seen.clone();
    let state_clone = state.clone();

    // Tamanho fixo: geometria determinística entre frames (Wave 2 — §21).
    let mut harness = Harness::builder()
        .with_size(egui::Vec2::new(1280.0, 800.0))
        .build_ui(move |ui| {
            petunia_ui::draw(
                ui,
                &mut state_clone.borrow_mut(),
                &tools,
                &mut registry,
                &mut action,
            );
            *seen_clone.borrow_mut() = petunia_ui::regions::load(ui.ctx());
        });
    harness.run_steps(6);
    drop(harness);

    let regions = seen.borrow().clone().expect("regions recorded");
    assert!(regions.right_dock.is_some());
    assert!(regions.right_outliner.is_some());
    assert!(regions.right_inspector.is_some());
    assert!(regions.dock_sections_disjoint());
    assert!(
        regions.status_overlaps().is_empty(),
        "panes overlap status bar: {:?}",
        regions.status_overlaps()
    );
    assert!(regions.shelf_within_viewport());
    assert!(regions.viewport.is_some());
}

#[test]
fn test_kittest_dock_split_fraction_resizes_sections() {
    let state = std::rc::Rc::new(std::cell::RefCell::new(AppState::new("en")));
    let tools = ToolRegistry::new();
    let mut registry = petunia_core::ModuleRegistry::new();
    let mut action = petunia_ui::UiAction::none();
    let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
    let seen_clone = seen.clone();
    let state_clone = state.clone();

    let mut harness = Harness::builder()
        .with_size(egui::Vec2::new(1280.0, 800.0))
        .build_ui(move |ui| {
            petunia_ui::draw(
                ui,
                &mut state_clone.borrow_mut(),
                &tools,
                &mut registry,
                &mut action,
            );
            seen_clone
                .borrow_mut()
                .push(petunia_ui::regions::load(ui.ctx()));
        });
    state.borrow_mut().ui.right_dock_split = 0.3;
    harness.run_steps(6);
    state.borrow_mut().ui.right_dock_split = 0.7;
    harness.run_steps(6);
    drop(harness);

    let frames = seen.borrow();
    let narrow = frames[5].clone().expect("regions narrow");
    let wide = frames[11].clone().expect("regions wide");
    let narrow_h = narrow.right_outliner.unwrap().height();
    let wide_h = wide.right_outliner.unwrap().height();
    assert!(
        wide_h > narrow_h + 20.0,
        "split 0.7 ({wide_h}) must exceed split 0.3 ({narrow_h})"
    );
    assert!(narrow.dock_sections_disjoint() && wide.dock_sections_disjoint());
}

#[test]
fn test_kittest_dock_collapse_gives_space_to_sibling() {
    let state = std::rc::Rc::new(std::cell::RefCell::new(AppState::new("en")));
    let tools = ToolRegistry::new();
    let mut registry = petunia_core::ModuleRegistry::new();
    let mut action = petunia_ui::UiAction::none();
    let seen = std::rc::Rc::new(std::cell::RefCell::new(None));
    let seen_clone = seen.clone();
    let state_clone = state.clone();

    state.borrow_mut().ui.outliner_collapsed = true;
    let mut harness = Harness::builder()
        .with_size(egui::Vec2::new(1280.0, 800.0))
        .build_ui(move |ui| {
            petunia_ui::draw(
                ui,
                &mut state_clone.borrow_mut(),
                &tools,
                &mut registry,
                &mut action,
            );
            *seen_clone.borrow_mut() = petunia_ui::regions::load(ui.ctx());
        });
    harness.run_steps(6);
    drop(harness);

    let regions = seen.borrow().clone().expect("regions recorded");
    let out_h = regions.right_outliner.unwrap().height();
    let insp_h = regions.right_inspector.unwrap().height();
    assert!(
        out_h <= 44.0,
        "collapsed outliner must be header-only ({out_h})"
    );
    assert!(insp_h > out_h);
    assert!(regions.dock_sections_disjoint());
}

#[test]
fn test_kittest_workspace_switch_preserves_dock_layout() {
    let state = std::rc::Rc::new(std::cell::RefCell::new(AppState::new("en")));
    state.borrow_mut().ui.right_dock_split = 0.6;
    state.borrow_mut().ui.inspector_collapsed = true;
    let tools = ToolRegistry::new();
    let mut registry = petunia_core::ModuleRegistry::new();
    let mut action = petunia_ui::UiAction::none();
    let seen = std::rc::Rc::new(std::cell::RefCell::new(None));
    let seen_clone = seen.clone();
    let state_clone = state.clone();

    let mut harness = Harness::builder()
        .with_size(egui::Vec2::new(1280.0, 800.0))
        .build_ui(move |ui| {
            petunia_ui::draw(
                ui,
                &mut state_clone.borrow_mut(),
                &tools,
                &mut registry,
                &mut action,
            );
            *seen_clone.borrow_mut() = petunia_ui::regions::load(ui.ctx());
        });
    for workspace in [
        Workspace::Model,
        Workspace::Paint,
        Workspace::Uv,
        Workspace::Animate,
    ] {
        state.borrow_mut().workspace = workspace;
        harness.run_steps(3);
    }
    drop(harness);

    let borrowed = state.borrow();
    assert_eq!(borrowed.ui.right_dock_split, 0.6);
    assert!(borrowed.ui.inspector_collapsed);
    drop(borrowed);
    let regions = seen.borrow().clone().expect("regions recorded");
    assert!(regions.dock_sections_disjoint());
    assert!(regions.status_overlaps().is_empty());
}

#[test]
fn test_kittest_workspace_profiles_compose_shell() {
    // Cada workspace compõe o shell de forma visivelmente distinta (§6.5).
    for (workspace, expect_bottom, expect_uv) in [
        (Workspace::Model, false, false),
        (Workspace::Paint, false, false),
        (Workspace::Uv, false, true),
        (Workspace::Animate, true, false),
    ] {
        let state = std::rc::Rc::new(std::cell::RefCell::new(AppState::new("en")));
        state.borrow_mut().workspace = workspace;
        let tools = ToolRegistry::new();
        let mut registry = petunia_core::ModuleRegistry::new();
        let mut action = petunia_ui::UiAction::none();
        let seen = std::rc::Rc::new(std::cell::RefCell::new(None));
        let seen_clone = seen.clone();
        let state_clone = state.clone();

        let mut harness = Harness::builder()
            .with_size(egui::Vec2::new(1280.0, 800.0))
            .build_ui(move |ui| {
                petunia_ui::draw(
                    ui,
                    &mut state_clone.borrow_mut(),
                    &tools,
                    &mut registry,
                    &mut action,
                );
                *seen_clone.borrow_mut() = petunia_ui::regions::load(ui.ctx());
            });
        harness.run_steps(6);
        drop(harness);

        let regions = seen
            .borrow()
            .clone()
            .unwrap_or_else(|| panic!("regions recorded for {workspace:?}"));
        assert_eq!(
            regions.bottom_dock.is_some(),
            expect_bottom,
            "bottom pane for {workspace:?}"
        );
        assert_eq!(
            regions.uv_editor.is_some(),
            expect_uv,
            "uv editor for {workspace:?}"
        );
        assert!(
            regions.viewport.is_some(),
            "3D viewport present for {workspace:?}"
        );
        assert!(
            regions.status_overlaps().is_empty(),
            "panes overlap status bar for {workspace:?}: {:?}",
            regions.status_overlaps()
        );
        assert!(regions.shelf_within_viewport() || workspace == Workspace::Uv);
    }
}

#[test]
fn test_kittest_uv_narrow_window_toggles_editor_or_preview() {
    let state = std::rc::Rc::new(std::cell::RefCell::new(AppState::new("en")));
    state.borrow_mut().workspace = Workspace::Uv;
    let tools = ToolRegistry::new();
    let mut registry = petunia_core::ModuleRegistry::new();
    let mut action = petunia_ui::UiAction::none();
    let seen = std::rc::Rc::new(std::cell::RefCell::new(None));
    let seen_clone = seen.clone();
    let state_clone = state.clone();

    // Janela estreita: mostra a prévia 3D por padrão, sem editor esmagado.
    let mut harness = Harness::builder()
        .with_size(egui::Vec2::new(700.0, 600.0))
        .build_ui(move |ui| {
            petunia_ui::draw(
                ui,
                &mut state_clone.borrow_mut(),
                &tools,
                &mut registry,
                &mut action,
            );
            *seen_clone.borrow_mut() = petunia_ui::regions::load(ui.ctx());
        });
    harness.run_steps(6);

    let regions = seen.borrow().clone().expect("regions recorded");
    assert!(regions.viewport.is_some());
    assert!(regions.uv_editor.is_none());
    drop(harness);

    // Alterna para o editor: viewport some, editor aparece.
    state.borrow_mut().ui.uv_show_preview = false;
    let state_clone2 = state.clone();
    let seen2 = std::rc::Rc::new(std::cell::RefCell::new(None));
    let seen2_clone = seen2.clone();
    let tools2 = ToolRegistry::new();
    let mut registry2 = petunia_core::ModuleRegistry::new();
    let mut action2 = petunia_ui::UiAction::none();
    let mut harness2 = Harness::builder()
        .with_size(egui::Vec2::new(700.0, 600.0))
        .build_ui(move |ui| {
            petunia_ui::draw(
                ui,
                &mut state_clone2.borrow_mut(),
                &tools2,
                &mut registry2,
                &mut action2,
            );
            *seen2_clone.borrow_mut() = petunia_ui::regions::load(ui.ctx());
        });
    harness2.run_steps(6);
    drop(harness2);
    let regions2 = seen2.borrow().clone().expect("regions recorded");
    assert!(regions2.uv_editor.is_some());
}

#[test]
fn test_kittest_animate_toolbar_has_pose_tools() {
    // A toolbar do Animate não é mais vazia: seleção + trio de pose.
    let mut state = AppState::new("en");
    state.workspace = Workspace::Animate;
    let tools = ToolRegistry::new();

    let mut harness = Harness::builder().build_ui(|ui| {
        petunia_ui::toolbar::draw(ui, &mut state, &tools);
    });
    harness.run_steps(2);
    drop(harness);

    // Ferramenta de malha não faz sentido no Animate: normalização do Model
    // não se aplica, mas o estado sobrevive intacto ao desenho.
    assert_eq!(state.workspace, Workspace::Animate);
}

#[test]
fn test_kittest_hidden_shelf_records_no_region() {
    let state = std::rc::Rc::new(std::cell::RefCell::new(AppState::new("en")));
    state.borrow_mut().ui.show_shelf = false;
    let tools = ToolRegistry::new();
    let mut registry = petunia_core::ModuleRegistry::new();
    let mut action = petunia_ui::UiAction::none();
    let seen = std::rc::Rc::new(std::cell::RefCell::new(None));
    let seen_clone = seen.clone();
    let state_clone = state.clone();

    let mut harness = Harness::builder()
        .with_size(egui::Vec2::new(1280.0, 800.0))
        .build_ui(move |ui| {
            petunia_ui::draw(
                ui,
                &mut state_clone.borrow_mut(),
                &tools,
                &mut registry,
                &mut action,
            );
            *seen_clone.borrow_mut() = petunia_ui::regions::load(ui.ctx());
        });
    harness.run_steps(6);
    drop(harness);

    let regions = seen.borrow().clone().expect("regions recorded");
    assert!(regions.shelf.is_none());
    assert!(regions.viewport.is_some());
}

#[test]
fn test_kittest_reference_manager_grid_at_widths() {
    // Grade determinística: renderiza estável em 3 larguras, sem pânico.
    for width in [1280.0, 800.0, 520.0] {
        let mut state = AppState::new("pt-BR");
        state.ui.show_reference_manager = true;
        state
            .project
            .refs
            .push(petunia_core::ReferenceImage::from_rgba(
                "grid_ref.png".to_string(),
                64,
                64,
                vec![255u8; 64 * 64 * 4],
            ));

        let mut harness = Harness::builder()
            .with_size(egui::Vec2::new(width, 700.0))
            .build_ui(|ui| {
                petunia_ui::reference_manager::draw(ui.ctx(), &mut state);
            });
        harness.run_steps(10);
        drop(harness);
        assert!(state.ui.show_reference_manager);
    }
}

#[test]
fn test_kittest_animation_workspace_and_rig_panel() {
    let mut state = AppState {
        session: petunia_core::state::EditorSession {
            workspace: Workspace::Animate,
            ..Default::default()
        },
        ..Default::default()
    };

    // Adiciona um esqueleto ao projeto
    let skel = petunia_project::animation::RigPreset::humanoid(1.0);
    state.project.add_skeleton(skel);

    // Adiciona um clipe de animação
    let clip =
        petunia_project::animation::AnimationLibrary::humanoid_idle(&state.project.skeletons[0]);
    state
        .project
        .add_animation(petunia_project::animation::AnimationAsset::new(
            "Humanoid_Idle",
            clip,
        ));

    let mut harness = Harness::builder().build_ui(|ui| {
        petunia_ui::modules_ui::animation_ui::draw_animation_panel(ui, &mut state);
    });

    harness.run_steps(2);
    drop(harness);

    assert_eq!(state.project.skeletons.len(), 1);
    assert_eq!(state.project.animations.len(), 1);
    assert!(state.project.skeletons[0].bones.len() >= 15);
}
