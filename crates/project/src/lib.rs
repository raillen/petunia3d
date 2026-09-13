//! Petunia3D — projeto autocontido: assets com UUID persistente,
//! serialização versionada (postcard) e exportação (OBJ, glTF/GLB).

use petunia_mesh::Mesh;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod autosave;
pub mod export;
pub mod format;
pub mod model_library;
pub mod palette;

pub use autosave::{AutosaveConfig, AutosaveService, RecoveryInfo, SessionLockInfo};
pub use export::{export_gltf, export_obj, ExportError};
pub use model_library::{AssetSummary, ModelLibraryQuery, ModelLibraryService, ModelLibrarySort};
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
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub collection: Option<String>,
    pub base_color: [f32; 3],
    pub texture: Option<Canvas>,
    #[serde(default)]
    pub favorite: bool,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl Asset {
    pub fn new(name: &str, mesh: Mesh) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            mesh,
            visible: true,
            locked: false,
            collection: None,
            base_color: [0.75, 0.75, 0.78],
            texture: None,
            favorite: false,
            tags: Vec::new(),
        }
    }

    /// Duplicata com novo UUID.
    pub fn duplicate(&self) -> Self {
        let mut c = self.clone();
        c.id = Uuid::new_v4();
        c.name = format!("{} copy", self.name);
        c
    }

    /// Adiciona uma tag normalizada (minúscula, sem espaços extras).
    pub fn add_tag(&mut self, tag: &str) -> bool {
        let trimmed = tag.trim().to_lowercase();
        if trimmed.is_empty() || self.tags.iter().any(|t| t.to_lowercase() == trimmed) {
            return false;
        }
        self.tags.push(trimmed);
        true
    }

    /// Remove uma tag.
    pub fn remove_tag(&mut self, tag: &str) {
        let trimmed = tag.trim().to_lowercase();
        self.tags.retain(|t| t.to_lowercase() != trimmed);
    }

    /// Verifica se possui determinada tag.
    pub fn has_tag(&self, tag: &str) -> bool {
        let trimmed = tag.trim().to_lowercase();
        self.tags.iter().any(|t| t.to_lowercase() == trimmed)
    }

    /// Alterna estado de favorito.
    pub fn toggle_favorite(&mut self) {
        self.favorite = !self.favorite;
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

fn default_stroke_color() -> [f32; 4] {
    [0.0, 0.74, 0.83, 1.0] // Ciano característico do Blender
}

fn default_stroke_width() -> f32 {
    2.0
}

const fn default_true() -> bool {
    true
}

const fn default_scale() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

/// Traço de anotação livre em espaço 3D (ferramenta Annotate).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AnnotationStroke {
    pub points: Vec<[f32; 3]>,
    #[serde(default = "default_stroke_color")]
    pub color: [f32; 4],
    #[serde(default = "default_stroke_width")]
    pub width: f32,
}

impl Default for AnnotationStroke {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            color: default_stroke_color(),
            width: default_stroke_width(),
        }
    }
}

/// Item de anotação pertencente à collection de Anotações do projeto.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AnnotationItem {
    pub id: Uuid,
    pub name: String,
    #[serde(default)]
    pub group: Option<String>,
    pub strokes: Vec<AnnotationStroke>,
    #[serde(default = "default_true")]
    pub visible: bool,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub translation: [f32; 3],
    #[serde(default)]
    pub rotation: [f32; 3], // Graus de Euler XYZ
    #[serde(default = "default_scale")]
    pub scale: [f32; 3], // [1.0, 1.0, 1.0]
}

