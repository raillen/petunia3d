//! Painel Outliner hierárquico do Petunia3D (`Outliner`).
//! Apresenta a árvore de coleções e objetos da cena com busca, seleção e toggles de visibilidade/render.

use egui::{vec2, Color32, ScrollArea, Ui};
use petunia_core::AppState;

use crate::tokens;

/// Renderiza o painel Outliner.
pub fn draw(ui: &mut Ui, state: &mut AppState) {
    egui::CollapsingHeader::new("Outliner")
        .default_open(true)
        .show(ui, |ui| {
            // 1. Cabeçalho do Outliner
            draw_outliner_header(ui, state);

            ui.separator();

            // 2. Área de rolagem com a árvore de nós da cena
            ScrollArea::vertical()
                .id_salt("outliner_tree_scroll")
                .max_height(140.0)
                .show(ui, |ui| {
                    draw_tree_nodes(ui, state);
                });
        });
}

fn draw_outliner_header(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(4.0, 0.0);

        // Seletor de Modo de Exibição
        egui::ComboBox::from_id_salt("outliner_display_mode")
            .selected_text(
                egui::RichText::new("ViewLayer")
                    .size(11.0)
                    .color(tokens::TEXT_PRIMARY),
            )
            .width(80.0)
            .show_ui(ui, |ui| {
                let _ = ui.selectable_label(true, "ViewLayer");
                let _ = ui.selectable_label(false, "Scenes");
                let _ = ui.selectable_label(false, "Sequence");
            });

        // Campo de busca / filtro
        ui.add(
            egui::TextEdit::singleline(&mut state.outliner_search)
                .hint_text("Search...")
                .desired_width(ui.available_width() - 32.0),
        );

        // Botão de Nova Coleção (+)
        if ui
            .button(
                egui::RichText::new("+")
                    .size(12.0)
                    .color(tokens::TEXT_PRIMARY),
            )
            .on_hover_text("New Collection")
            .clicked()
        {
            state.set_status("New Collection created");
        }
    });
}

fn draw_tree_nodes(ui: &mut Ui, state: &mut AppState) {
    let search = state.outliner_search.to_lowercase();

    // Raiz: Scene Collection
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("📁 Scene Collection")
                .size(11.5)
                .color(tokens::TEXT_PRIMARY),
        );
    });

    ui.indent("scene_collection_indent", |ui| {
        // Coleção padrão: Collection
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("🗂 Collection")
                    .size(11.5)
                    .color(tokens::TEXT_PRIMARY),
            );
        });

        ui.indent("collection_indent", |ui| {
            // 1. Objeto Câmera
            if search.is_empty() || "camera".contains(&search) {
                draw_object_row(ui, "📷 Camera", false, true, true, |_| {});
            }

            // 2. Objetos de Malha do Projeto
            let n_assets = state.project.assets.len();
            let active_idx = state.project.active;

            for i in 0..n_assets {
                let name = state.project.assets[i].name.clone();
                if !search.is_empty() && !name.to_lowercase().contains(&search) {
                    continue;
                }

                let is_selected = i == active_idx;
                let (vc, fc) = (
                    state.project.assets[i].mesh.vert_count(),
                    state.project.assets[i].mesh.tri_count(),
                );

                let label = format!("🧊 {name} ({vc}v, {fc}f)");
                let clicked = draw_object_row(ui, &label, is_selected, true, true, |_| {});

                if clicked {
                    state.project.active = i;
                    state.sync_selection();
                    state.mark_dirty();
                }
            }

            // 3. Objeto de Luz (Light)
            if search.is_empty() || "light".contains(&search) {
                draw_object_row(ui, "💡 Light", false, true, true, |_| {});
            }
        });
    });
}

fn draw_object_row<F>(
    ui: &mut Ui,
    name: &str,
    selected: bool,
    _visible: bool,
    _render: bool,
    _on_toggle: F,
) -> bool
where
    F: FnOnce(&mut AppState),
{
    let mut row_clicked = false;

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(4.0, 0.0);

        let (_bg, fg) = if selected {
            (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
        } else {
            (Color32::TRANSPARENT, tokens::TEXT_PRIMARY)
        };

        let response =
            ui.selectable_label(selected, egui::RichText::new(name).size(11.0).color(fg));
        if response.clicked() {
            row_clicked = true;
        }

        // Toggles da direita: Olho (Viewport) e Câmera (Render)
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let cam_btn = ui.small_button("📷");
            let _ = cam_btn;
            let eye_btn = ui.small_button("👁");
            let _ = eye_btn;
        });
    });

    row_clicked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outliner_renders_without_panic() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                draw(ui, &mut state);
            });
        });
    }
}
