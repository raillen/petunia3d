//! Leitura de tokens de tema a partir do estado ativo.
//!
//! Fachada fina sobre [`crate::tokens`]: `tokens` continua sendo a definição
//! física das cores e o ponto onde o tema do usuário é resolvido; este módulo
//! expõe a leitura por papel semântico, que é o que componentes e painéis podem
//! citar.
//!
//! Contrato: **zero cor literal** em UI pública. Uma cor só entra por token.

pub use crate::tokens::{
    bg_canvas, bg_header, bg_panel, bg_panel_header, bg_surface, bg_surface_active,
    bg_surface_hover, border_strong, border_subtle, color, stroke_border, stroke_border_dyn,
    stroke_focus, stroke_subtle, text_active, text_muted, text_primary, text_secondary,
};

/// Tokens semânticos disponíveis, na ordem em que o Design System os documenta.
pub use petunia_config::theme::ThemeToken;

/// Resolve um token para a cor do tema ativo.
pub fn token(state: &petunia_core::AppState, token: ThemeToken) -> egui::Color32 {
    color(state, token)
}

/// Cor de estado informativo (telemetria, dicas).
pub fn status_info(state: &petunia_core::AppState) -> egui::Color32 {
    color(state, ThemeToken::StatusInfo)
}

/// Cor de alerta (geometria não conforme, limites).
pub fn status_warning(state: &petunia_core::AppState) -> egui::Color32 {
    color(state, ThemeToken::StatusWarning)
}

/// Cor de erro (I/O, formato corrompido, colisão).
pub fn status_error(state: &petunia_core::AppState) -> egui::Color32 {
    color(state, ThemeToken::StatusError)
}

/// Cor de sucesso (salvo, exportado, snapshot).
pub fn status_success(state: &petunia_core::AppState) -> egui::Color32 {
    color(state, ThemeToken::StatusSuccess)
}

#[cfg(test)]
mod tests {
    use super::*;
    use petunia_core::AppState;

    #[test]
    fn every_semantic_token_resolves_to_an_opaque_color() {
        let state = AppState::new("en");
        for t in [
            ThemeToken::BgCanvas,
            ThemeToken::BgPanel,
            ThemeToken::TextPrimary,
            ThemeToken::AccentBlue,
            ThemeToken::BorderSubtle,
        ] {
            let c = token(&state, t);
            assert_eq!(c.a(), 255, "{t:?} precisa ser opaco no tema ativo");
        }
    }

    #[test]
    fn background_helpers_read_the_active_theme() {
        let state = AppState::new("en");
        assert_eq!(bg_canvas(&state), token(&state, ThemeToken::BgCanvas));
        assert_eq!(bg_panel(&state), token(&state, ThemeToken::BgPanel));
    }
}
