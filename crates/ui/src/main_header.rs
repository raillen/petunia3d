//! Cabeçalho superior principal do Petunia3D (`Main Header`).
//! Menus do sistema (File, Edit, Render, Window, Help) e abas de Workspaces em pílulas arredondadas.

use egui::{vec2, Color32, Context, Ui};
use petunia_core::{AppState, Workspace};

use crate::tokens;
use crate::UiAction;

/// Renderiza o cabeçalho superior completo da aplicação.
pub fn draw(ctx: &Context, state: &mut AppState, action: &mut UiAction) {
    egui::TopBottomPanel::top("main_header")
        .default_height(tokens::TOP_HEADER_HEIGHT)
        .height_range(tokens::TOP_HEADER_HEIGHT..=tokens::TOP_HEADER_MAX_HEIGHT)
        .resizable(true)
        .frame(
            egui::Frame::new()
                .fill(tokens::BG_HEADER)
                .stroke(tokens::stroke_border())
                .inner_margin(egui::Margin::symmetric(8, 2)),
        )
        .show(ctx, |ui| {
            ui.add_enabled_ui(!state.is_interacting(), |ui| {
                ui.horizontal_centered(|ui| {
                    ui.spacing_mut().item_spacing = vec2(6.0, 0.0);

                    // Ícone/Logo de branding Petunia3D
                    draw_app_brand(ui);

                    ui.separator();

                    // Menus da aplicação com visual flat limpo
                    draw_menus(ui, state, action);

                    ui.separator();

                    // Abas de Workspaces em pílulas arredondadas (estilo Blender.svg)
                    draw_workspace_pills(ui, state);

                    // Lado direito do header: Assets e Configurações
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let cfg_color = if state.show_settings {
                            tokens::ACCENT_BLUE
                        } else {
                            tokens::BG_SURFACE
                        };
                        let cfg_btn = egui::Button::new(
                            egui::RichText::new("⚙ Config")
                                .size(11.0)
                                .color(tokens::TEXT_PRIMARY),
                        )
                        .fill(cfg_color)
                        .corner_radius(tokens::RADIUS_CONTROL);

                        if ui
                            .add(cfg_btn)
                            .on_hover_text(
                                "Preferências e Configurações (Tema, Ícones, Idioma, Teclas)",
                            )
                            .clicked()
                        {
                            state.show_settings = !state.show_settings;
                            state.mark_dirty();
                        }

                        let asset_color = if state.show_asset_browser || state.show_asset_library {
                            tokens::ACCENT_BLUE
                        } else {
                            tokens::BG_SURFACE
                        };
                        let asset_btn = egui::Button::new(
                            egui::RichText::new("📦 Assets")
                                .size(11.0)
                                .color(tokens::TEXT_PRIMARY),
                        )
                        .fill(asset_color)
                        .corner_radius(tokens::RADIUS_CONTROL);

                        if ui
                            .add(asset_btn)
                            .on_hover_text("Alternar Painel de Assets do Projeto")
                            .clicked()
                        {
                            state.show_asset_browser = !state.show_asset_browser;
                            state.mark_dirty();
                        }
                    });
                });
            });
        });
}

fn draw_app_brand(ui: &mut Ui) {
    let (rect, response) = ui.allocate_exact_size(vec2(18.0, 18.0), egui::Sense::hover());
    if ui.is_rect_visible(rect) {
        // Losango geométrico elegante da marca Petunia
        let center = rect.center();
        let r = 7.0;
        let points = [
            egui::pos2(center.x, center.y - r),
            egui::pos2(center.x + r, center.y),
            egui::pos2(center.x, center.y + r),
            egui::pos2(center.x - r, center.y),
        ];
        ui.painter().add(egui::Shape::convex_polygon(
            points.to_vec(),
            tokens::ACCENT_BLUE,
            egui::Stroke::new(1.0_f32, tokens::ACCENT_BORDER),
        ));
    }
    response.on_hover_text("Petunia3D v0.5 — Shape-First 3D Creative Suite");
}