impl AnnotationItem {
    pub fn new(name: impl Into<String>, strokes: Vec<AnnotationStroke>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            group: None,
            strokes,
            visible: true,
            locked: false,
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        }
    }

    pub fn transform_matrix(&self) -> glam::Mat4 {
        let t = glam::Vec3::from(self.translation);
        let r = glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            self.rotation[0].to_radians(),
            self.rotation[1].to_radians(),
            self.rotation[2].to_radians(),
        );
        let s = glam::Vec3::from(self.scale);
        glam::Mat4::from_scale_rotation_translation(s, r, t)
    }

    pub fn transform_point(&self, pt: [f32; 3]) -> [f32; 3] {
        let m = self.transform_matrix();
        let v = m.transform_point3(glam::Vec3::from(pt));
        [v.x, v.y, v.z]
    }

    pub fn center(&self) -> [f32; 3] {
        let mut sum = glam::Vec3::ZERO;
        let mut count = 0;
        let m = self.transform_matrix();
        for s in &self.strokes {
            for &pt in &s.points {
                sum += m.transform_point3(glam::Vec3::from(pt));
                count += 1;
            }
        }
        if count > 0 {
            let avg = sum / (count as f32);
            [avg.x, avg.y, avg.z]
        } else {
            self.translation
        }
    }
}

/// Item de medição tridimensional com distância euclidiana e deltas cartesianos.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MeasurementItem {
    pub id: Uuid,
    pub name: String,
    pub start: [f32; 3],
    pub end: [f32; 3],
    pub distance: f32,
    #[serde(default = "default_true")]
    pub visible: bool,
}

impl MeasurementItem {
    pub fn new(name: impl Into<String>, start: [f32; 3], end: [f32; 3], distance: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            start,
            end,
            distance,
            visible: true,
        }
    }

    pub fn deltas(&self) -> [f32; 3] {
        [
            (self.end[0] - self.start[0]).abs(),
            (self.end[1] - self.start[1]).abs(),
            (self.end[2] - self.start[2]).abs(),
        ]
    }
}

fn default_project_name() -> String {
    "Untitled".to_string()
}

/// Projeto: metadados, lista de assets com UUID persistente, paleta, coleções, anotações e medições.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    #[serde(default = "default_project_name")]
    pub name: String,
    pub assets: Vec<Asset>,
    pub active: usize,
    #[serde(default = "default_palette")]
    pub palette: Vec<[f32; 3]>,
    #[serde(default)]
    pub collections: Vec<String>,
    #[serde(default)]
    pub annotations: Vec<AnnotationItem>,
    #[serde(default)]
    pub annotation_groups: Vec<String>,
    #[serde(default)]
    pub measurements: Vec<MeasurementItem>,
    #[serde(default = "default_true")]
    pub annotations_visible: bool,
    #[serde(default)]
    pub annotations_locked: bool,
    #[serde(default = "default_true")]
    pub measurements_visible: bool,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: default_project_name(),
            assets: Vec::new(),
            active: 0,
            palette: default_palette(),
            collections: Vec::new(),
            annotations: Vec::new(),
            annotation_groups: Vec::new(),
            measurements: Vec::new(),
            annotations_visible: true,
            annotations_locked: false,
            measurements_visible: true,
        }
    }
}

