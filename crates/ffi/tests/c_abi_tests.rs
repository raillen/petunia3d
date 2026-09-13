//! Testes unitários e de integração para a Camada C-ABI / FFI (Gauntlet G10).
//!
//! Exercita todas as funções `extern "C"` exportadas por `petunia_ffi` através de
//! ponteiros brutos C-ABI, validando integridade de memória, códigos de erro,
//! ausência de vazamentos, serialização JSON e operações de modelagem.

use std::ffi::{CStr, CString};
use std::fs;
use std::path::PathBuf;
use std::ptr;

use petunia_ffi::*;

fn temp_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("petunia_ffi_test_{}_{}", std::process::id(), name))
}

#[test]
fn test_context_lifecycle_and_null_safety() {
    unsafe {
        // Criar contexto com idioma nulo (fallback "en")
        let ctx = petunia_context_create(ptr::null());
        assert!(!ctx.is_null(), "Contexto não deve ser nulo");

        // Destruir contexto
        petunia_context_destroy(ctx);

        // Destruição de ponteiro nulo não deve causar pânico
        petunia_context_destroy(ptr::null_mut());

        // Criar contexto com idioma explícito
        let lang = CString::new("pt-BR").unwrap();
        let ctx2 = petunia_context_create(lang.as_ptr());
        assert!(!ctx2.is_null());
        petunia_context_destroy(ctx2);
    }
}

#[test]
fn test_null_pointer_error_codes() {
    unsafe {
        let null_ctx = ptr::null_mut();
        let dummy = CString::new("test").unwrap();

        assert_eq!(petunia_new_project(null_ctx), PETUNIA_ERR_NULL_PTR);
        assert_eq!(
            petunia_load_project(null_ctx, dummy.as_ptr()),
            PETUNIA_ERR_NULL_PTR
        );
        assert_eq!(
            petunia_save_project(null_ctx, dummy.as_ptr()),
            PETUNIA_ERR_NULL_PTR
        );
        assert_eq!(
            petunia_import_obj(null_ctx, dummy.as_ptr()),
            PETUNIA_ERR_NULL_PTR
        );
        assert_eq!(
            petunia_export_obj(null_ctx, dummy.as_ptr()),
            PETUNIA_ERR_NULL_PTR
        );
        assert_eq!(
            petunia_export_glb(null_ctx, dummy.as_ptr()),
            PETUNIA_ERR_NULL_PTR
        );
        assert_eq!(
            petunia_add_primitive(null_ctx, dummy.as_ptr()),
            PETUNIA_ERR_NULL_PTR
        );
        assert_eq!(petunia_undo(null_ctx), PETUNIA_ERR_NULL_PTR);
        assert_eq!(petunia_redo(null_ctx), PETUNIA_ERR_NULL_PTR);
        assert_eq!(petunia_select_all(null_ctx), PETUNIA_ERR_NULL_PTR);
        assert_eq!(petunia_clear_selection(null_ctx), PETUNIA_ERR_NULL_PTR);
        assert_eq!(petunia_delete_selection(null_ctx), PETUNIA_ERR_NULL_PTR);
        assert_eq!(petunia_duplicate_selection(null_ctx), PETUNIA_ERR_NULL_PTR);
        assert_eq!(petunia_get_asset_count(null_ctx), PETUNIA_ERR_NULL_PTR);
        assert_eq!(
            petunia_get_scene_summary(null_ctx, ptr::null_mut(), ptr::null_mut()),
            PETUNIA_ERR_NULL_PTR
        );

        let mut buf = [0i8; 64];
        assert_eq!(
            petunia_get_active_asset_name(null_ctx, buf.as_mut_ptr(), buf.len()),
            PETUNIA_ERR_NULL_PTR
        );
    }
}

