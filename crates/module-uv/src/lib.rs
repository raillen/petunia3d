//! module-uv — workspace UV (§12): projeção planar, seleção sincronizada
//! via eventos e transformações simples de ilhas (mover/escalar).

use petunia_core::{AppEvent, AppState, Module};

#[derive(Default)]
pub struct UvModule {
    /// faces com UV "suja" (precisam re-projeção) — dirty flag simples.
    pub dirty: bool,
}

impl UvModule {
    pub fn new() -> Self {
        Self { dirty: false }
    }

    /// Re-projeta planar o asset ativo (checkpoint + evento).
    pub fn reproject(state: &mut AppState) {
        state.checkpoint("uv planar");
        if let Some(m) = state.project.active_mesh_mut() {
            m.project_planar();
        }
        state.uv_selected.clear();
        state.emit_mesh_changed();
    }

    /// Move UVs das faces selecionadas (ou todas se vazio).
    pub fn move_selected(state: &mut AppState, du: f32, dv: f32) {
        let sel_empty = state.session.uv_selected.is_empty();
        let uv_selected = &state.session.uv_selected;
        if let Some(m) = state.project.active_mesh_mut() {
            for (fi, f) in m.faces.iter_mut().enumerate() {
                if sel_empty || uv_selected.contains(&fi) {
                    for uv in &mut f.uv {
                        uv[0] += du;
                        uv[1] += dv;
                    }
                }
            }
            state.mark_dirty();
        }
    }

    /// Escala UVs selecionadas em torno do centroide.
    pub fn scale_selected(state: &mut AppState, s: f32) {
        let sel_empty = state.session.uv_selected.is_empty();
        let uv_selected = &state.session.uv_selected;
        if let Some(m) = state.project.active_mesh_mut() {
            let mut c = [0.0f32; 2];
            let mut n = 0;
            for (fi, f) in m.faces.iter().enumerate() {
                if sel_empty || uv_selected.contains(&fi) {
                    for uv in &f.uv {
                        c[0] += uv[0];
                        c[1] += uv[1];
                        n += 1;
                    }
                }
            }
            if n == 0 {
                return;
            }
            c[0] /= n as f32;
            c[1] /= n as f32;
            for (fi, f) in m.faces.iter_mut().enumerate() {
                if sel_empty || uv_selected.contains(&fi) {
                    for uv in &mut f.uv {
                        uv[0] = c[0] + (uv[0] - c[0]) * s;
                        uv[1] = c[1] + (uv[1] - c[1]) * s;
                    }
                }
            }
            state.mark_dirty();
        }
    }
}

impl Module for UvModule {
    fn id(&self) -> &'static str {
        "uv"
    }
    fn on_event(&mut self, event: &AppEvent, state: &mut AppState) {
        match event {
            AppEvent::MeshChanged { .. } | AppEvent::ActiveAssetChanged { .. } => {
                // seleção UV pode ter morrido com a malha — limpa órfãs
                if let Some(o) = state.project.assets.get(state.project.active) {
                    let max_len = o.mesh.faces.len();
                    state.session.uv_selected.retain(|&fi| fi < max_len);
                }
                self.dirty = true;
            }
            AppEvent::SelectionChanged(sel) => {
                // espelha seleção de faces do viewport no editor UV
                state.uv_selected = sel.faces.iter().copied().collect();
            }
            _ => {}
        }
    }

    fn as_any(&self) -> &(dyn std::any::Any + 'static) {
        self
    }

    fn as_any_mut(&mut self) -> &mut (dyn std::any::Any + 'static) {
        self
    }
}

impl UvModule {
    pub fn uv_hit(state: &AppState, u: f32, v: f32) -> Option<usize> {
        uv_hit(state, u, v)
    }
}

/// Face cuja ilha contém (u,v). Testa tris do polígono UV.
pub fn uv_hit(state: &AppState, u: f32, v: f32) -> Option<usize> {
    let o = state.project.assets.get(state.project.active)?;
    for (fi, f) in o.mesh.faces.iter().enumerate() {
        if f.uv.len() < 3 {
            continue;
        }
        // fan a partir do vértice 0
        for k in 1..f.uv.len() - 1 {
            if point_in_tri_uv([u, v], f.uv[0], f.uv[k], f.uv[k + 1]) {
                return Some(fi);
            }
        }
    }
    None
}

fn point_in_tri_uv(p: [f32; 2], a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> bool {
    let v0 = [c[0] - a[0], c[1] - a[1]];
    let v1 = [b[0] - a[0], b[1] - a[1]];
    let v2 = [p[0] - a[0], p[1] - a[1]];
    let (d00, d01, d11, d20, d21) = (
        v0[0] * v0[0] + v0[1] * v0[1],
        v0[0] * v1[0] + v0[1] * v1[1],
        v1[0] * v1[0] + v1[1] * v1[1],
        v2[0] * v0[0] + v2[1] * v0[1],
        v2[0] * v1[0] + v2[1] * v1[1],
    );
    let den = d00 * d11 - d01 * d01;
    if den.abs() < 1e-12 {
        return false;
    }
    let v = (d11 * d20 - d01 * d21) / den;
    let w = (d00 * d21 - d01 * d20) / den;
    v >= -1e-6 && w >= -1e-6 && v + w <= 1.0 + 1e-6
}
