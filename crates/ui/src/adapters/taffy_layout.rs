//! Adapter de layout responsivo (`egui_taffy`).
//!
//! Este é o **único** arquivo autorizado a mencionar `egui_taffy` ou `taffy::`
//! (verificado por `cargo xtask ui-guard --strict`). Product code descreve o
//! arranjo com [`PetuniaResponsiveLayout`] e nunca monta `taffy::Style`.
//!
//! ```text
//! PetuniaResponsiveLayout   (contrato Petunia, §23 da diretiva)
//!          ↓
//! taffy::Style              (detalhe de crate)
//!          ↓
//! egui_taffy::tui(...)
//! ```
//!
//! ## Quando usar
//!
//! | Situação | Solução |
//! | --- | --- |
//! | uma linha ou coluna simples de 2–4 itens | `ui.horizontal` / `ui.vertical` |
//! | arranjo que precisa responder à largura disponível | [`responsive`] |
//! | grade com número variável de colunas | [`responsive`] com [`PetuniaLayoutMode::Grid`] |
//!
//! O erro que este adapter existe para impedir é medir o container à mão:
//! `if ui.available_width() < 560.0` ou dividir largura por um contador.

use egui_taffy::taffy;

/// Direção principal do arranjo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PetuniaLayoutMode {
    /// Itens em linha; com `wrap` viram múltiplas linhas.
    #[default]
    Row,
    /// Itens empilhados.
    Column,
    /// Grade com número fixo de colunas.
    Grid {
        /// Quantidade de colunas.
        columns: u16,
    },
}

/// Espaço entre itens.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PetuniaGap {
    /// Gap horizontal em logical px.
    pub horizontal: f32,
    /// Gap vertical em logical px.
    pub vertical: f32,
}

impl PetuniaGap {
    /// Mesmo gap nas duas direções.
    pub const fn uniform(gap: f32) -> Self {
        Self {
            horizontal: gap,
            vertical: gap,
        }
    }

    /// Gap nulo.
    pub const NONE: Self = Self::uniform(0.0);

    /// Gap entre controles relacionados ([`crate::foundation::spacing::RELATED`]).
    pub const fn related() -> Self {
        Self::uniform(crate::foundation::spacing::RELATED)
    }

    /// Gutter estrutural ([`crate::foundation::spacing::GUTTER`]).
    pub const fn gutter() -> Self {
        Self::uniform(crate::foundation::spacing::GUTTER)
    }
}

/// Alinhamento no eixo cruzado.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PetuniaAlign {
    /// Encosta no início do eixo cruzado.
    Start,
    /// Centraliza no eixo cruzado.
    #[default]
    Center,
    /// Encosta no fim do eixo cruzado (alinhado à direita em `Row`).
    End,
    /// Distribui ocupando todo o eixo cruzado.
    Stretch,
}

/// Distribuição no eixo principal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PetuniaJustify {
    /// Itens juntos no início.
    #[default]
    Start,
    /// Itens centralizados.
    Center,
    /// Espaço igual entre os itens.
    SpaceBetween,
    /// Itens encostados no fim do eixo principal (alinhamento à direita em `Row`).
    End,
    /// Extremos nas bordas.
    SpaceAround,
}

/// Layout do `Ui` entregue a cada item do sublayout.
///
/// O `egui_taffy` entrega a cada item um `Ui` que **herda o layout do pai**.
/// Isso é correto para um item que se comporta como célula única (o caso de
/// um campo numérico), mas errado para um item que é um **grupo horizontal**
/// desenhado direto no `Ui` recebido: dentro de um pai vertical, os filhos do
/// item empilham e a linha cresce em altura.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PetuniaItemLayout {
    /// Herda o layout do `Ui` pai (comportamento padrão do `egui_taffy`).
    #[default]
    Inherit,
    /// Cada item é uma linha: seus filhos diretos ficam lado a lado.
    Row,
}

/// Contrato Petunia para um sublayout responsivo.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PetuniaResponsiveLayout {
    /// Direção principal (e colunas, no caso de grade).
    pub mode: PetuniaLayoutMode,
    /// Espaço entre itens.
    pub gap: PetuniaGap,
    /// Alinhamento no eixo cruzado.
    pub align: PetuniaAlign,
    /// Distribuição no eixo principal.
    pub justify: PetuniaJustify,
    /// Permite quebra de linha em `Row`.
    pub wrap: bool,
    /// Layout do `Ui` de cada item.
    pub item_layout: PetuniaItemLayout,
}

impl Default for PetuniaResponsiveLayout {
    fn default() -> Self {
        Self::row()
    }
}

impl PetuniaResponsiveLayout {
    /// Linha sem quebra, itens centralizados.
    pub const fn row() -> Self {
        Self {
            mode: PetuniaLayoutMode::Row,
            gap: PetuniaGap::related(),
            align: PetuniaAlign::Center,
            justify: PetuniaJustify::Start,
            wrap: false,
            item_layout: PetuniaItemLayout::Inherit,
        }
    }

