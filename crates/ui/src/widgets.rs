//! Componentes visuais canônicos do Petunia Design System (`PetuniaWidget`).
//!
//! Encapsula botões de toolbar, tabs de workspaces, tabs verticais de propriedades,
//! cabeçalhos de painel e caixas de busca com estados completos:
//! (Default, Hover, Pressed, Selected, Focused, Disabled).

use egui::{
    vec2, Align2, Color32, FontId, Rect, Response, Sense, StrokeKind, TextEdit, Ui, WidgetInfo,
    WidgetType,
};

use crate::icon_registry::{IconRegistry, PetuniaIcon};
use crate::tokens;

/// Botão de ferramenta da Toolbar do Petunia3D.
///
/// Suporta modos compacto (ícone centralizado) e expandido (ícone + rótulo com recorte limpo).
pub struct PetuniaToolbarButton<'a> {
    pub icon: PetuniaIcon,
    pub label: &'a str,
    pub selected: bool,
    pub compact: bool,
    pub tooltip: Option<&'a str>,
}

impl<'a> PetuniaToolbarButton<'a> {
    pub fn new(icon: PetuniaIcon, label: &'a str) -> Self {
        Self {
            icon,
            label,
            selected: false,
            compact: true,
            tooltip: None,
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn compact(mut self, compact: bool) -> Self {
        self.compact = compact;
        self
    }

    pub fn tooltip(mut self, tooltip: &'a str) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let desired_size = if self.compact {
            vec2(tokens::TOOLBAR_WIDTH, tokens::TOOLBAR_WIDTH)
        } else {
            vec2(ui.available_width().max(tokens::TOOLBAR_WIDTH), 32.0)
        };

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        response.widget_info(|| {
            WidgetInfo::selected(
                WidgetType::Button,
                ui.is_enabled(),
                self.selected,
                self.label,
            )
        });

        if ui.is_rect_visible(rect) {
            let (bg_fill, fg_color) = if self.selected {
                (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
            } else if response.hovered() {
                (tokens::BG_SURFACE_HOVER, tokens::TEXT_ACTIVE)
            } else {
                (Color32::TRANSPARENT, tokens::TEXT_SECONDARY)
            };

            let painter = ui.painter().with_clip_rect(rect);

            if bg_fill != Color32::TRANSPARENT {
                painter.rect_filled(rect, tokens::RADIUS_CONTAINER, bg_fill);
            }

            // Indicador de foco acessível
            if response.has_focus() {
                painter.rect_stroke(
                    rect,
                    tokens::RADIUS_CONTAINER,
                    tokens::stroke_focus(),
                    StrokeKind::Inside,
                );
            }

            if self.compact {
                // Ícone de 20x20 perfeitamente centralizado no container de 40x40
                let icon_rect = Rect::from_center_size(rect.center(), vec2(20.0, 20.0));
                IconRegistry::paint(ui.ctx(), &painter, &self.icon, icon_rect, fg_color);
            } else {
                // Ícone de 20x20 alinhado à esquerda + rótulo tipográfico à direita
                let icon_rect = Rect::from_min_size(
                    egui::pos2(rect.min.x + 8.0, rect.min.y + (rect.height() - 20.0) * 0.5),
                    vec2(20.0, 20.0),
                );
                IconRegistry::paint(ui.ctx(), &painter, &self.icon, icon_rect, fg_color);

                let text_pos =
                    egui::pos2(rect.min.x + 36.0, rect.min.y + (rect.height() - 14.0) * 0.5);
                painter.text(
                    text_pos,
                    Align2::LEFT_TOP,
                    self.label,
                    FontId::proportional(13.0),
                    fg_color,
                );
            }
        }

        if let Some(hint) = self.tooltip {
            response.on_hover_text(hint)
        } else {
            response
        }
    }
}

/// Botão de aba vertical de categoria do painel Properties (`PropertyTabButton`).
pub struct PetuniaPropertyTabButton {
    pub icon: PetuniaIcon,
    pub selected: bool,
    pub tooltip: Option<String>,
    pub accent_color: Color32,
}

impl PetuniaPropertyTabButton {
    pub fn new(icon: PetuniaIcon, selected: bool) -> Self {
        Self {
            icon,
            selected,
            tooltip: None,
            accent_color: tokens::ACCENT_BLUE,
        }
    }

    pub fn accent_color(mut self, color: Color32) -> Self {
        self.accent_color = color;
        self
    }

    pub fn tooltip(mut self, hint: impl Into<String>) -> Self {
        self.tooltip = Some(hint.into());
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let desired_size = vec2(32.0, 28.0);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        if ui.is_rect_visible(rect) {
            let (bg_fill, fg_color) = if self.selected {
                (self.accent_color.gamma_multiply(0.35), Color32::WHITE)
            } else if response.hovered() {
                (self.accent_color.gamma_multiply(0.20), Color32::WHITE)
            } else {
                (Color32::TRANSPARENT, self.accent_color)
            };

            let painter = ui.painter().with_clip_rect(rect);

            if bg_fill != Color32::TRANSPARENT {
                painter.rect_filled(rect, tokens::RADIUS_CONTAINER, bg_fill);
            }

            if self.selected {
                // Indicador inferior elegante para aba ativa
                painter.line_segment(
                    [
                        egui::pos2(rect.left() + 4.0, rect.bottom() - 1.5),
                        egui::pos2(rect.right() - 4.0, rect.bottom() - 1.5),
                    ],
                    egui::Stroke::new(2.5_f32, self.accent_color),
                );
                // Borda sutil de destaque
                painter.rect_stroke(
                    rect,
                    tokens::RADIUS_CONTAINER,
                    egui::Stroke::new(1.0_f32, self.accent_color.gamma_multiply(0.6)),
                    StrokeKind::Inside,
                );
            }

            if response.has_focus() {
                painter.rect_stroke(
                    rect,
                    tokens::RADIUS_CONTAINER,
                    tokens::stroke_focus(),
                    StrokeKind::Inside,
                );
            }

            let icon_rect = Rect::from_center_size(
                rect.center() - vec2(0.0, if self.selected { 1.0 } else { 0.0 }),
                vec2(19.0, 19.0),
            );
            IconRegistry::paint(ui.ctx(), &painter, &self.icon, icon_rect, fg_color);
        }

        if let Some(hint) = self.tooltip {
            response.on_hover_text(hint)
        } else {
            response
        }
    }
}

/// Aba em formato de pílula arredondada do cabeçalho superior (`WorkspacePill`).
pub struct PetuniaWorkspacePill<'a> {
    pub label: &'a str,
    pub selected: bool,
}

impl<'a> PetuniaWorkspacePill<'a> {
    pub fn new(label: &'a str, selected: bool) -> Self {
        Self { label, selected }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let (bg, fg) = if self.selected {
            (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
        } else {
            (Color32::TRANSPARENT, tokens::TEXT_SECONDARY)
        };

        let btn = egui::Button::new(
            egui::RichText::new(self.label)
                .size(11.5)
                .color(fg)
                .strong(),
        )
        .fill(bg)
        .corner_radius(tokens::RADIUS_PILL);

        ui.add(btn)
    }
}

/// Campo de busca padronizado com ícone Phosphor de lupa e botão de limpar.
pub fn petunia_search_box(ui: &mut Ui, query: &mut String, hint: &str) -> Response {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(4.0, 0.0);

        let search_rect =
            Rect::from_center_size(ui.cursor().min + vec2(8.0, 10.0), vec2(16.0, 16.0));
        IconRegistry::paint(
            ui.ctx(),
            ui.painter(),
            &PetuniaIcon::Search,
            search_rect,
            tokens::TEXT_MUTED,
        );

        let edit = TextEdit::singleline(query)
            .hint_text(hint)
            .desired_width(ui.available_width().max(80.0));

        let resp = ui.add(edit);

        if !query.is_empty() {
            let clear_btn = egui::Button::new("×")
                .frame(false)
                .fill(Color32::TRANSPARENT);
            if ui.add(clear_btn).clicked() {
                query.clear();
            }
        }

        resp
    })
    .inner
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::Context;

    #[test]
    fn test_toolbar_button_widget_rendering() {
        let ctx = Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let btn = PetuniaToolbarButton::new(PetuniaIcon::Move, "Move")
                    .selected(true)
                    .compact(true)
                    .tooltip("Transladar · G");
                let _resp = btn.show(ui);

                let btn_wide = PetuniaToolbarButton::new(PetuniaIcon::Rotate, "Rotate")
                    .selected(false)
                    .compact(false);
                let _resp_wide = btn_wide.show(ui);
            });
        });
    }

    #[test]
    fn test_property_tab_button_widget_rendering() {
        let ctx = Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let tab = PetuniaPropertyTabButton::new(PetuniaIcon::PropRender, true)
                    .tooltip("Propriedades de Render");
                let _resp = tab.show(ui);
            });
        });
    }

    #[test]
    fn test_workspace_pill_widget_rendering() {
        let ctx = Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let pill = PetuniaWorkspacePill::new("MODEL", true);
                let _resp = pill.show(ui);
            });
        });
    }

    #[test]
    fn test_search_box_widget_rendering() {
        let ctx = Context::default();
        let mut query = String::from("Cube");
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _resp = petunia_search_box(ui, &mut query, "Search Scene...");
            });
        });
    }
}
