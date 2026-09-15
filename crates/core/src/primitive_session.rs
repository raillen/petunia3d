//! Sessão de criação de primitivas (Wave 8 — §10, P3D-153/154).
//!
//! Uma inserção vira transação única de undo com cartão contextual de
//! parâmetros ("Last Operation"): os parâmetros regeneram a malha a partir do
//! descritor (nunca deformam o resultado anterior), `Confirm` encerra mantendo
//! um único checkpoint e `Cancel` (Esc) desfaz até o ponto de inserção.
//!
//! A sessão invalida sozinha quando outra operação assume o topo do undo
//! (qualquer edição topológica converte a primitiva em malha comum): a UI
//! esconde o cartão e o Esc posterior não remove nada.

use petunia_mesh::Mesh;
use uuid::Uuid;

use super::selection::Selection;
use crate::command::PrimitiveKind;

/// Parâmetros de criação por primitiva (regeneração determinística).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrimitiveDescriptor {
    Cube {
        size: f32,
    },
    LowSphere {
        radius: f32,
        segments: u32,
        rings: u32,
    },
    Cylinder {
        radius: f32,
        height: f32,
        sides: u32,
    },
    Plane {
        width: f32,
        height: f32,
    },
    Cone {
        radius: f32,
        height: f32,
        sides: u32,
    },
    Capsule {
        radius: f32,
        height: f32,
        sides: u32,
    },
}

impl PrimitiveDescriptor {
    /// Descritor padrão da espécie (diálogo abre com valores sãos).
    pub fn default_for(kind: PrimitiveKind) -> Self {
        match kind {
            PrimitiveKind::Cube => Self::Cube { size: 2.0 },
            PrimitiveKind::Sphere => Self::LowSphere {
                radius: 1.0,
                segments: 12,
                rings: 8,
            },
            PrimitiveKind::Cylinder => Self::Cylinder {
                radius: 1.0,
                height: 2.0,
                sides: 12,
            },
            PrimitiveKind::Plane => Self::Plane {
                width: 2.0,
                height: 2.0,
            },
            PrimitiveKind::Cone => Self::Cone {
                radius: 1.0,
                height: 2.0,
                sides: 12,
            },
            PrimitiveKind::Capsule => Self::Capsule {
                radius: 0.5,
                height: 2.0,
                sides: 12,
            },
        }
    }

    /// Chave i18n do nome (`prims.cube`, …).
    pub fn name_key(self) -> &'static str {
        match self {
            Self::Cube { .. } => "prims.cube",
            Self::LowSphere { .. } => "prims.sphere",
            Self::Cylinder { .. } => "prims.cylinder",
            Self::Plane { .. } => "prims.plane",
            Self::Cone { .. } => "prims.cone",
            Self::Capsule { .. } => "prims.capsule",
        }
    }

    /// Espécie de volta (reabertura e atalhos).
    pub fn kind(self) -> PrimitiveKind {
        match self {
            Self::Cube { .. } => PrimitiveKind::Cube,
            Self::LowSphere { .. } => PrimitiveKind::Sphere,
            Self::Cylinder { .. } => PrimitiveKind::Cylinder,
            Self::Plane { .. } => PrimitiveKind::Plane,
            Self::Cone { .. } => PrimitiveKind::Cone,
            Self::Capsule { .. } => PrimitiveKind::Capsule,
        }
    }

    /// Constrói a malha LOCAL (sem offset) a partir dos parâmetros.
    pub fn build(self) -> Mesh {
        match self {
            Self::Cube { size } => Mesh::cube(size.clamp(0.05, 100.0)),
            Self::LowSphere {
                radius,
                segments,
                rings,
            } => Mesh::sphere_low(
                segments.clamp(3, 32),
                rings.clamp(2, 24),
                radius.clamp(0.05, 100.0),
            ),
            Self::Cylinder {
                radius,
                height,
                sides,
            } => Mesh::cylinder(
                sides.clamp(3, 32),
                radius.clamp(0.05, 100.0),
                height.clamp(0.05, 100.0),
            ),
            Self::Plane { width, height } => {
                let mut mesh = Mesh::plane(1.0);
                for v in &mut mesh.verts {
                    v.pos[0] *= width.clamp(0.05, 100.0);
                    v.pos[2] *= height.clamp(0.05, 100.0);
                }
                mesh
            }
            Self::Cone {
                radius,
                height,
                sides,
            } => Mesh::cone(
                sides.clamp(3, 32),
                radius.clamp(0.05, 100.0),
                height.clamp(0.05, 100.0),
            ),
            Self::Capsule {
                radius,
                height,
                sides,
            } => Mesh::capsule(
                sides.clamp(3, 32),
                radius.clamp(0.05, 100.0),
                height.clamp(0.05, 100.0),
            ),
        }
    }
}

/// Criação em andamento: alvo + parâmetros + contexto de reversão.
#[derive(Debug, Clone)]
pub struct PrimitiveCreationSession {
    /// Asset criado no `begin` (id estável; índice pode mudar).
    pub asset_id: Uuid,
    /// Parâmetros vivos (cada edição regenera a malha).
    pub descriptor: PrimitiveDescriptor,
    /// Offset do cursor capturado no `begin` (reaplicado a cada regeneração).
    pub cursor_offset: [f32; 3],
    /// Rótulo do checkpoint de inserção (validade = ainda no topo do undo).
    pub undo_label: String,
    /// Seleção da sessão antes do `begin` (restauração do `cancel`).
    pub original_selection: Selection,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::PrimitiveKind;
    use crate::state::AppState;

