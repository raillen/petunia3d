use petunia_core::AppState;

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
}

impl PrimitivesTool {
    /// Caminho canônico único (§82): abre a sessão de criação (transação
    /// única + cartão Last Operation) em vez de inserir a malha diretamente.
    ///
    /// Nomes legados mantidos para CLI/FFI/testes (`Cylinder8`, …).
    pub fn add_primitive(state: &mut AppState, name: &str) {
        use petunia_core::PrimitiveKind as K;
        let kind = match name {
            "Cube" => K::Cube,
            "Plane" => K::Plane,
            "Cylinder8" | "Cylinder" => K::Cylinder,
            "Sphere" => K::Sphere,
            "Capsule" => K::Capsule,
            "Cone" => K::Cone,
            "Wedge" => K::Wedge,
            "Circle" => K::Circle,
            "Torus" => K::Torus,
            "Icosphere" => K::Icosphere,
            _ => K::Cone,
        };
        state.begin_primitive(kind, Some(name.to_string()));
    }
}
