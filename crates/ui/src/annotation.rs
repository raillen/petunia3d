//! Ferramenta interativa de anotação 3D (`Annotate`).
//! Permite desenhar traços livres (grease pencil / annotations) no espaço 3D,
//! projetados na superfície da geometria ou no plano de trabalho do cursor 3D.

use egui::{Color32, PointerButton, Pos2, Rect, Response, Stroke};
use glam::Vec3;
use petunia_core::{picking::pick_mesh, AnnotationStroke, AppState, SelectMode};

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

/// Converte a posição do cursor em um ponto 3D na superfície da malha ou no plano do cursor 3D.
fn unproject_to_surface_or_plane(
    screen_pos: Pos2,
    state: &AppState,
    rect: Rect,
    pixels_per_point: f32,
) -> [f32; 3] {
    let ndc_x = ((screen_pos.x - rect.min.x) / rect.width()) * 2.0 - 1.0;
    let ndc_y = 1.0 - ((screen_pos.y - rect.min.y) / rect.height()) * 2.0;

    // 1. Tenta atingir a superfície da malha ativa
    if let Some(mesh) = state.project.active_mesh() {
        if let Some(hit) = pick_mesh(
            mesh,
            &state.camera,
            glam::Vec2::new(rect.width(), rect.height()) * pixels_per_point,
            glam::Vec2::new(ndc_x, ndc_y),
            SelectMode::Face,
            false,
        ) {
            let p = hit.position;
            return [p.x, p.y, p.z];
        }
    }

    // 2. Fallback: interseção com o plano horizontal do cursor 3D
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

/// Renderiza e manipula anotações e traços no viewport.
pub fn draw(
    ctx: &egui::Context,
    state: &mut AppState,
    rect: Rect,
    painter: &egui::Painter,
    response: &Response,
) -> bool {
    let is_annotate_tool = state.active_tool == "annotate";

    // 1. Limpeza por atalho Delete/Backspace
    if is_annotate_tool
        && (ctx.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)))
    {
        state.annotations.clear();
        state.active_annotation = None;
        state.mark_dirty();
        return true;
    }

    // 2. Manipulação de desenho de traços
    let mut handled = false;
    if is_annotate_tool {
        if let Some(pointer) = ctx.pointer_hover_pos().filter(|p| rect.contains(*p)) {
            if response.drag_started_by(PointerButton::Primary) {
                let pt =
                    unproject_to_surface_or_plane(pointer, state, rect, ctx.pixels_per_point());
                let mut stroke = AnnotationStroke::default();
                stroke.points.push(pt);
                state.active_annotation = Some(stroke);
                state.mark_dirty();
                handled = true;
            } else if response.dragged_by(PointerButton::Primary) {
                let pt =
                    unproject_to_surface_or_plane(pointer, state, rect, ctx.pixels_per_point());
                if let Some(ref mut stroke) = state.active_annotation {
                    let should_add = match stroke.points.last() {
                        Some(last) => {
                            let p_last = Vec3::from(*last);
                            let p_curr = Vec3::from(pt);
                            p_last.distance(p_curr) > 0.04
                        }
                        None => true,
                    };
                    if should_add {
                        stroke.points.push(pt);
                        state.mark_dirty();
                    }
                    handled = true;
                }
            } else if response.drag_stopped_by(PointerButton::Primary) {
                if let Some(stroke) = state.active_annotation.take() {
                    if stroke.points.len() >= 2 {
                        state.annotations.push(stroke);
                    }
                    state.mark_dirty();
                    handled = true;
                }
            }
        }
    }

    // 3. Renderização de todos os traços (salvos e ativo)
    let mut strokes_to_render: Vec<&AnnotationStroke> = state.annotations.iter().collect();
    if let Some(ref active) = state.active_annotation {
        strokes_to_render.push(active);
    }

    for stroke in strokes_to_render {
        if stroke.points.len() < 2 {
            continue;
        }

        let c = Color32::from_rgba_premultiplied(
            (stroke.color[0] * 255.0) as u8,
            (stroke.color[1] * 255.0) as u8,
            (stroke.color[2] * 255.0) as u8,
            (stroke.color[3] * 255.0) as u8,
        );
        let egui_stroke = Stroke::new(stroke.width.max(1.5_f32), c);

        let mut prev_screen: Option<Pos2> = None;
        for &pt in &stroke.points {
            if let Some(curr_screen) = project_to_screen(pt, state, rect) {
                if let Some(prev) = prev_screen {
                    painter.line_segment([prev, curr_screen], egui_stroke);
                }
                prev_screen = Some(curr_screen);
            } else {
                prev_screen = None;
            }
        }
    }

    handled
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_annotation_draw_without_panic() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");
        state.active_tool = "annotate".into();
        let mut stroke = AnnotationStroke::default();
        stroke.points.push([0.0, 0.0, 0.0]);
        stroke.points.push([1.0, 1.0, 0.0]);
        state.annotations.push(stroke);

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let rect = ui.available_rect_before_wrap();
                let painter = ui.painter_at(rect);
                let response = ui.allocate_rect(rect, egui::Sense::click_and_drag());
                let _ = draw(ctx, &mut state, rect, &painter, &response);
            });
        });
    }
}
