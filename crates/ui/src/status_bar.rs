//! Barra de status inferior do Petunia3D (`Status Bar`).
//! Exibe dicas contextuais do mouse à esquerda, status/mensagens no centro,
//! e telemetria de malha e desempenho da cena à direita.

use egui::{vec2, Context, TopBottomPanel, Ui};
use petunia_core::AppState;
use petunia_module_model::ToolRegistry;

use crate::tokens;

/// Renderiza a barra de status inferior.
pub fn draw(ctx: &Context, state: &mut AppState, tools: &ToolRegistry) {
    let (verts, faces) = state
        .project
        .active_mesh()
        .map(|m| (m.verts.len(), m.faces.len()))
        .unwrap_or((0, 0));

    let tris = state
        .project
        .active_mesh()
        .map(|m| m.tri_count())
        .unwrap_or(0);

    let active_name = state
        .project
        .assets
        .get(state.project.active)
        .map(|o| o.name.clone())
        .unwrap_or_else(|| "None".to_string());

    let hint = tools
        .get(&state.active_tool)
        .map(|t| state.t(t.hint_key()))
        .unwrap_or_else(|| state.t("hints.select"));

    let status_text = if state.status.is_empty() {
        hint
    } else {
        state.status.clone()
    };

    TopBottomPanel::bottom("status_bar")
        .exact_height(tokens::STATUS_BAR_HEIGHT)
        .frame(
            egui::Frame::new()
                .fill(tokens::BG_HEADER)
                .stroke(tokens::stroke_border())
                .inner_margin(egui::Margin::symmetric(8, 2)),
        )
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing = vec2(6.0, 0.0);

                // 1. Dicas de Navegação do Mouse (Lado Esquerdo)
                draw_mouse_hints(ui);

                ui.separator();

                // 2. Mensagem / Feedback de Status Central
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(&status_text)
                            .size(11.0)
                            .color(tokens::TEXT_SECONDARY),
                    )
                    .truncate(),
                )
                .on_hover_text(&status_text);

                // 3. Telemetria da Cena e Malha Ativa (Lado Direito)
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let telemetry = format!(
                        "Collection | {} | Verts: {} | Faces: {} | Tris: {} | {:.1} ms",
                        active_name, verts, faces, tris, state.stats.frame_ms
                    );
                    ui.monospace(
                        egui::RichText::new(telemetry)
                            .size(10.5)
                            .color(tokens::TEXT_MUTED),
                    );

                    ui.separator();

                    // Botões compactos de Desfazer / Refazer
                    if ui
                        .add_enabled(
                            !state.is_interacting() && state.undo.can_undo(),
                            egui::Button::new(egui::RichText::new("↩").size(10.5)),
                        )
                        .on_hover_text(format!(
                            "Desfazer · Ctrl+Z · {}",
                            state.undo.undo_label().unwrap_or("")
                        ))
                        .clicked()
                    {
                        state.undo();
                    }

                    if ui
                        .add_enabled(
                            !state.is_interacting() && state.undo.can_redo(),
                            egui::Button::new(egui::RichText::new("↪").size(10.5)),
                        )
                        .on_hover_text(format!(
                            "Refazer · Ctrl+Shift+Z · {}",
                            state.undo.redo_label().unwrap_or("")
                        ))
                        .clicked()
                    {
                        state.redo();
                    }
                });
            });
        });
}

fn draw_mouse_hints(ui: &mut Ui) {
    let hints = "🖱 Select: LMB | Orbit: MMB | Pan: Shift+MMB | Context: RMB";
    ui.label(
        egui::RichText::new(hints)
            .size(10.5)
            .color(tokens::TEXT_MUTED),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_renders_without_panic() {
        let ctx = Context::default();
        let mut state = AppState::new("en");
        let tools = ToolRegistry::default();

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            draw(ctx, &mut state, &tools);
        });
    }
}
