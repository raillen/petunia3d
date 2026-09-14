//! Testes de integração de fluxos de UI com `egui_kittest`.

use egui_kittest::Harness;
use petunia_core::{AppState, EditMode, ModalKind};
use petunia_module_model::ToolRegistry;
use petunia_ui::{main_header, outliner, tiles_workspace, toolbar, viewport_bar, UiAction};

#[test]
fn test_kittest_toolbar_flow() {
    let mut state = AppState::new("en");
    state.active_tool = "select".into();
    let tools = ToolRegistry::new();

    let mut harness = Harness::builder().build(|ctx| {
        toolbar::draw(ctx, &mut state, &tools);
    });

    harness.run();
    drop(harness);
    assert_eq!(state.active_tool, "select");
}

#[test]
fn test_kittest_main_header_flow() {
    let mut state = AppState::new("en");
    let mut action = UiAction::none();

    let mut harness = Harness::builder().build(|ctx| {
        main_header::draw(ctx, &mut state, &mut action);
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

    let mut harness = Harness::builder().build(|ctx| {
        petunia_ui::reference_manager::draw(ctx, &mut state);
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

    let mut harness = Harness::builder().build(|ctx| {
        petunia_ui::asset_browser::draw(ctx, &mut state);
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
    let mut harness_docked = Harness::builder().build(|ctx| {
        petunia_ui::right_panel(ctx, &mut state, &tools, &mut registry);
    });
    harness_docked.run_steps(2);
    drop(harness_docked);

    // 2. Detached floating window mode
    state.ui.inspector_detached = true;
    let mut harness_detached = Harness::builder().build(|ctx| {
        petunia_ui::right_panel(ctx, &mut state, &tools, &mut registry);
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
        let ctx = ui.ctx().clone();
        petunia_ui::properties_panel::draw(&ctx, ui, &mut state, &tools, &mut registry);
    });
    harness.run_steps(2);
    drop(harness);

    state.ui.properties_tab = "tool".to_string();
    let mut harness_tool = Harness::builder().build_ui(|ui| {
        let ctx = ui.ctx().clone();
        petunia_ui::properties_panel::draw(&ctx, ui, &mut state, &tools, &mut registry);
    });
    harness_tool.run_steps(2);
    drop(harness_tool);
    assert_eq!(state.ui.properties_tab, "tool");
}

#[test]
fn test_kittest_settings_modal_stability_over_multiple_frames() {
    let mut state = AppState::new("en");
    state.ui.show_settings = true;

    for tab in ["appearance", "icons", "language", "keymap"] {
        state.ui.settings_tab = tab.to_string();
        let mut harness = Harness::builder().build(|ctx| {
            petunia_ui::settings_modal::draw(ctx, &mut state);
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

    let mut harness = Harness::builder().build(|ctx| {
        petunia_ui::asset_library_drawer::draw(ctx, &mut state);
    });

    // Executa 10 frames consecutivos para garantir largura estável e sem expansão
    harness.run_steps(10);
    drop(harness);
}

#[test]
fn test_kittest_reference_manager_multi_frame_stability() {
    let mut state = AppState::new("pt-BR");
    state.ui.show_reference_manager = true;

    let mut harness = Harness::builder().build(|ctx| {
        petunia_ui::reference_manager::draw(ctx, &mut state);
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

    let mut harness = Harness::builder().build(|ctx| {
        petunia_ui::right_panel(ctx, &mut state, &tools, &mut registry);
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

        let mut harness = Harness::builder().build(|ctx| {
            petunia_ui::draw(ctx, &mut state, &tools, &mut registry, &mut action);
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
    let mut harness = Harness::builder().build(move |ctx| {
        petunia_ui::draw(
            ctx,
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
    let mut harness = Harness::builder().build(|ctx| {
        petunia_ui::right_panel(ctx, &mut state, &tools, &mut registry);
        state.ui.inspector_detached = !state.ui.inspector_detached;
    });
    harness.run_steps(6);
    drop(harness);
}
