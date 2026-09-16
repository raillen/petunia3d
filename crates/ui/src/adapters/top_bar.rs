//! Adapter da Top Bar de três zonas (`PetuniaTopBar`, §25).
//!
//! Diretriz normativa: §25 da Egui Ecosystem Final Push Directive
//! (Wave 5 — Top Bar e shell).
//!
//! ## O problema que este adapter remove
//!
//! O header somava a centralização por colunas e espaçadores:
//!
//! ```text
//! ui.columns(3, |cols| { cols[0]… cols[1].horizontal_centered(…) cols[2]… });
//! ui.spacing_mut().item_spacing = vec2(6.0, 0.0);   // por coluna
//! ```
//!
//! O resultado não é centralização: as pílulas ficam no meio do **terço** do
//! meio, que só coincide com o meio da barra quando as laterais têm a mesma
//! largura. Idioma longo de um lado e as pílulas deslizam.
//!
//! ## O contrato
//!
//! ```text
//! PetuniaTopBarSpec      (zonas e prioridade — semântica do produto)
//!          ↓
//! sonda offscreen         (largura real de cada faixa)
//!          ↓
//! plan_top_bar            (aritmética pura, testável sem egui)
//!          ↓
//! three_zone              (taffy: caixas laterais de largura igual)
//! ```
//!
//! Com o centro geométrico possível, [`PetuniaCentering::Centered`] dá às
//! laterais caixas de largura igual — o centro cai no meio exato da barra. Se
//! uma lateral não cabe na metade, o adapter escolhe o fallback
//! [`PetuniaCentering::Drifting`] **por medição** (nada é cortado, o centro
//! deixa de ser geométrico) em vez de esconder o problema.

use egui::Ui;

use super::taffy_layout::{
    PetuniaCentering, PetuniaGap, PetuniaItemLayout, PetuniaJustify, PetuniaResponsiveLayout,
    PetuniaZone, centered_side_width, responsive, three_zone,
};
use super::toolbar::{
    PetuniaToolbarCluster, PetuniaToolbarId, PetuniaToolbarOverflow, plan_row, plan_row_width,
    probe,
};
use crate::foundation::spacing::CONTROL;

/// Zona da Top Bar em que uma faixa foi desenhada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PetuniaTopBarZone {
    /// Menus do sistema / do projeto.
    Left,
    /// Pills de workspace.
    Center,
    /// Ações globais.
    Right,
}

/// Declaração da Top Bar: três zonas de faixas, com prioridade de queda na
/// direita (a única zona que pode perder itens para o overflow).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PetuniaTopBarSpec {
    /// Zona esquerda, na ordem visual. Nunca cai: é navegação.
    pub left: &'static [PetuniaToolbarCluster],
    /// Zona central, na ordem visual. Nunca cai: é o seletor de workspace.
    pub center: &'static [PetuniaToolbarCluster],
    /// Zona direita, na ordem visual. Faixas com `rank` menor caem primeiro.
    pub right: &'static [PetuniaToolbarCluster],
    /// Faixa que dá acesso às ações ocultas (`None` desliga o overflow).
    pub overflow: Option<PetuniaToolbarId>,
}

/// Contexto entregue ao desenho de uma faixa da Top Bar.
#[derive(Debug, Clone, Copy)]
pub struct PetuniaTopBarSlot<'a> {
    /// Faixa sendo desenhada — um dos ids declarados na spec.
    pub id: PetuniaToolbarId,
    /// Zona em que a faixa está.
    pub zone: PetuniaTopBarZone,
    /// Faixas ocultas desta barra (só chega preenchido na faixa de overflow).
    pub hidden: &'a [PetuniaToolbarId],
}

/// Decisão de layout da Top Bar: modo de centralização e o que ficou visível na
/// zona direita.
#[derive(Debug, Clone, PartialEq)]
pub struct PetuniaTopBarPlan {
    centering: PetuniaCentering,
    left_width: f32,
    center_width: f32,
    right_visible: Vec<PetuniaToolbarId>,
    right_hidden: Vec<PetuniaToolbarId>,
}

impl PetuniaTopBarPlan {
    /// Modo de centralização escolhido pela medição.
    pub fn centering(&self) -> PetuniaCentering {
        self.centering
    }

    /// `true` quando o centro cai no meio geométrico da barra.
    pub fn is_centered(&self) -> bool {
        self.centering == PetuniaCentering::Centered
    }

    /// Larguras reais medidas (testes e diagnóstico).
    pub fn measured_widths(&self) -> (f32, f32) {
        (self.left_width, self.center_width)
    }

    /// Faixas da zona direita que permanecem na barra, na ordem.
    pub fn right_visible(&self) -> &[PetuniaToolbarId] {
        &self.right_visible
    }

