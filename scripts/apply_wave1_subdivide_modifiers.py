#!/usr/bin/env python3
"""Wave 1: exact cut-count subdivision semantics plus modifier-domain tests.

This staging script is intentionally removed by the validating workflow after
all checks pass. It only touches the modeling/domain changes in this wave.
"""

from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]


def read(path: str) -> str:
    return (ROOT / path).read_text()


def write(path: str, text: str) -> None:
    (ROOT / path).write_text(text)


# ---------------------------------------------------------------------------
# 1. Replace recursive subdivision with exact N-cut subdivision.
# ---------------------------------------------------------------------------
ops_path = "crates/mesh/src/ops.rs"
ops = read(ops_path)
pattern = re.compile(
    r"    pub fn subdivide_selected\(&mut self\) \{.*?\n    fn mid_vert4\(&mut self, a: u32, b: u32, c: u32, d: u32\) -> u32 \{.*?\n    \}\n\n    /// Chanfra",
    re.S,
)
replacement = r'''    /// Subdivide selected triangle/quad faces with one uniform cut per edge.
    ///
    /// This is the compatibility entry point used by older commands. The actual
    /// implementation is `subdivide_selected_cuts`, which treats `cuts` as the
    /// number of inserted edge cuts on the original face (not recursive levels).
    pub fn subdivide_selected(&mut self) {
        self.subdivide_selected_cuts(1);
    }

    /// Subdivide selected triangle/quad faces using exactly `cuts` uniformly
    /// spaced cuts on every original edge.
    ///
    /// A quad produces `(cuts + 1)^2` quads. A triangle produces
    /// `(cuts + 1)^2` triangles. Boundary split vertices are shared between
    /// adjacent selected faces so the selected region does not crack.
    pub fn subdivide_selected_cuts(&mut self, cuts: u32) {
        let segments = cuts.clamp(1, 64) + 1;
        let selected_faces: Vec<usize> = self
            .faces
            .iter()
            .enumerate()
            .filter(|(_, face)| face.selected)
            .map(|(index, _)| index)
            .collect();
        if selected_faces.is_empty() {
            return;
        }

        let mut edge_vertices: HashMap<(u32, u32, u32), u32> = HashMap::new();
        let mut replacement_faces = Vec::new();
        let mut remove_faces = Vec::new();

        for face_index in selected_faces {
            let source = self.faces[face_index].clone();
            if source.uv.len() != source.verts.len() {
                continue;
            }

            match source.verts.as_slice() {
                [a, b, c, d] => {
                    let [a, b, c, d] = [*a, *b, *c, *d];
                    let side = (segments + 1) as usize;
                    let mut grid = vec![vec![0_u32; side]; side];

                    let pa = self.verts[a as usize].vec();
                    let pb = self.verts[b as usize].vec();
                    let pc = self.verts[c as usize].vec();
                    let pd = self.verts[d as usize].vec();
                    let ca = self.verts[a as usize].color;
                    let cb = self.verts[b as usize].color;
                    let cc = self.verts[c as usize].color;
                    let cd = self.verts[d as usize].color;

                    for row in 0..=segments {
                        for column in 0..=segments {
                            let vertex_index = if row == 0 {
                                self.subdivide_edge_vertex(
                                    &mut edge_vertices,
                                    a,
                                    b,
                                    column,
                                    segments,
                                )
                            } else if column == segments {
                                self.subdivide_edge_vertex(
                                    &mut edge_vertices,
                                    b,
                                    c,
                                    row,
                                    segments,
                                )
                            } else if row == segments {
                                self.subdivide_edge_vertex(
                                    &mut edge_vertices,
                                    d,
                                    c,
                                    column,
                                    segments,
                                )
                            } else if column == 0 {
                                self.subdivide_edge_vertex(
                                    &mut edge_vertices,
                                    a,
                                    d,
                                    row,
                                    segments,
                                )
                            } else {
                                let u = column as f32 / segments as f32;
                                let v = row as f32 / segments as f32;
                                let top = pa.lerp(pb, u);
                                let bottom = pd.lerp(pc, u);
                                let position = top.lerp(bottom, v);
                                let top_color = [
                                    ca[0] + (cb[0] - ca[0]) * u,
                                    ca[1] + (cb[1] - ca[1]) * u,
                                    ca[2] + (cb[2] - ca[2]) * u,
                                ];
                                let bottom_color = [
                                    cd[0] + (cc[0] - cd[0]) * u,
                                    cd[1] + (cc[1] - cd[1]) * u,
                                    cd[2] + (cc[2] - cd[2]) * u,
                                ];
                                let color = [
                                    top_color[0] + (bottom_color[0] - top_color[0]) * v,
                                    top_color[1] + (bottom_color[1] - top_color[1]) * v,
                                    top_color[2] + (bottom_color[2] - top_color[2]) * v,
                                ];
                                let index = self.verts.len() as u32;
                                self.verts.push(Vertex {
                                    pos: position.to_array(),
                                    color,
                                    selected: true,
                                });
                                index
                            };
                            grid[row as usize][column as usize] = vertex_index;
                        }
                    }

                    let uv_at = |column: u32, row: u32| {
                        let u = column as f32 / segments as f32;
                        let v = row as f32 / segments as f32;
                        let top = [
                            source.uv[0][0] + (source.uv[1][0] - source.uv[0][0]) * u,
                            source.uv[0][1] + (source.uv[1][1] - source.uv[0][1]) * u,
                        ];
                        let bottom = [
                            source.uv[3][0] + (source.uv[2][0] - source.uv[3][0]) * u,
                            source.uv[3][1] + (source.uv[2][1] - source.uv[3][1]) * u,
                        ];
                        [
                            top[0] + (bottom[0] - top[0]) * v,
                            top[1] + (bottom[1] - top[1]) * v,
                        ]
                    };

                    for row in 0..segments {
                        for column in 0..segments {
                            let mut face = Face::with_uv(
                                vec![
                                    grid[row as usize][column as usize],
                                    grid[row as usize][(column + 1) as usize],
                                    grid[(row + 1) as usize][(column + 1) as usize],
                                    grid[(row + 1) as usize][column as usize],
                                ],
                                vec![
                                    uv_at(column, row),
                                    uv_at(column + 1, row),
                                    uv_at(column + 1, row + 1),
                                    uv_at(column, row + 1),
                                ],
                            );
                            face.selected = true;
                            face.material_slot = source.material_slot;
                            replacement_faces.push(face);
                        }
                    }
                    remove_faces.push(face_index);
                }
                [a, b, c] => {
                    let [a, b, c] = [*a, *b, *c];
                    let mut grid: HashMap<(u32, u32), u32> = HashMap::new();
                    let pa = self.verts[a as usize].vec();
                    let pb = self.verts[b as usize].vec();
                    let pc = self.verts[c as usize].vec();
                    let ca = self.verts[a as usize].color;
                    let cb = self.verts[b as usize].color;
                    let cc = self.verts[c as usize].color;

                    for i in 0..=segments {
                        for j in 0..=segments - i {
                            let vertex_index = if j == 0 {
                                self.subdivide_edge_vertex(
                                    &mut edge_vertices,
                                    a,
                                    b,
                                    i,
                                    segments,
                                )
                            } else if i == 0 {
                                self.subdivide_edge_vertex(
                                    &mut edge_vertices,
                                    a,
                                    c,
                                    j,
                                    segments,
                                )
                            } else if i + j == segments {
                                self.subdivide_edge_vertex(
                                    &mut edge_vertices,
                                    b,
                                    c,
                                    j,
                                    segments,
                                )
                            } else {
                                let wb = i as f32 / segments as f32;
                                let wc = j as f32 / segments as f32;
                                let wa = 1.0 - wb - wc;
                                let position = pa * wa + pb * wb + pc * wc;
                                let color = [
                                    ca[0] * wa + cb[0] * wb + cc[0] * wc,
                                    ca[1] * wa + cb[1] * wb + cc[1] * wc,
                                    ca[2] * wa + cb[2] * wb + cc[2] * wc,
                                ];
                                let index = self.verts.len() as u32;
                                self.verts.push(Vertex {
                                    pos: position.to_array(),
                                    color,
                                    selected: true,
                                });
                                index
                            };
                            grid.insert((i, j), vertex_index);
                        }
                    }

                    let uv_at = |i: u32, j: u32| {
                        let wb = i as f32 / segments as f32;
                        let wc = j as f32 / segments as f32;
                        let wa = 1.0 - wb - wc;
                        [
                            source.uv[0][0] * wa + source.uv[1][0] * wb + source.uv[2][0] * wc,
                            source.uv[0][1] * wa + source.uv[1][1] * wb + source.uv[2][1] * wc,
                        ]
                    };

                    for i in 0..segments {
                        for j in 0..segments - i {
                            let mut first = Face::with_uv(
                                vec![grid[&(i, j)], grid[&(i + 1, j)], grid[&(i, j + 1)]],
                                vec![uv_at(i, j), uv_at(i + 1, j), uv_at(i, j + 1)],
                            );
                            first.selected = true;
                            first.material_slot = source.material_slot;
                            replacement_faces.push(first);

                            if i + j < segments - 1 {
                                let mut second = Face::with_uv(
                                    vec![
                                        grid[&(i + 1, j)],
                                        grid[&(i + 1, j + 1)],
                                        grid[&(i, j + 1)],
                                    ],
                                    vec![
                                        uv_at(i + 1, j),
                                        uv_at(i + 1, j + 1),
                                        uv_at(i, j + 1),
                                    ],
                                );
                                second.selected = true;
                                second.material_slot = source.material_slot;
                                replacement_faces.push(second);
                            }
                        }
                    }
                    remove_faces.push(face_index);
                }
                _ => {}
            }
        }

        remove_faces.sort_unstable_by(|left, right| right.cmp(left));
        for face_index in remove_faces {
            self.faces.remove(face_index);
        }
        for face in replacement_faces {
            self.push_face(face);
        }
        self.selected_edges.clear();
        self.sync_vert_selection_from_faces();
    }

    fn subdivide_edge_vertex(
        &mut self,
        cache: &mut HashMap<(u32, u32, u32), u32>,
        a: u32,
        b: u32,
        step: u32,
        segments: u32,
    ) -> u32 {
        if step == 0 {
            return a;
        }
        if step >= segments {
            return b;
        }

        let (low, high) = edge_key(a, b);
        let canonical_step = if a == low { step } else { segments - step };
        let key = (low, high, canonical_step);
        if let Some(&index) = cache.get(&key) {
            return index;
        }

        let t = step as f32 / segments as f32;
        let start = &self.verts[a as usize];
        let end = &self.verts[b as usize];
        let position = start.vec().lerp(end.vec(), t);
        let color = [
            start.color[0] + (end.color[0] - start.color[0]) * t,
            start.color[1] + (end.color[1] - start.color[1]) * t,
            start.color[2] + (end.color[2] - start.color[2]) * t,
        ];
        let index = self.verts.len() as u32;
        self.verts.push(Vertex {
            pos: position.to_array(),
            color,
            selected: true,
        });
        cache.insert(key, index);
        index
    }

    /// Chanfra'''
