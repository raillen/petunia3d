//! Tema externo (TOML) — spec §40. Subconjunto aplicado ao egui.

use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Theme {
    pub colors: ThemeColors,
    pub font: ThemeFont,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeColors {
    pub background: String,
    pub panel: String,
    pub accent: String,
    pub text: String,
    pub border: String,
    pub control: String,
    pub hover: String,
    pub selection: String,
    pub text_muted: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeFont {
    pub family: String,
    pub size: f32,
}

impl Default for ThemeFont {
    fn default() -> Self {
        Self { family: "proportional".into(), size: 14.0 }
    }
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            background: "#171b22".into(),
            panel: "#252a32".into(),
            accent: "#83baff".into(),
            text: "#e9edf3".into(),
            border: "#485260".into(),
            control: "#303945".into(),
            hover: "#3b495d".into(),
            selection: "#284d70".into(),
            text_muted: "#b7c2d0".into(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self { colors: ThemeColors::default(), font: ThemeFont::default() }
    }
}

impl Theme {
    pub fn load() -> Self {
        for path in ["assets/themes/dark.toml", "themes/dark.toml"] {
            if let Ok(text) = fs::read_to_string(path) {
                if let Ok(t) = toml::from_str::<Theme>(&text) {
                    return t;
                }
            }
        }
        Self::default()
    }

    fn hex(s: &str) -> Option<egui::Color32> {
        let s = s.trim().trim_start_matches('#');
        if s.len() != 6 || !s.is_ascii() {
            return None;
        }
        let r = u8::from_str_radix(&s[0..2], 16).ok()?;
        let g = u8::from_str_radix(&s[2..4], 16).ok()?;
        let b = u8::from_str_radix(&s[4..6], 16).ok()?;
        Some(egui::Color32::from_rgb(r, g, b))
    }

    /// Aplica ao egui (fundo/painéis/seleção).
    pub fn apply(&self, ctx: &egui::Context) {
        let mut v = egui::Visuals::dark();
        let defaults = ThemeColors::default();
        let color = |value: &str, fallback: &str| Self::hex(value).or_else(|| Self::hex(fallback)).unwrap_or(egui::Color32::WHITE);
        let bg = color(&self.colors.background, &defaults.background);
        let panel = color(&self.colors.panel, &defaults.panel);
        let text = color(&self.colors.text, &defaults.text);
        let accent = color(&self.colors.accent, &defaults.accent);
        let border = color(&self.colors.border, &defaults.border);
        let control = color(&self.colors.control, &defaults.control);
        let hover = color(&self.colors.hover, &defaults.hover);
        let selection = color(&self.colors.selection, &defaults.selection);
        v.extreme_bg_color = bg;
        v.text_edit_bg_color = Some(bg);
        v.panel_fill = panel;
        v.window_fill = panel;
        v.faint_bg_color = control;
        v.weak_text_color = Some(color(&self.colors.text_muted, &defaults.text_muted));
        v.selection.bg_fill = selection;
        v.selection.stroke = egui::Stroke::new(1.0_f32, text);
        v.hyperlink_color = accent;
        v.window_stroke = egui::Stroke::new(1.0_f32, border);
        for widget in [&mut v.widgets.noninteractive, &mut v.widgets.inactive, &mut v.widgets.hovered, &mut v.widgets.active, &mut v.widgets.open] {
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
        let size = if self.font.size.is_finite() { self.font.size.clamp(12.0, 20.0) } else { 14.0 };
        let family = if self.font.family == "monospace" { egui::FontFamily::Monospace } else { egui::FontFamily::Proportional };
        ctx.style_mut(|style| {
            for (name, points) in [(egui::TextStyle::Body, size), (egui::TextStyle::Button, size), (egui::TextStyle::Small, (size - 2.0).max(12.0)), (egui::TextStyle::Heading, size + 4.0)] {
                style.text_styles.insert(name, egui::FontId::new(points, family.clone()));
            }
            style.text_styles.insert(egui::TextStyle::Monospace, egui::FontId::monospace((size - 1.0).max(12.0)));
            style.spacing.item_spacing = egui::vec2(8.0, 6.0);
            style.spacing.button_padding = egui::vec2(10.0, 6.0);
            style.spacing.interact_size.y = 30.0;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn luminance(color: egui::Color32) -> f32 {
        [color.r(), color.g(), color.b()].into_iter().zip([0.2126, 0.7152, 0.0722]).map(|(channel, weight)| {
            let value = channel as f32 / 255.0;
            weight * if value <= 0.04045 { value / 12.92 } else { ((value + 0.055) / 1.055).powf(2.4) }
        }).sum()
    }
    #[test]
    fn default_text_and_icons_meet_contrast_targets_in_interactive_states() {
        let theme = Theme::default();
        for background in [&theme.colors.background, &theme.colors.panel, &theme.colors.control, &theme.colors.hover, &theme.colors.selection] {
            let bg = luminance(Theme::hex(background).unwrap());
            for foreground in [&theme.colors.text, &theme.colors.text_muted] {
                let fg = luminance(Theme::hex(foreground).unwrap());
                assert!((fg.max(bg) + 0.05) / (fg.min(bg) + 0.05) >= 4.5, "{foreground} on {background}");
            }
        }
    }
    #[test]
    fn legacy_partial_theme_and_invalid_font_are_safe() {
        let mut theme: Theme = toml::from_str("[colors]\naccent = '#83baff'\n").unwrap();
        theme.font.size = f32::NAN;
        theme.colors.text = "€abc".into();
        let ctx = egui::Context::default();
        theme.apply(&ctx);
        assert_eq!(ctx.style().text_styles[&egui::TextStyle::Body].size, 14.0);
        assert_eq!(ctx.style().visuals.text_color(), Theme::hex(&ThemeColors::default().text).unwrap());
    }
}
