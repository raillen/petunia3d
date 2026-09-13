//! Barra de status inferior do Petunia3D (`Status Bar`).
//! Estruturada em 3 blocos bem definidos:
//! 1. Identidade do Projeto (`● Salvo` / `○ Não salvo`), nome do arquivo e atalhos contextuais do mouse/ferramenta;
//! 2. Mensagem central de status / feedback do sistema;
//! 3. Telemetria agregada de cena (Tris, Vértices, Objetos, Frame Time) e botões de histórico (Undo/Redo).

use egui::{vec2, Align, Context, Layout, RichText, TopBottomPanel};
use petunia_core::AppState;
use petunia_module_model::ToolRegistry;

use crate::tokens;

/// Renderiza a barra de status inferior estruturada em 3 blocos.
pub fn draw(ctx: &Context, state: &mut AppState, tools: &ToolRegistry) {
    let proj_name = state
        .project_path
        .as_deref()
        .map(|p| {
            std::path::Path::new(p)
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or(p)
                .to_string()
        })
        .unwrap_or_else(|| "Sem Título".to_string());

    let is_saved = !state.dirty;
    let (save_indicator, save_color) = if is_saved {
        ("● Salvo", tokens::ACCENT_GREEN)
    } else {
        ("○ Não salvo", tokens::ACCENT_AMBER)
    };

    let hint = if state.is_interacting() {
        "Enter / LMB: Confirmar · Esc / RMB: Cancelar".to_string()
    } else {
        tools
            .get(&state.active_tool)
            .map(|t| state.t(t.hint_key()))
            .unwrap_or_else(|| "🖱 LMB: Selecionar · MMB: Orbitar · Shift+MMB: Pan".to_string())
    };

    let status_text = if state.status.is_empty() {
        hint.clone()
    } else {
        state.status.clone()
    };

    let scene_tris = state.scene_tris();
    let scene_verts = state.scene_verts();
    let obj_count = state.project.assets.len();
    let frame_ms = state.stats.frame_ms;

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

                // BLOCO 1: Identidade do Projeto e Dicas Contextuais (Esquerda)
                ui.label(
                    RichText::new(save_indicator)
                        .size(10.5)
                        .color(save_color)
                        .strong(),
                )
                .on_hover_text(if is_saved {
                    "Projeto salvo e sincronizado"
                } else {
                    "Alterações não salvas · Ctrl+S para salvar"
                });

                ui.label(
                    RichText::new(&proj_name)
                        .size(11.0)
                        .color(tokens::TEXT_PRIMARY),
                );

                ui.separator();

                ui.label(
                    RichText::new(&hint)
                        .size(10.5)
                        .color(tokens::TEXT_MUTED),
                );

                ui.separator();

                // BLOCO 3: Telemetria da Cena e Controles de Histórico (Lado Direito)
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let telemetry = format!(
                        "Tris: {scene_tris} │ Verts: {scene_verts} │ Objs: {obj_count} │ {frame_ms:.1}ms │ v0.6.0"
                    );
                    ui.monospace(
                        RichText::new(telemetry)
                            .size(10.5)
                            .color(tokens::TEXT_MUTED),
                    );

                    ui.separator();

                    // Botões de Desfazer / Refazer
                    if ui
                        .add_enabled(
                            !state.is_interacting() && state.undo.can_undo(),
                            egui::Button::new(RichText::new("↩").size(10.5)),
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
                            egui::Button::new(RichText::new("↪").size(10.5)),
                        )
                        .on_hover_text(format!(
                            "Refazer · Ctrl+Shift+Z · {}",
                            state.undo.redo_label().unwrap_or("")
                        ))
                        .clicked()
                    {
                        state.redo();
                    }

                    ui.separator();

                    // BLOCO 2: Mensagem / Feedback Central (Espaço restante)
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        ui.add(
                            egui::Label::new(
                                RichText::new(&status_text)
                                    .size(11.0)
                                    .color(tokens::TEXT_SECONDARY),
                            )
                            .truncate(),
                        )
                        .on_hover_text(&status_text);
                    });
                });
            });
        });
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

    #[test]
    fn test_status_bar_displays_dirty_state() {
        let mut state = AppState::new("en");
        state.dirty = false;
        assert!(!state.dirty);
        state.mark_dirty();
        assert!(state.dirty);
    }
}
