//! Tema externo (TOML) e registro centralizado de temas do Petunia3D.
//! Suporta tokens semânticos (`ThemeToken`), múltiplos temas embutidos e temas personalizados
//! criados por usuários em subpastas contendo `manifest.toml` e `theme.toml`.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use egui::Color32;
use serde::{Deserialize, Serialize};

/// Tokens canônicos de cores do sistema de design do Petunia3D.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThemeToken {
    BgCanvas,
    BgHeader,
    BgPanel,
    BgPanelHeader,
    BgSurface,
    BgSurfaceHover,
    BgSurfaceActive,
    TextPrimary,
    TextSecondary,
    TextMuted,
    TextActive,
    AccentBlue,
    AccentOrange,
    AccentHover,
    AccentBorder,
    BorderSubtle,
    BorderStrong,
    BorderFocus,
    StatusInfo,
    StatusWarning,
    StatusError,
    StatusSuccess,
}

/// Metadados de manifesto do tema (`manifest.toml`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeFile {
    pub theme: ThemeManifest,
}

/// Tema completo do Petunia3D.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Theme {
    #[serde(skip)]
    pub manifest: Option<ThemeManifest>,
    pub colors: ThemeColors,
    pub font: ThemeFont,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeColors {
    pub bg_canvas: String,
    pub bg_header: String,
    pub bg_panel: String,
    pub bg_panel_header: String,
    pub bg_surface: String,
    pub bg_surface_hover: String,
    pub bg_surface_active: String,
    pub text_primary: String,
    pub text_secondary: String,
    pub text_muted: String,
    pub text_active: String,
    pub accent_blue: String,
    pub accent_orange: String,
    pub accent_hover: String,
    pub accent_border: String,
    pub border_subtle: String,
    pub border_strong: String,
    pub border_focus: String,
    pub status_info: String,
    pub status_warning: String,
    pub status_error: String,
    pub status_success: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeFont {
    pub family: String,
    pub size: f32,
}

impl Default for ThemeFont {
    fn default() -> Self {
        Self {
            family: "proportional".into(),
            size: 14.0,
        }
    }
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            bg_canvas: "#17181c".into(),
            bg_header: "#202126".into(),
            bg_panel: "#24252a".into(),
            bg_panel_header: "#292a30".into(),
            bg_surface: "#30323a".into(),
            bg_surface_hover: "#3b3d47".into(),
            bg_surface_active: "#474a56".into(),
            text_primary: "#f0f2f5".into(),
            text_secondary: "#a3a7b5".into(),
            text_muted: "#6e7280".into(),
            text_active: "#ffffff".into(),
            accent_blue: "#3b82f6".into(),
            accent_orange: "#f97316".into(),
            accent_hover: "#60a5fa".into(),
            accent_border: "#2563eb".into(),
            border_subtle: "#32343c".into(),
            border_strong: "#434652".into(),
            border_focus: "#3b82f6".into(),
            status_info: "#38bdf8".into(),
            status_warning: "#fbbf24".into(),
            status_error: "#f87171".into(),
            status_success: "#4ade80".into(),
        }
    }
}

