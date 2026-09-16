//! Adapter de barra responsiva de faixas (`PetuniaResponsiveToolbar`).
//!
//! Diretriz normativa: §7 e §46 da Egui Ecosystem Final Push Directive
//! (Wave 4 — Viewport Toolbar).
//!
//! ## O problema que este adapter remove
//!
//! A barra da viewport somava pixels à mão para decidir o que cabe:
//!
//! ```text
//! let domain = 28.0 + 24.0 * 3.0 + 3.0 * 2.0;
//! let menus  = 8.0 + text_w(ui, l, 12.0) + 4.0 + 16.0;   // por botão
//! let transform = 14.0 + 3.0 + 62.0 + 2.0 + 14.0 + …
//! if used + width_of(cluster) + 16.0 > avail_w { hidden.push(cluster) }
//! if ui.available_height() > 40.0 { /* duas linhas */ }
//! ```
//!
//! Toda constante ali é um número que ninguém sabe manter: mudar o ícone, o
//! idioma ou a densidade torna a soma mentira — e a soma **mente para cima**,
//! escondendo ferramenta com tela sobrando, ou para baixo, cortando a barra.
//!
//! ## O contrato
//!
//! ```text
//! PetuniaToolbarSpec        (faixas, prioridade, linhas, acesso ao overflow)
//!          ↓
//! sonda offscreen            (largura/altura REAIS de cada faixa)
//!          ↓
//! plan_row                   (aritmética pura, testável sem egui)
//!          ↓
//! taffy flex row             (gap, alinhamento, distribuição)
//! ```
//!
//! O product code declara **semântica** (quais faixas existem, quais podem cair,
//! o que a seta esconde). O adapter responde por **layout e medição**.
//!
//! ## Como a medição funciona
//!
//! Cada faixa é desenhada uma vez numa sonda fora da tela: retângulo em
//! coordenadas muito negativas, [`UiBuilder::id_salt`] próprio (nenhum widget
//! compartilha `Id` com o desenho real e nenhum popup abre), layout
//! `left_to_right` — o mesmo da linha. O resultado é `min_rect()`, a largura
//! real que a faixa ocupa. Nada é somado, nada é estimado.
//!
//! ## Fluxo de uso
//!
//! ```ignore
//! let plan = toolbar.show(ui, &mut |ui, slot| match slot.id {
//!     SLOT_DOMAIN => draw_domain(ui, state),
//!     SLOT_OVERFLOW => draw_overflow(ui, state, slot.hidden),
//!     …
//! });
//! ```
//!
//! `show` mede, decide e desenha (uma ou duas linhas). Nenhum `available_width`
//! calculado, nenhum breakpoint de altura, nenhuma soma de ícones.

use egui::{Align, Layout, Rect, Sense, Ui, UiBuilder, pos2, vec2};

use super::taffy_layout::{
    PetuniaGap, PetuniaItemLayout, PetuniaJustify, PetuniaResponsiveLayout, responsive,
};
use crate::foundation::spacing::CONTROL;

/// Identificador estável de uma faixa da barra.
pub type PetuniaToolbarId = &'static str;

/// Intenção de permanência de uma faixa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetuniaToolbarPriority {
    /// Nunca sai da linha: é o núcleo do produto.
    ///
    /// O adapter recusa ocultar uma faixa `Pinned` mesmo que ela não caiba — a
    /// responsabilidade de manter o núcleo pequeno é do produto.
    Pinned,
    /// Sai para o acesso de overflow quando a largura não permite.
    ///
    /// `rank` maior = mais protegido (cai depois). A ordem na linha continua
    /// sendo a ordem da lista; o `rank` decide só a queda.
    Overflowable {
        /// Proteção relativa dentro da linha.
        rank: u8,
    },
}

/// Uma faixa declarada pelo produto.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PetuniaToolbarCluster {
    /// Identificador usado pelo callback de desenho.
    pub id: PetuniaToolbarId,
    /// Se a faixa pode cair quando falta largura.
    pub priority: PetuniaToolbarPriority,
}

