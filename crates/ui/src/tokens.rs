//! Design tokens e paleta semântica canônica para o Petunia3D.
//! Baseado na Golden Reference visual (`Blender.svg`) e sistema de design desktop profissional.

use egui::{Color32, CornerRadius, Stroke};

// ---------------------------------------------------------------- Cores de Fundo
pub const BG_APP: Color32 = Color32::from_rgb(0x12, 0x12, 0x12);
pub const BG_HEADER: Color32 = Color32::from_rgb(0x1a, 0x1a, 0x1a);
pub const BG_PANEL: Color32 = Color32::from_rgb(0x20, 0x20, 0x20);
pub const BG_PANEL_HEADER: Color32 = Color32::from_rgb(0x28, 0x28, 0x28);
pub const BG_SURFACE: Color32 = Color32::from_rgb(0x2d, 0x2d, 0x2d);
pub const BG_SURFACE_HOVER: Color32 = Color32::from_rgb(0x38, 0x38, 0x38);
pub const BG_SURFACE_ACTIVE: Color32 = Color32::from_rgb(0x31, 0x69, 0xe3);
pub const BG_INPUT: Color32 = Color32::from_rgb(0x16, 0x16, 0x16);
pub const BG_DROPDOWN: Color32 = Color32::from_rgb(0x22, 0x22, 0x22);
pub const BG_SHELF: Color32 = Color32::from_rgba_premultiplied(0x1e, 0x1e, 0x1e, 0xf0);

// ------------------------------------------------------------- Cores de Destaque
pub const ACCENT_BLUE: Color32 = Color32::from_rgb(0x31, 0x69, 0xe3);
pub const ACCENT_BLUE_HOVER: Color32 = Color32::from_rgb(0x47, 0x7c, 0xf5);
pub const ACCENT_BORDER: Color32 = Color32::from_rgb(0x5b, 0x8e, 0xff);
pub const ACCENT_GREEN: Color32 = Color32::from_rgb(0x2e, 0xcc, 0x71);
pub const ACCENT_AMBER: Color32 = Color32::from_rgb(0xf3, 0x9c, 0x12);

// ------------------------------------------------------------- Cores dos Modos
pub const MODE_OBJECT: Color32 = Color32::from_rgb(0x34, 0x98, 0xdb);
pub const MODE_EDIT: Color32 = Color32::from_rgb(0xe6, 0x7e, 0x22);
pub const MODE_PAINT: Color32 = Color32::from_rgb(0x2e, 0xcc, 0x71);

// -------------------------------------------------------------- Cores dos Eixos
pub const AXIS_X: Color32 = Color32::from_rgb(0xe0, 0x3c, 0x42);
pub const AXIS_Y: Color32 = Color32::from_rgb(0x62, 0xc9, 0x34);
pub const AXIS_Z: Color32 = Color32::from_rgb(0x31, 0x82, 0xf6);

// ---------------------------------------------------------------- Cores de Texto
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(0xde, 0xde, 0xde);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(0x9c, 0x9c, 0x9c);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(0x62, 0x62, 0x62);
pub const TEXT_ACTIVE: Color32 = Color32::from_rgb(0xff, 0xff, 0xff);

// ---------------------------------------------------------------- Cores de Borda
pub const BORDER_DARK: Color32 = Color32::from_rgb(0x14, 0x14, 0x14);
pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(0x2f, 0x2f, 0x2f);
pub const BORDER_LIGHT: Color32 = Color32::from_rgb(0x3e, 0x3e, 0x3e);

// ----------------------------------------------------------------- Dimensões (px)
pub const TOP_HEADER_HEIGHT: f32 = 28.0;
pub const TOP_HEADER_MAX_HEIGHT: f32 = 72.0;
pub const VIEWPORT_BAR_HEIGHT: f32 = 26.0;
pub const VIEWPORT_BAR_MAX_HEIGHT: f32 = 56.0;
pub const TOOLBAR_WIDTH: f32 = 40.0;
pub const TOOLBAR_MIN_WIDTH: f32 = 48.0;
pub const TOOLBAR_MAX_WIDTH: f32 = 240.0;
pub const STATUS_BAR_HEIGHT: f32 = 24.0;
pub const TIMELINE_HEIGHT: f32 = 56.0;
pub const PROPERTIES_DEFAULT_WIDTH: f32 = 290.0;
pub const OUTLINER_DEFAULT_HEIGHT: f32 = 230.0;

// -------------------------------------------------------------- Raios de Cantos
pub const RADIUS_PILL: CornerRadius = CornerRadius::same(12);
pub const RADIUS_CONTAINER: CornerRadius = CornerRadius::same(4);
pub const RADIUS_CONTROL: CornerRadius = CornerRadius::same(3);
pub const RADIUS_SMALL: CornerRadius = CornerRadius::same(2);

// --------------------------------------------------------------- Traços (Strokes)
pub fn stroke_subtle() -> Stroke {
    Stroke::new(1.0_f32, BORDER_SUBTLE)
}

pub fn stroke_border() -> Stroke {
    Stroke::new(1.0_f32, BORDER_DARK)
}

