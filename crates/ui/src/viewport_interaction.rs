//! Overlays e arbitragem: modal > gizmo > navegação > seleção.
use crate::{
    gizmo::{draw_gizmo, GizmoHandle, GizmoKind},
    modal_viewport,
};
use egui::{PointerButton, Pos2, Rect};
use glam::{Vec2, Vec3};
use petunia_core::picking::{pick_mesh, PickComponent};
use petunia_core::{AppState, EditMode, ModalConstraint, ModalKind, SelectMode, Workspace};

pub fn draw(
    ctx: &egui::Context,
    state: &mut AppState,
    rect: Rect,
    painter: &egui::Painter,
    response: &egui::Response,
) -> bool {
    if let Some((start, goal, elapsed)) = state.camera_frame.take() {
        let elapsed = elapsed + ctx.input(|i| i.stable_dt).clamp(0.001, 0.033);
        let t = (elapsed / 0.25).min(1.0);
        let eased = t * t * (3.0 - 2.0 * t);
        state.camera.target = start.target.lerp(goal.target, eased);
        state.camera.distance = start.distance + (goal.distance - start.distance) * eased;
        state.camera.ortho_half_h =
            start.ortho_half_h + (goal.ortho_half_h - start.ortho_half_h) * eased;
        if t < 1.0 {
            state.camera_frame = Some((start, goal, elapsed));
        }
        state.mark_dirty();
    }
    if modal_viewport::draw(ctx, state, rect, painter) {
        return true;
    }
    if crate::cutting::draw(ctx, state, rect, painter) {
        return true;
    }
    let pointer = ctx.pointer_hover_pos().filter(|p| rect.contains(*p));
    // Navegação usa deltas por frame e a base da câmera, inclusive nas vistas Top/Bottom.
    if response.dragged_by(PointerButton::Middle) {
        state.camera_frame = None;
        let delta = ctx.input(|i| i.pointer.delta());
        if ctx.input(|i| i.modifiers.shift) {
            state.camera.pan(delta.x, delta.y);
        } else {
            state.camera.orbit(delta.x, delta.y);
        }
        if delta != egui::Vec2::ZERO {
            state.mark_dirty();
        }
        return true;
    }
    if pointer.is_some() {
        let scroll = ctx.input(|i| i.raw_scroll_delta.y);
        if scroll != 0.0 {
            state.camera_frame = None;
            state.camera.zoom(scroll);
            state.mark_dirty();
        }
    }

    // Overlays 3D e controles de navegação do viewport (Blender.svg Golden Reference)
    crate::nav_gizmo::draw_3d_cursor(state, rect, painter);
    if crate::nav_gizmo::handle_3d_cursor_placement(ctx, state, rect) {
        return true;
    }
    if crate::nav_gizmo::draw_nav_gizmo(ctx, state, rect, painter) {
        return true;
    }
    crate::nav_gizmo::draw_context_menu(ctx, state);
    if pointer.is_some()
        && ctx.input(|i| !i.modifiers.shift && i.pointer.button_clicked(PointerButton::Secondary))
    {
        if let Some(pos) = pointer {
            state.context_menu_pos = Some([pos.x, pos.y]);
            return true;
        }
    }

    let paint = state.workspace == Workspace::Paint
        || state.mode == EditMode::TexturePaint
        || state.active_tool == "paint";
    if paint {
        return paint_preview(ctx, state, rect, painter, response);
    }
    if state.workspace != Workspace::Model || state.active_tool == "draw_profile" {
        return false;
    }
    let mode = if state.mode == EditMode::Object {
        SelectMode::Face
    } else {
        state.select_mode
    };
    let hit = pointer.and_then(|pos| {
        state.project.active_mesh().and_then(|mesh| {
            pick_mesh(
                mesh,
                &state.camera,
                Vec2::new(rect.width(), rect.height()) * ctx.pixels_per_point(),
                ndc(rect, pos),
                mode,
                state.shading == petunia_render::Shading::Wireframe,
            )
        })
    });
    if let (Some(hit), Some(mesh)) = (hit, state.project.active_mesh()) {
        let color = egui::Color32::from_rgb(125, 220, 255);
        let project = |p| screen(state, rect, p);
        match hit.component {
            PickComponent::Vertex(i) => {
                if let Some(v) = mesh.verts.get(i) {
                    painter.circle_stroke(project(v.vec()), 6.0, egui::Stroke::new(2.0_f32, color));
                }
            }
            PickComponent::Edge(a, b) => {
                if let (Some(a), Some(b)) = (mesh.verts.get(a as usize), mesh.verts.get(b as usize))
                {
                    painter.line_segment(
                        [project(a.vec()), project(b.vec())],
                        egui::Stroke::new(3.0_f32, color),
                    );
                }
            }
            PickComponent::Face(i) => {
                if let Some(face) = mesh.faces.get(i) {
                    for k in 0..face.verts.len() {
                        if let (Some(a), Some(b)) = (
                            mesh.verts.get(face.verts[k] as usize),
                            mesh.verts
                                .get(face.verts[(k + 1) % face.verts.len()] as usize),
                        ) {
                            painter.line_segment(
                                [project(a.vec()), project(b.vec())],
                                egui::Stroke::new(2.5_f32, color),
                            );
                        }
                    }
                }
            }
        }
    }
    if let Some(mesh) = state.project.active_mesh().filter(|m| m.has_selection()) {
        let pivot = Vec3::from_array(mesh.selection_center());
        let kind = match state.gizmo_mode {
            ModalKind::Rotate => GizmoKind::Rotate,
            ModalKind::Scale => GizmoKind::Scale,
            _ => GizmoKind::Translate,
        };
        if let Some(handle) = draw_gizmo(painter, &state.camera, rect, pivot, kind, pointer) {
            ctx.set_cursor_icon(egui::CursorIcon::Grab);
            if ctx.input(|i| i.pointer.button_pressed(PointerButton::Primary)) {
                if let Some(pos) = pointer {
                    let constraint = match handle {
                        GizmoHandle::Axis(a) => ModalConstraint::Axis(a as usize),
                        GizmoHandle::Plane(a) => ModalConstraint::Plane(a as usize),
                    };
                    modal_viewport::start_handle(ctx, state, state.gizmo_mode, constraint, pos);
                    state.box_select_start = None;
                    return true;
                }
            }
        }
    }
    response.context_menu(|ui| {
        ui.label("Modelagem");
        for (label, kind) in [
            ("Mover · G", ModalKind::Move),
            ("Rotacionar · R", ModalKind::Rotate),
            ("Escalar · S", ModalKind::Scale),
        ] {
            if ui.button(label).clicked() {
                state.pending_modal = Some(kind);
                ui.close();
            }
        }
        let faces = state
            .project
            .active_mesh()
            .is_some_and(|m| m.selected_face_count() > 0);
        if faces {
            for (label, kind) in [
                ("Extrude · E", ModalKind::Extrude),
                ("Inset · I", ModalKind::Inset),
                ("Push/Pull · P", ModalKind::PushPull),
            ] {
                if ui.button(label).clicked() {
                    state.pending_modal = Some(kind);
                    ui.close();
                }
            }
        }
        if state
            .project
            .active_mesh()
            .is_some_and(|m| !m.selected_edges.is_empty())
            && ui.button("Bevel · Ctrl+B").clicked()
        {
            state.pending_modal = Some(ModalKind::Bevel);
            ui.close();
        }
    });
    false
}

