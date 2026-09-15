//! Petunia3D mesh — primitivas procedurais.

use super::{Face, Mesh, Vertex, triangulate};
use std::collections::HashSet;

impl Mesh {
    // ---------------- primitivas ----------------

    pub fn cube(size: f32) -> Self {
        let h = size * 0.5;
        let mut m = Self {
            verts: vec![
                Vertex::new(-h, -h, -h),
                Vertex::new(h, -h, -h),
                Vertex::new(h, h, -h),
                Vertex::new(-h, h, -h),
                Vertex::new(-h, -h, h),
                Vertex::new(h, -h, h),
                Vertex::new(h, h, h),
                Vertex::new(-h, h, h),
            ],
            faces: vec![
                Face::new(vec![0, 3, 2, 1]),
                Face::new(vec![4, 5, 6, 7]),
                Face::new(vec![0, 1, 5, 4]),
                Face::new(vec![2, 3, 7, 6]),
                Face::new(vec![0, 4, 7, 3]),
                Face::new(vec![1, 2, 6, 5]),
            ],
            selected_edges: HashSet::new(),
        };
        m.project_planar();
        m
    }

    pub fn plane(size: f32) -> Self {
        let h = size * 0.5;
        let mut m = Self {
            verts: vec![
                Vertex::new(-h, 0.0, -h),
                Vertex::new(h, 0.0, -h),
                Vertex::new(h, 0.0, h),
                Vertex::new(-h, 0.0, h),
            ],
            faces: vec![Face::with_uv(
                vec![0, 3, 2, 1],
                vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]],
            )],
            selected_edges: HashSet::new(),
        };
        m.project_planar();
        m
    }

    pub fn cylinder(segments: u32, radius: f32, height: f32) -> Self {
        let seg = segments.clamp(3, 32) as usize;
        let h = height * 0.5;
        let mut m = Mesh::default();
        for i in 0..seg {
            let a = (i as f32 / seg as f32) * std::f32::consts::TAU;
            m.verts
                .push(Vertex::new(a.cos() * radius, -h, a.sin() * radius));
            m.verts
                .push(Vertex::new(a.cos() * radius, h, a.sin() * radius));
        }
        for i in 0..seg {
            let j = (i + 1) % seg;
            m.push_face(Face::new(vec![
                (i * 2) as u32,
                (i * 2 + 1) as u32,
                (j * 2 + 1) as u32,
                (j * 2) as u32,
            ]));
        }
        let cb = m.verts.len() as u32;
        m.verts.push(Vertex::new(0.0, -h, 0.0));
        let ct = m.verts.len() as u32;
        m.verts.push(Vertex::new(0.0, h, 0.0));
        for i in 0..seg {
            let j = (i + 1) % seg;
            m.push_face(Face::new(vec![cb, (i * 2) as u32, (j * 2) as u32]));
            m.push_face(Face::new(vec![ct, (j * 2 + 1) as u32, (i * 2 + 1) as u32]));
        }
        m.project_planar();
        m
    }

    pub fn cone(segments: u32, radius: f32, height: f32) -> Self {
        let seg = segments.clamp(3, 32) as usize;
        let h = height * 0.5;
        let mut m = Mesh::default();
        for i in 0..seg {
            let a = (i as f32 / seg as f32) * std::f32::consts::TAU;
            m.verts
                .push(Vertex::new(a.cos() * radius, -h, a.sin() * radius));
        }
        let tip = m.verts.len() as u32;
        m.verts.push(Vertex::new(0.0, h, 0.0));
        let cb = m.verts.len() as u32;
        m.verts.push(Vertex::new(0.0, -h, 0.0));
        for i in 0..seg {
            let j = (i + 1) % seg;
            m.push_face(Face::new(vec![tip, j as u32, i as u32]));
            m.push_face(Face::new(vec![cb, i as u32, j as u32]));
        }
        m.project_planar();
        m
    }

    pub fn sphere_low(segments: u32, rings: u32, radius: f32) -> Self {
        let seg = segments.clamp(3, 32) as usize;
        let rg = rings.clamp(2, 24) as usize;
        let mut m = Mesh::default();
        for r in 0..=rg {
            let v = r as f32 / rg as f32;
            let phi = v * std::f32::consts::PI;
            for s in 0..seg {
                let u = s as f32 / seg as f32;
                let theta = u * std::f32::consts::TAU;
                m.verts.push(Vertex::new(
                    radius * phi.sin() * theta.cos(),
                    radius * phi.cos(),
                    radius * phi.sin() * theta.sin(),
                ));
            }
        }
        for r in 0..rg {
            for s in 0..seg {
                let a = (r * seg + s) as u32;
                let b = (r * seg + (s + 1) % seg) as u32;
                let c = ((r + 1) * seg + (s + 1) % seg) as u32;
                let d = ((r + 1) * seg + s) as u32;
                if r == 0 {
                    m.push_face(Face::new(vec![a, c, d]));
                } else if r == rg - 1 {
                    m.push_face(Face::new(vec![a, b, d]));
                } else {
                    m.push_face(Face::new(vec![a, b, c, d]));
                }
            }
        }
        m.weld(1e-4);
        m.project_planar();
        m
    }

    /// Cápsula low-poly: corpo cilíndrico + calotas hemisféricas.
    pub fn capsule(segments: u32, radius: f32, height: f32) -> Self {
        let seg = segments.clamp(4, 32) as usize;
        let hh = (height * 0.5 - radius).max(0.0);
        let mut m = Mesh::default();
        // anéis: calota sul (2) + corpo (2) + calota norte (2)
        let rings_y = [
            -hh - radius,
            -hh - radius * 0.55,
            -hh,
            hh,
            hh + radius * 0.55,
            hh + radius,
        ];
        let rings_r = [0.001, radius * 0.83, radius, radius, radius * 0.83, 0.001];
        for (y, r) in rings_y.iter().zip(rings_r.iter()) {
            for s in 0..seg {
                let a = (s as f32 / seg as f32) * std::f32::consts::TAU;
                m.verts.push(Vertex::new(a.cos() * r, *y, a.sin() * r));
            }
        }
        for r in 0..rings_y.len() - 1 {
            for s in 0..seg {
                let a = (r * seg + s) as u32;
                let b = (r * seg + (s + 1) % seg) as u32;
                let c = ((r + 1) * seg + (s + 1) % seg) as u32;
                let d = ((r + 1) * seg + s) as u32;
                m.push_face(Face::new(vec![a, d, c, b]));
            }
        }
        m.project_planar();
        m
    }

    /// Revolve (torno): perfil `[[raio, altura], ..]` girado em torno do eixo Y.
    /// Raio é clampado em >= 0. Fecha polos quando o perfil encosta no eixo.
    pub fn revolve(profile: &[[f32; 2]], segments: u32) -> Result<Self, String> {
        if profile.len() < 2 {
            return Err("perfil precisa de ao menos 2 pontos".to_string());
        }
        let seg = segments.clamp(3, 64) as usize;
        let mut m = Mesh::default();
        // anel por ponto do perfil
        for &[r, y] in profile {
            let r = r.max(0.0);
            for s in 0..seg {
                let a = (s as f32 / seg as f32) * std::f32::consts::TAU;
                m.verts.push(Vertex::new(a.cos() * r, y, a.sin() * r));
            }
        }
        let axis = |k: usize| profile[k][0].max(0.0) < 1e-6;
        for k in 0..profile.len() - 1 {
            if axis(k) && axis(k + 1) {
                continue; // segmento degenerado no eixo
            }
            for s in 0..seg {
                let a = (k * seg + s) as u32;
                let b = (k * seg + (s + 1) % seg) as u32;
                let c = ((k + 1) * seg + (s + 1) % seg) as u32;
                let d = ((k + 1) * seg + s) as u32;
                if axis(k) {
                    m.push_face(Face::new(vec![a, d, c]));
                } else if axis(k + 1) {
                    m.push_face(Face::new(vec![a, d, b]));
                } else {
                    m.push_face(Face::new(vec![a, d, c, b]));
                }
            }
        }
        m.project_planar();
        Ok(m)
    }

    /// Malha a partir de polígono 2D (ear clipping) extrudado em +Z.
    pub fn from_polygon(points: &[[f32; 2]], depth: f32) -> Result<Self, String> {
        let tris = triangulate::ear_clip(points)?;
        let n = points.len();
        let mut m = Mesh::default();
        for &[x, y] in points {
            m.verts.push(Vertex::new(x, y, 0.0));
        }
        for &[x, y] in points {
            m.verts.push(Vertex::new(x, y, depth.max(0.01)));
        }
        let off = n as u32;
        for &[a, b, c] in &tris {
            // base (normal -Z) e tampa (+Z)
            m.push_face(Face::new(vec![c as u32, b as u32, a as u32]));
            m.push_face(Face::new(vec![
                off + a as u32,
                off + b as u32,
                off + c as u32,
            ]));
        }
        // laterais: segue a ordem do polígono (assume CCW após ear_clip)
        for k in 0..n {
            let k2 = (k + 1) % n;
            m.push_face(Face::new(vec![
                k as u32,
                k2 as u32,
                off + k2 as u32,
                off + k as u32,
            ]));
        }
        m.project_planar();
        Ok(m)
    }
}