    /// Faixas da zona direita que foram para o overflow, na ordem.
    pub fn right_hidden(&self) -> &[PetuniaToolbarId] {
        &self.right_hidden
    }

    /// `true` quando alguma ação da direita foi para o overflow.
    pub fn has_hidden(&self) -> bool {
        !self.right_hidden.is_empty()
    }

    /// `true` quando a faixa está visível na barra.
    pub fn is_visible(&self, id: &str) -> bool {
        self.right_visible.contains(&id)
    }
}

/// Larguras somadas de uma zona, incluindo os vãos entre as faixas.
fn zone_width(widths: &[f32], gap: f32) -> f32 {
    if widths.is_empty() {
        return 0.0;
    }
    let sum: f32 = widths.iter().sum();
    sum + gap * (widths.len() - 1) as f32
}

/// Decide centralização e overflow a partir de larguras **já medidas**.
///
/// Função pura — testável com larguras arbitrárias (é assim que o idioma longo
/// é coberto). Regras:
///
/// * cabe nos dois lados com caixas iguais? centro geométrico, nada oculto;
/// * a esquerda cabe mas a direita não? o centro continua geométrico e as ações
///   de `rank` menor caem para o overflow **dentro da mesma caixa**;
/// * nem a esquerda cabe na metade? fallback declarado: laterais empacotadas nas
///   bordas, centro onde couber, nada cortado.
pub fn plan_top_bar(
    left_width: f32,
    center_width: f32,
    right: &[PetuniaToolbarCluster],
    right_widths: &[f32],
    available: f32,
    gap: f32,
    overflow: Option<PetuniaToolbarOverflow>,
) -> PetuniaTopBarPlan {
    let gap = gap.max(0.0);
    let side = centered_side_width(available, center_width, gap);
    let right_total = zone_width(&right_widths[..right.len().min(right_widths.len())], gap);

    if left_width <= side && right_total <= side {
        return PetuniaTopBarPlan {
            centering: PetuniaCentering::Centered,
            left_width,
            center_width,
            right_visible: right.iter().map(|c| c.id).collect(),
            right_hidden: Vec::new(),
        };
    }

    // Orçamento da zona direita: a metade da barra no modo centralizado; o que
    // sobra de verdade quando a esquerda não cabe (fallback).
    let budget = if left_width <= side {
        side
    } else {
        (available - left_width - center_width - 2.0 * gap).max(0.0)
    };
    let row = plan_row(right, right_widths, budget, gap, overflow);
    // A centralização geométrica exige que o desenho da direita caiba na caixa
    // igual à da esquerda — medido do plano real (visíveis + seta + vãos), não
    // suposto a partir de "algo foi ocultado".
    let fits = plan_row_width(right, right_widths, &row, overflow, gap) <= side + 0.01;
    let centering = if left_width <= side && fits {
        PetuniaCentering::Centered
    } else {
        PetuniaCentering::Drifting
    };

    PetuniaTopBarPlan {
        centering,
        left_width,
        center_width,
        right_visible: row.visible().to_vec(),
        right_hidden: row.hidden().to_vec(),
    }
}

/// Top Bar de três zonas: mede, decide e desenha (§25).
#[derive(Debug, Clone, Copy)]
pub struct PetuniaTopBar {
    id: egui::Id,
    spec: &'static PetuniaTopBarSpec,
}

impl PetuniaTopBar {
    /// Cria a barra. O `id` é a raiz dos ids internos (zonas e sonda).
    pub fn new(id: impl Into<egui::Id>, spec: &'static PetuniaTopBarSpec) -> Self {
        Self {
            id: id.into(),
            spec,
        }
    }