fn draw_menus(ui: &mut Ui, state: &mut AppState, action: &mut UiAction) {
    let visuals = ui.visuals_mut();
    visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    visuals.widgets.inactive.bg_fill = Color32::TRANSPARENT;
    visuals.widgets.hovered.weak_bg_fill = tokens::BG_SURFACE_HOVER;
    visuals.widgets.active.weak_bg_fill = tokens::ACCENT_BLUE;

    ui.style_mut().text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(11.5, egui::FontFamily::Proportional),
    );

    ui.menu_button(state.t("menu.file"), |ui| {
        for (key, operation) in [
            ("file.new", 0),
            ("file.open_project", 1),
            ("file.save_asset", 6),
            ("file.save", 2),
            ("file.save_as", 3),
            ("file.import_obj", 4),
            ("file.export_obj", 7),
            ("file.export_glb", 8),
            ("file.quit", 5),
        ] {
            let label = match key {
                "file.save_asset" => "📥 Salvar Modelo Ativo como Asset".to_string(),
                "file.export_obj" => "📦 Exportar OBJ (.obj)...".to_string(),
                "file.export_glb" => "🌐 Exportar GLB (.glb)...".to_string(),
                _ => state.t(key),
            };
            if ui.button(label).clicked() {
                match operation {
                    0 => crate::new_project(state),
                    1 => crate::open_project_dialog(state),
                    2 => crate::save_project_dialog(state, false),
                    3 => crate::save_project_dialog(state, true),
                    4 => crate::import_obj_dialog(state),
                    6 => {
                        state.save_active_as_asset();
                    }
                    7 => crate::export_active_or_all(state, false),
                    8 => crate::export_active_or_all(state, true),
                    _ => action.quit = true,
                }
                ui.close();
            }
        }
    });

    ui.menu_button(state.t("menu.edit"), |ui| {
        if ui
            .add_enabled(
                state.undo.can_undo(),
                egui::Button::new(state.t("edit.undo")),
            )
            .clicked()
        {
            state.undo();
            ui.close();
        }
        if ui
            .add_enabled(
                state.undo.can_redo(),
                egui::Button::new(state.t("edit.redo")),
            )
            .clicked()
        {
            state.redo();
            ui.close();
        }
    });

    ui.menu_button(
        if state.i18n.lang == "en" {
            "Window"
        } else {
            "Janela"
        },
        |ui| {
            ui.checkbox(&mut state.show_perf, "Performance HUD (FPS & Overlays)");
            ui.separator();
            for lang in petunia_config::I18n::available() {
                if ui
                    .selectable_label(state.i18n.lang == lang, &lang)
                    .clicked()
                {
                    state.i18n.set_lang(&lang);
                    state.mark_dirty();
                }
            }
        },
    );

    if ui.button(state.t("menu.help")).clicked() {
        state.show_help = !state.show_help;
    }
}

fn draw_workspace_pills(ui: &mut Ui, state: &mut AppState) {
    let canonical_workspaces = [
        (Workspace::Model, state.t(Workspace::Model.key())),
        (Workspace::Paint, state.t(Workspace::Paint.key())),
        (Workspace::Uv, state.t(Workspace::Uv.key())),
        (Workspace::Animate, state.t(Workspace::Animate.key())),
    ];

    for (ws, label) in canonical_workspaces {
        let is_active = state.workspace == ws;
        let (bg, fg) = if is_active {
            (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
        } else {
            (Color32::TRANSPARENT, tokens::TEXT_SECONDARY)
        };

        let button = egui::Button::new(egui::RichText::new(&label).size(11.5).color(fg))
            .fill(bg)
            .corner_radius(tokens::RADIUS_PILL);

        if ui.add(button).clicked() {
            state.workspace = ws;
            state.mark_dirty();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_header_renders_without_panic() {
        let ctx = Context::default();
        let mut state = AppState::new("en");
        let mut action = UiAction::none();

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            draw(ctx, &mut state, &mut action);
        });
    }

    #[test]
    fn test_main_header_renders_at_different_heights() {
        let mut state = AppState::new("en");
        let mut action = UiAction::none();

        // Testa renderização com tela expandida e múltiplas dimensões
        for width in [800.0, 1280.0, 1920.0] {
            let ctx = Context::default();
            let raw_input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(width, 800.0),
                )),
                ..Default::default()
            };

            let _ = ctx.run(raw_input, |ctx| {
                draw(ctx, &mut state, &mut action);
            });
        }
    }
}
