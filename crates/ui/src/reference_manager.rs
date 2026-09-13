//! Gerenciador visual de conjuntos de imagens de referência (Reference Sets / P3D-013 & P3D-014).
//!
//! Permite atribuir, substituir, pré-visualizar e calibrar imagens de referência
//! independentes para os 6 slots ortográficos canônicos (Front, Back, Left, Right, Top, Bottom).

#![forbid(unsafe_code)]

use egui::{vec2, Color32, Margin, Pos2, Rect, Stroke, Ui};
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
    if !state.ui.show_reference_manager {
        return;
    }

    let mut open = state.ui.show_reference_manager;
    let mut remove_index = None;
    let mut align_preset = None;

    egui::Window::new(format!("{} · P3D-013", state.t("refs.manager_title")))
        .open(&mut open)
        .default_size(vec2(720.0, 560.0))
        .min_size(vec2(540.0, 420.0))
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

            // Grade de cartões dos 6 slots ortográficos canônicos
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("reference_slots_grid")
                    .num_columns(2)
                    .spacing(vec2(14.0, 14.0))
                    .min_col_width(ui.available_width() * 0.48)
                    .show(ui, |ui| {
                        for (i, &(axis, label, coord_hint, preset)) in SLOTS.iter().enumerate() {
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

                            if i % 2 == 1 {
                                ui.end_row();
                            }
                        }
                    });
            });

            ui.add_space(10.0);
            ui.separator();
            ui.horizontal(|ui| {
                let total = state.project.refs.len();
                ui.label(
                    egui::RichText::new(format!("{total} referência(s) carregada(s)"))
                        .color(tokens::TEXT_MUTED)
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
    egui::Frame::group(ui.style())
        .fill(tokens::BG_PANEL)
        .stroke(Stroke::new(1.0_f32, tokens::BORDER_SUBTLE))
        .corner_radius(tokens::RADIUS_CONTROL)
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            // Título do slot e badge
            ui.horizontal(|ui| {
                let badge_rect = ui
                    .allocate_exact_size(vec2(18.0, 18.0), egui::Sense::hover())
                    .0;
                ui.painter()
                    .rect_filled(badge_rect, 4.0, tokens::ACCENT_BLUE.gamma_multiply(0.25));
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
                        .color(tokens::TEXT_PRIMARY)
                        .size(13.0),
                );
                ui.label(
                    egui::RichText::new(coord_hint)
                        .color(tokens::TEXT_MUTED)
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

            ui.add_space(6.0);

            if let Some(idx) = ref_idx {
                let thumb = crate::get_ref_texture(ctx, &state.project.refs[idx]);
                let ref_name = state.project.refs[idx].name.clone();
                let (ref_w, ref_h) = (
                    state.project.refs[idx].width,
                    state.project.refs[idx].height,
                );
                let mut vis = state.project.refs[idx].visible;
                let mut locked = state.project.refs[idx].locked;
                let mut changed_flags = false;

                // Miniatura e Metadados
                ui.horizontal(|ui| {
                    let thumb_size = vec2(64.0, 64.0);
                    let (rect, _) = ui.allocate_exact_size(thumb_size, egui::Sense::hover());
                    ui.painter().rect_filled(rect, 4.0, Color32::BLACK);
                    ui.painter().image(
                        thumb.id(),
                        rect,
                        Rect::from_min_max(Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)),
                        Color32::WHITE,
                    );
                    ui.painter().rect_stroke(
                        rect,
                        4.0,
                        Stroke::new(1.0_f32, tokens::BORDER_LIGHT),
                        egui::StrokeKind::Inside,
                    );

                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(&ref_name)
                                .color(tokens::TEXT_PRIMARY)
                                .size(12.0)
                                .strong(),
                        );
                        ui.label(
                            egui::RichText::new(format!("{ref_w} × {ref_h} px"))
                                .color(tokens::TEXT_MUTED)
                                .size(11.0),
                        );

                        ui.horizontal(|ui| {
                            // Toggle visibilidade
                            let vis_text = if vis { "Visível" } else { "Oculto" };
                            if ui.checkbox(&mut vis, vis_text).changed() {
                                changed_flags = true;
                            }

                            // Toggle lock
                            if ui.checkbox(&mut locked, "Travar").changed() {
                                changed_flags = true;
                            }
                        });
                    });
                });

                if changed_flags {
                    state.project.refs[idx].visible = vis;
                    state.project.refs[idx].locked = locked;
                    state.mark_dirty();
                }

                ui.add_space(4.0);

                // Controles de ajuste fino (Opacidade, Tamanho, Offset, Rotação)
                let ref_mut = &mut state.project.refs[idx];
                let mut changed = false;

                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Opacidade:")
                            .size(11.0)
                            .color(tokens::TEXT_SECONDARY),
                    );
                    if ui
                        .add(egui::Slider::new(&mut ref_mut.opacity, 0.0..=1.0).show_value(true))
                        .changed()
                    {
                        changed = true;
                    }
                    if ui
                        .small_button("↺")
                        .on_hover_text("Resetar opacidade para 0.6")
                        .clicked()
                    {
                        ref_mut.opacity = 0.6;
                        changed = true;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Escala:")
                            .size(11.0)
                            .color(tokens::TEXT_SECONDARY),
                    );
                    if ui
                        .add(egui::Slider::new(&mut ref_mut.size, 0.1..=20.0).show_value(true))
                        .changed()
                    {
                        changed = true;
                    }
                    if ui
                        .small_button("↺")
                        .on_hover_text("Resetar escala para 4.0")
                        .clicked()
                    {
                        ref_mut.size = 4.0;
                        changed = true;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Posição / Offset:")
                            .size(11.0)
                            .color(tokens::TEXT_SECONDARY),
                    );
                    if ui
                        .add(egui::Slider::new(&mut ref_mut.offset, -20.0..=20.0).show_value(true))
                        .changed()
                    {
                        changed = true;
                    }
                    if ui
                        .small_button("↺")
                        .on_hover_text("Resetar offset para -3.0")
                        .clicked()
                    {
                        ref_mut.offset = -3.0;
                        changed = true;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Rotação:")
                            .size(11.0)
                            .color(tokens::TEXT_SECONDARY),
                    );
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
                        .on_hover_text("Resetar rotação para 0°")
                        .clicked()
                    {
                        ref_mut.rotation = 0.0;
                        changed = true;
                    }
                });

                if changed {
                    state.mark_dirty();
                }

                ui.add_space(4.0);

                // Ações de Substituição e Remoção
                ui.horizontal(|ui| {
                    if ui
                        .button("Substituir...")
                        .on_hover_text("Escolher outra imagem para este slot")
                        .clicked()
                    {
                        if let Some(path) = file_dialog_service::pick_image_file() {
                            match crate::load_image_rgba(&path) {
                                Ok((w, h, rgba)) => {
                                    let name = path
                                        .file_name()
                                        .and_then(|s| s.to_str())
                                        .unwrap_or("ref")
                                        .to_string();
                                    ProjectService::set_reference_slot(
                                        state, axis, name, w, h, rgba,
                                    );
                                }
                                Err(e) => state.set_status(format!("Erro ao carregar imagem: {e}")),
                            }
                        }
                    }

                    if ui
                        .button("Remover")
                        .on_hover_text("Remover imagem deste slot")
                        .clicked()
                    {
                        *remove_index = Some(idx);
                    }
                });
            } else {
                // Slot Vazio
                let empty_rect = ui
                    .allocate_exact_size(vec2(ui.available_width(), 70.0), egui::Sense::hover())
                    .0;
                ui.painter().rect_filled(empty_rect, 4.0, tokens::BG_INPUT);
                ui.painter().rect_stroke(
                    empty_rect,
                    4.0,
                    Stroke::new(1.0_f32, tokens::BORDER_SUBTLE),
                    egui::StrokeKind::Inside,
                );

                let center = empty_rect.center();
                ui.painter().text(
                    Pos2::new(center.x, center.y - 12.0),
                    egui::Align2::CENTER_CENTER,
                    "Nenhuma imagem atribuída",
                    egui::FontId::proportional(12.0),
                    tokens::TEXT_MUTED,
                );

                let btn_rect =
                    Rect::from_center_size(Pos2::new(center.x, center.y + 14.0), vec2(130.0, 22.0));
                let resp = ui.allocate_rect(btn_rect, egui::Sense::click());
                let hovered = resp.hovered();
                ui.painter().rect_filled(
                    btn_rect,
                    4.0,
                    if hovered {
                        tokens::BG_SURFACE_HOVER
                    } else {
                        tokens::BG_SURFACE
                    },
                );
                ui.painter().rect_stroke(
                    btn_rect,
                    4.0,
                    Stroke::new(1.0_f32, tokens::BORDER_LIGHT),
                    egui::StrokeKind::Inside,
                );
                ui.painter().text(
                    btn_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "📁 Carregar Imagem...",
                    egui::FontId::proportional(11.0),
                    tokens::TEXT_PRIMARY,
                );

                if resp.clicked() {
                    if let Some(path) = file_dialog_service::pick_image_file() {
                        match crate::load_image_rgba(&path) {
                            Ok((w, h, rgba)) => {
                                let name = path
                                    .file_name()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("ref")
                                    .to_string();
                                ProjectService::set_reference_slot(state, axis, name, w, h, rgba);
                            }
                            Err(e) => state.set_status(format!("Erro ao carregar imagem: {e}")),
                        }
                    }
                }
            }
        });
}
