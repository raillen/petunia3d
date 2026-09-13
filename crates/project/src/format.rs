//! Formato `.petunia`: binário compacto versionado (postcard).
//! V1: `PetuniaFile { version: 1, project }`. Migrações futuras comparam
//! `version` e convertem.

use serde::{Deserialize, Serialize};

use super::Project;

pub const PROJECT_VERSION: u32 = 1;
const MAGIC: &[u8; 8] = b"PETUNIA\0";

#[derive(Debug, Serialize, Deserialize)]
struct PetuniaFile {
    magic: [u8; 8],
    version: u32,
    project: Project,
}

#[derive(Debug, thiserror::Error)]
pub enum ProjectError {
    #[error("I/O: {0}")]
    Io(String),
    #[error("formato inválido: {0}")]
    Format(String),
    #[error("versão {0} não suportada (máx {1})")]
    Version(u32, u32),
}

pub fn save(project: &Project, path: &std::path::Path) -> Result<(), ProjectError> {
    let file = PetuniaFile {
        magic: *MAGIC,
        version: PROJECT_VERSION,
        project: project.clone(),
    };
    let bytes = postcard::to_allocvec(&file).map_err(|e| ProjectError::Format(e.to_string()))?;
    std::fs::write(path, bytes).map_err(|e| ProjectError::Io(e.to_string()))
}

pub fn load(path: &std::path::Path) -> Result<Project, ProjectError> {
    let bytes = std::fs::read(path).map_err(|e| ProjectError::Io(e.to_string()))?;
    let file: PetuniaFile =
        postcard::from_bytes(&bytes).map_err(|e| ProjectError::Format(e.to_string()))?;
    if file.magic != *MAGIC {
        return Err(ProjectError::Format("magic inválido".into()));
    }
    if file.version > PROJECT_VERSION {
        return Err(ProjectError::Version(file.version, PROJECT_VERSION));
    }
    // v1 -> v1: valida o trust boundary (M2/M3/M4) e converte se preciso
    let mut project = file.project;
    project.validate();
    Ok(project)
}

#[cfg(test)]
mod tests {
    use super::*;
    use petunia_mesh::Mesh;

    #[test]
    fn save_load_roundtrip() {
        let mut p = Project::new();
        p.add("Plane", Mesh::plane(1.0));
        let dir = std::env::temp_dir();
        let path = dir.join("petunia_test_roundtrip.petunia");
        save(&p, &path).unwrap();
        let q = load(&path).unwrap();
        assert_eq!(q.assets.len(), 2);
        assert_eq!(q.assets[1].name, "Plane");
        assert_eq!(q.assets[1].id, p.assets[1].id); // UUID persistente
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn rejects_garbage() {
        let dir = std::env::temp_dir();
        let path = dir.join("petunia_test_garbage.petunia");
        std::fs::write(&path, b"lixo total").unwrap();
        assert!(load(&path).is_err());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn hostile_project_normalized() {
        // M2/M3/M4: projeto corrompido abre sem panic e normalizado
        use super::super::{Asset, Canvas};
        use petunia_mesh::{Face, Mesh, Vertex};
        let bad_mesh = Mesh {
            verts: vec![Vertex::new(0.0, 0.0, 0.0), Vertex::new(1.0, 0.0, 0.0)],
            faces: vec![Face {
                verts: vec![0, 1, 77],
                uv: vec![[0.0, 0.0]],
                selected: false,
            }],
            selected_edges: Default::default(),
        };
        let bad = super::super::Project {
            assets: vec![Asset {
                id: uuid::Uuid::new_v4(),
                name: "bad".into(),
                mesh: bad_mesh,
                visible: true,
                locked: false,
                collection: None,
                base_color: [f32::NAN, 0.0, 0.0],
                texture: Some(Canvas {
                    w: 0,
                    h: 999999,
                    pixels: vec![],
                }),
            }],
            active: 42,
            palette: vec![],
            collections: vec![],
            ..Default::default()
        };
        let dir = std::env::temp_dir();
        let path = dir.join("petunia_test_hostile.petunia");
        save(&bad, &path).unwrap();
        let q = load(&path).unwrap();
        assert!(!q.assets.is_empty());
        assert!(q.active < q.assets.len());
        let a = &q.assets[0];
        assert!(a.mesh.faces.iter().all(|f| f.uv.len() == f.verts.len()));
        assert!(a.base_color.iter().all(|x| x.is_finite()));
        if let Some(cv) = &a.texture {
            assert_eq!(cv.pixels.len(), (cv.w * cv.h * 4) as usize);
        }
        let _ = std::fs::remove_file(&path);
    }
}
