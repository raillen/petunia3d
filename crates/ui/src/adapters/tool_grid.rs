//! # `PetuniaToolGrid` — grade de ferramentas da paleta lateral (§48)
//!
//! Wave 6: a paleta vertical de ferramentas deixa de decidir o próprio arranjo
//! com dois breakpoints escritos à mão
//! (`available_width() < 90.0` para o modo ícone-only e
//! `available_width() >= 100.0` para o par de colunas). O produto declara duas
//! larguras mínimas — a célula só com ícone e a célula que já comporta o
//! rótulo — e o adapter mede, decide as colunas e entrega a largura definitiva
//! a cada botão.
//!
//! ```text
//! toolbar.rs  →  TOOL_GRID.show(ui, id, entradas, |i, cell, ui| ..)
//! ```
//!
//! ## Por que a célula carrega `labeled`
//!
//! "Cabe o rótulo?" é a **mesma** pergunta que o breakpoint respondia, mas
//! agora é respondida pela largura que a célula realmente recebeu — não por um
//! número escolhido a dedo sobre o container. Numa paleta estreita a célula
//! fica abaixo de [`PetuniaToolGridSpec::label_min_cell`] e o botão vira
//! ícone-only; numa paleta larga ou em uma coluna só, o rótulo aparece. O mesmo
//! código cobre escala de UI maior, idioma com rótulo longo e dois painéis lado
//! a lado.

use egui::Ui;

use crate::adapters::taffy_layout::{self, PetuniaColumnSpec};

/// Especificação da grade de ferramentas.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PetuniaToolGridSpec {
    /// Menor largura em que a célula ainda é utilizável (modo só ícone).
    pub min_cell: f32,
    /// Largura a partir da qual o botão consegue mostrar o rótulo.
    pub label_min_cell: f32,
    /// Vão horizontal e vertical entre células.
    pub gap: f32,
    /// Teto de colunas por linha (1 = lista simples).
    pub max_columns: u16,
}

impl PetuniaToolGridSpec {
    /// Grade de uma coluna (lista), que é o arranjo padrão da paleta.
    pub const fn new(min_cell: f32, label_min_cell: f32, gap: f32) -> Self {
        Self {
            min_cell,
            label_min_cell,
            gap,
            max_columns: 1,
        }
    }

    /// Define o teto de colunas por linha.
    pub const fn with_max_columns(mut self, max_columns: u16) -> Self {
        self.max_columns = max_columns;
        self
    }

    /// Quantas colunas cabem e a largura de cada célula.
    ///
    /// Puro: delega a [`taffy_layout::fit_columns`], o único lugar da fórmula de
    /// divisão — a contagem sai do mínimo real da célula, não de um breakpoint.
    pub fn plan(&self, available: f32) -> (u16, f32) {
        taffy_layout::fit_columns(available, self.max_columns.max(1), self.min_cell, self.gap)
    }

    /// O botão desta largura consegue mostrar o rótulo?
    pub fn labels_fit(&self, width: f32) -> bool {
        width >= self.label_min_cell
    }

    /// Célula resolvida para um item.
    pub fn cell(&self, width: f32, columns: u16) -> PetuniaToolCell {
        PetuniaToolCell {
            width,
            columns,
            labeled: self.labels_fit(width),
        }
    }

    /// Desenha `count` itens na grade, entregando a célula resolvida.
    ///
    /// `item` recebe o índice, a célula (largura definitiva, colunas e se cabe
    /// rótulo) e o `Ui` do item; `after` roda depois da grade e devolve o valor
    /// final — mesmo contrato de [`taffy_layout::columns`].
    ///
    /// O callback roda **mais de uma vez por item** (a passagem de medida do
    /// taffy desenha o item para conhecê-lo, e a passagem de posicionamento
    /// desenha de novo). Ele é um desenho, não um acumulador: nada de `push` em
    /// vetor nem de contador de frame lá dentro. Mutações de produto são
    /// seguras porque reagem a `clicked()`, que só é verdadeiro na passagem
    /// real de interação.
    pub fn show<R>(
        &self,
        ui: &mut Ui,
        id: impl Into<egui::Id>,
        count: usize,
        mut item: impl FnMut(usize, PetuniaToolCell, &mut Ui),
        after: impl FnMut(&mut Ui) -> R,
    ) -> R {
        let (columns, _) = self.plan(ui.available_width());
        let spec = PetuniaColumnSpec::responsive(count, self.min_cell, self.gap)
            .with_max_columns(self.max_columns.max(1));
        taffy_layout::columns(
            ui,
            id,
            spec,
            move |index, width, ui| item(index, self.cell(width, columns), ui),
            after,
        )
    }
}

