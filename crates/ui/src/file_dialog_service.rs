//! Serviço integrado de diálogos de arquivo para o Petunia3D com `egui-file-dialog`.
//!
//! Fornece seleção e salvamento de arquivos dentro do canvas egui (ou modal nativo),
//! estilizado com os tokens visuais do Petunia Design System.

use std::path::PathBuf;

use egui::Context;
use egui_file_dialog::FileDialog;
use petunia_core::AppState;
use petunia_mesh::Mesh;
use petunia_project::{export, format};

/// Ação pendente solicitada através do diálogo de arquivos.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileDialogAction {
    OpenProject,
    SaveProject,
    SaveProjectAs,
    ImportObj,
    ExportObj,
    ExportGlb,
}

/// Serviço gerenciador de diálogo de arquivos integrado à UI.
pub struct PetuniaFileDialogService {
    dialog: FileDialog,
    pending_action: Option<FileDialogAction>,
}

impl Default for PetuniaFileDialogService {
    fn default() -> Self {
        Self::new()
    }
}

impl PetuniaFileDialogService {
    pub fn new() -> Self {
        Self {
            dialog: FileDialog::new()
                .as_modal(true)
                .title("Petunia3D File Explorer"),
            pending_action: None,
        }
    }

    /// Abre o diálogo para carregar um projeto (.petunia).
    pub fn open_project(&mut self) {
        self.pending_action = Some(FileDialogAction::OpenProject);
        let mut dialog = FileDialog::new()
            .as_modal(true)
            .title("Open Petunia Project")
            .add_file_filter_extensions("Petunia Project (*.petunia)", vec!["petunia"]);
        dialog.pick_file();
        self.dialog = dialog;
    }

    /// Abre o diálogo para salvar o projeto (.petunia).
    pub fn save_project(&mut self, current_path: Option<&str>, state: &mut AppState) {
        if let Some(path) = current_path {
            let p = PathBuf::from(path);
            self.execute_action(FileDialogAction::SaveProject, p, state);
        } else {
            self.save_project_as();
        }
    }

    /// Abre o diálogo para salvar como novo arquivo (.petunia).
    pub fn save_project_as(&mut self) {
        self.pending_action = Some(FileDialogAction::SaveProjectAs);
        let mut dialog = FileDialog::new()
            .as_modal(true)
            .title("Save Petunia Project")
            .add_file_filter_extensions("Petunia Project (*.petunia)", vec!["petunia"])
            .default_file_name("project.petunia");
        dialog.save_file();
        self.dialog = dialog;
    }

    /// Abre o diálogo para importar malha Wavefront OBJ.
    pub fn import_obj(&mut self) {
        self.pending_action = Some(FileDialogAction::ImportObj);
        let mut dialog = FileDialog::new()
            .as_modal(true)
            .title("Import OBJ Mesh")
            .add_file_filter_extensions("Wavefront OBJ (*.obj)", vec!["obj"]);
        dialog.pick_file();
        self.dialog = dialog;
    }

    /// Abre o diálogo para exportar malha Wavefront OBJ.
    pub fn export_obj(&mut self, default_name: &str) {
        self.pending_action = Some(FileDialogAction::ExportObj);
        let file_name = format!("{default_name}.obj");
        let mut dialog = FileDialog::new()
            .as_modal(true)
            .title("Export OBJ Mesh")
            .add_file_filter_extensions("Wavefront OBJ (*.obj)", vec!["obj"])
            .default_file_name(&file_name);
        dialog.save_file();
        self.dialog = dialog;
    }

    /// Abre o diálogo para exportar malha glTF Binário (.glb).
    pub fn export_glb(&mut self, default_name: &str) {
        self.pending_action = Some(FileDialogAction::ExportGlb);
        let file_name = format!("{default_name}.glb");
        let mut dialog = FileDialog::new()
            .as_modal(true)
            .title("Export glTF Binary")
            .add_file_filter_extensions("glTF Binary (*.glb)", vec!["glb"])
            .default_file_name(&file_name);
        dialog.save_file();
        self.dialog = dialog;
    }

    /// Atualiza o diálogo de arquivos no frame e despacha a ação quando confirmada.
    pub fn update(&mut self, ctx: &Context, state: &mut AppState) {
        self.dialog.update(ctx);

        if let Some(path) = self.dialog.take_picked() {
            if let Some(action) = self.pending_action.take() {
                self.execute_action(action, path, state);
            }
        }
    }

    fn execute_action(&self, action: FileDialogAction, path: PathBuf, state: &mut AppState) {
        match action {
            FileDialogAction::OpenProject => match format::load(&path) {
                Ok(p) => {
                    state.palette = p.palette.clone();
                    state.project = p;
                    state.undo.clear();
                    state.uv_selected.clear();
                    state.project_path = Some(path.to_string_lossy().to_string());
                    state.events.emit(petunia_core::AppEvent::ProjectLoaded);
                    state.sync_selection();
                    state.set_status(format!("open {}", path.display()));
                }
                Err(e) => state.set_status(format!("open err: {e}")),
            },
            FileDialogAction::SaveProject | FileDialogAction::SaveProjectAs => {
                state.project.palette = state.palette.clone();
                match format::save(&state.project, &path) {
                    Ok(()) => {
                        state.project_path = Some(path.to_string_lossy().to_string());
                        state.set_status(format!("saved {}", path.display()));
                    }
                    Err(e) => state.set_status(format!("save err: {e}")),
                }
            }
            FileDialogAction::ImportObj => match std::fs::read_to_string(&path) {
                Ok(text) => {
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
                }
                Err(e) => state.set_status(format!("import err: {e}")),
            },
            FileDialogAction::ExportObj => {
                if let Some(asset) = state.project.assets.get(state.project.active) {
                    let obj = export::export_obj(asset);
                    match std::fs::write(&path, obj) {
                        Ok(()) => state.set_status(format!("exported obj {}", path.display())),
                        Err(e) => state.set_status(format!("export obj err: {e}")),
                    }
                }
            }
            FileDialogAction::ExportGlb => {
                let active = state.project.active;
                match export::export_gltf(&state.project, &[active]) {
                    Ok(bytes) => match std::fs::write(&path, bytes) {
                        Ok(()) => state.set_status(format!("exported glb {}", path.display())),
                        Err(e) => state.set_status(format!("export glb err: {e}")),
                    },
                    Err(e) => state.set_status(format!("export glb err: {e}")),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_dialog_service_initialization() {
        let service = PetuniaFileDialogService::new();
        assert!(service.pending_action.is_none());
    }

    #[test]
    fn test_file_dialog_action_triggers() {
        let mut service = PetuniaFileDialogService::new();
        service.open_project();
        assert_eq!(service.pending_action, Some(FileDialogAction::OpenProject));

        service.import_obj();
        assert_eq!(service.pending_action, Some(FileDialogAction::ImportObj));

        service.save_project_as();
        assert_eq!(
            service.pending_action,
            Some(FileDialogAction::SaveProjectAs)
        );

        service.export_obj("test_model");
        assert_eq!(service.pending_action, Some(FileDialogAction::ExportObj));

        service.export_glb("test_model");
        assert_eq!(service.pending_action, Some(FileDialogAction::ExportGlb));
    }
}
