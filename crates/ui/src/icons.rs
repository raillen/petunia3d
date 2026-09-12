//! Vetores de ícones e botões de ferramentas para a UI do Petunia3D (egui).
//! Grade lógica de 24 pontos sem dependências externas ou caracteres Unicode.

use egui::{vec2, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, WidgetInfo, WidgetType};

/// Renderiza um ícone vetorial numa grade lógica de 24 pontos dentro do retângulo especificado.
pub fn paint(painter: &egui::Painter, id: &str, target_rect: Rect, color: Color32) {
    let stroke = Stroke::new(1.5_f32, color);
    let fill = color;

    // Calcula escala e centraliza a grade 24x24 dentro de target_rect
    let side = target_rect.width().min(target_rect.height()).min(24.0);
    let min_x = target_rect.min.x + (target_rect.width() - side) * 0.5;
    let min_y = target_rect.min.y + (target_rect.height() - side) * 0.5;
    let grid_rect = Rect::from_min_size(Pos2::new(min_x, min_y), vec2(side, side));

    let p = |gx: f32, gy: f32| -> Pos2 {
        Pos2::new(
            grid_rect.min.x + (gx / 24.0) * grid_rect.width(),
            grid_rect.min.y + (gy / 24.0) * grid_rect.height(),
        )
    };

    match id {
        "select" => {
            // Seta de seleção (cursor)
            let points = vec![
                p(4.0, 4.0),
                p(4.0, 19.0),
                p(8.5, 15.0),
                p(12.5, 21.0),
                p(15.0, 19.5),
                p(11.0, 13.5),
                p(17.0, 13.5),
            ];
            painter.add(egui::Shape::convex_polygon(
                points,
                fill.linear_multiply(0.25),
                stroke,
            ));
        }
        "transform" => {
            // Quatro setas cardeais (Move/Translate)
            // Cruz central
            painter.line_segment([p(12.0, 3.0), p(12.0, 21.0)], stroke);
            painter.line_segment([p(3.0, 12.0), p(21.0, 12.0)], stroke);
            // Pontas
            painter.line_segment([p(9.5, 5.5), p(12.0, 3.0)], stroke);
            painter.line_segment([p(14.5, 5.5), p(12.0, 3.0)], stroke);

            painter.line_segment([p(9.5, 18.5), p(12.0, 21.0)], stroke);
            painter.line_segment([p(14.5, 18.5), p(12.0, 21.0)], stroke);

            painter.line_segment([p(5.5, 9.5), p(3.0, 12.0)], stroke);
            painter.line_segment([p(5.5, 14.5), p(3.0, 12.0)], stroke);

            painter.line_segment([p(18.5, 9.5), p(21.0, 12.0)], stroke);
            painter.line_segment([p(18.5, 14.5), p(21.0, 12.0)], stroke);
        }
        "rotate" => {
            // Arco circular com ponta de seta
            let center = p(12.0, 12.0);
            let radius = side * 0.35;
            let mut arc_pts = Vec::new();
            for i in 0..=24 {
                let angle =
                    -std::f32::consts::FRAC_PI_2 + (i as f32 / 24.0) * (std::f32::consts::PI * 1.5);
                arc_pts.push(Pos2::new(
                    center.x + angle.cos() * radius,
                    center.y + angle.sin() * radius,
                ));
            }
            for w in arc_pts.windows(2) {
                painter.line_segment([w[0], w[1]], stroke);
            }
            // Seta no final do arco
            if let Some(&tip) = arc_pts.last() {
                painter.line_segment([tip + vec2(-4.0, -1.0), tip], stroke);
                painter.line_segment([tip + vec2(-1.0, 4.0), tip], stroke);
            }
        }
        "scale" => {
            // Quadrado pequeno na base e expansão diagonal para quadrado maior
            let s_box = Rect::from_min_max(p(4.0, 14.0), p(10.0, 20.0));
            painter.rect_stroke(s_box, 1.0_f32, stroke, egui::StrokeKind::Inside);
            let b_box = Rect::from_min_max(p(14.0, 4.0), p(20.0, 10.0));
            painter.rect_stroke(b_box, 1.0_f32, stroke, egui::StrokeKind::Inside);
            // Seta diagonal
            painter.line_segment([p(10.0, 14.0), p(14.0, 10.0)], stroke);
            painter.line_segment([p(11.5, 10.0), p(14.0, 10.0)], stroke);
            painter.line_segment([p(14.0, 12.5), p(14.0, 10.0)], stroke);
        }
        "primitives" => {
            // Cubo isométrico
            let top = p(12.0, 3.0);
            let top_left = p(4.0, 7.5);
            let top_right = p(20.0, 7.5);
            let center = p(12.0, 12.0);
            let bot_left = p(4.0, 16.5);
            let bot_right = p(20.0, 16.5);
            let bot = p(12.0, 21.0);

            painter.line_segment([top, top_left], stroke);
            painter.line_segment([top, top_right], stroke);
            painter.line_segment([top_left, bot_left], stroke);
            painter.line_segment([top_right, bot_right], stroke);
            painter.line_segment([bot_left, bot], stroke);
            painter.line_segment([bot_right, bot], stroke);

            painter.line_segment([center, top], stroke);
            painter.line_segment([center, bot_left], stroke);
            painter.line_segment([center, bot_right], stroke);
        }
        "extrude" => {
            // Face estendida para cima com setas laterais
            painter.line_segment([p(4.0, 18.0), p(20.0, 18.0)], stroke);
            painter.line_segment([p(4.0, 10.0), p(20.0, 10.0)], stroke);
            painter.line_segment([p(4.0, 18.0), p(4.0, 10.0)], stroke);
            painter.line_segment([p(20.0, 18.0), p(20.0, 10.0)], stroke);

            // Seta central para cima
            painter.line_segment([p(12.0, 14.0), p(12.0, 4.0)], stroke);
            painter.line_segment([p(9.0, 7.0), p(12.0, 4.0)], stroke);
            painter.line_segment([p(15.0, 7.0), p(12.0, 4.0)], stroke);
        }
        "inset" => {
            // Quadrado externo e quadrado concêntrico interno com linhas diagonais
            let out_r = Rect::from_min_max(p(3.0, 3.0), p(21.0, 21.0));
            painter.rect_stroke(out_r, 1.0_f32, stroke, egui::StrokeKind::Inside);
            let in_r = Rect::from_min_max(p(8.0, 8.0), p(16.0, 16.0));
            painter.rect_stroke(in_r, 1.0_f32, stroke, egui::StrokeKind::Inside);

            painter.line_segment([p(3.0, 3.0), p(8.0, 8.0)], stroke);
            painter.line_segment([p(21.0, 3.0), p(16.0, 8.0)], stroke);
            painter.line_segment([p(3.0, 21.0), p(8.0, 16.0)], stroke);
            painter.line_segment([p(21.0, 21.0), p(16.0, 16.0)], stroke);
        }
        "bevel" => {
            // Quina com corte em chanfro / bisel
            painter.line_segment([p(4.0, 4.0), p(14.0, 4.0)], stroke);
            painter.line_segment([p(14.0, 4.0), p(20.0, 10.0)], stroke); // chanfro
            painter.line_segment([p(20.0, 10.0), p(20.0, 20.0)], stroke);
            painter.line_segment([p(20.0, 20.0), p(4.0, 20.0)], stroke);
            painter.line_segment([p(4.0, 20.0), p(4.0, 4.0)], stroke);

            // Linha secundária mostrando a faceta chanfrada
            let dotted_stroke = Stroke::new(1.0_f32, color.linear_multiply(0.6));
            painter.line_segment([p(14.0, 4.0), p(14.0, 10.0)], dotted_stroke);
            painter.line_segment([p(14.0, 10.0), p(20.0, 10.0)], dotted_stroke);
        }
        "pushpull" => {
            // Superfície plana com vetor normal perpendicular bidirecional
            painter.line_segment([p(4.0, 16.0), p(20.0, 16.0)], stroke);
            painter.line_segment([p(12.0, 16.0), p(12.0, 4.0)], stroke);
            painter.line_segment([p(9.0, 7.0), p(12.0, 4.0)], stroke);
            painter.line_segment([p(15.0, 7.0), p(12.0, 4.0)], stroke);
            // Seta oposta (baixo)
            painter.line_segment([p(12.0, 16.0), p(12.0, 21.0)], stroke);
            painter.line_segment([p(10.0, 19.0), p(12.0, 21.0)], stroke);
            painter.line_segment([p(14.0, 19.0), p(12.0, 21.0)], stroke);
        }
        "loop_cut" => {
            // Anel de corte dividindo o retângulo ao meio
            let outer = Rect::from_min_max(p(4.0, 4.0), p(20.0, 20.0));
            painter.rect_stroke(outer, 1.0_f32, stroke, egui::StrokeKind::Inside);
            // Linha de loop no meio
            let loop_stroke = Stroke::new(2.0_f32, color);
            painter.line_segment([p(3.0, 12.0), p(21.0, 12.0)], loop_stroke);
            painter.circle_filled(p(12.0, 12.0), 2.0, color);
        }
        "knife" => {
            // Lâmina de corte diagonal com cabo
            // Lâmina
            painter.line_segment([p(5.0, 19.0), p(12.0, 12.0)], stroke);
            painter.line_segment([p(5.0, 19.0), p(9.0, 19.0)], stroke);
            painter.line_segment([p(9.0, 19.0), p(14.0, 14.0)], stroke);
            // Cabo
            painter.line_segment([p(12.0, 12.0), p(19.0, 5.0)], Stroke::new(2.5_f32, color));
        }
        "slice" => {
            // Plano fatiando um quadrado
            let cube = Rect::from_min_max(p(6.0, 6.0), p(18.0, 18.0));
            painter.rect_stroke(cube, 1.0_f32, stroke, egui::StrokeKind::Inside);
            // Linha de corte cortando diagonalmente
            let cut_stroke = Stroke::new(2.0_f32, color);
            painter.line_segment([p(2.0, 19.0), p(22.0, 5.0)], cut_stroke);
        }
        "subdivide" => {
            // Quadrado particionado em 4 (cruz interna)
            let box_rect = Rect::from_min_max(p(4.0, 4.0), p(20.0, 20.0));
            painter.rect_stroke(box_rect, 1.0_f32, stroke, egui::StrokeKind::Inside);
            painter.line_segment([p(12.0, 4.0), p(12.0, 20.0)], stroke);
            painter.line_segment([p(4.0, 12.0), p(20.0, 12.0)], stroke);
        }
        "draw_profile" => {
            // Curva com pontos de controle (caneta/perfil)
            painter.line_segment([p(4.0, 18.0), p(10.0, 6.0)], stroke);
            painter.line_segment([p(10.0, 6.0), p(16.0, 14.0)], stroke);
            painter.line_segment([p(16.0, 14.0), p(20.0, 8.0)], stroke);
            painter.circle_filled(p(4.0, 18.0), 2.0, color);
            painter.circle_filled(p(10.0, 6.0), 2.0, color);
            painter.circle_filled(p(16.0, 14.0), 2.0, color);
            painter.circle_filled(p(20.0, 8.0), 2.0, color);
        }
        "connect" => {
            // Dois vértices com aresta de conexão
            painter.circle_filled(p(6.0, 18.0), 2.5, color);
            painter.circle_filled(p(18.0, 6.0), 2.5, color);
            painter.line_segment([p(6.0, 18.0), p(18.0, 6.0)], Stroke::new(2.0_f32, color));
        }
        "dissolve" => {
            // Aresta com indicador de remoção
            painter.line_segment([p(5.0, 19.0), p(19.0, 5.0)], stroke);
            // X de dissolver no meio
            painter.line_segment([p(9.0, 9.0), p(15.0, 15.0)], Stroke::new(2.0_f32, color));
            painter.line_segment([p(15.0, 9.0), p(9.0, 15.0)], Stroke::new(2.0_f32, color));
        }
        "mirror" => {
            // Linha de simetria com triângulos espelhados
            painter.line_segment(
                [p(12.0, 3.0), p(12.0, 21.0)],
                Stroke::new(1.0_f32, color.linear_multiply(0.7)),
            );
            // Triângulo esquerdo
            painter.line_segment([p(11.0, 6.0), p(4.0, 14.0)], stroke);
            painter.line_segment([p(4.0, 14.0), p(11.0, 18.0)], stroke);
            painter.line_segment([p(11.0, 18.0), p(11.0, 6.0)], stroke);
            // Triângulo direito (espelho)
            painter.line_segment([p(13.0, 6.0), p(20.0, 14.0)], stroke);
            painter.line_segment([p(20.0, 14.0), p(13.0, 18.0)], stroke);
            painter.line_segment([p(13.0, 18.0), p(13.0, 6.0)], stroke);
        }
        "merge" => {
            // Dois vértices convergindo com setas para um ponto central
            let center = p(12.0, 12.0);
            painter.circle_filled(center, 3.0, color);
            painter.line_segment([p(4.0, 6.0), p(10.0, 10.0)], stroke);
            painter.line_segment([p(20.0, 6.0), p(14.0, 10.0)], stroke);
            painter.line_segment([p(4.0, 18.0), p(10.0, 14.0)], stroke);
            painter.line_segment([p(20.0, 18.0), p(14.0, 14.0)], stroke);
        }
        "paint" => {
            // Pincel de pintura
            painter.line_segment([p(6.0, 18.0), p(14.0, 10.0)], Stroke::new(2.0_f32, color));
            painter.line_segment([p(14.0, 10.0), p(19.0, 5.0)], Stroke::new(3.0_f32, color));
            // Cerdas / ponta do pincel
            painter.circle_filled(p(5.0, 19.0), 2.5, color);
        }
        "camera" => {
            // Câmera clássica
            let body = Rect::from_min_max(p(4.0, 9.0), p(20.0, 19.0));
            painter.rect_stroke(body, 2.0_f32, stroke, egui::StrokeKind::Inside);
            // Lente
            painter.circle_stroke(p(12.0, 14.0), 3.0, stroke);
            // Visor / topo
            painter.line_segment([p(8.0, 9.0), p(10.0, 6.0)], stroke);
            painter.line_segment([p(10.0, 6.0), p(14.0, 6.0)], stroke);
            painter.line_segment([p(14.0, 6.0), p(16.0, 9.0)], stroke);
        }
        "undo" => {
            // Seta curvada para a esquerda
            let center = p(13.0, 13.0);
            let radius = side * 0.32;
            let mut arc_pts = Vec::new();
            for i in 0..=16 {
                let angle = (i as f32 / 16.0) * std::f32::consts::PI;
                arc_pts.push(Pos2::new(
                    center.x + angle.cos() * radius,
                    center.y - angle.sin() * radius,
                ));
            }
            for w in arc_pts.windows(2) {
                painter.line_segment([w[0], w[1]], stroke);
            }
            if let Some(&tip) = arc_pts.first() {
                painter.line_segment([tip + vec2(0.0, -4.0), tip], stroke);
                painter.line_segment([tip + vec2(4.0, -1.0), tip], stroke);
            }
        }
        "redo" => {
            // Seta curvada para a direita
            let center = p(11.0, 13.0);
            let radius = side * 0.32;
            let mut arc_pts = Vec::new();
            for i in 0..=16 {
                let angle = (i as f32 / 16.0) * std::f32::consts::PI;
                arc_pts.push(Pos2::new(
                    center.x - angle.cos() * radius,
                    center.y - angle.sin() * radius,
                ));
            }
            for w in arc_pts.windows(2) {
                painter.line_segment([w[0], w[1]], stroke);
            }
            if let Some(&tip) = arc_pts.first() {
                painter.line_segment([tip + vec2(0.0, -4.0), tip], stroke);
                painter.line_segment([tip + vec2(-4.0, -1.0), tip], stroke);
            }
        }
        _ => {
            // Fallback genérico: losango geométrico
            let top = p(12.0, 4.0);
            let right = p(20.0, 12.0);
            let bottom = p(12.0, 20.0);
            let left = p(4.0, 12.0);
            painter.line_segment([top, right], stroke);
            painter.line_segment([right, bottom], stroke);
            painter.line_segment([bottom, left], stroke);
            painter.line_segment([left, top], stroke);
        }
    }
}

