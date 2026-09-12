//! Corte por corda entre duas arestas de uma face, soldado às faces vizinhas.
use crate::{edge_key, Face, Mesh, Vertex};
use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct EdgePoint {
    pub edge: (u32, u32),
    pub position: Vec3,
}

pub fn cut_face(source: &Mesh, start: EdgePoint, end: EdgePoint) -> Result<Mesh, String> {
    // Endpoints podem reutilizar vértices sem passar pela inserção; o contrato
    // estrutural precisa ser verificado antes de qualquer caminho de corte.
    if source
        .verts
        .iter()
        .any(|v| !v.vec().is_finite() || v.color.iter().any(|component| !component.is_finite()))
        || source.faces.iter().any(|face| {
            face.verts.len() < 3
                || face.uv.len() != face.verts.len()
                || face.verts.iter().any(|&v| v as usize >= source.verts.len())
                || face
                    .uv
                    .iter()
                    .flatten()
                    .any(|component| !component.is_finite())
        })
    {
        return Err("Malha inválida: revise índices, UVs e coordenadas antes de cortar".into());
    }
    if !start.position.is_finite()
        || !end.position.is_finite()
        || edge_key(start.edge.0, start.edge.1) == edge_key(end.edge.0, end.edge.1)
    {
        return Err("Escolha duas arestas distintas da mesma face".into());
    }
    let contains = |face: &Face, edge: (u32, u32)| {
        (0..face.verts.len()).any(|k| {
            edge_key(face.verts[k], face.verts[(k + 1) % face.verts.len()])
                == edge_key(edge.0, edge.1)
        })
    };
    let face_index = source
        .faces
        .iter()
        .position(|face| contains(face, start.edge) && contains(face, end.edge))
        .ok_or("O segmento deve atravessar uma face; clique nas arestas intermediárias")?;
    let mut mesh = source.clone();
    let a = insert_point(&mut mesh, start)?;
    let b = insert_point(&mut mesh, end)?;
    let face = &mesh.faces[face_index];
    let ai = face
        .verts
        .iter()
        .position(|&v| v == a)
        .ok_or("Ponto inicial fora da face")?;
    let bi = face
        .verts
        .iter()
        .position(|&v| v == b)
        .ok_or("Ponto final fora da face")?;
    let path = |from: usize, to: usize| {
        let mut indices = vec![];
        let mut uv = vec![];
        let mut k = from;
        loop {
            indices.push(face.verts[k]);
            uv.push(face.uv[k]);
            if k == to {
                break;
            }
            k = (k + 1) % face.verts.len();
        }
        Face::with_uv(indices, uv)
    };
    let mut left = path(ai, bi);
    let mut right = path(bi, ai);
    if left.verts.len() < 3 || right.verts.len() < 3 {
        return Err("Corte coincide com o contorno".into());
    }
    let normal = source.face_normal(face_index);
    for polygon in [&left, &right] {
        let points: Vec<_> = polygon
            .verts
            .iter()
            .map(|&v| mesh.verts[v as usize].vec())
            .collect();
        // Signed triangle area prevents collapsed/reversed portions; concave cuts are rejected.
        for i in 1..points.len() - 1 {
            if (points[i] - points[0])
                .cross(points[i + 1] - points[0])
                .dot(normal)
                < -1e-6
            {
                return Err("Corte côncavo não suportado: divida em segmentos menores".into());
            }
        }
        if crate::triangulate::face_normal_of(&mesh.verts, &polygon.verts).length_squared() < 0.5 {
            return Err("Corte degenerado".into());
        }
    }
    left.selected = true;
    right.selected = true;
    mesh.faces[face_index] = left;
    mesh.faces.push(right);
    mesh.selected_edges.clear();
    mesh.selected_edges.insert(edge_key(a, b));
    mesh.sync_vert_selection_from_faces();
    Ok(mesh)
}

