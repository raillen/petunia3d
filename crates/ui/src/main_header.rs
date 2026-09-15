//! Cabeçalho superior principal do Petunia3D (`Main Header`).
//! Menus do sistema (File, Edit, Render, Window, Help) e abas de Workspaces em pílulas arredondadas.

use egui::{Color32, Ui, vec2};
use petunia_core::{AppState, DocsTopic, Workspace};

use crate::UiAction;
use crate::icon_registry::PetuniaIcon;
use crate::tokens;
use crate::widgets::{
    self, PetuniaMenuCheckboxItem, PetuniaMenuItem, PetuniaMenuRadioItem, petunia_menu_separator,
};

/// Renderiza o cabeçalho superior completo da aplicação.
pub fn draw(ui: &mut Ui, state: &mut AppState, action: &mut UiAction) {
    let header_resp = egui::Panel::top("main_header")
        .default_size(tokens::TOP_HEADER_HEIGHT)
        .size_range(tokens::TOP_HEADER_HEIGHT..=tokens::TOP_HEADER_MAX_HEIGHT)
        .resizable(true)
        .frame(
            egui::Frame::new()
                .fill(tokens::bg_header(state))
                .stroke(tokens::stroke_border_dyn(state))
                .inner_margin(egui::Margin::symmetric(8, 2)),
        )
        .show(ui, |ui| {
            ui.add_enabled_ui(!state.is_interacting(), |ui| {
                ui.horizontal_centered(|ui| {
                    ui.spacing_mut().item_spacing = vec2(6.0, 0.0);

                    // 1. Logotipo e Identidade visual Petunia
                    draw_app_brand(ui);

                    // 2. Menus do sistema no padrão de aplicação criativa (File, Edit, Window, Help)
                    draw_menus(ui, state, action);

                    ui.separator();

                    // 3. Abas de Workspaces em pílulas elegantes (Model, Paint, UV, Animate)
                    draw_workspace_pills(ui, state);

                    // Lado direito do header: Assets e Configurações
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let pref_resp = widgets::petunia_action_button(
                            ui,
                            Some(PetuniaIcon::Settings),
                            "Config",
                            false,
                        );

                        if pref_resp
                            .on_hover_text(
                                "Preferências e Configurações (Tema, Ícones, Idioma, Teclas)",
                            )
                            .clicked()
                        {
                            state.ui.show_settings = !state.ui.show_settings;
                            state.mark_dirty();
                        }

                        let asset_resp = widgets::petunia_action_button(
                            ui,
                            Some(PetuniaIcon::Folder),
                            "Assets",
                            false,
                        );

                        if asset_resp
                            .on_hover_text("Alternar Painel de Assets do Projeto")
                            .clicked()
                        {
                            state.ui.show_asset_browser = !state.ui.show_asset_browser;
                            state.mark_dirty();
                        }
                    });
                });
            });
        });
    crate::regions::record(
        ui.ctx(),
        crate::regions::RegionSlot::Header,
        header_resp.response.rect,
    );
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

    // 1. FILE MENU
    ui.menu_button(state.t("menu.file"), |ui| {
        let new_sc = state.ui.keybinds.format_shortcut("global.new_project");
        if PetuniaMenuItem::new(&state.t("file.new"))
            .shortcut(new_sc.as_deref().or(Some("Ctrl+N")))
            .show(ui)
            .clicked()
        {
            crate::new_project(state);
            ui.close();
        }

        let open_sc = state.ui.keybinds.format_shortcut("global.open_project");
        if PetuniaMenuItem::new(&state.t("file.open_project"))
            .shortcut(open_sc.as_deref().or(Some("Ctrl+O")))
            .show(ui)
            .clicked()
        {
            crate::open_project_dialog(state);
            ui.close();
        }

        let recent_projects = state.project.recent_projects.projects().to_vec();
        ui.menu_button(state.t("menu.recent_projects"), |ui| {
            if recent_projects.is_empty() {
                PetuniaMenuItem::new("No Recent Projects")
                    .enabled(false)
                    .show(ui);
            } else {
                let mut path_to_open = None;
                for entry in &recent_projects {
                    let name = if !entry.name.is_empty() {
                        &entry.name
                    } else {
                        "Untitled"
                    };
                    if PetuniaMenuItem::new(name).show(ui).clicked() {
                        path_to_open = Some(entry.path.clone());
                        ui.close();
                    }
                }
                if let Some(path) = path_to_open
                    && let Err(e) = state.open_project(&path)
                {
                    state.set_status(format!("Failed to open project: {}", e));
                }
            }
        });

        petunia_menu_separator(ui);

        let save_sc = state.ui.keybinds.format_shortcut("global.save_project");
        if PetuniaMenuItem::new(&state.t("file.save"))
            .shortcut(save_sc.as_deref().or(Some("Ctrl+S")))
            .show(ui)
            .clicked()
        {
            crate::save_project_dialog(state, false);
            ui.close();
        }

        let save_as_sc = state.ui.keybinds.format_shortcut("global.save_project_as");
        if PetuniaMenuItem::new(&state.t("file.save_as"))
            .shortcut(save_as_sc.as_deref().or(Some("Ctrl+Shift+S")))
            .show(ui)
            .clicked()
        {
            crate::save_project_dialog(state, true);
            ui.close();
        }

        if PetuniaMenuItem::new("Save Active Model as Asset")
            .show(ui)
            .clicked()
        {
            state.save_active_as_asset();
            ui.close();
        }

        petunia_menu_separator(ui);

        if PetuniaMenuItem::new("Import OBJ (.obj)...")
            .show(ui)
            .clicked()
        {
            crate::import_obj_dialog(state);
            ui.close();
        }

        if PetuniaMenuItem::new("Export OBJ (.obj)...")
            .show(ui)
            .clicked()
        {
            crate::export_active_or_all(state, false);
            ui.close();
        }

        if PetuniaMenuItem::new("Export GLB (.glb)...")
            .show(ui)
            .clicked()
        {
            crate::export_active_or_all(state, true);
            ui.close();
        }

        petunia_menu_separator(ui);

        let quit_sc = state.ui.keybinds.format_shortcut("global.quit");
        if PetuniaMenuItem::new(&state.t("file.quit"))
            .shortcut(quit_sc.as_deref().or(Some("Ctrl+Q")))
            .show(ui)
            .clicked()
        {
            action.quit = true;
            ui.close();
        }
    });

    // 2. EDIT MENU
    ui.menu_button(state.t("menu.edit"), |ui| {
        let undo_sc = state.ui.keybinds.format_shortcut("global.undo");
        if PetuniaMenuItem::new(&state.t("edit.undo"))
            .shortcut(undo_sc.as_deref().or(Some("Ctrl+Z")))
            .enabled(state.project.undo.can_undo())
            .show(ui)
            .clicked()
        {
            state.undo();
            ui.close();
        }

        let redo_sc = state.ui.keybinds.format_shortcut("global.redo");
        if PetuniaMenuItem::new(&state.t("edit.redo"))
            .shortcut(redo_sc.as_deref().or(Some("Ctrl+Y")))
            .enabled(state.project.undo.can_redo())
            .show(ui)
            .clicked()
        {
            state.redo();
            ui.close();
        }

        petunia_menu_separator(ui);

        let cp_sc = state.ui.keybinds.format_shortcut("global.command_palette");
        if PetuniaMenuItem::new(&state.t("menu.command_palette"))
            .shortcut(cp_sc.as_deref().or(Some("Ctrl+P")))
            .show(ui)
            .clicked()
        {
            state.ui.show_command_palette = !state.ui.show_command_palette;
            if state.ui.show_command_palette {
                state.ui.command_palette_query.clear();
                state.ui.command_palette_selected_index = 0;
            }
            state.mark_dirty();
            ui.close();
        }

        let pref_sc = state.ui.keybinds.format_shortcut("global.settings");
        if PetuniaMenuItem::new(&state.t("menu.preferences"))
            .shortcut(pref_sc.as_deref().or(Some("Ctrl+,")))
            .show(ui)
            .clicked()
        {
            state.ui.show_settings = !state.ui.show_settings;
            state.mark_dirty();
            ui.close();
        }
    });

    // 3. WINDOW MENU
    ui.menu_button(state.t("menu.window"), |ui| {
        if PetuniaMenuCheckboxItem::new("Performance HUD", state.ui.show_perf)
            .show(ui)
            .clicked()
        {
            state.ui.show_perf = !state.ui.show_perf;
            state.mark_dirty();
        }

        if PetuniaMenuCheckboxItem::new("Asset Browser", state.ui.show_asset_browser)
            .show(ui)
            .clicked()
        {
            state.ui.show_asset_browser = !state.ui.show_asset_browser;
            state.mark_dirty();
        }

        if PetuniaMenuCheckboxItem::new("Asset Library Drawer", state.ui.show_asset_library)
            .show(ui)
            .clicked()
        {
            state.ui.show_asset_library = !state.ui.show_asset_library;
            state.mark_dirty();
        }

        if PetuniaMenuCheckboxItem::new("Reference Set Manager", state.ui.show_reference_manager)
            .show(ui)
            .clicked()
        {
            state.ui.show_reference_manager = !state.ui.show_reference_manager;
            state.mark_dirty();
        }

        petunia_menu_separator(ui);

        // Language Submenu
        ui.menu_button(state.t("ui.language"), |ui| {
            for lang in petunia_config::I18n::available() {
                let is_active = state.ui.i18n.lang == lang;
                if PetuniaMenuRadioItem::new(&lang, is_active)
                    .show(ui)
                    .clicked()
                {
                    state.ui.i18n.set_lang(&lang);
                    state.mark_dirty();
                    ui.close();
                }
            }
        });

        // Theme Submenu
        ui.menu_button("Theme", |ui| {
            for &(theme_id, label) in &[
                ("petunia-dark", "Petunia Dark (Default)"),
                ("petunia-light", "Petunia Light"),
                ("capuccino", "Capuccino"),
                ("tokyo-nights", "Tokyo Nights"),
            ] {
                let is_active = state.ui.active_theme_id == theme_id;
                if PetuniaMenuRadioItem::new(label, is_active)
                    .show(ui)
                    .clicked()
                {
                    state.ui.active_theme_id = theme_id.to_string();
                    state.mark_dirty();
                    ui.close();
                }
            }
        });
    });

    // 4. HELP MENU
    ui.menu_button(state.t("menu.help"), |ui| {
        let topics = [
            ("Quick Start Guide", DocsTopic::GettingStarted),
            ("Extrude & Modeling", DocsTopic::Extrude),
            ("Keyboard Shortcuts", DocsTopic::Keymaps),
            ("Themes & Styling", DocsTopic::Themes),
            ("Navigation Controls", DocsTopic::Navigation),
        ];

        for (label, topic) in topics {
            if PetuniaMenuItem::new(label).show(ui).clicked() {
                ui.ctx()
                    .open_url(egui::OpenUrl::new_tab(topic.canonical_url()));
                ui.close();
            }
        }

        petunia_menu_separator(ui);

        if PetuniaMenuItem::new("About Petunia3D...")
            .show(ui)
            .clicked()
        {
            state.ui.show_help = true;
            state.mark_dirty();
            ui.close();
        }
    });
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
            // Transição com memória de layout por workspace (Wave 3).
            state.switch_workspace(ws);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_header_renders_without_panic() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");
        let mut action = UiAction::none();

        ctx.run_ui(egui::RawInput::default(), |ui| {
            draw(ui, &mut state, &mut action);
        })
        .textures_delta
        .clear();
    }

    #[test]
    fn test_main_header_renders_at_different_heights() {
        let mut state = AppState::new("en");
        let mut action = UiAction::none();

        // Testa renderização com tela expandida e múltiplas dimensões
        for width in [800.0, 1280.0, 1920.0] {
            let ctx = egui::Context::default();
            let raw_input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(width, 800.0),
                )),
                ..Default::default()
            };

            ctx.run_ui(raw_input, |ui| {
                draw(ui, &mut state, &mut action);
            })
            .textures_delta
            .clear();
        }
    }
}
