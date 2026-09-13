//! Modal central de Preferências e Configurações (`Settings Modal`).
//! Fornece abas para:
//! 1. **Aparência**: Seletor de Temas (Petunia Dark, Light, Capuccino, Tokyo Nights e customizados) com visualizador de tokens de cores;
//! 2. **Ícones**: Seletor de Pacotes de Ícones (Petunia, Phosphor, Tabler, Iconoir, Lucide e customizados) com grade de pré-visualização;
//! 3. **Idioma**: Seletor i18n (pt-BR, en-US) e suporte a novos arquivos TOML de localização;
//! 4. **Atalhos de Teclado (Keymap)**: Seletor dos 8 perfis canônicos, busca de comandos e detecção automática de conflitos.

use egui::{
    vec2, Align, Color32, CornerRadius, FontId, Layout, RichText, ScrollArea, Stroke, Ui, Window,
};
use petunia_config::{Keybinds, ThemeRegistry, ThemeToken};
use petunia_core::AppState;

use crate::icon_registry::{IconRegistry, PetuniaIcon};
use crate::tokens;
use crate::widgets;

/// Renderiza a janela modal de preferências quando `state.show_settings` for verdadeiro.
pub fn draw(ctx: &egui::Context, state: &mut AppState) {
    if !state.show_settings {
        return;
    }

    let mut open = state.show_settings;
    let screen_rect = ctx.screen_rect();
    let default_width = (screen_rect.width() * 0.70).clamp(520.0, 840.0);
    let default_height = (screen_rect.height() * 0.70).clamp(420.0, 680.0);

    Window::new(RichText::new(state.t("settings.title")).strong().size(14.0))
        .open(&mut open)
        .default_size(vec2(default_width, default_height))
        .min_size(vec2(460.0, 360.0))
        .resizable(true)
        .collapsible(false)
        .frame(
            egui::Frame::window(&ctx.style())
                .fill(tokens::BG_PANEL)
                .stroke(tokens::stroke_border())
                .inner_margin(egui::Margin::same(12)),
        )
        .show(ctx, |ui| {
            draw_settings_content(ctx, ui, state);
        });

    state.show_settings = open;
}

fn draw_settings_content(ctx: &egui::Context, ui: &mut Ui, state: &mut AppState) {
    // 1. Barra de Abas de Configuração
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(6.0, 0.0);

        let tabs = [
            (
                "appearance",
                "🎨 Aparência",
                "Temas visuais, paletas e tokens semânticos",
            ),
            (
                "icons",
                "✨ Ícones",
                "Pacotes de ícones e símbolos da interface",
            ),
            (
                "language",
                "🌐 Idioma",
                "Localização, traduções e arquivos TOML",
            ),
            (
                "keymap",
                "⌨ Atalhos",
                "Perfis de keymap e detecção de conflitos",
            ),
        ];

        for (tab_id, label, hint) in tabs {
            let is_selected = state.settings_tab == tab_id;
            let (bg, fg) = if is_selected {
                (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
            } else {
                (tokens::BG_SURFACE, tokens::TEXT_SECONDARY)
            };

            let btn = egui::Button::new(RichText::new(label).size(12.0).color(fg))
                .fill(bg)
                .corner_radius(tokens::RADIUS_CONTROL);

            if ui.add(btn).on_hover_text(hint).clicked() {
                state.settings_tab = tab_id.to_string();
                state.mark_dirty();
            }
        }
    });

    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);

    // 2. Conteúdo da Aba Ativa
    match state.settings_tab.as_str() {
        "appearance" => draw_appearance_tab(ctx, ui, state),
        "icons" => draw_icons_tab(ui, state),
        "language" => draw_language_tab(ui, state),
        "keymap" => draw_keymap_tab(ui, state),
        _ => draw_appearance_tab(ctx, ui, state),
    }
}