#[test]
fn test_modeling_primitives_and_scene_queries() {
    unsafe {
        let ctx = petunia_context_create(ptr::null());
        assert!(!ctx.is_null());

        // Projeto novo possui 1 cubo
        assert_eq!(petunia_get_asset_count(ctx), 1);

        let mut name_buf = [0i8; 64];
        assert_eq!(
            petunia_get_active_asset_name(ctx, name_buf.as_mut_ptr(), name_buf.len()),
            PETUNIA_OK
        );
        let active_name = CStr::from_ptr(name_buf.as_ptr()).to_str().unwrap();
        assert_eq!(active_name, "Cube");

        let mut verts = 0u32;
        let mut faces = 0u32;
        assert_eq!(
            petunia_get_active_asset_stats(ctx, &mut verts, &mut faces),
            PETUNIA_OK
        );
        assert_eq!(verts, 8);
        assert_eq!(faces, 6);

        // Adiciona Esfera e Cilindro
        let sphere_name = CString::new("Sphere").unwrap();
        assert_eq!(petunia_add_primitive(ctx, sphere_name.as_ptr()), PETUNIA_OK);

        let cylinder_name = CString::new("Cylinder8").unwrap();
        assert_eq!(
            petunia_add_primitive(ctx, cylinder_name.as_ptr()),
            PETUNIA_OK
        );

        assert_eq!(petunia_get_asset_count(ctx), 3);

        let mut total_verts = 0u32;
        let mut total_faces = 0u32;
        assert_eq!(
            petunia_get_scene_summary(ctx, &mut total_verts, &mut total_faces),
            PETUNIA_OK
        );
        assert!(total_verts > 24);
        assert!(total_faces > 12);

        // Desfazer (Undo)
        assert_eq!(petunia_undo(ctx), PETUNIA_OK);
        assert_eq!(petunia_get_asset_count(ctx), 2);

        // Refazer (Redo)
        assert_eq!(petunia_redo(ctx), PETUNIA_OK);
        assert_eq!(petunia_get_asset_count(ctx), 3);

        petunia_context_destroy(ctx);
    }
}

#[test]
fn test_json_dto_queries() {
    unsafe {
        let ctx = petunia_context_create(ptr::null());
        assert!(!ctx.is_null());

        let plane_name = CString::new("Plane").unwrap();
        petunia_add_primitive(ctx, plane_name.as_ptr());

        // Buffer muito pequeno deve retornar PETUNIA_ERR_BUFFER_TOO_SMALL
        let mut small_buf = [0i8; 10];
        assert_eq!(
            petunia_query_scene_hierarchy_json(ctx, small_buf.as_mut_ptr(), small_buf.len()),
            PETUNIA_ERR_BUFFER_TOO_SMALL
        );

        // Buffer adequado
        let mut json_buf = vec![0i8; 8192];
        assert_eq!(
            petunia_query_scene_hierarchy_json(ctx, json_buf.as_mut_ptr(), json_buf.len()),
            PETUNIA_OK
        );

        let json_str = CStr::from_ptr(json_buf.as_ptr()).to_str().unwrap();
        assert!(json_str.contains("\"assets\":"));
        assert!(json_str.contains("\"Plane\""));
        assert!(json_str.contains("\"Cube\""));

        // Query selection details
        let mut sel_buf = vec![0i8; 4096];
        assert_eq!(
            petunia_query_selection_details_json(ctx, sel_buf.as_mut_ptr(), sel_buf.len()),
            PETUNIA_OK
        );
        let sel_str = CStr::from_ptr(sel_buf.as_ptr()).to_str().unwrap();
        assert!(sel_str.contains("\"selected_verts_count\""));

        // Query tool status
        let mut tool_buf = vec![0i8; 4096];
        assert_eq!(
            petunia_query_tool_status_json(ctx, tool_buf.as_mut_ptr(), tool_buf.len()),
            PETUNIA_OK
        );
        let tool_str = CStr::from_ptr(tool_buf.as_ptr()).to_str().unwrap();
        assert!(tool_str.contains("\"active_tool\""));

        petunia_context_destroy(ctx);
    }
}

#[test]
fn test_project_io_and_obj_glb_exports() {
    unsafe {
        let ctx = petunia_context_create(ptr::null());
        assert!(!ctx.is_null());

        let sphere = CString::new("Sphere").unwrap();
        petunia_add_primitive(ctx, sphere.as_ptr());

        let prj_path = temp_path("c_abi_export.petunia");
        let obj_path = temp_path("c_abi_export.obj");
        let glb_path = temp_path("c_abi_export.glb");

        let prj_c = CString::new(prj_path.to_str().unwrap()).unwrap();
        let obj_c = CString::new(obj_path.to_str().unwrap()).unwrap();
        let glb_c = CString::new(glb_path.to_str().unwrap()).unwrap();

        // Salva projeto
        assert_eq!(petunia_save_project(ctx, prj_c.as_ptr()), PETUNIA_OK);
        assert!(prj_path.exists());

        // Exporta OBJ
        assert_eq!(petunia_export_obj(ctx, obj_c.as_ptr()), PETUNIA_OK);
        assert!(obj_path.exists());
        let obj_content = fs::read_to_string(&obj_path).unwrap();
        assert!(obj_content.contains("v ") && obj_content.contains("f "));

        // Exporta GLB
        assert_eq!(petunia_export_glb(ctx, glb_c.as_ptr()), PETUNIA_OK);
        assert!(glb_path.exists());
        let glb_bytes = fs::read(&glb_path).unwrap();
        assert_eq!(&glb_bytes[0..4], b"glTF", "Magic bytes de GLB válidos");

        // Novo projeto e recarrega projeto salvo
        assert_eq!(petunia_new_project(ctx), PETUNIA_OK);
        assert_eq!(petunia_get_asset_count(ctx), 1);

        assert_eq!(petunia_load_project(ctx, prj_c.as_ptr()), PETUNIA_OK);
        assert_eq!(petunia_get_asset_count(ctx), 2);

        // Limpeza
        let _ = fs::remove_file(prj_path);
        let _ = fs::remove_file(obj_path);
        let _ = fs::remove_file(glb_path);

        petunia_context_destroy(ctx);
    }
}

