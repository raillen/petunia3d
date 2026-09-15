//! Headless egui interaction regressions for multi-stage viewport cutting.
use egui::{Event, Key, Modifiers, PointerButton, Pos2, Rect};
use glam::Vec3;
use petunia_core::{AppState, EditMode, ViewPreset};
use petunia_mesh::{Mesh, loop_cut::LoopRing};

fn rect() -> Rect {
    Rect::from_min_size(Pos2::ZERO, egui::vec2(800.0, 600.0))
}
fn button(pos: Pos2, kind: PointerButton, pressed: bool) -> Event {
    Event::PointerButton {
        pos,
        button: kind,
        pressed,
        modifiers: Modifiers::NONE,
    }
}
fn key(key: Key) -> Event {
    Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }
}
fn frame(ctx: &egui::Context, state: &mut AppState, events: Vec<Event>) {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(rect()),
            events,
            ..Default::default()
        },
        |_ui| {
            let painter = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Middle,
                egui::Id::new("cut.test.viewport"),
            ));
            crate::cutting::draw(ctx, state, rect(), &painter);
        },
    )
    .textures_delta
    .clear();
}
fn state(tool: &str) -> AppState {
    let mut state = AppState::new("en");
    *state.project.active_mesh_mut().unwrap() = Mesh::cube(2.0);
    state.mode = EditMode::Edit;
    state.active_tool = tool.into();
    state.camera.set_preset(ViewPreset::Front);
    state.camera.aspect = rect().aspect_ratio();
    state.sync_selection();
    state
}
fn screen(state: &AppState, point: Vec3) -> Pos2 {
    let ndc = state.camera.project_ndc(point);
    egui::pos2(
        rect().center().x + ndc.x * rect().width() * 0.5,
        rect().center().y - ndc.y * rect().height() * 0.5,
    )
}
fn front_edges(state: &AppState) -> [(u32, u32); 2] {
    let mesh = state.project.active_mesh().unwrap();
    let face = mesh
        .faces
        .iter()
        .enumerate()
        .find(|(i, _)| mesh.face_normal(*i).z > 0.99)
        .unwrap()
        .1;
    [
        (face.verts[0], face.verts[1]),
        (face.verts[2], face.verts[3]),
    ]
}
fn edge_midpoint(state: &AppState, edge: (u32, u32)) -> Pos2 {
    let mesh = state.project.active_mesh().unwrap();
    screen(
        state,
        (mesh.verts[edge.0 as usize].vec() + mesh.verts[edge.1 as usize].vec()) * 0.5,
    )
}
fn assert_mesh(actual: &Mesh, expected: &Mesh) {
    assert_eq!(actual.verts.len(), expected.verts.len());
    assert_eq!(actual.faces.len(), expected.faces.len());
    assert_eq!(actual.selected_edges, expected.selected_edges);
    for (a, b) in actual.verts.iter().zip(&expected.verts) {
        assert_eq!(a.pos, b.pos);
        assert_eq!(a.color, b.color);
        assert_eq!(a.selected, b.selected);
    }
    for (a, b) in actual.faces.iter().zip(&expected.faces) {
        assert_eq!(a.verts, b.verts);
        assert_eq!(a.uv, b.uv);
        assert_eq!(a.selected, b.selected);
    }
}
fn click(ctx: &egui::Context, state: &mut AppState, pos: Pos2) {
    frame(
        ctx,
        state,
        vec![
            Event::PointerMoved(pos),
            button(pos, PointerButton::Primary, true),
        ],
    );
    frame(ctx, state, vec![button(pos, PointerButton::Primary, false)]);
}
fn begin_loop_slide(ctx: &egui::Context, state: &mut AppState) -> Pos2 {
    let anchor = edge_midpoint(state, front_edges(state)[0]);
    let original_vertices = state.project.active_mesh().unwrap().verts.len();
    frame(ctx, state, vec![Event::PointerMoved(anchor)]);
    assert!(state.mesh_preview.is_some());
    assert_eq!(
        state.project.active_mesh().unwrap().verts.len(),
        original_vertices,
        "hover only previews the overlay"
    );
    click(ctx, state, anchor);
    assert!(
        state.project.active_mesh().unwrap().verts.len() > original_vertices,
        "first click creates a real loop preview"
    );
    assert!(state.mesh_preview.is_some());
    assert_eq!(state.project.undo.depth(), (0, 0));
    anchor
}

#[test]
fn loop_cut_first_click_slide_then_escape_restores_exact_mesh() {
    let ctx = egui::Context::default();
    let mut state = state("loop_cut");
    let original = state.project.active_mesh().unwrap().clone();
    let anchor = begin_loop_slide(&ctx, &mut state);
    let centered = state.project.active_mesh().unwrap().to_obj();
    frame(
        &ctx,
        &mut state,
        vec![Event::PointerMoved(anchor + egui::vec2(60.0, 0.0))],
    );
    assert_ne!(state.project.active_mesh().unwrap().to_obj(), centered);
    frame(&ctx, &mut state, vec![key(Key::Escape)]);
    assert_mesh(state.project.active_mesh().unwrap(), &original);
    assert!(state.mesh_preview.is_none());
    assert_eq!(state.active_tool, "select");
    assert_eq!(state.project.undo.depth(), (0, 0));
}

