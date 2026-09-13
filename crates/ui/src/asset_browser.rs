//! Painel Lateral do Navegador de Assets do Petunia3D (`Asset Browser`).
//! Fornece visualização dedicada, busca, filtragem por categoria, instanciação
//! na coordenada do 3D Cursor e gerenciamento de modelos salvos no projeto.

use egui::{vec2, Align, Color32, Layout, RichText, ScrollArea, SidePanel, Ui};
use petunia_core::AppState;

use crate::tokens;
use crate::widgets;

/// Renderiza o painel lateral retrátil do Asset Browser.
pub fn draw(ctx: &egui::Context, state: &mut AppState) {
    if !state.show_asset_browser {
        return;
    }

    let min_w = 220.0;
    let max_w = 340.0;

    SidePanel::left("asset_browser_panel")
        .default_width(260.0)
        .width_range(min_w..=max_w)
        .resizable(true)
        .frame(
            egui::Frame::new()
                .fill(tokens::BG_PANEL)
                .stroke(tokens::stroke_border())
                .inner_margin(egui::Margin::symmetric(8, 6)),
        )
        .show(ctx, |ui| {
            draw_header(ui, state);
            ui.separator();
            draw_categories(ui, state);
            ui.separator();
            draw_asset_cards(ui, state);
            ui.separator();
            draw_footer_actions(ui, state);
        });
}

fn draw_header(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new("📦 Asset Browser")
                .strong()
                .size(12.5)
                .color(tokens::TEXT_PRIMARY),
        );
        let count = state.project.assets.len();
        ui.label(
            RichText::new(format!("({count})"))
                .size(11.0)
                .color(tokens::TEXT_MUTED),
        );

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui
                .small_button("✕")
                .on_hover_text("Fechar Asset Browser")
                .clicked()
            {
                state.show_asset_browser = false;
                state.mark_dirty();
            }
        });
    });

    ui.add_space(4.0);

    // Campo de busca padronizado com PetuniaSearchBox
    let filter_id = egui::Id::new("asset_browser_search_query");
    let mut query = ui
        .ctx()
        .data_mut(|d| d.get_temp::<String>(filter_id).unwrap_or_default());

    if widgets::petunia_search_box(ui, &mut query, "Buscar assets...").changed() {
        ui.ctx()
            .data_mut(|d| d.insert_temp(filter_id, query.clone()));
        state.mark_dirty();
    }
}

fn draw_categories(ui: &mut Ui, _state: &mut AppState) {
    let cat_id = egui::Id::new("asset_browser_active_category");
    let active_cat = ui.ctx().data_mut(|d| {
        d.get_temp::<String>(cat_id)
            .unwrap_or_else(|| "all".to_string())
    });

    let categories = [
        ("all", "Todos"),
        ("props", "Props"),
        ("chars", "Personagens"),
        ("env", "Cenário"),
    ];

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(3.0, 3.0);
        for (id, label) in categories {
            let is_sel = active_cat == id;
            let (bg, fg) = if is_sel {
                (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
            } else {
                (tokens::BG_SURFACE, tokens::TEXT_SECONDARY)
            };

            let btn = egui::Button::new(RichText::new(label).size(10.5).color(fg))
                .fill(bg)
                .corner_radius(tokens::RADIUS_CONTROL);

            if ui.add(btn).clicked() {
                ui.ctx().data_mut(|d| d.insert_temp(cat_id, id.to_string()));
            }
        }
    });
}

