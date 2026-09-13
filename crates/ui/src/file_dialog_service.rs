//! Serviço integrado de diálogos de arquivo para o Petunia3D.
//!
//! Encapsula todas as chamadas a diálogos nativos do sistema operacional (`rfd::FileDialog`)
//! e diálogos in-canvas (`egui-file-dialog`), delegando todas as operações de persistência
//! e conversão de formatos ao `ProjectService` puro do `petunia_core`.

use std::path::PathBuf;

use egui::Context;
use egui_file_dialog::FileDialog;
use petunia_core::{AppState, ProjectService};

/// Ação pendente solicitada através do diálogo de arquivos in-canvas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileDialogAction {
    OpenProject,
    SaveProject,
    SaveProjectAs,
    ImportObj,
    ExportObj,
    ExportGlb,
    ImportPalette,
    ExportPalette,
}

/// Serviço gerenciador de diálogo de arquivos integrado à UI (in-canvas modal).
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

    /// Abre o diálogo para importar paleta de cores.
    pub fn import_palette(&mut self) {
        self.pending_action = Some(FileDialogAction::ImportPalette);
        let mut dialog = FileDialog::new()
            .as_modal(true)
            .title("Import Color Palette")
            .add_file_filter_extensions("Palette (*.hex, *.gpl)", vec!["hex", "gpl"]);
        dialog.pick_file();
        self.dialog = dialog;
    }

    /// Abre o diálogo para exportar paleta de cores.
    pub fn export_palette(&mut self, default_name: &str) {
        self.pending_action = Some(FileDialogAction::ExportPalette);
        let file_name = format!("{default_name}.gpl");
        let mut dialog = FileDialog::new()
            .as_modal(true)
            .title("Export GIMP Palette")
            .add_file_filter_extensions("GIMP Palette (*.gpl)", vec!["gpl"])
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
            FileDialogAction::OpenProject => {
                if let Err(e) = ProjectService::load_project(state, &path) {
                    state.set_status(format!("open err: {e}"));
                }
            }
            FileDialogAction::SaveProject | FileDialogAction::SaveProjectAs => {
                if let Err(e) = ProjectService::save_project(state, &path) {
                    state.set_status(format!("save err: {e}"));
                }
            }
            FileDialogAction::ImportObj => {
                if let Err(e) = ProjectService::import_obj(state, &path) {
                    state.set_status(format!("import err: {e}"));
                }
            }
            FileDialogAction::ExportObj => {
                let active = state.project.active;
                match ProjectService::export_obj(state, active, &path) {
                    Ok(()) => state.set_status(format!("exported obj {}", path.display())),
                    Err(e) => state.set_status(format!("export obj err: {e}")),
                }
            }
            FileDialogAction::ExportGlb => {
                let active = state.project.active;
                match ProjectService::export_glb(state, &[active], &path) {
                    Ok(()) => state.set_status(format!("exported glb {}", path.display())),
                    Err(e) => state.set_status(format!("export glb err: {e}")),
                }
            }
            FileDialogAction::ImportPalette => {
                if let Err(e) = ProjectService::import_palette(state, &path) {
                    state.set_status(format!("import palette err: {e}"));
                }
            }
            FileDialogAction::ExportPalette => {
                match ProjectService::export_palette(&state.palette, "Petunia Palette", &path) {
                    Ok(()) => state.set_status(format!("exported palette to {}", path.display())),
                    Err(e) => state.set_status(format!("export palette err: {e}")),
                }
            }
        }
    }
}

/// Diálogos de arquivo nativos do sistema operacional (Desktop / Fallback síncrono).
pub mod native {
    use std::path::PathBuf;

    /// Abre diálogo nativo do sistema para selecionar um arquivo de projeto (.petunia).
    pub fn pick_project_file() -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter("Petunia Project (*.petunia)", &["petunia"])
            .pick_file()
    }

    /// Abre diálogo nativo para salvar um arquivo de projeto (.petunia).
    pub fn pick_save_project_file(default_name: &str) -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter("Petunia Project (*.petunia)", &["petunia"])
            .set_file_name(default_name)
            .save_file()
    }

    /// Abre diálogo nativo para selecionar malha Wavefront OBJ.
    pub fn pick_obj_file() -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter("Wavefront OBJ (*.obj)", &["obj"])
            .pick_file()
    }

    /// Abre diálogo nativo para salvar malha Wavefront OBJ.
    pub fn pick_export_obj_file(default_name: &str) -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter("Wavefront OBJ (*.obj)", &["obj"])
            .set_file_name(default_name)
            .save_file()
    }

    /// Abre diálogo nativo para salvar arquivo glTF Binário (.glb).
    pub fn pick_export_glb_file(default_name: &str) -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter("glTF Binary (*.glb)", &["glb"])
            .set_file_name(default_name)
            .save_file()
    }

    /// Abre diálogo nativo para selecionar um diretório destino.
    pub fn pick_folder() -> Option<PathBuf> {
        rfd::FileDialog::new().pick_folder()
    }

    /// Abre diálogo nativo para carregar imagens de textura/referência.
    pub fn pick_image_file() -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter(
                "Imagens (*.png, *.jpg, *.jpeg, *.webp)",
                &["png", "jpg", "jpeg", "webp"],
            )
            .pick_file()
    }

    /// Abre diálogo nativo para importar paleta (.hex ou .gpl).
    pub fn pick_palette_import_file() -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter("Paleta (*.hex, *.gpl)", &["hex", "gpl"])
            .pick_file()
    }

    /// Abre diálogo nativo para exportar paleta (.gpl).
    pub fn pick_palette_export_file(default_name: &str) -> Option<PathBuf> {
        rfd::FileDialog::new()
            .add_filter("GIMP Palette (*.gpl)", &["gpl"])
            .set_file_name(default_name)
            .save_file()
    }
}

pub use native::*;

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

        service.import_palette();
        assert_eq!(
            service.pending_action,
            Some(FileDialogAction::ImportPalette)
        );

        service.export_palette("test_palette");
        assert_eq!(
            service.pending_action,
            Some(FileDialogAction::ExportPalette)
        );
    }
}
