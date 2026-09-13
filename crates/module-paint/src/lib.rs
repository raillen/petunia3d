//! module-paint — workspace PAINT (§11): vertex paint + canvas 2D.
//! Brush/soft/fill/eyedropper operam em vértices; o canvas alimenta o
//! preview texturizado (albedo). Layers: baseline futuro — V1 usa 1 layer.

use petunia_core::{AppState, Module};

#[derive(Default)]
pub struct PaintModule;

impl PaintModule {
    pub fn new() -> Self {
        Self
    }

    /// Preenche seleção (ou tudo) com a cor atual. Retorna nº de verts.
    pub fn fill_selection(state: &mut AppState) -> usize {
        let before = state.project.clone();
        let col = state.paint_color;
        let mut n = 0;
        if let Some(m) = state.project.active_mesh_mut() {
            let any = m.verts.iter().any(|v| v.selected);
            for v in &mut m.verts {
                if v.selected || !any {
                    v.color = col;
                    n += 1;
                }
            }
        }
        if n > 0 {
            state.project.undo.checkpoint("fill", &before);
            state.emit_mesh_changed();
        }
        n
    }

    /// Eyedropper: copia a cor do vértice para o pincel.
    pub fn eyedrop_vertex(state: &mut AppState, vi: usize) {
        if let Some(o) = state.project.assets.get(state.project.active) {
            if let Some(v) = o.mesh.verts.get(vi) {
                state.paint_color = v.color;
                state.mark_dirty();
            }
        }
    }

    /// Registra cor na palette (recentes, máx 16) no ProjectState.
    pub fn push_palette(state: &mut AppState, c: [f32; 3]) {
        state
            .project
            .palette
            .retain(|&x| (x[0] - c[0]).abs() + (x[1] - c[1]).abs() + (x[2] - c[2]).abs() > 1e-3);
        state.project.palette.insert(0, c);
        state.project.palette.truncate(16);
    }

    /// Substitui a paleta atual no ProjectState.
    pub fn set_palette(state: &mut AppState, pal: Vec<[f32; 3]>) {
        state.project.palette = pal;
        state.mark_dirty();
    }

    /// Importa paleta a partir de arquivo (.hex ou .gpl) usando o ProjectService.
    pub fn import_palette_file(
        state: &mut AppState,
        path: &std::path::Path,
    ) -> Result<usize, petunia_core::ProjectServiceError> {
        petunia_core::ProjectService::import_palette(state, path)
    }

    /// Exporta a paleta atual para arquivo (.gpl) usando o ProjectService.
    pub fn export_palette_file(
        state: &AppState,
        path: &std::path::Path,
    ) -> Result<(), petunia_core::ProjectServiceError> {
        petunia_core::ProjectService::export_palette(
            &state.project.palette,
            "Petunia Palette",
            path,
        )
    }

    // ---- canvas 2D ----

    pub fn has_canvas(state: &AppState) -> bool {
        state
            .project
            .assets
            .get(state.project.active)
            .map(|o| o.texture.is_some())
            .unwrap_or(true)
    }

    pub fn ensure_canvas(state: &mut AppState) {
        if let Some(o) = state.project.active_mut() {
            if o.texture.is_none() {
                let c = o.base_color;
                o.texture = Some(petunia_project::Canvas::new(
                    256,
                    256,
                    [
                        (c[0] * 255.0) as u8,
                        (c[1] * 255.0) as u8,
                        (c[2] * 255.0) as u8,
                        255,
                    ],
                ));
                state.mark_dirty();
            }
        }
    }

    pub fn canvas_brush(state: &mut AppState, x: u32, y: u32, erase: bool) {
        Self::ensure_canvas(state);
        let (col, r) = if erase {
            ([0, 0, 0, 0], state.canvas_brush)
        } else {
            let c = state.paint_color;
            (
                [
                    (c[0] * 255.0) as u8,
                    (c[1] * 255.0) as u8,
                    (c[2] * 255.0) as u8,
                    255,
                ],
                state.canvas_brush,
            )
        };
        if let Some(o) = state.project.active_mut() {
            if let Some(cv) = o.texture.as_mut() {
                let r = r as i32;
                for dy in -r..=r {
                    for dx in -r..=r {
                        if dx * dx + dy * dy <= r * r {
                            cv.set(
                                (x as i32 + dx).max(0) as u32,
                                (y as i32 + dy).max(0) as u32,
                                col,
                            );
                        }
                    }
                }
                state.mark_dirty();
            }
        }
    }

    pub fn canvas_fill(state: &mut AppState) {
        Self::ensure_canvas(state);
        let c = state.paint_color;
        let col = [
            (c[0] * 255.0) as u8,
            (c[1] * 255.0) as u8,
            (c[2] * 255.0) as u8,
            255,
        ];
        if let Some(o) = state.project.active_mut() {
            if let Some(cv) = o.texture.as_mut() {
                cv.fill(col);
                state.mark_dirty();
            }
        }
    }
}

impl Module for PaintModule {
    fn id(&self) -> &'static str {
        "paint"
    }

    fn as_any(&self) -> &(dyn std::any::Any + 'static) {
        self
    }

    fn as_any_mut(&mut self) -> &mut (dyn std::any::Any + 'static) {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_set_palette_sync() {
        let mut state = AppState::new("en");

        PaintModule::push_palette(&mut state, [0.5, 0.5, 0.5]);
        assert_eq!(state.project.palette[0], [0.5, 0.5, 0.5]);

        let p8 = petunia_project::preset_pico8();
        PaintModule::set_palette(&mut state, p8.clone());
        assert_eq!(state.project.palette.len(), 16);
        assert_eq!(state.project.palette, p8);
    }
}
