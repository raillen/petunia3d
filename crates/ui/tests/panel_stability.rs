//! Regressão: nenhum painel pode "crescer sozinho" com o passar dos frames.
//!
//! O egui 0.36 grava o tamanho do painel a partir do retângulo do conteúdo;
//! conteúdo que ultrapassa a largura por frações de pixel (arredondamento de
//! `add_sized`/`allocate_ui`) alarga o dock um pouco a cada frame até o limite.
//! Este teste roda o shell real e exige larguras idênticas entre frames.

use egui_kittest::Harness;
use petunia_core::{AppState, DockOrientation, ModuleRegistry, Workspace};
use petunia_mesh::Mesh;
use petunia_module_model::ToolRegistry;

fn state_ws(ws: Workspace, orient: DockOrientation) -> AppState {
    let mut state = AppState::new("pt-BR");
    state.workspace = ws;
    state.ui.dock_orientation = orient;
    state.project.assets.clear();
    state.project.add("Cube", Mesh::cube(2.0));
    state.project.add("Vase", Mesh::cylinder(12, 0.4, 2.0));
    state.project.active = 0;
    state
}

fn widths(harness: &Harness<'_, AppState>) -> [f32; 4] {
    harness
        .ctx
        .data(|d| {
            d.get_temp::<petunia_ui::regions::UiRegions>(egui::Id::new("petunia_ui_regions"))
                .map(|r| {
                    [
                        r.right_dock.map_or(-1.0, |x| x.width()),
                        r.right_outliner.map_or(-1.0, |x| x.width()),
                        r.right_inspector.map_or(-1.0, |x| x.width()),
                        r.left_tools.map_or(-1.0, |x| x.width()),
                    ]
                })
        })
        .unwrap_or([-1.0; 4])
}

fn assert_stable(name: &str, ws: Workspace, orient: DockOrientation) {
    let state = state_ws(ws, orient);
    let tools = ToolRegistry::new();
    let mut registry = ModuleRegistry::new();
    let mut action = petunia_ui::UiAction::none();
    let mut harness = Harness::builder()
        .with_size(egui::vec2(1280.0, 800.0))
        .build_ui_state(
            |ui, state: &mut AppState| {
                petunia_ui::draw(ui, state, &tools, &mut registry, &mut action);
            },
            state,
        );
    // Alguns frames para acomodar fontes/tema, depois a medida de referência.
    for _ in 0..10 {
        harness.step();
    }
    let reference = widths(&harness);
    for frame in 0..50 {
        harness.step();
        assert_eq!(
            widths(&harness),
            reference,
            "{name}: painel mudou no frame {frame} (crescimento lento de layout)"
        );
    }
}

/// Altura da barra da viewport registrada no frame corrente.
///
/// A barra é montada pelo `egui_taffy` (Wave 4), que mede em duas passagens:
/// uma altura instável aqui significaria a barra "pulando" de linha entre
/// frames — exatamente o tipo de regressão que este arquivo existe para pegar.
fn viewport_bar_height(harness: &Harness<'_, AppState>) -> f32 {
    harness.ctx.data(|d| {
        d.get_temp::<petunia_ui::regions::UiRegions>(egui::Id::new("petunia_ui_regions"))
            .and_then(|r| r.viewport_toolbar)
            .map_or(-1.0, |x| x.height())
    })
}

#[test]
fn viewport_toolbar_height_is_stable_and_inside_the_panel_band() {
    let state = state_ws(Workspace::Model, DockOrientation::Stacked);
    let tools = ToolRegistry::new();
    let mut registry = ModuleRegistry::new();
    let mut action = petunia_ui::UiAction::none();
    let mut harness = Harness::builder()
        .with_size(egui::vec2(1280.0, 800.0))
        .build_ui_state(
            |ui, state: &mut AppState| {
                petunia_ui::draw(ui, state, &tools, &mut registry, &mut action);
            },
            state,
        );
    for _ in 0..10 {
        harness.step();
    }
    let reference = viewport_bar_height(&harness);
    assert!(
        (26.0..=56.0).contains(&reference),
        "altura da barra fora da faixa do painel: {reference}"
    );
    for frame in 0..50 {
        harness.step();
        assert_eq!(
            viewport_bar_height(&harness),
            reference,
            "barra da viewport mudou de altura no frame {frame}"
        );
    }
}

#[test]
fn dock_stays_stable_model_stacked() {
    assert_stable(
        "model/empilhado",
        Workspace::Model,
        DockOrientation::Stacked,
    );
}

#[test]
fn dock_stays_stable_model_side_by_side() {
    assert_stable(
        "model/lado-a-lado",
        Workspace::Model,
        DockOrientation::SideBySide,
    );
}

#[test]
fn dock_stays_stable_paint() {
    assert_stable(
        "paint/empilhado",
        Workspace::Paint,
        DockOrientation::Stacked,
    );
}

#[test]
fn dock_stays_stable_uv_side_by_side() {
    assert_stable("uv/lado-a-lado", Workspace::Uv, DockOrientation::SideBySide);
}
