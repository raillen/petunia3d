//! Petunia3D — projeto autocontido: assets com UUID persistente,
//! serialização versionada (postcard) e exportação (OBJ, glTF/GLB).

use petunia_mesh::Mesh;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod export;
pub mod format;
pub mod palette;

pub use export::{export_gltf, export_obj, ExportError};
pub use palette::{export_gpl, export_hex, import_gpl, import_hex, preset_gameboy, preset_pico8};

/// Canvas de textura simples (albedo) por asset — workspace PAINT.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Canvas {
    pub w: u32,
    pub h: u32,
    /// RGBA8 row-major, origem em cima.
    pub pixels: Vec<u8>,
}

impl Canvas {
    pub fn new(w: u32, h: u32, fill: [u8; 4]) -> Self {
        let w = w.clamp(1, 1024);
        let h = h.clamp(1, 1024);
        let mut pixels = vec![0u8; (w * h * 4) as usize];
        for i in (0..pixels.len()).step_by(4) {
            pixels[i..i + 4].copy_from_slice(&fill);
        }
        Self { w, h, pixels }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if x >= self.w || y >= self.h {
            return None;
        }
        let i = ((y * self.w + x) * 4) as usize;
        Some([
            self.pixels[i],
            self.pixels[i + 1],
            self.pixels[i + 2],
            self.pixels[i + 3],
        ])
    }

    pub fn set(&mut self, x: u32, y: u32, c: [u8; 4]) {
        if x >= self.w || y >= self.h {
            return;
        }
        let i = ((y * self.w + x) * 4) as usize;
        self.pixels[i..i + 4].copy_from_slice(&c);
    }

    pub fn fill(&mut self, c: [u8; 4]) {
        for i in (0..self.pixels.len()).step_by(4) {
            self.pixels[i..i + 4].copy_from_slice(&c);
        }
    }

    /// Repara canvas vindo de arquivo (M4): dims 1..1024 + pixels exatos.
    pub fn validate(&mut self) {
        self.w = self.w.clamp(1, 1024);
        self.h = self.h.clamp(1, 1024);
        let want = (self.w * self.h * 4) as usize;
        self.pixels.resize(want, 0);
    }
}

/// Um asset do projeto. `id` nunca muda (rename seguro).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    pub id: Uuid,
    pub name: String,
    pub mesh: Mesh,
    pub visible: bool,
    pub base_color: [f32; 3],
    pub texture: Option<Canvas>,
}

impl Asset {
    pub fn new(name: &str, mesh: Mesh) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            mesh,
            visible: true,
            base_color: [0.75, 0.75, 0.78],
            texture: None,
        }
    }

    /// Duplicata com novo UUID.
    pub fn duplicate(&self) -> Self {
        let mut c = self.clone();
        c.id = Uuid::new_v4();
        c.name = format!("{} copy", self.name);
        c
    }
}

fn default_palette() -> Vec<[f32; 3]> {
    vec![
        [1.0, 0.2, 0.2],
        [1.0, 0.8, 0.2],
        [0.2, 0.8, 0.3],
        [0.3, 0.5, 1.0],
    ]
}

/// Projeto: lista de assets + ativo + paleta persistente.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub assets: Vec<Asset>,
    pub active: usize,
    #[serde(default = "default_palette")]
    pub palette: Vec<[f32; 3]>,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            assets: Vec::new(),
            active: 0,
            palette: default_palette(),
        }
    }
}

impl Project {
    pub fn new() -> Self {
        Self {
            assets: vec![Asset::new("Cube", Mesh::cube(2.0))],
            active: 0,
            palette: default_palette(),
        }
    }

    pub fn active(&self) -> Option<&Asset> {
        self.assets.get(self.active)
    }
    pub fn active_mesh(&self) -> Option<&Mesh> {
        self.active().map(|o| &o.mesh)
    }

    pub fn active_mut(&mut self) -> Option<&mut Asset> {
        self.assets.get_mut(self.active)
    }
    pub fn active_mesh_mut(&mut self) -> Option<&mut Mesh> {
        self.active_mut().map(|o| &mut o.mesh)
    }

    pub fn add(&mut self, name: &str, mesh: Mesh) {
        self.assets.push(Asset::new(name, mesh));
        self.active = self.assets.len() - 1;
    }

    pub fn remove(&mut self, i: usize) {
        if self.assets.len() > 1 && i < self.assets.len() {
            self.assets.remove(i);
            if i < self.active && self.active > 0 {
                self.active -= 1;
            } else {
                self.active = self.active.min(self.assets.len() - 1);
            }
        }
    }

    pub fn totals(&self) -> (usize, usize) {
        let (mut v, mut f) = (0, 0);
        for o in &self.assets {
            v += o.mesh.vert_count();
            f += o.mesh.tri_count();
        }
        (v, f)
    }

    pub fn find(&self, id: Uuid) -> Option<usize> {
        self.assets.iter().position(|a| a.id == id)
    }

    /// Normaliza projeto vindo de arquivo (M2/M3): malhas válidas,
    /// no mínimo 1 asset, `active` dentro dos limites.
    pub fn validate(&mut self) {
        for a in &mut self.assets {
            a.mesh.validate();
            if let Some(cv) = a.texture.as_mut() {
                cv.validate();
            }
            if !a.base_color.iter().all(|x| x.is_finite()) {
                a.base_color = [0.75, 0.75, 0.78];
            }
        }
        if self.assets.is_empty() {
            self.assets.push(Asset::new("Cube", Mesh::cube(2.0)));
        }
        self.active = self.active.min(self.assets.len() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_remove_shifts_active_index() {
        let mut p = Project::new();
        p.add("Asset 1", Mesh::cube(1.0));
        p.add("Asset 2", Mesh::cube(1.0));
        assert_eq!(p.assets.len(), 3);
        p.active = 2; // Asset 2 active

        // Remove Asset 0 (before active)
        p.remove(0);
        assert_eq!(p.assets.len(), 2);
        assert_eq!(p.active, 1);
        assert_eq!(p.assets[p.active].name, "Asset 2");

        // Remove active asset
        p.remove(1);
        assert_eq!(p.assets.len(), 1);
        assert_eq!(p.active, 0);
        assert_eq!(p.assets[p.active].name, "Asset 1");
    }
}
