//! Adaptador de eventos egui para a transação modal do domínio.
use egui::{Key, PointerButton, Pos2, Rect, Vec2};
use glam::Vec3;
use petunia_core::modal::{ModalConstraint, ModalKind};
use petunia_core::{AppState, Projection};

#[derive(Clone)]
struct PointerSession {
    anchor: Pos2,
    numeric: String,
    drag_handle: bool,
    valid_preview: bool,
    last_pos: Pos2,
}

pub fn start_handle(
    ctx: &egui::Context,
    state: &mut AppState,
    kind: ModalKind,
    constraint: ModalConstraint,
    anchor: Pos2,
) {
    match state.begin_modal(kind) {
        Ok(()) => {
            if let Err(error) = state.set_modal_constraint(constraint) {
                state.set_status(error.to_string());
            }
            ctx.data_mut(|d| {
                d.insert_temp(
                    egui::Id::new("modal.pointer"),
                    PointerSession {
                        anchor,
                        numeric: String::new(),
                        drag_handle: true,
                        valid_preview: true,
                        last_pos: anchor,
                    },
                )
            });
        }
        Err(error) => state.set_status(error.to_string()),
    }
}

/// Retorna true enquanto a sessão possui o viewport (inclusive no frame de término).
pub fn draw(
    ctx: &egui::Context,
    state: &mut AppState,
    rect: Rect,
    painter: &egui::Painter,
) -> bool {
    if crate::tool_fields::owns_modal(ctx) { return state.modal.is_some(); }
    let id = egui::Id::new("modal.pointer");
    let mut started = false;
    if let Some(kind) = state.pending_modal {
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            state.pending_modal = None;
            return true;
        }
        if let Some(pos) = ctx.pointer_hover_pos().filter(|pos| rect.contains(*pos)) {
            state.pending_modal = None;
            match state.begin_modal(kind) {
                Ok(()) => {
                    ctx.data_mut(|d| {
                        d.insert_temp(
                            id,
                            PointerSession {
                                anchor: pos,
                                numeric: String::new(),
                                drag_handle: false,
                                valid_preview: true,
                                last_pos: pos,
                            },
                        )
                    });
                    started = true;
                }
                Err(error) => state.set_status(error.to_string()),
            }
        } else {
            painter.text(
                rect.center_top() + egui::vec2(0.0, 24.0),
                egui::Align2::CENTER_TOP,
                "Mova o cursor para o viewport · Esc: cancelar",
                egui::FontId::proportional(14.0),
                egui::Color32::WHITE,
            );
            return true;
        }
    }
    let Some(op) = state.modal.as_ref() else {
        return started;
    };
    let (kind, pivot, normal) = (op.kind, op.pivot, op.normal);
    if matches!(kind, ModalKind::Move | ModalKind::Rotate | ModalKind::Scale) {
        crate::gizmo::draw_gizmo(
            painter,
            &state.camera,
            rect,
            pivot,
            match kind {
                ModalKind::Rotate => crate::gizmo::GizmoKind::Rotate,
                ModalKind::Scale => crate::gizmo::GizmoKind::Scale,
                _ => crate::gizmo::GizmoKind::Translate,
            },
            ctx.pointer_hover_pos(),
        );
    }
    let mut pointer = ctx
        .data_mut(|d| d.get_temp::<PointerSession>(id))
        .unwrap_or(PointerSession {
            anchor: rect.center(),
            numeric: String::new(),
            drag_handle: false,
            valid_preview: true,
            last_pos: rect.center(),
        });
    let pos = ctx.pointer_hover_pos().unwrap_or(pointer.last_pos);
    pointer.last_pos = pos;
    ctx.set_cursor_icon(egui::CursorIcon::Crosshair);
    let cancel = ctx.input(|i| {
        i.key_pressed(Key::Escape) || i.pointer.button_pressed(PointerButton::Secondary)
    });
    if cancel {
        state.cancel_modal();
        ctx.data_mut(|d| d.remove::<PointerSession>(id));
        return true;
    }
    let mut changed = started || ctx.input(|i| !i.events.is_empty());
    for (key, axis) in [(Key::X, 0), (Key::Y, 1), (Key::Z, 2)] {
        if ctx.input(|i| i.key_pressed(key)) {
            let constraint = if ctx.input(|i| i.modifiers.shift) {
                ModalConstraint::Plane(axis)
            } else {
                ModalConstraint::Axis(axis)
            };
            let current = state.modal.as_ref().map(|op| op.constraint);
            if let Err(error) = state.set_modal_constraint(if current == Some(constraint) {
                ModalConstraint::Free
            } else {
                constraint
            }) {
                state.set_status(error.to_string());
            }
            changed = true;
        }
    }
    ctx.input(|i| {
        for event in &i.events {
            if matches!(
                event,
                egui::Event::Key {
                    key: Key::Backspace,
                    pressed: true,
                    ..
                }
            ) {
                pointer.numeric.pop();
            }
            if let egui::Event::Text(text) = event {
                if !i.modifiers.command
                    && !i.modifiers.ctrl
                    && text
                        .chars()
                        .all(|c| c.is_ascii_digit() || matches!(c, '.' | ',' | '-' | '+'))
                    && pointer.numeric.len() + text.len() <= 64
                {
                    pointer.numeric.push_str(&text.replace(',', "."));
                }
            }
        }
    });
    let constraint = state
        .modal
        .as_ref()
        .map(|op| op.constraint)
        .unwrap_or(ModalConstraint::Free);
    let delta = pos - pointer.anchor;
    let units = world_per_pixel(state, rect, pivot);
    let translation = (state.camera.right() * delta.x - state.camera.up() * delta.y) * units;
    let axis = match constraint {
        ModalConstraint::Axis(axis) | ModalConstraint::Plane(axis) => [Vec3::X, Vec3::Y, Vec3::Z]
            .get(axis)
            .copied()
            .unwrap_or(Vec3::X),
        _ => normal,
    };
    let translation = match constraint {
        ModalConstraint::Plane(axis) => {
            let normal = [Vec3::X, Vec3::Y, Vec3::Z][axis];
            match (
                plane_point(state, rect, pointer.anchor, pivot, normal),
                plane_point(state, rect, pos, pivot, normal),
            ) {
                (Some(start), Some(end)) => end - start,
                _ => translation - normal * translation.dot(normal),
            }
        }
        _ => translation,
    };
    let mut value = match kind {
        ModalKind::Move if !matches!(constraint, ModalConstraint::Axis(_)) => translation.length(),
        ModalKind::Move | ModalKind::Extrude | ModalKind::PushPull => {
            projected_distance(state, rect, pivot, axis, delta, units)
        }
        ModalKind::Rotate => {
            match (
                plane_point(state, rect, pointer.anchor, pivot, axis),
                plane_point(state, rect, pos, pivot, axis),
            ) {
                (Some(a), Some(b))
                    if (a - pivot).length() > units * 12.0
                        && (b - pivot).length() > units * 12.0 =>
                {
                    let a = (a - pivot).normalize();
                    let b = (b - pivot).normalize();
                    axis.dot(a.cross(b)).atan2(a.dot(b)).to_degrees()
                }
                _ => delta.x * 0.5,
            }
        }
        ModalKind::Scale => {
            if matches!(constraint, ModalConstraint::Axis(_)) {
                1.0 + projected_distance(state, rect, pivot, axis, delta, units) / (80.0 * units)
            } else {
                1.0 + delta.x * 0.01
            }
        }
        ModalKind::Inset => delta.x * 0.005,
        ModalKind::Bevel => delta.x * units,
    };
    let numeric = if pointer.numeric.is_empty() {
        None
    } else {
        pointer
            .numeric
            .parse::<f32>()
            .ok()
            .filter(|n| n.is_finite())
    };
    let valid_number = pointer.numeric.is_empty() || numeric.is_some();
    if let Some(number) = numeric {
        value = number;
    }
    let snap = ctx.input(|i| i.modifiers.ctrl);
    let translation = if snap && numeric.is_none() {
        value = snap_value(value, if kind == ModalKind::Rotate { 15.0 } else { 0.1 });
        (translation / 0.1).round() * 0.1
    } else {
        translation
    };
    let translation = if numeric.is_some()
        && !matches!(constraint, ModalConstraint::Axis(_))
        && kind == ModalKind::Move
    {
        let fallback = match constraint {
            ModalConstraint::Plane(axis) => [Vec3::Y, Vec3::Z, Vec3::X][axis],
            _ => state.camera.right(),
        };
        translation.try_normalize().unwrap_or(fallback) * value
    } else {
        translation
    };
    let mut valid_preview = valid_number && pointer.valid_preview;
    if changed && valid_number {
        valid_preview = true;
        if let Err(error) = state.update_modal(translation, value) {
            state.set_status(error.to_string());
            valid_preview = false;
        }
    }
    pointer.valid_preview = valid_preview;
    let confirm = !started
        && ctx.input(|i| {
            i.key_pressed(Key::Enter)
                || (rect.contains(pos)
                    && if pointer.drag_handle {
                        i.pointer.button_released(PointerButton::Primary)
                    } else {
                        i.pointer.button_pressed(PointerButton::Primary)
                    })
        });
    if confirm && valid_preview {
        state.commit_modal();
        ctx.data_mut(|d| d.remove::<PointerSession>(id));
        return true;
    }
    painter.line_segment(
        [pointer.anchor, pos],
        egui::Stroke::new(1.0_f32, egui::Color32::LIGHT_BLUE),
    );
    painter.circle_stroke(
        pointer.anchor,
        4.0,
        egui::Stroke::new(1.0_f32, egui::Color32::WHITE),
    );
    let unit = match kind {
        ModalKind::Rotate => "°",
        ModalKind::Scale => "×",
        ModalKind::Inset => " fator",
        _ => " m",
    };
    let label = match kind {
        ModalKind::Move => "Mover",
        ModalKind::Rotate => "Rotacionar",
        ModalKind::Scale => "Escalar",
        ModalKind::Extrude => "Extrude",
        ModalKind::Inset => "Inset",
        ModalKind::Bevel => "Bevel",
        ModalKind::PushPull => "Push/Pull",
    };
    let value_label = if pointer.numeric.is_empty() {
        format!("{value:+.3}")
    } else {
        pointer.numeric.clone()
    };
    let text = format!("{label}: {value_label}{unit}  {constraint:?}\nCtrl: snap · X/Y/Z: eixo · Shift: plano\nEnter/LMB: aplicar · Esc/RMB: cancelar{}",
        if valid_preview { "" } else { "\nValor inválido — ajuste ou cancele" });
    let galley = painter.layout_no_wrap(text, egui::FontId::monospace(12.0), egui::Color32::WHITE);
    let hud_pos = Pos2::new(
        (pos.x + 18.0)
            .min(rect.right() - galley.size().x - 12.0)
            .max(rect.left() + 8.0),
        (pos.y + 18.0)
            .min(rect.bottom() - galley.size().y - 12.0)
            .max(rect.top() + 8.0),
    );
    painter.rect_filled(
        Rect::from_min_size(
            hud_pos - Vec2::splat(6.0),
            galley.size() + Vec2::splat(12.0),
        ),
        5.0,
        egui::Color32::from_black_alpha(225),
    );
    painter.galley(hud_pos, galley, egui::Color32::WHITE);
    ctx.data_mut(|d| d.insert_temp(id, pointer));
    true
}

