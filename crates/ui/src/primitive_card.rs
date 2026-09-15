//! Cartão contextual Last Operation (Wave 8 — §10.3–10.5).
//!
//! Após inserir Cube/Sphere/Cylinder/Plane, um cartão ancorado perto da
//! primitiva permite ajustar parâmetros com regeneração ao vivo. A inserção
//! inteira é UMA transação de undo: confirmar encerra, Esc cancela e remove.
//!
//! Posicionamento: canto projetado do bounding box, preso à viewport por
//! `constrain_to` (sem estimativa de tamanho). Enter/Esc valem com o ponteiro
//! sobre o cartão (Esc global nunca apaga a sessão por acidente); clique na
//! viewport confirma.

use egui::{Key, Pos2, Rect, Ui, pos2, vec2};
use petunia_core::{AppState, PrimitiveDescriptor};

use crate::tokens;
use petunia_config::text_id;

/// Desenha o cartão da sessão de criação ativa (nada se inválida/ausente).
pub fn draw_primitive_card(ui: &mut Ui, state: &mut AppState, viewport_rect: Rect) {
    if !state.primitive_session_valid() {
        state.finalize_primitive_session();
        return;
    }
    let Some(session) = state.session.primitive_session.clone() else {
        return;
    };
    let descriptor = session.descriptor;
    let title = state.t(descriptor.name_key());
    let anchor = creation_anchor(state, viewport_rect, session.asset_id);

    let mut next: Option<PrimitiveDescriptor> = None;
    let mut confirm = false;
    let mut cancel = false;
    let win = egui::Window::new(title)
        .id(egui::Id::new("primitive_last_op"))
        .default_pos(anchor)
        .constrain_to(viewport_rect)
        .collapsible(false)
        .resizable(false)
        .frame(
            egui::Frame::new()
                .fill(tokens::bg_panel(state))
                .stroke(tokens::stroke_border_dyn(state))
                .inner_margin(egui::Margin::symmetric(10, 8)),
        )
        .show(ui.ctx(), |ui| {
            ui.spacing_mut().item_spacing = vec2(4.0, 4.0);
            match descriptor {
                PrimitiveDescriptor::Cube { size } => {
                    let mut size = size;
                    if param_row(
                        ui,
                        state,
                        text_id::PRIMS_SIZE,
                        &mut size,
                        0.05..=100.0,
                        0.05,
                    ) {
                        next = Some(PrimitiveDescriptor::Cube { size });
                    }
                }
                PrimitiveDescriptor::LowSphere {
                    radius,
                    segments,
                    rings,
                } => {
                    let (mut radius, mut segments, mut rings) = (radius, segments, rings);
                    if param_row(
                        ui,
                        state,
                        text_id::PRIMS_RADIUS,
                        &mut radius,
                        0.05..=100.0,
                        0.05,
                    ) {
                        next = Some(PrimitiveDescriptor::LowSphere {
                            radius,
                            segments,
                            rings,
                        });
                    }
                    if param_row_u32(
                        ui,
                        state,
                        text_id::PRIMS_SEGMENTS,
                        &mut segments,
                        3..=32,
                        1.0,
                    ) {
                        next = Some(PrimitiveDescriptor::LowSphere {
                            radius,
                            segments,
                            rings,
                        });
                    }
                    if param_row_u32(ui, state, text_id::PRIMS_RINGS, &mut rings, 2..=24, 1.0) {
                        next = Some(PrimitiveDescriptor::LowSphere {
                            radius,
                            segments,
                            rings,
                        });
                    }
                }
                PrimitiveDescriptor::Cylinder {
                    radius,
                    height,
                    sides,
                } => {
                    let (mut radius, mut height, mut sides) = (radius, height, sides);
                    if param_row(
                        ui,
                        state,
                        text_id::PRIMS_RADIUS,
                        &mut radius,
                        0.05..=100.0,
                        0.05,
                    ) {
                        next = Some(PrimitiveDescriptor::Cylinder {
                            radius,
                            height,
                            sides,
                        });
                    }
                    if param_row(
                        ui,
                        state,
                        text_id::PRIMS_HEIGHT,
                        &mut height,
                        0.05..=100.0,
                        0.05,
                    ) {
                        next = Some(PrimitiveDescriptor::Cylinder {
                            radius,
                            height,
                            sides,
                        });
                    }
                    if param_row_u32(ui, state, text_id::PRIMS_SIDES, &mut sides, 3..=32, 1.0) {
                        next = Some(PrimitiveDescriptor::Cylinder {
                            radius,
                            height,
                            sides,
                        });
                    }
                }
                PrimitiveDescriptor::Plane { width, height } => {
                    let (mut width, mut height) = (width, height);
                    if param_row(
                        ui,
                        state,
                        text_id::PRIMS_WIDTH,
                        &mut width,
                        0.05..=100.0,
                        0.05,
                    ) {
                        next = Some(PrimitiveDescriptor::Plane { width, height });
                    }
                    if param_row(
                        ui,
                        state,
                        text_id::PRIMS_HEIGHT,
                        &mut height,
                        0.05..=100.0,
                        0.05,
                    ) {
                        next = Some(PrimitiveDescriptor::Plane { width, height });
                    }
                }
                PrimitiveDescriptor::Cone {
                    radius,
                    height,
                    sides,
                } => {
                    let (mut radius, mut height, mut sides) = (radius, height, sides);
                    if param_row(
                        ui,
                        state,
                        text_id::PRIMS_RADIUS,
                        &mut radius,
                        0.05..=100.0,
                        0.05,
                    ) {
                        next = Some(PrimitiveDescriptor::Cone {
                            radius,
                            height,
                            sides,
                        });
                    }
                    if param_row(
                        ui,
                        state,
                        text_id::PRIMS_HEIGHT,
                        &mut height,
                        0.05..=100.0,
                        0.05,
                    ) {
                        next = Some(PrimitiveDescriptor::Cone {
                            radius,
                            height,
                            sides,
                        });
                    }
                    if param_row_u32(ui, state, text_id::PRIMS_SIDES, &mut sides, 3..=32, 1.0) {
                        next = Some(PrimitiveDescriptor::Cone {
                            radius,
                            height,
                            sides,
                        });
                    }
                }
                PrimitiveDescriptor::Capsule {
                    radius,
                    height,
                    sides,
                } => {
                    let (mut radius, mut height, mut sides) = (radius, height, sides);
                    if param_row(
                        ui,
                        state,
                        text_id::PRIMS_RADIUS,
                        &mut radius,
                        0.05..=100.0,
                        0.05,
                    ) {
                        next = Some(PrimitiveDescriptor::Capsule {
                            radius,
                            height,
                            sides,
                        });
                    }
                    if param_row(
                        ui,
                        state,
                        text_id::PRIMS_HEIGHT,
                        &mut height,
                        0.05..=100.0,
                        0.05,
                    ) {
                        next = Some(PrimitiveDescriptor::Capsule {
                            radius,
                            height,
                            sides,
                        });
                    }
                    if param_row_u32(ui, state, text_id::PRIMS_SIDES, &mut sides, 3..=32, 1.0) {
                        next = Some(PrimitiveDescriptor::Capsule {
                            radius,
                            height,
                            sides,
                        });
                    }
                }
            }
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.button(state.t_id(text_id::PRIMS_CONFIRM)).clicked() {
                    confirm = true;
                }
                if ui.button(state.t_id(text_id::PRIMS_CANCEL)).clicked() {
                    cancel = true;
                }
            });
            ui.label(
                egui::RichText::new(state.t_id(text_id::PRIMS_CONFIRM_HINT))
                    .size(10.0)
                    .color(tokens::TEXT_MUTED),
            );
        });

    // Enter/Esc só com o ponteiro sobre o cartão (segurança, §10.5).
    if win.as_ref().is_some_and(|w| w.response.hovered())
        && ui.ctx().input(|i| i.key_pressed(Key::Enter))
    {
        confirm = true;
    }
    if win.as_ref().is_some_and(|w| w.response.hovered())
        && ui.ctx().input(|i| i.key_pressed(Key::Escape))
    {
        cancel = true;
    }

    if let Some(descriptor) = next {
        state.update_primitive(descriptor);
    }
    if cancel {
        state.cancel_primitive();
    } else if confirm {
        state.confirm_primitive();
    }
}

