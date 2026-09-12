//! Gaveta / Modal da Biblioteca de Assets do Projeto (`Asset Library Drawer`).
//! Exibe todos os modelos/assets criados e salvos neste projeto,
//! permitindo instanciar na cena (na posição do 3D Cursor), duplicar, renomear,
//! e estabelece a clara distinção conceitual:
//! - **Salvar Asset**: Salva/registra o modelo ativo dentro da biblioteca do projeto;
//! - **Salvar Projeto**: Salva o arquivo completo `.petunia` (cena, todos os assets, materiais, câmera).

use egui::{vec2, Align, Color32, CornerRadius, Layout, RichText, ScrollArea, Stroke, Ui, Window};
use petunia_core::AppState;

use crate::tokens;

/// Renderiza o modal / gaveta da Biblioteca de Assets quando `state.show_asset_library` estiver ativo.
pub fn draw(ctx: &egui::Context, state: &mut AppState) {
    if !state.show_asset_library {
        return;
    }

    let mut open = state.show_asset_library;
    let screen_rect = ctx.screen_rect();
    let default_width = (screen_rect.width() * 0.65).clamp(420.0, 780.0);
    let default_height = (screen_rect.height() * 0.60).clamp(340.0, 600.0);

    Window::new(
        RichText::new("📦 Biblioteca de Assets do Projeto")
            .strong()
            .size(14.0),
    )
    .open(&mut open)
    .default_size(vec2(default_width, default_height))
    .min_size(vec2(380.0, 280.0))
    .resizable(true)
    .collapsible(false)
    .frame(
        egui::Frame::window(&ctx.style())
            .fill(tokens::BG_PANEL)
            .stroke(tokens::stroke_border())
            .inner_margin(egui::Margin::same(12)),
    )
    .show(ctx, |ui| {
        draw_contents(ui, state);
    });

    state.show_asset_library = open;
}