fn snap_value(value: f32, step: f32) -> f32 {
    (value / step).round() * step
}

fn world_per_pixel(state: &AppState, rect: Rect, pivot: Vec3) -> f32 {
    let half_height = match state.camera.proj {
        Projection::Ortho => state.camera.ortho_half_h,
        Projection::Perspective => {
            (pivot - state.camera.eye())
                .dot(state.camera.forward())
                .max(0.01)
                * (state.camera.fov_y * 0.5).tan()
        }
    };
    2.0 * half_height / rect.height().max(1.0)
}

fn projected_distance(
    state: &AppState,
    rect: Rect,
    pivot: Vec3,
    axis: Vec3,
    delta: Vec2,
    units: f32,
) -> f32 {
    let a = state.camera.project_ndc(pivot);
    let b = state.camera.project_ndc(pivot + axis);
    let projected = Vec2::new(
        (b.x - a.x) * rect.width() * 0.5,
        (a.y - b.y) * rect.height() * 0.5,
    );
    if projected.length_sq() < 16.0 {
        delta.x * units
    } else {
        delta.dot(projected) / projected.length_sq()
    }
}

fn plane_point(state: &AppState, rect: Rect, pos: Pos2, pivot: Vec3, normal: Vec3) -> Option<Vec3> {
    let nx = (pos.x - rect.left()) / rect.width() * 2.0 - 1.0;
    let ny = 1.0 - (pos.y - rect.top()) / rect.height() * 2.0;
    let (origin, direction) = state.camera.ray(nx, ny);
    let denominator = direction.dot(normal);
    if denominator.abs() < 1e-5 {
        return None;
    }
    let distance = (pivot - origin).dot(normal) / denominator;
    let point = origin + direction * distance;
    (distance >= 0.0 && point.is_finite()).then_some(point)
}
