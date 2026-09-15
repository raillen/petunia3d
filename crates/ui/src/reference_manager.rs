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
use crate::widgets;
use petunia_config::text_id;

/// Slots ortográficos canônicos: eixo + marcador neutro de orientação + preset.
/// Nomes vêm do i18n (`refs.front`…); o marcador (+Z…) é símbolo, não texto.
const SLOTS: [(RefAxis, &str, ViewPreset); 6] = [
    (RefAxis::Front, "+Z", ViewPreset::Front),
    (RefAxis::Back, "-Z", ViewPreset::Back),
    (RefAxis::Left, "-X", ViewPreset::Left),
    (RefAxis::Right, "+X", ViewPreset::Right),
    (RefAxis::Top, "+Y", ViewPreset::Top),
    (RefAxis::Bottom, "-Y", ViewPreset::Bottom),
];

/// Colunas da grade responsiva a partir da largura disponível (Wave 5 — §9.1).
fn grid_columns(available_w: f32) -> usize {
    const GAP: f32 = 14.0;
    const MIN_CARD: f32 = 230.0;
    (((available_w + GAP) / (MIN_CARD + GAP)).floor() as usize).clamp(1, 3)
}

/// Largura exata do cartão na grade (sem sobra nem falta).
fn grid_card_width(available_w: f32, columns: usize) -> f32 {
    const GAP: f32 = 14.0;
    ((available_w - GAP * (columns as f32 - 1.0)) / columns as f32).max(200.0)
}

/// Renderiza a janela utilitária do Gerenciador de Referências (P3D-013).
pub fn draw(ctx: &egui::Context, state: &mut AppState) {
    puffin::profile_function!();
    if !state.ui.show_reference_manager {
        return;
    }

    let mut open = state.ui.show_reference_manager;
    let mut remove_index = None;
    let mut align_preset = None;

    // Wave 5 (§9.4): mínimo e máximo nunca excedem a viewport útil.
    let screen_rect = ctx.viewport_rect();
    let (default_size, min_size, max_size) = crate::regions::modal_sizes(
        screen_rect,
        vec2(720.0, 560.0),
        vec2(480.0, 360.0),
        vec2(860.0, 720.0),
    );

    egui::Window::new(format!("{} · P3D-013", state.t("refs.manager_title")))
        .open(&mut open)
        .default_size(default_size)
        .min_size(min_size)
        .max_size(max_size)
        .collapsible(false)
        .resizable(true)
        .show(ctx, |ui| {
            // Cabeçalho descritivo e ações globais (empilha no estreito).
            if ui.available_width() < 560.0 {
                ui.label(
                    egui::RichText::new(state.t("refs.manager_desc"))
                        .color(tokens::TEXT_SECONDARY)
                        .size(12.0),
                );
                ui.horizontal(|ui| {
                    draw_global_actions(ui, state);
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(state.t("refs.manager_desc"))
                            .color(tokens::TEXT_SECONDARY)
                            .size(12.0),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        draw_global_actions(ui, state);
                    });
                });
            }

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // Grade determinística de cartões (Wave 5 — §9.1): colunas pela
            // largura disponível, cartões com largura exata, linhas alinhadas.
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let avail = ui.available_width();
                    let columns = grid_columns(avail).min(SLOTS.len()).max(1);
                    let card_w = grid_card_width(avail, columns);
                    for (row, chunk) in SLOTS.chunks(columns).enumerate() {
                        if row > 0 {
                            ui.add_space(14.0);
                        }
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = vec2(14.0, 14.0);
                            for &(axis, tag, preset) in chunk {
                                let matching_idx = state.project.refs.iter().position(|r| {
                                    r.axis == axis
                                        || (axis == RefAxis::Right && r.axis == RefAxis::Side)
                                });

                                draw_slot_card(
                                    ui,
                                    ctx,
                                    state,
                                    axis,
                                    tag,
                                    card_w,
                                    preset,
                                    matching_idx,
                                    &mut remove_index,
                                    &mut align_preset,
                                );
                            }
                        });
                    }
                });

            ui.add_space(10.0);
            ui.separator();
            ui.horizontal(|ui| {
                let total = state.project.refs.len();
                ui.label(
                    egui::RichText::new(format!("{total} {}", state.t_id(text_id::REFS_LOADED)))
                        .color(tokens::text_muted(state))
                        .size(11.0),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(state.t_id(text_id::UI_CLOSE)).clicked() {
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

fn draw_global_actions(ui: &mut Ui, state: &mut AppState) {
    if ui
        .button(state.t("refs.clear_all"))
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
}

/// Linha de slider com reset (zona de ajuste fino do cartão).
#[allow(clippy::too_many_arguments)]
fn fine_slider(
    ui: &mut Ui,
    text_color: Color32,
    label: &str,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    reset_to: f32,
    suffix: Option<&str>,
    reset_tip: &str,
    changed: &mut bool,
) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(label).size(10.5).color(text_color));
        let mut slider = egui::Slider::new(value, range).show_value(true);
        if let Some(suffix) = suffix {
            slider = slider.suffix(suffix);
        }
        if ui.add(slider).changed() {
            *changed = true;
        }
        if widgets::PetuniaIconButton::new(PetuniaIcon::Undo, reset_tip, 18.0)
            .show(ui)
            .clicked()
        {
            *value = reset_to;
            *changed = true;
        }
    });
}

