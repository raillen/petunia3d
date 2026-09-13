//! Ferramenta interativa de medição 3D (`Measure`).
//! Permite medir distâncias entre pontos no espaço e vértices da malha ativa,
//! exibindo linhas de medição com marcas de régua e indicador flutuante com distância e deltas (ΔX, ΔY, ΔZ),
//! com suporte transacional a Undo/Redo (Ctrl+Z) e coleção dedicada no Outliner.

use egui::{vec2, Color32, FontId, PointerButton, Pos2, Rect, Response, Stroke};
use glam::Vec3;
use petunia_core::{AppState, MeasurementItem};

/// Projeta uma coordenada 3D de mundo para a coordenada 2D de tela dentro do retângulo do viewport.
fn project_to_screen(point: [f32; 3], state: &AppState, rect: Rect) -> Option<Pos2> {
    let p3 = Vec3::from(point);
    let ndc = state.camera.project_ndc(p3);
    if state.camera.proj == petunia_core::Projection::Perspective {
        let fwd = state.camera.forward();
        let to_p = (p3 - state.camera.eye()).normalize_or_zero();
        if fwd.dot(to_p) <= 0.0 {
            return None;
        }
    }
    if ndc.x.abs() > 2.0 || ndc.y.abs() > 2.0 {
        return None;
    }
    let screen_x = rect.min.x + (ndc.x * 0.5 + 0.5) * rect.width();
    let screen_y = rect.min.y + (1.0 - (ndc.y * 0.5 + 0.5)) * rect.height();
    Some(Pos2::new(screen_x, screen_y))
}

/// Encontra o ponto 3D sob o cursor, com snapping opcional para o vértice mais próximo da malha ativa.
fn unproject_cursor_or_snap(screen_pos: Pos2, state: &AppState, rect: Rect) -> [f32; 3] {
    // 1. Tenta snapping para o vértice mais próximo da malha ativa (raio de 16px na tela)
    if let Some(mesh) = state.project.active_mesh() {
        let mut best_dist_sq = 16.0 * 16.0_f32;
        let mut best_pos = None;

        for v in &mesh.verts {
            if let Some(sp) = project_to_screen(v.pos, state, rect) {
                let d_sq = sp.distance_sq(screen_pos);
                if d_sq < best_dist_sq {
                    best_dist_sq = d_sq;
                    best_pos = Some(v.pos);
                }
            }
        }

        if let Some(pos) = best_pos {
            return pos;
        }
    }

    // 2. Fallback: interseção do raio da câmera com o plano horizontal do cursor 3D
    let ndc_x = ((screen_pos.x - rect.min.x) / rect.width()) * 2.0 - 1.0;
    let ndc_y = 1.0 - ((screen_pos.y - rect.min.y) / rect.height()) * 2.0;
    let (ray_origin, ray_dir) = state.camera.ray(ndc_x, ndc_y);

    let plane_y = state.cursor_3d[1];
    if ray_dir.y.abs() > 1e-4 {
        let t = (plane_y - ray_origin.y) / ray_dir.y;
        if t > 0.0 {
            let hit = ray_origin + ray_dir * t;
            return [hit.x, hit.y, hit.z];
        }
    }

    [
        ray_origin.x + ray_dir.x * 5.0,
        ray_origin.y + ray_dir.y * 5.0,
        ray_origin.z + ray_dir.z * 5.0,
    ]
}

