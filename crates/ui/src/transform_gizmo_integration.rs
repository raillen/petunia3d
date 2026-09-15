//! Integração do Petunia3D com `transform-gizmo-egui` para manipulação tridimensional no viewport.
//!
//! Desacopla a toolbar do executor de transformação 3D conforme especificado na arquitetura:
//! Toolbar (estado) -> Viewport (interpretação) -> TransformGizmo (execução).

use egui::{Rect, Ui};
use glam::{Mat4, Vec3};
use petunia_core::{AppState, ModalKind};
use transform_gizmo::math::Transform;
use transform_gizmo_egui::GizmoExt;
use transform_gizmo_egui::prelude::*;

/// Ferramenta de transformação desacoplada da interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformTool {
    Move,
    Rotate,
    Scale,
    Universal,
}

impl TransformTool {
    /// Determina a ferramenta ativa a partir do estado do aplicativo.
    pub fn from_app_state(state: &AppState) -> Option<Self> {
        if state.active_tool == "transform" {
            match state.gizmo_mode {
                ModalKind::Move => Some(TransformTool::Move),
                ModalKind::Rotate => Some(TransformTool::Rotate),
                ModalKind::Scale => Some(TransformTool::Scale),
                _ => None,
            }
        } else {
            match state.active_tool.as_str() {
                "move" => Some(TransformTool::Move),
                "rotate" => Some(TransformTool::Rotate),
                "scale" => Some(TransformTool::Scale),
                _ => None,
            }
        }
    }

    /// Mapeia para o conjunto de modos de gizmo do `transform-gizmo`.
    pub fn to_gizmo_modes(&self) -> EnumSet<GizmoMode> {
        match self {
            TransformTool::Move => {
                GizmoMode::TranslateX
                    | GizmoMode::TranslateY
                    | GizmoMode::TranslateZ
                    | GizmoMode::TranslateView
            }
            TransformTool::Rotate => {
                GizmoMode::RotateX | GizmoMode::RotateY | GizmoMode::RotateZ | GizmoMode::RotateView
            }
            TransformTool::Scale => {
                GizmoMode::ScaleX | GizmoMode::ScaleY | GizmoMode::ScaleZ | GizmoMode::ScaleUniform
            }
            TransformTool::Universal => GizmoMode::all(),
        }
    }
}

/// Estado persistente do gizmo no viewport.
pub struct PetuniaTransformGizmo {
    gizmo: Gizmo,
}

impl Default for PetuniaTransformGizmo {
    fn default() -> Self {
        Self::new()
    }
}

fn mat4_to_row_matrix4(m: Mat4) -> mint::RowMatrix4<f64> {
    let cols = m.to_cols_array();
    mint::RowMatrix4 {
        x: mint::Vector4 {
            x: cols[0] as f64,
            y: cols[4] as f64,
            z: cols[8] as f64,
            w: cols[12] as f64,
        },
        y: mint::Vector4 {
            x: cols[1] as f64,
            y: cols[5] as f64,
            z: cols[9] as f64,
            w: cols[13] as f64,
        },
        z: mint::Vector4 {
            x: cols[2] as f64,
            y: cols[6] as f64,
            z: cols[10] as f64,
            w: cols[14] as f64,
        },
        w: mint::Vector4 {
            x: cols[3] as f64,
            y: cols[7] as f64,
            z: cols[11] as f64,
            w: cols[15] as f64,
        },
    }
}

impl PetuniaTransformGizmo {
    pub fn new() -> Self {
        let mut gizmo = Gizmo::default();
        gizmo.update_config(GizmoConfig {
            modes: GizmoMode::all(),
            orientation: GizmoOrientation::Global,
            visuals: GizmoVisuals {
                stroke_width: 2.0,
                gizmo_size: 75.0,
                ..Default::default()
            },
            ..Default::default()
        });
        Self { gizmo }
    }

    /// Atualiza matrizes de câmera e orientação a partir da cena.
    pub fn sync_camera(&mut self, state: &AppState, viewport: Rect) {
        let view: Mat4 = state.camera.view();
        let proj: Mat4 = state.camera.proj();

        let mut config = *self.gizmo.config();
        config.view_matrix = mat4_to_row_matrix4(view);
        config.projection_matrix = mat4_to_row_matrix4(proj);
        config.viewport = viewport;

        config.orientation = match state.transform_orientation {
            petunia_core::TransformOrientation::Global => GizmoOrientation::Global,
            petunia_core::TransformOrientation::Local => GizmoOrientation::Local,
        };

        if let Some(tool) = TransformTool::from_app_state(state) {
            config.modes = tool.to_gizmo_modes();
        }

        self.gizmo.update_config(config);
    }

    /// Interage com o gizmo para o objeto ou seleção ativa.
    pub fn interact(&mut self, ui: &Ui, pivot: Vec3) -> Option<(GizmoResult, Vec<Transform>)> {
        let transform = Transform::from_scale_rotation_translation(
            mint::Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            mint::Quaternion {
                v: mint::Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                s: 1.0,
            },
            mint::Vector3 {
                x: pivot.x as f64,
                y: pivot.y as f64,
                z: pivot.z as f64,
            },
        );
        self.gizmo.interact(ui, &[transform])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petunia_core::AppState;

    #[test]
    fn test_transform_tool_mapping() {
        let mut state = AppState::new("en");
        state.active_tool = "transform".into();
        state.gizmo_mode = ModalKind::Move;

        assert_eq!(
            TransformTool::from_app_state(&state),
            Some(TransformTool::Move)
        );

        state.gizmo_mode = ModalKind::Rotate;
        assert_eq!(
            TransformTool::from_app_state(&state),
            Some(TransformTool::Rotate)
        );

        state.gizmo_mode = ModalKind::Scale;
        assert_eq!(
            TransformTool::from_app_state(&state),
            Some(TransformTool::Scale)
        );
    }

    #[test]
    fn test_gizmo_modes_conversion() {
        let tool = TransformTool::Move;
        let modes = tool.to_gizmo_modes();
        assert!(modes.contains(GizmoMode::TranslateX));
        assert!(!modes.contains(GizmoMode::RotateX));
    }

    #[test]
    fn test_petunia_transform_gizmo_instantiation() {
        let mut gizmo = PetuniaTransformGizmo::new();
        let state = AppState::new("en");
        let viewport = Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(800.0, 600.0));
        gizmo.sync_camera(&state, viewport);
    }
}