ops, count = pattern.subn(replacement, ops, count=1)
if count != 1:
    raise SystemExit(f"expected one subdivision block, replaced {count}")
write(ops_path, ops)


# ---------------------------------------------------------------------------
# 2. Make the modeling tool call exact cut semantics once.
# ---------------------------------------------------------------------------
subdivide_path = "crates/module-model/src/subdivide.rs"
subdivide = read(subdivide_path)
old = '''        let cuts = state.subdivide_cuts.clamp(1, 6);
        if let Some(m) = state.project.active_mesh_mut() {
            for _ in 0..cuts {
                m.subdivide_selected();
            }
        }
'''
new = '''        let cuts = state.subdivide_cuts.clamp(1, 6);
        if let Some(m) = state.project.active_mesh_mut() {
            m.subdivide_selected_cuts(cuts);
        }
'''
if old not in subdivide:
    raise SystemExit("recursive SubdivideTool block not found")
subdivide = subdivide.replace(old, new, 1)
write(subdivide_path, subdivide)


# ---------------------------------------------------------------------------
# 3. Focused mesh tests: counts, UVs and shared selected-region boundaries.
# ---------------------------------------------------------------------------
mesh_test = ROOT / "crates/mesh/tests/subdivide_cuts.rs"
mesh_test.parent.mkdir(parents=True, exist_ok=True)
mesh_test.write_text(r'''use petunia_mesh::{Face, Mesh, Vertex};
use std::collections::HashSet;

#[test]
fn quad_cut_count_is_uniform_not_recursive() {
    let mut mesh = Mesh::plane(2.0);
    mesh.select_all();
    mesh.subdivide_selected_cuts(2);

    // Two cuts create three segments per original edge: 3 x 3 = 9 quads.
    assert_eq!(mesh.faces.len(), 9);
    assert_eq!(mesh.verts.len(), 16);
    assert!(mesh.faces.iter().all(|face| face.verts.len() == 4));
    assert!(mesh.faces.iter().all(|face| face.selected));
    assert!(mesh.faces.iter().all(|face| face.uv.len() == face.verts.len()));
}

#[test]
fn triangle_cut_count_produces_n_squared_triangles() {
    let mut face = Face::with_uv(
        vec![0, 1, 2],
        vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]],
    );
    face.selected = true;
    let mut mesh = Mesh {
        verts: vec![
            Vertex::new(0.0, 0.0, 0.0),
            Vertex::new(3.0, 0.0, 0.0),
            Vertex::new(0.0, 3.0, 0.0),
        ],
        faces: vec![face],
        selected_edges: HashSet::new(),
    };

    mesh.subdivide_selected_cuts(2);

    // Three segments per edge create 3^2 triangles and 10 lattice vertices.
    assert_eq!(mesh.faces.len(), 9);
    assert_eq!(mesh.verts.len(), 10);
    assert!(mesh.faces.iter().all(|face| face.verts.len() == 3));
    assert!(mesh.faces.iter().all(|face| face.uv.len() == 3));
}

#[test]
fn adjacent_selected_quads_share_edge_split_vertices() {
    let left = Face::with_uv(
        vec![0, 1, 4, 3],
        vec![[0.0, 0.0], [0.5, 0.0], [0.5, 1.0], [0.0, 1.0]],
    );
    let right = Face::with_uv(
        vec![1, 2, 5, 4],
        vec![[0.5, 0.0], [1.0, 0.0], [1.0, 1.0], [0.5, 1.0]],
    );
    let mut mesh = Mesh {
        verts: vec![
            Vertex::new(0.0, 0.0, 0.0),
            Vertex::new(1.0, 0.0, 0.0),
            Vertex::new(2.0, 0.0, 0.0),
            Vertex::new(0.0, 1.0, 0.0),
            Vertex::new(1.0, 1.0, 0.0),
            Vertex::new(2.0, 1.0, 0.0),
        ],
        faces: vec![left, right],
        selected_edges: HashSet::new(),
    };
    mesh.select_all();

    mesh.subdivide_selected_cuts(2);

    assert_eq!(mesh.faces.len(), 18);
    let shared_edge_vertices = mesh
        .verts
        .iter()
        .filter(|vertex| {
            (vertex.pos[0] - 1.0).abs() < 1.0e-5
                && vertex.pos[1] >= -1.0e-5
                && vertex.pos[1] <= 1.0 + 1.0e-5
                && vertex.pos[2].abs() < 1.0e-5
        })
        .count();
    // endpoints + two inserted cuts; duplicate boundary vertices would make this 6.
    assert_eq!(shared_edge_vertices, 4);
    assert!(mesh.faces.iter().all(|face| face.uv.len() == face.verts.len()));
}
''')