#[allow(clippy::too_many_arguments)]
fn draw_slot_card(
    ui: &mut Ui,
    ctx: &egui::Context,
    state: &mut AppState,
    axis: RefAxis,
    axis_tag: &str,
    card_w: f32,
    preset: ViewPreset,
    ref_idx: Option<usize>,
    remove_index: &mut Option<usize>,
    align_preset: &mut Option<ViewPreset>,
) {
    // Título traduzido (símbolo de orientação é neutro, não texto).
    let title = format!("{axis_tag} · {}", state.t(axis.key()));
    let align_tip = state.t_id(text_id::REFS_ALIGN_VIEW);
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
                    // Header (zonas fixas: badge + título + ação).
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
                            egui::RichText::new(&title)
                                .strong()
                                .color(tokens::text_primary(state))
                                .size(13.0),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if widgets::PetuniaIconButton::new(
                                PetuniaIcon::Eye,
                                &format!("{} · {align_tip}", state.t(axis.key())),
                                20.0,
                            )
                            .show(ui)
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
                        let thumb = crate::get_ref_texture(
                            ctx,
                            &state.project.refs[idx],
                            &state.project.refs,
                        );
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
                                state.t_id(text_id::REFS_REPLACE),
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

                        let vis_label = state.t_id(text_id::REFS_VISIBLE);
                        let lock_label = state.t_id(text_id::REFS_LOCK);
                        let remove_tip = state.t_id(text_id::REFS_REMOVE);
                        ui.horizontal(|ui| {
                            if ui.checkbox(&mut vis, vis_label).changed() {
                                flags_changed = true;
                            }
                            if ui.checkbox(&mut locked, lock_label).changed() {
                                flags_changed = true;
                            }
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.small_button("✕").on_hover_text(remove_tip).clicked() {
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

                        // Ajuste fino colapsável: cartões carregados mantêm a
                        // mesma altura base; sliders abrem sob demanda.
                        let fine_label = state.t_id(text_id::REFS_FINE_TUNE);
                        let reset_tip = state.t_id(text_id::REFS_RESET_DEFAULT);
                        let opacity_label = format!("{}:", state.t_id(text_id::REFS_OPACITY));
                        let size_label = format!("{}:", state.t_id(text_id::REFS_SIZE));
                        let offset_label = format!("{}:", state.t_id(text_id::REFS_OFFSET));
                        let rotation_label = format!("{}:", state.t_id(text_id::REFS_ROTATION));
                        egui::CollapsingHeader::new(fine_label)
                            .default_open(false)
                            .show(ui, |ui| {
                                let text_sec = tokens::text_secondary(state);
                                let ref_mut = &mut state.project.refs[idx];
                                let mut changed = false;

                                fine_slider(
                                    ui,
                                    text_sec,
                                    &opacity_label,
                                    &mut ref_mut.opacity,
                                    0.0..=1.0,
                                    0.6,
                                    None,
                                    &reset_tip,
                                    &mut changed,
                                );
                                fine_slider(
                                    ui,
                                    text_sec,
                                    &size_label,
                                    &mut ref_mut.size,
                                    0.1..=20.0,
                                    4.0,
                                    None,
                                    &reset_tip,
                                    &mut changed,
                                );
                                fine_slider(
                                    ui,
                                    text_sec,
                                    &offset_label,
                                    &mut ref_mut.offset,
                                    -20.0..=20.0,
                                    -3.0,
                                    None,
                                    &reset_tip,
                                    &mut changed,
                                );
                                fine_slider(
                                    ui,
                                    text_sec,
                                    &rotation_label,
                                    &mut ref_mut.rotation,
                                    -180.0..=180.0,
                                    0.0,
                                    Some("°"),
                                    &reset_tip,
                                    &mut changed,
                                );

                                if changed {
                                    state.mark_dirty();
                                }
                            });
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

                        // Marcador central com ícone semântico (Wave 6).
                        let center = box_rect.center();
                        IconRegistry::paint(
                            ctx,
                            ui.painter(),
                            &PetuniaIcon::Folder,
                            Rect::from_center_size(center, vec2(22.0, 22.0)),
                            tokens::text_muted(state),
                        );

                        // Reserva de altura: espelha as zonas de status do cartão
                        // carregado (nome + flags) para a grade não escalonar.
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("—")
                                    .color(tokens::text_muted(state))
                                    .size(11.0),
                            );
                        });
                        ui.add_space(26.0);
                        ui.label(
                            egui::RichText::new(state.t_id(text_id::REFS_NO_IMAGE))
                                .size(10.5)
                                .color(tokens::text_muted(state))
                                .italics(),
                        );
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(state.t_id(text_id::REFS_CLICK_TO_LOAD))
                                .size(11.5)
                                .color(if is_hovered {
                                    tokens::text_primary(state)
                                } else {
                                    tokens::text_muted(state)
                                }),
                        );
                    }
                });
        },
    );
}

#[cfg(test)]
mod grid_tests {
    use super::{grid_card_width, grid_columns};

    #[test]
    fn columns_follow_available_width() {
        assert_eq!(grid_columns(1200.0), 3);
        assert_eq!(grid_columns(800.0), 3);
        assert_eq!(grid_columns(500.0), 2);
        assert_eq!(grid_columns(300.0), 1);
        assert_eq!(grid_columns(0.0), 1);
    }

    #[test]
    fn cards_fill_row_exactly() {
        for avail in [300.0, 500.0, 800.0, 1200.0] {
            let cols = grid_columns(avail);
            let card = grid_card_width(avail, cols);
            let row = card * cols as f32 + 14.0 * (cols as f32 - 1.0);
            assert!((row - avail).abs() < 1.0, "avail {avail}: row {row}");
            assert!(card >= 200.0);
        }
    }
}