// ---------------------------------------------------------------- Aba 1: Aparência e Temas
fn draw_appearance_tab(ctx: &egui::Context, ui: &mut Ui, state: &mut AppState) {
    let registry = ThemeRegistry::global();
    let themes = registry.available();

    ui.label(
        RichText::new("Temas do Sistema")
            .strong()
            .size(13.0)
            .color(tokens::TEXT_PRIMARY),
    );
    ui.label(
        RichText::new("O Petunia3D suporta temas personalizados em arquivos TOML. Crie uma pasta em 'assets/themes/<nome>/' contendo 'manifest.toml' e 'theme.toml'.")
            .size(11.0)
            .color(tokens::TEXT_SECONDARY),
    );

    ui.add_space(8.0);

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for manifest in themes {
                let is_active = state.active_theme_id == manifest.id;
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
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(&manifest.name)
                                            .strong()
                                            .size(13.0)
                                            .color(tokens::TEXT_PRIMARY),
                                    );
                                    if is_active {
                                        ui.label(
                                            RichText::new("● Ativo")
                                                .size(10.5)
                                                .color(tokens::ACCENT_BLUE),
                                        );
                                    }
                                });
                                if let Some(ref desc) = manifest.description {
                                    ui.label(
                                        RichText::new(desc)
                                            .size(11.0)
                                            .color(tokens::TEXT_SECONDARY),
                                    );
                                }
                            });

                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if !is_active {
                                    if ui
                                        .button(RichText::new("Aplicar Tema").size(11.0))
                                        .clicked()
                                    {
                                        state.active_theme_id = manifest.id.clone();
                                        if let Some(t) = registry.get_theme(&manifest.id) {
                                            t.apply(ctx);
                                        }
                                        state.mark_dirty();
                                    }
                                } else {
                                    ui.label(
                                        RichText::new("✔ Aplicado").color(tokens::ACCENT_BLUE),
                                    );
                                }
                            });
                        });

                        // Amostras de cores do tema
                        if let Some(theme) = registry.get_theme(&manifest.id) {
                            ui.add_space(6.0);
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new("Tokens:")
                                        .size(10.5)
                                        .color(tokens::TEXT_MUTED),
                                );
                                for (name, token) in [
                                    ("Canvas", ThemeToken::BgCanvas),
                                    ("Header", ThemeToken::BgHeader),
                                    ("Panel", ThemeToken::BgPanel),
                                    ("Surface", ThemeToken::BgSurface),
                                    ("Accent", ThemeToken::AccentBlue),
                                    ("Orange", ThemeToken::AccentOrange),
                                    ("Border", ThemeToken::BorderSubtle),
                                ] {
                                    let col = theme.colors.get_token_color(token);
                                    let (rect, resp) = ui.allocate_exact_size(
                                        vec2(18.0, 14.0),
                                        egui::Sense::hover(),
                                    );
                                    ui.painter().rect_filled(rect, 2.0, col);
                                    ui.painter().rect_stroke(
                                        rect,
                                        2.0,
                                        Stroke::new(1.0_f32, Color32::from_gray(80)),
                                        egui::StrokeKind::Inside,
                                    );
                                    resp.on_hover_text(name);
                                }
                            });
                        }
                    });

                ui.add_space(6.0);
            }
        });
}

