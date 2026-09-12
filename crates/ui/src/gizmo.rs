//! Camera-projected manipulation handles. The app owns drag/modal transactions.

#![forbid(unsafe_code)]

use egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, Vec2};
use glam::Vec3;
use petunia_core::{Camera, Projection};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GizmoKind {
    Translate,
    Rotate,
    Scale,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GizmoHandle {
    Axis(u8),
    /// Normal axis: 0 = YZ, 1 = XZ, 2 = XY.
    Plane(u8),
}

const COLORS: [Color32; 3] = [
    Color32::from_rgb(240, 85, 85),
    Color32::from_rgb(100, 220, 120),
    Color32::from_rgb(95, 150, 255),
];
const AXES: [Vec3; 3] = [Vec3::X, Vec3::Y, Vec3::Z];

/// Draws foreground handles and returns the handle under the pointer.
/// Distances use egui points to preserve usability on high-DPI displays.
pub fn draw_gizmo(
    painter: &Painter,
    camera: &Camera,
    viewport: Rect,
    pivot: Vec3,
    kind: GizmoKind,
    pointer: Option<Pos2>,
) -> Option<GizmoHandle> {
    if viewport.height() <= 0.0 || !pivot.is_finite() {
        return None;
    }
    let depth = (pivot - camera.eye()).dot(camera.forward());
    if depth <= camera.near || depth >= camera.far {
        return None;
    }
    let world_per_point = match camera.proj {
        Projection::Ortho => 2.0 * camera.ortho_half_h / viewport.height(),
        Projection::Perspective => 2.0 * depth * (camera.fov_y * 0.5).tan() / viewport.height(),
    };
    let length = world_per_point * 80.0;
    let project = |point: Vec3| -> Option<Pos2> {
        let clip = camera.view_proj() * point.extend(1.0);
        if !clip.is_finite() || clip.w <= 0.0 || clip.z < 0.0 || clip.z > clip.w {
            return None;
        }
        Some(Pos2::new(
            viewport.center().x + clip.x / clip.w * viewport.width() * 0.5,
            viewport.center().y - clip.y / clip.w * viewport.height() * 0.5,
        ))
    };
    let center = project(pivot)?;
    let mut best: Option<(f32, GizmoHandle)> = None;
    if kind == GizmoKind::Translate {
        for (normal, &base_color) in COLORS.iter().enumerate() {
            let a = AXES[(normal + 1) % 3] * length;
            let b = AXES[(normal + 2) % 3] * length;
            let corners: Option<Vec<_>> = [(0.22, 0.22), (0.44, 0.22), (0.44, 0.44), (0.22, 0.44)]
                .into_iter()
                .map(|(u, v)| project(pivot + a * u + b * v))
                .collect();
            let Some(corners) = corners else {
                continue;
            };
            let hovered = pointer.is_some_and(|point| inside_convex(&corners, point));
            let color = if hovered { Color32::WHITE } else { base_color };
            painter.add(Shape::convex_polygon(
                corners,
                color.gamma_multiply(0.25),
                Stroke::new(1.0_f32, color),
            ));
            if hovered {
                best = Some((6.0, GizmoHandle::Plane(normal as u8)));
            }
        }
    }
    for (axis, &color) in COLORS.iter().enumerate() {
        if kind == GizmoKind::Rotate {
            let a = AXES[(axis + 1) % 3] * length * 0.8;
            let b = AXES[(axis + 2) % 3] * length * 0.8;
            let mut previous = None;
            let mut lines = Vec::new();
            let mut distance = f32::INFINITY;
            for step in 0..=64 {
                let angle = step as f32 / 64.0 * std::f32::consts::TAU;
                let current = project(pivot + a * angle.cos() + b * angle.sin());
                if let (Some(start), Some(end)) = (previous, current) {
                    if let Some(pointer) = pointer {
                        distance = distance.min(segment_distance(pointer, start, end));
                    }
                    lines.push([start, end]);
                }
                previous = current;
            }
            let hovered = distance <= 7.0;
            for line in lines {
                painter.line_segment(
                    line,
                    Stroke::new(
                        if hovered { 3.5_f32 } else { 2.0_f32 },
                        if hovered { Color32::WHITE } else { color },
                    ),
                );
            }
            if hovered && best.is_none_or(|(previous, _)| distance < previous) {
                best = Some((distance, GizmoHandle::Axis(axis as u8)));
            }
        } else {
            let (Some(start), Some(end)) = (
                project(pivot + AXES[axis] * length * 0.12),
                project(pivot + AXES[axis] * length),
            ) else {
                continue;
            };
            // An axis facing the camera has no meaningful drag direction.
            if start.distance(end) < 10.0 {
                continue;
            }
            let distance =
                pointer.map_or(f32::INFINITY, |point| segment_distance(point, start, end));
            let hovered = distance <= 7.0;
            let color = if hovered { Color32::WHITE } else { color };
            painter.line_segment(
                [start, end],
                Stroke::new(5.0_f32, Color32::from_black_alpha(170)),
            );
            painter.line_segment([start, end], Stroke::new(2.5_f32, color));
            if kind == GizmoKind::Scale {
                painter.rect_filled(Rect::from_center_size(end, Vec2::splat(9.0)), 1.0, color);
            } else {
                let direction = (end - start).normalized();
                let perpendicular = Vec2::new(-direction.y, direction.x);
                painter.add(Shape::convex_polygon(
                    vec![
                        end,
                        end - direction * 12.0 + perpendicular * 5.0,
                        end - direction * 12.0 - perpendicular * 5.0,
                    ],
                    color,
                    Stroke::NONE,
                ));
            }
            if hovered && best.is_none_or(|(previous, _)| distance < previous) {
                best = Some((distance, GizmoHandle::Axis(axis as u8)));
            }
        }
    }
    painter.circle_filled(center, 3.0, Color32::WHITE);
    best.map(|(_, handle)| handle)
}

fn segment_distance(point: Pos2, start: Pos2, end: Pos2) -> f32 {
    let edge = end - start;
    let t = if edge.length_sq() > 1e-6 {
        ((point - start).dot(edge) / edge.length_sq()).clamp(0.0, 1.0)
    } else {
        0.0
    };
    point.distance(start + edge * t)
}

fn inside_convex(points: &[Pos2], point: Pos2) -> bool {
    if points.len() < 3 {
        return false;
    }
    let mut positive = false;
    let mut negative = false;
    let mut area = 0.0;
    for index in 0..points.len() {
        let start = points[index];
        let end = points[(index + 1) % points.len()];
        let edge = end - start;
        let offset = point - start;
        let cross = edge.x * offset.y - edge.y * offset.x;
        positive |= cross > 0.001;
        negative |= cross < -0.001;
        area += start.x * end.y - end.x * start.y;
    }
    area.abs() > 1.0 && !(positive && negative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projected_axes_and_plane_return_drag_constraints() {
        let context = egui::Context::default();
        let mut camera = Camera::default();
        camera.set_preset(petunia_core::ViewPreset::Front);
        camera.aspect = 1.0;
        camera.ortho_half_h = 2.0;
        let viewport = Rect::from_min_size(Pos2::ZERO, Vec2::splat(800.0));
        let _ = context.run(egui::RawInput::default(), |context| {
            let painter = context.layer_painter(egui::LayerId::background());
            for kind in [GizmoKind::Translate, GizmoKind::Scale] {
                assert_eq!(
                    draw_gizmo(
                        &painter,
                        &camera,
                        viewport,
                        Vec3::ZERO,
                        kind,
                        Some(Pos2::new(460.0, 400.0))
                    ),
                    Some(GizmoHandle::Axis(0))
                );
            }
            assert_eq!(
                draw_gizmo(
                    &painter,
                    &camera,
                    viewport,
                    Vec3::ZERO,
                    GizmoKind::Translate,
                    Some(Pos2::new(424.0, 376.0))
                ),
                Some(GizmoHandle::Plane(2))
            );
            assert_eq!(
                draw_gizmo(
                    &painter,
                    &camera,
                    viewport,
                    Vec3::ZERO,
                    GizmoKind::Translate,
                    Some(Pos2::new(700.0, 700.0))
                ),
                None
            );
        });
    }

    #[test]
    fn edge_on_plane_cannot_capture_arbitrary_clicks() {
        let line = [
            Pos2::new(0.0, 0.0),
            Pos2::new(1.0, 0.0),
            Pos2::new(2.0, 0.0),
            Pos2::new(3.0, 0.0),
        ];
        assert!(!inside_convex(&line, Pos2::new(100.0, 0.0)));
    }

    #[test]
    fn convex_plane_hit_is_winding_independent() {
        let mut square = vec![
            Pos2::new(0.0, 0.0),
            Pos2::new(10.0, 0.0),
            Pos2::new(10.0, 10.0),
            Pos2::new(0.0, 10.0),
        ];
        for _ in 0..2 {
            assert!(inside_convex(&square, Pos2::new(5.0, 5.0)));
            assert!(!inside_convex(&square, Pos2::new(11.0, 5.0)));
            square.reverse();
        }
    }
}
