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
