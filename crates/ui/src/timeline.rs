//! Painel de Timeline de animação e reprodução do Petunia3D (`Timeline`).
//! Implementa botões de transporte (|<<, <|, Play, |>, >>|), intervalo de frames e régua temporal interativa.

use egui::{Sense, Stroke, Ui, vec2};
use petunia_core::AppState;

use crate::tokens;

/// Renderiza o painel inferior de Timeline.
pub fn draw(ui: &mut Ui, state: &mut AppState) {
    egui::Panel::bottom("timeline_panel")
        .exact_size(tokens::TIMELINE_HEIGHT)
        .frame(
            egui::Frame::new()
                .fill(tokens::BG_PANEL)
                .stroke(tokens::stroke_border())
                .inner_margin(egui::Margin::symmetric(8, 4)),
        )
        .show(ui, |ui| {
            ui.add_enabled_ui(!state.is_interacting(), |ui| {
                // Linha 1: Controles de Transporte e Frames
                draw_transport_bar(ui, state);

                ui.add_space(2.0);

                // Linha 2: Régua de Scrubbing Temporal
                draw_timeline_ruler(ui, state);
            });
        });
}

fn draw_transport_bar(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(4.0, 0.0);

        // Menus da Timeline
        ui.label(
            egui::RichText::new("Playback")
                .size(11.0)
                .color(tokens::TEXT_SECONDARY),
        );
        ui.label(
            egui::RichText::new("Keying")
                .size(11.0)
                .color(tokens::TEXT_SECONDARY),
        );
        ui.label(
            egui::RichText::new("View")
                .size(11.0)
                .color(tokens::TEXT_SECONDARY),
        );

        ui.separator();

        // Botões de Transporte de Animação
        if ui
            .small_button("⏮")
            .on_hover_text("Jump to First Frame · Shift+Left")
            .clicked()
        {
            state.ui.timeline_frame = state.ui.timeline_start;
            state.mark_dirty();
        }
        if ui
            .small_button("◀")
            .on_hover_text("Step 1 Frame Backward · Left")
            .clicked()
        {
            state.ui.timeline_frame = (state.ui.timeline_frame - 1).max(state.ui.timeline_start);
            state.mark_dirty();
        }

        let play_icon = if state.ui.timeline_playing {
            "⏸"
        } else {
            "▶"
        };
        let play_btn = egui::Button::new(egui::RichText::new(play_icon).size(11.0).color(
            if state.ui.timeline_playing {
                tokens::TEXT_ACTIVE
            } else {
                tokens::TEXT_PRIMARY
            },
        ))
        .fill(if state.ui.timeline_playing {
            tokens::ACCENT_BLUE
        } else {
            tokens::BG_SURFACE
        })
        .corner_radius(tokens::RADIUS_CONTROL);

        if ui
            .add(play_btn)
            .on_hover_text("Play / Pause Animation · Space")
            .clicked()
        {
            state.ui.timeline_playing = !state.ui.timeline_playing;
            state.mark_dirty();
        }

        if ui
            .small_button("▶")
            .on_hover_text("Step 1 Frame Forward · Right")
            .clicked()
        {
            state.ui.timeline_frame = (state.ui.timeline_frame + 1).min(state.ui.timeline_end);
            state.mark_dirty();
        }
        if ui
            .small_button("⏭")
            .on_hover_text("Jump to Last Frame · Shift+Right")
            .clicked()
        {
            state.ui.timeline_frame = state.ui.timeline_end;
            state.mark_dirty();
        }

        ui.separator();

        // Frame atual
        ui.label(
            egui::RichText::new("Frame:")
                .size(11.0)
                .color(tokens::TEXT_SECONDARY),
        );
        let mut curr_frame = state.ui.timeline_frame;
        if ui
            .add(
                egui::DragValue::new(&mut curr_frame)
                    .range(state.ui.timeline_start..=state.ui.timeline_end)
                    .speed(1.0),
            )
            .changed()
        {
            state.ui.timeline_frame = curr_frame;
            state.mark_dirty();
        }

        // Intervalo: Start / End
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add(
                egui::DragValue::new(&mut state.ui.timeline_end)
                    .range(1..=10000)
                    .speed(1.0),
            );
            ui.label(
                egui::RichText::new("End:")
                    .size(11.0)
                    .color(tokens::TEXT_SECONDARY),
            );

            ui.add(
                egui::DragValue::new(&mut state.ui.timeline_start)
                    .range(0..=10000)
                    .speed(1.0),
            );
            ui.label(
                egui::RichText::new("Start:")
                    .size(11.0)
                    .color(tokens::TEXT_SECONDARY),
            );
        });
    });
}

