//! Invariante da paleta lateral (§48): ela **nunca** desenha fora da própria
//! coluna.
//!
//! Era o defeito silencioso que a Wave 6 encontrou e que nenhum teste pegava: o
//! grupo de ferramentas desenhava botão + seta lado a lado com o botão já
//! ocupando a célula inteira, então a seta do menu da família (Seleção /
//! Transformação) saía do paine e ficava visível para ninguém. Em vez de um
//! breakpoint novo, o arranjo do grupo passou a ser **derivado** da célula
//! ([`petunia_ui::toolbar::plan_group`]) e o mínimo da coluna passou a ser
//! derivado da linha de grupo (`tokens::TOOLBAR_MIN_WIDTH`).
//!
//! O teste mede a largura realmente ocupada pelo conteúdo e compara com a
//! coluna: qualquer regressão que reintroduza geometria fora do paine volta a
//! falhar aqui.

use petunia_core::AppState;
use petunia_module_model::ToolRegistry;

/// Largura ocupada pelo conteúdo da paleta dentro de uma coluna de `pane_width`.
fn used_width(pane_width: f32) -> f32 {
    let ctx = egui::Context::default();
    let tools = ToolRegistry::default();
    let mut state = AppState::new("en");
    let mut used = f32::NAN;
    let mut out = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::pos2(0.0, 0.0),
                egui::vec2(pane_width, 600.0),
            )),
            ..Default::default()
        },
        |ui| {
            petunia_ui::toolbar::draw_contents(ui, &mut state, &tools);
            used = ui.min_rect().width();
        },
    );
    out.textures_delta.clear();
    used
}

#[test]
fn the_palette_never_draws_past_its_column() {
    let minimum = petunia_ui::tokens::TOOLBAR_MIN_WIDTH;
    for pane_width in [minimum, minimum + 16.0, 120.0, 180.0, 240.0] {
        let used = used_width(pane_width);
        assert!(
            used <= pane_width + 0.5,
            "coluna de {pane_width}px: conteúdo ocupa {used}px"
        );
    }
}

#[test]
fn the_palette_keeps_the_whole_column_at_the_declared_minimum() {
    // O mínimo não é só "não estoura": ele entrega a célula completa de uma
    // linha de grupo, que é o que a coluna existe para hospedar.
    let cell = petunia_ui::tokens::TOOLBAR_MIN_WIDTH - petunia_ui::tokens::TOOLBAR_CHROME_WIDTH;
    let row = petunia_ui::toolbar::plan_group(cell);
    assert!(row.chevron, "seta do menu da família visível no mínimo");
    assert!(
        row.button
            + petunia_ui::tokens::TOOLBAR_CONTROL_GAP
            + petunia_ui::tokens::TOOLBAR_CHEVRON_WIDTH
            <= cell + f32::EPSILON
    );
}
