//! Papéis tipográficos.
//!
//! A escala reúne os tamanhos que o produto **já usa** (`10.0`, `10.5`, `11.0`,
//! `11.5`, `12.0`, `13.0`, `14.0`), transformando literais espalhados em papéis
//! nomeados. Não é uma fonte nova nem uma segunda escala: `ThemeFont.size`
//! (default `14.0`) continua sendo o tamanho base do tema.
//!
//! Motivação medida: 74 ocorrências de `.size(11.0)` e 35 de `.size(10.5)` em
//! `crates/ui/src` — o valor existe, só não tinha nome.

/// Tamanho base do texto do tema (espelha `ThemeFont::default().size`).
pub const BASE: f32 = 14.0;

/// Papel tipográfico. O valor é o tamanho em logical px.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextRole {
    /// Rótulos em pílulas e chips de workspace.
    Pill,
    /// Corpo padrão da UI: rótulos, itens de menu, dicas.
    Label,
    /// Texto de campos, valores numéricos e conteúdo de tabela.
    Field,
    /// Título de seção dentro de um painel.
    SectionTitle,
    /// Cabeçalho de painel / título de modal.
    Heading,
    /// Legenda secundária (metadados, contadores).
    Caption,
    /// Micro-texto (badges, telemetria da status bar).
    Micro,
}

impl TextRole {
    /// Tamanho em logical px.
    pub const fn size(self) -> f32 {
        match self {
            TextRole::Pill => 11.5,
            TextRole::Label => 11.0,
            TextRole::Field => 12.0,
            TextRole::SectionTitle => 11.0,
            TextRole::Heading => 13.0,
            TextRole::Caption => 10.5,
            TextRole::Micro => 10.0,
        }
    }

    /// Todos os papéis, do menor ao maior.
    pub const ALL: [TextRole; 7] = [
        TextRole::Micro,
        TextRole::Caption,
        TextRole::Label,
        TextRole::SectionTitle,
        TextRole::Pill,
        TextRole::Field,
        TextRole::Heading,
    ];
}

/// Fonte proporcional de um papel.
pub fn font(role: TextRole) -> egui::FontId {
    egui::FontId::proportional(role.size())
}

/// `RichText` de um papel, ainda sem cor (o chamador escolhe o token).
pub fn rich(text: impl Into<String>, role: TextRole) -> egui::RichText {
    egui::RichText::new(text).size(role.size())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_matches_theme_default() {
        assert_eq!(BASE, petunia_config::theme::ThemeFont::default().size);
    }

    #[test]
    fn sizes_match_values_already_used_by_the_product() {
        // Se um papel mudar de tamanho, o teste obriga a atualizar a evidência.
        assert_eq!(TextRole::Label.size(), 11.0);
        assert_eq!(TextRole::Pill.size(), 11.5);
        assert_eq!(TextRole::Field.size(), 12.0);
        assert_eq!(TextRole::Heading.size(), 13.0);
        assert_eq!(TextRole::Caption.size(), 10.5);
    }

    #[test]
    fn smallest_role_is_micro_and_all_fit_under_base() {
        assert_eq!(TextRole::ALL[0], TextRole::Micro);
        for role in TextRole::ALL {
            assert!(
                role.size() <= BASE,
                "{role:?} maior que o texto base do tema"
            );
        }
    }

    #[test]
    fn font_helper_matches_role_size() {
        assert_eq!(font(TextRole::Field).size, TextRole::Field.size());
    }
}