#[test]
fn test_mesh_transforms_and_subdivision() {
    unsafe {
        let ctx = petunia_context_create(ptr::null());
        assert!(!ctx.is_null());

        let mut verts_before = 0u32;
        let mut faces_before = 0u32;
        petunia_get_active_asset_stats(ctx, &mut verts_before, &mut faces_before);
        assert_eq!(verts_before, 8);

        // Selecionar tudo e subdividir
        assert_eq!(petunia_select_all(ctx), PETUNIA_OK);
        assert_eq!(petunia_subdivide_selection(ctx), PETUNIA_OK);

        let mut verts_after = 0u32;
        let mut faces_after = 0u32;
        petunia_get_active_asset_stats(ctx, &mut verts_after, &mut faces_after);
        assert!(
            verts_after > verts_before,
            "Vértices devem aumentar após subdivisão"
        );
        assert!(
            faces_after > faces_before,
            "Faces devem aumentar após subdivisão"
        );

        // Escalar
        assert_eq!(petunia_scale_selection(ctx, 2.0), PETUNIA_OK);

        // Extrudar
        assert_eq!(petunia_extrude_selection(ctx, 1.5), PETUNIA_OK);

        petunia_context_destroy(ctx);
    }
}

#[test]
fn test_uuid_activation_deletion_and_error_message() {
    unsafe {
        let ctx = petunia_context_create(ptr::null());
        assert!(!ctx.is_null());

        let cone = CString::new("Cone").unwrap();
        petunia_add_primitive(ctx, cone.as_ptr());

        // Obtém a hierarquia para recuperar o UUID do Cone
        let mut json_buf = vec![0i8; 8192];
        petunia_query_scene_hierarchy_json(ctx, json_buf.as_mut_ptr(), json_buf.len());
        let json_str = CStr::from_ptr(json_buf.as_ptr()).to_str().unwrap();

        let hierarchy: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let cone_uuid_str = hierarchy["assets"][1]["id"].as_str().unwrap();
        let cone_uuid_c = CString::new(cone_uuid_str).unwrap();

        // Ativação por UUID
        assert_eq!(
            petunia_set_active_asset_by_id(ctx, cone_uuid_c.as_ptr()),
            PETUNIA_OK
        );

        let mut active_buf = [0i8; 64];
        petunia_get_active_asset_name(ctx, active_buf.as_mut_ptr(), active_buf.len());
        assert_eq!(
            CStr::from_ptr(active_buf.as_ptr()).to_str().unwrap(),
            "Cone"
        );

        // Deleta por UUID
        assert_eq!(
            petunia_delete_asset_by_id(ctx, cone_uuid_c.as_ptr()),
            PETUNIA_OK
        );
        assert_eq!(petunia_get_asset_count(ctx), 1);

        // Tentativa de deletar UUID inexistente deve retornar NOT_FOUND e registrar mensagem de erro
        assert_eq!(
            petunia_delete_asset_by_id(ctx, cone_uuid_c.as_ptr()),
            PETUNIA_ERR_NOT_FOUND
        );

        let mut err_buf = [0i8; 256];
        assert_eq!(
            petunia_last_error_message(err_buf.as_mut_ptr(), err_buf.len()),
            PETUNIA_OK
        );
        let err_msg = CStr::from_ptr(err_buf.as_ptr()).to_str().unwrap();
        assert!(!err_msg.is_empty(), "Mensagem de erro deve ser populada");
        assert!(err_msg.contains("não encontrado"));

        petunia_context_destroy(ctx);
    }
}