    /// Coluna (usada quando a largura disponível não permite linha).
    pub const fn column() -> Self {
        Self {
            mode: PetuniaLayoutMode::Column,
            gap: PetuniaGap::related(),
            align: PetuniaAlign::Start,
            justify: PetuniaJustify::Start,
            wrap: false,
            item_layout: PetuniaItemLayout::Inherit,
        }
    }

    /// Linha que quebra em várias linhas — substitui o breakpoint manual.
    pub const fn wrap_row() -> Self {
        Self {
            wrap: true,
            ..Self::row()
        }
    }

    /// Grade de `columns` colunas.
    pub const fn grid(columns: u16) -> Self {
        Self {
            mode: PetuniaLayoutMode::Grid { columns },
            gap: PetuniaGap::gutter(),
            align: PetuniaAlign::Stretch,
            justify: PetuniaJustify::Start,
            wrap: true,
            item_layout: PetuniaItemLayout::Inherit,
        }
    }

    /// Define o gap.
    pub const fn with_gap(mut self, gap: PetuniaGap) -> Self {
        self.gap = gap;
        self
    }

    /// Define o layout de cada item (ver [`PetuniaItemLayout`]).
    pub const fn with_item_layout(mut self, item_layout: PetuniaItemLayout) -> Self {
        self.item_layout = item_layout;
        self
    }

    /// Define o alinhamento no eixo cruzado.
    pub const fn with_align(mut self, align: PetuniaAlign) -> Self {
        self.align = align;
        self
    }

    /// Define a distribuição no eixo principal.
    pub const fn with_justify(mut self, justify: PetuniaJustify) -> Self {
        self.justify = justify;
        self
    }

    /// Número de colunas efetivo (1 quando o modo não é grade).
    pub const fn columns(&self) -> u16 {
        match self.mode {
            PetuniaLayoutMode::Grid { columns } => columns,
            _ => 1,
        }
    }
}

// ------------------------------------------------------------------ runtime

fn direction(mode: PetuniaLayoutMode) -> taffy::FlexDirection {
    match mode {
        PetuniaLayoutMode::Column => taffy::FlexDirection::Column,
        // Uma grade é um flex row com wrap: o `columns` decide o `flex_basis`.
        PetuniaLayoutMode::Row | PetuniaLayoutMode::Grid { .. } => taffy::FlexDirection::Row,
    }
}

fn style(layout: PetuniaResponsiveLayout) -> taffy::Style {
    taffy::Style {
        flex_direction: direction(layout.mode),
        justify_content: Some(match layout.justify {
            PetuniaJustify::Start => taffy::JustifyContent::Start,
            PetuniaJustify::Center => taffy::JustifyContent::Center,
            PetuniaJustify::SpaceBetween => taffy::JustifyContent::SpaceBetween,
            PetuniaJustify::SpaceAround => taffy::JustifyContent::SpaceAround,
            PetuniaJustify::End => taffy::JustifyContent::FlexEnd,
        }),
        align_items: Some(match layout.align {
            PetuniaAlign::Start => taffy::AlignItems::FlexStart,
            PetuniaAlign::Center => taffy::AlignItems::Center,
            PetuniaAlign::End => taffy::AlignItems::FlexEnd,
            PetuniaAlign::Stretch => taffy::AlignItems::Stretch,
        }),
        flex_wrap: if layout.wrap {
            taffy::FlexWrap::Wrap
        } else {
            taffy::FlexWrap::NoWrap
        },
        gap: taffy::Size {
            width: taffy::LengthPercentage::length(layout.gap.horizontal),
            height: taffy::LengthPercentage::length(layout.gap.vertical),
        },
        ..Default::default()
    }
}

/// Renderiza um sublayout responsivo preenchendo a largura disponível.
///
/// `items` recebe um `TuiBuilderLogic` por item; a ordem é a ordem visual.
///
/// Em [`PetuniaLayoutMode::Grid`] delega para [`columns`], a única
/// implementação que resolve **largura definitiva** de célula.
pub fn responsive<R>(
    ui: &mut egui::Ui,
    id: impl Into<egui::Id>,
    layout: PetuniaResponsiveLayout,
    item_count: usize,
    mut items: impl FnMut(usize, &mut egui::Ui),
    mut after: impl FnMut(&mut egui::Ui) -> R,
) -> R {
    use egui_taffy::TuiBuilderLogic;
    if matches!(layout.mode, PetuniaLayoutMode::Grid { .. }) {
        return columns(
            ui,
            id,
            PetuniaColumnSpec::fixed(item_count, layout.gap.horizontal)
                .with_max_columns(layout.columns()),
            move |index, _width, ui| items(index, ui),
            after,
        );
    }
    let st = style(layout);
    // Sem layout explícito o item herda o `Ui` do chamador; `Row` dá a cada item
    // a linha em que seus filhos diretos ficam lado a lado.
    let item_egui_layout = match layout.item_layout {
        PetuniaItemLayout::Inherit => None,
        PetuniaItemLayout::Row => Some(egui::Layout::left_to_right(egui::Align::Center)),
    };
    let mut out = None;
    egui_taffy::tui(ui, id)
        .reserve_available_width()
        .style(st)
        .show(|tui| {
            for index in 0..item_count {
                match item_egui_layout {
                    Some(item_layout) => {
                        tui.egui_layout(item_layout).ui(|ui| items(index, ui));
                    }
                    None => {
                        tui.ui(|ui| items(index, ui));
                    }
                }
            }
            out = Some(tui.ui(|ui| after(ui)));
        });
    out.expect("sublayout taffy sempre produz a resposta final")
}

