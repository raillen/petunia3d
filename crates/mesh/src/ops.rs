//! Petunia3D mesh — operações de modelagem.

use super::{Face, Mesh, Vertex, edge_key, triangulate};
use glam::Vec3;
use std::collections::HashMap;

impl Mesh {
    // ---------------- seleção ----------------

    pub fn select_all(&mut self) {
        for v in &mut self.verts {
            v.selected = true;
        }
        for f in &mut self.faces {
            f.selected = true;
        }
        self.selected_edges = self.edges_unique().into_iter().collect();
    }
    pub fn deselect_all(&mut self) {
        for v in &mut self.verts {
            v.selected = false;
        }
        for f in &mut self.faces {
            f.selected = false;
        }
        self.selected_edges.clear();
    }

    /// Inverte o estado de seleção de todos os vértices e sincroniza faces e arestas.
    pub fn invert_selection(&mut self) {
        for v in &mut self.verts {
            v.selected = !v.selected;
        }
        self.sync_face_selection_from_verts();
        self.sync_edge_selection_from_verts();
    }

    /// Seleciona todos os vértices e faces conectados geometricamente aos elementos já selecionados (Select Linked).
    pub fn select_linked(&mut self) {
        let mut queue = std::collections::VecDeque::new();
        let mut visited = vec![false; self.verts.len()];

        for (i, v) in self.verts.iter().enumerate() {
            if v.selected {
                queue.push_back(i);
                visited[i] = true;
            }
        }

        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); self.verts.len()];
        for f in &self.faces {
            let m = f.verts.len();
            for k in 0..m {
                let u = f.verts[k] as usize;
                let v = f.verts[(k + 1) % m] as usize;
                if u < self.verts.len() && v < self.verts.len() {
                    adj[u].push(v);
                    adj[v].push(u);
                }
            }
        }

        while let Some(u) = queue.pop_front() {
            for &v in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    self.verts[v].selected = true;
                    queue.push_back(v);
                }
            }
        }

        self.sync_face_selection_from_verts();
        self.sync_edge_selection_from_verts();
    }

    /// Seleciona vértices contidos dentro de um retângulo em coordenadas normalizadas de tela (NDC [-1, 1]).
    pub fn box_select(
        &mut self,
        min_ndc: [f32; 2],
        max_ndc: [f32; 2],
        view_proj: &[f32; 16],
        add_mode: bool,
    ) {
        let vp = glam::Mat4::from_cols_array(view_proj);
        let x_min = min_ndc[0].min(max_ndc[0]);
        let x_max = min_ndc[0].max(max_ndc[0]);
        let y_min = min_ndc[1].min(max_ndc[1]);
        let y_max = min_ndc[1].max(max_ndc[1]);

        for v in &mut self.verts {
            let clip = vp * v.vec().extend(1.0);
            let inside = if clip.w > 1e-4 {
                let ndc = clip.truncate() / clip.w;
                ndc.x >= x_min
                    && ndc.x <= x_max
                    && ndc.y >= y_min
                    && ndc.y <= y_max
                    && ndc.z >= -1.0
                    && ndc.z <= 1.0
            } else {
                false
            };
            if add_mode {
                v.selected |= inside;
            } else {
                v.selected = inside;
            }
        }
        self.sync_face_selection_from_verts();
        self.sync_edge_selection_from_verts();
    }
    pub fn selected_vert_count(&self) -> usize {
        self.verts.iter().filter(|v| v.selected).count()
    }
    pub fn selected_face_count(&self) -> usize {
        self.faces.iter().filter(|f| f.selected).count()
    }
    pub fn has_selection(&self) -> bool {
        self.selected_vert_count() > 0
            || self.selected_face_count() > 0
            || !self.selected_edges.is_empty()
    }

    pub fn sync_face_selection_from_verts(&mut self) {
        for f in &mut self.faces {
            f.selected =
                !f.verts.is_empty() && f.verts.iter().all(|&i| self.verts[i as usize].selected);
        }
    }
    pub fn sync_vert_selection_from_faces(&mut self) {
        for v in &mut self.verts {
            v.selected = false;
        }
        for f in &self.faces {
            if f.selected {
                for &i in &f.verts {
                    self.verts[i as usize].selected = true;
                }
            }
        }
    }
    /// Arestas totalmente selecionadas (ambos os extremos marcados).
    pub fn sync_edge_selection_from_verts(&mut self) {
        self.selected_edges.clear();
        for (a, b) in self.edges_unique() {
            if self.verts[a as usize].selected && self.verts[b as usize].selected {
                self.selected_edges.insert((a, b));
            }
        }
    }

    // ---------------- transformações ----------------

    pub fn translate_selected(&mut self, d: [f32; 3]) {
        let any = self.verts.iter().any(|v| v.selected);
        for v in &mut self.verts {
            if v.selected || !any {
                v.pos[0] += d[0];
                v.pos[1] += d[1];
                v.pos[2] += d[2];
            }
        }
    }

    pub fn scale_selected(&mut self, s: f32, center: [f32; 3]) {
        for v in &mut self.verts {
            if v.selected {
                for (p, c) in v.pos.iter_mut().zip(center.iter()) {
                    *p = c + (*p - c) * s;
                }
            }
        }
    }

    pub fn selection_center(&self) -> [f32; 3] {
        let mut c = Vec3::ZERO;
        let mut n = 0;
        for v in &self.verts {
            if v.selected {
                c += v.vec();
                n += 1;
            }
        }
        if n == 0 {
            for v in &self.verts {
                c += v.vec();
            }
            n = self.verts.len().max(1);
        }
        (c / n as f32).to_array()
    }

    // ---------------- operações ----------------

    pub fn extrude_selected(&mut self, dist: f32) {
        if !dist.is_finite() {
            return;
        }
        let sel: Vec<usize> = self
            .faces
            .iter()
            .enumerate()
            .filter(|(_, f)| f.selected)
            .map(|(i, _)| i)
            .collect();
        if sel.is_empty() {
            return;
        }
        // A região compartilha vértices e apenas o contorno ganha paredes.
        // Distância zero é permitida para a preparação de previews modais.
        let mut normal = Vec3::ZERO;
        let mut edge_uses: HashMap<(u32, u32), usize> = HashMap::new();
        for &fi in &sel {
            let src = &self.faces[fi];
            if src.verts.len() < 3
                || src.uv.len() != src.verts.len()
                || src.verts.iter().any(|&vi| {
                    self.verts
                        .get(vi as usize)
                        .is_none_or(|v| !v.vec().is_finite())
                })
            {
                return;
            }
            normal += self.face_normal(fi);
            for k in 0..src.verts.len() {
                *edge_uses
                    .entry(edge_key(src.verts[k], src.verts[(k + 1) % src.verts.len()]))
                    .or_default() += 1;
            }
        }
        if edge_uses.values().any(|&count| count > 2) {
            return;
        }
        let delta = normal.normalize_or_zero() * dist;
        if !delta.is_finite() {
            return;
        }
        let mut remap = HashMap::new();
        let mut new_vertices = Vec::new();
        for &fi in &sel {
            for &vi in &self.faces[fi].verts {
                if let std::collections::hash_map::Entry::Vacant(entry) = remap.entry(vi) {
                    let mut v = self.verts[vi as usize].clone();
                    v.pos = (v.vec() + delta).to_array();
                    if !v.vec().is_finite() {
                        return;
                    }
                    v.selected = true;
                    let Ok(new_index) = u32::try_from(self.verts.len() + new_vertices.len()) else {
                        return;
                    };
                    entry.insert(new_index);
                    new_vertices.push(v);
                }
            }
        }
        self.verts.extend(new_vertices);
        for &fi in &sel {
            let src = self.faces[fi].clone();
            let new_idx: Vec<u32> = src.verts.iter().map(|vi| remap[vi]).collect();
            // Substitui a base: mantê-la criaria três faces na mesma aresta.
            self.faces[fi].verts = new_idx.clone();
            let m = src.verts.len();
            for k in 0..m {
                let k2 = (k + 1) % m;
                if edge_uses[&edge_key(src.verts[k], src.verts[k2])] != 1 {
                    continue;
                }
                // laterais: u ao longo do perímetro, v 0 embaixo / 1 em cima
                let side = Face::with_uv(
                    vec![src.verts[k], src.verts[k2], new_idx[k2], new_idx[k]],
                    vec![
                        [k as f32 / m as f32, 0.0],
                        [(k + 1) as f32 / m as f32, 0.0],
                        [(k + 1) as f32 / m as f32, 1.0],
                        [k as f32 / m as f32, 1.0],
                    ],
                );
                self.push_face(side);
            }
        }
        self.selected_edges.clear();
        self.remove_isolated_vertices();
        self.sync_vert_selection_from_faces();
    }

    /// Extrusão individual de cada face selecionada ao longo de sua própria normal (Alt+E).
    /// Gera prismas desacoplados sem compartilhar paredes laterais entre faces vizinhas.
    pub fn extrude_individual(&mut self, dist: f32) {
        if !dist.is_finite() {
            return;
        }
        let sel: Vec<usize> = self
            .faces
            .iter()
            .enumerate()
            .filter(|(_, f)| f.selected)
            .map(|(i, _)| i)
            .collect();
        if sel.is_empty() {
            return;
        }

        for &fi in &sel {
            let src = self.faces[fi].clone();
            let m = src.verts.len();
            if m < 3 {
                continue;
            }
            let normal = self.face_normal(fi);
            let delta = normal.normalize_or_zero() * dist;

            // Cria novos vértices exclusivos para o topo desta face
            let mut top_verts = Vec::with_capacity(m);
            for &vi in &src.verts {
                let mut v = self.verts[vi as usize].clone();
                v.pos = (v.vec() + delta).to_array();
                v.selected = true;
                let new_idx = self.verts.len() as u32;
                self.verts.push(v);
                top_verts.push(new_idx);
            }

            // Atualiza a face original para ser a tampa superior extrudada
            self.faces[fi].verts = top_verts.clone();

            // Gera as paredes laterais conectando a base original ao novo topo
            for k in 0..m {
                let k2 = (k + 1) % m;
                let bottom0 = src.verts[k];
                let bottom1 = src.verts[k2];
                let top0 = top_verts[k];
                let top1 = top_verts[k2];

                let wall_face = Face::new(vec![bottom0, bottom1, top1, top0]);
                self.push_face(wall_face);
            }
        }

        self.selected_edges.clear();
        self.sync_vert_selection_from_faces();
    }

    /// Push/Pull: move a seleção ao longo da normal média (sem criar faces).
    pub fn push_pull(&mut self, dist: f32) {
        let mut n = Vec3::ZERO;
        let mut count = 0;
        for (i, f) in self.faces.iter().enumerate() {
            if f.selected {
                n += self.face_normal(i);
                count += 1;
            }
        }
        let dir = if count > 0 {
            (n / count as f32).normalize_or_zero()
        } else {
            Vec3::Y
        };
        let d = (dir * dist).to_array();
        self.translate_selected(d);
    }

    /// Inset métrico com proteção de auto-interseção e preservação de orientação topológica.
    pub fn inset_selected(&mut self, factor: f32) {
        if !factor.is_finite() || factor <= 0.0 {
            return;
        }
        let sel: Vec<usize> = self
            .faces
            .iter()
            .enumerate()
            .filter(|(_, x)| x.selected)
            .map(|(i, _)| i)
            .collect();
        for &fi in &sel {
            let src = self.faces[fi].clone();
            let m = src.verts.len();
            if m < 3 {
                continue;
            }
            let orig_normal = self.face_normal(fi);

            let mut c = Vec3::ZERO;
            let mut cuv = [0.0f32; 2];
            for (k, &vi) in src.verts.iter().enumerate() {
                c += self.verts[vi as usize].vec();
                cuv[0] += src.uv[k][0];
                cuv[1] += src.uv[k][1];
            }
            let mf = m as f32;
            c /= mf;
            cuv[0] /= mf;
            cuv[1] /= mf;

            // Ajusta o fator dinamicamente para prevenir auto-interseção ou inversão de winding
            let mut safe_f = factor.clamp(0.01, 0.95);
            for _ in 0..5 {
                let p0 = self.verts[src.verts[0] as usize].vec().lerp(c, safe_f);
                let p1 = self.verts[src.verts[1] as usize].vec().lerp(c, safe_f);
                let p2 = self.verts[src.verts[2] as usize].vec().lerp(c, safe_f);
                let inner_n = (p1 - p0).cross(p2 - p0);
                if inner_n.dot(orig_normal) > 1e-4 {
                    break;
                }
                safe_f *= 0.5;
            }

            let mut inner = Vec::with_capacity(m);
            let mut inner_uv = Vec::with_capacity(m);
            for (k, &vi) in src.verts.iter().enumerate() {
                let p = self.verts[vi as usize].vec().lerp(c, safe_f);
                let uv = [
                    src.uv[k][0] + (cuv[0] - src.uv[k][0]) * safe_f,
                    src.uv[k][1] + (cuv[1] - src.uv[k][1]) * safe_f,
                ];
                self.verts.push(Vertex {
                    pos: p.to_array(),
                    color: self.verts[vi as usize].color,
                    selected: true,
                });
                inner.push((self.verts.len() - 1) as u32);
                inner_uv.push(uv);
            }
            self.faces[fi].verts = inner.clone();
            self.faces[fi].uv = inner_uv;

            for k in 0..m {
                let k2 = (k + 1) % m;
                let quad_uv = vec![
                    src.uv[k],
                    src.uv[k2],
                    self.faces[fi].uv[k2],
                    self.faces[fi].uv[k],
                ];
                let f_new = Face::with_uv(
                    vec![src.verts[k], src.verts[k2], inner[k2], inner[k]],
                    quad_uv,
                );
                self.push_face(f_new);
            }
        }
        self.selected_edges.clear();
        self.sync_vert_selection_from_faces();
    }

    pub fn subdivide_selected(&mut self) {
        let sel: Vec<usize> = self
            .faces
            .iter()
            .enumerate()
            .filter(|(_, x)| x.selected)
            .map(|(i, _)| i)
            .collect();
        if sel.is_empty() {
            return;
        }
        let mut new_faces: Vec<Face> = Vec::new();
        let mut remove: Vec<usize> = Vec::new();
        for &fi in &sel {
            let src = self.faces[fi].clone();
            if src.verts.len() == 4 {
                let (a, b, c, d) = (src.verts[0], src.verts[1], src.verts[2], src.verts[3]);
                let (uva, uvb, uvc, uvd) = (src.uv[0], src.uv[1], src.uv[2], src.uv[3]);
                let mab = self.mid_vert(a, b);
                let mbc = self.mid_vert(b, c);
                let mcd = self.mid_vert(c, d);
                let mda = self.mid_vert(d, a);
                let ctr = self.mid_vert4(a, b, c, d);
                let muv = |x: [f32; 2], y: [f32; 2]| [(x[0] + y[0]) * 0.5, (x[1] + y[1]) * 0.5];
                let (mab_uv, mbc_uv, mcd_uv, mda_uv) =
                    (muv(uva, uvb), muv(uvb, uvc), muv(uvc, uvd), muv(uvd, uva));
                let ctr_uv = [
                    (uva[0] + uvb[0] + uvc[0] + uvd[0]) * 0.25,
                    (uva[1] + uvb[1] + uvc[1] + uvd[1]) * 0.25,
                ];
                new_faces.push(Face::with_uv(
                    vec![a, mab, ctr, mda],
                    vec![uva, mab_uv, ctr_uv, mda_uv],
                ));
                new_faces.push(Face::with_uv(
                    vec![mab, b, mbc, ctr],
                    vec![mab_uv, uvb, mbc_uv, ctr_uv],
                ));
                new_faces.push(Face::with_uv(
                    vec![ctr, mbc, c, mcd],
                    vec![ctr_uv, mbc_uv, uvc, mcd_uv],
                ));
                new_faces.push(Face::with_uv(
                    vec![mda, ctr, mcd, d],
                    vec![mda_uv, ctr_uv, mcd_uv, uvd],
                ));
                for nf in new_faces.iter_mut().rev().take(4) {
                    nf.selected = true;
                }
                remove.push(fi);
            } else if src.verts.len() == 3 {
                let (a, b, c) = (src.verts[0], src.verts[1], src.verts[2]);
                let (uva, uvb, uvc) = (src.uv[0], src.uv[1], src.uv[2]);
                let mab = self.mid_vert(a, b);
                let mbc = self.mid_vert(b, c);
                let mca = self.mid_vert(c, a);
                let muv = |x: [f32; 2], y: [f32; 2]| [(x[0] + y[0]) * 0.5, (x[1] + y[1]) * 0.5];
                let (mab_uv, mbc_uv, mca_uv) = (muv(uva, uvb), muv(uvb, uvc), muv(uvc, uva));
                for t in [
                    (vec![a, mab, mca], vec![uva, mab_uv, mca_uv]),
                    (vec![mab, b, mbc], vec![mab_uv, uvb, mbc_uv]),
                    (vec![mca, mbc, c], vec![mca_uv, mbc_uv, uvc]),
                    (vec![mab, mbc, mca], vec![mab_uv, mbc_uv, mca_uv]),
                ] {
                    let mut nf = Face::with_uv(t.0, t.1);
                    nf.selected = true;
                    new_faces.push(nf);
                }
                remove.push(fi);
            }
        }
        remove.sort_unstable_by(|a, b| b.cmp(a));
        for fi in remove {
            self.faces.remove(fi);
        }
        for nf in new_faces {
            self.push_face(nf);
        }
        self.sync_vert_selection_from_faces();
    }

    fn mid_vert(&mut self, a: u32, b: u32) -> u32 {
        let pa = self.verts[a as usize].vec();
        let pb = self.verts[b as usize].vec();
        let ca = self.verts[a as usize].color;
        let cb = self.verts[b as usize].color;
        self.verts.push(Vertex {
            pos: ((pa + pb) * 0.5).to_array(),
            color: [
                (ca[0] + cb[0]) * 0.5,
                (ca[1] + cb[1]) * 0.5,
                (ca[2] + cb[2]) * 0.5,
            ],
            selected: false,
        });
        (self.verts.len() - 1) as u32
    }

    fn mid_vert4(&mut self, a: u32, b: u32, c: u32, d: u32) -> u32 {
        let p = (self.verts[a as usize].vec()
            + self.verts[b as usize].vec()
            + self.verts[c as usize].vec()
            + self.verts[d as usize].vec())
            * 0.25;
        self.verts.push(Vertex {
            pos: p.to_array(),
            color: [0.75, 0.75, 0.78],
            selected: false,
        });
        (self.verts.len() - 1) as u32
    }

    /// Chanfra uma aresta convexa manifold com extremidades trivalentes (1 segmento padrão).
    pub fn bevel_selected(&mut self, amount: f32) -> (usize, usize) {
        self.bevel_selected_segments(amount, 1)
    }

    /// Chanfra uma aresta convexa manifold com suporte a multi-segmentos para filetagem arredondada.
    pub fn bevel_selected_segments(&mut self, amount: f32, segments: u32) -> (usize, usize) {
        let count = self.selected_edges.len();
        if count == 0 {
            return (0, 0);
        }
        if count != 1 || !amount.is_finite() || amount <= 0.0 {
            return (0, count);
        }
        let Some(&(a, b)) = self.selected_edges.iter().next() else {
            return (0, 0);
        };
        match crate::bevel::bevel_edge_segments(self, a, b, amount, segments) {
            Some(mesh) => {
                *self = mesh;
                (1, 0)
            }
            None => (0, count),
        }
    }

    /// Inverte a diagonal de triangulação interna de quads selecionados (rotacionando o fan de corte),
    /// ou executa um edge-flip em aresta selecionada compartilhada por dois triângulos adjacentes.
    pub fn flip_diagonal(&mut self) -> bool {
        let mut modified = false;

        // 1. Quads selecionados: rotação cíclica inverte a diagonal interna do fan
        for face in &mut self.faces {
            if face.selected && face.verts.len() == 4 {
                face.verts.rotate_left(1);
                if face.uv.len() == 4 {
                    face.uv.rotate_left(1);
                }
                modified = true;
            }
        }
        if modified {
            return true;
        }

        // 2. Aresta selecionada entre dois triângulos adjacentes (Delaunay Edge Flip)
        let edge = if let Some(&(u, v)) = self.selected_edges.iter().next() {
            Some((u, v))
        } else {
            let sel_verts: Vec<u32> = self
                .verts
                .iter()
                .enumerate()
                .filter(|(_, v)| v.selected)
                .map(|(i, _)| i as u32)
                .collect();
            if sel_verts.len() == 2 {
                Some((sel_verts[0], sel_verts[1]))
            } else {
                None
            }
        };

        if let Some((u, v)) = edge {
            let adj = self.edge_faces(u, v);
            if adj.len() == 2 {
                let f0 = &self.faces[adj[0]];
                let f1 = &self.faces[adj[1]];
                if f0.verts.len() == 3 && f1.verts.len() == 3 {
                    let w0 = f0.verts.iter().copied().find(|&x| x != u && x != v);
                    let w1 = f1.verts.iter().copied().find(|&x| x != u && x != v);
                    if let (Some(w0), Some(w1)) = (w0, w1)
                        && w0 != w1
                    {
                        let p_u = f0.verts.iter().position(|&x| x == u).unwrap();
                        let next_u = f0.verts[(p_u + 1) % 3];
                        let (new_f0, new_f1) = if next_u == v {
                            (vec![w0, u, w1], vec![w1, v, w0])
                        } else {
                            (vec![w0, w1, u], vec![w1, w0, v])
                        };

                        self.faces[adj[0]] = Face::new(new_f0);
                        self.faces[adj[1]] = Face::new(new_f1);
                        self.faces[adj[0]].selected = true;
                        self.faces[adj[1]].selected = true;
                        self.selected_edges.clear();
                        self.selected_edges.insert(crate::edge_key(w0, w1));
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Revoluciona a seleção de vértices/arestas em torno de um eixo (0: X, 1: Y, 2: Z)
    /// passando por `center` com `segments` subdivisões ao longo de `angle_deg` graus.
    pub fn revolve_selection(
        &mut self,
        segments: u32,
        angle_deg: f32,
        axis: usize,
        center: [f32; 3],
    ) -> bool {
        let sel_indices: Vec<u32> = self
            .verts
            .iter()
            .enumerate()
            .filter(|(_, v)| v.selected)
            .map(|(i, _)| i as u32)
            .collect();
        if sel_indices.len() < 2 {
            return false;
        }

        let seg = segments.clamp(3, 64) as usize;
        let total_rad = angle_deg.to_radians();
        let is_full_circle = (angle_deg.abs() - 360.0).abs() < 1e-3;
        let c = Vec3::from(center);
        let ax = axis.min(2);

        let rotate_pt = |pt: Vec3, rad: f32| -> Vec3 {
            let rel = pt - c;
            let (cos, sin) = (rad.cos(), rad.sin());
            let rot = match ax {
                0 => Vec3::new(rel.x, rel.y * cos - rel.z * sin, rel.y * sin + rel.z * cos),
                1 => Vec3::new(rel.x * cos + rel.z * sin, rel.y, -rel.x * sin + rel.z * cos),
                _ => Vec3::new(rel.x * cos - rel.y * sin, rel.x * sin + rel.y * cos, rel.z),
            };
            rot + c
        };

        let mut edges_to_revolve = Vec::new();
        for &(u, v) in &self.selected_edges {
            if sel_indices.contains(&u) && sel_indices.contains(&v) {
                edges_to_revolve.push((u, v));
            }
        }
        if edges_to_revolve.is_empty() {
            for i in 0..sel_indices.len() - 1 {
                edges_to_revolve.push((sel_indices[i], sel_indices[i + 1]));
            }
        }
        if edges_to_revolve.is_empty() {
            return false;
        }

        let steps = if is_full_circle { seg } else { seg + 1 };
        let mut rings: Vec<Vec<u32>> = Vec::with_capacity(sel_indices.len());

        for &vi in &sel_indices {
            let orig_pos = self.verts[vi as usize].vec();
            let mut ring = Vec::with_capacity(steps);
            ring.push(vi);
            for s in 1..steps {
                let frac = s as f32 / seg as f32;
                let angle = frac * total_rad;
                let new_pos = rotate_pt(orig_pos, angle);
                let mut v = self.verts[vi as usize].clone();
                v.pos = new_pos.to_array();
                v.selected = true;
                let new_idx = self.verts.len() as u32;
                self.verts.push(v);
                ring.push(new_idx);
            }
            rings.push(ring);
        }

        let vert_to_ring_idx: HashMap<u32, usize> = sel_indices
            .iter()
            .enumerate()
            .map(|(r_idx, &v_idx)| (v_idx, r_idx))
            .collect();

        for (u, v) in edges_to_revolve {
            let Some(&ring_u_idx) = vert_to_ring_idx.get(&u) else {
                continue;
            };
            let Some(&ring_v_idx) = vert_to_ring_idx.get(&v) else {
                continue;
            };
            let ring_u = &rings[ring_u_idx];
            let ring_v = &rings[ring_v_idx];

            for s in 0..seg {
                let u0 = ring_u[s];
                let u1 = if is_full_circle && s + 1 == seg {
                    ring_u[0]
                } else {
                    ring_u[s + 1]
                };
                let v0 = ring_v[s];
                let v1 = if is_full_circle && s + 1 == seg {
                    ring_v[0]
                } else {
                    ring_v[s + 1]
                };

                let mut face = Face::new(vec![u0, u1, v1, v0]);
                face.selected = true;
                self.push_face(face);
            }
        }

        self.selected_edges.clear();
        self.sync_vert_selection_from_faces();
        true
    }

    /// Mirror da seleção (ou tudo) no eixo, com weld opcional no plano.
    pub fn mirror(&mut self, axis: usize, weld_eps: f32) {
        let axis = axis.min(2);
        let any_sel = self.verts.iter().any(|v| v.selected);
        // coleta antes (não itera + empurra ao mesmo tempo)
        let src: Vec<(u32, Vertex)> = self
            .verts
            .iter()
            .enumerate()
            .filter(|(_, v)| v.selected || !any_sel)
            .map(|(i, v)| (i as u32, v.clone()))
            .collect();
        let mut remap: HashMap<u32, u32> = HashMap::new();
        for (i, v) in src {
            let mut nv = v;
            nv.pos[axis] = -nv.pos[axis];
            self.verts.push(nv);
            remap.insert(i, (self.verts.len() - 1) as u32);
        }
        let mut nf = Vec::new();
        for f in &self.faces {
            let all = f.verts.iter().all(|i| remap.contains_key(i));
            let any_face = any_sel && f.selected;
            if (any_sel && any_face) || (!any_sel && all) {
                let mut verts: Vec<u32> =
                    f.verts.iter().map(|i| *remap.get(i).unwrap_or(i)).collect();
                verts.reverse(); // espelho inverte winding
                let mut q = Face::with_uv(verts, f.uv.iter().rev().copied().collect());
                q.selected = true;
                nf.push(q);
            }
        }
        for f in nf {
            self.push_face(f);
        }
        if weld_eps > 0.0 {
            self.weld(weld_eps);
        }
        self.sync_vert_selection_from_faces();
    }

    /// Funde vértices duplicados dentro de `eps` (O(n²), ok p/ low-poly).
    /// Índices manuais são necessários (mutação durante iteração).
    #[allow(clippy::needless_range_loop)]
    pub fn weld(&mut self, eps: f32) {
        let n = self.verts.len();
        let mut rep: Vec<u32> = (0..n as u32).collect();
        for i in 0..n {
            if rep[i] != i as u32 {
                continue;
            }
            for j in (i + 1)..n {
                if rep[j] != j as u32 {
                    continue;
                }
                let d = (self.verts[i].vec() - self.verts[j].vec()).length();
                if d <= eps {
                    rep[j] = i as u32;
                    self.verts[i].selected |= self.verts[j].selected;
                }
            }
        }
        // compacta
        let mut new_id = vec![u32::MAX; n];
        let mut nv = Vec::new();
        for i in 0..n {
            if rep[i] == i as u32 {
                new_id[i] = nv.len() as u32;
                nv.push(self.verts[i].clone());
            }
        }
        for i in 0..n {
            if rep[i] != i as u32 {
                new_id[i] = new_id[rep[i] as usize];
            }
        }
        for f in &mut self.faces {
            for vi in &mut f.verts {
                *vi = new_id[*vi as usize];
            }
        }
        self.faces.retain(|f| {
            let mut u = f.verts.clone();
            u.sort_unstable();
            u.dedup();
            u.len() >= 3
        });
        self.verts = nv;
        // remapeia arestas selecionadas
        let edges: Vec<(u32, u32)> = self.selected_edges.iter().copied().collect();
        self.selected_edges.clear();
        for (a, b) in edges {
            let (na, nb) = (new_id[a as usize], new_id[b as usize]);
            if na != u32::MAX && nb != u32::MAX && na != nb {
                self.selected_edges.insert(edge_key(na, nb));
            }
        }
    }

    pub fn merge_center(&mut self) {
        let sel: Vec<u32> = self
            .verts
            .iter()
            .enumerate()
            .filter(|(_, v)| v.selected)
            .map(|(i, _)| i as u32)
            .collect();
        if sel.len() < 2 {
            return;
        }
        let mut c = Vec3::ZERO;
        for &i in &sel {
            c += self.verts[i as usize].vec();
        }
        c /= sel.len() as f32;
        let keep = sel[0] as usize;
        self.verts[keep].pos = c.to_array();
        self.verts[keep].selected = true;
        let keep_orig = sel[0] as usize;
        let dead_set: std::collections::HashSet<usize> =
            sel.iter().skip(1).map(|&i| i as usize).collect();
        let mut remap = vec![0u32; self.verts.len()];
        let mut new_verts = Vec::with_capacity(self.verts.len() - dead_set.len());

        for (i, v) in self.verts.iter().enumerate() {
            if dead_set.contains(&i) {
                continue;
            }
            let new_idx = new_verts.len() as u32;
            new_verts.push(v.clone());
            remap[i] = new_idx;
        }
        let keep_new = remap[keep_orig];
        for &d in &dead_set {
            remap[d] = keep_new;
        }

        self.verts = new_verts;
        for f in &mut self.faces {
            for vi in &mut f.verts {
                if let Some(&new_i) = remap.get(*vi as usize) {
                    *vi = new_i;
                }
            }
        }
        self.faces.retain(|f| {
            let mut uniq = f.verts.clone();
            uniq.sort_unstable();
            uniq.dedup();
            uniq.len() >= 3
        });
        for f in &mut self.faces {
            f.fix_uv();
        }
        self.selected_edges.clear();
    }

    pub fn delete_selected(&mut self) {
        if self.faces.iter().any(|f| f.selected) {
            self.faces = self.faces.iter().filter(|f| !f.selected).cloned().collect();
            let mut used = vec![false; self.verts.len()];
            for f in &self.faces {
                for &i in &f.verts {
                    used[i as usize] = true;
                }
            }
            let mut remap = vec![0u32; self.verts.len()];
            let mut nv = Vec::new();
            for (i, v) in self.verts.iter().enumerate() {
                if used[i] {
                    remap[i] = nv.len() as u32;
                    nv.push(v.clone());
                }
            }
            for f in &mut self.faces {
                for vi in &mut f.verts {
                    *vi = remap[*vi as usize];
                }
            }
            self.verts = nv;
        } else {
            let mut remap = vec![u32::MAX; self.verts.len()];
            let mut nv = Vec::new();
            for (i, v) in self.verts.iter().enumerate() {
                if !v.selected {
                    remap[i] = nv.len() as u32;
                    nv.push(v.clone());
                }
            }
            self.faces
                .retain(|f| f.verts.iter().all(|&i| remap[i as usize] != u32::MAX));
            for f in &mut self.faces {
                for vi in &mut f.verts {
                    *vi = remap[*vi as usize];
                }
            }
            self.verts = nv;
        }
        self.deselect_all();
    }

    pub fn duplicate_selected(&mut self) {
        let sel: Vec<(u32, Vertex)> = self
            .verts
            .iter()
            .enumerate()
            .filter(|(_, v)| v.selected)
            .map(|(i, v)| (i as u32, v.clone()))
            .collect();
        let mut remap: HashMap<u32, u32> = HashMap::new();
        for (i, v) in sel {
            let mut nv = v;
            nv.pos[0] += 0.3;
            self.verts.push(nv);
            remap.insert(i, (self.verts.len() - 1) as u32);
        }
        let mut nf = Vec::new();
        for f in &self.faces {
            if f.selected {
                let verts: Vec<u32> = f.verts.iter().map(|i| *remap.get(i).unwrap_or(i)).collect();
                let mut q = Face::with_uv(verts, f.uv.clone());
                q.selected = true;
                nf.push(q);
            }
        }
        for f in &mut self.faces {
            f.selected = false;
        }
        for f in nf {
            self.push_face(f);
        }
        self.sync_vert_selection_from_faces();
    }

    pub fn triangulate(&mut self) {
        let mut out = Vec::new();
        for f in &self.faces {
            let m = f.verts.len();
            if m > 3 {
                for i in 1..(m - 1) {
                    let mut t = Face::with_uv(
                        vec![f.verts[0], f.verts[i], f.verts[i + 1]],
                        vec![
                            f.uv.first().copied().unwrap_or([0.0, 0.0]),
                            f.uv.get(i).copied().unwrap_or([0.0, 0.0]),
                            f.uv.get(i + 1).copied().unwrap_or([0.0, 0.0]),
                        ],
                    );
                    t.selected = f.selected;
                    out.push(t);
                }
            } else {
                out.push(f.clone());
            }
        }
        self.faces = out;
    }

    // ---------------- novas operações geométricas ----------------

    /// Inverte o sentido de enrolamento (winding) e as normais das faces selecionadas
    /// (ou de todas as faces se nenhuma estiver selecionada).
    pub fn flip_normals(&mut self) {
        let any_sel = self.faces.iter().any(|f| f.selected);
        for f in &mut self.faces {
            if f.selected || !any_sel {
                f.verts.reverse();
                f.uv.reverse();
            }
        }
    }

    /// Recalcula e unifica a orientação das faces em componentes conexos
    /// para que as normais apontem consistentemente para fora (volume positivo).
    pub fn recalculate_normals(&mut self) {
        if self.faces.is_empty() {
            return;
        }

        // 1. Mapeia arestas direcionadas para faces
        let mut edge_to_face: HashMap<(u32, u32), usize> = HashMap::new();
        for (fi, f) in self.faces.iter().enumerate() {
            let m = f.verts.len();
            for k in 0..m {
                let u = f.verts[k];
                let v = f.verts[(k + 1) % m];
                edge_to_face.insert((u, v), fi);
            }
        }

        // 2. BFS para unificar orientação por componente
        let mut visited = vec![false; self.faces.len()];
        for start_fi in 0..self.faces.len() {
            if visited[start_fi] {
                continue;
            }

            let mut component = Vec::new();
            let mut queue = std::collections::VecDeque::new();
            visited[start_fi] = true;
            queue.push_back(start_fi);
            component.push(start_fi);

            while let Some(fi) = queue.pop_front() {
                let m = self.faces[fi].verts.len();
                for k in 0..m {
                    let u = self.faces[fi].verts[k];
                    let v = self.faces[fi].verts[(k + 1) % m];

                    // Face vizinha orientada no sentido correto deve ter aresta (v, u)
                    if let Some(&adj_fi) = edge_to_face.get(&(v, u)) {
                        if !visited[adj_fi] {
                            visited[adj_fi] = true;
                            queue.push_back(adj_fi);
                            component.push(adj_fi);
                        }
                    } else if let Some(&adj_fi) = edge_to_face.get(&(u, v)) {
                        // Face vizinha está com orientação invertida: precisa inverter
                        if !visited[adj_fi] {
                            visited[adj_fi] = true;
                            self.faces[adj_fi].verts.reverse();
                            self.faces[adj_fi].uv.reverse();
                            queue.push_back(adj_fi);
                            component.push(adj_fi);
                        }
                    }
                }
            }

            // 3. Testa se o volume do componente é negativo (dentro para fora)
            let mut signed_volume = 0.0f32;
            for &fi in &component {
                let f = &self.faces[fi];
                if f.verts.len() >= 3 {
                    let p0 = self.verts[f.verts[0] as usize].vec();
                    for k in 1..(f.verts.len() - 1) {
                        let p1 = self.verts[f.verts[k] as usize].vec();
                        let p2 = self.verts[f.verts[k + 1] as usize].vec();
                        signed_volume += p0.dot(p1.cross(p2));
                    }
                }
            }

            // Se o volume assinado for negativo, inverte todas as faces do componente
            if signed_volume < -1e-5 {
                for &fi in &component {
                    self.faces[fi].verts.reverse();
                    self.faces[fi].uv.reverse();
                }
            }
        }
    }

    /// Dissolve elementos selecionados (arestas compartilhadas ou vértices redundantes)
    /// sem deixar buracos na geometria.
    pub fn dissolve_selected(&mut self) {
        // 1. Dissolve iterativo de arestas selecionadas compartilhadas por exatamente 2 faces
        loop {
            let mut dissolved_any = false;
            let edges_to_dissolve: Vec<(u32, u32)> = self.selected_edges.iter().copied().collect();
            for (a, b) in edges_to_dissolve {
                let adj = self.edge_faces(a, b);
                if adj.len() == 2 {
                    let (f1_idx, f2_idx) = (adj[0], adj[1]);
                    let f1 = &self.faces[f1_idx];
                    let f2 = &self.faces[f2_idx];

                    // Encontra posições dos vértices nas duas faces
                    let p1_a = f1.verts.iter().position(|&x| x == a);
                    let p1_b = f1.verts.iter().position(|&x| x == b);
                    let p2_a = f2.verts.iter().position(|&x| x == a);
                    let p2_b = f2.verts.iter().position(|&x| x == b);

                    if let (Some(i1_a), Some(i1_b), Some(i2_a), Some(i2_b)) =
                        (p1_a, p1_b, p2_a, p2_b)
                    {
                        // Constrói polígono fundido
                        let mut merged_verts = Vec::new();
                        let mut merged_uvs = Vec::new();

                        let m1 = f1.verts.len();
                        let m2 = f2.verts.len();

                        // Se f1 vai de a -> b e f2 vai de b -> a
                        let f1_forward = (i1_a + 1) % m1 == i1_b;
                        let start1 = if f1_forward { i1_b } else { i1_a };

                        let mut k = start1;
                        loop {
                            merged_verts.push(f1.verts[k]);
                            merged_uvs.push(f1.uv[k]);
                            k = (k + 1) % m1;
                            if (f1_forward && k == i1_a) || (!f1_forward && k == i1_b) {
                                merged_verts.push(f1.verts[k]);
                                merged_uvs.push(f1.uv[k]);
                                break;
                            }
                        }

                        let start2 = if f1_forward {
                            (i2_a + 1) % m2
                        } else {
                            (i2_b + 1) % m2
                        };
                        let end2 = if f1_forward { i2_b } else { i2_a };
                        let mut k2 = start2;
                        while k2 != end2 {
                            merged_verts.push(f2.verts[k2]);
                            merged_uvs.push(f2.uv[k2]);
                            k2 = (k2 + 1) % m2;
                        }

                        // Remove vértices consecutivos duplicados
                        let mut clean_verts = Vec::new();
                        let mut clean_uvs = Vec::new();
                        for (idx, &v) in merged_verts.iter().enumerate() {
                            if clean_verts.last() != Some(&v) {
                                clean_verts.push(v);
                                clean_uvs.push(merged_uvs[idx]);
                            }
                        }
                        if clean_verts.len() > 1 && clean_verts.first() == clean_verts.last() {
                            clean_verts.pop();
                            clean_uvs.pop();
                        }

                        if clean_verts.len() >= 3 {
                            self.faces[f1_idx].verts = clean_verts;
                            self.faces[f1_idx].uv = clean_uvs;
                            self.faces[f1_idx].selected = true;
                            self.faces.remove(f2_idx);
                            self.selected_edges.remove(&(a.min(b), a.max(b)));
                            dissolved_any = true;
                            break;
                        }
                    }
                }
            }
            if !dissolved_any {
                break;
            }
        }
        self.remove_isolated_vertices();
        self.selected_edges.clear();
    }

    /// Fatiamento planar (Slice Plane): bissecciona a geometria ao longo de um plano definido por
    /// ponto e normal, inserindo vértices de corte e dividindo faces interpolando posições e UVs.
    /// Com `fill_cap`, mantém apenas o lado positivo da normal e fecha o corte.
    /// Sem tampa, preserva ambos os lados e apenas subdivide a superfície.
    pub fn slice_plane(&mut self, plane_point: Vec3, plane_normal: Vec3, fill_cap: bool) {
        if !plane_point.is_finite() || !plane_normal.is_finite() {
            return;
        }
        let n = plane_normal.normalize_or_zero();
        if n.length_squared() < 1e-4 {
            return;
        }
        if fill_cap {
            let distances = || self.verts.iter().map(|v| (v.vec() - plane_point).dot(n));
            if distances().any(|d| d < -1e-5) && !distances().any(|d| d > 1e-5) {
                *self = Self::default();
                return;
            }
        }

        let mut new_faces: Vec<Face> = Vec::new();
        let mut original_face_indices_to_remove: Vec<usize> = Vec::new();
        let mut cut_edge_cache: std::collections::HashMap<(u32, u32), u32> =
            std::collections::HashMap::new();

        for (fi, face) in self.faces.iter().enumerate() {
            let m = face.verts.len();
            if m < 3 {
                continue;
            }

            // Distância assinada de cada vértice ao plano
            let mut dists = Vec::with_capacity(m);
            for &vi in &face.verts {
                let p = self.verts[vi as usize].vec();
                let d = (p - plane_point).dot(n);
                dists.push(d);
            }

            let eps = 1e-5;
            let all_pos = dists.iter().all(|&d| d >= -eps);
            let all_neg = dists.iter().all(|&d| d <= eps);

            // Se todos os vértices estão no mesmo lado do plano, mantém a face
            if all_pos {
                continue;
            }
            if all_neg {
                if fill_cap {
                    original_face_indices_to_remove.push(fi);
                }
                continue;
            }

            // A face cruza o plano: divide o polígono
            original_face_indices_to_remove.push(fi);

            let mut pos_poly: Vec<(u32, [f32; 2])> = Vec::new();
            let mut neg_poly: Vec<(u32, [f32; 2])> = Vec::new();

            for k in 0..m {
                let k2 = (k + 1) % m;
                let v1 = face.verts[k];
                let v2 = face.verts[k2];
                let uv1 = face.uv[k];
                let uv2 = face.uv[k2];
                let d1 = dists[k];
                let d2 = dists[k2];

                if d1 >= -eps {
                    pos_poly.push((v1, uv1));
                }
                if d1 <= eps {
                    neg_poly.push((v1, uv1));
                }

                // Aresta cruza o plano: interpola ou reutiliza ponto de corte compartilhado
                if (d1 > eps && d2 < -eps) || (d1 < -eps && d2 > eps) {
                    let e_key = edge_key(v1, v2);
                    let t = (d1 / (d1 - d2)).clamp(0.0, 1.0);
                    let uv_cut = [
                        uv1[0] + (uv2[0] - uv1[0]) * t,
                        uv1[1] + (uv2[1] - uv1[1]) * t,
                    ];

                    let cut_idx = if let Some(&existing) = cut_edge_cache.get(&e_key) {
                        existing
                    } else {
                        let p1 = self.verts[v1 as usize].vec();
                        let p2 = self.verts[v2 as usize].vec();
                        let p_cut = p1.lerp(p2, t);
                        let col1 = self.verts[v1 as usize].color;
                        let col2 = self.verts[v2 as usize].color;
                        let col_cut = [
                            col1[0] + (col2[0] - col1[0]) * t,
                            col1[1] + (col2[1] - col1[1]) * t,
                            col1[2] + (col2[2] - col1[2]) * t,
                        ];

                        self.verts.push(Vertex {
                            pos: p_cut.to_array(),
                            color: col_cut,
                            selected: false,
                        });
                        let idx = (self.verts.len() - 1) as u32;
                        cut_edge_cache.insert(e_key, idx);
                        idx
                    };

                    pos_poly.push((cut_idx, uv_cut));
                    neg_poly.push((cut_idx, uv_cut));
                }
            }

            if pos_poly.len() >= 3 {
                let verts = pos_poly.iter().map(|(v, _)| *v).collect();
                let uvs = pos_poly.iter().map(|(_, uv)| *uv).collect();
                new_faces.push(Face::with_uv(verts, uvs));
            }
            if !fill_cap && neg_poly.len() >= 3 {
                let verts = neg_poly.iter().map(|(v, _)| *v).collect();
                let uvs = neg_poly.iter().map(|(_, uv)| *uv).collect();
                new_faces.push(Face::with_uv(verts, uvs));
            }
        }

        // Remove faces originais que foram fatiadas
        original_face_indices_to_remove.sort_unstable_by(|a, b| b.cmp(a));
        for fi in original_face_indices_to_remove {
            self.faces.remove(fi);
        }

        // Adiciona sub-faces fatiadas
        for f in new_faces {
            self.push_face(f);
        }

        // Deriva o contorno do resultado, incluindo vértices que já estavam no plano.
        // Apenas arestas incidentes a uma face precisam de tampa; arestas internas
        // ou faces coplanares preservadas não recebem geometria sobreposta.
        if fill_cap {
            let mut edge_uses: HashMap<(u32, u32), (usize, (u32, u32))> = HashMap::new();
            for face in &self.faces {
                for k in 0..face.verts.len() {
                    let a = face.verts[k];
                    let b = face.verts[(k + 1) % face.verts.len()];
                    let entry = edge_uses.entry(edge_key(a, b)).or_insert((0, (a, b)));
                    entry.0 += 1;
                }
            }
            let mut remaining_edges: Vec<(u32, u32)> = edge_uses
                .values()
                .filter(|(count, (a, b))| {
                    *count == 1
                        && [a, b].iter().all(|&&vi| {
                            (self.verts[vi as usize].vec() - plane_point).dot(n).abs() <= 1e-5
                        })
                })
                .map(|(_, edge)| *edge)
                .collect();
            remaining_edges.sort_unstable();
            while !remaining_edges.is_empty() {
                let first = remaining_edges.remove(0);
                let mut loop_verts = vec![first.0, first.1];
                let mut current = first.1;
                let mut closed = false;

                loop {
                    let mut found = None;
                    for (i, &(u, v)) in remaining_edges.iter().enumerate() {
                        if u == current {
                            found = Some((i, v));
                            break;
                        } else if v == current {
                            found = Some((i, u));
                            break;
                        }
                    }
                    if let Some((i, next_v)) = found {
                        remaining_edges.remove(i);
                        if next_v == first.0 {
                            closed = true;
                            break;
                        }
                        loop_verts.push(next_v);
                        current = next_v;
                    } else {
                        break;
                    }
                }

                if closed && loop_verts.len() >= 3 {
                    let norm = plane_normal.normalize_or_zero();
                    let tangent = if norm.x.abs() < 0.9 {
                        Vec3::X.cross(norm).normalize_or_zero()
                    } else {
                        Vec3::Y.cross(norm).normalize_or_zero()
                    };
                    let bitangent = norm.cross(tangent).normalize_or_zero();

                    let cap_uvs: Vec<[f32; 2]> = loop_verts
                        .iter()
                        .map(|&vi| {
                            let p = self.verts[vi as usize].vec() - plane_point;
                            [p.dot(tangent) * 0.5 + 0.5, p.dot(bitangent) * 0.5 + 0.5]
                        })
                        .collect();

                    let mut cap = Face::with_uv(loop_verts, cap_uvs);
                    let c_norm = triangulate::face_normal_of(&self.verts, &cap.verts);
                    if c_norm.dot(norm) > 0.0 {
                        cap.verts.reverse();
                        cap.uv.reverse();
                    }
                    cap.selected = true;
                    self.push_face(cap);
                }
            }
            self.selected_edges.clear();
            self.remove_isolated_vertices();
            self.sync_vert_selection_from_faces();
        }
    }

    /// Extrusão por varredura (Sweep): estruza um perfil 2D ao longo de um caminho 3D (path),
    /// calculando referenciais locais (Rotation-Minimizing Frames) para evitar torções inesperadas.
    pub fn sweep(
        &mut self,
        profile: &[[f32; 2]],
        path: &[Vec3],
        cap_ends: bool,
    ) -> Result<(), String> {
        if profile.len() < 3 {
            return Err("Perfil deve conter pelo menos 3 pontos".into());
        }
        if path.len() < 2 {
            return Err("Caminho deve conter pelo menos 2 nós".into());
        }

        let m = profile.len();
        let num_sections = path.len();
        let mut section_vert_indices: Vec<Vec<u32>> = Vec::with_capacity(num_sections);

        // Frame inicial
        let t0 = (path[1] - path[0]).normalize_or_zero();
        let up = if t0.dot(Vec3::Y).abs() < 0.99 {
            Vec3::Y
        } else {
            Vec3::Z
        };
        let mut n_curr = t0.cross(up).normalize_or_zero();
        let mut b_curr = t0.cross(n_curr).normalize_or_zero();

        for i in 0..num_sections {
            let p_curr = path[i];
            let t_curr = if i + 1 < num_sections {
                (path[i + 1] - path[i]).normalize_or_zero()
            } else {
                (path[i] - path[i - 1]).normalize_or_zero()
            };

            // Transporte paralelo simples
            if i > 0 {
                let t_prev = (path[i] - path[i - 1]).normalize_or_zero();
                let axis = t_prev.cross(t_curr);
                let angle = t_prev.dot(t_curr).clamp(-1.0, 1.0).acos();
                if axis.length_squared() > 1e-6 && angle.abs() > 1e-5 {
                    let rot = glam::Quat::from_axis_angle(axis.normalize(), angle);
                    n_curr = rot.mul_vec3(n_curr).normalize_or_zero();
                    b_curr = rot.mul_vec3(b_curr).normalize_or_zero();
                }
            }

            let mut ring = Vec::with_capacity(m);
            for (k, pt) in profile.iter().enumerate() {
                let pos = p_curr + n_curr * pt[0] + b_curr * pt[1];
                let color = [0.75, 0.75, 0.78];
                self.verts.push(Vertex {
                    pos: pos.to_array(),
                    color,
                    selected: true,
                });
                ring.push((self.verts.len() - 1) as u32);
                let _ = k;
            }
            section_vert_indices.push(ring);
        }

        // Conecta seções consecutivas em anéis de quads
        for i in 0..(num_sections - 1) {
            let r1 = &section_vert_indices[i];
            let r2 = &section_vert_indices[i + 1];
            let u_start = i as f32 / (num_sections - 1) as f32;
            let u_end = (i + 1) as f32 / (num_sections - 1) as f32;

            for k in 0..m {
                let k2 = (k + 1) % m;
                let v0 = r1[k];
                let v1 = r1[k2];
                let v2 = r2[k2];
                let v3 = r2[k];

                let v_start = k as f32 / m as f32;
                let v_end = (k + 1) as f32 / m as f32;

                let quad = Face::with_uv(
                    vec![v0, v1, v2, v3],
                    vec![
                        [u_start, v_start],
                        [u_start, v_end],
                        [u_end, v_end],
                        [u_end, v_start],
                    ],
                );
                self.push_face(quad);
            }
        }

        // Tampas inicial e final
        if cap_ends {
            let start_ring = section_vert_indices[0].clone();
            let mut start_rev = start_ring;
            start_rev.reverse();
            let start_cap = Face::new(start_rev);
            self.push_face(start_cap);

            let end_cap = Face::new(section_vert_indices[num_sections - 1].clone());
            self.push_face(end_cap);
        }

        Ok(())
    }

    // ---------------- operações de combinação ----------------

    /// Combina outra malha nesta, mantendo ilhas disjuntas em um único objeto.
    pub fn join(&mut self, other: &Mesh) {
        let base_idx = self.verts.len() as u32;
        self.verts.extend(other.verts.iter().cloned());

        for f in &other.faces {
            let remapped_verts: Vec<u32> = f.verts.iter().map(|&i| i + base_idx).collect();
            let mut nf = Face::with_uv(remapped_verts, f.uv.clone());
            nf.selected = f.selected;
            self.push_face(nf);
        }

        for &(a, b) in &other.selected_edges {
            self.selected_edges.insert((a + base_idx, b + base_idx));
        }
    }

    /// Conecta (bridge) duas faces criando uma faixa perimétrica de quads entre elas
    /// e removendo as faces originais para produzir um tubo/ponte aberto e contínuo.
    pub fn connect_loops(&mut self, face_a: usize, face_b: usize) -> Result<(), String> {
        if face_a >= self.faces.len() || face_b >= self.faces.len() {
            return Err("Índice de face inválido".into());
        }
        if face_a == face_b {
            return Err("Não é possível conectar uma face a si mesma".into());
        }

        let m = self.faces[face_a].verts.len();
        if m != self.faces[face_b].verts.len() {
            return Err(format!(
                "Faces incompatíveis para conexão: face A possui {} vértices e face B possui {}",
                m,
                self.faces[face_b].verts.len()
            ));
        }

        let verts_a = self.faces[face_a].verts.clone();
        let verts_b = self.faces[face_b].verts.clone();

        // Encontra melhor alinhamento testando ambas orientações (direta e invertida)
        // e todos os deslocamentos cíclicos para evitar torções (bowtie)
        let mut best_shift = 0;
        let mut best_reversed = false;
        let mut min_dist_sq = f32::MAX;

        for reversed in [false, true] {
            for shift in 0..m {
                let mut dist_sq = 0.0f32;
                for (k, &va) in verts_a.iter().enumerate() {
                    let pa = self.verts[va as usize].vec();
                    let kb = if !reversed {
                        (k + shift) % m
                    } else {
                        (m + shift - (k % m)) % m
                    };
                    let pb = self.verts[verts_b[kb] as usize].vec();
                    dist_sq += (pa - pb).length_squared();
                }
                if dist_sq < min_dist_sq {
                    min_dist_sq = dist_sq;
                    best_shift = shift;
                    best_reversed = reversed;
                }
            }
        }

        // Gera os quads de conexão
        for k in 0..m {
            let k2 = (k + 1) % m;
            let va1 = verts_a[k];
            let va2 = verts_a[k2];
            let kb1 = if !best_reversed {
                (k + best_shift) % m
            } else {
                (m + best_shift - (k % m)) % m
            };
            let kb2 = if !best_reversed {
                (k2 + best_shift) % m
            } else {
                (m + best_shift - (k2 % m)) % m
            };
            let vb1 = verts_b[kb1];
            let vb2 = verts_b[kb2];

            let mut quad = Face::new(vec![va1, va2, vb2, vb1]);
            quad.selected = true;
            self.push_face(quad);
        }

        // Remove as duas faces originais para abrir a passagem
        let first = face_a.max(face_b);
        let second = face_a.min(face_b);
        self.faces.remove(first);
        self.faces.remove(second);

        Ok(())
    }

    /// Funde outra malha com esta, soldando vértices coincidentes dentro do raio `eps`.
    pub fn fuse_approximate(&mut self, other: &Mesh, eps: f32) {
        self.join(other);
        self.weld(eps);
    }
}

#[cfg(test)]
mod region_tests {
    use super::*;

    fn assert_closed(mesh: &Mesh) {
        let report = mesh.validate_topology();
        assert!(report.is_manifold, "{report:?}");
        assert!(report.is_closed, "{report:?}");
        assert_eq!(report.isolated_vertex_count, 0);
        assert!(mesh.faces.iter().all(|f| f.uv.len() == f.verts.len()));
        assert!(mesh.verts.iter().all(|v| v.vec().is_finite()));
    }

    #[test]
    fn extrude_cube_face_replaces_base_and_preserves_closed_surface() {
        let mut mesh = Mesh::cube(2.0);
        mesh.faces[0].selected = true;
        mesh.extrude_selected(0.5);
        assert_closed(&mesh);
        assert_eq!((mesh.verts.len(), mesh.faces.len()), (12, 10));
        assert_eq!(mesh.faces.iter().filter(|f| f.selected).count(), 1);
        assert!(
            mesh.faces[0]
                .verts
                .iter()
                .all(|&i| mesh.verts[i as usize].pos[2] == -1.5)
        );
    }

    #[test]
    fn extrude_adjacent_faces_has_shared_cap_and_only_boundary_walls() {
        let mut mesh = Mesh::cube(2.0);
        mesh.faces[0].selected = true;
        mesh.faces[2].selected = true;
        mesh.extrude_selected(0.5);
        assert_closed(&mesh);
        assert_eq!((mesh.verts.len(), mesh.faces.len()), (14, 12));
        assert_eq!(
            mesh.faces[0]
                .verts
                .iter()
                .filter(|vi| mesh.faces[2].verts.contains(vi))
                .count(),
            2
        );
    }

    #[test]
    fn extrude_large_region_removes_old_interior_vertices() {
        let mut mesh = Mesh::cube(2.0);
        mesh.select_all();
        mesh.faces[1].selected = false;
        mesh.extrude_selected(0.2);
        assert_closed(&mesh);
        assert_eq!(mesh.verts.len(), 12);
    }

    #[test]
    fn extrude_negative_distance_remains_closed() {
        let mut mesh = Mesh::cube(2.0);
        mesh.faces[0].selected = true;
        mesh.extrude_selected(-0.25);
        assert_closed(&mesh);
        assert!(
            mesh.faces[0]
                .verts
                .iter()
                .all(|&i| mesh.verts[i as usize].pos[2] == -0.75)
        );
    }

    #[test]
    fn extrude_zero_prepares_selected_cap_for_modal_displacement() {
        let mut mesh = Mesh::cube(2.0);
        mesh.faces[0].selected = true;
        mesh.extrude_selected(0.0);
        for v in mesh.verts.iter_mut().filter(|v| v.selected) {
            v.pos[2] -= 0.5;
        }
        assert_closed(&mesh);
        assert_eq!(mesh.verts.iter().filter(|v| v.selected).count(), 4);
    }

    #[test]
    fn nonfinite_extrusion_and_invalid_inset_do_not_mutate() {
        for value in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let mut mesh = Mesh::cube(2.0);
            mesh.faces[0].selected = true;
            let before = format!("{mesh:?}");
            mesh.extrude_selected(value);
            assert_eq!(format!("{mesh:?}"), before);
            mesh.inset_selected(value);
            assert_eq!(format!("{mesh:?}"), before);
        }
        let mut mesh = Mesh::cube(2.0);
        mesh.faces[0].selected = true;
        let before = format!("{mesh:?}");
        mesh.inset_selected(0.0);
        mesh.inset_selected(-1.0);
        assert_eq!(format!("{mesh:?}"), before);
    }

    #[test]
    fn inset_cube_face_is_closed_even_at_clamped_maximum() {
        let mut mesh = Mesh::cube(2.0);
        mesh.faces[0].selected = true;
        mesh.inset_selected(1e20);
        assert_closed(&mesh);
        assert_eq!((mesh.verts.len(), mesh.faces.len()), (12, 10));
    }

    #[test]
    fn capped_slice_retains_positive_side_and_outward_cap() {
        let mut mesh = Mesh::cube(2.0);
        mesh.slice_plane(Vec3::ZERO, Vec3::Y, true);
        assert_closed(&mesh);
        assert!(mesh.verts.iter().all(|v| v.pos[1] >= 0.0));
        let cap = mesh.faces.iter().position(|f| f.selected).expect("cut cap");
        assert!(mesh.face_normal(cap).dot(-Vec3::Y) > 0.99);
        assert_eq!((mesh.verts.len(), mesh.faces.len()), (8, 6));
    }

    #[test]
    fn capped_slice_through_existing_vertices_is_welded() {
        for normal in [Vec3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 1.0, 1.0)] {
            let mut mesh = Mesh::cube(2.0);
            let point = if normal.z == 0.0 {
                Vec3::ZERO
            } else {
                Vec3::new(1.0, 0.0, 0.0)
            };
            mesh.slice_plane(point, normal, true);
            assert_closed(&mesh);
            assert!(
                mesh.verts
                    .iter()
                    .all(|v| (v.vec() - point).dot(normal) >= -1e-5)
            );
        }
    }

    #[test]
    fn tangent_slice_does_not_duplicate_existing_face() {
        let mut mesh = Mesh::cube(2.0);
        mesh.slice_plane(Vec3::new(0.0, -1.0, 0.0), Vec3::Y, true);
        assert_closed(&mesh);
        assert_eq!((mesh.verts.len(), mesh.faces.len()), (8, 6));

        mesh.slice_plane(Vec3::new(0.0, 1.0, 0.0), Vec3::Y, true);
        assert!(mesh.faces.is_empty());
        assert!(mesh.verts.is_empty());
    }

    #[test]
    fn uncapped_slice_subdivides_both_sides_without_internal_face() {
        let mut mesh = Mesh::cube(2.0);
        mesh.slice_plane(Vec3::ZERO, Vec3::Y, false);
        assert_closed(&mesh);
        assert_eq!((mesh.verts.len(), mesh.faces.len()), (12, 10));
        assert!(mesh.verts.iter().any(|v| v.pos[1] < 0.0));
        assert!(mesh.verts.iter().any(|v| v.pos[1] > 0.0));
    }
}
