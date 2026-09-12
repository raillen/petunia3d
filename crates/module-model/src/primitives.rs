use petunia_core::AppState;
use petunia_mesh::Mesh;

use super::Tool;

#[derive(Default)]
pub struct PrimitivesTool;
impl Tool for PrimitivesTool {
    fn id(&self) -> &'static str {
        "primitives"
    }
    fn label_key(&self) -> &'static str {
        "tools.primitives"
    }
    fn hint_key(&self) -> &'static str {
        "hints.primitives"
    }
    fn icon(&self) -> &'static str {
        "⬢"
    }
    fn shortcut(&self) -> &'static str {
        "A"
    }
    fn ui(&self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        let l_title = state.t("tools.primitives");
        ui.label(l_title);
        let items: Vec<(&str, String)> = vec![
            ("Cube", state.t("prims.cube")),
            ("Plane", state.t("prims.plane")),
            ("Cylinder8", state.t("prims.cylinder")),
            ("Sphere", state.t("prims.sphere")),
            ("Capsule", state.t("prims.capsule")),
            ("Cone", state.t("prims.cone")),
        ];
        for (name, label) in items {
            if ui.button(label).clicked() {
                let mesh = match name {
                    "Cube" => Mesh::cube(2.0),
                    "Plane" => Mesh::plane(2.0),
                    "Cylinder8" => Mesh::cylinder(8, 1.0, 2.0),
                    "Sphere" => Mesh::sphere_low(10, 7, 1.2),
                    "Capsule" => Mesh::capsule(10, 0.6, 2.4),
                    _ => Mesh::cone(8, 1.0, 2.0),
                };
                let owned = name.to_string();
                state.checkpoint("add primitive");
                state.project.add(&owned, mesh);
                state.set_status(format!("+ {owned}"));
                state.sync_selection();
                state.emit_mesh_changed();
            }
        }
    }
}