// --------------------------------------------------------------- três zonas

/// Como o centro de uma barra de três zonas se posiciona (§25).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PetuniaCentering {
    /// Centro **geométrico**: as zonas laterais recebem caixas de largura igual
    /// (base zero + `flex_grow` igual), então o centro cai exatamente no meio da
    /// barra independentemente do conteúdo de cada lado.
    ///
    /// Exige que o conteúdo das laterais caiba na caixa; o adapter escolhe este
    /// modo por medição, não por palpite.
    #[default]
    Centered,
    /// Laterais empacotadas nas bordas (`space-between`) com o centro onde couber.
    ///
    /// É o fallback quando uma lateral não cabe na metade da barra: nada é
    /// cortado nem empurrado para fora, mas o centro deixa de ser geométrico.
    Drifting,
}

/// Zona de uma barra de três colunas (ver [`three_zone`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetuniaZone {
    /// Zona inicial, alinhada à esquerda.
    Left,
    /// Zona central.
    Center,
    /// Zona final, alinhada à direita.
    Right,
}

/// Barra de três zonas (esquerda · centro · direita) sem `add_space` falso.
///
/// Substitui o hack de centralização `ui.columns(3, …)` + `add_space` repetido:
/// o centro é resolvido por **distribuição**, não por espaçadores somados à mão.
/// Com [`PetuniaCentering::Centered`] o centro é geométrico por construção.
///
/// Cada zona recebe um `Ui` próprio com layout de linha. Um **único** callback
/// atende as três zonas — é o que permite desenhar as zonas com o mesmo closure
/// de produto sem emprestar o mesmo estado três vezes.
///
/// O alinhamento dentro da caixa é do conteúdo: a direita recebe a caixa mas
/// quem a preenche usa [`PetuniaJustify::End`] via [`responsive`] (a direção do
/// `Ui` não decide a ordem dos itens do taffy).
pub fn three_zone<R>(
    ui: &mut egui::Ui,
    id: impl Into<egui::Id>,
    gap: f32,
    centering: PetuniaCentering,
    mut zone: impl FnMut(PetuniaZone, &mut egui::Ui),
    mut after: impl FnMut(&mut egui::Ui) -> R,
) -> R {
    use egui_taffy::{TuiBuilderLogic, taffy};
    let gap = gap.max(0.0);
    let st = taffy::Style {
        flex_direction: taffy::FlexDirection::Row,
        justify_content: Some(match centering {
            PetuniaCentering::Centered => taffy::JustifyContent::Start,
            PetuniaCentering::Drifting => taffy::JustifyContent::SpaceBetween,
        }),
        align_items: Some(taffy::AlignItems::Center),
        gap: taffy::Size {
            width: taffy::LengthPercentage::length(gap),
            height: taffy::LengthPercentage::length(0.0),
        },
        ..Default::default()
    };
    // Laterais flexíveis e iguais: base zero + o mesmo `flex_grow` em ambas. Com
    // o centro em base automática, o que sobra é dividido igualmente e o centro
    // fica no meio exato da barra.
    let side = match centering {
        PetuniaCentering::Centered => taffy::Style {
            flex_basis: taffy::Dimension::length(0.0),
            flex_grow: 1.0,
            flex_shrink: 0.0,
            ..Default::default()
        },
        PetuniaCentering::Drifting => taffy::Style {
            flex_basis: taffy::Dimension::auto(),
            flex_grow: 0.0,
            flex_shrink: 0.0,
            ..Default::default()
        },
    };
    let middle = taffy::Style {
        flex_basis: taffy::Dimension::auto(),
        flex_grow: 0.0,
        flex_shrink: 0.0,
        ..Default::default()
    };
    let mut out = None;
    egui_taffy::tui(ui, id)
        .reserve_available_width()
        .style(st)
        .show(|tui| {
            tui.style(side.clone())
                .egui_layout(egui::Layout::left_to_right(egui::Align::Center))
                .ui(|ui| zone(PetuniaZone::Left, ui));
            tui.style(middle)
                .egui_layout(egui::Layout::left_to_right(egui::Align::Center))
                .ui(|ui| zone(PetuniaZone::Center, ui));
            tui.style(side)
                .egui_layout(egui::Layout::left_to_right(egui::Align::Center))
                .ui(|ui| zone(PetuniaZone::Right, ui));
            out = Some(tui.ui(|ui| after(ui)));
        });
    out.expect("barra de três zonas sempre produz a resposta final")
}