fn insert_point(mesh: &mut Mesh, point: EdgePoint) -> Result<u32, String> {
    let (a, b) = point.edge;
    let va = mesh.verts.get(a as usize).ok_or("Aresta inválida")?;
    let vb = mesh.verts.get(b as usize).ok_or("Aresta inválida")?;
    let delta = vb.vec() - va.vec();
    let length = delta.length_squared();
    if length < 1e-12 {
        return Err("Aresta degenerada".into());
    }
    let t = (point.position - va.vec()).dot(delta) / length;
    if !(-1e-4..=1.0001).contains(&t)
        || (va.vec() + delta * t - point.position).length_squared() > length * 1e-6
    {
        return Err("Ponto fora da aresta".into());
    }
    if t <= 1e-5 {
        return Ok(a);
    }
    if t >= 1.0 - 1e-5 {
        return Ok(b);
    }
    let color = std::array::from_fn(|i| va.color[i] + (vb.color[i] - va.color[i]) * t);
    let index = u32::try_from(mesh.verts.len()).map_err(|_| "Limite de vértices excedido")?;
    mesh.verts.push(Vertex {
        pos: point.position.to_array(),
        color,
        selected: true,
    });
    for face in &mut mesh.faces {
        if face.uv.len() != face.verts.len() {
            return Err("UV inválido".into());
        }
        let found = (0..face.verts.len()).find(|&k| {
            edge_key(face.verts[k], face.verts[(k + 1) % face.verts.len()]) == edge_key(a, b)
        });
        if let Some(k) = found {
            let factor = if face.verts[k] == a { t } else { 1.0 - t };
            let next = (k + 1) % face.verts.len();
            let uv = std::array::from_fn(|i| {
                face.uv[k][i] + (face.uv[next][i] - face.uv[k][i]) * factor
            });
            face.verts.insert(k + 1, index);
            face.uv.insert(k + 1, uv);
        }
    }
    Ok(index)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn knife_splits_cube_face_without_opening_neighbor_faces() {
        let mesh = Mesh::cube(2.0);
        let f = &mesh.faces[0];
        let edge = |a, b| EdgePoint {
            edge: (a, b),
            position: (mesh.verts[a as usize].vec() + mesh.verts[b as usize].vec()) * 0.5,
        };
        let result = cut_face(
            &mesh,
            edge(f.verts[0], f.verts[1]),
            edge(f.verts[2], f.verts[3]),
        )
        .unwrap();
        assert_eq!(result.faces.len(), 7);
        let (topology, defects) = crate::HalfEdgeMesh::from_mesh(&result);
        assert!(
            defects.is_empty() && topology.is_closed_manifold(),
            "{defects:?}"
        );
        assert!(result.faces.iter().all(|f| f.verts.len() == f.uv.len()));
    }

    #[test]
    fn knife_rejects_missing_uv_even_when_both_points_are_existing_vertices() {
        let mut mesh = Mesh::cube(2.0);
        let start = EdgePoint {
            edge: (0, 3),
            position: mesh.verts[0].vec(),
        };
        let end = EdgePoint {
            edge: (2, 1),
            position: mesh.verts[2].vec(),
        };
        mesh.faces[0].uv.clear();
        let before = format!("{mesh:?}");
        assert!(cut_face(&mesh, start, end).is_err());
        assert_eq!(format!("{mesh:?}"), before);
    }

    #[test]
    fn knife_rejects_invalid_nonendpoint_index_before_geometry_access() {
        let mut mesh = Mesh::cube(2.0);
        let start = EdgePoint {
            edge: (0, 3),
            position: mesh.verts[0].vec(),
        };
        let end = EdgePoint {
            edge: (2, 1),
            position: mesh.verts[2].vec(),
        };
        mesh.faces[0].verts.push(u32::MAX);
        mesh.faces[0].uv.push([0.0, 0.0]);
        let before = format!("{mesh:?}");
        assert!(cut_face(&mesh, start, end).is_err());
        assert_eq!(format!("{mesh:?}"), before);
    }

    #[test]
    fn knife_rejects_nonfinite_source_attributes() {
        for corrupt_uv in [false, true] {
            let mut mesh = Mesh::cube(2.0);
            let start = EdgePoint {
                edge: (0, 3),
                position: mesh.verts[0].vec(),
            };
            let end = EdgePoint {
                edge: (2, 1),
                position: mesh.verts[2].vec(),
            };
            if corrupt_uv {
                mesh.faces[0].uv[1][0] = f32::NAN;
            } else {
                mesh.verts[7].pos[0] = f32::INFINITY;
            }
            assert!(cut_face(&mesh, start, end).is_err());
        }
    }
}