impl PetuniaToolbarCluster {
    /// Faixa que nunca é ocultada.
    pub const fn pinned(id: PetuniaToolbarId) -> Self {
        Self {
            id,
            priority: PetuniaToolbarPriority::Pinned,
        }
    }

    /// Faixa ocultável; `rank` maior cai depois.
    pub const fn overflowable(id: PetuniaToolbarId, rank: u8) -> Self {
        Self {
            id,
            priority: PetuniaToolbarPriority::Overflowable { rank },
        }
    }

    /// `true` quando a faixa não pode ser ocultada.
    pub const fn is_pinned(&self) -> bool {
        matches!(self.priority, PetuniaToolbarPriority::Pinned)
    }

    /// Ranking de queda (`u8::MAX` para pinadas, que não caem).
    const fn rank(&self) -> u8 {
        match self.priority {
            PetuniaToolbarPriority::Pinned => u8::MAX,
            PetuniaToolbarPriority::Overflowable { rank } => rank,
        }
    }
}

/// Declaração completa de uma barra responsiva.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PetuniaToolbarSpec {
    /// Faixas da linha principal, na ordem em que aparecem.
    pub primary: &'static [PetuniaToolbarCluster],
    /// Faixas que só existem quando a barra usa duas linhas.
    ///
    /// Com uma linha, `primary` e `secondary` aparecem juntas na mesma linha,
    /// na mesma ordem.
    pub secondary: &'static [PetuniaToolbarCluster],
    /// Máximo de linhas (`1` desliga a segunda linha).
    pub max_rows: u8,
    /// Faixa que dá acesso às ocultadas. `None` desliga o overflow.
    pub overflow: Option<PetuniaToolbarId>,
}

impl PetuniaToolbarSpec {
    /// Faixas da linha principal mais as da segunda, na ordem visual.
    fn all(&self) -> Vec<PetuniaToolbarCluster> {
        let mut out = Vec::with_capacity(self.primary.len() + self.secondary.len());
        out.extend_from_slice(self.primary);
        out.extend_from_slice(self.secondary);
        out
    }
}

/// Contexto entregue ao desenho de uma faixa.
#[derive(Debug, Clone, Copy)]
pub struct PetuniaToolbarSlot<'a> {
    /// Faixa sendo desenhada — um dos ids declarados na [`PetuniaToolbarSpec`].
    pub id: PetuniaToolbarId,
    /// Linha em que a faixa está sendo desenhada (`1` ou `2`).
    pub row: u8,
    /// Faixas ocultas nesta linha.
    ///
    /// Só chega preenchido para a faixa de overflow: é o que a seta precisa
    /// tornar alcançável.
    pub hidden: &'a [PetuniaToolbarId],
}

/// Partição de uma linha entre faixas visíveis e ocultas.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PetuniaToolbarRowPlan {
    visible: Vec<PetuniaToolbarId>,
    hidden: Vec<PetuniaToolbarId>,
}

impl PetuniaToolbarRowPlan {
    /// Faixas visíveis, na ordem da linha.
    pub fn visible(&self) -> &[PetuniaToolbarId] {
        &self.visible
    }

    /// Faixas ocultas, na ordem da linha.
    pub fn hidden(&self) -> &[PetuniaToolbarId] {
        &self.hidden
    }

    /// `true` quando a faixa está visível nesta linha.
    pub fn is_visible(&self, id: &str) -> bool {
        self.visible.contains(&id)
    }

    /// `true` quando algo foi ocultado.
    pub fn has_hidden(&self) -> bool {
        !self.hidden.is_empty()
    }
}

/// Resultado do planejamento: quantas linhas, quem ficou visível e as medidas
/// reais usadas na decisão.
#[derive(Debug, Clone, PartialEq)]
pub struct PetuniaToolbarPlan {
    rows: u8,
    line_height: f32,
    divider_width: f32,
    line1: PetuniaToolbarRowPlan,
    line2: PetuniaToolbarRowPlan,
}

impl PetuniaToolbarPlan {
    /// Linhas em uso (`1` ou `2`).
    pub fn rows(&self) -> u8 {
        self.rows
    }