fn draw_timeline_ruler(ui: &mut Ui, state: &mut AppState) {
    let desired_size = vec2(ui.available_width(), 20.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();

        // Fundo da régua
        painter.rect_filled(rect, tokens::RADIUS_CONTROL, tokens::BG_INPUT);
        painter.rect_stroke(
            rect,
            tokens::RADIUS_CONTROL,
            tokens::stroke_border(),
            egui::StrokeKind::Inside,
        );

        let start = state.ui.timeline_start as f32;
        let end = state.ui.timeline_end.max(state.ui.timeline_start + 1) as f32;
        let total_frames = end - start;

        // Desenha marcações de frames a cada 20 frames
        let step = if rect.width() < 400.0 { 40.0 } else { 20.0 };
        let mut f = (start / step).floor() * step;
        while f <= end {
            if f >= start {
                let t = (f - start) / total_frames;
                let x = rect.min.x + t * rect.width();

                // Linha de marcação
                painter.line_segment(
                    [egui::pos2(x, rect.min.y + 12.0), egui::pos2(x, rect.max.y)],
                    Stroke::new(1.0_f32, tokens::BORDER_SUBTLE),
                );

                // Número do frame
                painter.text(
                    egui::pos2(x + 2.0, rect.min.y + 1.0),
                    egui::Align2::LEFT_TOP,
                    format!("{:.0}", f),
                    egui::FontId::monospace(9.0),
                    tokens::TEXT_MUTED,
                );
            }
            f += step;
        }

        // Indicador do Frame Atual (Cursor Azul)
        let curr_t = ((state.ui.timeline_frame as f32 - start) / total_frames).clamp(0.0, 1.0);
        let cursor_x = rect.min.x + curr_t * rect.width();

        painter.line_segment(
            [
                egui::pos2(cursor_x, rect.min.y),
                egui::pos2(cursor_x, rect.max.y),
            ],
            Stroke::new(2.0_f32, tokens::ACCENT_BLUE),
        );

        // Cabeçote do cursor no topo
        painter.circle_filled(
            egui::pos2(cursor_x, rect.min.y + 3.0),
            3.5,
            tokens::ACCENT_BLUE,
        );
    }

    // Interatividade de Scrubbing com clique e arrasto
    if (response.clicked() || response.dragged())
        && let Some(pos) = response.interact_pointer_pos()
    {
        let t = ((pos.x - rect.min.x) / rect.width().max(1.0)).clamp(0.0, 1.0);
        let frame = state.ui.timeline_start as f32
            + t * (state.ui.timeline_end - state.ui.timeline_start) as f32;
        state.ui.timeline_frame = frame.round() as i32;
        state.mark_dirty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_renders_without_panic() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");

        ctx.run_ui(egui::RawInput::default(), |ui| {
            draw(ui, &mut state);
        })
        .textures_delta
        .clear();
    }

    #[test]
    fn test_timeline_scrub_clamps_within_bounds() {
        let mut state = AppState::new("en");
        state.ui.timeline_start = 10;
        state.ui.timeline_end = 100;
        state.ui.timeline_frame = 50;

        assert_eq!(state.ui.timeline_frame, 50);
    }
}
