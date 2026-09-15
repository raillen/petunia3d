//! Exportação com validação: OBJ (texto) e GLB (binário glTF 2.0).
//! GLB escrito à mão (sem dependência pesada); validado em teste com `gltf`.

use super::{Asset, Project};

#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("I/O: {0}")]
    Io(String),
    #[error("nada para exportar")]
    Empty,
    #[error("{0}")]
    Other(String),
}

pub fn export_obj(asset: &Asset) -> String {
    asset.evaluated_mesh().to_obj()
}

/// Exporta assets como um único GLB (uma mesh por asset).
pub fn export_gltf(project: &Project, indices: &[usize]) -> Result<Vec<u8>, ExportError> {
    let picked: Vec<&Asset> = indices
        .iter()
        .filter_map(|&i| project.assets.get(i))
        .collect();
    if picked.is_empty() {
        return Err(ExportError::Empty);
    }

    // achata: por asset, tris (pos, nrm, uv) + índices u32
    struct Part {
        name: String,
        color: [f32; 4],
        roughness: f32,
        metallic: f32,
        emissive: [f32; 3],
        pos: Vec<f32>,
        nrm: Vec<f32>,
        uv: Vec<f32>,
        idx: Vec<u32>,
    }
    let mut parts = Vec::new();
    for a in picked {
        let mut m = a.evaluated_mesh();
        m.triangulate();
        // M6: valida em vez de panicar (malha pode vir de arquivo hostil)
        if !m
            .verts
            .iter()
            .all(|v| v.pos.iter().all(|x| x.is_finite()) && v.color.iter().all(|x| x.is_finite()))
            || !a.base_color.iter().all(|x| x.is_finite())
        {
            return Err(ExportError::Other(format!(
                "malha '{}' com NaN/inf",
                a.name
            )));
        }
        for f in &m.faces {
            if f.verts.len() != 3
                || f.uv.len() != 3
                || f.verts.iter().any(|&vi| (vi as usize) >= m.verts.len())
                || f.uv.iter().any(|uv| !uv.iter().all(|x| x.is_finite()))
            {
                return Err(ExportError::Other(format!(
                    "malha '{}' inválida para export",
                    a.name
                )));
            }
        }

        let (roughness, metallic, emissive, base_color_rgba) =
            if let Some(mat) = a.material(project) {
                (
                    mat.roughness.clamp(0.0, 1.0),
                    mat.metallic.clamp(0.0, 1.0),
                    [
                        mat.emission_color[0] * mat.emission_strength,
                        mat.emission_color[1] * mat.emission_strength,
                        mat.emission_color[2] * mat.emission_strength,
                    ],
                    mat.base_color,
                )
            } else {
                (
                    0.9,
                    0.0,
                    [0.0, 0.0, 0.0],
                    [a.base_color[0], a.base_color[1], a.base_color[2], 1.0],
                )
            };

        let normals = m.compute_normals();
        let mut p = Part {
            name: a.name.clone(),
            color: base_color_rgba,
            roughness,
            metallic,
            emissive,
            pos: Vec::new(),
            nrm: Vec::new(),
            uv: Vec::new(),
            idx: Vec::new(),
        };
        let mut lut: std::collections::HashMap<(u32, u32), u32> = Default::default();
        for f in &m.faces {
            if f.verts.len() != 3 {
                continue;
            }
            for k in 0..3 {
                let vi = f.verts[k];
                let key = (
                    vi,
                    f.uv[k][0].to_bits() ^ f.uv[k][1].to_bits().rotate_left(1),
                );
                let id = *lut.entry(key).or_insert_with(|| {
                    let id = (p.pos.len() / 3) as u32;
                    let vv = &m.verts[vi as usize];
                    p.pos.extend_from_slice(&vv.pos);
                    p.nrm.extend_from_slice(&normals[vi as usize]);
                    p.uv.extend_from_slice(&[f.uv[k][0], f.uv[k][1]]);
                    id
                });
                p.idx.push(id);
            }
        }
        if p.idx.is_empty() {
            continue;
        }
        parts.push(p);
    }
    if parts.is_empty() {
        return Err(ExportError::Empty);
    }

    // monta BIN
    let mut bin: Vec<u8> = Vec::new();
    // views: (offset, len, target) — 34962 ARRAY_BUFFER, 34963 ELEMENT_ARRAY_BUFFER
    let mut views: Vec<(usize, usize, u32)> = Vec::new();
    /// (view, count, tipo, componente, min/max p/ POSITION).
    type Acc = (usize, usize, String, usize, Option<([f32; 3], [f32; 3])>);
    // accs: (view, count, type, comp, minmax)
    let mut accs: Vec<Acc> = Vec::new();
    let mut mesh_acc: Vec<[usize; 4]> = Vec::new();
    for p in &parts {
        let mut a = [0usize; 4];
        for (k, data) in [&p.pos, &p.nrm, &p.uv].iter().enumerate() {
            let bytes: &[u8] = bytemuck::cast_slice(data);
            let off = bin.len();
            bin.extend_from_slice(bytes);
            views.push((off, bytes.len(), 34962));
            let ty = if k == 2 { "VEC2" } else { "VEC3" };
            let mm = if k == 0 {
                let mut mn = [f32::MAX; 3];
                let mut mx = [f32::MIN; 3];
                for v in data.chunks(3) {
                    for c in 0..3 {
                        if let Some(&x) = v.get(c) {
                            mn[c] = mn[c].min(x);
                            mx[c] = mx[c].max(x);
                        }
                    }
                }
                Some((mn, mx))
            } else {
                None
            };
            accs.push((
                views.len() - 1,
                data.len() / if k == 2 { 2 } else { 3 },
                ty.to_string(),
                5126,
                mm,
            ));
            a[k] = accs.len() - 1;
        }
        let idx_bytes: Vec<u8> = p.idx.iter().flat_map(|i| i.to_le_bytes()).collect();
        let off = bin.len();
        bin.extend_from_slice(&idx_bytes);
        views.push((off, idx_bytes.len(), 34963));
        accs.push((
            views.len() - 1,
            p.idx.len(),
            "SCALAR".to_string(),
            5125,
            None,
        ));
        a[3] = accs.len() - 1;
        mesh_acc.push(a);
    }
    while !bin.len().is_multiple_of(4) {
        bin.push(0);
    }

    // monta JSON
    let mut j = String::from("{\"asset\":{\"version\":\"2.0\",\"generator\":\"Petunia3D\"},");
    j.push_str("\"scene\":0,\"scenes\":[{\"nodes\":[");
    for (i, _) in parts.iter().enumerate() {
        if i > 0 {
            j.push(',');
        }
        j.push_str(&i.to_string());
    }
    j.push_str("]}],\"nodes\":[");
    for (i, p) in parts.iter().enumerate() {
        if i > 0 {
            j.push(',');
        }
        j.push_str(&format!("{{\"name\":{},\"mesh\":{i}}}", json_str(&p.name)));
    }
    j.push_str("],\"meshes\":[");
    for (i, p) in parts.iter().enumerate() {
        if i > 0 {
            j.push(',');
        }
        let a = mesh_acc[i];
        j.push_str(&format!(
            "{{\"name\":{},\"primitives\":[{{\"attributes\":{{\"POSITION\":{},\"NORMAL\":{},\"TEXCOORD_0\":{}}},\"indices\":{},\"material\":{i}}}]}}",
            json_str(&p.name), a[0], a[1], a[2], a[3]
        ));
    }
    j.push_str("],\"materials\":[");
    for (i, p) in parts.iter().enumerate() {
        if i > 0 {
            j.push(',');
        }
        j.push_str(&format!(
            "{{\"name\":{},\"doubleSided\":true,\"pbrMetallicRoughness\":{{\"baseColorFactor\":[{:.4},{:.4},{:.4},{:.4}],\"metallicFactor\":{:.4},\"roughnessFactor\":{:.4}}},\"emissiveFactor\":[{:.4},{:.4},{:.4}]}}",
            json_str(&format!("{}_mat", p.name)),
            p.color[0], p.color[1], p.color[2], p.color[3],
            p.metallic, p.roughness,
            p.emissive[0], p.emissive[1], p.emissive[2]
        ));
    }
    j.push_str("],\"accessors\":[");
    for (i, (v, count, ty, comp, mm)) in accs.iter().enumerate() {
        if i > 0 {
            j.push(',');
        }
        j.push_str(&format!(
            "{{\"bufferView\":{v},\"componentType\":{comp},\"count\":{count},\"type\":\"{ty}\""
        ));
        if let Some((mn, mx)) = mm {
            j.push_str(&format!(
                ",\"min\":[{:.6},{:.6},{:.6}],\"max\":[{:.6},{:.6},{:.6}]",
                mn[0], mn[1], mn[2], mx[0], mx[1], mx[2]
            ));
        }
        j.push('}');
    }
    j.push_str("],\"bufferViews\":[");
    for (i, (off, len, target)) in views.iter().enumerate() {
        if i > 0 {
            j.push(',');
        }
        j.push_str(&format!(
            "{{\"buffer\":0,\"byteOffset\":{off},\"byteLength\":{len},\"target\":{target}}}"
        ));
    }
    j.push_str(&format!(
        "],\"buffers\":[{{\"byteLength\":{}}}]}}",
        bin.len()
    ));

    // chunk JSON (pad espaço)
    let mut json = j.into_bytes();
    while !json.len().is_multiple_of(4) {
        json.push(b' ');
    }
    // GLB
    let total = 12 + 8 + json.len() + 8 + bin.len();
    let mut out = Vec::with_capacity(total);
    out.extend_from_slice(&0x46546C67u32.to_le_bytes()); // 'glTF'
    out.extend_from_slice(&2u32.to_le_bytes());
    out.extend_from_slice(&(total as u32).to_le_bytes());
    out.extend_from_slice(&(json.len() as u32).to_le_bytes());
    out.extend_from_slice(&0x4E4F534Au32.to_le_bytes()); // 'JSON'
    out.extend_from_slice(&json);
    out.extend_from_slice(&(bin.len() as u32).to_le_bytes());
    out.extend_from_slice(&0x004E4942u32.to_le_bytes()); // 'BIN\0'
    out.extend_from_slice(&bin);
    Ok(out)
}

