//! Gerenciador visual de conjuntos de imagens de referência (Reference Sets / P3D-013 & P3D-014).
//!
//! Permite atribuir, substituir, pré-visualizar e calibrar imagens de referência
//! independentes para os 6 slots ortográficos canônicos (Front, Back, Left, Right, Top, Bottom).

#![forbid(unsafe_code)]

use egui::{Color32, Margin, Pos2, Rect, Stroke, Ui, vec2};
use petunia_core::project_service::ProjectService;
use petunia_core::{AppState, RefAxis, ViewPreset};

use crate::file_dialog_service;
use crate::icon_registry::{IconRegistry, PetuniaIcon};
use crate::tokens;

const SLOTS: [(RefAxis, &str, &str, ViewPreset); 6] = [
    (RefAxis::Front, "Front", "+Z (Frente)", ViewPreset::Front),
    (RefAxis::Back, "Back", "-Z (Traseira)", ViewPreset::Back),
    (RefAxis::Left, "Left", "-X (Esquerda)", ViewPreset::Left),
    (RefAxis::Right, "Right", "+X (Direita)", ViewPreset::Right),
    (RefAxis::Top, "Top", "+Y (Superior)", ViewPreset::Top),
    (
        RefAxis::Bottom,
        "Bottom",
        "-Y (Inferior)",
        ViewPreset::Bottom,
    ),
];