/// Largura da caixa lateral de [`three_zone`] quando o centro é geométrico.
///
/// É a metade do que sobra depois do centro e dos dois vãos. O adapter usa isto
/// para decidir entre [`PetuniaCentering::Centered`] e
/// [`PetuniaCentering::Drifting`] — e o produto nunca precisa saber da fórmula.
pub fn centered_side_width(available: f32, center_width: f32, gap: f32) -> f32 {
    ((available.max(0.0) - center_width.max(0.0) - 2.0 * gap.max(0.0)) * 0.5).max(0.0)
}

// ----------------------------------------------------------------- colunas

/// Largura de **uma** célula quando `available` é dividido em `columns` colunas
/// iguais separadas por `gap`.
///
/// É a única fórmula de divisão do produto, e vive aqui — não no componente —
/// porque quem produz layout é o adapter (§37). O resultado é truncado para
/// baixo: `columns * largura + (columns - 1) * gap` nunca ultrapassa
/// `available`. Sem o truncamento o egui arredonda a alocação para cima e o
/// painel inteiro se alarga ~1px por frame.
pub fn column_width(available: f32, columns: u16, gap: f32) -> f32 {
    if columns == 0 {
        return 0.0;
    }
    let available = available.max(0.0);
    if columns == 1 {
        return available.floor();
    }
    let gap = gap.max(0.0);
    ((available - gap * (columns - 1) as f32) / columns as f32)
        .max(0.0)
        .floor()
}

/// Quantas colunas de largura igual cabem, e a largura de cada uma.
///
/// Substituto normativo dos breakpoints mágicos (`if avail >= 300.0 { 3 } else
/// if avail >= 190.0 { 1 }`): o número de colunas é **derivado** do mínimo real
/// do item, não de um número escolhido a dedo. O resultado fica sempre entre `1`
/// e `max_columns`, então nunca há largura negativa nem coluna invisível.
///
/// `min_item == 0.0` significa "cabe sempre" — use em faixas que não podem
/// quebrar (abas, por exemplo), onde só a divisão igual interessa.
pub fn fit_columns(available: f32, max_columns: u16, min_item: f32, gap: f32) -> (u16, f32) {
    if max_columns == 0 {
        return (0, 0.0);
    }
    let available = available.max(0.0);
    let gap = gap.max(0.0);
    let step = (min_item.max(0.0) + gap).max(f32::MIN_POSITIVE);
    // `available + gap` conta o gap que a última célula não usa.
    let fits = (((available + gap) / step).floor() as i64).clamp(1, max_columns as i64) as u16;
    (fits, column_width(available, fits, gap))
}

/// Largura de um controle que quer `desired`, mas nunca mais do que existe.
///
/// Substitui `ui.available_width().clamp(a, b)` em paineis (§45, Wave 3): o
/// painel não mede o container, ele pergunta ao adapter qual largura usar. O
/// `min` é o piso em que o controle ainda é utilizável.
pub fn clamped_width(ui: &egui::Ui, min: f32, desired: f32) -> f32 {
    ui.available_width().clamp(min, desired.max(min))
}

/// Largura que sobra para um item que deve preencher a linha, já descontando
/// `reserved` (a faixa de ações/controles à direita).
///
/// Substitui `(ui.available_width() - 108.0).floor().max(60.0)` — a fórmula
/// existia certa, mas o número era mágico **e** o painel media o container. Aqui
/// o truncamento para baixo é deliberado: `add_sized` arredonda para cima e a
/// linha inteira ultrapassaria o painel (que então se alarga ~1px por frame).
pub fn fill_remaining(ui: &egui::Ui, reserved: f32, min: f32) -> f32 {
    (ui.available_width() - reserved).floor().max(min)
}

/// Especificação de uma faixa de colunas de largura igual.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PetuniaColumnSpec {
    /// Quantos itens serão desenhados.
    pub count: usize,
    /// Teto de colunas por linha (normalmente igual a `count`).
    pub max_columns: u16,
    /// Menor largura **utilizável** de um item; `0.0` significa "cabe sempre"
    /// (faixas que nunca podem quebrar, como abas).
    pub min_item: f32,
    /// Vão horizontal entre colunas.
    pub gap: f32,
}