fn draw_contents(ui: &mut Ui, state: &mut AppState) {
    // 1. Barra de Ações Superior (Distinção clara: Salvar Asset vs Salvar Projeto)
    ui.horizontal(|ui| {
        ui.label(RichText::new("Biblioteca interna do projeto").color(tokens::TEXT_SECONDARY).size(11.5));

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            // Botão: Salvar Projeto Completo
            let save_proj_btn = egui::Button::new(
                RichText::new("💾 Salvar Projeto (.petunia)")
                    .size(11.0)
                    .color(tokens::TEXT_PRIMARY),
            )
            .fill(tokens::BG_SURFACE)
            .stroke(Stroke::new(1.0_f32, tokens::BORDER_SUBTLE))
            .corner_radius(tokens::RADIUS_CONTROL);

            if ui
                .add(save_proj_btn)
                .on_hover_text("Salva o projeto completo com todos os assets, câmera, paleta e estado em disco (.petunia)")
                .clicked()
            {
                crate::save_project_dialog(state, false);
            }

            // Botão: Salvar Ativo como Asset
            let save_asset_btn = egui::Button::new(
                RichText::new("📥 Salvar Modelo Ativo como Asset")
                    .size(11.0)
                    .color(tokens::TEXT_ACTIVE),
            )
            .fill(tokens::ACCENT_BLUE)
            .corner_radius(tokens::RADIUS_CONTROL);

            if ui
                .add(save_asset_btn)
                .on_hover_text("Salva uma cópia do modelo 3D atualmente selecionado dentro da biblioteca interna deste projeto")
                .clicked()
            {
                state.save_active_as_asset();
            }
        });
    });

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);

    // 2. Campo de busca / filtro rápido
    let filter_id = egui::Id::new("asset_library_filter_text");
    let mut search_query = ui
        .ctx()
        .data_mut(|d| d.get_temp::<String>(filter_id).unwrap_or_default());

    ui.horizontal(|ui| {
        ui.label(RichText::new("🔍").size(12.0));
        let resp = ui.add(
            egui::TextEdit::singleline(&mut search_query)
                .hint_text("Filtrar assets por nome...")
                .desired_width(260.0),
        );
        if resp.changed() {
            ui.ctx()
                .data_mut(|d| d.insert_temp(filter_id, search_query.clone()));
        }

        ui.label(
            RichText::new(format!("Total: {} asset(s)", state.project.assets.len()))
                .size(11.0)
                .color(tokens::TEXT_SECONDARY),
        );
    });

    ui.add_space(8.0);

    // 3. Grade / Lista de Cards de Assets
    let total_assets = state.project.assets.len();
    let mut to_instantiate: Option<usize> = None;
    let mut to_activate: Option<usize> = None;
    let mut to_duplicate: Option<usize> = None;
    let mut to_remove: Option<usize> = None;

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = vec2(10.0, 10.0);

                for i in 0..total_assets {
                    let asset = &state.project.assets[i];

                    // Aplica filtro de busca
                    if !search_query.is_empty()
                        && !asset.name.to_lowercase().contains(&search_query.to_lowercase())
                    {
                        continue;
                    }

                    let is_active = state.project.active == i;
                    let (card_bg, border_color) = if is_active {
                        (tokens::BG_SURFACE_ACTIVE, tokens::ACCENT_BLUE)
                    } else {
                        (tokens::BG_SURFACE, tokens::BORDER_SUBTLE)
                    };

                    egui::Frame::new()
                        .fill(card_bg)
                        .stroke(Stroke::new(1.0_f32, border_color))
                        .corner_radius(CornerRadius::same(6))
                        .inner_margin(egui::Margin::same(10))
                        .show(ui, |ui| {
                            ui.set_width(210.0);
                            ui.set_min_height(140.0);

                            // Cabeçalho do Card: Nome e Tag Ativo
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(&asset.name).strong().size(12.5).color(tokens::TEXT_PRIMARY));
                                if is_active {
                                    ui.label(
                                        RichText::new("● Ativo")
                                            .size(10.0)
                                            .color(tokens::ACCENT_BLUE),
                                    );
                                }
                            });

                            // Indicador de Cor Base / Miniatura de Material
                            ui.add_space(4.0);
                            ui.horizontal(|ui| {
                                let c = Color32::from_rgb(
                                    (asset.base_color[0] * 255.0) as u8,
                                    (asset.base_color[1] * 255.0) as u8,
                                    (asset.base_color[2] * 255.0) as u8,
                                );
                                let (chip_rect, _) = ui.allocate_exact_size(vec2(16.0, 16.0), egui::Sense::hover());
                                ui.painter().rect_filled(chip_rect, 3.0, c);
                                ui.painter().rect_stroke(chip_rect, 3.0, Stroke::new(1.0_f32, tokens::BORDER_SUBTLE), egui::StrokeKind::Inside);

                                ui.label(
                                    RichText::new(format!(
                                        "{} verts · {} faces",
                                        asset.mesh.vert_count(),
                                        asset.mesh.face_count()
                                    ))
                                    .size(10.5)
                                    .color(tokens::TEXT_SECONDARY),
                                );
                            });

                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(4.0);

                            // Botões de Ação do Card
                            ui.horizontal(|ui| {
                                if ui
                                    .button(RichText::new("➕ Instanciar").size(10.5))
                                    .on_hover_text("Cria uma cópia deste modelo na posição atual do Cursor 3D")
                                    .clicked()
                                {
                                    to_instantiate = Some(i);
                                }

                                if !is_active && ui
                                    .button(RichText::new("🎯 Editar").size(10.5))
                                    .on_hover_text("Torna este asset o modelo ativo no viewport para edição")
                                    .clicked()
                                {
                                    to_activate = Some(i);
                                }
                            });

                            ui.horizontal(|ui| {
                                if ui
                                    .button(RichText::new("📋 Duplicar").size(10.0))
                                    .on_hover_text("Cria uma cópia deste asset na biblioteca")
                                    .clicked()
                                {
                                    to_duplicate = Some(i);
                                }

                                if total_assets > 1 && ui
                                    .button(RichText::new("🗑").size(10.0).color(Color32::from_rgb(255, 100, 100)))
                                    .on_hover_text("Remove este asset da biblioteca do projeto")
                                    .clicked()
                                {
                                    to_remove = Some(i);
                                }
                            });
                        });
                }
            });
        });

    // 4. Aplicação das ações pendentes fora dos borrows
    if let Some(idx) = to_instantiate {
        state.instantiate_asset_at_cursor(idx);
    }
    if let Some(idx) = to_activate {
        state.project.active = idx;
        state.sync_selection();
        state.emit_mesh_changed();
        state.mark_dirty();
    }
    if let Some(idx) = to_duplicate {
        if let Some(asset) = state.project.assets.get(idx) {
            let copy = asset.duplicate();
            state.checkpoint("duplicate asset");
            state.project.assets.push(copy);
            state.mark_dirty();
        }
    }
    if let Some(idx) = to_remove {
        state.checkpoint("remove asset");
        state.project.remove(idx);
        state.sync_selection();
        state.emit_mesh_changed();
        state.mark_dirty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_library_drawer_renders_without_panic() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");
        state.show_asset_library = true;

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            draw(ctx, &mut state);
        });
    }

    #[test]
    fn test_save_active_as_asset_and_instantiate() {
        let mut state = AppState::new("en");
        let initial_count = state.project.assets.len();
        assert_eq!(initial_count, 1);

        // Salvar ativo como novo asset
        assert!(state.save_active_as_asset());
        assert_eq!(state.project.assets.len(), 2);

        // Instanciar asset na cena
        assert!(state.instantiate_asset_at_cursor(0));
        assert_eq!(state.project.assets.len(), 3);
    }
}