    /// `true` quando a faixa de acesso ao overflow está desenhada na linha.
    pub fn overflow_visible(&self, overflow: PetuniaToolbarId) -> bool {
        self.is_visible(overflow)
    }

    /// Altura real de uma linha, medida da sonda.
    pub fn line_height(&self) -> f32 {
        self.line_height
    }

    /// Largura real do divisor entre faixas, medida do estilo.
    pub fn divider_width(&self) -> f32 {
        self.divider_width
    }

    /// Plano da linha `row` (`1` ou `2`; linha inexistente é vazia).
    pub fn line(&self, row: u8) -> &PetuniaToolbarRowPlan {
        match row {
            2 => &self.line2,
            _ => &self.line1,
        }
    }

    /// Faixas visíveis na linha `row`, na ordem.
    pub fn visible(&self, row: u8) -> &[PetuniaToolbarId] {
        self.line(row).visible()
    }

    /// Faixas ocultas na linha `row`, na ordem.
    pub fn hidden(&self, row: u8) -> &[PetuniaToolbarId] {
        self.line(row).hidden()
    }

    /// `true` quando a faixa está visível em alguma linha.
    pub fn is_visible(&self, id: &str) -> bool {
        self.line1.is_visible(id) || self.line2.is_visible(id)
    }

    /// Todas as faixas ocultas (linha 1 seguida da linha 2).
    pub fn hidden_all(&self) -> Vec<PetuniaToolbarId> {
        self.line1
            .hidden()
            .iter()
            .chain(self.line2.hidden())
            .copied()
            .collect()
    }

    /// `true` quando há algo oculto em qualquer linha.
    pub fn has_hidden(&self) -> bool {
        self.line1.has_hidden() || self.line2.has_hidden()
    }
}

/// Quantas linhas cabem na altura disponível.
///
/// Substitui o breakpoint `ui.available_height() > 40.0` do product code: a
/// decisão usa a **altura medida** de uma linha e o gap vertical real do tema,
/// então uma linha nunca é espremida dentro de um espaço que só cabe meia.
pub fn rows_that_fit(available_height: f32, line_height: f32, row_gap: f32, max_rows: u8) -> u8 {
    let max_rows = max_rows.max(1);
    if max_rows == 1 || line_height <= 0.0 {
        return 1;
    }
    let step = line_height + row_gap.max(0.0);
    // `available + gap` desconta o gap que a última linha não usa.
    let fits = ((available_height + row_gap.max(0.0)) / step).floor();
    (fits.max(1.0) as u8).min(max_rows).max(1)
}

/// Faixa de acesso ao overflow: identificador + largura real medida.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PetuniaToolbarOverflow {
    /// Identificador desenhado no fim da linha quando algo é ocultado.
    pub id: PetuniaToolbarId,
    /// Largura real da faixa de acesso (medida por sonda).
    pub width: f32,
}