impl Project {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: default_project_name(),
            assets: vec![Asset::new("Cube", Mesh::cube(2.0))],
            active: 0,
            palette: default_palette(),
            collections: Vec::new(),
            annotations: Vec::new(),
            annotation_groups: Vec::new(),
            measurements: Vec::new(),
            annotations_visible: true,
            annotations_locked: false,
            measurements_visible: true,
        }
    }

    pub fn add_annotation(&mut self, item: AnnotationItem) {
        self.annotations.push(item);
    }

    pub fn remove_annotation(&mut self, id: Uuid) {
        self.annotations.retain(|a| a.id != id);
    }

    pub fn add_annotation_group(&mut self, name: &str) -> bool {
        let trimmed = name.trim();
        if trimmed.is_empty() || self.annotation_groups.iter().any(|g| g == trimmed) {
            return false;
        }
        self.annotation_groups.push(trimmed.to_string());
        true
    }

    pub fn remove_annotation_group(&mut self, name: &str) {
        self.annotation_groups.retain(|g| g != name);
        for a in &mut self.annotations {
            if a.group.as_deref() == Some(name) {
                a.group = None;
            }
        }
    }

    pub fn add_measurement(&mut self, item: MeasurementItem) {
        self.measurements.push(item);
    }

    pub fn remove_measurement(&mut self, id: Uuid) {
        self.measurements.retain(|m| m.id != id);
    }

    pub fn add_collection(&mut self, name: &str) -> bool {
        let trimmed = name.trim();
        if trimmed.is_empty() || self.collections.iter().any(|c| c == trimmed) {
            return false;
        }
        self.collections.push(trimmed.to_string());
        true
    }

    pub fn remove_collection(&mut self, name: &str) {
        self.collections.retain(|c| c != name);
        for a in &mut self.assets {
            if a.collection.as_deref() == Some(name) {
                a.collection = None;
            }
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

    /// Localiza asset por ID estável retornando índice e referência.
    pub fn find_by_id(&self, id: Uuid) -> Option<(usize, &Asset)> {
        self.assets.iter().enumerate().find(|(_, a)| a.id == id)
    }

    /// Localiza asset por ID estável retornando índice e referência mutável.
    pub fn find_by_id_mut(&mut self, id: Uuid) -> Option<(usize, &mut Asset)> {
        self.assets.iter_mut().enumerate().find(|(_, a)| a.id == id)
    }

    /// Remove asset por ID estável mantendo invariants de seleção.
    pub fn remove_by_id(&mut self, id: Uuid) -> bool {
        if let Some(pos) = self.find(id) {
            self.remove(pos);
            true
        } else {
            false
        }
    }

    /// Duplica asset por ID gerando novo UUID persistente e ativando-o.
    pub fn duplicate_by_id(&mut self, id: Uuid) -> Option<Uuid> {
        let dup = {
            let (_, asset) = self.find_by_id(id)?;
            asset.duplicate()
        };
        let new_id = dup.id;
        self.assets.push(dup);
        self.active = self.assets.len() - 1;
        Some(new_id)
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

    #[test]
    fn test_annotation_item_transform() {
        let mut stroke = AnnotationStroke::default();
        stroke.points.push([1.0, 0.0, 0.0]);
        stroke.points.push([3.0, 0.0, 0.0]);

        let mut item = AnnotationItem::new("Note 1", vec![stroke]);
        item.translation = [10.0, 5.0, 2.0];
        item.scale = [2.0, 2.0, 2.0];

        let p0 = item.transform_point([1.0, 0.0, 0.0]);
        assert_eq!(p0, [12.0, 5.0, 2.0]);

        let center = item.center();
        assert_eq!(center, [14.0, 5.0, 2.0]);
    }

    #[test]
    fn test_project_annotations_and_measurements_management() {
        let mut p = Project::new();
        assert!(p.annotations.is_empty());
        assert!(p.measurements.is_empty());

        let a_item = AnnotationItem::new("A1", vec![AnnotationStroke::default()]);
        let a_id = a_item.id;
        p.add_annotation(a_item);
        assert_eq!(p.annotations.len(), 1);

        assert!(p.add_annotation_group("Rascunhos"));
        p.annotations[0].group = Some("Rascunhos".to_string());
        p.remove_annotation_group("Rascunhos");
        assert!(p.annotations[0].group.is_none());

        p.remove_annotation(a_id);
        assert!(p.annotations.is_empty());

        let m_item = MeasurementItem::new("M1", [0.0, 0.0, 0.0], [1.0, 2.0, 2.0], 3.0);
        let m_id = m_item.id;
        p.add_measurement(m_item);
        assert_eq!(p.measurements.len(), 1);
        assert_eq!(p.measurements[0].deltas(), [1.0, 2.0, 2.0]);

        p.remove_measurement(m_id);
        assert!(p.measurements.is_empty());
    }
}