/// Célula resolvida de um item da grade.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PetuniaToolCell {
    /// Largura definitiva da célula (nunca zero em painel utilizável).
    pub width: f32,
    /// Quantas colunas a grade está usando neste arranjo.
    pub columns: u16,
    /// `true` quando a largura comporta ícone + rótulo.
    pub labeled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Paleta padrão: célula de 40px, rótulo a partir de 120px.
    const GRID: PetuniaToolGridSpec = PetuniaToolGridSpec::new(40.0, 120.0, 3.0);

    #[test]
    fn a_single_column_takes_the_whole_palette() {
        let (columns, width) = GRID.plan(172.0);
        assert_eq!(columns, 1);
        assert_eq!(width, 172.0);
        assert!(GRID.labels_fit(width), "paleta de 172px comporta o rótulo");
    }

    #[test]
    fn two_columns_split_the_palette_and_drop_the_label() {
        let grid = GRID.with_max_columns(2);
        let (columns, width) = grid.plan(172.0);
        assert_eq!(columns, 2);
        assert!(width < 100.0, "célula de duas colunas: {width}");
        assert!(
            !grid.labels_fit(width),
            "duas colunas numa paleta de 172px não comportam o rótulo"
        );
    }

    #[test]
    fn a_wide_palette_keeps_two_columns_with_labels() {
        let grid = GRID.with_max_columns(2);
        let (columns, width) = grid.plan(560.0);
        assert_eq!(columns, 2);
        assert!(grid.labels_fit(width), "célula larga: {width}");
    }

    #[test]
    fn columns_never_overflow_the_project_or_go_negative() {
        let grid = GRID.with_max_columns(2);
        for available in [0.0, 24.0, 80.0, 300.0, 4000.0] {
            let (columns, width) = grid.plan(available);
            assert!(columns >= 1, "largura {available}");
            assert!(width >= 0.0, "largura {available}");
            let used = width * f32::from(columns) + GRID.gap * f32::from(columns - 1);
            assert!(
                used <= available.max(0.0) + 1.0,
                "largura {available}: grade usa {used}"
            );
        }
    }

    #[test]
    fn every_item_receives_the_same_resolved_cell() {
        let ctx = egui::Context::default();
        for max_columns in [1u16, 2] {
            let grid = GRID.with_max_columns(max_columns);
            let (expected_columns, expected_width) = grid.plan(180.0);
            let mut cells = Vec::new();
            let mut out = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::pos2(0.0, 0.0),
                        egui::vec2(180.0, 600.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    grid.show(
                        ui,
                        "tool-grid-test",
                        6,
                        |index, cell, ui| {
                            cells.push((index, cell));
                            ui.label(format!("{index}"));
                        },
                        |_ui| (),
                    );
                },
            );
            out.textures_delta.clear();

            // O callback roda mais de uma vez por item (medida + desenho), então
            // o que se verifica é o conjunto de índices e a célula de cada um.
            assert!(
                cells.len() >= 6 && cells.len() % 6 == 0,
                "colunas {max_columns}: {} chamadas para 6 itens",
                cells.len()
            );
            let mut indices: Vec<usize> = cells.iter().map(|(index, _)| *index).collect();
            indices.sort_unstable();
            indices.dedup();
            assert_eq!(
                indices,
                (0..6).collect::<Vec<_>>(),
                "a ordem visual é a ordem da declaração"
            );
            for (index, cell) in &cells {
                assert_eq!(cell.columns, expected_columns, "item {index}");
                assert!(
                    (cell.width - expected_width).abs() <= 1.0,
                    "item {index}: célula {:.1}, esperada {expected_width:.1}",
                    cell.width
                );
            }
        }
    }
}