impl ThemeColors {
    /// Converte um token canônico na cor correspondente deste tema.
    pub fn get_token_color(&self, token: ThemeToken) -> Color32 {
        let hex = match token {
            ThemeToken::BgCanvas => &self.bg_canvas,
            ThemeToken::BgHeader => &self.bg_header,
            ThemeToken::BgPanel => &self.bg_panel,
            ThemeToken::BgPanelHeader => &self.bg_panel_header,
            ThemeToken::BgSurface => &self.bg_surface,
            ThemeToken::BgSurfaceHover => &self.bg_surface_hover,
            ThemeToken::BgSurfaceActive => &self.bg_surface_active,
            ThemeToken::TextPrimary => &self.text_primary,
            ThemeToken::TextSecondary => &self.text_secondary,
            ThemeToken::TextMuted => &self.text_muted,
            ThemeToken::TextActive => &self.text_active,
            ThemeToken::AccentBlue => &self.accent_blue,
            ThemeToken::AccentOrange => &self.accent_orange,
            ThemeToken::AccentHover => &self.accent_hover,
            ThemeToken::AccentBorder => &self.accent_border,
            ThemeToken::BorderSubtle => &self.border_subtle,
            ThemeToken::BorderStrong => &self.border_strong,
            ThemeToken::BorderFocus => &self.border_focus,
            ThemeToken::StatusInfo => &self.status_info,
            ThemeToken::StatusWarning => &self.status_warning,
            ThemeToken::StatusError => &self.status_error,
            ThemeToken::StatusSuccess => &self.status_success,
        };

        Theme::hex(hex).unwrap_or_else(|| {
            // Fallback para as cores padrão
            let defaults = ThemeColors::default();
            let def_hex = match token {
                ThemeToken::BgCanvas => &defaults.bg_canvas,
                ThemeToken::BgHeader => &defaults.bg_header,
                ThemeToken::BgPanel => &defaults.bg_panel,
                ThemeToken::BgPanelHeader => &defaults.bg_panel_header,
                ThemeToken::BgSurface => &defaults.bg_surface,
                ThemeToken::BgSurfaceHover => &defaults.bg_surface_hover,
                ThemeToken::BgSurfaceActive => &defaults.bg_surface_active,
                ThemeToken::TextPrimary => &defaults.text_primary,
                ThemeToken::TextSecondary => &defaults.text_secondary,
                ThemeToken::TextMuted => &defaults.text_muted,
                ThemeToken::TextActive => &defaults.text_active,
                ThemeToken::AccentBlue => &defaults.accent_blue,
                ThemeToken::AccentOrange => &defaults.accent_orange,
                ThemeToken::AccentHover => &defaults.accent_hover,
                ThemeToken::AccentBorder => &defaults.accent_border,
                ThemeToken::BorderSubtle => &defaults.border_subtle,
                ThemeToken::BorderStrong => &defaults.border_strong,
                ThemeToken::BorderFocus => &defaults.border_focus,
                ThemeToken::StatusInfo => &defaults.status_info,
                ThemeToken::StatusWarning => &defaults.status_warning,
                ThemeToken::StatusError => &defaults.status_error,
                ThemeToken::StatusSuccess => &defaults.status_success,
            };
            Theme::hex(def_hex).unwrap_or(Color32::WHITE)
        })
    }
}

impl Theme {
    /// Carrega o tema pelo identificador. Se não encontrar, tenta carregar o tema Petunia Dark.
    pub fn load_by_id(id: &str) -> Self {
        let registry = ThemeRegistry::global();
        registry.get_theme(id).cloned().unwrap_or_default()
    }

    /// Carrega o tema padrão do sistema (`petunia-dark`).
    pub fn load() -> Self {
        Self::load_by_id("petunia-dark")
    }

    pub fn hex(s: &str) -> Option<Color32> {
        let s = s.trim().trim_start_matches('#');
        if s.len() != 6 || !s.is_ascii() {
            return None;
        }
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some(Color32::from_rgb(r, g, b))
    }

