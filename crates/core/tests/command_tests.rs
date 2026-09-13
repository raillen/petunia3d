//! Testes do sistema de comandos (CommandDispatcher e Comandos Básicos) do Petunia3D.
//! Garante a execução headless e o ciclo transacional de Undo/Redo sem qualquer dependência de UI.
#![allow(clippy::field_reassign_with_default)]

use petunia_core::command::{
    AddPrimitiveCmd, ClearSelectionCmd, CommandDispatcher, CommandError, DeleteAssetCmd,
    DeleteSelectionCmd, DuplicateAssetCmd, DuplicateSelectionCmd, InvertSelectionCmd,
    PrimitiveKind, SelectAllCmd,
};
use petunia_core::state::{AppState, EditMode};

#[test]
fn test_add_primitive_commands_and_undo_redo() {
    let mut state = AppState::default();
    assert_eq!(state.project.assets.len(), 1); // Cubo padrão inicial

    // Adiciona Sphere
    let add_sphere = AddPrimitiveCmd::new(PrimitiveKind::Sphere);
    state.dispatch(&add_sphere).expect("deve adicionar esfera");
    assert_eq!(state.project.assets.len(), 2);
    assert!(state.project.assets[1].name.contains("Sphere"));

    // Adiciona Cylinder
    let add_cyl = AddPrimitiveCmd::new(PrimitiveKind::Cylinder);
    state.dispatch(&add_cyl).expect("deve adicionar cilindro");
    assert_eq!(state.project.assets.len(), 3);

    // Adiciona Plane
    let add_plane = AddPrimitiveCmd::new(PrimitiveKind::Plane);
    state.dispatch(&add_plane).expect("deve adicionar plano");
    assert_eq!(state.project.assets.len(), 4);

    // Adiciona Cone
    let add_cone = AddPrimitiveCmd::new(PrimitiveKind::Cone);
    state.dispatch(&add_cone).expect("deve adicionar cone");
    assert_eq!(state.project.assets.len(), 5);

    // Adiciona Capsule
    let add_capsule = AddPrimitiveCmd::new(PrimitiveKind::Capsule);
    state
        .dispatch(&add_capsule)
        .expect("deve adicionar cápsula");
    assert_eq!(state.project.assets.len(), 6);

    // Testa Undo em cadeia
    assert!(state.undo());
    assert_eq!(state.project.assets.len(), 5);

    assert!(state.undo());
    assert_eq!(state.project.assets.len(), 4);

    // Testa Redo em cadeia
    assert!(state.redo());
    assert_eq!(state.project.assets.len(), 5);

    assert!(state.redo());
    assert_eq!(state.project.assets.len(), 6);
}

#[test]
fn test_duplicate_and_delete_asset_cmd() {
    let mut state = AppState::default();
    assert_eq!(state.project.assets.len(), 1);

    // Duplica o cubo ativo
    let dup_cmd = DuplicateAssetCmd { asset_index: None };
    state.dispatch(&dup_cmd).expect("deve duplicar asset ativo");
    assert_eq!(state.project.assets.len(), 2);
    assert_eq!(state.project.assets[1].name, "Cube copy");

    // Undo restaura para 1 asset
    assert!(state.undo());
    assert_eq!(state.project.assets.len(), 1);

    // Redo volta para 2 assets
    assert!(state.redo());
    assert_eq!(state.project.assets.len(), 2);

    // Deleta o asset recém-duplicado
    let del_cmd = DeleteAssetCmd {
        asset_index: Some(1),
    };
    state
        .dispatch(&del_cmd)
        .expect("deve deletar asset no índice 1");
    assert_eq!(state.project.assets.len(), 1);

    // Undo restaura o asset deletado
    assert!(state.undo());
    assert_eq!(state.project.assets.len(), 2);

    // Erro ao tentar deletar índice inexistente
    let invalid_del = DeleteAssetCmd {
        asset_index: Some(999),
    };
    let err = state.dispatch(&invalid_del).unwrap_err();
    assert_eq!(err, CommandError::InvalidAssetIndex(999));
}

#[test]
fn test_delete_selection_in_edit_and_object_modes() {
    let mut state = AppState::default();

    // 1. Em Object Mode, DeleteSelectionCmd remove o asset
    state.mode = EditMode::Object;
    let add_cyl = AddPrimitiveCmd::new(PrimitiveKind::Cylinder);
    state.dispatch(&add_cyl).expect("adiciona cilindro");
    assert_eq!(state.project.assets.len(), 2);

    let del_sel = DeleteSelectionCmd;
    state.dispatch(&del_sel).expect("deleta asset ativo");
    assert_eq!(state.project.assets.len(), 1);

    assert!(state.undo());
    assert_eq!(state.project.assets.len(), 2);

    // 2. Em Edit Mode, DeleteSelectionCmd remove sub-elementos da malha
    state.mode = EditMode::Edit;
    let initial_verts = state.project.active_mesh().unwrap().verts.len();
    assert!(initial_verts > 0);

    // Seleciona tudo e deleta
    state.dispatch(&SelectAllCmd).expect("seleciona tudo");
    state
        .dispatch(&del_sel)
        .expect("deleta geometria selecionada");

    assert_eq!(state.project.active_mesh().unwrap().verts.len(), 0);

    // Undo restaura a geometria da malha ativa
    assert!(state.undo());
    assert_eq!(
        state.project.active_mesh().unwrap().verts.len(),
        initial_verts
    );
}

