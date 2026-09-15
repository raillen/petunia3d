//! Fluxos da sidebar direita redesenhada (Scene + Inspector contextual).
//!
//! Interações reais via `egui_kittest` headless: busca, teclado na árvore,
//! rename, pin e undo por gesto. O estado vive dentro do harness
//! (`build_ui_state`) para permitir asserts entre passos.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use petunia_core::{AppState, ModuleRegistry, Workspace};
use petunia_mesh::Mesh;
use petunia_module_model::ToolRegistry;

fn state_with_two_cubes() -> AppState {
    let mut state = AppState::new("en");
    state.workspace = Workspace::Model;
    state.project.assets.clear();
    state.project.add("Cube", Mesh::cube(2.0));
    state.project.add("Wedge", Mesh::cube(1.0));
    state.project.active = 0;
    state
}

#[test]
fn scene_search_filters_tree_without_losing_selection() {
    let mut state = state_with_two_cubes();
    state.ui.outliner_search = "wedge".to_string();

    let mut harness = Harness::builder().build_ui_state(
        |ui, state: &mut AppState| {
            petunia_ui::outliner::draw_body(ui, state);
        },
        state,
    );
    harness.run();

    // "Cube" filtrado some; "Wedge" permanece; ativo intacto.
    assert!(harness.query_by_label("Cube").is_none());
    assert!(harness.query_by_label("Wedge").is_some());
    assert_eq!(harness.state().project.active, 0);
}

#[test]
fn tree_arrow_down_moves_active_object() {
    let state = state_with_two_cubes();

    let mut harness = Harness::builder().build_ui_state(
        |ui, state: &mut AppState| {
            petunia_ui::outliner::draw_body(ui, state);
        },
        state,
    );
    harness.run();
    assert_eq!(harness.state().project.active, 0);

    // Clica a linha (seleciona + foca a árvore) e desce: move a seleção.
    harness.get_by_label("Cube").click();
    harness.run();
    harness.key_press(egui::Key::ArrowDown);
    harness.run();

    assert_eq!(
        harness.state().project.active,
        1,
        "seta para baixo deveria ativar o próximo objeto"
    );
}

#[test]
fn inspector_pin_survives_selection_change() {
    let state = state_with_two_cubes();
    let tools = ToolRegistry::new();
    let mut registry = ModuleRegistry::new();
    let pinned_id = state.project.assets[0].id;

    let mut harness = Harness::builder().build_ui_state(
        |ui, state: &mut AppState| {
            petunia_ui::properties_panel::draw(ui, state, &tools, &mut registry);
        },
        state,
    );
    harness.run();

    // Fixa via botão (tooltip acessível) e troca o ativo: inspecionado fica.
    harness.get_by_label_contains("Pin Inspector").click();
    harness.run();
    assert_eq!(harness.state().ui.inspector_pinned, Some(pinned_id));

    harness.state_mut().project.active = 1;
    harness.run();
    assert_eq!(
        petunia_ui::inspector_context::inspected_asset_idx(harness.state()),
        Some(0),
        "pin deveria segurar o Cube"
    );
}

#[test]
fn rename_asset_via_keyboard() {
    let state = state_with_two_cubes();

    let mut harness = Harness::builder().build_ui_state(
        |ui, state: &mut AppState| {
            petunia_ui::outliner::draw_body(ui, state);
        },
        state,
    );
    harness.run();

    // Enter na linha inicia o rename; digita e confirma.
    harness.get_by_label("Cube").click();
    harness.run();
    harness.key_press(egui::Key::Enter);
    harness.run();
    // Assenta o foco no campo antes de digitar.
    harness.run();
    for ch in "Renamed".chars() {
        harness.event(egui::Event::Text(ch.into()));
    }
    harness.run();
    harness.key_press(egui::Key::Enter);
    harness.run();

    assert_eq!(harness.state().project.assets[0].name, "Renamed");
}

#[test]
fn transform_edit_is_single_undo_level() {
    let mut state = state_with_two_cubes();
    let base = state.project.undo.depth().0;
    // Simula um gesto de scrub: N mudanças dentro de uma sessão.
    let session = petunia_ui::inspector_widgets::EditSession::new("probe");
    let ctx = egui::Context::default();
    ctx.run_ui(egui::RawInput::default(), |ui| {
        let resp = ui.allocate_response(egui::vec2(10.0, 10.0), egui::Sense::click());
        for _ in 0..5 {
            session.poll(ui, &mut state, &resp, "probe edit", true);
        }
        session.finish(ui, true);
    })
    .textures_delta
    .clear();
    assert_eq!(state.project.undo.depth().0, base + 1);
}

#[test]
fn inspector_tabs_and_sections_layout_metrics() {
    for (w, h) in [(1280.0, 800.0), (900.0, 650.0)] {
        let state = state_with_two_cubes();
        let tools = ToolRegistry::new();
        let mut registry = ModuleRegistry::new();
        let mut harness = Harness::builder()
            .with_size(egui::vec2(w, h))
            .build_ui_state(
                |ui, state: &mut AppState| {
                    petunia_ui::right_panel(ui, state, &tools, &mut registry);
                },
                state,
            );
        harness.run();

        // Abas na mesma linha, ordenadas, sem sobreposição, altura ≥ 28.
        let rows: Vec<egui::Rect> = ["Object", "Modify", "Material"]
            .iter()
            .map(|label| harness.get_by_label(label).rect())
            .collect();
        for rect in &rows {
            assert!(
                rect.height() >= 28.0 - 0.5,
                "aba com altura insuficiente: {rect:?}"
            );
        }
        assert!(
            (rows[0].min.y - rows[1].min.y).abs() < 2.0
                && (rows[1].min.y - rows[2].min.y).abs() < 2.0,
            "abas fora da mesma linha: {rows:?}"
        );
        assert!(
            rows[0].max.x <= rows[1].min.x + 0.5 && rows[1].max.x <= rows[2].min.x + 0.5,
            "abas sobrepostas ou fora de ordem: {rows:?}"
        );
        // Seções alcançáveis por acessibilidade.
        assert!(harness.query_by_label("Transform").is_some());
        assert!(harness.query_by_label("Geometry").is_some());
    }
}
