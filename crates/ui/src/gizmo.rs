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
    Center,
    Axis(u8),
    /// Normal axis: 0 = YZ, 1 = XZ, 2 = XY.
    Plane(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GizmoInteraction {
    pub kind: GizmoKind,
    pub handle: GizmoHandle,
}

const COLORS: [Color32; 3] = [
    Color32::from_rgb(240, 85, 85),
    Color32::from_rgb(100, 220, 120),
    Color32::from_rgb(95, 150, 255),
];
const AXES: [Vec3; 3] = [Vec3::X, Vec3::Y, Vec3::Z];

/// Visual gizmo extent in logical points. Large viewports retain the familiar
/// 80pt reach; narrow panes shrink the gizmo instead of covering the model.
fn gizmo_extent_points(viewport: Rect) -> f32 {
    (viewport.width().min(viewport.height()) * 0.14).clamp(56.0, 80.0)
}

/// Calcula eixos de coordenadas locais a partir da seleção da malha.
pub fn local_axes_for_mesh(mesh: &petunia_mesh::Mesh) -> [Vec3; 3] {
    let mut normal = Vec3::ZERO;
    let mut count = 0;
    for (fi, f) in mesh.faces.iter().enumerate() {
        if f.selected {
            normal += mesh.face_normal(fi);
            count += 1;
        }
    }
    // Se nenhuma face estiver explicitamente selecionada (ex: Object Mode ou seleção de arestas):
    if count == 0 {
        // 1. Tenta aresta selecionada
        if let Some(&(a, b)) = mesh.selected_edges.iter().next()
            && let (Some(va), Some(vb)) = (mesh.verts.get(a as usize), mesh.verts.get(b as usize))
        {
            let dir = (vb.vec() - va.vec()).normalize_or_zero();
            if dir.length_squared() > 1e-4 {
                let up = if dir.y.abs() < 0.95 { Vec3::Y } else { Vec3::Z };
                let y = dir.cross(up).normalize();
                let z = dir.cross(y).normalize();
                return [dir, y, z];
            }
        }
        // 2. Se for o objeto inteiro (Object Mode) ou sem seleção, orienta pela primeira face da malha
        if !mesh.faces.is_empty() {
            normal = mesh.face_normal(0);
            count = 1;
        }
    }
    if count > 0 && normal.length_squared() > 1e-4 {
        let z = normal.normalize();
        let up = if z.y.abs() < 0.95 { Vec3::Y } else { Vec3::Z };
        let x = up.cross(z).normalize();
        let y = z.cross(x).normalize();
        [x, y, z]
    } else {
        AXES
    }
}

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
    draw_gizmo_oriented(painter, camera, viewport, pivot, kind, pointer, AXES)
}