/// Renderiza a janela utilitária do Gerenciador de Referências (P3D-013).
pub fn draw(ctx: &egui::Context, state: &mut AppState) {
    puffin::profile_function!();
    if !state.ui.show_reference_manager {
        return;
    }

    let mut open = state.ui.show_reference_manager;
    let mut remove_index = None;
    let mut align_preset = None;

    let screen_rect = ctx.viewport_rect();
    let max_w = (screen_rect.width() - 32.0).max(540.0);
    let max_h = (screen_rect.height() - 32.0).max(420.0);

    egui::Window::new(format!("{} · P3D-013", state.t("refs.manager_title")))
        .open(&mut open)
        .default_size(vec2(720.0, 560.0))
        .min_size(vec2(540.0, 420.0))
        .max_size(vec2(max_w, max_h))
        .collapsible(false)
        .resizable(true)
        .show(ctx, |ui| {
            // Cabeçalho descritivo e ações globais
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(state.t("refs.manager_desc"))
                        .color(tokens::TEXT_SECONDARY)
                        .size(12.0),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(format!("🗑 {}", state.t("refs.clear_all")))
                        .on_hover_text(state.t("refs.clear_all_tooltip"))
                        .clicked()
                    {
                        ProjectService::clear_references(state);
                    }

                    if ui
                        .button(format!("+ {}", state.t("refs.add_custom")))
                        .on_hover_text(state.t("refs.add_custom_tooltip"))
                        .clicked()
                    {
                        crate::pick_and_add_reference_image(state);
                    }
                });
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // Grade flexível de cartões dos 6 slots ortográficos canônicos
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = vec2(14.0, 14.0);
                        for &(axis, label, coord_hint, preset) in SLOTS.iter() {
                            let matching_idx = state.project.refs.iter().position(|r| {
                                r.axis == axis
                                    || (axis == RefAxis::Right && r.axis == RefAxis::Side)
                            });

                            draw_slot_card(
                                ui,
                                ctx,
                                state,
                                axis,
                                label,
                                coord_hint,
                                preset,
                                matching_idx,
                                &mut remove_index,
                                &mut align_preset,
                            );
                        }
                    });
                });

            ui.add_space(10.0);
            ui.separator();
            ui.horizontal(|ui| {
                let total = state.project.refs.len();
                ui.label(
                    egui::RichText::new(format!("{total} referência(s) carregada(s)"))
                        .color(tokens::text_muted(state))
                        .size(11.0),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(state.t("ui.close")).clicked() {
                        state.ui.show_reference_manager = false;
                    }
                });
            });
        });

    state.ui.show_reference_manager = open;

    if let Some(idx) = remove_index {
        ProjectService::remove_reference(state, idx);
    }

    if let Some(preset) = align_preset {
        state.camera_frame = None;
        state.camera.set_preset(preset);
        state.mark_dirty();
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_slot_card(
    ui: &mut Ui,
    ctx: &egui::Context,
    state: &mut AppState,
    axis: RefAxis,
    label: &str,
    coord_hint: &str,
    preset: ViewPreset,
    ref_idx: Option<usize>,
    remove_index: &mut Option<usize>,
    align_preset: &mut Option<ViewPreset>,
) {
    let card_w = 260.0;
    ui.allocate_ui_with_layout(
        vec2(card_w, 0.0),
        egui::Layout::top_down_justified(egui::Align::Min),
        |ui| {
            ui.set_width(card_w);
            egui::Frame::group(ui.style())
                .fill(tokens::bg_panel(state))
                .stroke(tokens::stroke_border_dyn(state))
                .corner_radius(tokens::RADIUS_CONTROL)
                .inner_margin(Margin::same(10))
                .show(ui, |ui| {
                    // Header
                    ui.horizontal(|ui| {
                        let badge_rect = ui
                            .allocate_exact_size(vec2(18.0, 18.0), egui::Sense::hover())
                            .0;
                        ui.painter().rect_filled(
                            badge_rect,
                            4.0,
                            tokens::ACCENT_BLUE.gamma_multiply(0.25),
                        );
                        IconRegistry::paint(
                            ctx,
                            ui.painter(),
                            &PetuniaIcon::ReferenceImage,
                            badge_rect.shrink(2.0),
                            tokens::ACCENT_BLUE_HOVER,
                        );

                        ui.label(
                            egui::RichText::new(label)
                                .strong()
                                .color(tokens::text_primary(state))
                                .size(13.0),
                        );
                        ui.label(
                            egui::RichText::new(coord_hint)
                                .color(tokens::text_muted(state))
                                .size(11.0),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .small_button("👁 Vista")
                                .on_hover_text("Alinhar câmera 3D com este ângulo de referência")
                                .clicked()
                            {
                                *align_preset = Some(preset);
                            }
                        });
                    });

                    ui.add_space(8.0);

                    // Square Clickable Thumbnail Box
                    let thumb_box_size = vec2(card_w - 20.0, 140.0);
                    let (box_rect, box_resp) =
                        ui.allocate_exact_size(thumb_box_size, egui::Sense::click());
                    let is_hovered = box_resp.hovered();

                    if box_resp.clicked() {
                        file_dialog_service::open_reference_image_dialog(Some(axis));
                    }

                    if let Some(idx) = ref_idx {
                        let thumb = crate::get_ref_texture(ctx, &state.project.refs[idx]);
                        ui.painter()
                            .rect_filled(box_rect, 6.0, Color32::from_rgb(18, 18, 20));

                        let ref_item = &state.project.refs[idx];
                        let img_w = ref_item.width as f32;
                        let img_h = ref_item.height as f32;
                        let aspect = if img_h > 0.0 { img_w / img_h } else { 1.0 };
                        let box_aspect = box_rect.width() / box_rect.height();

                        let draw_rect = if aspect > box_aspect {
                            let draw_h = box_rect.width() / aspect;
                            Rect::from_center_size(
                                box_rect.center(),
                                vec2(box_rect.width(), draw_h),
                            )
                        } else {
                            let draw_w = box_rect.height() * aspect;
                            Rect::from_center_size(
                                box_rect.center(),
                                vec2(draw_w, box_rect.height()),
                            )
                        };

                        ui.painter().image(
                            thumb.id(),
                            draw_rect,
                            Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)),
                            Color32::WHITE,
                        );

                        let border_color = if is_hovered {
                            tokens::ACCENT_BLUE
                        } else {
                            tokens::border_subtle(state)
                        };
                        ui.painter().rect_stroke(
                            box_rect,
                            6.0,
                            Stroke::new(1.5, border_color),
                            egui::StrokeKind::Inside,
                        );

                        if is_hovered {
                            ui.painter()
                                .rect_filled(box_rect, 6.0, Color32::from_black_alpha(140));
                            ui.painter().text(
                                box_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "↻ Clique para substituir",
                                egui::FontId::proportional(12.0),
                                Color32::WHITE,
                            );
                        }

                        ui.add_space(6.0);

                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(&ref_item.name)
                                    .color(tokens::text_primary(state))
                                    .size(11.0)
                                    .strong(),
                            );
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "{}×{}px",
                                            ref_item.width, ref_item.height
                                        ))
                                        .color(tokens::text_muted(state))
                                        .size(10.0),
                                    );
                                },
                            );
                        });

                        ui.add_space(4.0);

                        let mut vis = state.project.refs[idx].visible;
                        let mut locked = state.project.refs[idx].locked;
                        let mut flags_changed = false;

                        ui.horizontal(|ui| {
                            if ui.checkbox(&mut vis, "Visível").changed() {
                                flags_changed = true;
                            }
                            if ui.checkbox(&mut locked, "Travar").changed() {
                                flags_changed = true;
                            }
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui
                                        .small_button("🗑")
                                        .on_hover_text("Remover imagem de referência")
                                        .clicked()
                                    {
                                        *remove_index = Some(idx);
                                    }
                                },
                            );
                        });

                        if flags_changed {
                            state.project.refs[idx].visible = vis;
                            state.project.refs[idx].locked = locked;
                            state.mark_dirty();
                        }

                        ui.add_space(6.0);

                        let text_sec = tokens::text_secondary(state);
                        let ref_mut = &mut state.project.refs[idx];
                        let mut changed = false;

                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Opacidade:").size(10.5).color(text_sec));
                            if ui
                                .add(
                                    egui::Slider::new(&mut ref_mut.opacity, 0.0..=1.0)
                                        .show_value(true),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                            if ui
                                .small_button("↺")
                                .on_hover_text("Resetar para 0.6")
                                .clicked()
                            {
                                ref_mut.opacity = 0.6;
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Escala:").size(10.5).color(text_sec));
                            if ui
                                .add(
                                    egui::Slider::new(&mut ref_mut.size, 0.1..=20.0)
                                        .show_value(true),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                            if ui
                                .small_button("↺")
                                .on_hover_text("Resetar para 4.0")
                                .clicked()
                            {
                                ref_mut.size = 4.0;
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Offset:").size(10.5).color(text_sec));
                            if ui
                                .add(
                                    egui::Slider::new(&mut ref_mut.offset, -20.0..=20.0)
                                        .show_value(true),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                            if ui
                                .small_button("↺")
                                .on_hover_text("Resetar para -3.0")
                                .clicked()
                            {
                                ref_mut.offset = -3.0;
                                changed = true;
                            }
                        });

                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Rotação:").size(10.5).color(text_sec));
                            if ui
                                .add(
                                    egui::Slider::new(&mut ref_mut.rotation, -180.0..=180.0)
                                        .suffix("°")
                                        .show_value(true),
                                )
                                .changed()
                            {
                                changed = true;
                            }
                            if ui
                                .small_button("↺")
                                .on_hover_text("Resetar para 0°")
                                .clicked()
                            {
                                ref_mut.rotation = 0.0;
                                changed = true;
                            }
                        });

                        if changed {
                            state.mark_dirty();
                        }
                    } else {
                        // Slot Vazio
                        let bg = if is_hovered {
                            tokens::bg_surface_hover(state)
                        } else {
                            tokens::bg_surface(state)
                        };
                        let border = if is_hovered {
                            tokens::ACCENT_BLUE
                        } else {
                            tokens::border_subtle(state)
                        };
                        ui.painter().rect_filled(box_rect, 6.0, bg);
                        ui.painter().rect_stroke(
                            box_rect,
                            6.0,
                            Stroke::new(1.0, border),
                            egui::StrokeKind::Inside,
                        );

                        let center = box_rect.center();
                        ui.painter().text(
                            Pos2::new(center.x, center.y - 12.0),
                            egui::Align2::CENTER_CENTER,
                            "📁",
                            egui::FontId::proportional(22.0),
                            tokens::text_muted(state),
                        );
                        ui.painter().text(
                            Pos2::new(center.x, center.y + 14.0),
                            egui::Align2::CENTER_CENTER,
                            "Clique para carregar",
                            egui::FontId::proportional(11.5),
                            if is_hovered {
                                tokens::text_primary(state)
                            } else {
                                tokens::text_muted(state)
                            },
                        );

                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new("Nenhuma imagem vinculada a esta vista")
                                .size(10.5)
                                .color(tokens::text_muted(state))
                                .italics(),
                        );
                    }
                });
        },
    );
}
