//! Type-safe style foundation bridge (`twill`, P1-20, BASELINE CANDIDATE).
//!
//! Petunia stays the semantic owner of its design system: `twill` (without
//! its `egui` backend, which pins egui 0.33) provides type-safe
//! token/style/variant construction, and this adapter maps the result onto
//! Petunia tokens and egui visuals:
//!
//! ```text
//! twill tokens (type-safe, invalid states unrepresentable)
//!         ↓
//! Petunia mapping (this module, unit-tested against tokens.rs)
//!         ↓
//! egui visuals + Petunia components
//! ```
//!
//! Pilot scope: toolbar button corner radius. Broader adoption (theme
//! completeness validator, more components) is future work, not a visual
//! takeover: every mapping is pinned to the current Petunia value by test.

use twill::{BorderRadius, Spacing};

/// Corner radius in px for toolbar buttons, from the type-safe token.
/// Must equal [`crate::tokens::RADIUS_CONTAINER`]; the test pins it.
pub fn toolbar_corner_radius_px() -> f32 {
    border_radius_px(BorderRadius::Sm)
}

/// Toolbar button radius as an egui [`egui::CornerRadius`], wired into
/// `PetuniaToolbarButton` (real product path, zero visual delta by test).
pub fn toolbar_button_radius() -> egui::CornerRadius {
    egui::CornerRadius::same(toolbar_corner_radius_px() as u8)
}

/// Maps a twill [`BorderRadius`] token to logical px (16px base).
/// Values follow the documented rem scale; `Full` (pill) saturates to 9999.
pub fn border_radius_px(radius: BorderRadius) -> f32 {
    match radius {
        BorderRadius::None => 0.0,
        BorderRadius::Xs => 2.0,
        BorderRadius::Sm => 4.0,
        BorderRadius::Md => 6.0,
        BorderRadius::Lg => 8.0,
        BorderRadius::Xl => 12.0,
        BorderRadius::S2xl => 16.0,
        BorderRadius::S3xl => 24.0,
        BorderRadius::S4xl => 32.0,
        BorderRadius::Full => 9999.0,
    }
}

/// Maps a twill [`Spacing`] token to logical px.
pub fn spacing_px(spacing: Spacing) -> f32 {
    spacing.to_px().unwrap_or(0) as f32
}

/// Renders the token proof as CSS (twill `ToCss`), used by the theme
/// reference tooling (P1-21) to diff Petunia values against the scale.
pub fn button_css_proof() -> String {
    use twill::ToCss;
    let style = twill::Style::new()
        .padding(twill::Padding::symmetric(Spacing::S1, Spacing::S2))
        .rounded(BorderRadius::Sm);
    style.to_css()
}

#[cfg(test)]
mod tests {
    use super::*;
    use twill::ComputeValue;

    #[test]
    fn toolbar_radius_matches_petunia_token() {
        assert_eq!(toolbar_corner_radius_px(), 4.0);
        assert_eq!(crate::tokens::RADIUS_CONTAINER, egui::CornerRadius::same(4));
    }

    #[test]
    fn spacing_scale_matches_documented_px() {
        assert_eq!(spacing_px(Spacing::S0), 0.0);
        assert_eq!(spacing_px(Spacing::S1), 4.0);
        assert_eq!(spacing_px(Spacing::S2), 8.0);
        assert_eq!(spacing_px(Spacing::S4), 16.0);
    }

    #[test]
    fn css_proof_is_deterministic() {
        let first = button_css_proof();
        assert_eq!(first, button_css_proof());
        assert!(first.contains("0.25rem") || first.contains("4px") || first.contains("padding"));
    }

    #[test]
    fn twill_color_computes() {
        let hex = twill::Color::blue(twill::Scale::S500).compute().to_hex();
        assert_eq!(hex.len(), 7);
        assert!(hex.starts_with('#'));
    }
}