/// Draws foreground handles with oriented coordinate axes.
pub fn draw_gizmo_oriented(
    painter: &Painter,
    camera: &Camera,
    viewport: Rect,
    pivot: Vec3,
    kind: GizmoKind,
    pointer: Option<Pos2>,
    axes: [Vec3; 3],
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
    let length = world_per_point * gizmo_extent_points(viewport);
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

    // Alça Central para Movimento / Transformação Livre (View-Plane Free Translation)
    let center_radius = 8.5_f32;
    let center_dist = pointer.map_or(f32::INFINITY, |p| p.distance(center));
    let center_hovered = center_dist <= center_radius;
    let center_color = if center_hovered {
        Color32::WHITE
    } else {
        Color32::from_white_alpha(180)
    };
    painter.circle(
        center,
        center_radius,
        Color32::from_black_alpha(100),
        Stroke::new(1.5, center_color),
    );
    painter.circle_filled(center, 3.0, center_color);
    if center_hovered {
        best = Some((center_dist, GizmoHandle::Center));
    }

    if kind == GizmoKind::Translate {
        for (normal, &base_color) in COLORS.iter().enumerate() {
            let a = axes[(normal + 1) % 3] * length;
            let b = axes[(normal + 2) % 3] * length;
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
            let a = axes[(axis + 1) % 3] * length * 0.8;
            let b = axes[(axis + 2) % 3] * length * 0.8;
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
                project(pivot + axes[axis] * length * 0.12),
                project(pivot + axes[axis] * length),
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
    best.map(|(_, handle)| handle)
}

/// Combined Move/Rotate/Scale gizmo for the Universal Transform tool.
pub fn draw_universal_gizmo_oriented(
    painter: &Painter,
    camera: &Camera,
    viewport: Rect,
    pivot: Vec3,
    pointer: Option<Pos2>,
    axes: [Vec3; 3],
) -> Option<GizmoInteraction> {
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
    let length = world_per_point * gizmo_extent_points(viewport);
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
    let mut best: Option<(f32, GizmoInteraction)> = None;
    let mut consider = |distance: f32, kind: GizmoKind, handle: GizmoHandle| {
        if best.is_none_or(|(previous, _)| distance < previous) {
            best = Some((distance, GizmoInteraction { kind, handle }));
        }
    };

    let center_distance = pointer.map_or(f32::INFINITY, |point| point.distance(center));
    let center_hovered = center_distance <= 7.0;
    painter.circle(
        center,
        7.0,
        Color32::from_black_alpha(115),
        Stroke::new(
            1.5,
            if center_hovered {
                Color32::WHITE
            } else {
                Color32::from_white_alpha(190)
            },
        ),
    );
    painter.circle_filled(center, 2.5, Color32::from_white_alpha(210));
    if center_hovered {
        consider(center_distance, GizmoKind::Translate, GizmoHandle::Center);
    }

    for (normal, &base_color) in COLORS.iter().enumerate() {
        let a = axes[(normal + 1) % 3] * length;
        let b = axes[(normal + 2) % 3] * length;
        let corners: Option<Vec<_>> = [(0.18, 0.18), (0.34, 0.18), (0.34, 0.34), (0.18, 0.34)]
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
            color.gamma_multiply(0.20),
            Stroke::new(1.0, color),
        ));
        if hovered {
            consider(5.0, GizmoKind::Translate, GizmoHandle::Plane(normal as u8));
        }
    }

    for (axis, &base_color) in COLORS.iter().enumerate() {
        let a = axes[(axis + 1) % 3] * length * 0.58;
        let b = axes[(axis + 2) % 3] * length * 0.58;
        let mut previous = None;
        let mut ring_lines = Vec::new();
        let mut ring_distance = f32::INFINITY;
        for step in 0..=48 {
            let angle = step as f32 / 48.0 * std::f32::consts::TAU;
            let current = project(pivot + a * angle.cos() + b * angle.sin());
            if let (Some(start), Some(end)) = (previous, current) {
                if let Some(pointer) = pointer {
                    ring_distance = ring_distance.min(segment_distance(pointer, start, end));
                }
                ring_lines.push([start, end]);
            }
            previous = current;
        }
        let ring_hovered = ring_distance <= 5.0;
        for line in ring_lines {
            painter.line_segment(
                line,
                Stroke::new(
                    if ring_hovered { 3.0 } else { 1.5 },
                    if ring_hovered {
                        Color32::WHITE
                    } else {
                        base_color.gamma_multiply(0.75)
                    },
                ),
            );
        }
        if ring_hovered {
            consider(
                ring_distance,
                GizmoKind::Rotate,
                GizmoHandle::Axis(axis as u8),
            );
        }

        let (Some(start), Some(move_end)) = (
            project(pivot + axes[axis] * length * 0.12),
            project(pivot + axes[axis] * length * 0.82),
        ) else {
            continue;
        };
        if start.distance(move_end) < 10.0 {
            continue;
        }
        let move_distance = pointer.map_or(f32::INFINITY, |point| {
            segment_distance(point, start, move_end)
        });
        let move_hovered = move_distance <= 6.0;
        let move_color = if move_hovered {
            Color32::WHITE
        } else {
            base_color
        };
        painter.line_segment(
            [start, move_end],
            Stroke::new(5.0, Color32::from_black_alpha(170)),
        );
        painter.line_segment([start, move_end], Stroke::new(2.5, move_color));
        let direction = (move_end - start).normalized();
        let perpendicular = Vec2::new(-direction.y, direction.x);
        painter.add(Shape::convex_polygon(
            vec![
                move_end,
                move_end - direction * 11.0 + perpendicular * 4.5,
                move_end - direction * 11.0 - perpendicular * 4.5,
            ],
            move_color,
            Stroke::NONE,
        ));
        if move_hovered {
            consider(
                move_distance,
                GizmoKind::Translate,
                GizmoHandle::Axis(axis as u8),
            );
        }

        if let Some(scale_end) = project(pivot + axes[axis] * length * 1.04) {
            let scale_rect = Rect::from_center_size(scale_end, Vec2::splat(9.0));
            let scale_hovered = pointer.is_some_and(|point| scale_rect.expand(3.0).contains(point));
            let scale_color = if scale_hovered {
                Color32::WHITE
            } else {
                base_color
            };
            painter.line_segment(
                [move_end, scale_end],
                Stroke::new(1.5, base_color.gamma_multiply(0.65)),
            );
            painter.rect_filled(scale_rect, 1.5, scale_color);
            if scale_hovered {
                consider(1.0, GizmoKind::Scale, GizmoHandle::Axis(axis as u8));
            }
        }
    }

    best.map(|(_, interaction)| interaction)
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
        context
            .run_ui(egui::RawInput::default(), |_ui| {
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
            })
            .textures_delta
            .clear();
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

    #[test]
    fn gizmo_extent_is_responsive_but_keeps_accessible_floor() {
        let tiny = Rect::from_min_size(Pos2::ZERO, Vec2::new(220.0, 180.0));
        let medium = Rect::from_min_size(Pos2::ZERO, Vec2::new(520.0, 420.0));
        let large = Rect::from_min_size(Pos2::ZERO, Vec2::new(1280.0, 800.0));
        assert_eq!(gizmo_extent_points(tiny), 56.0);
        assert!(gizmo_extent_points(medium) > gizmo_extent_points(tiny));
        assert_eq!(gizmo_extent_points(large), 80.0);
    }
}