    /// Mede as faixas (sonda offscreen) e decide o plano — sem desenhar.
    pub fn plan(
        &self,
        ui: &mut Ui,
        slot: &mut impl FnMut(&mut Ui, PetuniaTopBarSlot<'_>),
    ) -> PetuniaTopBarPlan {
        let gap = CONTROL;
        let row_layout = egui::Layout::left_to_right(egui::Align::Center);

        let measure = |ui: &mut Ui,
                       clusters: &[PetuniaToolbarCluster],
                       zone: PetuniaTopBarZone,
                       slot: &mut dyn FnMut(&mut Ui, PetuniaTopBarSlot<'_>)|
         -> Vec<f32> {
            clusters
                .iter()
                .map(|cluster| {
                    probe(ui, cluster.id, row_layout, |ui| {
                        slot(
                            ui,
                            PetuniaTopBarSlot {
                                id: cluster.id,
                                zone,
                                hidden: &[],
                            },
                        );
                    })
                    .x
                })
                .collect()
        };

        let left_widths = measure(ui, self.spec.left, PetuniaTopBarZone::Left, slot);
        let center_widths = measure(ui, self.spec.center, PetuniaTopBarZone::Center, slot);
        let right_widths = measure(ui, self.spec.right, PetuniaTopBarZone::Right, slot);

        let overflow = self.spec.overflow.map(|id| PetuniaToolbarOverflow {
            id,
            width: probe(ui, id, row_layout, |ui| {
                slot(
                    ui,
                    PetuniaTopBarSlot {
                        id,
                        zone: PetuniaTopBarZone::Right,
                        hidden: &[],
                    },
                );
            })
            .x,
        });

        plan_top_bar(
            zone_width(&left_widths, gap),
            zone_width(&center_widths, gap),
            self.spec.right,
            &right_widths,
            ui.available_width(),
            gap,
            overflow,
        )
    }

    /// Planeja e desenha a barra.
    pub fn show(
        &self,
        ui: &mut Ui,
        slot: &mut impl FnMut(&mut Ui, PetuniaTopBarSlot<'_>),
    ) -> PetuniaTopBarPlan {
        let plan = self.plan(ui, slot);
        let hidden = plan.right_hidden().to_vec();
        let overflow = self.spec.overflow;

        // A ordem visual da direita é a ordem da spec; a seta de acesso fica no
        // fim da linha (posição única para o mesmo controle, como na barra da
        // viewport), então ela entra por último na lista.
        let mut right_visible: Vec<PetuniaToolbarId> = plan
            .right_visible()
            .iter()
            .copied()
            .filter(|id| Some(*id) != overflow)
            .collect();
        if let Some(overflow) = overflow
            && plan.has_hidden()
        {
            right_visible.push(overflow);
        }

        three_zone(
            ui,
            self.id.with("zones"),
            CONTROL,
            plan.centering(),
            |zone, ui| {
                let (clusters, zone_id) = match zone {
                    PetuniaZone::Left => (self.spec.left, PetuniaTopBarZone::Left),
                    PetuniaZone::Center => (self.spec.center, PetuniaTopBarZone::Center),
                    PetuniaZone::Right => (self.spec.right, PetuniaTopBarZone::Right),
                };
                let mut cells: Vec<(PetuniaToolbarId, bool)> = clusters
                    .iter()
                    .map(|c| c.id)
                    .filter(|id| zone != PetuniaZone::Right || right_visible.contains(id))
                    .map(|id| (id, false))
                    .collect();
                // A seta de acesso fica no fim da linha (posição única para o
                // mesmo controle, como na barra da viewport) e é a única célula
                // que recebe a lista de faixas ocultas.
                if zone == PetuniaZone::Right
                    && let Some(overflow) = overflow
                    && plan.has_hidden()
                {
                    cells.push((overflow, true));
                }
                if cells.is_empty() {
                    return;
                }
                let count = cells.len();
                // A ordem visual é a ordem da lista em todas as zonas: a direita
                // se alinha pela borda com `End`, não por direção de `Ui`.
                let layout = PetuniaResponsiveLayout::row()
                    .with_gap(PetuniaGap::uniform(CONTROL))
                    .with_item_layout(PetuniaItemLayout::Row)
                    .with_justify(match zone_id {
                        PetuniaTopBarZone::Right => PetuniaJustify::End,
                        PetuniaTopBarZone::Left | PetuniaTopBarZone::Center => {
                            PetuniaJustify::Start
                        }
                    });
                responsive(
                    ui,
                    self.id.with(("zone", zone_id as u8)),
                    layout,
                    count,
                    |index, ui| {
                        let (id, is_overflow) = cells[index];
                        slot(
                            ui,
                            PetuniaTopBarSlot {
                                id,
                                zone: zone_id,
                                hidden: if is_overflow { &hidden } else { &[] },
                            },
                        );
                    },
                    |_ui| (),
                );
            },
            |_ui| (),
        );

        plan
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clusters() -> Vec<PetuniaToolbarCluster> {
        vec![
            PetuniaToolbarCluster::overflowable("assets", 30),
            PetuniaToolbarCluster::overflowable("settings", 20),
            PetuniaToolbarCluster::overflowable("help", 10),
        ]
    }

    fn overflow() -> Option<PetuniaToolbarOverflow> {
        Some(PetuniaToolbarOverflow {
            id: "overflow",
            width: 22.0,
        })
    }

    #[test]
    fn wide_bar_centers_exactly_and_hides_nothing() {
        let c = clusters();
        let plan = plan_top_bar(
            320.0,
            240.0,
            &c,
            &[120.0, 90.0, 60.0],
            1_400.0,
            6.0,
            overflow(),
        );
        assert!(plan.is_centered(), "cabe: o centro é geométrico");
        assert!(!plan.has_hidden());
        assert_eq!(plan.right_visible().len(), 3);
        // A caixa lateral precisa comportar os dois lados.
        let side = centered_side_width(1_400.0, 240.0, 6.0);
        assert!(side >= 320.0, "caixa {side} menor que a esquerda medida");
    }

    #[test]
    fn center_is_geometric_even_when_sides_differ_wildly() {
        // Esquerda estreita, direita larga: o centro continua no meio porque o
        // plano usa caixas laterais de largura **igual**.
        let c = vec![PetuniaToolbarCluster::overflowable("wide-actions", 10)];
        let plan = plan_top_bar(120.0, 300.0, &c, &[280.0], 1_200.0, 6.0, overflow());
        assert!(plan.is_centered());
        assert!(plan.is_visible("wide-actions"));
    }

    #[test]
    fn low_rank_actions_go_to_overflow_instead_of_cutting_the_bar() {
        let c = clusters();
        // Caixa lateral: (700 − 240 − 12) / 2 = 224 — comporta a esquerda (200)
        // mas não as três ações (120 + 90 + 60 + 2 vãos = 282).
        let available = 700.0;
        let plan = plan_top_bar(
            200.0,
            240.0,
            &c,
            &[120.0, 90.0, 60.0],
            available,
            6.0,
            overflow(),
        );
        assert!(
            plan.is_centered(),
            "a esquerda ainda cabe: centro geométrico"
        );
        assert!(plan.has_hidden());
        assert!(!plan.is_visible("help"), "rank menor cai primeiro");
        assert!(
            plan.is_visible("overflow"),
            "o acesso entra junto com os ocultos"
        );
        let side = centered_side_width(available, 240.0, 6.0);
        let visible_width = 22.0
            + plan
                .right_visible()
                .iter()
                .filter(|id| **id != "overflow")
                .map(|id| match *id {
                    "assets" => 120.0,
                    "settings" => 90.0,
                    "help" => 60.0,
                    _ => 0.0,
                })
                .sum::<f32>()
            + 6.0 * (plan.right_visible().len() - 1) as f32;
        assert!(
            visible_width <= side + 0.01,
            "zona direita ({visible_width}) maior que a caixa ({side})"
        );
    }

    #[test]
    fn long_locale_keeps_the_core_and_sends_the_rest_to_overflow() {
        let c = clusters();
        // Idioma longo: menus ~3x maiores, ações também.
        let plan = plan_top_bar(
            900.0,
            700.0,
            &c,
            &[300.0, 260.0, 240.0],
            1_600.0,
            6.0,
            overflow(),
        );
        assert!(!plan.right_visible().is_empty() || plan.has_hidden());
        assert!(!plan.is_visible("help"), "o menos protegido cai");
        assert!(
            plan.is_visible("overflow"),
            "o que caiu continua alcançável pela seta"
        );
    }

    #[test]
    fn narrow_bar_falls_back_to_drifting_without_losing_access() {
        let c = clusters();
        let plan = plan_top_bar(
            700.0,
            400.0,
            &c,
            &[120.0, 90.0, 60.0],
            900.0,
            6.0,
            overflow(),
        );
        assert!(
            plan.centering() == PetuniaCentering::Drifting,
            "esquerda não cabe na metade: fallback declarado"
        );
        assert!(
            plan.is_visible("overflow"),
            "no fallback a seta continua pendurada na borda direita"
        );
    }

    #[test]
    fn every_action_is_either_visible_or_reachable() {
        let c = clusters();
        for available in [700.0, 900.0, 1_100.0, 1_400.0, 2_000.0] {
            let plan = plan_top_bar(
                320.0,
                240.0,
                &c,
                &[120.0, 90.0, 60.0],
                available,
                6.0,
                overflow(),
            );
            let visible = plan
                .right_visible()
                .iter()
                .filter(|id| **id != "overflow")
                .count();
            assert_eq!(
                visible + plan.right_hidden().len(),
                c.len(),
                "ação perdida no limbo com {available}px"
            );
            assert_eq!(
                plan.is_visible("overflow"),
                plan.has_hidden(),
                "seta e ocultos precisam aparecer juntos ({available}px)"
            );
        }
    }

    #[test]
    fn empty_right_zone_still_plans_the_center() {
        let plan = plan_top_bar(200.0, 240.0, &[], &[], 1_000.0, 6.0, None);
        assert!(plan.is_centered());
        assert!(plan.right_visible().is_empty());
        assert!(!plan.has_hidden());
    }
}