/// Decide o que fica visível numa linha a partir de larguras **já medidas**.
///
/// Função pura — nenhuma dependência de egui, testável com larguras arbitrárias
/// (é assim que o caso de idioma longo é coberto). Toda a aritmética de overflow
/// do produto vive aqui.
///
/// Regras:
///
/// * faixa `Pinned` nunca é ocultada;
/// * a linha inteira cabe? então nada é ocultado e a faixa de acesso não
///   consome largura (sem desperdício de reserva);
/// * caso contrário, a queda acontece **em ordem de `rank`** (menor cai
///   primeiro; empate → a mais à direita cai primeiro) até a linha caber, e a
///   faixa de acesso entra no fim do `visible` (é assim que a linha é
///   desenhada).
///
/// O plano devolvido descreve a linha **como ela é desenhada**: se há ocultos,
/// a faixa de acesso está entre as visíveis. Ocultar sem oferecer acesso é um
/// estado que esta função não produz.
///
/// A queda é iterativa ("derruba a menos protegida até caber") e não gulosa
/// item a item: com larguras diferentes, um guloso mantinha uma faixa de rank
/// baixo e derrubava uma de rank alto só porque ela foi avaliada depois —
/// contradizendo a própria regra de prioridade.
pub fn plan_row(
    clusters: &[PetuniaToolbarCluster],
    widths: &[f32],
    available: f32,
    divider: f32,
    overflow: Option<PetuniaToolbarOverflow>,
) -> PetuniaToolbarRowPlan {
    let n = clusters.len().min(widths.len());
    if n == 0 {
        return PetuniaToolbarRowPlan::default();
    }
    let divider = divider.max(0.0);
    let overflow_width = overflow.map(|o| o.width.max(0.0)).unwrap_or(0.0);

    let width_of = |kept: &[usize]| -> f32 {
        kept.iter().map(|index| widths[*index]).sum::<f32>()
            + divider * kept.len().saturating_sub(1) as f32
    };

    let mut kept: Vec<usize> = (0..n).collect();
    if width_of(&kept) <= available {
        return PetuniaToolbarRowPlan {
            visible: clusters[..n].iter().map(|c| c.id).collect(),
            hidden: Vec::new(),
        };
    }

    // Precisa ocultar: a faixa de acesso ocupa a própria largura **e** o divisor
    // que a separa do último item visível.
    let budget = (available - overflow_width - divider).max(0.0);
    while width_of(&kept) > budget {
        // A próxima a cair é a de menor rank; empate resolve pela ordem da
        // linha (a mais à direita cai primeiro).
        let falling = kept
            .iter()
            .copied()
            .filter(|index| !clusters[*index].is_pinned())
            .min_by(|a, b| clusters[*a].rank().cmp(&clusters[*b].rank()).then(b.cmp(a)));
        match falling {
            Some(index) => kept.retain(|other| *other != index),
            // Só restam faixas núcleo: o produto é responsável por manter o
            // núcleo pequeno (a alternativa seria cortar a barra).
            None => break,
        }
    }

    let mut visible = Vec::with_capacity(kept.len() + 1);
    let mut hidden = Vec::new();
    for (index, cluster) in clusters[..n].iter().enumerate() {
        if kept.contains(&index) {
            visible.push(cluster.id);
        } else {
            hidden.push(cluster.id);
        }
    }
    // A seta só existe quando há algo para alcançar: uma seta vazia é um
    // controle morto na barra.
    if let Some(overflow) = overflow
        && !hidden.is_empty()
    {
        visible.push(overflow.id);
    }
    PetuniaToolbarRowPlan { visible, hidden }
}

/// Largura que um plano de linha **ocupa quando desenhado**: faixas visíveis,
/// a faixa de acesso (se ela existe) e os divisores entre elas.
///
/// Existe para que quem decide centralização (a Top Bar, §25) compare a largura
/// real do desenho com a caixa disponível — sem reimplementar a conta e sem
/// supor que "cabe" só porque algo foi ocultado.
pub fn plan_row_width(
    clusters: &[PetuniaToolbarCluster],
    widths: &[f32],
    plan: &PetuniaToolbarRowPlan,
    overflow: Option<PetuniaToolbarOverflow>,
    divider: f32,
) -> f32 {
    let visible = plan.visible();
    if visible.is_empty() {
        return 0.0;
    }
    let overflow_id = overflow.map(|o| o.id);
    let mut total = 0.0_f32;
    for id in visible {
        if Some(*id) == overflow_id {
            total += overflow.map(|o| o.width.max(0.0)).unwrap_or(0.0);
            continue;
        }
        if let Some(index) = clusters.iter().position(|cluster| cluster.id == *id)
            && let Some(width) = widths.get(index)
        {
            total += width.max(0.0);
        }
    }
    total + divider.max(0.0) * (visible.len() - 1) as f32
}

/// Barra responsiva: mede, decide e desenha (§46).
#[derive(Debug, Clone, Copy)]
pub struct PetuniaResponsiveToolbar {
    id: egui::Id,
    spec: &'static PetuniaToolbarSpec,
}

impl PetuniaResponsiveToolbar {
    /// Cria a barra. O `id` é a raiz dos ids internos (linhas e sonda).
    pub fn new(id: impl Into<egui::Id>, spec: &'static PetuniaToolbarSpec) -> Self {
        Self {
            id: id.into(),
            spec,
        }
    }

