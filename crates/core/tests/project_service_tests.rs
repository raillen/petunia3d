//! Testes de conformidade e integração headless do ProjectService no petunia_core.
//!
//! Valida o ciclo completo de persistência, importação/exportação e manipulação
//! de arquivos sem qualquer envolvimento ou dependência de UI/diálogos.
#![allow(clippy::field_reassign_with_default)]

use petunia_core::project_service::{sanitize_filename, ProjectService, ProjectServiceError};
use petunia_core::state::AppState;
use petunia_mesh::Mesh;

#[test]
fn test_new_project_resets_state() {
    let mut state = AppState::default();
    state.project.add("CustomAsset", Mesh::plane(2.0));
    state.project.project_path = Some("/tmp/fake_project.petunia".to_string());
    state
        .project
        .refs
        .push(petunia_core::ReferenceImage::from_rgba(
            "ref".to_string(),
            10,
            10,
            vec![0u8; 400],
        ));
    assert_eq!(state.project.assets.len(), 2);

    ProjectService::new_project(&mut state);

    assert_eq!(state.project.assets.len(), 1);
    assert_eq!(state.project.assets[0].name, "Cube");
    assert!(state.project.project_path.is_none());
    assert!(state.project.refs.is_empty());
    assert!(state.render.dirty);
}

#[test]
fn test_save_and_load_project_roundtrip() {
    let mut state = AppState::default();
    state.project.add("Pyramid", Mesh::cube(3.0));
    state.project.palette = vec![[0.1, 0.2, 0.3], [0.4, 0.5, 0.6]];

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!(
        "petunia_test_roundtrip_{}.petunia",
        uuid::Uuid::new_v4()
    ));

    // Salva o projeto
    ProjectService::save_project(&mut state, &file_path).expect("deve salvar projeto");
    assert_eq!(
        state.project.project_path.as_deref(),
        Some(file_path.to_str().unwrap())
    );

    // Cria um novo estado e carrega o arquivo
    let mut loaded_state = AppState::default();
    ProjectService::load_project(&mut loaded_state, &file_path).expect("deve carregar projeto");

    assert_eq!(loaded_state.project.assets.len(), 2);
    assert_eq!(loaded_state.project.assets[1].name, "Pyramid");
    assert_eq!(
        loaded_state.project.assets[1].id,
        state.project.assets[1].id
    );
    assert_eq!(
        loaded_state.project.palette,
        vec![[0.1, 0.2, 0.3], [0.4, 0.5, 0.6]]
    );
    assert_eq!(
        loaded_state.project.project_path.as_deref(),
        Some(file_path.to_str().unwrap())
    );

    let _ = std::fs::remove_file(&file_path);
}

#[test]
fn test_import_and_export_obj() {
    let mut state = AppState::default();
    let temp_dir = std::env::temp_dir();
    let obj_input_path = temp_dir.join(format!("test_in_{}.obj", uuid::Uuid::new_v4()));

    // Cria um arquivo OBJ mínimo (triângulo)
    let obj_content = "v 0.0 0.0 0.0\nv 1.0 0.0 0.0\nv 0.0 1.0 0.0\nf 1 2 3\n";
    std::fs::write(&obj_input_path, obj_content).expect("escrever obj temporario");

    // Importa OBJ
    let name = ProjectService::import_obj(&mut state, &obj_input_path).expect("importar obj");
    assert!(name.contains("test_in_"));
    assert_eq!(state.project.assets.len(), 2);

    let imported_mesh = &state.project.assets[1].mesh;
    assert_eq!(imported_mesh.vert_count(), 3);
    assert_eq!(imported_mesh.tri_count(), 1);

    // Testa se o checkpoint foi criado e pode ser desfeito
    assert!(state.undo());
    assert_eq!(state.project.assets.len(), 1);
    assert!(state.redo());
    assert_eq!(state.project.assets.len(), 2);

    // Exporta asset para novo OBJ
    let obj_output_path = temp_dir.join(format!("test_out_{}.obj", uuid::Uuid::new_v4()));
    ProjectService::export_obj(&state, 1, &obj_output_path).expect("exportar obj");

    let exported_text = std::fs::read_to_string(&obj_output_path).expect("ler obj exportado");
    assert!(exported_text.contains("v "));
    assert!(exported_text.contains("f "));

    let _ = std::fs::remove_file(&obj_input_path);
    let _ = std::fs::remove_file(&obj_output_path);
}