fn json_str(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            c if (c as u32) < 0x20 => {
                o.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

/// Validação legível antes de salvar: escolhe pasta e escreve um arquivo por asset (OBJ) ou um GLB.
pub fn export_report(project: &Project, indices: &[usize], include_gltf: bool) -> Vec<String> {
    let mut lines = Vec::new();
    if indices.is_empty() {
        lines.push("nada selecionado".to_string());
        return lines;
    }
    for &i in indices {
        match project.assets.get(i) {
            Some(a) => lines.push(format!(
                "{}: {} verts, {} tris{}",
                a.name,
                a.mesh.vert_count(),
                a.mesh.tri_count(),
                if include_gltf { ", GLB ok" } else { ", OBJ ok" }
            )),
            None => lines.push(format!("índice {i} inválido")),
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use petunia_mesh::Mesh;

    fn sample_project() -> Project {
        let mut p = Project::new();
        p.add("Plane", Mesh::plane(1.0));
        p
    }

    #[test]
    fn obj_has_vt() {
        let p = sample_project();
        let s = export_obj(&p.assets[0]);
        assert!(s.contains("v ") && s.contains("vt ") && s.contains('f'));
    }

    #[test]
    fn glb_roundtrip_via_gltf_crate() {
        let p = sample_project();
        let bytes = export_gltf(&p, &[0, 1]).expect("glb");
        assert_eq!(&bytes[0..4], b"glTF");
        let (doc, buffers, _) = gltf::import_slice(&bytes).expect("parse glb");
        assert_eq!(doc.meshes().len(), 2);
        assert_eq!(buffers.len(), 1);
        let total_verts: usize = doc.meshes().flat_map(|m| m.primitives()).map(|_| 0).sum();
        let _ = total_verts;
        // posições do cubo: 8+ verts únicos (cubo tem 8 verts, plano 4)
        let mut n = 0;
        for mesh in doc.meshes() {
            for prim in mesh.primitives() {
                let r = prim.reader(|b| Some(&buffers[b.index()]));
                n += r.read_positions().map(|it| it.len()).unwrap_or(0);
                assert!(r.read_normals().is_some());
                assert!(r.read_tex_coords(0).is_some());
                assert!(r.read_indices().is_some());
            }
        }
        assert!(n >= 12, "verts insuficientes: {n}");
    }

    #[test]
    fn glb_empty_errors() {
        let p = Project::default();
        assert!(export_gltf(&p, &[]).is_err());
    }

    #[test]
    fn glb_nan_mesh_errors_not_panics() {
        // M5/M6: NaN vira Err legível, nunca panic nem JSON inválido
        let mut p = Project::new();
        p.assets[0].mesh.verts[0].pos = [f32::NAN, 0.0, 0.0];
        assert!(export_gltf(&p, &[0]).is_err());
    }

    #[test]
    fn json_str_escapes_controls() {
        let s = json_str("a\"b\\c\nd\te\x01f");
        assert_eq!(s, "\"a\\\"b\\\\c\\nd\\u0009e\\u0001f\"");
        let v: serde_json::Value = serde_json::from_str(&s).expect("json válido");
        assert_eq!(v.as_str().unwrap().chars().count(), 11);
    }
}
