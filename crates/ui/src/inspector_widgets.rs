//! Blocos reutilizáveis do inspector contextual: densidade, campos numéricos
//! com undo correto, seções leves e abas textuais.
//!
//! Regras do sistema: tokens de densidade (sem segundo settings), undo por
//! sessão de edição (1 nível por gesto), foco visível + `widget_info` em tudo
//! custom, i18n fora daqui (chamador traduz).

use egui::{Color32, FontId, Rect, Sense, StrokeKind, Ui, WidgetInfo, WidgetType, vec2};
use petunia_core::{AppState, UiDensity};

use crate::icon_registry::{IconRegistry, PetuniaIcon};
use crate::inspector_context::InspectorTab;
use crate::tokens;
use crate::widgets::{ChevronDir, paint_chevron};

// ---------------------------------------------------------------- densidade

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
    row_h(density)
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

// ------------------------------------------------------- sessão de edição

/// Sessão de edição com checkpoint único (undo correto por gesto).
///
/// `DragValue` muta antes de avisar: checkpoint no `drag_started`
/// (pré-mutação, sem desperdício) + fallback na primeira mudança por teclado.
/// `finish` fecha a sessão (soltar botão / perder foco). UI-state vive em
/// temp-data do egui; domínio nunca é duplicado aqui.
pub struct EditSession {
    key: &'static str,
}

impl EditSession {
    pub fn new(key: &'static str) -> Self {
        Self { key }
    }

    fn flag_id(&self) -> egui::Id {
        egui::Id::new(("petunia_edit_session", self.key))
    }

    fn checkpoint_once(&self, ui: &mut Ui, state: &mut AppState, undo_label: &str) {
        let active = ui
            .ctx()
            .data(|d| d.get_temp::<bool>(self.flag_id()).unwrap_or(false));
        if !active {
            state.checkpoint(undo_label);
            ui.ctx().data_mut(|d| d.insert_temp(self.flag_id(), true));
        }
    }

    /// Observa a resposta do campo; retorna se houve mudança.
    pub fn poll(
        &self,
        ui: &mut Ui,
        state: &mut AppState,
        resp: &egui::Response,
        undo_label: &str,
        changed: bool,
    ) -> bool {
        if resp.drag_started() || changed {
            self.checkpoint_once(ui, state, undo_label);
        }
        changed
    }

    pub fn finish(&self, ui: &mut Ui, finished: bool) {
        if finished {
            ui.ctx().data_mut(|d| d.insert_temp(self.flag_id(), false));
        }
    }
}

// ------------------------------------------------------------ campo numérico

/// Opções de formatação de campo numérico.
#[derive(Debug, Clone, Copy)]
pub struct NumericOpts {
    pub speed: f32,
    pub decimals: usize,
    pub suffix: &'static str,
    pub min: f32,
    pub max: f32,
}

impl NumericOpts {
    pub fn plain(speed: f32, decimals: usize) -> Self {
        Self {
            speed,
            decimals,
            suffix: "",
            min: f32::NEG_INFINITY,
            max: f32::INFINITY,
        }
    }
}

/// Evento de campo: mudou e/ou sessão terminou (soltou/tirou foco).
#[derive(Debug, Clone, Copy, Default)]
pub struct FieldEvent {
    pub changed: bool,
    pub finished: bool,
}

/// Campo numérico com `DragValue` (arrastar + teclado + setas), formatação
/// fixa (sem ruído decimal) e undo por sessão. Retorna o evento; quem chama
/// aplica o efeito no domínio.
pub struct NumericField<'a> {
    pub value: &'a mut f32,
    pub opts: NumericOpts,
    pub session_key: &'static str,
    pub undo_label: &'a str,
    pub width: f32,
    pub height: f32,
}

impl<'a> NumericField<'a> {
    pub fn show(self, ui: &mut Ui, state: &mut AppState) -> FieldEvent {
        let session = EditSession::new(self.session_key);
        let decimals = self.opts.decimals;
        let suffix = self.opts.suffix;
        let mut staging = self.value.clamp(self.opts.min, self.opts.max);
        let resp = ui.add_sized(
            vec2(self.width, self.height),
            egui::DragValue::new(&mut staging)
                .speed(self.opts.speed)
                .range(self.opts.min..=self.opts.max)
                .custom_formatter(move |n, _| format!("{n:.decimals$}{suffix}")),
        );
        let changed = session.poll(ui, state, &resp, self.undo_label, resp.changed());
        if changed {
            *self.value = staging;
        }
        let finished = resp.drag_stopped() || resp.lost_focus();
        session.finish(ui, finished);
        FieldEvent { changed, finished }
    }
}

// --------------------------------------------------------------- vetor xyz

/// Evento por eixo do campo vetorial.
#[derive(Debug, Clone, Copy, Default)]
pub struct AxisEvent {
    pub changed: [bool; 3],
    pub finished: bool,
}

