//! Testes unitários para Triangulation Inspection e Flip Diagonal (Core V1).

use petunia_mesh::{Face, Mesh, Vertex};

#[test]
fn test_triangulation_wireframe_cube_has_six_diagonals() {
    let cube = Mesh::cube(2.0);
    let wire = cube.triangulation_wireframe();
    // Um cubo possui 6 faces quadrangulares; cada uma possui exatamente 1 diagonal interna de fan.
    assert_eq!(wire.len(), 6);
    for (p0, p1) in wire {
        assert_ne!(p0, p1, "Diagonais não devem ser degeneradas");
    }
}

#[test]
fn test_flip_diagonal_quad() {
    let mut mesh = Mesh::default();
    mesh.verts.push(Vertex::new(0.0, 0.0, 0.0)); // 0
    mesh.verts.push(Vertex::new(1.0, 0.0, 0.0)); // 1
    mesh.verts.push(Vertex::new(1.0, 1.0, 0.0)); // 2
    mesh.verts.push(Vertex::new(0.0, 1.0, 0.0)); // 3

    let mut face = Face::new(vec![0, 1, 2, 3]);
    face.selected = true;
    mesh.push_face(face);

    // Antes do flip: fan gera diagonais a partir do vertice 0 -> diagonal (0, 2)
    let wire_before = mesh.triangulation_wireframe();
    assert_eq!(wire_before.len(), 1);
    assert_eq!(wire_before[0].0, mesh.verts[0].pos);
    assert_eq!(wire_before[0].1, mesh.verts[2].pos);

    // Executa flip
    assert!(
        mesh.flip_diagonal(),
        "Flip em quad selecionado deve ter sucesso"
    );

    // Depois do flip: vertices foram rotacionados para [1, 2, 3, 0] -> fan gera diagonal (1, 3)
    let wire_after = mesh.triangulation_wireframe();
    assert_eq!(wire_after.len(), 1);
    assert_eq!(wire_after[0].0, mesh.verts[1].pos);
    assert_eq!(wire_after[0].1, mesh.verts[3].pos);
}

#[test]
fn test_flip_diagonal_shared_triangles_edge() {
    let mut mesh = Mesh::default();
    mesh.verts.push(Vertex::new(0.0, 1.0, 0.0)); // 0 (w0)
    mesh.verts.push(Vertex::new(1.0, 0.0, 0.0)); // 1 (u)
    mesh.verts.push(Vertex::new(-1.0, 0.0, 0.0)); // 2 (v)
    mesh.verts.push(Vertex::new(0.0, -1.0, 0.0)); // 3 (w1)

    // Triângulo 0: (1, 2, 0) -> aresta compartilhada é (1, 2)
    // Triângulo 1: (2, 1, 3) -> aresta compartilhada é (2, 1)
    mesh.push_face(Face::new(vec![1, 2, 0]));
    mesh.push_face(Face::new(vec![2, 1, 3]));

    // Seleciona aresta (1, 2)
    mesh.selected_edges.insert(petunia_mesh::edge_key(1, 2));

    assert!(
        mesh.flip_diagonal(),
        "Flip em aresta de dois triângulos deve ter sucesso"
    );

    // A aresta compartilhada agora deve ser entre (0, 3) e as faces devem conectar (0, 1, 3) e (3, 2, 0)
    assert_eq!(mesh.faces.len(), 2);
    assert!(mesh.faces[0].verts.contains(&0) && mesh.faces[0].verts.contains(&3));
    assert!(mesh.faces[1].verts.contains(&0) && mesh.faces[1].verts.contains(&3));
    assert!(!mesh.faces[0].verts.contains(&2) || !mesh.faces[0].verts.contains(&1));
}
