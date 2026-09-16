//! Métricas por preset de densidade.
//!
//! Esta é a **casa canônica** das métricas de densidade (`Compact`,
//! `Comfortable`, `Spacious`). Elas viviam dentro de `inspector_widgets.rs`, o
//! que fazia do inspector o dono de um contrato usado pelo shell inteiro.
//!
//! O preset em si continua sendo estado de sessão do produto (`UiDensity`, no
//! core); aqui mora só a tradução preset → número, em logical px.

use petunia_core::UiDensity;

/// Fachada tipada das métricas de densidade.
///
/// Existe para que componentes leiam `PetuniaDensity::row_h(d)` em vez de
/// espalhar `match density { ... }` por cada painel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PetuniaDensity;

impl PetuniaDensity {
    /// Altura de linha (linhas da Scene, cabeçalhos de seção, abas).
    pub fn row_h(density: UiDensity) -> f32 {
        match density {
            UiDensity::Compact => 28.0,
            UiDensity::Comfortable => 32.0,
            UiDensity::Spacious => 36.0,
        }
    }

    /// Hitbox mínima de botões-ícone (o glifo pode ser 16–20px, o alvo não).
    pub fn hit_size(density: UiDensity) -> f32 {
        Self::row_h(density)
    }

    /// Espaçamento vertical entre propriedades.
    pub fn spacing_y(density: UiDensity) -> f32 {
        match density {
            UiDensity::Compact => 2.0,
            UiDensity::Comfortable => 4.0,
            UiDensity::Spacious => 6.0,
        }
    }

    /// Altura de campos de entrada.
    pub fn input_h(density: UiDensity) -> f32 {
        match density {
            UiDensity::Compact => 22.0,
            UiDensity::Comfortable => 26.0,
            UiDensity::Spacious => 30.0,
        }
    }

    /// Folga entre seções.
    pub fn section_gap(density: UiDensity) -> f32 {
        match density {
            UiDensity::Compact => 8.0,
            UiDensity::Comfortable => 10.0,
            UiDensity::Spacious => 14.0,
        }
    }
}

// Aliases livres — mantêm o código existente legível sem forçar `PetuniaDensity::`.
pub use PetuniaDensity as Density;

/// Altura de linha. Ver [`PetuniaDensity::row_h`].
pub fn row_h(density: UiDensity) -> f32 {
    PetuniaDensity::row_h(density)
}

/// Hitbox mínima de botão-ícone. Ver [`PetuniaDensity::hit_size`].
pub fn hit_size(density: UiDensity) -> f32 {
    PetuniaDensity::hit_size(density)
}

/// Espaçamento vertical entre propriedades. Ver [`PetuniaDensity::spacing_y`].
pub fn spacing_y(density: UiDensity) -> f32 {
    PetuniaDensity::spacing_y(density)
}

/// Altura de campo de entrada. Ver [`PetuniaDensity::input_h`].
pub fn input_h(density: UiDensity) -> f32 {
    PetuniaDensity::input_h(density)
}

/// Folga entre seções. Ver [`PetuniaDensity::section_gap`].
pub fn section_gap(density: UiDensity) -> f32 {
    PetuniaDensity::section_gap(density)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_metric_grows_with_density() {
        for metric in [
            PetuniaDensity::row_h,
            PetuniaDensity::hit_size,
            PetuniaDensity::spacing_y,
            PetuniaDensity::input_h,
            PetuniaDensity::section_gap,
        ] {
            let c = metric(UiDensity::Compact);
            let cm = metric(UiDensity::Comfortable);
            let s = metric(UiDensity::Spacious);
            assert!(c < cm && cm < s, "métrica precisa crescer: {c} {cm} {s}");
        }
    }

    #[test]
    fn row_height_respects_control_baseline() {
        // Cap. 36: control baseline de 28px; a densidade Compact não pode ir abaixo.
        assert!(PetuniaDensity::row_h(UiDensity::Compact) >= 28.0);
        assert!(PetuniaDensity::input_h(UiDensity::Comfortable) <= 28.0);
    }

    #[test]
    fn hit_target_is_never_smaller_than_the_row() {
        for density in UiDensity::all() {
            assert_eq!(
                PetuniaDensity::hit_size(density),
                PetuniaDensity::row_h(density)
            );
        }
    }

    #[test]
    fn free_functions_forward_to_the_facade() {
        assert_eq!(
            row_h(UiDensity::Compact),
            PetuniaDensity::row_h(UiDensity::Compact)
        );
        assert_eq!(
            section_gap(UiDensity::Spacious),
            PetuniaDensity::section_gap(UiDensity::Spacious)
        );
    }
}