impl PetuniaColumnSpec {
    /// Faixa de `count` itens que nunca quebra linha.
    pub const fn fixed(count: usize, gap: f32) -> Self {
        Self {
            count,
            max_columns: count as u16,
            min_item: 0.0,
            gap,
        }
    }

    /// Faixa que quebra quando um item não cabe com `min_item` de largura.
    pub const fn responsive(count: usize, min_item: f32, gap: f32) -> Self {
        Self {
            count,
            max_columns: count as u16,
            min_item,
            gap,
        }
    }

    /// Define o teto de colunas por linha.
    pub const fn with_max_columns(mut self, max_columns: u16) -> Self {
        self.max_columns = max_columns;
        self
    }
}

/// Renderiza colunas de largura igual e entrega a largura resolvida a cada item.
///
/// [`PetuniaColumnSpec::max_columns`] é o teto (normalmente a quantidade de
/// itens); o adapter decide quantas cabem a partir de
/// [`PetuniaColumnSpec::min_item`] e quebra em linhas o restante.
///
/// O `f32` do callback é a largura **definitiva** da célula — já correta na
/// primeira passagem de medida do `egui_taffy`, e não zero. Por isso o
/// componente nunca precisa ler `ui.available_width()` para dimensionar o que
/// desenha: ele recebe a largura pronta e a repassa ao campo.
pub fn columns<R>(
    ui: &mut egui::Ui,
    id: impl Into<egui::Id>,
    spec: PetuniaColumnSpec,
    mut items: impl FnMut(usize, f32, &mut egui::Ui),
    mut after: impl FnMut(&mut egui::Ui) -> R,
) -> R {
    use egui_taffy::{TuiBuilderLogic, taffy};
    let gap = spec.gap.max(0.0);
    let (_, width) = fit_columns(ui.available_width(), spec.max_columns, spec.min_item, gap);
    let st = taffy::Style {
        flex_direction: taffy::FlexDirection::Row,
        flex_wrap: taffy::FlexWrap::Wrap,
        justify_content: Some(taffy::JustifyContent::Start),
        align_items: Some(taffy::AlignItems::Stretch),
        gap: taffy::Size {
            width: taffy::LengthPercentage::length(gap),
            height: taffy::LengthPercentage::length(gap),
        },
        ..Default::default()
    };
    let mut out = None;
    egui_taffy::tui(ui, id)
        .reserve_available_width()
        .style(st)
        .show(|tui| {
            for index in 0..spec.count {
                // Basis em **comprimento**, não em percentual: `percent()` é
                // resolvido contra um pai de tamanho automático na passagem de
                // medida e volta zero — a célula colapsaria para a largura do
                // próprio conteúdo. Comprimento definido é estável nas duas
                // passagens, então a célula tem a largura certa desde a medida.
                //
                // O estilo é **encadeado** no `add`: `tui.style(..)` solto é
                // descartado antes do filho, e a célula volta a ser do tamanho
                // do conteúdo (o bug silencioso que este adapter tinha).
                tui.style(taffy::Style {
                    flex_basis: taffy::LengthPercentage::length(width).into(),
                    flex_grow: 0.0,
                    flex_shrink: 0.0,
                    ..Default::default()
                })
                .ui(|ui| items(index, width, ui));
            }
            out = Some(tui.ui(|ui| after(ui)));
        });
    out.expect("sublayout taffy sempre produz a resposta final")
}

/// Linha de duas células **assimétricas**: rótulo de largura declarada e
/// controle que recebe toda a largura restante.
///
/// Existe separada de [`columns`] porque a grade entrega a **mesma** largura a
/// todas as células, e um formulário não é simétrico: o rótulo tem largura fixa
/// e o controle cresce. É a primitiva do contrato de formulário
/// ([`crate::adapters::form::PetuniaForm`], §48).
///
/// O controle recebe a largura **definitiva** (nunca zero) — e é a mesma
/// largura do `Ui` do item, então o campo pode repassá-la ao widget sem medir o
/// container.
///
/// A largura do rótulo é limitada ao que existe: num painel muito estreito o
/// rótulo encolhe em vez de empurrar o controle para fora.
pub fn fixed_label_row<R>(
    ui: &mut egui::Ui,
    id: impl Into<egui::Id>,
    label_width: f32,
    gap: f32,
    label: impl FnOnce(&mut egui::Ui),
    control: impl FnOnce(f32, &mut egui::Ui) -> R,
) -> R {
    use egui_taffy::{TuiBuilderLogic, taffy};
    let gap = gap.max(0.0);
    let available = ui.available_width().max(0.0);
    // O rótulo leva a largura declarada, mas nunca domina o campo: no painel
    // muito estreito ele fica com no máximo metade da faixa útil, e o controle
    // mantém uma largura utilizável em vez de receber zero.
    let label_w = label_width.clamp(0.0, ((available - gap) * 0.5).max(0.0));
    let control_w = (available - label_w - gap).max(0.0);
    let st = taffy::Style {
        flex_direction: taffy::FlexDirection::Row,
        justify_content: Some(taffy::JustifyContent::Start),
        align_items: Some(taffy::AlignItems::Center),
        gap: taffy::Size {
            width: taffy::LengthPercentage::length(gap),
            height: taffy::LengthPercentage::length(gap),
        },
        ..Default::default()
    };
    let mut out = None;
    egui_taffy::tui(ui, id)
        .reserve_available_width()
        .style(st)
        .show(|tui| {
            tui.style(taffy::Style {
                flex_basis: taffy::LengthPercentage::length(label_w).into(),
                flex_grow: 0.0,
                flex_shrink: 0.0,
                ..Default::default()
            })
            .ui(|ui| label(ui));
            out = Some(
                tui.style(taffy::Style {
                    flex_basis: taffy::LengthPercentage::length(control_w).into(),
                    flex_grow: 1.0,
                    flex_shrink: 0.0,
                    ..Default::default()
                })
                .ui(|ui| control(control_w, ui)),
            );
        });
    out.expect("sublayout taffy sempre produz a resposta final")
}