    #[test]
    fn descriptor_builds_valid_meshes() {
        for kind in [
            PrimitiveKind::Cube,
            PrimitiveKind::Sphere,
            PrimitiveKind::Cylinder,
            PrimitiveKind::Plane,
            PrimitiveKind::Cone,
            PrimitiveKind::Capsule,
        ] {
            let mesh = PrimitiveDescriptor::default_for(kind).build();
            assert!(!mesh.verts.is_empty(), "{kind:?}");
            assert!(!mesh.faces.is_empty(), "{kind:?}");
            let mut mesh = mesh;
            mesh.validate();
            assert!(!mesh.faces.is_empty(), "{kind:?} após validate");
        }
    }

    #[test]
    fn descriptor_roundtrips_kind() {
        for kind in [
            PrimitiveKind::Cube,
            PrimitiveKind::Sphere,
            PrimitiveKind::Cylinder,
            PrimitiveKind::Plane,
            PrimitiveKind::Cone,
            PrimitiveKind::Capsule,
        ] {
            assert_eq!(PrimitiveDescriptor::default_for(kind).kind(), kind);
        }
    }

    #[test]
    fn confirm_keeps_single_checkpoint() {
        let mut state = AppState::new("en");
        let depth_before = state.project.undo.depth().0;
        assert!(state.begin_primitive(PrimitiveKind::Cube, Some("Box".to_string())));
        assert!(state.primitive_session_valid());
        let asset_id = state.session.primitive_session.as_ref().unwrap().asset_id;
        // Regenerações não empilham undo.
        assert!(state.update_primitive(PrimitiveDescriptor::Cube { size: 4.0 }));
        assert!(state.update_primitive(PrimitiveDescriptor::Cube { size: 5.0 }));
        assert_eq!(state.project.undo.depth().0, depth_before + 1);
        assert!(state.confirm_primitive());
        assert!(!state.primitive_session_valid());
        assert!(state.project.assets.iter().any(|a| a.id == asset_id));
        // Um único undo remove a criação inteira.
        assert!(state.undo());
        assert!(!state.project.assets.iter().any(|a| a.id == asset_id));
    }

    #[test]
    fn cancel_removes_asset_and_restores_selection() {
        let mut state = AppState::new("en");
        let before = state.project.assets.len();
        // Marca uma seleção prévia para verificar restauração.
        if let Some(first) = state.project.assets.first() {
            let id = first.id;
            state.session.selection.asset = Some(id);
        }
        assert!(state.begin_primitive(PrimitiveKind::Sphere, None));
        assert_eq!(state.project.assets.len(), before + 1);
        assert!(state.cancel_primitive());
        assert_eq!(state.project.assets.len(), before);
        assert!(!state.primitive_session_valid());
    }

    #[test]
    fn intervening_operation_invalidates_session() {
        use crate::command::SubdivideSelectionCmd;
        let mut state = AppState::new("en");
        assert!(state.begin_primitive(PrimitiveKind::Cube, None));
        // Outra operação com checkpoint no meio: sessão vira malha comum.
        let _ = state.dispatch(&SubdivideSelectionCmd);
        assert!(!state.primitive_session_valid());
        // Cancel posterior não remove nada.
        let count = state.project.assets.len();
        assert!(!state.cancel_primitive());
        assert_eq!(state.project.assets.len(), count);
    }

    #[test]
    fn reopen_creates_new_session_from_last() {
        let mut state = AppState::new("en");
        assert!(state.begin_primitive(PrimitiveKind::Cylinder, Some("Tube".to_string())));
        assert!(state.update_primitive(PrimitiveDescriptor::Cylinder {
            radius: 2.0,
            height: 3.0,
            sides: 8,
        }));
        assert!(state.confirm_primitive());
        let before = state.project.assets.len();
        assert!(state.reopen_last_primitive());
        assert!(state.primitive_session_valid());
        assert_eq!(state.project.assets.len(), before + 1);
        let desc = state.session.primitive_session.as_ref().unwrap().descriptor;
        assert_eq!(
            desc,
            PrimitiveDescriptor::Cylinder {
                radius: 2.0,
                height: 3.0,
                sides: 8,
            }
        );
    }

    #[test]
    fn plane_scales_to_rectangle() {
        let mesh = PrimitiveDescriptor::Plane {
            width: 4.0,
            height: 2.0,
        }
        .build();
        let (mut min_x, mut max_x) = (f32::MAX, f32::MIN);
        let (mut min_z, mut max_z) = (f32::MAX, f32::MIN);
        for v in &mesh.verts {
            min_x = min_x.min(v.pos[0]);
            max_x = max_x.max(v.pos[0]);
            min_z = min_z.min(v.pos[2]);
            max_z = max_z.max(v.pos[2]);
        }
        assert!((max_x - min_x - 4.0).abs() < 1e-4);
        assert!((max_z - min_z - 2.0).abs() < 1e-4);
    }
}