# ---------------------------------------------------------------------------
# 4. Modifier domain tests: non-destructive evaluation, enable toggle, export,
#    and persistence roundtrip.
# ---------------------------------------------------------------------------
modifier_test = ROOT / "crates/project/tests/modifier_stack.rs"
modifier_test.write_text(r'''use petunia_mesh::Mesh;
use petunia_project::{format, export_obj, Asset, ModifierInstance, Project};

#[test]
fn modifier_evaluation_does_not_mutate_base_mesh() {
    let mut asset = Asset::new("Cube", Mesh::cube(2.0));
    let base_vertices = asset.mesh.verts.len();
    let base_faces = asset.mesh.faces.len();
    asset.modifiers.push(ModifierInstance::mirror(0, 0.0));

    let evaluated = asset.evaluated_mesh();

    assert_eq!(asset.mesh.verts.len(), base_vertices);
    assert_eq!(asset.mesh.faces.len(), base_faces);
    assert_eq!(evaluated.verts.len(), base_vertices * 2);
    assert_eq!(evaluated.faces.len(), base_faces * 2);
}

#[test]
fn disabled_modifier_is_not_evaluated() {
    let mut asset = Asset::new("Cube", Mesh::cube(2.0));
    let mut mirror = ModifierInstance::mirror(0, 0.0);
    mirror.enabled = false;
    asset.modifiers.push(mirror);

    let evaluated = asset.evaluated_mesh();

    assert_eq!(evaluated.verts.len(), asset.mesh.verts.len());
    assert_eq!(evaluated.faces.len(), asset.mesh.faces.len());
}

#[test]
fn export_obj_uses_evaluated_modifier_stack() {
    let mut asset = Asset::new("Cube", Mesh::cube(2.0));
    asset.modifiers.push(ModifierInstance::mirror(0, 0.0));

    let obj = export_obj(&asset);
    let exported_vertices = obj.lines().filter(|line| line.starts_with("v ")).count();

    assert_eq!(exported_vertices, asset.mesh.verts.len() * 2);
}

#[test]
fn modifier_stack_roundtrips_through_project_format() {
    let mut project = Project::new();
    project.assets.clear();
    let mut asset = Asset::new("Modified Cube", Mesh::cube(2.0));
    let mirror = ModifierInstance::mirror(0, 0.0);
    let mirror_id = mirror.id;
    asset.modifiers.push(mirror);
    asset.modifiers.push(ModifierInstance::symmetry(1, true, 0.001));
    project.assets.push(asset);
    project.active = 0;

    let path = std::env::temp_dir().join(format!(
        "petunia_modifier_roundtrip_{}.petunia",
        uuid::Uuid::new_v4()
    ));
    format::save(&project, &path).unwrap();
    let loaded = format::load(&path).unwrap();
    let _ = std::fs::remove_file(&path);

    assert_eq!(loaded.assets.len(), 1);
    assert_eq!(loaded.assets[0].modifiers.len(), 2);
    assert_eq!(loaded.assets[0].modifiers[0].id, mirror_id);
    assert_eq!(loaded.assets[0].modifiers, project.assets[0].modifiers);
}
''')


# ---------------------------------------------------------------------------
# 5. Remove stale insta development artifact from the previous wave.
# ---------------------------------------------------------------------------
pending_snapshot = ROOT / "crates/project/tests/.save_tree.rs.pending-snap"
if pending_snapshot.exists():
    pending_snapshot.unlink()