// ------------------------------------------------------------- piloto legado

/// Linha densa chave/valor (chave à esquerda, valor à direita).
///
/// Mantida em product path porque foi o primeiro consumidor real do adapter;
/// implementada sobre a mesma API, sem `taffy::Style` no chamador.
pub fn flex_key_value_row(
    ui: &mut egui::Ui,
    id: impl Into<egui::Id>,
    key: &str,
    value: &str,
) -> egui::Response {
    let layout = PetuniaResponsiveLayout::row().with_justify(PetuniaJustify::SpaceBetween);
    let mut value_response = None;
    responsive(
        ui,
        id,
        layout,
        2,
        |index, ui| {
            let role = crate::foundation::typography::TextRole::Label;
            if index == 0 {
                ui.label(
                    crate::foundation::typography::rich(key, role)
                        .color(crate::tokens::TEXT_SECONDARY),
                );
            } else {
                value_response = Some(
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(crate::foundation::typography::rich(value, role).strong());
                    })
                    .response,
                );
            }
        },
        |_ui| (),
    );
    value_response.expect("a linha chave/valor sempre renderiza o valor")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_defaults_are_row() {
        assert_eq!(PetuniaLayoutMode::default(), PetuniaLayoutMode::Row);
        assert_eq!(
            PetuniaResponsiveLayout::default().mode,
            PetuniaLayoutMode::Row
        );
    }

    #[test]
    fn wrap_row_only_differs_by_wrap() {
        let plain = PetuniaResponsiveLayout::row();
        let wrapped = PetuniaResponsiveLayout::wrap_row();
        assert!(!plain.wrap && wrapped.wrap);
        assert_eq!(plain.mode, wrapped.mode);
        assert_eq!(plain.gap, wrapped.gap);
    }

    /// Executa `f` numa UI de largura fixa e devolve o valor medido.
    fn measure_at_width(width: f32, mut f: impl FnMut(&mut egui::Ui) -> f32) -> f32 {
        let ctx = egui::Context::default();
        let raw = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::pos2(0.0, 0.0),
                egui::vec2(width, 600.0),
            )),
            ..Default::default()
        };
        let mut got = f32::NAN;
        let mut out = ctx.run_ui(raw, |ui| {
            got = f(ui);
        });
        out.textures_delta.clear();
        got
    }

    #[test]
    fn clamped_width_respects_the_floor_and_the_ceiling() {
        assert_eq!(
            measure_at_width(80.0, |ui| clamped_width(ui, 96.0, 180.0)),
            96.0
        );
        assert_eq!(
            measure_at_width(900.0, |ui| clamped_width(ui, 96.0, 180.0)),
            180.0
        );
    }

    #[test]
    fn fill_remaining_never_goes_below_the_minimum() {
        assert_eq!(
            measure_at_width(200.0, |ui| fill_remaining(ui, 108.0, 60.0)),
            92.0
        );
        assert_eq!(
            measure_at_width(120.0, |ui| fill_remaining(ui, 108.0, 60.0)),
            60.0,
            "estreito demais → piso"
        );
    }

    #[test]
    fn column_spec_builders_keep_the_contract_readable() {
        let fixed = PetuniaColumnSpec::fixed(3, 4.0);
        assert_eq!(
            (fixed.count, fixed.max_columns, fixed.min_item),
            (3, 3, 0.0)
        );
        let resp = PetuniaColumnSpec::responsive(3, 78.0, 6.0);
        assert_eq!((resp.max_columns, resp.min_item), (3, 78.0));
        assert_eq!(
            PetuniaColumnSpec::fixed(7, 8.0)
                .with_max_columns(3)
                .max_columns,
            3
        );
    }

    #[test]
    fn column_width_never_overflows_and_floors() {
        for available in [0.0f32, 1.0, 63.5, 130.0, 232.0, 320.0, 800.0] {
            for columns in 1u16..=4 {
                let gap = 4.0;
                let w = column_width(available, columns, gap);
                assert!(w >= 0.0, "largura negativa em {available}x{columns}");
                // As **larguras** nunca ultrapassam o container. Os gaps podem
                // não caber num container minúsculo; aí o wrap separa as linhas.
                assert!(
                    w * columns as f32 <= available.max(0.0),
                    "{columns} colunas de {w} ultrapassam {available}"
                );
                assert_eq!(w, w.floor(), "a largura precisa ficar truncada");
            }
        }
        assert_eq!(column_width(100.0, 0, 4.0), 0.0);
    }

    #[test]
    fn fit_columns_derives_the_count_from_the_item_minimum() {
        // Um item de 78 + gap de 6: 3 cabem a partir de 3*78 + 2*6 = 246.
        assert_eq!(fit_columns(245.0, 3, 78.0, 6.0).0, 2);
        assert_eq!(fit_columns(246.0, 3, 78.0, 6.0).0, 3);
        assert_eq!(
            fit_columns(1000.0, 3, 78.0, 6.0).0,
            3,
            "o teto é respeitado"
        );
        assert_eq!(fit_columns(40.0, 3, 78.0, 6.0).0, 1);
        assert_eq!(fit_columns(0.0, 3, 78.0, 6.0).0, 1);
        assert_eq!(fit_columns(500.0, 0, 10.0, 4.0), (0, 0.0));
    }

    #[test]
    fn fit_columns_with_zero_minimum_keeps_every_column_on_one_line() {
        // Abas: o teto é sempre atingido; a largura é a divisão igual.
        for available in [120.0f32, 232.0, 480.0] {
            let (n, w) = fit_columns(available, 3, 0.0, 4.0);
            assert_eq!(n, 3, "abas nunca quebram linha");
            assert_eq!(w, ((available - 8.0) / 3.0).floor());
        }
    }

    #[test]
    fn columns_hands_each_item_its_resolved_width_on_one_line() {
        let ctx = egui::Context::default();
        let raw = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::pos2(0.0, 0.0),
                egui::vec2(320.0, 600.0),
            )),
            ..Default::default()
        };
        let mut seen: Vec<(usize, f32, f32)> = Vec::new();
        let mut out = ctx.run_ui(raw, |ui| {
            seen.clear();
            columns(
                ui,
                "columns-equal-width",
                PetuniaColumnSpec::responsive(3, 78.0, 6.0),
                |index, width, ui| {
                    let rect = ui.allocate_rect(ui.max_rect(), egui::Sense::hover()).rect;
                    seen.push((index, width, rect.width()));
                },
                |_ui| (),
            );
        });
        out.textures_delta.clear();
        assert_eq!(seen.len(), 3);
        let widths: Vec<f32> = seen.iter().map(|(_, w, _)| *w).collect();
        assert_eq!(widths, vec![102.0, 102.0, 102.0]);
        for (_, promised, actual) in &seen {
            assert!(
                (promised - actual).abs() < 0.5,
                "a largura prometida ({promised}) precisa ser a alocada ({actual})"
            );
        }
    }

    #[test]
    fn columns_stacks_full_width_when_only_one_fits() {
        let ctx = egui::Context::default();
        let raw = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::pos2(0.0, 0.0),
                egui::vec2(120.0, 600.0),
            )),
            ..Default::default()
        };
        let mut seen: Vec<(f32, f32)> = Vec::new();
        let mut out = ctx.run_ui(raw, |ui| {
            seen.clear();
            columns(
                ui,
                "columns-stacked",
                PetuniaColumnSpec::responsive(3, 78.0, 6.0),
                |_index, width, ui| {
                    let rect = ui.allocate_rect(ui.max_rect(), egui::Sense::hover()).rect;
                    seen.push((width, rect.top()));
                },
                |_ui| (),
            );
        });
        out.textures_delta.clear();
        assert_eq!(seen.len(), 3, "os três eixos continuam sendo desenhados");
        assert!(
            seen.iter().all(|(w, _)| (*w - 120.0).abs() < 0.5),
            "empilhado = largura cheia: {seen:?}"
        );
        let mut tops: Vec<f32> = seen.iter().map(|(_, top)| *top).collect();
        tops.dedup();
        assert_eq!(tops.len(), 3, "empilhado = três linhas distintas: {seen:?}");
    }

    #[test]
    fn grid_mode_now_really_splits_the_row() {
        let ctx = egui::Context::default();
        let raw = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::pos2(0.0, 0.0),
                egui::vec2(300.0, 600.0),
            )),
            ..Default::default()
        };
        let mut lefts: Vec<f32> = Vec::new();
        let mut out = ctx.run_ui(raw, |ui| {
            lefts.clear();
            responsive(
                ui,
                "grid-splits-row",
                PetuniaResponsiveLayout::grid(3),
                3,
                |_index, ui| {
                    let rect = ui.allocate_rect(ui.max_rect(), egui::Sense::hover()).rect;
                    lefts.push(rect.left());
                },
                |_ui| (),
            );
        });
        out.textures_delta.clear();
        assert_eq!(lefts.len(), 3);
        // 300 → 100 por coluna (8 de gap) → os itens não ficam colados à
        // esquerda, que era o sintoma do `percent()` resolvendo para zero.
        assert!(
            lefts[1] > 50.0 && lefts[2] > 100.0,
            "grade precisa distribuir a linha: {lefts:?}"
        );
    }

    #[test]
    fn grid_carries_its_column_count() {
        let grid = PetuniaResponsiveLayout::grid(3);
        assert_eq!(grid.columns(), 3);
        assert_eq!(grid.mode, PetuniaLayoutMode::Grid { columns: 3 });
        assert_eq!(PetuniaResponsiveLayout::row().columns(), 1);
    }

    #[test]
    fn gaps_come_from_foundation_tokens() {
        assert_eq!(
            PetuniaGap::related().horizontal,
            crate::foundation::spacing::RELATED
        );
        assert_eq!(
            PetuniaGap::gutter().horizontal,
            crate::foundation::spacing::GUTTER
        );
        assert_eq!(PetuniaGap::NONE.horizontal, 0.0);
    }

    #[test]
    fn builders_are_chainable_without_changing_the_mode() {
        let layout = PetuniaResponsiveLayout::column()
            .with_gap(PetuniaGap::gutter())
            .with_align(PetuniaAlign::Stretch)
            .with_justify(PetuniaJustify::Center);
        assert_eq!(layout.mode, PetuniaLayoutMode::Column);
        assert_eq!(layout.gap, PetuniaGap::gutter());
        assert_eq!(layout.align, PetuniaAlign::Stretch);
        assert_eq!(layout.justify, PetuniaJustify::Center);
    }

    #[test]
    fn style_maps_every_mode_and_never_panics() {
        for layout in [
            PetuniaResponsiveLayout::row(),
            PetuniaResponsiveLayout::column(),
            PetuniaResponsiveLayout::wrap_row(),
            PetuniaResponsiveLayout::grid(2),
        ] {
            let st = style(layout);
            assert_eq!(
                st.gap.width,
                taffy::LengthPercentage::length(layout.gap.horizontal)
            );
        }
    }

    #[test]
    fn responsive_renders_and_reports_the_tail_response() {
        let ctx = egui::Context::default();
        ctx.run_ui(egui::RawInput::default(), |ui| {
            for n in 0..3 {
                let marker = responsive(
                    ui,
                    format!("taffy-adapter-{n}"),
                    PetuniaResponsiveLayout::wrap_row(),
                    3,
                    |index, ui| {
                        ui.label(format!("item {index}"));
                    },
                    |ui| ui.min_rect().width(),
                );
                assert!(marker >= 0.0);
            }
        })
        .textures_delta
        .clear();
    }

    #[test]
    fn fixed_label_row_gives_the_label_its_width_and_the_control_the_rest() {
        for available in [240.0, 360.0, 640.0] {
            let control = measure_at_width(available, |ui| {
                fixed_label_row(ui, "form-row", 96.0, 12.0, |_ui| {}, |width, _ui| width)
            });
            let expected = available - 96.0 - 12.0;
            assert!(
                (control - expected).abs() <= 1.0,
                "largura {available}: controle {control}, esperado {expected}"
            );
        }
    }

    #[test]
    fn fixed_label_row_never_starves_the_control() {
        // Painel mais estreito que rótulo + vão: o rótulo encolhe e o controle
        // continua com largura utilizável (nunca zero, nunca negativa).
        let control = measure_at_width(40.0, |ui| {
            fixed_label_row(
                ui,
                "form-row-narrow",
                96.0,
                12.0,
                |_ui| {},
                |width, _ui| width,
            )
        });
        assert!(control > 0.0, "controle recebeu largura {control}");
        assert!(
            control <= 40.0,
            "controle passou da largura do painel: {control}"
        );
    }

    #[test]
    fn key_value_row_still_renders_without_panic() {
        let ctx = egui::Context::default();
        ctx.run_ui(egui::RawInput::default(), |ui| {
            for n in 0..3 {
                let r = flex_key_value_row(ui, format!("taffy-kv-{n}"), "Tris", "1_024");
                assert!(
                    r.rect.width().is_finite(),
                    "a resposta do valor precisa ter largura finita"
                );
            }
        })
        .textures_delta
        .clear();
    }
}