    /// Aplica as configurações do tema ao contexto egui.
    pub fn apply(&self, ctx: &egui::Context) {
        let is_light = self
            .manifest
            .as_ref()
            .map(|m| m.id.contains("light"))
            .unwrap_or(false);
        let mut v = if is_light {
            egui::Visuals::light()
        } else {
            egui::Visuals::dark()
        };

        let bg = self.colors.get_token_color(ThemeToken::BgCanvas);
        let panel = self.colors.get_token_color(ThemeToken::BgPanel);
        let text = self.colors.get_token_color(ThemeToken::TextPrimary);
        let text_muted = self.colors.get_token_color(ThemeToken::TextMuted);
        let accent = self.colors.get_token_color(ThemeToken::AccentBlue);
        let border = self.colors.get_token_color(ThemeToken::BorderSubtle);
        let control = self.colors.get_token_color(ThemeToken::BgSurface);
        let hover = self.colors.get_token_color(ThemeToken::BgSurfaceHover);
        let selection = self.colors.get_token_color(ThemeToken::BgSurfaceActive);

        v.extreme_bg_color = bg;
        v.text_edit_bg_color = Some(control);
        v.panel_fill = panel;
        v.window_fill = panel;
        v.faint_bg_color = control;
        v.weak_text_color = Some(text_muted);
        v.selection.bg_fill = selection;
        v.selection.stroke = egui::Stroke::new(1.0_f32, text);
        v.hyperlink_color = accent;
        v.window_stroke = egui::Stroke::new(1.0_f32, border);

        for widget in [
            &mut v.widgets.noninteractive,
            &mut v.widgets.inactive,
            &mut v.widgets.hovered,
            &mut v.widgets.active,
            &mut v.widgets.open,
        ] {
            widget.fg_stroke = egui::Stroke::new(1.5_f32, text);
            widget.bg_stroke = egui::Stroke::new(1.0_f32, border);
            widget.corner_radius = egui::CornerRadius::same(5);
            widget.expansion = 0.0;
        }

        v.widgets.noninteractive.bg_fill = panel;
        v.widgets.noninteractive.weak_bg_fill = panel;
        v.widgets.inactive.bg_fill = control;
        v.widgets.inactive.weak_bg_fill = control;
        v.widgets.hovered.bg_fill = hover;
        v.widgets.hovered.weak_bg_fill = hover;
        v.widgets.hovered.bg_stroke = egui::Stroke::new(1.0_f32, accent);
        v.widgets.active.bg_fill = selection;
        v.widgets.active.weak_bg_fill = selection;
        v.widgets.active.bg_stroke = egui::Stroke::new(1.5_f32, accent);
        v.widgets.open = v.widgets.active;

        ctx.set_visuals(v);

        let size = if self.font.size.is_finite() {
            self.font.size.clamp(12.0, 20.0)
        } else {
            14.0
        };
        let family = if self.font.family == "monospace" {
            egui::FontFamily::Monospace
        } else {
            egui::FontFamily::Proportional
        };

        ctx.style_mut(|style| {
            for (name, points) in [
                (egui::TextStyle::Body, size),
                (egui::TextStyle::Button, size),
                (egui::TextStyle::Small, (size - 2.0).max(11.0)),
                (egui::TextStyle::Heading, size + 4.0),
            ] {
                style
                    .text_styles
                    .insert(name, egui::FontId::new(points, family.clone()));
            }
            style.text_styles.insert(
                egui::TextStyle::Monospace,
                egui::FontId::monospace((size - 1.0).max(11.0)),
            );
            style.spacing.item_spacing = egui::vec2(8.0, 6.0);
            style.spacing.button_padding = egui::vec2(8.0, 5.0);
            style.spacing.interact_size.y = 28.0;
        });
    }
}

/// Registro central de descoberta e carregamento de temas instalados.
pub struct ThemeRegistry {
    themes: HashMap<String, Theme>,
    manifests: Vec<ThemeManifest>,
}

impl ThemeRegistry {
    /// Obtém instância singleton ou carrega temas disponíveis.
    pub fn global() -> Self {
        let mut reg = Self {
            themes: HashMap::new(),
            manifests: Vec::new(),
        };
        reg.scan_and_load();
        reg
    }

    /// Retorna a lista de manifestos de todos os temas válidos encontrados.
    pub fn available(&self) -> &[ThemeManifest] {
        &self.manifests
    }

    /// Busca um tema carregado pelo ID.
    pub fn get_theme(&self, id: &str) -> Option<&Theme> {
        self.themes
            .get(id)
            .or_else(|| self.themes.get("petunia-dark"))
    }

    /// Escaneia pastas de temas em busca de `manifest.toml` e `theme.toml`.
    pub fn scan_and_load(&mut self) {
        // 1. Carrega os 4 temas built-in garantidos em memória
        self.register_builtin_themes();

        // 2. Escaneia diretórios do disco (assets/themes e diretório local)
        let candidate_dirs = ["assets/themes", "themes"];
        for dir_name in candidate_dirs {
            let path = PathBuf::from(dir_name);
            if let Ok(entries) = fs::read_dir(&path) {
                for entry in entries.flatten() {
                    let subpath = entry.path();
                    if subpath.is_dir() {
                        self.try_load_theme_folder(&subpath);
                    }
                }
            }
        }
    }