fn draw_asset_cards(ui: &mut Ui, state: &mut AppState) {
    let filter_id = egui::Id::new("asset_browser_search_query");
    let query = ui
        .ctx()
        .data_mut(|d| d.get_temp::<String>(filter_id).unwrap_or_default())
        .trim()
        .to_lowercase();

    let n = state.project.assets.len();
    if n == 0 {
        ui.add_space(20.0);
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new("Nenhum asset no projeto")
                    .color(tokens::TEXT_MUTED)
                    .size(11.0),
            );
            ui.label(
                RichText::new("Salve o modelo ativo abaixo para catalogar.")
                    .color(tokens::TEXT_MUTED)
                    .size(10.0),
            );
        });
        return;
    }

    ScrollArea::vertical()
        .id_salt("asset_browser_cards_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 6.0);

            let mut to_instantiate: Option<usize> = None;
            let mut to_activate: Option<usize> = None;
            let mut to_duplicate: Option<usize> = None;
            let mut to_delete: Option<usize> = None;

            for i in 0..n {
                let a = &state.project.assets[i];
                if !query.is_empty() && !a.name.to_lowercase().contains(&query) {
                    continue;
                }

                let is_active = state.project.active == i;
                let tris = a.mesh.tri_count();
                let verts = a.mesh.vert_count();
                let col = a.base_color;
                let name = a.name.clone();

                let card_bg = if is_active {
                    tokens::BG_SURFACE_ACTIVE
                } else {
                    tokens::BG_SURFACE
                };

                let card_stroke = if is_active {
                    egui::Stroke::new(1.0_f32, tokens::ACCENT_BORDER)
                } else {
                    tokens::stroke_border()
                };

                egui::Frame::new()
                    .fill(card_bg)
                    .stroke(card_stroke)
                    .corner_radius(tokens::RADIUS_CONTROL)
                    .inner_margin(egui::Margin::symmetric(8, 6))
                    .show(ui, |ui| {
                        // Linha 1: Ícone de cor + Nome + Status ativo
                        ui.horizontal(|ui| {
                            let (c_rect, _) = ui.allocate_exact_size(vec2(12.0, 12.0), egui::Sense::hover());
                            let c_fill = Color32::from_rgb(
                                (col[0] * 255.0) as u8,
                                (col[1] * 255.0) as u8,
                                (col[2] * 255.0) as u8,
                            );
                            ui.painter().rect_filled(c_rect, tokens::RADIUS_SMALL, c_fill);

                            ui.label(RichText::new(&name).strong().size(11.5).color(tokens::TEXT_PRIMARY));

                            if is_active {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.label(
                                        RichText::new("● ATIVO")
                                            .size(9.0)
                                            .strong()
                                            .color(tokens::ACCENT_BLUE),
                                    );
                                });
                            }
                        });

                        // Linha 2: Métricas de Geometria
                        ui.label(
                            RichText::new(format!("{tris} tris │ {verts} verts"))
                                .size(10.0)
                                .color(tokens::TEXT_MUTED),
                        );

                        ui.add_space(2.0);

                        // Linha 3: Ações Rápidas do Card
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = vec2(3.0, 0.0);

                            let inst_btn = egui::Button::new(
                                RichText::new("➕ Instanciar")
                                    .size(10.0)
                                    .color(tokens::TEXT_ACTIVE),
                            )
                            .fill(tokens::ACCENT_BLUE)
                            .corner_radius(tokens::RADIUS_SMALL);

                            if ui
                                .add(inst_btn)
                                .on_hover_text("Cria uma nova instância deste asset nas coordenadas do 3D Cursor")
                                .clicked()
                            {
                                to_instantiate = Some(i);
                            }

                            if !is_active {
                                let act_btn = egui::Button::new(
                                    RichText::new("🎯 Ativar")
                                        .size(10.0)
                                        .color(tokens::TEXT_PRIMARY),
                                )
                                .fill(tokens::BG_PANEL)
                                .corner_radius(tokens::RADIUS_SMALL);

                                if ui
                                    .add(act_btn)
                                    .on_hover_text("Torna este asset o modelo ativo para edição")
                                    .clicked()
                                {
                                    to_activate = Some(i);
                                }
                            }

                            let dup_btn = egui::Button::new(
                                RichText::new("📋")
                                    .size(10.0)
                                    .color(tokens::TEXT_SECONDARY),
                            )
                            .fill(tokens::BG_PANEL)
                            .corner_radius(tokens::RADIUS_SMALL);

                            if ui
                                .add(dup_btn)
                                .on_hover_text("Duplicar este asset")
                                .clicked()
                            {
                                to_duplicate = Some(i);
                            }

                            if n > 1 {
                                let del_btn = egui::Button::new(
                                    RichText::new("🗑")
                                        .size(10.0)
                                        .color(tokens::TEXT_MUTED),
                                )
                                .fill(tokens::BG_PANEL)
                                .corner_radius(tokens::RADIUS_SMALL);

                                if ui
                                    .add(del_btn)
                                    .on_hover_text("Remover asset do projeto")
                                    .clicked()
                                {
                                    to_delete = Some(i);
                                }
                            }
                        });
                    });
            }

            if let Some(idx) = to_instantiate {
                state.instantiate_asset_at_cursor(idx);
            }
            if let Some(idx) = to_activate {
                if idx < state.project.assets.len() {
                    state.project.active = idx;
                    state.sync_selection();
                    state.mark_dirty();
                }
            }
            if let Some(idx) = to_duplicate {
                if let Some(a) = state.project.assets.get(idx).cloned() {
                    state.checkpoint("duplicate asset");
                    let dup = a.duplicate();
                    state.project.assets.push(dup);
                    state.project.active = state.project.assets.len() - 1;
                    state.sync_selection();
                    state.emit_mesh_changed();
                }
            }
            if let Some(idx) = to_delete {
                if idx < state.project.assets.len() {
                    state.checkpoint("delete asset");
                    state.project.remove(idx);
                    state.sync_selection();
                    state.emit_mesh_changed();
                }
            }
        });
}

fn draw_footer_actions(ui: &mut Ui, state: &mut AppState) {
    ui.vertical_centered_justified(|ui| {
        let save_active_btn = egui::Button::new(
            RichText::new("📥 Salvar Modelo Ativo como Asset")
                .size(11.0)
                .color(tokens::TEXT_ACTIVE),
        )
        .fill(tokens::ACCENT_BLUE)
        .corner_radius(tokens::RADIUS_CONTROL);

        if ui
            .add(save_active_btn)
            .on_hover_text("Salva o modelo atualmente selecionado como asset permanente do projeto")
            .clicked()
        {
            state.save_active_as_asset();
        }
    });
}