#[test]
fn loop_cut_second_click_commits_one_checkpoint() {
    let ctx = egui::Context::default();
    let mut state = state("loop_cut");
    let original = state.project.active_mesh().unwrap().clone();
    let anchor = begin_loop_slide(&ctx, &mut state);
    let end = anchor + egui::vec2(40.0, 0.0);
    frame(&ctx, &mut state, vec![Event::PointerMoved(end)]);
    let preview = state.project.active_mesh().unwrap().clone();
    click(&ctx, &mut state, end);
    assert!(state.mesh_preview.is_none());
    assert_eq!(state.project.undo.depth(), (1, 0));
    assert_mesh(state.project.active_mesh().unwrap(), &preview);
    state.undo();
    assert_mesh(state.project.active_mesh().unwrap(), &original);
    state.redo();
    assert_mesh(state.project.active_mesh().unwrap(), &preview);
}

#[test]
fn loop_cut_right_click_outside_viewport_centers_and_commits() {
    let ctx = egui::Context::default();
    let mut state = state("loop_cut");
    let source = state.project.active_mesh().unwrap().clone();
    let ring = LoopRing::discover(&source, front_edges(&state)[0]).unwrap();
    let expected = ring.apply(&source, 1, 0.0).unwrap();
    let anchor = begin_loop_slide(&ctx, &mut state);
    frame(
        &ctx,
        &mut state,
        vec![Event::PointerMoved(anchor + egui::vec2(60.0, 0.0))],
    );
    let outside = egui::pos2(-20.0, 40.0);
    frame(
        &ctx,
        &mut state,
        vec![
            Event::PointerMoved(outside),
            button(outside, PointerButton::Secondary, true),
        ],
    );
    assert!(
        state.mesh_preview.is_none(),
        "RMB must finish even outside the viewport"
    );
    assert_eq!(state.project.undo.depth(), (1, 0));
    assert_mesh(state.project.active_mesh().unwrap(), &expected);
}

#[test]
fn knife_two_edge_clicks_then_enter_commit_connected_cut() {
    let ctx = egui::Context::default();
    let mut state = state("knife");
    let source = state.project.active_mesh().unwrap().clone();
    let [a, b] = front_edges(&state).map(|edge| edge_midpoint(&state, edge));
    frame(&ctx, &mut state, vec![Event::PointerMoved(a)]);
    click(&ctx, &mut state, a);
    assert_mesh(state.project.active_mesh().unwrap(), &source);
    click(&ctx, &mut state, b);
    assert_eq!(
        state.project.active_mesh().unwrap().faces.len(),
        source.faces.len() + 1
    );
    assert_eq!(state.project.undo.depth(), (0, 0));
    assert!(
        state
            .project
            .active_mesh()
            .unwrap()
            .validate_topology()
            .is_manifold
    );
    frame(&ctx, &mut state, vec![key(Key::Enter)]);
    assert!(state.mesh_preview.is_none());
    assert_eq!(state.project.undo.depth(), (1, 0));
    state.undo();
    assert_mesh(state.project.active_mesh().unwrap(), &source);
}

#[test]
fn slice_drag_then_escape_restores_source_without_checkpoint() {
    let ctx = egui::Context::default();
    let mut state = state("slice");
    let source = state.project.active_mesh().unwrap().clone();
    let anchor = rect().center() - egui::vec2(0.0, 50.0);
    let end = rect().center() + egui::vec2(0.0, 50.0);
    frame(&ctx, &mut state, vec![Event::PointerMoved(anchor)]);
    frame(
        &ctx,
        &mut state,
        vec![button(anchor, PointerButton::Primary, true)],
    );
    frame(&ctx, &mut state, vec![Event::PointerMoved(end)]);
    let preview = state.project.active_mesh().unwrap();
    assert!(source.verts.iter().any(|v| v.pos[0] < -0.5));
    assert!(preview.verts.iter().all(|v| v.pos[0] >= -1e-5));
    assert!(preview.verts.iter().any(|v| v.pos[0].abs() < 1e-5));
    assert!(preview.validate_topology().is_closed);
    frame(
        &ctx,
        &mut state,
        vec![button(end, PointerButton::Primary, false)],
    );
    assert!(state.mesh_preview.is_some());
    assert_eq!(state.project.undo.depth(), (0, 0));
    frame(&ctx, &mut state, vec![key(Key::Escape)]);
    assert_mesh(state.project.active_mesh().unwrap(), &source);
    assert!(state.mesh_preview.is_none());
    assert_eq!(state.project.undo.depth(), (0, 0));
}