    fn try_load_theme_folder(&mut self, folder: &Path) {
        let manifest_path = folder.join("manifest.toml");
        let theme_path = folder.join("theme.toml");

        if !manifest_path.exists() || !theme_path.exists() {
            return;
        }

        // Tenta ler manifest
        let manifest_text = match fs::read_to_string(&manifest_path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!(
                    "[Petunia3D ThemeRegistry] Aviso: Falha ao ler '{:?}': {e}",
                    manifest_path
                );
                return;
            }
        };

        let manifest: ThemeManifest = match toml::from_str::<ThemeFile>(&manifest_text) {
            Ok(tf) => tf.theme,
            Err(_) => match toml::from_str::<ThemeManifest>(&manifest_text) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!(
                        "[Petunia3D ThemeRegistry] Aviso: manifest.toml inválido em '{:?}': {e}. Usando fallback Petunia Dark.",
                        folder
                    );
                    return;
                }
            },
        };

        // Tenta ler theme.toml
        let theme_text = match fs::read_to_string(&theme_path) {
            Ok(t) => t,
            Err(e) => {
                eprintln!(
                    "[Petunia3D ThemeRegistry] Aviso: Falha ao ler '{:?}': {e}",
                    theme_path
                );
                return;
            }
        };

        let mut theme: Theme = match toml::from_str(&theme_text) {
            Ok(t) => t,
            Err(e) => {
                eprintln!(
                    "[Petunia3D ThemeRegistry] Aviso: theme.toml inválido em '{:?}': {e}. Usando fallback Petunia Dark.",
                    folder
                );
                return;
            }
        };

        theme.manifest = Some(manifest.clone());

        // Atualiza ou insere
        if let Some(existing) = self.manifests.iter_mut().find(|m| m.id == manifest.id) {
            *existing = manifest.clone();
        } else {
            self.manifests.push(manifest.clone());
        }
        self.themes.insert(manifest.id, theme);
    }

    fn register_builtin_themes(&mut self) {
        // Petunia Dark
        let dark_manifest = ThemeManifest {
            id: "petunia-dark".into(),
            name: "Petunia Dark".into(),
            version: "1.0.0".into(),
            author: Some("Petunia3D Team".into()),
            description: Some("Tema escuro canônico padrão".into()),
        };
        let dark_theme = Theme {
            manifest: Some(dark_manifest.clone()),
            ..Default::default()
        };
        self.manifests.push(dark_manifest.clone());
        self.themes.insert("petunia-dark".into(), dark_theme);

        // Petunia Light
        let light_manifest = ThemeManifest {
            id: "petunia-light".into(),
            name: "Petunia Light".into(),
            version: "1.0.0".into(),
            author: Some("Petunia3D Team".into()),
            description: Some("Tema claro técnico e limpo".into()),
        };
        let light_colors = ThemeColors {
            bg_canvas: "#eaecef".into(),
            bg_header: "#f3f4f6".into(),
            bg_panel: "#f8f9fa".into(),
            bg_panel_header: "#e9ebed".into(),
            bg_surface: "#ffffff".into(),
            bg_surface_hover: "#e2e5e9".into(),
            bg_surface_active: "#d1d5db".into(),
            text_primary: "#1f2937".into(),
            text_secondary: "#4b5563".into(),
            text_muted: "#9ca3af".into(),
            text_active: "#111827".into(),
            accent_blue: "#2563eb".into(),
            border_subtle: "#e5e7eb".into(),
            border_strong: "#cbd5e1".into(),
            ..Default::default()
        };

        let light_theme = Theme {
            manifest: Some(light_manifest.clone()),
            colors: light_colors,
            font: ThemeFont::default(),
        };
        self.manifests.push(light_manifest.clone());
        self.themes.insert("petunia-light".into(), light_theme);

        // Petunia Capuccino
        let cap_manifest = ThemeManifest {
            id: "petunia-capuccino".into(),
            name: "Petunia Capuccino".into(),
            version: "1.0.0".into(),
            author: Some("Petunia3D Team".into()),
            description: Some("Tons terrosos e aconchegantes de café".into()),
        };
        let cap_colors = ThemeColors {
            bg_canvas: "#231e1c".into(),
            bg_header: "#2b2522".into(),
            bg_panel: "#302926".into(),
            bg_panel_header: "#362e2a".into(),
            bg_surface: "#3f3631".into(),
            bg_surface_hover: "#4d423c".into(),
            bg_surface_active: "#5c4f48".into(),
            text_primary: "#f5ede8".into(),
            text_secondary: "#c4b5ac".into(),
            text_muted: "#8c7e75".into(),
            accent_blue: "#d97736".into(),
            ..Default::default()
        };

        let cap_theme = Theme {
            manifest: Some(cap_manifest.clone()),
            colors: cap_colors,
            font: ThemeFont::default(),
        };
        self.manifests.push(cap_manifest.clone());
        self.themes.insert("petunia-capuccino".into(), cap_theme);

        // Petunia Tokyo Nights
        let tokyo_manifest = ThemeManifest {
            id: "petunia-tokyo-nights".into(),
            name: "Petunia Tokyo Nights".into(),
            version: "1.0.0".into(),
            author: Some("Petunia3D Team".into()),
            description: Some("Deep neon cyberpunk indigo e magenta".into()),
        };
        let tokyo_colors = ThemeColors {
            bg_canvas: "#13141f".into(),
            bg_header: "#1a1b26".into(),
            bg_panel: "#1f2335".into(),
            bg_panel_header: "#24283b".into(),
            bg_surface: "#292e42".into(),
            bg_surface_hover: "#3b4261".into(),
            bg_surface_active: "#414868".into(),
            text_primary: "#c0caf5".into(),
            text_secondary: "#9aa5ce".into(),
            text_muted: "#565f89".into(),
            accent_blue: "#7aa2f7".into(),
            accent_orange: "#bb9af7".into(),
            ..Default::default()
        };

        let tokyo_theme = Theme {
            manifest: Some(tokyo_manifest.clone()),
            colors: tokyo_colors,
            font: ThemeFont::default(),
        };
        self.manifests.push(tokyo_manifest.clone());
        self.themes
            .insert("petunia-tokyo-nights".into(), tokyo_theme);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_registry_loads_all_4_builtin_themes() {
        let registry = ThemeRegistry::global();
        let available = registry.available();
        assert!(available.len() >= 4);

        for id in [
            "petunia-dark",
            "petunia-light",
            "petunia-capuccino",
            "petunia-tokyo-nights",
        ] {
            let theme = registry.get_theme(id);
            assert!(theme.is_some(), "Theme {id} must be registered");
            let theme = theme.unwrap();
            assert_eq!(theme.manifest.as_ref().unwrap().id, id);
        }
    }

    #[test]
    fn test_theme_tokens_resolve_valid_colors() {
        let registry = ThemeRegistry::global();
        for manifest in registry.available() {
            let theme = registry.get_theme(&manifest.id).unwrap();
            for token in [
                ThemeToken::BgCanvas,
                ThemeToken::BgHeader,
                ThemeToken::BgPanel,
                ThemeToken::BgSurface,
                ThemeToken::TextPrimary,
                ThemeToken::AccentBlue,
                ThemeToken::BorderSubtle,
            ] {
                let c = theme.colors.get_token_color(token);
                assert_ne!(c, Color32::TRANSPARENT);
            }
        }
    }

    #[test]
    fn test_theme_apply_sets_visuals_without_panic() {
        let ctx = egui::Context::default();
        let registry = ThemeRegistry::global();
        for manifest in registry.available() {
            let theme = registry.get_theme(&manifest.id).unwrap();
            theme.apply(&ctx);
        }
    }
}