pub fn stroke_focus() -> Stroke {
    Stroke::new(1.5_f32, ACCENT_BORDER)
}

pub use petunia_config::ThemeToken;

/// Converte um ColorRgba do petunia_config para egui::Color32.
pub fn rgba_to_color32(c: petunia_config::ColorRgba) -> Color32 {
    let [r, g, b, a] = c.to_rgba_u8();
    Color32::from_rgba_unmultiplied(r, g, b, a)
}

/// Obtém dinamicamente a cor correspondente a um ThemeToken para o tema ativo no AppState.
pub fn color(state: &petunia_core::AppState, token: ThemeToken) -> Color32 {
    let registry = petunia_config::ThemeRegistry::global();
    if let Some(theme) = registry.get_theme(&state.active_theme_id) {
        rgba_to_color32(theme.colors.get_token_color(token))
    } else {
        match token {
            ThemeToken::BgCanvas => BG_APP,
            ThemeToken::BgHeader => BG_HEADER,
            ThemeToken::BgPanel => BG_PANEL,
            ThemeToken::BgPanelHeader => BG_PANEL_HEADER,
            ThemeToken::BgSurface => BG_SURFACE,
            ThemeToken::BgSurfaceHover => BG_SURFACE_HOVER,
            ThemeToken::BgSurfaceActive => BG_SURFACE_ACTIVE,
            ThemeToken::TextPrimary => TEXT_PRIMARY,
            ThemeToken::TextSecondary => TEXT_SECONDARY,
            ThemeToken::TextMuted => TEXT_MUTED,
            ThemeToken::TextActive => TEXT_ACTIVE,
            ThemeToken::AccentBlue => ACCENT_BLUE,
            ThemeToken::AccentOrange => MODE_EDIT,
            ThemeToken::AccentHover => ACCENT_BLUE_HOVER,
            ThemeToken::AccentBorder => ACCENT_BORDER,
            ThemeToken::BorderSubtle => BORDER_SUBTLE,
            ThemeToken::BorderStrong => BORDER_LIGHT,
            ThemeToken::BorderFocus => ACCENT_BLUE,
            ThemeToken::StatusInfo => ACCENT_BLUE,
            ThemeToken::StatusWarning => MODE_EDIT,
            ThemeToken::StatusError => AXIS_X,
            ThemeToken::StatusSuccess => MODE_PAINT,
        }
    }
}

/// Aplica as configurações do tema do Petunia3D ao contexto egui.
pub fn apply_theme_to_egui(theme: &petunia_config::Theme, ctx: &egui::Context) {
    let is_light = theme
        .manifest
        .as_ref()
        .map(|m| m.id.contains("light"))
        .unwrap_or(false);
    let mut v = if is_light {
        egui::Visuals::light()
    } else {
        egui::Visuals::dark()
    };

    let get_c = |token: ThemeToken| rgba_to_color32(theme.colors.get_token_color(token));

    let bg = get_c(ThemeToken::BgCanvas);
    let panel = get_c(ThemeToken::BgPanel);
    let text = get_c(ThemeToken::TextPrimary);
    let text_muted = get_c(ThemeToken::TextMuted);
    let accent = get_c(ThemeToken::AccentBlue);
    let border = get_c(ThemeToken::BorderSubtle);
    let control = get_c(ThemeToken::BgSurface);
    let hover = get_c(ThemeToken::BgSurfaceHover);
    let selection = get_c(ThemeToken::BgSurfaceActive);

    v.extreme_bg_color = bg;
    v.text_edit_bg_color = Some(control);
    v.panel_fill = panel;
    v.window_fill = panel;
    v.faint_bg_color = control;
    v.weak_text_color = Some(text_muted);
    v.selection.bg_fill = selection;
    v.selection.stroke = Stroke::new(1.0_f32, text);
    v.hyperlink_color = accent;
    v.window_stroke = Stroke::new(1.0_f32, border);

    for widget in [
        &mut v.widgets.noninteractive,
        &mut v.widgets.inactive,
        &mut v.widgets.hovered,
        &mut v.widgets.active,
        &mut v.widgets.open,
    ] {
        widget.fg_stroke = Stroke::new(1.5_f32, text);
        widget.bg_stroke = Stroke::new(1.0_f32, border);
        widget.corner_radius = egui::CornerRadius::same(5);
        widget.expansion = 0.0;
    }

    v.widgets.noninteractive.bg_fill = panel;
    v.widgets.noninteractive.weak_bg_fill = panel;
    v.widgets.inactive.bg_fill = control;
    v.widgets.inactive.weak_bg_fill = control;
    v.widgets.hovered.bg_fill = hover;
    v.widgets.hovered.weak_bg_fill = hover;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, accent);
    v.widgets.active.bg_fill = selection;
    v.widgets.active.weak_bg_fill = selection;
    v.widgets.active.bg_stroke = Stroke::new(1.5_f32, accent);
    v.widgets.open = v.widgets.active;

    ctx.set_visuals(v);

    let size = if theme.font.size.is_finite() {
        theme.font.size.clamp(12.0, 20.0)
    } else {
        14.0
    };
    let family = if theme.font.family == "monospace" {
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