#[test]
fn test_duplicate_selection_in_edit_mode() {
    let mut state = AppState::default();
    state.mode = EditMode::Edit;

    let initial_verts = state.project.active_mesh().unwrap().verts.len();
    assert_eq!(initial_verts, 8); // Cubo

    // Seleciona tudo
    state.dispatch(&SelectAllCmd).expect("seleciona tudo");

    // Duplica geometria selecionada
    let dup_sel = DuplicateSelectionCmd;
    state.dispatch(&dup_sel).expect("duplica geometria");

    let new_verts = state.project.active_mesh().unwrap().verts.len();
    assert_eq!(new_verts, 16); // 8 originais + 8 duplicados

    // Undo restaura para 8
    assert!(state.undo());
    assert_eq!(state.project.active_mesh().unwrap().verts.len(), 8);

    // Redo re-aplica duplicação para 16
    assert!(state.redo());
    assert_eq!(state.project.active_mesh().unwrap().verts.len(), 16);
}

#[test]
fn test_selection_commands_are_non_destructive() {
    let mut state = AppState::default();
    state.mode = EditMode::Edit;

    let mesh = state.project.active_mesh_mut().unwrap();
    mesh.deselect_all();
    assert!(mesh.verts.iter().all(|v| !v.selected));

    // SelectAllCmd
    state.dispatch(&SelectAllCmd).expect("select all");
    let mesh = state.project.active_mesh().unwrap();
    assert!(mesh.verts.iter().all(|v| v.selected));

    // ClearSelectionCmd
    state.dispatch(&ClearSelectionCmd).expect("clear selection");
    let mesh = state.project.active_mesh().unwrap();
    assert!(mesh.verts.iter().all(|v| !v.selected));

    // InvertSelectionCmd
    let mesh = state.project.active_mesh_mut().unwrap();
    mesh.verts[0].selected = true;
    state
        .dispatch(&InvertSelectionCmd)
        .expect("invert selection");
    let mesh = state.project.active_mesh().unwrap();
    assert!(!mesh.verts[0].selected);
    assert!(mesh.verts[1..].iter().all(|v| v.selected));

    // Seleção não é destrutiva: undo() retorna false porque nenhuma operação destrutiva foi feita
    assert!(!state.undo());
}

#[test]
fn test_command_dispatcher_registry() {
    let mut state = AppState::default();
    let mut dispatcher = CommandDispatcher::new();

    dispatcher.register(
        "add_sphere",
        Box::new(AddPrimitiveCmd::new(PrimitiveKind::Sphere)),
    );
    dispatcher.register("select_all", Box::new(SelectAllCmd));

    // Executa comando registrado via dispatcher
    dispatcher
        .execute("add_sphere", &mut state)
        .expect("deve executar add_sphere");
    assert_eq!(state.project.assets.len(), 2);

    // Executa seleção via dispatcher
    dispatcher
        .execute("select_all", &mut state)
        .expect("deve executar select_all");
    let mesh = state.project.active_mesh().unwrap();
    assert!(mesh.verts.iter().all(|v| v.selected));

    // Comando não registrado deve retornar erro
    let err = dispatcher
        .execute("unknown_command", &mut state)
        .unwrap_err();
    assert!(matches!(err, CommandError::Execution(_)));
}

#[test]
fn test_headless_full_modeling_session() {
    // Prova de execução 100% headless sem qualquer binding de UI
    let mut state = AppState::default();

    // 1. Adiciona um cilindro
    state
        .dispatch(&AddPrimitiveCmd::new(PrimitiveKind::Cylinder))
        .expect("adiciona cilindro");
    assert_eq!(state.project.assets.len(), 2);

    // 2. Entra em modo de edição
    state.mode = EditMode::Edit;

    // 3. Seleciona toda a malha e duplica
    state.dispatch(&SelectAllCmd).expect("seleciona");
    let initial_count = state.project.active_mesh().unwrap().verts.len();
    state.dispatch(&DuplicateSelectionCmd).expect("duplica");
    assert_eq!(
        state.project.active_mesh().unwrap().verts.len(),
        initial_count * 2
    );

    // 4. Inverte seleção e limpa
    state.dispatch(&InvertSelectionCmd).expect("inverte");
    state.dispatch(&ClearSelectionCmd).expect("limpa");
    assert_eq!(
        state
            .project
            .active_mesh()
            .unwrap()
            .verts
            .iter()
            .filter(|v| v.selected)
            .count(),
        0
    );

    // 5. Undo desfaz a duplicação
    assert!(state.undo());
    assert_eq!(
        state.project.active_mesh().unwrap().verts.len(),
        initial_count
    );

    // 6. Undo desfaz a criação do cilindro
    assert!(state.undo());
    assert_eq!(state.project.assets.len(), 1);
}
