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

proptest! {
    /// Primitivas radiais: finitas, válidas e sem degeneradas em toda a faixa.
    #[test]
    fn radial_primitives_hold_invariants(
        sides in 0u32..70,
        radius in 0.0f32..50.0,
        height in 0.0f32..50.0,
        top in 0.0f32..50.0,
    ) {
        for mesh in [
            Mesh::cylinder(sides, radius, height),
            Mesh::cone(sides, radius, height),
            Mesh::radial_frustum(radius, top, height, sides, true, true),
            Mesh::radial_frustum(radius, top, height, sides, false, false),
        ] {
            let a = petunia_mesh::primitives::primitive_audit(&mesh);
            prop_assert!(a.all_finite);
            prop_assert!(a.indices_valid);
            prop_assert_eq!(a.degenerate_faces, 0);
            prop_assert!(a.bounds_min.iter().all(|x| x.is_finite()));
        }
    }

    /// Redondas/orgânicas: finitas, válidas e sem degeneradas em toda a faixa.
    #[test]
    fn round_primitives_hold_invariants(
        radius in 0.0f32..50.0,
        seg in 0u32..70,
        rings in 0u32..50,
        subdiv in 0u32..8,
    ) {
        for mesh in [
            Mesh::sphere_low(seg, rings, radius),
            Mesh::icosphere(radius, subdiv),
            Mesh::capsule_profile(seg, radius, radius * 2.0, 2),
            Mesh::torus(radius + 1.0, radius * 0.4 + 0.01, seg, rings),
            Mesh::circle(radius, seg, true),
            Mesh::circle(radius, seg, false),
        ] {
            let a = petunia_mesh::primitives::primitive_audit(&mesh);
            prop_assert!(a.all_finite);
            prop_assert!(a.indices_valid);
            prop_assert_eq!(a.degenerate_faces, 0);
            prop_assert_eq!(a.coincident_verts, 0);
        }
    }

    /// Caixas/cunhas/planos: invariantes em dimensões arbitrárias.
    #[test]
    fn box_family_holds_invariants(
        w in 0.0f32..50.0,
        h in 0.0f32..50.0,
        d in 0.0f32..50.0,
    ) {
        for mesh in [
            Mesh::cube(w),
            Mesh::box_dim(w, h, d),
            Mesh::wedge(w, h, d),
            Mesh::plane(w),
        ] {
            let a = petunia_mesh::primitives::primitive_audit(&mesh);
            prop_assert!(a.all_finite);
            prop_assert!(a.indices_valid);
            prop_assert_eq!(a.degenerate_faces, 0);
            prop_assert_eq!(a.outward_violations, 0);
        }
    }
}