// ---------------------------------------------------------------- Aba 2: Pacotes de Ícones
fn draw_icons_tab(ui: &mut Ui, state: &mut AppState) {
    let packs = IconRegistry::available_packs();

    ui.label(
        RichText::new("Pacotes de Ícones")
            .strong()
            .size(13.0)
            .color(tokens::TEXT_PRIMARY),
    );
    ui.label(
        RichText::new("Selecione a família de ícones ativa. Você pode adicionar novos pacotes criando subpastas em 'assets/icons/<pacote>/' com 'manifest.toml' e 'icons.toml'.")
            .size(11.0)
            .color(tokens::TEXT_SECONDARY),
    );

    ui.add_space(8.0);

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for pack in packs {
                let is_active = state.active_icon_pack_id == pack.id;
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
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(&pack.name)
                                            .strong()
                                            .size(13.0)
                                            .color(tokens::TEXT_PRIMARY),
                                    );
                                    if is_active {
                                        ui.label(
                                            RichText::new("● Ativo")
                                                .size(10.5)
                                                .color(tokens::ACCENT_BLUE),
                                        );
                                    }
                                });
                                if let Some(ref desc) = pack.description {
                                    ui.label(
                                        RichText::new(desc)
                                            .size(11.0)
                                            .color(tokens::TEXT_SECONDARY),
                                    );
                                }
                            });

                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if !is_active {
                                    if ui
                                        .button(RichText::new("Usar este Pacote").size(11.0))
                                        .clicked()
                                    {
                                        state.active_icon_pack_id = pack.id.clone();
                                        state.mark_dirty();
                                    }
                                } else {
                                    ui.label(
                                        RichText::new("✔ Selecionado").color(tokens::ACCENT_BLUE),
                                    );
                                }
                            });
                        });

                        // Pré-visualização de ícones representativos
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("Exemplos:")
                                    .size(10.5)
                                    .color(tokens::TEXT_MUTED),
                            );
                            for (label, icon) in [
                                ("Mover", PetuniaIcon::Move),
                                ("Girar", PetuniaIcon::Rotate),
                                ("Escalar", PetuniaIcon::Scale),
                                ("Medir", PetuniaIcon::Measure),
                                ("Anotar", PetuniaIcon::Annotate),
                                ("Busca", PetuniaIcon::Search),
                                ("Pasta", PetuniaIcon::Folder),
                                ("Config", PetuniaIcon::Settings),
                            ] {
                                let (rect, resp) =
                                    ui.allocate_exact_size(vec2(20.0, 20.0), egui::Sense::hover());
                                IconRegistry::paint(
                                    ui.ctx(),
                                    ui.painter(),
                                    &icon,
                                    rect,
                                    tokens::TEXT_PRIMARY,
                                );
                                resp.on_hover_text(label);
                            }
                        });
                    });

                ui.add_space(6.0);
            }
        });
}

// ---------------------------------------------------------------- Aba 3: Idioma e Tradução
fn draw_language_tab(ui: &mut Ui, state: &mut AppState) {
    ui.label(
        RichText::new("Idioma da Interface (i18n)")
            .strong()
            .size(13.0)
            .color(tokens::TEXT_PRIMARY),
    );
    ui.label(
        RichText::new("Todo o texto da aplicação é mapeado através de arquivos TOML em 'assets/locales/<idioma>.toml'. Qualquer usuário pode adicionar novos idiomas ou customizar textos com facilidade.")
            .size(11.0)
            .color(tokens::TEXT_SECONDARY),
    );

    ui.add_space(10.0);

    let languages = [
        ("pt-BR", "Português do Brasil", "assets/locales/pt-BR.toml"),
        ("en", "English (United States)", "assets/locales/en.toml"),
    ];

    for (code, name, file_path) in languages {
        let is_active = state.i18n.lang == code || (code == "en" && state.i18n.lang == "en-US");
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
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(name)
                                    .strong()
                                    .size(13.0)
                                    .color(tokens::TEXT_PRIMARY),
                            );
                            if is_active {
                                ui.label(
                                    RichText::new("● Idioma Ativo")
                                        .size(10.5)
                                        .color(tokens::ACCENT_BLUE),
                                );
                            }
                        });
                        ui.label(
                            RichText::new(format!("Código: {code} · Arquivo: {file_path}"))
                                .size(11.0)
                                .color(tokens::TEXT_SECONDARY),
                        );
                    });

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if !is_active {
                            if ui.button(RichText::new("Selecionar").size(11.0)).clicked() {
                                state.i18n.set_lang(code);
                                state.mark_dirty();
                            }
                        } else {
                            ui.label(RichText::new("✔ Selecionado").color(tokens::ACCENT_BLUE));
                        }
                    });
                });
            });

        ui.add_space(6.0);
    }
}

