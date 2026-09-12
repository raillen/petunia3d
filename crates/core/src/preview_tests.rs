//! Public transaction behavior shared by painting and multi-stage cutting tools.
use crate::{AppState, ModalKind};
use glam::Vec3;

fn snapshot(state: &AppState) -> String {
    format!("{:?}", state.project.active_mesh().unwrap())
}

#[test]
fn repeated_paint_samples_commit_a_single_undo_and_redo_restores_colors() {
    let mut state = AppState::new("en");
    state.paint_radius = 100.0;
    state.paint_strength = 0.25;
    let original = snapshot(&state);
    state.begin_paint_stroke();
    for point in [Vec3::ZERO, Vec3::X, Vec3::Y] {
        state.paint_at(point);
        assert_eq!(state.undo.depth(), (0, 0));
    }
    let result = snapshot(&state);
    assert_ne!(result, original);
    state.finish_paint_stroke(false);
    assert_eq!(state.undo.depth(), (1, 0));
    state.undo();
    assert_eq!(snapshot(&state), original);
    state.redo();
    assert_eq!(snapshot(&state), result);
}

#[test]
fn paint_cancel_restores_colors_without_discarding_redo() {
    let mut state = AppState::new("en");
    state.checkpoint("prior");
    state
        .project
        .active_mesh_mut()
        .unwrap()
        .translate_selected([1.0, 0.0, 0.0]);
    state.undo();
    let original = snapshot(&state);
    state.paint_radius = 100.0;
    state.begin_paint_stroke();
    state.paint_at(Vec3::ZERO);
    state.finish_paint_stroke(true);
    assert_eq!(snapshot(&state), original);
    assert_eq!(state.undo.depth(), (0, 1));
}

#[test]
fn cut_preview_cancel_restores_topology_and_commit_is_atomic() {
    let mut state = AppState::new("en");
    let original = state.project.active_mesh().unwrap().clone();
    let original_snapshot = snapshot(&state);
    assert!(state.begin_mesh_preview("Slice"));
    for offset in [0.1, 0.25, 0.4] {
        let mut preview = original.clone();
        preview.slice_plane(Vec3::Y * offset, Vec3::Y, true);
        state.preview_mesh(preview);
        assert_eq!(state.undo.depth(), (0, 0));
    }
    state.finish_mesh_preview(true);
    assert_eq!(snapshot(&state), original_snapshot);
    assert!(state.begin_mesh_preview("Slice"));
    let mut preview = original;
    preview.slice_plane(Vec3::ZERO, Vec3::X, true);
    state.preview_mesh(preview);
    let result = snapshot(&state);
    state.finish_mesh_preview(false);
    assert_eq!(state.undo.depth(), (1, 0));
    state.undo();
    assert_eq!(snapshot(&state), original_snapshot);
    state.redo();
    assert_eq!(snapshot(&state), result);
}

#[test]
fn identity_cut_preview_does_not_pollute_history() {
    let mut state = AppState::new("en");
    let original = state.project.active_mesh().unwrap().clone();
    assert!(state.begin_mesh_preview("Slice"));
    state.preview_mesh(original);
    state.finish_mesh_preview(false);
    assert_eq!(state.undo.depth(), (0, 0));
}

#[test]
fn generic_preview_cannot_start_during_modal_or_paint_stroke() {
    let mut state = AppState::new("en");
    state.begin_modal(ModalKind::Move).unwrap();
    assert!(!state.begin_mesh_preview("Slice"));
    state.cancel_modal();
    state.begin_paint_stroke();
    assert!(!state.begin_mesh_preview("Slice"));
}

#[test]
fn modal_cannot_start_during_generic_preview_or_paint_stroke() {
    let mut state = AppState::new("en");
    assert!(state.begin_mesh_preview("Slice"));
    assert_eq!(
        state.begin_modal(ModalKind::Move),
        Err(crate::ModalError::AlreadyActive)
    );
    state.finish_mesh_preview(true);
    state.begin_paint_stroke();
    assert_eq!(
        state.begin_modal(ModalKind::Move),
        Err(crate::ModalError::AlreadyActive)
    );
}

#[test]
fn paint_stroke_cannot_start_inside_modal_or_cut_preview() {
    let mut state = AppState::new("en");
    state.begin_modal(ModalKind::Move).unwrap();
    state.begin_paint_stroke();
    assert!(state.paint_stroke.is_none());
    assert!(state.modal.is_some());
    state.cancel_modal();
    assert!(state.begin_mesh_preview("Slice"));
    state.begin_paint_stroke();
    assert!(state.paint_stroke.is_none());
    assert!(state.mesh_preview.is_some());
}
