//! Formato `.petunia`: binário compacto versionado (postcard).
//! V1: `PetuniaFile { version: 1, project }`. Migrações futuras comparam
//! `version` e convertem.

use serde::{Deserialize, Serialize};
use std::io::Write;

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

/// Salva o projeto no disco utilizando escrita atômica segura (P3D-001).
pub fn save(project: &Project, path: &std::path::Path) -> Result<(), ProjectError> {
    save_atomic(project, path)
}

/// Salva o projeto de forma atômica e resiliente a falhas:
/// 1. Serializa para buffer binário postcard;
/// 2. Cria arquivo temporário oculto no mesmo diretório (.petunia.tmp.{uuid});
/// 3. Grava e realiza sync_all() garantindo persistência física;
/// 4. Renomeia atomicamente sobre o destino final.
///
/// Em caso de qualquer erro, o arquivo de destino original permanece 100% intacto.
pub fn save_atomic(project: &Project, path: &std::path::Path) -> Result<(), ProjectError> {
    let file = PetuniaFile {
        magic: *MAGIC,
        version: PROJECT_VERSION,
        project: project.clone(),
    };
    let bytes = postcard::to_allocvec(&file).map_err(|e| ProjectError::Format(e.to_string()))?;

    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    if !parent.as_os_str().is_empty() {
        std::fs::create_dir_all(parent).map_err(|e| ProjectError::Io(e.to_string()))?;
    }

    let file_stem = path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("project");
    let temp_file_name = format!(".{file_stem}.tmp.{}", uuid::Uuid::new_v4());
    let temp_path = parent.join(temp_file_name);

    let write_res = (|| -> Result<(), std::io::Error> {
        let mut temp_file = std::fs::File::create(&temp_path)?;
        temp_file.write_all(&bytes)?;
        temp_file.sync_all()?;
        Ok(())
    })();

    if let Err(e) = write_res {
        let _ = std::fs::remove_file(&temp_path);
        return Err(ProjectError::Io(e.to_string()));
    }

    if let Err(e) = std::fs::rename(&temp_path, path) {
        let _ = std::fs::remove_file(&temp_path);
        return Err(ProjectError::Io(e.to_string()));
    }

    Ok(())
}

pub fn load(path: &std::path::Path) -> Result<Project, ProjectError> {
    let bytes = std::fs::read(path).map_err(|e| ProjectError::Io(e.to_string()))?;
    load_bytes(&bytes)
}

/// Parses a project from raw bytes (fuzz boundary P0-13: never panics on
/// hostile input, only returns typed errors).
pub fn load_bytes(bytes: &[u8]) -> Result<Project, ProjectError> {
    let file: PetuniaFile =
        postcard::from_bytes(bytes).map_err(|e| ProjectError::Format(e.to_string()))?;
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
    use super::super::Asset;
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
                material_slot: None,
            }],
            selected_edges: Default::default(),
        };
        let bad = super::super::Project {
            id: uuid::Uuid::new_v4(),
            name: "bad".into(),
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
                material_id: None,
                skeleton_id: None,
                skin_data: None,
                favorite: false,
                tags: vec![],
                modifiers: vec![],
                paint_stack: None,
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

    #[test]
    fn test_future_version_rejected() {
        let p = Project::new();
        let file = PetuniaFile {
            magic: *MAGIC,
            version: 999, // Versão futura
            project: p,
        };
        let bytes = postcard::to_allocvec(&file).unwrap();
        let dir = std::env::temp_dir();
        let path = dir.join("petunia_test_future_ver.petunia");
        std::fs::write(&path, bytes).unwrap();

        let res = load(&path);
        assert!(matches!(res, Err(ProjectError::Version(999, 1))));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_project_metadata_and_asset_tags_roundtrip() {
        let mut p = Project::new();
        p.name = "My Adventure".to_string();
        let mut cube_asset = Asset::new("Hero", Mesh::cube(1.5));
        cube_asset.favorite = true;
        assert!(cube_asset.add_tag("character"));
        assert!(cube_asset.add_tag("protagonist"));
        let hero_id = cube_asset.id;
        p.assets.push(cube_asset);

        let dir = std::env::temp_dir();
        let path = dir.join("petunia_test_tags_roundtrip.petunia");
        save(&p, &path).unwrap();

        let loaded = load(&path).unwrap();
        assert_eq!(loaded.id, p.id);
        assert_eq!(loaded.name, "My Adventure");
        let hero = loaded.assets.iter().find(|a| a.id == hero_id).unwrap();
        assert!(hero.favorite);
        assert!(hero.has_tag("character"));
        assert!(hero.has_tag("protagonist"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_atomic_save_preserves_original_on_failure() {
        let mut original = Project::new();
        original.name = "Original Intact".to_string();
        let dir =
            std::env::temp_dir().join(format!("petunia_atomic_test_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("project.petunia");

        // Salva versão original válida
        save(&original, &path).unwrap();
        let valid_bytes = std::fs::read(&path).unwrap();

        // Tentativa de salvar num caminho inválido onde o diretório pai é um arquivo comum
        let invalid_path = path.join("sub_project.petunia");
        let mut corrupt_attempt = Project::new();
        corrupt_attempt.name = "Corrupt".to_string();
        let err = save(&corrupt_attempt, &invalid_path);
        assert!(err.is_err(), "Deve falhar ao tentar salvar sob um arquivo");

        // O arquivo original deve permanecer 100% inalterado
        let current_bytes = std::fs::read(&path).unwrap();
        assert_eq!(current_bytes, valid_bytes);
        let loaded = load(&path).unwrap();
        assert_eq!(loaded.name, "Original Intact");

        let _ = std::fs::remove_dir_all(&dir);
    }
}