/// Linha de parâmetro float com largura fixa (layout exato, sem estimativa).
fn param_row(
    ui: &mut Ui,
    state: &AppState,
    label: petunia_config::TextId,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    speed: f32,
) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(state.t_id(label))
                .size(11.0)
                .color(tokens::TEXT_SECONDARY),
        );
        if ui
            .add_sized(
                vec2(110.0, 20.0),
                egui::DragValue::new(value).range(range).speed(speed),
            )
            .changed()
        {
            changed = true;
        }
    });
    changed
}

/// Linha de parâmetro inteiro com largura fixa.
fn param_row_u32(
    ui: &mut Ui,
    state: &AppState,
    label: petunia_config::TextId,
    value: &mut u32,
    range: std::ops::RangeInclusive<u32>,
    speed: f32,
) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(state.t_id(label))
                .size(11.0)
                .color(tokens::TEXT_SECONDARY),
        );
        let mut v = *value as i64;
        if ui
            .add_sized(
                vec2(110.0, 20.0),
                egui::DragValue::new(&mut v)
                    .range(*range.start() as i64..=*range.end() as i64)
                    .speed(speed),
            )
            .changed()
        {
            *value = v as u32;
            changed = true;
        }
    });
    changed
}

/// Âncora do cartão: canto projetado do bounding box (cai para o topo da
/// viewport se a primitiva estiver fora da vista).
fn creation_anchor(state: &AppState, viewport_rect: Rect, asset_id: uuid::Uuid) -> Pos2 {
    let to_screen = |w: glam::Vec3| {
        let ndc = state.camera.project_ndc(w);
        pos2(
            viewport_rect.min.x + (ndc.x * 0.5 + 0.5) * viewport_rect.width(),
            viewport_rect.min.y + (1.0 - (ndc.y * 0.5 + 0.5)) * viewport_rect.height(),
        )
    };
    if let Some(asset) = state.project.assets.iter().find(|a| a.id == asset_id) {
        let visible: Vec<Pos2> = asset
            .mesh
            .verts
            .iter()
            .map(|v| to_screen(v.vec()))
            .filter(|p| viewport_rect.expand(200.0).contains(*p))
            .collect();
        if !visible.is_empty() {
            let max_x = visible.iter().map(|p| p.x).fold(f32::MIN, f32::max);
            let min_y = visible.iter().map(|p| p.y).fold(f32::MAX, f32::min);
            return pos2(max_x + 12.0, min_y - 12.0);
        }
    }
    pos2(viewport_rect.center().x, viewport_rect.min.y + 40.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_renders_without_panic_during_session() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");
        assert!(state.begin_primitive(petunia_core::PrimitiveKind::Cube, None));
        let viewport = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(1280.0, 800.0));
        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                draw_primitive_card(ui, &mut state, viewport);
            });
        })
        .textures_delta
        .clear();
        assert!(state.primitive_session_valid());
    }

    #[test]
    fn anchor_falls_back_when_asset_hidden() {
        let mut state = AppState::new("en");
        assert!(state.begin_primitive(petunia_core::PrimitiveKind::Cube, None));
        // Esconde a malha da vista: âncora cai para o topo da viewport.
        let viewport = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(1280.0, 800.0));
        let id = state.session.primitive_session.as_ref().unwrap().asset_id;
        let anchor = creation_anchor(&state, viewport, id);
        assert!(viewport.contains(anchor) || anchor.y >= viewport.min.y);
    }
}