    /// Mede as faixas (sonda offscreen) e decide o que cabe — sem desenhar.
    ///
    /// `slot` é chamado apenas como sondagem: o desenho acontece fora da tela,
    /// com `Id` próprio e sem interação.
    pub fn plan(
        &self,
        ui: &mut Ui,
        slot: &mut impl FnMut(&mut Ui, PetuniaToolbarSlot<'_>),
    ) -> PetuniaToolbarPlan {
        // Largura do divisor = a largura que o `Separator` do tema ocupa numa
        // linha. Não é um número nosso: sai do estilo, medido.
        let divider_width = probe(ui, "divider", Layout::left_to_right(Align::Center), |ui| {
            ui.separator();
        })
        .x;

        let all = self.spec.all();
        let mut widths = Vec::with_capacity(all.len());
        let mut line_height = 0.0_f32;
        for cluster in &all {
            let size = probe(ui, cluster.id, Layout::left_to_right(Align::Center), |ui| {
                slot(
                    ui,
                    PetuniaToolbarSlot {
                        id: cluster.id,
                        row: 1,
                        hidden: &[],
                    },
                );
            });
            widths.push(size.x);
            line_height = line_height.max(size.y);
        }

        let overflow_slot = self.spec.overflow.map(|id| PetuniaToolbarOverflow {
            id,
            width: probe(ui, id, Layout::left_to_right(Align::Center), |ui| {
                slot(
                    ui,
                    PetuniaToolbarSlot {
                        id,
                        row: 1,
                        hidden: &[],
                    },
                );
            })
            .x,
        });

        let available = ui.available_width();
        let row_gap = ui.spacing().item_spacing.y;
        let rows = rows_that_fit(
            ui.available_height(),
            line_height,
            row_gap,
            self.spec.max_rows,
        );

        let primary_len = self.spec.primary.len();
        let (line1, line2) = if rows >= 2 {
            (
                plan_row(
                    self.spec.primary,
                    &widths[..primary_len],
                    available,
                    divider_width,
                    overflow_slot,
                ),
                plan_row(
                    self.spec.secondary,
                    &widths[primary_len..],
                    available,
                    divider_width,
                    overflow_slot,
                ),
            )
        } else {
            (
                plan_row(&all, &widths, available, divider_width, overflow_slot),
                PetuniaToolbarRowPlan::default(),
            )
        };

        PetuniaToolbarPlan {
            rows,
            line_height,
            divider_width,
            line1,
            line2,
        }
    }

    /// Planeja e desenha a barra (uma ou duas linhas).
    pub fn show(
        &self,
        ui: &mut Ui,
        slot: &mut impl FnMut(&mut Ui, PetuniaToolbarSlot<'_>),
    ) -> PetuniaToolbarPlan {
        let plan = self.plan(ui, slot);
        self.draw_row(ui, 1, &plan, slot);
        if plan.rows() >= 2 {
            self.draw_row(ui, 2, &plan, slot);
        }
        plan
    }

    /// Uma linha: taffy cuida de gap, alinhamento e distribuição.
    fn draw_row(
        &self,
        ui: &mut Ui,
        row: u8,
        plan: &PetuniaToolbarPlan,
        slot: &mut impl FnMut(&mut Ui, PetuniaToolbarSlot<'_>),
    ) {
        // A lista do plano já é a sequência visual da linha, com a faixa de
        // acesso ao overflow no fim quando há ocultos. Aqui só entram os
        // divisores entre elas.
        let visible = plan.visible(row);
        let hidden = plan.hidden(row);
        if visible.is_empty() {
            return;
        }
        let mut entries: Vec<Option<PetuniaToolbarId>> = Vec::with_capacity(visible.len() * 2 + 2);
        for (index, id) in visible.iter().enumerate() {
            if index > 0 {
                entries.push(None);
            }
            entries.push(Some(*id));
        }

        let divisor_width = plan.divider_width();
        let line_height = plan.line_height();
        let overflow = self.spec.overflow;
        // Cada item é um grupo horizontal: sem isso o item herdaria o layout
        // vertical do painel e empilharia os próprios filhos.
        let layout = PetuniaResponsiveLayout::row()
            .with_gap(PetuniaGap::uniform(CONTROL))
            .with_justify(PetuniaJustify::Center)
            .with_item_layout(PetuniaItemLayout::Row);

        responsive(
            ui,
            self.id.with(("row", row)),
            layout,
            entries.len(),
            |index, ui| match entries[index] {
                Some(id) => slot(
                    ui,
                    PetuniaToolbarSlot {
                        id,
                        row,
                        hidden: if Some(id) == overflow { hidden } else { &[] },
                    },
                ),
                None => draw_divider(ui, divisor_width, line_height),
            },
            |_ui| (),
        );
    }
}

/// Divisor entre faixas.
///
/// Mesmo traço e mesma largura de estilo que o `Separator::vertical` do egui,
/// mas com altura **explícita**: dentro de um nó taffy não existe "altura
/// disponível", e um divisor que pede a altura toda inflaria a linha.
fn draw_divider(ui: &mut Ui, width: f32, height: f32) {
    let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
    let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    if ui.is_rect_visible(rect) {
        ui.painter().vline(rect.center().x, rect.y_range(), stroke);
    }
}

/// Sonda: desenha `draw` fora da tela e devolve o tamanho real ocupado.
///
/// O retângulo fica em coordenadas muito negativas — nenhum widget pode ser
/// hoverado ou clicado ali — e o `id_salt` próprio garante que nada compartilhe
/// `Id` com o desenho real (nem popup abre durante a sondagem).
///
/// A extensão **cruzada** do retângulo de sonda é zero de propósito: num layout
/// de linha o `min_rect` do egui cresce até a extensão cruzada do `max_rect`,
/// então um retângulo alto devolveria a altura da sonda em vez da altura do
/// conteúdo. Com a cruz em zero, o cursor cresce exatamente até o conteúdo — a
/// altura medida é a altura real da faixa na linha.
pub(super) fn probe<R>(
    ui: &mut Ui,
    key: &str,
    layout: Layout,
    draw: impl FnOnce(&mut Ui) -> R,
) -> egui::Vec2 {
    /// Longe o bastante para não haver interação, e largo o bastante para
    /// nenhum controle quebrar linha na medição.
    const SPAN: f32 = 16_384.0;

    let cross = if layout.main_dir().is_horizontal() {
        vec2(SPAN, 0.0)
    } else {
        vec2(0.0, SPAN)
    };
    let mut child = ui.new_child(
        UiBuilder::new()
            .max_rect(Rect::from_min_size(pos2(-SPAN, -SPAN), cross))
            .layout(layout)
            .id_salt(("petunia-toolbar-probe", key)),
    );
    draw(&mut child);
    child.min_rect().size()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clusters() -> Vec<PetuniaToolbarCluster> {
        vec![
            PetuniaToolbarCluster::pinned("domain"),
            PetuniaToolbarCluster::pinned("menus"),
            PetuniaToolbarCluster::overflowable("transform", 10),
            PetuniaToolbarCluster::overflowable("snap-prop", 20),
            PetuniaToolbarCluster::pinned("display"),
        ]
    }

    fn widths(domain: f32, menus: f32, transform: f32, snap: f32, display: f32) -> Vec<f32> {
        vec![domain, menus, transform, snap, display]
    }

    /// Faixa de acesso ao overflow, com a largura que a seta ocupa.
    fn overflow() -> Option<PetuniaToolbarOverflow> {
        Some(PetuniaToolbarOverflow {
            id: "overflow",
            width: 22.0,
        })
    }

    #[test]
    fn wide_line_keeps_everything_and_spends_no_overflow_reserve() {
        let c = clusters();
        let w = widths(120.0, 400.0, 300.0, 120.0, 260.0);
        let plan = plan_row(&c, &w, 1_400.0, 6.0, overflow());
        assert_eq!(plan.visible().len(), 5);
        assert!(plan.hidden().is_empty());

        // Exatamente o suficiente para tudo **sem** reservar a seta: o
        // total real é 1200 + 4*6 = 1224.
        let plan = plan_row(&c, &w, 1_224.0, 6.0, overflow());
        assert_eq!(plan.visible().len(), 5, "cabe por 0px de folga");
        assert!(plan.hidden().is_empty(), "nada oculto: a seta não existe");
    }

    #[test]
    fn pinned_clusters_never_leave_the_line() {
        let c = clusters();
        let w = widths(120.0, 400.0, 300.0, 120.0, 260.0);
        // Nem numa linha absurdamente estreita.
        let plan = plan_row(&c, &w, 200.0, 6.0, overflow());
        for id in ["domain", "menus", "display"] {
            assert!(plan.is_visible(id), "{id} é núcleo e não pode cair");
        }
    }

    #[test]
    fn lower_rank_falls_first_and_the_rightmost_breaks_the_tie() {
        let c = clusters();
        let w = widths(100.0, 200.0, 150.0, 150.0, 100.0);
        // Cabe domain+menus+display+snap-prop(rank 20) mas não também transform.
        let plan = plan_row(&c, &w, 620.0, 6.0, overflow());
        assert!(plan.is_visible("snap-prop"), "rank maior é protegido");
        assert!(!plan.is_visible("transform"), "rank menor cai primeiro");
        assert_eq!(plan.hidden(), ["transform"]);

        let tie = vec![
            PetuniaToolbarCluster::pinned("domain"),
            PetuniaToolbarCluster::overflowable("left", 5),
            PetuniaToolbarCluster::overflowable("right", 5),
        ];
        let plan = plan_row(&tie, &[10.0, 100.0, 100.0], 150.0, 6.0, overflow());
        assert!(plan.is_visible("left"), "empate: a mais à direita cai");
        assert!(!plan.is_visible("right"));
    }

    #[test]
    fn every_occluded_cluster_is_still_reachable_through_the_plan() {
        let c = clusters();
        let w = widths(100.0, 200.0, 150.0, 150.0, 100.0);
        for available in [120.0, 300.0, 520.0, 700.0, 1_500.0] {
            let plan = plan_row(&c, &w, available, 6.0, overflow());
            // Toda faixa do modelo está visível ou oculta — e a faixa de acesso
            // só aparece no `visible` quando existe algo para alcançar.
            let occluded = plan
                .visible()
                .iter()
                .filter(|id| **id != "overflow")
                .count();
            assert_eq!(
                occluded + plan.hidden().len(),
                c.len(),
                "faixa perdida no limbo com {available}px"
            );
            assert_eq!(
                plan.is_visible("overflow"),
                !plan.hidden().is_empty(),
                "seta e ocultos precisam aparecer juntos ({available}px)"
            );
        }
    }

    #[test]
    fn long_locale_labels_push_clusters_to_overflow_instead_of_cutting_the_bar() {
        // Idioma longo: rótulos de menu ~3x maiores.
        let c = clusters();
        let w = widths(120.0, 1_200.0, 300.0, 120.0, 260.0);
        let plan = plan_row(&c, &w, 900.0, 6.0, overflow());
        assert!(plan.is_visible("domain") && plan.is_visible("menus"));
        assert!(plan.is_visible("display"), "display é núcleo");
        assert_eq!(
            plan.hidden().len(),
            2,
            "o que não cabe vai para a seta, não para fora da barra"
        );
    }

    #[test]
    fn planned_width_counts_visible_plus_arrow_plus_dividers() {
        let c = clusters();
        let w = widths(100.0, 200.0, 150.0, 150.0, 100.0);
        let plan = plan_row(&c, &w, 620.0, 6.0, overflow());
        // domain(100) + menus(200) + snap(150) + display(100) + seta(22) + 4 vãos
        assert_eq!(plan.hidden(), ["transform"]);
        assert!((plan_row_width(&c, &w, &plan, overflow(), 6.0) - 596.0).abs() < 0.01);

        // Linha que cabe inteira: sem seta, sem reserva.
        let plan = plan_row(&c, &w, 2_000.0, 6.0, overflow());
        assert!((plan_row_width(&c, &w, &plan, overflow(), 6.0) - 724.0).abs() < 0.01);
    }

    #[test]
    fn a_more_protected_cluster_never_falls_before_a_less_protected_one() {
        // Invariante de prioridade: se algo caiu, o que ficou tem rank maior ou
        // igual. Um guloso item a item violava isto quando uma faixa larga de
        // rank alto era avaliada primeiro e não cabia.
        let c = vec![
            PetuniaToolbarCluster::overflowable("wide-protected", 30),
            PetuniaToolbarCluster::overflowable("narrow-weak", 10),
        ];
        let widths = [300.0, 100.0];
        for available in [140.0, 200.0, 260.0, 320.0, 420.0, 600.0] {
            let plan = plan_row(&c, &widths, available, 6.0, overflow());
            let rank_of = |id: &str| c.iter().find(|x| x.id == id).map(|x| x.rank());
            let lowest_visible = plan
                .visible()
                .iter()
                .filter_map(|id| rank_of(id))
                .min()
                .unwrap_or(u8::MAX);
            let highest_hidden = plan
                .hidden()
                .iter()
                .filter_map(|id| rank_of(id))
                .max()
                .unwrap_or(0);
            assert!(
                lowest_visible >= highest_hidden,
                "com {available}px o rank {highest_hidden} caiu enquanto o rank {lowest_visible} ficou"
            );
            // E o plano desenhado nunca ultrapassa a linha.
            if !plan.hidden().is_empty() {
                let drawn = plan_row_width(&c, &widths, &plan, overflow(), 6.0);
                assert!(drawn <= available + 0.01, "{drawn} > {available}");
            }
        }
    }

    #[test]
    fn empty_line_plans_to_nothing() {
        let plan = plan_row(&[], &[], 500.0, 6.0, overflow());
        assert!(plan.visible().is_empty());
        assert!(!plan.has_hidden());
    }

    #[test]
    fn rows_that_fit_uses_the_measured_line_height() {
        // Uma linha de 22px + gap de 3px.
        assert_eq!(rows_that_fit(22.0, 22.0, 3.0, 2), 1);
        assert_eq!(rows_that_fit(26.0, 22.0, 3.0, 2), 1, "26px não é 2 linhas");
        assert_eq!(rows_that_fit(47.0, 22.0, 3.0, 2), 2);
        assert_eq!(rows_that_fit(400.0, 22.0, 3.0, 2), 2, "teto respeitado");
        assert_eq!(
            rows_that_fit(400.0, 22.0, 3.0, 1),
            1,
            "max_rows=1 é sempre 1"
        );
        assert_eq!(
            rows_that_fit(400.0, 0.0, 3.0, 2),
            1,
            "sem medida, sem linha"
        );
    }

    #[test]
    fn probe_measures_real_widths_off_screen() {
        let ctx = egui::Context::default();
        ctx.run_ui(egui::RawInput::default(), |ui| {
            // Um divisor de estilo tem largura de tema (não zero, não absurda).
            // A altura medida é a do **conteúdo**, não a do retângulo da sonda.
            let divider = probe(ui, "divider", Layout::left_to_right(Align::Center), |ui| {
                ui.separator();
            });
            assert!(
                divider.x > 0.0 && divider.x < 40.0,
                "largura de divisor fora do plausível: {divider:?}"
            );

            // Uma faixa larga mede perto da largura pedida, não da sonda.
            let wide = probe(ui, "wide", Layout::left_to_right(Align::Center), |ui| {
                let (rect, _) = ui.allocate_exact_size(vec2(300.0, 22.0), Sense::hover());
                let _ = rect;
            });
            assert!(
                (wide.x - 300.0).abs() < 1.0,
                "sonda não mediu o conteúdo: {wide:?}"
            );
            assert!(
                (wide.y - 22.0).abs() < 1.0,
                "sonda não mediu a altura do conteúdo: {wide:?}"
            );
        })
        .textures_delta
        .clear();
    }
}
