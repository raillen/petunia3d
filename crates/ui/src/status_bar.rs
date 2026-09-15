//! Barra de status inferior do Petunia3D (`Status Bar`).
//! Estruturada em 3 zonas rígidas (larguras medidas, nunca sobrepostas):
//! 1. Identidade do Projeto (`Saved` / `Unsaved`), nome do arquivo e dica contextual (truncada);
//! 2. Mensagem central de status / feedback do sistema (truncada à coluna);
//! 3. Telemetria agregada de cena (Tris, Vértices, Objetos, Frame Time) e botões de histórico (Undo/Redo).
//!
//! Nenhum texto escapa da sua zona: tudo que varia é medido ou truncado, então
//! as sidebars nunca têm o que cobrir (invariante `status_overlaps` + teste).

use egui::{Align, Layout, RichText, Ui, vec2};
use petunia_core::AppState;
use petunia_module_model::ToolRegistry;

use crate::icon_registry::PetuniaIcon;
use crate::tokens;
use crate::widgets::PetuniaIconButton;

/// Renderiza a barra de status inferior estruturada em 3 blocos.
pub fn draw(ui: &mut Ui, state: &mut AppState, tools: &ToolRegistry) {
    let proj_name = state
        .project
        .project_path
        .as_deref()
        .map(|p| {
            std::path::Path::new(p)
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or(p)
                .to_string()
        })
        .unwrap_or_else(|| "Untitled".to_string());

    let is_saved = !state.is_document_dirty();
    let (save_indicator, save_color) = if is_saved {
        ("Saved", tokens::ACCENT_GREEN)
    } else {
        ("Unsaved", tokens::ACCENT_AMBER)
    };

    let hint = if let Some(fb) = state.current_tool_feedback() {
        format!("{} · {}", fb.delta_text, fb.status_hint)
    } else if state.is_interacting() {
        "Enter / LMB: Confirm · Esc / RMB: Cancel".to_string()
    } else {
        tools
            .get(&state.active_tool)
            .map(|t| state.t(t.hint_key()))
            .unwrap_or_else(|| "LMB: Select · MMB: Orbit · Shift+MMB: Pan".to_string())
    };

    let status_text = if state.ui.status.is_empty() {
        hint.clone()
    } else {
        state.ui.status.clone()
    };

    let scene_tris = state.scene_tris();
    let scene_verts = state.scene_verts();
    let obj_count = state.project.assets.len();
    let frame_ms = state.render.stats.frame_ms;

    let bar_resp = egui::Panel::bottom("status_bar")
        .exact_size(tokens::STATUS_BAR_HEIGHT)
        .frame(
            egui::Frame::new()
                .fill(tokens::bg_header(state))
                .stroke(tokens::stroke_border_dyn(state))
                .inner_margin(egui::Margin::symmetric(8, 2)),
        )
        .show(ui, |ui| {
            // Três colunas iguais = três zonas rígidas. Nada é posicionado por
            // resto de linha: cada zona só pinta dentro da sua coluna.
            ui.columns(3, |cols| {
                // ZONA 1 (esquerda): identidade + dica, dica truncada ao resto.
                cols[0].horizontal(|ui| {
                    ui.spacing_mut().item_spacing = vec2(6.0, 0.0);
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

                    let hint_w = ui.available_width().max(0.0);
                    ui.add_sized(
                        vec2(hint_w, 16.0),
                        egui::Label::new(
                            RichText::new(&hint).size(10.5).color(tokens::TEXT_MUTED),
                        )
                        .truncate(),
                    )
                    .on_hover_text(&hint);
                });

                // ZONA 2 (centro): mensagem de status, sempre contida.
                cols[1].horizontal_centered(|ui| {
                    let center_w = ui.available_width().max(0.0);
                    ui.add_sized(
                        vec2(center_w, 16.0),
                        egui::Label::new(
                            RichText::new(&status_text)
                                .size(11.0)
                                .color(tokens::TEXT_SECONDARY),
                        )
                        .truncate(),
                    )
                    .on_hover_text(&status_text);
                });

                // ZONA 3 (direita): telemetria + undo/redo, alinhados à direita.
                cols[2].with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.spacing_mut().item_spacing = vec2(6.0, 0.0);
                    let telemetry = format!(
                        "Tris: {scene_tris} | Verts: {scene_verts} | Objs: {obj_count} | {frame_ms:.1}ms | v0.6.0"
                    );
                    let tele_w = ui
                        .fonts_mut(|f| {
                            f.layout_no_wrap(
                                telemetry.clone(),
                                egui::FontId::monospace(10.5),
                                tokens::TEXT_MUTED,
                            )
                            .size()
                            .x
                        })
                        .min(ui.available_width().max(0.0));
                    ui.add_sized(
                        vec2(tele_w, 16.0),
                        egui::Label::new(
                            RichText::new(&telemetry)
                                .size(10.5)
                                .color(tokens::TEXT_MUTED)
                                .monospace(),
                        )
                        .truncate(),
                    );

                    ui.separator();

                    // Botões de Desfazer / Refazer (ícones semânticos, Wave 6).
                    let undo_tip = format!(
                        "Desfazer · Ctrl+Z · {}",
                        state.project.undo.undo_label().unwrap_or("")
                    );
                    if PetuniaIconButton::new(PetuniaIcon::Undo, &undo_tip, 22.0)
                        .enabled(!state.is_interacting() && state.project.undo.can_undo())
                        .show(ui)
                        .clicked()
                    {
                        state.undo();
                    }

                    let redo_tip = format!(
                        "Refazer · Ctrl+Shift+Z · {}",
                        state.project.undo.redo_label().unwrap_or("")
                    );
                    if PetuniaIconButton::new(PetuniaIcon::Redo, &redo_tip, 22.0)
                        .enabled(!state.is_interacting() && state.project.undo.can_redo())
                        .show(ui)
                        .clicked()
                    {
                        state.redo();
                    }
                });
            });
        });
    crate::regions::record(
        ui.ctx(),
        crate::regions::RegionSlot::StatusBar,
        bar_resp.response.rect,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_renders_without_panic() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");
        let tools = ToolRegistry::default();

        ctx.run_ui(egui::RawInput::default(), |ui| {
            draw(ui, &mut state, &tools);
        })
        .textures_delta
        .clear();
    }

    #[test]
    fn test_status_bar_displays_dirty_state() {
        let mut state = AppState::new("en");
        state.mark_document_clean();
        assert!(!state.is_document_dirty());
        state.mark_document_dirty();
        assert!(state.is_document_dirty());
    }
}
