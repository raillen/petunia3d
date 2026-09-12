//! Fluxos de faca, plano de corte e loop cut com checkpoint ao confirmar.
use egui::{Key, PointerButton, Pos2, Rect};
use glam::{Vec2, Vec3};
use petunia_core::picking::{pick_mesh, PickComponent};
use petunia_core::{AppState, SelectMode};
use petunia_mesh::{knife::EdgePoint, loop_cut::LoopRing, Mesh};

#[derive(Clone)]
struct CutSession {
    source: Mesh,
    anchor: Option<Pos2>,
    edge_start: Option<EdgePoint>,
    ring: Option<LoopRing>,
    cuts: usize,
    sliding: bool,
}

pub fn draw(
    ctx: &egui::Context,
    state: &mut AppState,
    rect: Rect,
    painter: &egui::Painter,
) -> bool {
    let tool = state.active_tool.clone();
    if !matches!(tool.as_str(), "knife" | "slice" | "loop_cut") {
        return false;
    }
    let id = egui::Id::new("cut.session");
    if state.mesh_preview.is_none() {
        let Some(mesh) = state.project.active_mesh().cloned() else {
            return true;
        };
        if !state.begin_mesh_preview(match tool.as_str() {
            "knife" => "Knife",
            "slice" => "Slice",
            _ => "Loop cut",
        }) {
            return false;
        }
        ctx.data_mut(|d| {
            d.insert_temp(
                id,
                CutSession {
                    source: mesh,
                    anchor: None,
                    edge_start: None,
                    ring: None,
                    cuts: 1,
                    sliding: false,
                },
            )
        });
    }
    let Some(mut session) = ctx.data_mut(|d| d.get_temp::<CutSession>(id)) else {
        state.finish_mesh_preview(true);
        return true;
    };
    let cancel = ctx.input(|i| {
        i.key_pressed(Key::Escape) || i.pointer.button_pressed(PointerButton::Secondary)
    });
    if cancel
        && !(tool == "loop_cut" && session.sliding && !ctx.input(|i| i.key_pressed(Key::Escape)))
    {
        finish(ctx, state, id, true);
        return true;
    }
    if ctx.input(|i| i.key_pressed(Key::Enter)) {
        finish(ctx, state, id, false);
        return true;
    }
    if cancel && tool == "loop_cut" && session.sliding {
        if let Some(ring) = &session.ring {
            if let Ok(mesh) = ring.apply(&session.source, session.cuts, 0.0) {
                state.preview_mesh(mesh);
                finish(ctx, state, id, false);
                return true;
            }
        }
    }
    let Some(pos) = ctx.pointer_hover_pos().filter(|p| rect.contains(*p)) else {
        return true;
    };
    ctx.set_cursor_icon(egui::CursorIcon::Crosshair);
    let ndc = Vec2::new(
        (pos.x - rect.left()) / rect.width() * 2.0 - 1.0,
        1.0 - (pos.y - rect.top()) / rect.height() * 2.0,
    );
    let pressed = ctx.input(|i| i.pointer.button_pressed(PointerButton::Primary));
    let moved = ctx.input(|i| i.pointer.delta() != egui::Vec2::ZERO);
    let mut message = "Clique nas arestas · Enter: aplicar · Esc/RMB: cancelar".to_owned();
    if tool == "knife" {
        let hit = state.project.active_mesh().and_then(|mesh| {
            pick_mesh(
                mesh,
                &state.camera,
                Vec2::new(rect.width(), rect.height()) * ctx.pixels_per_point(),
                ndc,
                SelectMode::Edge,
                false,
            )
        });
        if let Some(start) = session.edge_start {
            painter.line_segment(
                [screen(state, rect, start.position), pos],
                egui::Stroke::new(2.0_f32, egui::Color32::YELLOW),
            );
        }
        if pressed {
            if let Some(hit) = hit {
                if let PickComponent::Edge(a, b) = hit.component {
                    let point = EdgePoint {
                        edge: (a, b),
                        position: hit.position,
                    };
                    if let Some(start) = session.edge_start {
                        if let Some(mesh) = state.project.active_mesh() {
                            match petunia_mesh::knife::cut_face(mesh, start, point) {
                                Ok(mesh) => {
                                    state.preview_mesh(mesh);
                                    session.edge_start = None;
                                }
                                Err(error) => state.set_status(error),
                            }
                        }
                    } else {
                        session.edge_start = Some(point);
                    }
                }
            }
        }
    } else if tool == "slice" {
        message = "Arraste o plano · Enter/LMB: aplicar · Esc/RMB: cancelar".into();
        if pressed {
            if session.sliding {
                finish(ctx, state, id, false);
                return true;
            }
            session.anchor = Some(pos);
        }
        if let Some(anchor) = session.anchor {
            painter.line_segment(
                [anchor, pos],
                egui::Stroke::new(2.0_f32, egui::Color32::YELLOW),
            );
            if moved && ctx.input(|i| i.pointer.button_down(PointerButton::Primary)) {
                let delta = pos - anchor;
                if delta.length() > 4.0 {
                    let normal = (state.camera.right() * delta.y + state.camera.up() * delta.x)
                        .normalize_or_zero();
                    let center = Vec3::from_array(session.source.selection_center());
                    let anchor_ndc = Vec2::new(
                        (anchor.x - rect.left()) / rect.width() * 2.0 - 1.0,
                        1.0 - (anchor.y - rect.top()) / rect.height() * 2.0,
                    );
                    let (origin, direction) = state.camera.ray(anchor_ndc.x, anchor_ndc.y);
                    let forward = state.camera.forward();
                    let denominator = direction.dot(forward);
                    if denominator.abs() > 1e-5 {
                        let point =
                            origin + direction * ((center - origin).dot(forward) / denominator);
                        let mut mesh = session.source.clone();
                        mesh.slice_plane(point, normal, true);
                        state.preview_mesh(mesh);
                    }
                }
            }
            if ctx.input(|i| i.pointer.button_released(PointerButton::Primary)) {
                session.sliding = true;
            }
        }
    } else {
        let scroll = ctx.input(|i| i.raw_scroll_delta.y);
        if scroll != 0.0 {
            session.cuts =
                (session.cuts as i32 + if scroll > 0.0 { 1 } else { -1 }).clamp(1, 32) as usize;
        }
        if !session.sliding {
            session.ring = None;
            if let Some(hit) = pick_mesh(
                &session.source,
                &state.camera,
                Vec2::new(rect.width(), rect.height()) * ctx.pixels_per_point(),
                ndc,
                SelectMode::Edge,
                false,
            ) {
                if let PickComponent::Edge(a, b) = hit.component {
                    match LoopRing::discover(&session.source, (a, b)) {
                        Ok(ring) => session.ring = Some(ring),
                        Err(error) => {
                            session.ring = None;
                            state.set_status(error.to_string());
                        }
                    }
                }
            }
        }
        let slide = if cancel {
            0.0
        } else {
            session
                .anchor
                .map(|anchor| ((pos.x - anchor.x) / 200.0).clamp(-1.0, 1.0))
                .unwrap_or(0.0)
        };
        if let Some(ring) = &session.ring {
            if let Ok(lines) = ring.preview(&session.source, session.cuts, slide) {
                for [a, b] in lines {
                    painter.line_segment(
                        [screen(state, rect, a), screen(state, rect, b)],
                        egui::Stroke::new(2.5_f32, egui::Color32::YELLOW),
                    );
                }
            }
            if pressed || (session.sliding && (moved || scroll != 0.0 || cancel)) {
                match ring.apply(&session.source, session.cuts, slide) {
                    Ok(mesh) => state.preview_mesh(mesh),
                    Err(error) => state.set_status(error.to_string()),
                }
                if session.sliding && (pressed || cancel) {
                    finish(ctx, state, id, false);
                    return true;
                }
                if pressed {
                    session.sliding = true;
                    session.anchor = Some(pos);
                }
            }
        }
        message = format!(
            "Loop cut: {} · Scroll: cortes · LMB: {} · Esc: cancelar",
            session.cuts,
            if session.sliding {
                "aplicar · RMB: centro"
            } else {
                "deslizar"
            }
        );
    }
    painter.text(
        rect.left_top() + egui::vec2(12.0, 32.0),
        egui::Align2::LEFT_TOP,
        message,
        egui::FontId::monospace(13.0),
        egui::Color32::YELLOW,
    );
    ctx.data_mut(|d| d.insert_temp(id, session));
    true
}
fn finish(ctx: &egui::Context, state: &mut AppState, id: egui::Id, cancel: bool) {
    state.finish_mesh_preview(cancel);
    state.active_tool = "select".into();
    ctx.data_mut(|d| d.remove::<CutSession>(id));
}
fn screen(state: &AppState, rect: Rect, p: Vec3) -> Pos2 {
    let p = state.camera.project_ndc(p);
    Pos2::new(
        rect.center().x + p.x * rect.width() * 0.5,
        rect.center().y - p.y * rect.height() * 0.5,
    )
}
