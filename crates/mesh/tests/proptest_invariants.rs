//! Property tests for mesh invariants (`proptest`, P0-12).
//!
//! Topology, serialization and transform properties that must hold for every
//! generated input, not just hand-picked fixtures.

use petunia_mesh::{HalfEdgeMesh, Mesh};
use proptest::prelude::*;

proptest! {
    /// Cubes of any sane size stay closed manifolds with 8 verts / 6 quads.
    #[test]
    fn cube_topology_holds_for_any_size(size in 0.01f32..100.0) {
        let mesh = Mesh::cube(size);
        prop_assert_eq!(mesh.verts.len(), 8);
        prop_assert_eq!(mesh.faces.len(), 6);
        prop_assert!(mesh.faces.iter().all(|f| f.verts.len() == 4));
        let (topology, defects) = HalfEdgeMesh::from_mesh(&mesh);
        prop_assert!(defects.is_empty());
        prop_assert!(topology.is_closed_manifold());
        let report = mesh.validate_topology();
        prop_assert_eq!(report.isolated_vertex_count, 0);
    }

    /// Triangulation preserves vertex count and yields only tris.
    #[test]
    fn triangulate_keeps_verts_and_tris_only(size in 0.01f32..10.0) {
        let mut mesh = Mesh::cube(size);
        mesh.triangulate();
        prop_assert_eq!(mesh.verts.len(), 8);
        prop_assert!(!mesh.faces.is_empty());
        prop_assert!(mesh.faces.iter().all(|f| f.verts.len() == 3));
        prop_assert!(mesh.faces.iter().all(|f| f.uv.len() == f.verts.len()));
    }

    /// Translate-then-untranslate roundtrips vertex positions.
    #[test]
    fn translate_roundtrip(size in 0.01f32..10.0, dx in -50.0f32..50.0) {
        let mut mesh = Mesh::cube(size);
        mesh.select_all();
        let before: Vec<[f32; 3]> = mesh.verts.iter().map(|v| v.pos).collect();
        mesh.translate_selected([dx, 0.0, 0.0]);
        mesh.translate_selected([-dx, 0.0, 0.0]);
        for (a, b) in mesh.verts.iter().map(|v| v.pos).zip(before) {
            for k in 0..3 {
                prop_assert!((a[k] - b[k]).abs() < 1e-4);
            }
        }
    }
}