/// Renderiza e manipula a ferramenta de medição interativa no viewport com suporte a undo/redo.
pub fn draw(
    ctx: &egui::Context,
    state: &mut AppState,
    rect: Rect,
    painter: &egui::Painter,
    response: &Response,
) -> bool {
    let is_measure_tool = state.active_tool == "measure";

    // 1. Limpeza / Exclusão por atalho Delete ou Backspace
    if is_measure_tool
        && (ctx.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)))
    {
        if let Some(sel_id) = state.selected_measurement {
            state.checkpoint("delete measurement");
            state.project.remove_measurement(sel_id);
            state.selected_measurement = None;
            state.mark_dirty();
            return true;
        } else if !state.project.measurements.is_empty() {
            state.checkpoint("clear measurements");
            state.project.measurements.clear();
            state.active_measurement = None;
            state.selected_measurement = None;
            state.mark_dirty();
            return true;
        }
    }

    // 2. Manipulação de arrasto de medição quando a ferramenta estiver ativa
    let mut handled = false;
    if is_measure_tool {
        if let Some(pointer) = ctx.pointer_hover_pos().filter(|p| rect.contains(*p)) {
            if response.drag_started_by(PointerButton::Primary) {
                let start_3d = unproject_cursor_or_snap(pointer, state, rect);
                state.active_measurement =
                    Some(MeasurementItem::new("Active", start_3d, start_3d, 0.0));
                state.mark_dirty();
                handled = true;
            } else if response.dragged_by(PointerButton::Primary) {
                let curr_3d = unproject_cursor_or_snap(pointer, state, rect);
                if let Some(ref mut m) = state.active_measurement {
                    let dx = curr_3d[0] - m.start[0];
                    let dy = curr_3d[1] - m.start[1];
                    let dz = curr_3d[2] - m.start[2];
                    m.end = curr_3d;
                    m.distance = (dx * dx + dy * dy + dz * dz).sqrt();
                    state.mark_dirty();
                    handled = true;
                }
            } else if response.drag_stopped_by(PointerButton::Primary) {
                if let Some(m) = state.active_measurement.take() {
                    if m.distance > 0.001 {
                        // Checkpoint transacional para que a medida possa ser desfeita com Ctrl+Z!
                        state.checkpoint("add measurement");
                        let count = state.project.measurements.len() + 1;
                        let item = MeasurementItem::new(
                            format!("Medida {count}"),
                            m.start,
                            m.end,
                            m.distance,
                        );
                        let item_id = item.id;
                        state.project.add_measurement(item);
                        state.selected_measurement = Some(item_id);
                    }
                    state.mark_dirty();
                    handled = true;
                }
            }
        }
    }

    // 3. Renderização de todas as medições salvas (se a coleção estiver visível)
    let c_yellow = Color32::from_rgb(255, 213, 79);
    let c_blue = Color32::from_rgb(64, 196, 255);
    let c_bg = Color32::from_rgba_unmultiplied(24, 25, 29, 230);
    let c_white = Color32::WHITE;

    if state.project.measurements_visible {
        for m in &state.project.measurements {
            if !m.visible {
                continue;
            }
            let is_selected = state.selected_measurement == Some(m.id);
            let p0 = project_to_screen(m.start, state, rect);
            let p1 = project_to_screen(m.end, state, rect);

            if let (Some(s0), Some(s1)) = (p0, p1) {
                let line_color = if is_selected {
                    Color32::from_rgb(100, 220, 255)
                } else {
                    c_yellow
                };
                let stroke_w = if is_selected { 2.5_f32 } else { 2.0_f32 };

                // Linha principal de medição
                painter.line_segment([s0, s1], Stroke::new(stroke_w, line_color));

                // Miras nas pontas (+)
                for pt in [s0, s1] {
                    painter.line_segment(
                        [pt + vec2(-4.0, 0.0), pt + vec2(4.0, 0.0)],
                        Stroke::new(1.5_f32, c_white),
                    );
                    painter.line_segment(
                        [pt + vec2(0.0, -4.0), pt + vec2(0.0, 4.0)],
                        Stroke::new(1.5_f32, c_white),
                    );
                }

                // Ponto médio e badge informativo
                let mid = Pos2::new((s0.x + s1.x) * 0.5, (s0.y + s1.y) * 0.5);
                let deltas = m.deltas();
                let text = format!(
                    "{:.2}m (ΔX:{:.2} ΔY:{:.2} ΔZ:{:.2})",
                    m.distance, deltas[0], deltas[1], deltas[2]
                );
                let font = FontId::monospace(11.0);
                let text_color = if is_selected {
                    Color32::from_rgb(100, 220, 255)
                } else {
                    c_blue
                };
                let galley = painter.layout_no_wrap(text, font, text_color);
                let badge_rect = Rect::from_center_size(mid, galley.size() + vec2(10.0, 6.0));

                painter.rect_filled(badge_rect, 4.0, c_bg);
                painter.rect_stroke(
                    badge_rect,
                    4.0,
                    Stroke::new(
                        1.0_f32,
                        if is_selected {
                            Color32::from_rgb(100, 220, 255)
                        } else {
                            Color32::from_white_alpha(40)
                        },
                    ),
                    egui::StrokeKind::Inside,
                );
                painter.galley(badge_rect.min + vec2(5.0, 3.0), galley, text_color);
            }
        }
    }

    // 4. Medição ativa em tempo real durante arrasto
    if let Some(ref m) = state.active_measurement {
        let p0 = project_to_screen(m.start, state, rect);
        let p1 = project_to_screen(m.end, state, rect);
        if let (Some(s0), Some(s1)) = (p0, p1) {
            painter.line_segment([s0, s1], Stroke::new(2.0_f32, c_yellow));
            for pt in [s0, s1] {
                painter.line_segment(
                    [pt + vec2(-4.0, 0.0), pt + vec2(4.0, 0.0)],
                    Stroke::new(1.5_f32, c_white),
                );
                painter.line_segment(
                    [pt + vec2(0.0, -4.0), pt + vec2(0.0, 4.0)],
                    Stroke::new(1.5_f32, c_white),
                );
            }
            let mid = Pos2::new((s0.x + s1.x) * 0.5, (s0.y + s1.y) * 0.5);
            let deltas = m.deltas();
            let text = format!(
                "{:.2}m (ΔX:{:.2} ΔY:{:.2} ΔZ:{:.2})",
                m.distance, deltas[0], deltas[1], deltas[2]
            );
            let font = FontId::monospace(11.0);
            let galley = painter.layout_no_wrap(text, font, c_blue);
            let badge_rect = Rect::from_center_size(mid, galley.size() + vec2(10.0, 6.0));
            painter.rect_filled(badge_rect, 4.0, c_bg);
            painter.rect_stroke(
                badge_rect,
                4.0,
                Stroke::new(1.0_f32, Color32::from_white_alpha(40)),
                egui::StrokeKind::Inside,
            );
            painter.galley(badge_rect.min + vec2(5.0, 3.0), galley, c_blue);
        }
    }

    handled
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurement_draw_without_panic() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");
        state.active_tool = "measure".into();
        state.project.add_measurement(MeasurementItem::new(
            "M1",
            [0.0, 0.0, 0.0],
            [1.0, 2.0, 0.0],
            2.23,
        ));

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let rect = ui.available_rect_before_wrap();
                let painter = ui.painter_at(rect);
                let response = ui.allocate_rect(rect, egui::Sense::click_and_drag());
                let _ = draw(ctx, &mut state, rect, &painter, &response);
            });
        });

        assert_eq!(state.project.measurements.len(), 1);
    }

    #[test]
    fn test_measurement_deltas_computation() {
        let m = MeasurementItem::new("M1", [0.0, 0.0, 0.0], [3.0, 4.0, 0.0], 5.0);
        assert!((m.distance - 5.0).abs() < 1e-4);
        let deltas = m.deltas();
        assert_eq!(deltas, [3.0, 4.0, 0.0]);
    }

    #[test]
    fn test_measurement_undo_redo() {
        let mut state = AppState::new("en");
        state.checkpoint("initial");
        let initial_count = state.project.measurements.len();

        // Adiciona medição com checkpoint
        state.checkpoint("add measurement");
        let item = MeasurementItem::new("M1", [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 1.0);
        state.project.add_measurement(item);
        assert_eq!(state.project.measurements.len(), initial_count + 1);

        // Undo (Ctrl+Z)
        assert!(state.undo());
        assert_eq!(state.project.measurements.len(), initial_count);

        // Redo (Ctrl+Shift+Z)
        assert!(state.redo());
        assert_eq!(state.project.measurements.len(), initial_count + 1);
    }
}