// ---------------------------------------------------------------- Aba 4: Keymaps e Atalhos
fn draw_keymap_tab(ui: &mut Ui, state: &mut AppState) {
    let profiles = Keybinds::available_profiles();

    ui.label(
        RichText::new("Perfis de Teclado (Keymaps)")
            .strong()
            .size(13.0)
            .color(tokens::TEXT_PRIMARY),
    );
    ui.label(
        RichText::new("Alterne entre o mapa nativo do Petunia3D e padrões consagrados da indústria como Blender, Maya, 3ds Max e Cinema 4D.")
            .size(11.0)
            .color(tokens::TEXT_SECONDARY),
    );

    ui.add_space(8.0);

    // Seletor de Perfil Ativo
    ui.horizontal(|ui| {
        ui.label(RichText::new("Perfil Ativo:").strong().size(12.0));
        for profile in &profiles {
            let is_active = state.active_keymap_id == profile.id;
            let (bg, fg) = if is_active {
                (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
            } else {
                (tokens::BG_SURFACE, tokens::TEXT_SECONDARY)
            };

            let btn = egui::Button::new(RichText::new(&profile.name).size(11.0).color(fg))
                .fill(bg)
                .corner_radius(tokens::RADIUS_CONTROL);

            if ui.add(btn).on_hover_text(&profile.description).clicked() {
                state.active_keymap_id = profile.id.clone();
                state.keybinds = Keybinds::load_profile(&profile.id);
                state.mark_dirty();
            }
        }
    });

    ui.add_space(8.0);

    // Verificação de Conflitos
    let conflicts = state.keybinds.detect_conflicts();
    if !conflicts.is_empty() {
        egui::Frame::new()
            .fill(Color32::from_rgba_unmultiplied(239, 83, 80, 30))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(239, 83, 80)))
            .corner_radius(CornerRadius::same(4))
            .inner_margin(egui::Margin::symmetric(10, 6))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("⚠ Atenção: Conflito de atalhos detectado!")
                            .strong()
                            .color(Color32::from_rgb(255, 120, 120)),
                    );
                });
                for (a, b, key) in &conflicts {
                    ui.small(format!(
                        "• '{a}' e '{b}' compartilham o mesmo atalho: [{key}]"
                    ));
                }
            });
        ui.add_space(8.0);
    }

    // Busca e Listagem de Atalhos
    let search_id = egui::Id::new("keymap_search_query");
    let mut search_query = ui
        .ctx()
        .data_mut(|d| d.get_temp::<String>(search_id).unwrap_or_default());

    ui.horizontal(|ui| {
        let resp = widgets::petunia_search_box(
            ui,
            &mut search_query,
            "Filter shortcuts by action or key...",
        );
        if resp.changed() {
            ui.ctx()
                .data_mut(|d| d.insert_temp(search_id, search_query.clone()));
        }
    });

    ui.add_space(6.0);

    let all_bindings = state.keybinds.all_bindings();

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            egui::Grid::new("keymaps_table_grid")
                .striped(true)
                .min_col_width(200.0)
                .spacing(vec2(16.0, 6.0))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new("Ação / Comando")
                            .strong()
                            .color(tokens::TEXT_SECONDARY),
                    );
                    ui.label(
                        RichText::new("Atalho de Teclado")
                            .strong()
                            .color(tokens::TEXT_SECONDARY),
                    );
                    ui.end_row();

                    for (action, shortcut) in all_bindings {
                        if !search_query.is_empty()
                            && !action.to_lowercase().contains(&search_query.to_lowercase())
                            && !shortcut
                                .to_lowercase()
                                .contains(&search_query.to_lowercase())
                        {
                            continue;
                        }

                        ui.label(
                            RichText::new(&action)
                                .size(11.5)
                                .color(tokens::TEXT_PRIMARY),
                        );

                        // Badge de tecla
                        ui.horizontal(|ui| {
                            let (badge_rect, _) = ui.allocate_exact_size(
                                vec2(shortcut.len() as f32 * 7.5 + 14.0, 20.0),
                                egui::Sense::hover(),
                            );
                            ui.painter()
                                .rect_filled(badge_rect, 3.0, tokens::BG_SURFACE_HOVER);
                            ui.painter().rect_stroke(
                                badge_rect,
                                3.0,
                                Stroke::new(1.0_f32, tokens::BORDER_SUBTLE),
                                egui::StrokeKind::Inside,
                            );
                            ui.painter().text(
                                badge_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                &shortcut,
                                FontId::monospace(11.0),
                                tokens::TEXT_ACTIVE,
                            );
                        });

                        ui.end_row();
                    }
                });
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_modal_renders_all_tabs_without_panic() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");
        state.show_settings = true;

        for tab in ["appearance", "icons", "language", "keymap"] {
            state.settings_tab = tab.into();
            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                draw(ctx, &mut state);
            });
        }
    }
}
