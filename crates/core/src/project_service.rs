//! Serviço canônico de ciclo de vida, persistência e I/O de projetos do Petunia3D.
//!
//! Isola completamente operações com disco, parsing de malhas e serialização
//! do ecossistema de interface de usuário (egui/rfd), permitindo execução pura
//! e headless em testes, CLI ou frontends alternativos.

use std::path::Path;

use petunia_mesh::Mesh;
use petunia_project::{export, format, palette};

use crate::state::AppState;
use crate::{AppEvent, ReferenceImage};

/// Erros estruturados ocorridos durante operações de I/O de projetos e assets.
#[derive(Debug, thiserror::Error)]
pub enum ProjectServiceError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Format error: {0}")]
    Format(String),

    #[error("Asset not found at index {0}")]
    AssetNotFound(usize),

    #[error("Export error: {0}")]
    Export(String),

    #[error("No valid colors found in palette file")]
    InvalidPalette,
}

/// Serviço puro de aplicação para carregamento, salvamento, importação e exportação de projetos e assets.
pub struct ProjectService;

impl ProjectService {
    /// Reinicia a sessão para um projeto vazio padrão (P3D-001 §New Project).
    pub fn new_project(state: &mut AppState) {
        state.project.reset();
        state.session.tools.uv_selected.clear();
        state.sync_selection();
        state.mark_document_clean();
        state.set_status("new project".to_string());
        state.events.emit(AppEvent::ProjectLoaded);
        state.mark_dirty();
    }

    /// Carrega um arquivo de projeto (.petunia) e sincroniza o estado da aplicação (P3D-001 §Open Project).
    pub fn load_project(state: &mut AppState, path: &Path) -> Result<(), ProjectServiceError> {
        let p = format::load(path).map_err(|e| ProjectServiceError::Format(e.to_string()))?;
        state.project.palette = p.palette.clone();
        state.project.project = p;
        state.project.undo.clear();
        state.session.tools.uv_selected.clear();
        state.project.project_path = Some(path.to_string_lossy().to_string());
        state.mark_document_clean();
        state.events.emit(AppEvent::ProjectLoaded);
        state.sync_selection();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let name = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Untitled".to_string());
        state.project.recent_projects.add(path, &name, now);
        let _ = state.project.recent_projects.save();
        state.set_status(format!("open {}", path.display()));
        state.mark_dirty();
        Ok(())
    }

    /// Salva o estado atual do projeto no arquivo especificado (.petunia) usando escrita atômica segura (P3D-001 §Save).
    pub fn save_project(state: &mut AppState, path: &Path) -> Result<(), ProjectServiceError> {
        state.events.emit(AppEvent::ProjectSaving);
        state.project.project.palette = state.project.palette.clone();
        format::save(&state.project.project, path)
            .map_err(|e| ProjectServiceError::Format(e.to_string()))?;
        state.project.project_path = Some(path.to_string_lossy().to_string());
        state.mark_document_clean();
        state.events.emit(AppEvent::ProjectSaved);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let name = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Untitled".to_string());
        state.project.recent_projects.add(path, &name, now);
        let _ = state.project.recent_projects.save();
        state.set_status(format!("saved {}", path.display()));
        state.mark_dirty();
        Ok(())
    }

    /// Salva o projeto em um novo destino de arquivo e atualiza o caminho ativo (P3D-001 §Save As).
    pub fn save_as_project(
        state: &mut AppState,
        new_path: &Path,
    ) -> Result<(), ProjectServiceError> {
        Self::save_project(state, new_path)
    }

    /// Recupera projeto a partir de snapshot de autosave preservando dirty state e sem sobrescrever arquivo principal (P3D-002 §Recovery).
    pub fn recover_from_snapshot(
        state: &mut AppState,
        snapshot_path: &Path,
        original_path: Option<&Path>,
    ) -> Result<(), ProjectServiceError> {
        let p =
            format::load(snapshot_path).map_err(|e| ProjectServiceError::Format(e.to_string()))?;
        state.project.palette = p.palette.clone();
        state.project.project = p;
        state.project.undo.clear();
        state.session.tools.uv_selected.clear();
        state.project.project_path = original_path.map(|p| p.to_string_lossy().to_string());
        // Ao recuperar, o documento entra como modificado/dirty (P3D-002)
        state.mark_document_dirty();
        state.events.emit(AppEvent::ProjectLoaded);
        state.sync_selection();
        state.set_status("recovered from autosave snapshot".to_string());
        state.mark_dirty();
        Ok(())
    }

