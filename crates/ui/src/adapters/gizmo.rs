//! Adapter do gizmo 3D de transformação (`transform-gizmo`).
//!
//! A implementação vive em [`crate::transform_gizmo_integration`]. Aqui fica a
//! superfície pública: o produto fala de transformação e de seleção, não de
//! `Gizmo`/`Transform` da crate.
//!
//! Exceção legítima de §36: o gizmo é uma das áreas autorizadas a calcular
//! geometria própria (desenho sobre o viewport, hit-test em espaço de tela). A
//! exceção está confinada neste adapter, não espalhada por painéis.

pub use crate::transform_gizmo_integration::{PetuniaTransformGizmo, TransformTool};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_exposes_the_transform_tools() {
        // O conjunto de ferramentas do gizmo é o contrato; a crate é detalhe.
        let tools = [
            TransformTool::Move,
            TransformTool::Rotate,
            TransformTool::Scale,
            TransformTool::Universal,
        ];
        assert_eq!(tools.len(), 4);
    }
}