fn paint_preview(
    ctx: &egui::Context,
    state: &mut AppState,
    rect: Rect,
    painter: &egui::Painter,
    response: &egui::Response,
) -> bool {
    let radius_id = egui::Id::new("paint.radius");
    let cancelled_id = egui::Id::new("paint.cancelled_until_release");
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        state.finish_paint_stroke(true);
        if let Some((_, radius)) = ctx.data_mut(|d| d.get_temp::<(Pos2, f32)>(radius_id)) {
            state.paint_radius = radius;
            ctx.data_mut(|d| d.remove::<(Pos2, f32)>(radius_id));
        }
        if ctx.input(|i| i.pointer.button_down(PointerButton::Primary)) {
            ctx.data_mut(|d| d.insert_temp(cancelled_id, true));
        } else {
            ctx.data_mut(|d| d.remove::<bool>(cancelled_id));
        }
        return true;
    }
    if ctx.input(|i| i.pointer.button_released(PointerButton::Primary)) {
        state.finish_paint_stroke(false);
        ctx.data_mut(|d| d.remove::<bool>(cancelled_id));
    }
    if ctx.input(|i| i.pointer.button_pressed(PointerButton::Primary)) {
        ctx.data_mut(|d| d.remove::<bool>(cancelled_id));
    }
    if ctx
        .data_mut(|d| d.get_temp::<bool>(cancelled_id))
        .unwrap_or(false)
    {
        return true;
    }
    let Some(pos) = ctx.pointer_hover_pos().filter(|p| rect.contains(*p)) else {
        return true;
    };
    if ctx.input(|i| i.key_pressed(egui::Key::F)) {
        ctx.data_mut(|d| d.insert_temp(radius_id, (pos, state.paint_radius)));
    }
    if let Some((anchor, radius)) = ctx.data_mut(|d| d.get_temp::<(Pos2, f32)>(radius_id)) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            state.paint_radius = radius;
            ctx.data_mut(|d| d.remove::<(Pos2, f32)>(radius_id));
        } else {
            state.paint_radius = (radius + (pos.x - anchor.x) * 0.01).clamp(0.01, 100.0);
            if ctx.input(|i| {
                i.pointer.button_pressed(PointerButton::Primary) || i.key_pressed(egui::Key::Enter)
            }) {
                ctx.data_mut(|d| d.remove::<(Pos2, f32)>(radius_id));
            }
        }
        painter.text(
            pos + egui::vec2(18.0, 18.0),
            egui::Align2::LEFT_TOP,
            format!("Raio: {:.2} m", state.paint_radius),
            egui::FontId::monospace(14.0),
            egui::Color32::WHITE,
        );
        return true;
    }
    let point = ndc(rect, pos);
    let hit = state.project.active_mesh().and_then(|mesh| {
        pick_mesh(
            mesh,
            &state.camera,
            Vec2::new(rect.width(), rect.height()) * ctx.pixels_per_point(),
            point,
            SelectMode::Face,
            false,
        )
    });
    if let Some(hit) = hit {
        if let PickComponent::Face(face) = hit.component {
            let normal = state
                .project
                .active_mesh()
                .map(|m| m.face_normal(face))
                .unwrap_or(Vec3::Y);
            let tangent = normal
                .cross(if normal.y.abs() < 0.9 {
                    Vec3::Y
                } else {
                    Vec3::X
                })
                .normalize_or_zero();
            let bitangent = normal.cross(tangent);
            for step in 0..48 {
                let a = step as f32 / 48.0 * std::f32::consts::TAU;
                let b = (step + 1) as f32 / 48.0 * std::f32::consts::TAU;
                painter.line_segment(
                    [
                        screen(
                            state,
                            rect,
                            hit.position
                                + (tangent * a.cos() + bitangent * a.sin()) * state.paint_radius,
                        ),
                        screen(
                            state,
                            rect,
                            hit.position
                                + (tangent * b.cos() + bitangent * b.sin()) * state.paint_radius,
                        ),
                    ],
                    egui::Stroke::new(1.5_f32, egui::Color32::WHITE),
                );
            }
            let sample = ctx.input(|i| i.modifiers.alt || i.key_pressed(egui::Key::G));
            if sample {
                if ctx.input(|i| {
                    i.pointer.button_pressed(PointerButton::Primary) || i.key_pressed(egui::Key::G)
                }) {
                    if let Some(mesh) = state.project.active_mesh() {
                        if let Some(vertex) = mesh
                            .faces
                            .get(face)
                            .into_iter()
                            .flat_map(|f| &f.verts)
                            .filter_map(|&v| mesh.verts.get(v as usize))
                            .min_by(|a, b| {
                                (a.vec() - hit.position)
                                    .length_squared()
                                    .total_cmp(&(b.vec() - hit.position).length_squared())
                            })
                        {
                            state.paint_color = vertex.color;
                            state.mark_dirty();
                        }
                    }
                }
            } else if (response.dragged_by(PointerButton::Primary)
                && ctx.input(|i| i.pointer.delta() != egui::Vec2::ZERO))
                || ctx.input(|i| i.pointer.button_pressed(PointerButton::Primary))
            {
                state.begin_paint_stroke();
                state.paint_at(hit.position);
            }
        }
    }
    true
}
fn ndc(rect: Rect, p: Pos2) -> Vec2 {
    Vec2::new(
        (p.x - rect.left()) / rect.width() * 2.0 - 1.0,
        1.0 - (p.y - rect.top()) / rect.height() * 2.0,
    )
}
fn screen(state: &AppState, rect: Rect, p: Vec3) -> Pos2 {
    let q = state.camera.project_ndc(p);
    Pos2::new(
        rect.center().x + q.x * rect.width() * 0.5,
        rect.center().y - q.y * rect.height() * 0.5,
    )
}
