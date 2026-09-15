//! Representative mesh hot-path benchmarks (`criterion`, P1-01).
//!
//! Real fixtures on real code paths (cube topology validation, quad
//! triangulation, profile-area predicate), not toy loops.

use criterion::{Criterion, criterion_group, criterion_main};
use petunia_mesh::{HalfEdgeMesh, Mesh};

fn bench_validate_cube(c: &mut Criterion) {
    let mesh = Mesh::cube(2.0);
    c.bench_function("mesh/validate_cube", |b| {
        b.iter(|| {
            let (_, defects) = HalfEdgeMesh::from_mesh(&mesh);
            std::hint::black_box(defects.is_empty())
        })
    });
}

fn bench_triangulate_cube(c: &mut Criterion) {
    c.bench_function("mesh/triangulate_cube", |b| {
        b.iter(|| {
            let mut mesh = Mesh::cube(2.0);
            mesh.triangulate();
            std::hint::black_box(mesh.faces.len())
        })
    });
}

fn bench_profile_area(c: &mut Criterion) {
    let square = [[0.0, 0.0], [2.0, 0.0], [2.0, 2.0], [0.0, 2.0]];
    c.bench_function("mesh/profile_area_square", |b| {
        b.iter(|| std::hint::black_box(petunia_mesh::profile_geo::profile_area(&square)))
    });
}

criterion_group!(
    benches,
    bench_validate_cube,
    bench_triangulate_cube,
    bench_profile_area
);
criterion_main!(benches);
