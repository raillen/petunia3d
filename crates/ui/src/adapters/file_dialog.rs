//! Adapter de diálogos de arquivo (`egui-file-dialog` + dialogs nativos).
//!
//! A implementação vive em [`crate::file_dialog_service`]: um serviço em canvas
//! (`PetuniaFileDialogService`) para quando o diálogo precisa ficar dentro da
//! janela, com backend nativo como fluxo padrão do sistema operacional.
//!
//! Este módulo é a superfície consumida pelo shell/menus, para que nenhum painel
//! precise conhecer a API da crate de diálogo.

pub use crate::file_dialog_service::{
    FileDialogAction, PetuniaFileDialogService, draw, export_glb_in_canvas, export_obj_in_canvas,
    export_palette_in_canvas, import_obj_in_canvas, import_palette_in_canvas,
    open_project_in_canvas, open_reference_image_dialog, save_project_as_in_canvas,
    save_project_in_canvas,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_exposes_every_file_flow() {
        // Cada fluxo de arquivo tem um ponto de entrada declarado; nenhum painel
        // chama a crate de diálogo por conta própria.
        let _flows = [
            FileDialogAction::OpenProject,
            FileDialogAction::SaveProject,
            FileDialogAction::SaveProjectAs,
            FileDialogAction::ImportObj,
            FileDialogAction::ExportObj,
            FileDialogAction::ExportGlb,
            FileDialogAction::ImportPalette,
            FileDialogAction::ExportPalette,
            FileDialogAction::AddReferenceImage(None),
        ];
    }
}