/// Botão de ferramenta com suporte a layout compacto (40x40) ou expandido com rótulo.
/// Respeita estados de hover, ativo, selecionado e foco de teclado com marcador lateral.
pub fn tool_button(ui: &mut Ui, id: &str, label: &str, selected: bool, compact: bool) -> Response {
    let desired_size = if compact {
        vec2(40.0, 40.0)
    } else {
        vec2(ui.available_width().max(136.0), 34.0)
    };

    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    response
        .widget_info(|| WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), selected, label));

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().visuals.clone();

        // Determina cor de fundo
        let bg_fill = if selected {
            visuals.selection.bg_fill
        } else if response.hovered() {
            visuals.widgets.hovered.bg_fill
        } else {
            Color32::TRANSPARENT
        };

        if bg_fill != Color32::TRANSPARENT {
            ui.painter().rect_filled(rect, 4.0, bg_fill);
        }

        // Marcador lateral para indicar seleção
        if selected {
            let marker_rect = Rect::from_min_size(
                Pos2::new(rect.min.x + 2.0, rect.min.y + 6.0),
                vec2(3.0, (rect.height() - 12.0).max(4.0)),
            );
            ui.painter()
                .rect_filled(marker_rect, 1.5, visuals.hyperlink_color);
        }

        // Borda de foco visual acessível
        if response.has_focus() {
            ui.painter().rect_stroke(
                rect,
                4.0,
                Stroke::new(1.5_f32, visuals.hyperlink_color),
                egui::StrokeKind::Inside,
            );
        }

        // Cor do ícone e texto
        let fg_color = if !ui.is_enabled() {
            visuals.widgets.noninteractive.fg_stroke.color
        } else if selected {
            visuals.selection.stroke.color
        } else if response.hovered() {
            visuals.widgets.hovered.fg_stroke.color
        } else {
            visuals.widgets.inactive.fg_stroke.color
        };

        if compact {
            // Centraliza o ícone 24x24 no botão 40x40
            let icon_rect = Rect::from_center_size(rect.center(), vec2(24.0, 24.0));
            paint(ui.painter(), id, icon_rect, fg_color);
        } else {
            // Ícone na esquerda + texto alinhado
            let icon_rect = Rect::from_min_size(
                Pos2::new(rect.min.x + 8.0, rect.min.y + (rect.height() - 22.0) * 0.5),
                vec2(22.0, 22.0),
            );
            paint(ui.painter(), id, icon_rect, fg_color);

            let text_pos = Pos2::new(rect.min.x + 36.0, rect.min.y + (rect.height() - 14.0) * 0.5);
            ui.painter().text(
                text_pos,
                egui::Align2::LEFT_TOP,
                label,
                egui::FontId::proportional(13.5),
                fg_color,
            );
        }
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_tool_icons_paint_without_panic() {
        let tools = [
            "select",
            "transform",
            "rotate",
            "scale",
            "primitives",
            "extrude",
            "inset",
            "bevel",
            "pushpull",
            "loop_cut",
            "knife",
            "slice",
            "subdivide",
            "draw_profile",
            "connect",
            "dissolve",
            "mirror",
            "merge",
            "paint",
            "camera",
            "undo",
            "redo",
            "unknown_tool",
        ];

        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let r = Rect::from_min_size(Pos2::ZERO, vec2(24.0, 24.0));
                for id in tools {
                    paint(ui.painter(), id, r, Color32::WHITE);
                }
            });
        });
    }

    #[test]
    fn test_tool_button_compact_and_wide_rendering() {
        let ctx = egui::Context::default();
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let resp_compact = tool_button(ui, "select", "Selecionar", true, true);
                assert_eq!(resp_compact.rect.width(), 40.0);
                assert_eq!(resp_compact.rect.height(), 40.0);

                let resp_wide = tool_button(ui, "extrude", "Extrusão", false, false);
                assert!(resp_wide.rect.width() >= 136.0);
                assert_eq!(resp_wide.rect.height(), 34.0);
            });
        });
    }
}