/// Campo vetorial responsivo pela largura DISPONÍVEL (nunca resolução global):
/// larga (≥300px) uma linha, média uma linha compacta, estreita (<190px) uma
/// coluna por eixo com campos cheios. Mesma sessão de undo pros 3 eixos.
pub struct Vector3Field<'a> {
    pub axis_names: [&'a str; 3],
    pub axis_colors: [Color32; 3],
    pub values: &'a mut [f32; 3],
    pub opts: NumericOpts,
    pub session_key: &'static str,
    pub undo_label: &'a str,
}

impl<'a> Vector3Field<'a> {
    pub fn show(self, ui: &mut Ui, state: &mut AppState) -> AxisEvent {
        let density = state.ui.density;
        let avail = ui.available_width().max(0.0);
        let height = input_h(density);
        let mut out = AxisEvent::default();
        let [vx, vy, vz] = self.values;
        // (largura do rótulo, vão): larga, média; vazio = coluna por eixo.
        let row: Option<(f32, f32, f32)> = if avail >= 300.0 {
            Some((14.0, 6.0, 40.0))
        } else if avail >= 190.0 {
            Some((12.0, 4.0, 32.0))
        } else {
            None
        };
        match row {
            Some((label_w, gap, min_field)) => {
                let field_w = ((avail - 3.0 * label_w - 2.0 * gap) / 3.0).max(min_field);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = vec2(gap, 0.0);
                    for (i, value) in [vx, vy, vz].into_iter().enumerate() {
                        ui.label(
                            egui::RichText::new(self.axis_names[i])
                                .strong()
                                .color(self.axis_colors[i]),
                        );
                        let ev = NumericField {
                            value,
                            opts: self.opts,
                            session_key: self.session_key,
                            undo_label: self.undo_label,
                            width: field_w,
                            height,
                        }
                        .show(ui, state);
                        out.changed[i] = ev.changed;
                        out.finished |= ev.finished;
                    }
                });
            }
            None => {
                for (i, value) in [vx, vy, vz].into_iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = vec2(6.0, 0.0);
                        ui.label(
                            egui::RichText::new(self.axis_names[i])
                                .strong()
                                .color(self.axis_colors[i]),
                        );
                        let field_w = (ui.available_width()).max(40.0);
                        let ev = NumericField {
                            value,
                            opts: self.opts,
                            session_key: self.session_key,
                            undo_label: self.undo_label,
                            width: field_w,
                            height,
                        }
                        .show(ui, state);
                        out.changed[i] = ev.changed;
                        out.finished |= ev.finished;
                    });
                }
            }
        }
        out
    }
}

// ------------------------------------------------------------------- seção

/// Seção leve do inspector (tipografia + separador + disclosure, sem cards).
///
/// Cabeçalho de altura `row_h` clicável por inteiro (teclado focável, anel de
/// foco, `widget_info`); `summary` aparece à direita quando colapsada
/// (ex. "8 vertices · 12 triangles"). Estado em `CollapsingState` persistente;
/// `force_open` (busca do inspector) abre sem gravar.
/// Opções da seção (título traduzido fora, resumo colapsado, abertura).
pub struct SectionOpts<'a> {
    pub title: &'a str,
    pub summary: Option<&'a str>,
    pub default_open: bool,
    pub force_open: bool,
}

pub fn section(
    ui: &mut Ui,
    density: UiDensity,
    id_salt: &str,
    opts: SectionOpts<'_>,
    body: impl FnOnce(&mut Ui),
) {
    let h = row_h(density);
    let row_w = ui.available_width().max(0.0);
    let id = ui.make_persistent_id(id_salt);
    let mut collapsed = egui::collapsing_header::CollapsingState::load_with_default_open(
        ui.ctx(),
        id,
        opts.default_open,
    );
    let is_open = opts.force_open || collapsed.is_open();

    let (rect, resp) = ui.allocate_exact_size(vec2(row_w, h), Sense::click());
    resp.widget_info(|| WidgetInfo::selected(WidgetType::Button, true, is_open, opts.title));
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        if resp.hovered() {
            painter.rect_filled(rect, tokens::RADIUS_SMALL, tokens::BG_SURFACE_HOVER);
        }
        if resp.has_focus() {
            painter.rect_stroke(
                rect,
                tokens::RADIUS_SMALL,
                tokens::stroke_focus(),
                StrokeKind::Inside,
            );
        }
        paint_chevron(
            painter,
            Rect::from_center_size(
                egui::pos2(rect.min.x + 11.0, rect.center().y),
                vec2(12.0, 12.0),
            ),
            tokens::TEXT_SECONDARY,
            if is_open {
                ChevronDir::Down
            } else {
                ChevronDir::Right
            },
        );
        painter.text(
            egui::pos2(rect.min.x + 24.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            opts.title.to_uppercase(),
            FontId::proportional(11.5),
            tokens::TEXT_PRIMARY,
        );
        if !is_open && let Some(summary) = opts.summary {
            painter.text(
                egui::pos2(rect.max.x - 6.0, rect.center().y),
                egui::Align2::RIGHT_CENTER,
                summary,
                FontId::proportional(10.5),
                tokens::TEXT_MUTED,
            );
        }
    }
    if resp.on_hover_text(opts.title).clicked() && !opts.force_open {
        collapsed.toggle(ui);
    }
    if is_open {
        ui.add_space(2.0);
        body(ui);
        ui.add_space(2.0);
    } else {
        collapsed.store(ui.ctx());
    }
}