#[test]
fn test_export_all_obj_to_dir() {
    let mut state = AppState::default();
    state
        .project
        .add("Special Chair / Model #1", Mesh::plane(1.0));
    let temp_dir = std::env::temp_dir().join(format!("petunia_batch_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&temp_dir).expect("criar pasta de teste");

    let count = ProjectService::export_all_obj_to_dir(&state, &[0, 1], &temp_dir)
        .expect("export batch obj");
    assert_eq!(count, 2);

    assert!(temp_dir.join("Cube.obj").exists());
    assert!(
        temp_dir.join("Special_Chair___Model__1.obj").exists()
            || temp_dir.read_dir().unwrap().count() == 2
    );

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_export_glb() {
    let state = AppState::default();
    let temp_dir = std::env::temp_dir();
    let glb_path = temp_dir.join(format!("test_model_{}.glb", uuid::Uuid::new_v4()));

    ProjectService::export_glb(&state, &[0], &glb_path).expect("deve exportar glb");

    let bytes = std::fs::read(&glb_path).expect("ler glb exportado");
    assert!(bytes.starts_with(b"glTF"));
    assert!(bytes.len() > 20);

    let _ = std::fs::remove_file(&glb_path);
}

#[test]
fn test_palette_import_and_export() {
    let mut state = AppState::default();
    let temp_dir = std::env::temp_dir();
    let gpl_path = temp_dir.join(format!("palette_{}.gpl", uuid::Uuid::new_v4()));

    let sample_palette = vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    ProjectService::export_palette(&sample_palette, "TestPalette", &gpl_path)
        .expect("exportar gpl");

    let count = ProjectService::import_palette(&mut state, &gpl_path).expect("importar gpl");
    assert_eq!(count, 3);
    assert_eq!(state.project.palette.len(), 3);
    assert_eq!(state.project.project.palette.len(), 3);

    // Testa formato HEX
    let hex_path = temp_dir.join(format!("palette_{}.hex", uuid::Uuid::new_v4()));
    std::fs::write(&hex_path, "FF0000\n00FF00\n").expect("escrever hex");

    let count_hex = ProjectService::import_palette(&mut state, &hex_path).expect("importar hex");
    assert_eq!(count_hex, 2);
    assert_eq!(state.project.palette.len(), 2);

    let _ = std::fs::remove_file(&gpl_path);
    let _ = std::fs::remove_file(&hex_path);
}

#[test]
fn test_add_reference_image() {
    let mut state = AppState::default();
    assert!(state.project.refs.is_empty());

    ProjectService::add_reference_image(
        &mut state,
        "blueprint".to_string(),
        100,
        100,
        vec![255u8; 100 * 100 * 4],
    );

    assert_eq!(state.project.refs.len(), 1);
    assert_eq!(state.project.refs[0].name, "blueprint");
    assert_eq!(state.project.refs[0].width, 100);
    assert_eq!(state.project.refs[0].height, 100);
}

#[test]
fn test_error_handling() {
    let mut state = AppState::default();
    let nonexistent = std::path::Path::new("/tmp/definitely_non_existent_file_9999.petunia");

    // Load inexistente
    match ProjectService::load_project(&mut state, nonexistent) {
        Err(ProjectServiceError::Format(_)) | Err(ProjectServiceError::Io(_)) => {}
        other => panic!("Esperado erro de IO ou formato, obtido: {other:?}"),
    }

    // Export com asset_idx inválido
    let temp_dir = std::env::temp_dir();
    let dummy_path = temp_dir.join("dummy.obj");
    match ProjectService::export_obj(&state, 999, &dummy_path) {
        Err(ProjectServiceError::AssetNotFound(999)) => {}
        other => panic!("Esperado AssetNotFound(999), obtido: {other:?}"),
    }

    // Import paleta com conteúdo inválido
    let bad_palette = temp_dir.join(format!("bad_{}.hex", uuid::Uuid::new_v4()));
    std::fs::write(&bad_palette, "INVALID_HEX_DATA").expect("escrever lixo");
    match ProjectService::import_palette(&mut state, &bad_palette) {
        Err(ProjectServiceError::InvalidPalette) => {}
        other => panic!("Esperado InvalidPalette, obtido: {other:?}"),
    }
    let _ = std::fs::remove_file(&bad_palette);
}

#[test]
fn test_sanitize_filename() {
    assert_eq!(sanitize_filename("NormalName"), "NormalName");
    assert_eq!(sanitize_filename("Asset with spaces"), "Asset_with_spaces");
    assert_eq!(
        sanitize_filename("A/B\\C:D*E?F\"G<H>I|J"),
        "A_B_C_D_E_F_G_H_I_J"
    );
    assert_eq!(sanitize_filename(""), "asset");
}

#[test]
fn test_project_dirty_state_lifecycle() {
    let mut state = AppState::default();
    assert!(!state.is_document_dirty());

    // Modificação destrutiva marca dirty
    state
        .dispatch(&petunia_core::AddPrimitiveCmd::new(
            petunia_core::PrimitiveKind::Sphere,
        ))
        .unwrap();
    assert!(state.is_document_dirty());

    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("petunia_dirty_{}.petunia", uuid::Uuid::new_v4()));

    // Salvar marca limpo (clean)
    ProjectService::save_project(&mut state, &file_path).unwrap();
    assert!(
        !state.is_document_dirty(),
        "Salvar deve marcar documento como clean"
    );

    // Nova alteração marca dirty
    state
        .dispatch(&petunia_core::AddPrimitiveCmd::new(
            petunia_core::PrimitiveKind::Cone,
        ))
        .unwrap();
    assert!(state.is_document_dirty());

    // Undo até o ponto do save volta a ser clean!
    assert!(state.undo());
    assert!(
        !state.is_document_dirty(),
        "Desfazer até o ponto salvo restaura clean state"
    );

    // Redo volta a ser dirty
    assert!(state.redo());
    assert!(
        state.is_document_dirty(),
        "Refazer alteração volta a ser dirty"
    );

    // Recarregar o arquivo salvo restaura estado limpo
    ProjectService::load_project(&mut state, &file_path).unwrap();
    assert!(
        !state.is_document_dirty(),
        "Carregar projeto do disco deve iniciar limpo"
    );

    let _ = std::fs::remove_file(&file_path);
}

#[test]
fn test_recovery_from_snapshot() {
    let mut state = AppState::default();
    let dir = std::env::temp_dir().join(format!("petunia_rec_svc_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&dir).unwrap();
    let main_path = dir.join("main.petunia");
    let snap_path = dir.join("autosave-001.petunia");

    // Salva main
    ProjectService::save_project(&mut state, &main_path).unwrap();
    assert!(!state.is_document_dirty());

    // Modifica e grava snapshot
    state
        .dispatch(&petunia_core::AddPrimitiveCmd::new(
            petunia_core::PrimitiveKind::Cylinder,
        ))
        .unwrap();
    petunia_project::format::save(&state.project.project, &snap_path).unwrap();

    // Recupera a partir do snapshot num novo AppState
    let mut rec_state = AppState::default();
    ProjectService::recover_from_snapshot(&mut rec_state, &snap_path, Some(&main_path)).unwrap();

    // Recuperação carrega conteúdo do snapshot com status dirty e path do main_path
    assert!(
        rec_state.is_document_dirty(),
        "Projeto recuperado deve estar marcado como dirty"
    );
    assert_eq!(
        rec_state.project.project_path.as_deref(),
        Some(main_path.to_str().unwrap())
    );
    assert_eq!(rec_state.project.assets.len(), 2);
    assert_eq!(rec_state.project.assets[1].name, "Cylinder");

    let _ = std::fs::remove_dir_all(&dir);
}
