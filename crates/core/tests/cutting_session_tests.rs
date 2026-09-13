//! Testes unitários para as máquinas de estado de sessões puras do Core:
//! CutSession (Knife, Slice, Loop Cut) e PointerSession (Transformações Modais G/R/S).
//!
//! Garante funcionamento 100% autônomo, headless e agnóstico de UI.

use petunia_core::camera::Camera;
use petunia_core::cutting_session::CutSession;
use petunia_core::modal::PointerSession;
use petunia_core::state::AppState;
use petunia_core::viewport::LogicalRect;
use petunia_mesh::loop_cut::LoopRing;
use petunia_mesh::Mesh;

#[test]
fn test_cut_session_lifecycle_and_cuts_adjustment() {
    let mesh = Mesh::cube(1.0);
    let mut session = CutSession::new(mesh.clone());

    assert_eq!(session.cuts, 1);
    assert!(!session.sliding);
    assert!(session.anchor.is_none());
    assert!(session.ring.is_none());
    assert!(session.edge_start.is_none());
    assert_eq!(session.source.verts.len(), mesh.verts.len());

    // Incrementa cuts
    session.adjust_cuts(2);
    assert_eq!(session.cuts, 3);

    // Incrementa acima do limite máximo (32)
    session.adjust_cuts(50);
    assert_eq!(session.cuts, 32);

    // Decrementa abaixo do limite mínimo (1)
    session.adjust_cuts(-50);
    assert_eq!(session.cuts, 1);
}

#[test]
fn test_cut_session_loop_slide_calculation() {
    let mesh = Mesh::cube(1.0);
    let mut session = CutSession::new(mesh);

    // Sem âncora, slide é neutro (0.0)
    assert_eq!(session.compute_loop_slide(150.0), 0.0);

    // Define âncora em X = 200.0
    session.anchor = Some([200.0, 300.0]);

    // X = 200 -> delta 0 -> slide 0.0
    assert_eq!(session.compute_loop_slide(200.0), 0.0);

    // X = 300 -> delta +100 / 200 = 0.5
    assert_eq!(session.compute_loop_slide(300.0), 0.5);

    // X = 100 -> delta -100 / 200 = -0.5
    assert_eq!(session.compute_loop_slide(100.0), -0.5);

    // Clamping superior (+1.0)
    assert_eq!(session.compute_loop_slide(500.0), 1.0);

    // Clamping inferior (-1.0)
    assert_eq!(session.compute_loop_slide(-100.0), -1.0);
}

#[test]
fn test_cut_session_compute_slice() {
    let mesh = Mesh::cube(1.0);
    let session = CutSession::new(mesh);
    let camera = Camera::default();
    let viewport = LogicalRect::from_min_max([0.0, 0.0], [800.0, 600.0]);

    // Arrasto ínfimo (< 4px de distância) deve retornar None
    let slice_tiny = session.compute_slice(&camera, [100.0, 100.0], [101.0, 101.0], viewport);
    assert!(slice_tiny.is_none());

    // Arrasto significativo diagonal cortando o cubo pelo centro
    let slice_result = session.compute_slice(&camera, [350.0, 250.0], [450.0, 350.0], viewport);
    assert!(slice_result.is_some());
    let sliced_mesh = slice_result.unwrap();
    // A malha fatiada deve ter mais vértices que o cubo original
    assert!(sliced_mesh.verts.len() > session.source.verts.len());
}

#[test]
fn test_cut_session_loop_cut_flow() {
    let mesh = Mesh::cube(1.0);
    let mut session = CutSession::new(mesh.clone());

    // Sem ring definido, preview retorna lista vazia e apply retorna erro
    assert!(session.preview_loop_lines(0.0).unwrap().is_empty());
    assert!(session.apply_loop_cut(0.0).is_err());

    // Descobre ring na aresta (0, 1) do cubo
    let ring = LoopRing::discover(&mesh, (0, 1)).expect("deve descobrir ring no cubo");
    session.ring = Some(ring);
    session.cuts = 2;

    // Gera linhas de preview do loop cut
    let lines = session
        .preview_loop_lines(0.2)
        .expect("deve gerar linhas de preview");
    assert!(!lines.is_empty());

    // Aplica o loop cut
    let cut_mesh = session.apply_loop_cut(0.2).expect("deve aplicar loop cut");
    assert!(cut_mesh.verts.len() > mesh.verts.len());
    assert!(cut_mesh.faces.len() > mesh.faces.len());
}

#[test]
fn test_pointer_session_numeric_input() {
    let mut session = PointerSession::new([100.0, 200.0], false);
    assert_eq!(session.anchor, [100.0, 200.0]);
    assert!(session.numeric.is_empty());
    assert_eq!(session.parse_numeric(), None);

    session.push_char('1');
    session.push_char('.');
    session.push_char('7');
    session.push_char('5');
    assert_eq!(session.parse_numeric(), Some(1.75));

    session.pop_char();
    assert_eq!(session.parse_numeric(), Some(1.7));

    session.pop_char();
    session.pop_char();
    session.pop_char();
    assert!(session.numeric.is_empty());
    assert_eq!(session.parse_numeric(), None);

    // Entrada com sinal negativo
    session.push_char('-');
    session.push_char('3');
    assert_eq!(session.parse_numeric(), Some(-3.0));
}

#[test]
fn test_mesh_preview_clears_cut_session_on_finish() {
    let mut state = AppState::default();
    let mesh = Mesh::cube(1.0);
    state.cut_session = Some(CutSession::new(mesh));
    assert!(state.cut_session.is_some());

    assert!(state.begin_mesh_preview("Test Cut"));
    state.finish_mesh_preview(true);

    assert!(
        state.cut_session.is_none(),
        "cut_session deve ser limpa ao finalizar mesh preview"
    );
}