// -------------------------------------------------------------------- abas

/// Abas textuais contextuais (texto > memorização de ícones).
///
/// Botões custom de largura igual, altura `row_h`, foco visível e semântica
/// de seleção. Retorna a aba clicada (teclado Enter/Espaço incluso).
pub fn context_tabs(
    ui: &mut Ui,
    density: UiDensity,
    tabs: &[(InspectorTab, String)],
    active: InspectorTab,
) -> Option<InspectorTab> {
    if tabs.is_empty() {
        return None;
    }
    let h = row_h(density);
    let gap = 4.0;
    let tab_w = ((ui.available_width() - gap * (tabs.len().saturating_sub(1)) as f32)
        / tabs.len() as f32)
        .max(0.0);
    let mut picked = None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(gap, 0.0);
        for (tab, label) in tabs {
            let selected = *tab == active;
            let (rect, resp) = ui.allocate_exact_size(vec2(tab_w, h), Sense::click());
            resp.widget_info(|| {
                WidgetInfo::selected(WidgetType::SelectableLabel, true, selected, label.clone())
            });
            if ui.is_rect_visible(rect) {
                let painter = ui.painter();
                let (fill, fg) = if selected {
                    (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
                } else if resp.hovered() {
                    (tokens::BG_SURFACE_HOVER, tokens::TEXT_PRIMARY)
                } else {
                    (tokens::BG_SURFACE, tokens::TEXT_SECONDARY)
                };
                painter.rect_filled(rect, tokens::RADIUS_CONTROL, fill);
                if resp.has_focus() {
                    painter.rect_stroke(
                        rect,
                        tokens::RADIUS_CONTROL,
                        tokens::stroke_focus(),
                        StrokeKind::Inside,
                    );
                }
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    label,
                    FontId::proportional(12.0),
                    fg,
                );
            }
            if resp.on_hover_text(label).clicked() {
                picked = Some(*tab);
            }
        }
    });
    picked
}

/// Linha de cabeçalho de bloco com ícone + título (cabeçalhos de contexto).
pub fn block_header(ui: &mut Ui, icon: PetuniaIcon, title: &str) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(6.0, 0.0);
        let (icon_rect, _) = ui.allocate_exact_size(vec2(16.0, 16.0), Sense::hover());
        IconRegistry::paint(
            ui.ctx(),
            ui.painter(),
            &icon,
            icon_rect,
            tokens::ACCENT_BLUE,
        );
        ui.label(egui::RichText::new(title).strong().size(12.5));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn density_tokens_grow_monotonically() {
        use UiDensity::{Comfortable, Compact, Spacious};
        for f in [row_h, hit_size, spacing_y, input_h, section_gap] {
            assert!(f(Compact) < f(Comfortable));
            assert!(f(Comfortable) < f(Spacious));
        }
        assert!(row_h(UiDensity::Compact) >= 28.0);
    }

    #[test]
    fn edit_session_checkpoints_once_per_gesture() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");
        let base_depth = state.project.undo.depth().0;
        let session = EditSession::new("test_gesture");
        ctx.run_ui(egui::RawInput::default(), |ui| {
            // Foco/teclado sem arrasto: primeira mudança carimba, resto não.
            let resp = ui.allocate_response(vec2(10.0, 10.0), Sense::click());
            assert!(session.poll(ui, &mut state, &resp, "test edit", true));
            assert!(session.poll(ui, &mut state, &resp, "test edit", true));
            session.finish(ui, true);
            // Nova sessão carimba de novo.
            assert!(session.poll(ui, &mut state, &resp, "test edit", true));
        })
        .textures_delta
        .clear();
        assert_eq!(state.project.undo.depth().0, base_depth + 2);
    }

    #[test]
    fn context_tabs_render_and_pick() {
        let ctx = egui::Context::default();
        let mut picked = None;
        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                picked = context_tabs(
                    ui,
                    UiDensity::Comfortable,
                    &[
                        (InspectorTab::Object, "Object".to_string()),
                        (InspectorTab::Modify, "Modify".to_string()),
                    ],
                    InspectorTab::Object,
                );
            });
        })
        .textures_delta
        .clear();
        // Sem clique, nada escolhido.
        assert_eq!(picked, None);
    }
}