    /// Importa uma malha Wavefront OBJ do disco e a anexa como novo asset do projeto.
    pub fn import_obj(state: &mut AppState, path: &Path) -> Result<String, ProjectServiceError> {
        let text = std::fs::read_to_string(path)?;
        let mesh = Mesh::from_obj(&text);
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("mesh")
            .to_string();

        state.checkpoint("import obj");
        state.project.add(&name, mesh);
        state.sync_selection();
        state.emit_mesh_changed();
        state.set_status(format!("import {}", path.display()));
        state.mark_dirty();
        Ok(name)
    }

    /// Exporta um asset do projeto para um arquivo Wavefront OBJ.
    pub fn export_obj(
        state: &AppState,
        asset_idx: usize,
        path: &Path,
    ) -> Result<(), ProjectServiceError> {
        let asset = state
            .project
            .assets
            .get(asset_idx)
            .ok_or(ProjectServiceError::AssetNotFound(asset_idx))?;
        let obj_text = export::export_obj(asset);
        std::fs::write(path, obj_text)?;
        Ok(())
    }

    /// Exporta múltiplos assets para arquivos OBJ individuais dentro de um diretório.
    pub fn export_all_obj_to_dir(
        state: &AppState,
        asset_indices: &[usize],
        dir: &Path,
    ) -> Result<usize, ProjectServiceError> {
        let mut count = 0;
        for &i in asset_indices {
            if let Some(asset) = state.project.assets.get(i) {
                let sanitized_name = sanitize_filename(&asset.name);
                let target_path = dir.join(format!("{sanitized_name}.obj"));
                if std::fs::write(&target_path, export::export_obj(asset)).is_ok() {
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    /// Exporta os assets indicados para um arquivo glTF Binário (.glb).
    pub fn export_glb(
        state: &AppState,
        asset_indices: &[usize],
        path: &Path,
    ) -> Result<(), ProjectServiceError> {
        let bytes = export::export_gltf(&state.project, asset_indices)
            .map_err(|e| ProjectServiceError::Export(e.to_string()))?;
        std::fs::write(path, bytes)?;
        Ok(())
    }

    /// Importa paleta de cores a partir de um arquivo .hex ou .gpl.
    pub fn import_palette(state: &mut AppState, path: &Path) -> Result<usize, ProjectServiceError> {
        let text = std::fs::read_to_string(path)?;
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let colors = if ext.eq_ignore_ascii_case("gpl") {
            palette::import_gpl(&text)
        } else {
            palette::import_hex(&text)
        };

        if colors.is_empty() {
            return Err(ProjectServiceError::InvalidPalette);
        }

        let count = colors.len();
        state.project.palette = colors.clone();
        state.project.project.palette = colors;
        state.set_status(format!("imported {count} colors"));
        state.mark_dirty();
        Ok(count)
    }

    /// Exporta uma paleta de cores no formato GIMP Palette (.gpl).
    pub fn export_palette(
        palette: &[[f32; 3]],
        title: &str,
        path: &Path,
    ) -> Result<(), ProjectServiceError> {
        let gpl = palette::export_gpl(title, palette);
        std::fs::write(path, gpl)?;
        Ok(())
    }

    /// Adiciona uma imagem de referência à cena 3D.
    pub fn add_reference_image(
        state: &mut AppState,
        name: String,
        width: u32,
        height: u32,
        rgba: Vec<u8>,
    ) {
        state
            .project
            .refs
            .push(ReferenceImage::from_rgba(name.clone(), width, height, rgba));
        state.set_status(format!("ref {name}"));
        state.mark_dirty();
    }
}

/// Sanitiza strings para uso seguro como nomes de arquivo.
pub fn sanitize_filename(name: &str) -> String {
    let sanitized: String = name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    if sanitized.is_empty() {
        "asset".to_string()
    } else {
        sanitized
    }
}
