//! Testes unitários do Material System (P3D-050 a P3D-054, P3D-140).

use petunia_project::{
    AlphaMode, Canvas, Material, Project, ShaderProfile, TextureChannel, format,
};
use uuid::Uuid;

#[test]
fn test_material_defaults_and_creation() {
    let mat = Material::new("Gold PBR");
    assert_eq!(mat.name, "Gold PBR");
    assert_eq!(mat.base_color, [0.75, 0.75, 0.78, 1.0]);
    assert_eq!(mat.roughness, 0.5);
    assert_eq!(mat.metallic, 0.0);
    assert_eq!(mat.profile, ShaderProfile::Pbr);
    assert_eq!(mat.alpha_mode, AlphaMode::Opaque);
    assert!(mat.albedo_texture.is_none());
}

#[test]
fn test_material_with_color_and_duplicate() {
    let mat = Material::with_color("Ruby", [0.9, 0.1, 0.15, 1.0]);
    assert_eq!(mat.base_color, [0.9, 0.1, 0.15, 1.0]);

    let dup = mat.duplicate();
    assert_ne!(mat.id, dup.id);
    assert_eq!(dup.name, "Ruby Copy");
    assert_eq!(dup.base_color, mat.base_color);
}

#[test]
fn test_material_channels_and_sampling() {
    let mut mat = Material::new("Textured Wood");
    assert!(mat.channel_texture(TextureChannel::Albedo).is_none());

    // Cria e garante canvas
    let cv = mat.ensure_channel_canvas(TextureChannel::Albedo, 2, 2, [200, 100, 50, 255]);
    assert_eq!(cv.w, 2);
    assert_eq!(cv.h, 2);
    assert!(mat.channel_texture(TextureChannel::Albedo).is_some());

    // Amostragem de albedo
    let sampled = mat.sample_albedo([0.25, 0.25]);
    assert!((sampled[0] - (200.0 / 255.0) * mat.base_color[0]).abs() < 1e-4);
}

#[test]
fn test_material_glossiness_conversion() {
    let mut mat = Material::new("Glossy Car Paint");
    mat.set_glossiness(0.8);
    assert!((mat.roughness - 0.2).abs() < 1e-4);
    assert!((mat.glossiness() - 0.8).abs() < 1e-4);
}

#[test]
fn test_material_validation_sanitizes_floats() {
    let mut mat = Material::new("Hazard Mat");
    mat.base_color = [f32::NAN, 2.5, -0.5, f32::INFINITY];
    mat.roughness = f32::NAN;
    mat.metallic = 999.0;
    mat.normal_scale = -1.0;
    mat.emission_strength = -5.0;
    mat.validate();

    assert_eq!(mat.base_color[0], 1.0);
    assert_eq!(mat.base_color[1], 1.0);
    assert_eq!(mat.base_color[2], 0.0);
    assert_eq!(mat.base_color[3], 1.0);
    assert_eq!(mat.roughness, 0.5);
    assert_eq!(mat.metallic, 1.0);
    assert_eq!(mat.normal_scale, 0.0);
    assert_eq!(mat.emission_strength, 0.0);
}

#[test]
fn test_project_material_management() {
    let mut project = Project::new();
    assert_eq!(project.materials.len(), 1);
    let default_id = project.materials[0].id;

    // Asset padrão deve estar associado ao material padrão
    assert_eq!(project.assets[0].material_id, Some(default_id));
    assert_eq!(project.active_material().unwrap().id, default_id);

    // Adiciona novo material
    let copper = Material::with_color("Copper", [0.95, 0.64, 0.54, 1.0]);
    let copper_id = project.add_material(copper);
    assert_eq!(project.materials.len(), 2);

    // Atribui copper ao asset
    project.assets[0].material_id = Some(copper_id);
    assert_eq!(project.active_material().unwrap().name, "Copper");

    // Remove copper -> asset material_id reseta para None
    assert!(project.remove_material(copper_id));
    assert_eq!(project.materials.len(), 1);
    assert_eq!(project.assets[0].material_id, None);
}

#[test]
fn test_project_validate_ensures_valid_material_association() {
    let mut project = Project::new();
    project.materials.clear();
    project.assets[0].material_id = Some(Uuid::new_v4()); // ID órfão

    project.validate();
    assert_eq!(project.materials.len(), 1);
    let fallback_id = project.materials[0].id;
    assert_eq!(project.assets[0].material_id, Some(fallback_id));
}

#[test]
fn test_material_roundtrip_persistence() {
    let temp = assert_fs::TempDir::new().unwrap();
    let file_path = temp.path().join("material_test.petunia");

    let mut project = Project::new();
    let mut mat = Material::with_color("Emerald", [0.1, 0.8, 0.3, 1.0]);
    mat.profile = ShaderProfile::Toon;
    mat.metallic = 0.2;
    mat.roughness = 0.4;
    mat.albedo_texture = Some(Canvas::new(4, 4, [10, 80, 30, 255]));
    let mat_id = project.add_material(mat);
    project.assets[0].material_id = Some(mat_id);

    format::save(&project, &file_path).expect("salva projeto com material");
    let loaded = format::load(&file_path).expect("carrega projeto com material");

    assert_eq!(loaded.materials.len(), 2);
    let loaded_mat = loaded
        .get_material(mat_id)
        .expect("encontra material salvo");
    assert_eq!(loaded_mat.name, "Emerald");
    assert_eq!(loaded_mat.profile, ShaderProfile::Toon);
    assert_eq!(loaded_mat.base_color, [0.1, 0.8, 0.3, 1.0]);
    assert_eq!(loaded_mat.metallic, 0.2);
    assert_eq!(loaded_mat.roughness, 0.4);
    assert!(loaded_mat.albedo_texture.is_some());
    assert_eq!(loaded.assets[0].material_id, Some(mat_id));
}
