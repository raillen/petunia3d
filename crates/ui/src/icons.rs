//! Vetores de ícones e botões de ferramentas para a UI do Petunia3D (egui).
//! Grade lógica de 24 pontos sem dependências externas ou caracteres Unicode.

use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, WidgetInfo, WidgetType, vec2};

/// Renderiza um ícone vetorial numa grade lógica de 24 pontos dentro do retângulo especificado.
pub fn paint(painter: &egui::Painter, id: &str, target_rect: Rect, color: Color32) {
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

    let alpha_factor = (color.a() as f32 / 255.0).clamp(0.2, 1.0);
    let modulate = |c: Color32| -> Color32 { c.linear_multiply(alpha_factor) };

    let c_red = modulate(Color32::from_rgb(240, 75, 75));
    let c_green = modulate(Color32::from_rgb(76, 175, 80));
    let c_blue = modulate(Color32::from_rgb(66, 165, 245));
    let c_yellow = modulate(Color32::from_rgb(255, 215, 64));
    let c_orange = modulate(Color32::from_rgb(255, 152, 0));
    let c_cyan = modulate(Color32::from_rgb(38, 198, 218));
    let c_white = modulate(Color32::from_rgb(245, 245, 245));
    let c_dark = modulate(Color32::from_rgb(33, 33, 33));
    let c_neutral = modulate(Color32::from_rgb(180, 190, 200));

    match id {
        "select_box" => {
            // Caixa de seleção retangular pontilhada/contornada em azul ciano com cursor seta
            let box_rect = Rect::from_min_max(p(4.0, 4.0), p(20.0, 18.0));
            painter.rect_filled(box_rect, 1.5, c_blue.linear_multiply(0.2));
            painter.rect_stroke(
                box_rect,
                1.5,
                Stroke::new(2.0_f32, c_blue),
                egui::StrokeKind::Inside,
            );

            // Seta ponteiro branca com contorno escuro
            let points = vec![
                p(2.0, 2.0),
                p(2.0, 13.0),
                p(5.0, 10.5),
                p(8.0, 16.0),
                p(10.5, 14.5),
                p(7.5, 9.5),
                p(12.0, 9.5),
            ];
            painter.add(egui::Shape::convex_polygon(
                points,
                c_white,
                Stroke::new(1.8_f32, c_dark),
            ));
        }
        "select" => {
            // Seta de seleção (cursor) clássica com contorno reforçado
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
                c_white,
                Stroke::new(2.0_f32, c_dark),
            ));
        }
        "cursor_3d" => {
            // 3D Cursor canônico do Blender: anel vermelho com segmentos brancos e mira vazada
            let center = p(12.0, 12.0);
            let radius = side * 0.28;
            painter.circle_stroke(center, radius, Stroke::new(1.8_f32, c_red));

            // Segmentos tracejados brancos do anel (4 quadrantes)
            for i in [1, 3, 5, 7] {
                let start_ang = (i as f32) * std::f32::consts::FRAC_PI_4 - 0.22;
                let end_ang = (i as f32) * std::f32::consts::FRAC_PI_4 + 0.22;
                let p1 = Pos2::new(
                    center.x + start_ang.cos() * radius,
                    center.y + start_ang.sin() * radius,
                );
                let p2 = Pos2::new(
                    center.x + end_ang.cos() * radius,
                    center.y + end_ang.sin() * radius,
                );
                painter.line_segment([p1, p2], Stroke::new(1.9_f32, c_white));
            }

            // Mira externa vazada
            let cross_stroke = Stroke::new(1.8_f32, c_red);
            painter.line_segment([p(12.0, 1.5), p(12.0, 4.5)], cross_stroke);
            painter.line_segment([p(12.0, 19.5), p(12.0, 22.5)], cross_stroke);
            painter.line_segment([p(1.5, 12.0), p(4.5, 12.0)], cross_stroke);
            painter.line_segment([p(19.5, 12.0), p(22.5, 12.0)], cross_stroke);

            // Pontinhos de mira brancos
            painter.circle_filled(p(12.0, 2.0), 1.0, c_white);
            painter.circle_filled(p(12.0, 22.0), 1.0, c_white);
            painter.circle_filled(p(2.0, 12.0), 1.0, c_white);
            painter.circle_filled(p(22.0, 12.0), 1.0, c_white);

            // Ponto central sutil
            painter.circle_filled(center, 1.2, c_red);
        }
        "move" => {
            // Gizmo de Translação RGB do Blender (X=Vermelho, Y=Verde, Z=Azul com pontas sólidas)
            let origin = p(6.5, 17.5);

            // Eixo Z (Azul - para cima)
            let z_tip = p(6.5, 3.5);
            painter.line_segment([origin, p(6.5, 7.5)], Stroke::new(1.9_f32, c_blue));
            painter.add(egui::Shape::convex_polygon(
                vec![p(4.0, 8.0), p(9.0, 8.0), z_tip],
                c_blue,
                Stroke::NONE,
            ));

            // Eixo X (Vermelho - para a direita)
            let x_tip = p(20.5, 17.5);
            painter.line_segment([origin, p(16.5, 17.5)], Stroke::new(1.9_f32, c_red));
            painter.add(egui::Shape::convex_polygon(
                vec![p(16.0, 15.0), p(16.0, 20.0), x_tip],
                c_red,
                Stroke::NONE,
            ));

            // Eixo Y (Verde - perspectiva isométrica / diagonal superior)
            let y_tip = p(18.5, 8.0);
            painter.line_segment([origin, p(14.5, 10.5)], Stroke::new(1.9_f32, c_green));
            painter.add(egui::Shape::convex_polygon(
                vec![p(12.5, 12.5), p(16.5, 8.0), y_tip],
                c_green,
                Stroke::NONE,
            ));

            // Origem central
            painter.circle_filled(origin, 2.4, c_white);
        }
        "rotate" => {
            // Gizmo de Rotação RGB do Blender (Arcos cardeais circulares nas 3 dimensões)
            let center = p(12.0, 12.0);
            let radius = side * 0.38;

            // Anel externo Z (Azul) com seta de rotação
            let mut z_pts = Vec::new();
            for i in 0..=18 {
                let ang =
                    -std::f32::consts::FRAC_PI_2 + (i as f32 / 18.0) * (std::f32::consts::PI * 1.4);
                z_pts.push(Pos2::new(
                    center.x + ang.cos() * radius,
                    center.y + ang.sin() * radius,
                ));
            }
            for w in z_pts.windows(2) {
                painter.line_segment([w[0], w[1]], Stroke::new(1.8_f32, c_blue));
            }
            if let Some(&tip) = z_pts.last() {
                painter.add(egui::Shape::convex_polygon(
                    vec![tip + vec2(-4.0, -1.0), tip + vec2(-1.0, 4.0), tip],
                    c_blue,
                    Stroke::NONE,
                ));
            }

            // Arco elíptico X (Vermelho)
            let mut x_pts = Vec::new();
            for i in 0..=12 {
                let ang = -0.7 + (i as f32 / 12.0) * 1.4;
                x_pts.push(Pos2::new(
                    center.x + ang.cos() * radius * 0.7,
                    center.y + ang.sin() * radius * 0.35,
                ));
            }
            for w in x_pts.windows(2) {
                painter.line_segment([w[0], w[1]], Stroke::new(1.9_f32, c_red));
            }

            // Arco elíptico Y (Verde)
            let mut y_pts = Vec::new();
            for i in 0..=12 {
                let ang = -0.7 + (i as f32 / 12.0) * 1.4;
                y_pts.push(Pos2::new(
                    center.x + ang.sin() * radius * 0.35,
                    center.y + ang.cos() * radius * 0.7,
                ));
            }
            for w in y_pts.windows(2) {
                painter.line_segment([w[0], w[1]], Stroke::new(1.9_f32, c_green));
            }
        }
        "scale" => {
            // Gizmo de Escala RGB do Blender (Eixos terminados em cubos preenchidos sólidos)
            let origin = p(6.5, 17.5);

            // Eixo Z (Azul - cubo no topo)
            let z_cube = Rect::from_center_size(p(6.5, 4.5), vec2(5.5, 5.5));
            painter.line_segment([origin, p(6.5, 7.5)], Stroke::new(1.9_f32, c_blue));
            painter.rect_filled(z_cube, 1.0, c_blue);

            // Eixo X (Vermelho - cubo na direita)
            let x_cube = Rect::from_center_size(p(18.5, 17.5), vec2(5.5, 5.5));
            painter.line_segment([origin, p(15.5, 17.5)], Stroke::new(1.9_f32, c_red));
            painter.rect_filled(x_cube, 1.0, c_red);

            // Eixo Y (Verde - cubo na diagonal)
            let y_cube = Rect::from_center_size(p(17.5, 8.0), vec2(5.5, 5.5));
            painter.line_segment([origin, p(15.0, 10.5)], Stroke::new(1.9_f32, c_green));
            painter.rect_filled(y_cube, 1.0, c_green);

            // Origem central branca
            let o_cube = Rect::from_center_size(origin, vec2(4.5, 4.5));
            painter.rect_filled(o_cube, 0.8, c_white);
        }
        "transform" => {
            // Gizmo Combinado do Blender (Translação + Rotação + Escala)
            let origin = p(6.5, 17.5);

            // Eixo Z: Seta de translação azul
            let z_tip = p(6.5, 3.5);
            painter.line_segment([origin, p(6.5, 7.5)], Stroke::new(1.9_f32, c_blue));
            painter.add(egui::Shape::convex_polygon(
                vec![p(4.0, 8.0), p(9.0, 8.0), z_tip],
                c_blue,
                Stroke::NONE,
            ));

            // Eixo X: Cubo de escala vermelho
            let x_cube = Rect::from_center_size(p(18.5, 17.5), vec2(5.5, 5.5));
            painter.line_segment([origin, p(15.5, 17.5)], Stroke::new(1.9_f32, c_red));
            painter.rect_filled(x_cube, 1.0, c_red);

            // Eixo Y: Seta verde
            let y_tip = p(18.5, 8.0);
            painter.line_segment([origin, p(14.5, 10.5)], Stroke::new(1.9_f32, c_green));
            painter.add(egui::Shape::convex_polygon(
                vec![p(12.5, 12.5), p(16.5, 8.0), y_tip],
                c_green,
                Stroke::NONE,
            ));

            // Arco de rotação amarelo
            let arc_stroke = Stroke::new(1.7_f32, c_yellow);
            painter.line_segment([p(11.0, 11.5), p(12.5, 14.5)], arc_stroke);
            painter.line_segment([p(12.5, 14.5), p(15.0, 15.0)], arc_stroke);

            // Centro
            painter.circle_filled(origin, 2.4, c_white);
        }
        "annotate" => {
            // Lápis Grease Pencil com corpo ciano, madeira amarela e ponta de grafite
            let p_body = vec![p(9.5, 11.5), p(16.5, 4.5), p(19.5, 7.5), p(12.5, 14.5)];
            painter.add(egui::Shape::convex_polygon(
                p_body,
                c_cyan,
                Stroke::new(1.6_f32, c_cyan.linear_multiply(0.8)),
            ));

            let p_wood = vec![p(9.5, 11.5), p(12.5, 14.5), p(6.0, 18.0)];
            painter.add(egui::Shape::convex_polygon(p_wood, c_yellow, Stroke::NONE));

            let p_lead = vec![p(8.0, 16.0), p(9.5, 17.5), p(6.0, 18.0)];
            painter.add(egui::Shape::convex_polygon(p_lead, c_dark, Stroke::NONE));

            let stroke_line = Stroke::new(1.9_f32, c_cyan);
            painter.line_segment([p(3.0, 20.0), p(7.0, 21.5)], stroke_line);
            painter.line_segment([p(7.0, 21.5), p(13.0, 20.0)], stroke_line);
        }
        "measure" => {
            // Régua de medição diagonal amarela com graduações pretas e miras nas pontas
            let r_poly = vec![p(3.5, 17.5), p(17.5, 3.5), p(20.5, 6.5), p(6.5, 20.5)];
            painter.add(egui::Shape::convex_polygon(
                r_poly,
                c_yellow,
                Stroke::new(1.6_f32, c_orange),
            ));

            // Graduações (traços da régua)
            let tick_stroke = Stroke::new(1.3_f32, c_dark);
            painter.line_segment([p(8.5, 12.5), p(10.0, 14.0)], tick_stroke);
            painter.line_segment([p(12.0, 9.0), p(13.5, 10.5)], tick_stroke);
            painter.line_segment([p(15.5, 5.5), p(17.0, 7.0)], tick_stroke);

            // Miras nas pontas (+ em azul claro)
            let cross_stroke = Stroke::new(1.8_f32, c_blue);
            painter.line_segment([p(2.0, 19.0), p(6.0, 19.0)], cross_stroke);
            painter.line_segment([p(4.0, 17.0), p(4.0, 21.0)], cross_stroke);

            painter.line_segment([p(18.0, 3.0), p(22.0, 3.0)], cross_stroke);
            painter.line_segment([p(20.0, 1.0), p(20.0, 5.0)], cross_stroke);
        }
        "primitives" | "add_primitive" => {
            // Cubo isométrico sombreado em tons de laranja do Blender + badge com sinal '+'
            let top = p(10.0, 3.0);
            let top_left = p(3.0, 7.0);
            let top_right = p(17.0, 7.0);
            let center = p(10.0, 11.0);
            let bot_left = p(3.0, 15.5);
            let bot_right = p(17.0, 15.5);
            let bot = p(10.0, 19.5);

            let border_stroke = Stroke::new(
                1.8_f32,
                Color32::from_rgb(255, 224, 178).linear_multiply(alpha_factor),
            );

            // Face superior (laranja claro)
            painter.add(egui::Shape::convex_polygon(
                vec![top, top_right, center, top_left],
                Color32::from_rgb(255, 167, 38).linear_multiply(alpha_factor),
                border_stroke,
            ));

            // Face esquerda (laranja escuro)
            painter.add(egui::Shape::convex_polygon(
                vec![top_left, center, bot, bot_left],
                Color32::from_rgb(230, 81, 0).linear_multiply(alpha_factor),
                border_stroke,
            ));

            // Face direita (laranja médio)
            painter.add(egui::Shape::convex_polygon(
                vec![center, top_right, bot_right, bot],
                Color32::from_rgb(251, 140, 0).linear_multiply(alpha_factor),
                border_stroke,
            ));

            // Badge '+' no canto inferior direito
            let badge_c = p(18.0, 17.5);
            painter.circle_filled(badge_c, 4.5, c_blue);
            let plus_stroke = Stroke::new(2.0_f32, c_white);
            painter.line_segment(
                [badge_c + vec2(0.0, -2.5), badge_c + vec2(0.0, 2.5)],
                plus_stroke,
            );
            painter.line_segment(
                [badge_c + vec2(-2.5, 0.0), badge_c + vec2(2.5, 0.0)],
                plus_stroke,
            );
        }
        "extrude" => {
            // Face estendida para cima com setas e realce laranja
            let base_stroke = Stroke::new(2.0_f32, c_neutral);
            painter.line_segment([p(4.0, 19.0), p(20.0, 19.0)], base_stroke);
            painter.line_segment([p(4.0, 19.0), p(4.0, 12.0)], base_stroke);
            painter.line_segment([p(20.0, 19.0), p(20.0, 12.0)], base_stroke);

            // Face extrudada no topo (laranja)
            let top_face = Rect::from_min_max(p(4.0, 9.0), p(20.0, 13.0));
            painter.rect_filled(top_face, 1.0, c_orange.linear_multiply(0.3));
            painter.rect_stroke(
                top_face,
                1.0,
                Stroke::new(2.2_f32, c_orange),
                egui::StrokeKind::Inside,
            );

            // Seta central para cima em laranja
            let arrow_tip = p(12.0, 2.5);
            painter.line_segment([p(12.0, 9.0), p(12.0, 5.0)], Stroke::new(2.4_f32, c_orange));
            painter.add(egui::Shape::convex_polygon(
                vec![p(8.5, 6.0), p(15.5, 6.0), arrow_tip],
                c_orange,
                Stroke::NONE,
            ));
        }
        "inset" => {
            // Quadrado externo e quadrado concêntrico interno com realce laranja
            let out_r = Rect::from_min_max(p(3.0, 3.0), p(21.0, 21.0));
            painter.rect_stroke(
                out_r,
                1.0,
                Stroke::new(2.0_f32, c_neutral),
                egui::StrokeKind::Inside,
            );

            let in_r = Rect::from_min_max(p(7.5, 7.5), p(16.5, 16.5));
            painter.rect_filled(in_r, 1.0, c_orange.linear_multiply(0.3));
            painter.rect_stroke(
                in_r,
                1.0,
                Stroke::new(2.2_f32, c_orange),
                egui::StrokeKind::Inside,
            );

            let diag_stroke = Stroke::new(1.8_f32, c_orange);
            painter.line_segment([p(3.0, 3.0), p(7.5, 7.5)], diag_stroke);
            painter.line_segment([p(21.0, 3.0), p(16.5, 7.5)], diag_stroke);
            painter.line_segment([p(3.0, 21.0), p(7.5, 16.5)], diag_stroke);
            painter.line_segment([p(21.0, 21.0), p(16.5, 16.5)], diag_stroke);
        }
        "bevel" => {
            // Quina chanfrada com faceta destacada em dourado/laranja
            let neutral_stroke = Stroke::new(2.0_f32, c_neutral);
            painter.line_segment([p(4.0, 4.0), p(13.0, 4.0)], neutral_stroke);
            painter.line_segment([p(20.0, 11.0), p(20.0, 20.0)], neutral_stroke);
            painter.line_segment([p(20.0, 20.0), p(4.0, 20.0)], neutral_stroke);
            painter.line_segment([p(4.0, 20.0), p(4.0, 4.0)], neutral_stroke);

            // Chanfro bold em dourado/laranja
            let bevel_stroke = Stroke::new(2.5_f32, c_yellow);
            painter.line_segment([p(13.0, 4.0), p(20.0, 11.0)], bevel_stroke);

            // Faceta tracejada interna
            let facet_stroke = Stroke::new(1.5_f32, c_orange);
            painter.line_segment([p(13.0, 4.0), p(13.0, 11.0)], facet_stroke);
            painter.line_segment([p(13.0, 11.0), p(20.0, 11.0)], facet_stroke);
        }
        "pushpull" => {
            // Superfície com setas normais bidirecionais em ciano vibrante
            let base_stroke = Stroke::new(2.0_f32, c_neutral);
            painter.line_segment([p(4.0, 16.0), p(20.0, 16.0)], base_stroke);

            // Seta para cima (ciano)
            let up_tip = p(12.0, 3.0);
            painter.line_segment([p(12.0, 16.0), p(12.0, 6.0)], Stroke::new(2.4_f32, c_cyan));
            painter.add(egui::Shape::convex_polygon(
                vec![p(9.0, 6.5), p(15.0, 6.5), up_tip],
                c_cyan,
                Stroke::NONE,
            ));

            // Seta para baixo (ciano)
            let down_tip = p(12.0, 22.0);
            painter.line_segment([p(12.0, 16.0), p(12.0, 19.0)], Stroke::new(2.4_f32, c_cyan));
            painter.add(egui::Shape::convex_polygon(
                vec![p(9.0, 18.5), p(15.0, 18.5), down_tip],
                c_cyan,
                Stroke::NONE,
            ));
        }
        "loop_cut" => {
            // Anel de corte dividindo o retângulo ao meio com linha amarela viva do Blender
            let outer = Rect::from_min_max(p(4.0, 4.0), p(20.0, 20.0));
            painter.rect_stroke(
                outer,
                1.0,
                Stroke::new(2.0_f32, c_neutral),
                egui::StrokeKind::Inside,
            );

            // Linha amarela viva do loop cut
            let loop_stroke = Stroke::new(2.6_f32, c_yellow);
            painter.line_segment([p(2.5, 12.0), p(21.5, 12.0)], loop_stroke);
            painter.circle_filled(p(12.0, 12.0), 3.0, c_yellow);
        }
        "knife" => {
            // Faca com lâmina metálica e traçado de corte amarelo vivo
            let blade = vec![p(5.0, 19.0), p(13.0, 11.0), p(10.0, 8.0)];
            painter.add(egui::Shape::convex_polygon(
                blade,
                Color32::from_rgb(200, 214, 229).linear_multiply(alpha_factor),
                Stroke::new(1.8_f32, c_dark),
            ));
            painter.line_segment(
                [p(11.0, 10.0), p(19.0, 3.0)],
                Stroke::new(
                    3.2_f32,
                    Color32::from_rgb(131, 149, 167).linear_multiply(alpha_factor),
                ),
            );

            // Linha de corte tracejada em amarelo
            let cut_stroke = Stroke::new(2.2_f32, c_yellow);
            painter.line_segment([p(2.0, 14.0), p(6.0, 14.0)], cut_stroke);
            painter.line_segment([p(8.0, 14.0), p(12.0, 14.0)], cut_stroke);
            painter.line_segment([p(14.0, 14.0), p(18.0, 14.0)], cut_stroke);
        }
        "slice" => {
            // Plano fatiando um quadrado com linha de corte em laranja vibrante
            let cube = Rect::from_min_max(p(6.0, 6.0), p(18.0, 18.0));
            painter.rect_stroke(
                cube,
                1.0,
                Stroke::new(2.0_f32, c_neutral),
                egui::StrokeKind::Inside,
            );

            let cut_stroke = Stroke::new(2.6_f32, c_orange);
            painter.line_segment([p(2.0, 20.0), p(22.0, 4.0)], cut_stroke);
        }
        "subdivide" => {
            // Quadrado particionado em 4 com grade interna laranja
            let box_rect = Rect::from_min_max(p(4.0, 4.0), p(20.0, 20.0));
            painter.rect_stroke(
                box_rect,
                1.0,
                Stroke::new(2.0_f32, c_neutral),
                egui::StrokeKind::Inside,
            );

            let sub_stroke = Stroke::new(2.4_f32, c_orange);
            painter.line_segment([p(12.0, 4.0), p(12.0, 20.0)], sub_stroke);
            painter.line_segment([p(4.0, 12.0), p(20.0, 12.0)], sub_stroke);
        }
        "draw_profile" => {
            // Curva com pontos de controle laranja
            let line_stroke = Stroke::new(2.2_f32, c_neutral);
            painter.line_segment([p(4.0, 18.0), p(9.5, 6.5)], line_stroke);
            painter.line_segment([p(9.5, 6.5), p(15.5, 14.5)], line_stroke);
            painter.line_segment([p(15.5, 14.5), p(20.0, 8.0)], line_stroke);

            painter.circle_filled(p(4.0, 18.0), 2.5, c_orange);
            painter.circle_filled(p(9.5, 6.5), 2.5, c_orange);
            painter.circle_filled(p(15.5, 14.5), 2.5, c_orange);
            painter.circle_filled(p(20.0, 8.0), 2.5, c_orange);
        }
        "connect" => {
            // Dois vértices com aresta de conexão
            painter.circle_filled(p(6.0, 18.0), 2.5, c_cyan);
            painter.circle_filled(p(18.0, 6.0), 2.5, c_cyan);
            painter.line_segment(
                [p(6.0, 18.0), p(18.0, 6.0)],
                Stroke::new(2.2_f32, c_neutral),
            );
        }
        "dissolve" => {
            // Aresta com indicador de remoção
            painter.line_segment(
                [p(5.0, 19.0), p(19.0, 5.0)],
                Stroke::new(2.0_f32, c_neutral),
            );
            // X vermelho de dissolver no meio
            painter.line_segment([p(9.0, 9.0), p(15.0, 15.0)], Stroke::new(2.2_f32, c_red));
            painter.line_segment([p(15.0, 9.0), p(9.0, 15.0)], Stroke::new(2.2_f32, c_red));
        }
        "mirror" => {
            // Linha de simetria com triângulos espelhados
            painter.line_segment(
                [p(12.0, 3.0), p(12.0, 21.0)],
                Stroke::new(1.5_f32, c_cyan.linear_multiply(0.7)),
            );
            // Triângulo esquerdo
            let t_stroke = Stroke::new(2.0_f32, c_neutral);
            painter.line_segment([p(11.0, 6.0), p(4.0, 14.0)], t_stroke);
            painter.line_segment([p(4.0, 14.0), p(11.0, 18.0)], t_stroke);
            painter.line_segment([p(11.0, 18.0), p(11.0, 6.0)], t_stroke);
            // Triângulo direito (espelho)
            painter.line_segment([p(13.0, 6.0), p(20.0, 14.0)], t_stroke);
            painter.line_segment([p(20.0, 14.0), p(13.0, 18.0)], t_stroke);
            painter.line_segment([p(13.0, 18.0), p(13.0, 6.0)], t_stroke);
        }
        "merge" => {
            // Dois vértices convergindo com setas para um ponto central
            let center = p(12.0, 12.0);
            painter.circle_filled(center, 3.2, c_orange);
            let arrow_stroke = Stroke::new(2.0_f32, c_neutral);
            painter.line_segment([p(4.0, 6.0), p(10.0, 10.0)], arrow_stroke);
            painter.line_segment([p(20.0, 6.0), p(14.0, 10.0)], arrow_stroke);
            painter.line_segment([p(4.0, 18.0), p(10.0, 14.0)], arrow_stroke);
            painter.line_segment([p(20.0, 18.0), p(14.0, 14.0)], arrow_stroke);
        }
        "flip_diagonal" => {
            let quad = Rect::from_min_max(p(5.0, 5.0), p(19.0, 19.0));
            painter.rect_stroke(
                quad,
                1.0,
                Stroke::new(1.8_f32, c_neutral),
                egui::StrokeKind::Inside,
            );
            painter.line_segment([p(6.0, 18.0), p(18.0, 6.0)], Stroke::new(2.2_f32, c_cyan));
            painter.line_segment([p(6.0, 6.0), p(9.0, 9.0)], Stroke::new(1.4_f32, c_yellow));
            painter.line_segment(
                [p(18.0, 18.0), p(15.0, 15.0)],
                Stroke::new(1.4_f32, c_yellow),
            );
        }
        "revolve" => {
            let axis_stroke = Stroke::new(2.2_f32, c_orange);
            painter.line_segment([p(12.0, 3.0), p(12.0, 21.0)], axis_stroke);
            painter.circle_filled(p(12.0, 3.0), 1.8, c_orange);
            painter.circle_filled(p(12.0, 21.0), 1.8, c_orange);

            let arc_stroke = Stroke::new(2.0_f32, c_cyan);
            let arc_pts = [
                p(5.0, 12.0),
                p(7.0, 16.0),
                p(12.0, 17.5),
                p(17.0, 16.0),
                p(19.0, 12.0),
            ];
            for w in arc_pts.windows(2) {
                painter.line_segment([w[0], w[1]], arc_stroke);
            }
            painter.add(egui::Shape::convex_polygon(
                vec![p(17.0, 13.5), p(20.5, 11.5), p(19.0, 15.5)],
                c_cyan,
                Stroke::NONE,
            ));
        }
        "paint" => {
            // Pincel de textura com cabo e cerdas em ciano vibrante
            painter.line_segment(
                [p(6.0, 18.0), p(13.5, 10.5)],
                Stroke::new(2.4_f32, c_neutral),
            );
            painter.line_segment(
                [p(13.5, 10.5), p(19.0, 5.0)],
                Stroke::new(3.5_f32, c_orange),
            );
            painter.circle_filled(p(5.0, 19.0), 3.0, c_cyan);
        }
        "camera" => {
            // Câmera clássica
            let body = Rect::from_min_max(p(4.0, 9.0), p(20.0, 19.0));
            painter.rect_stroke(
                body,
                2.0,
                Stroke::new(2.2_f32, c_neutral),
                egui::StrokeKind::Inside,
            );
            painter.circle_stroke(p(12.0, 14.0), 3.0, Stroke::new(2.2_f32, c_cyan));
            let top_stroke = Stroke::new(2.0_f32, c_neutral);
            painter.line_segment([p(8.0, 9.0), p(10.0, 6.0)], top_stroke);
            painter.line_segment([p(10.0, 6.0), p(14.0, 6.0)], top_stroke);
            painter.line_segment([p(14.0, 6.0), p(16.0, 9.0)], top_stroke);
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
                painter.line_segment([w[0], w[1]], Stroke::new(2.2_f32, c_neutral));
            }
            if let Some(&tip) = arc_pts.first() {
                painter.line_segment(
                    [tip + vec2(0.0, -4.0), tip],
                    Stroke::new(2.2_f32, c_neutral),
                );
                painter.line_segment(
                    [tip + vec2(4.0, -1.0), tip],
                    Stroke::new(2.2_f32, c_neutral),
                );
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
                painter.line_segment([w[0], w[1]], Stroke::new(2.2_f32, c_neutral));
            }
            if let Some(&tip) = arc_pts.first() {
                painter.line_segment(
                    [tip + vec2(0.0, -4.0), tip],
                    Stroke::new(2.2_f32, c_neutral),
                );
                painter.line_segment(
                    [tip + vec2(-4.0, -1.0), tip],
                    Stroke::new(2.2_f32, c_neutral),
                );
            }
        }
        "mode_object" => {
            // Cubo isométrico vazado em laranja/neutro indicando modo de objeto
            let p_top = p(12.0, 4.0);
            let p_right = p(19.0, 8.0);
            let p_mid = p(12.0, 12.0);
            let p_left = p(5.0, 8.0);
            let p_bot_left = p(5.0, 16.0);
            let p_bot = p(12.0, 20.0);
            let p_bot_right = p(19.0, 16.0);

            // Face superior levemente preenchida
            painter.add(egui::Shape::convex_polygon(
                vec![p_top, p_right, p_mid, p_left],
                c_orange.linear_multiply(0.25),
                Stroke::new(1.8_f32, c_orange),
            ));

            let stroke = Stroke::new(1.8_f32, c_orange);
            painter.line_segment([p_left, p_bot_left], stroke);
            painter.line_segment([p_mid, p_bot], stroke);
            painter.line_segment([p_right, p_bot_right], stroke);
            painter.line_segment([p_bot_left, p_bot], stroke);
            painter.line_segment([p_bot, p_bot_right], stroke);
        }
        "mode_edit" => {
            // Cubo isométrico com vértices destacados (autêntico Blender Edit Mode)
            let p_top = p(12.0, 4.0);
            let p_right = p(19.0, 8.0);
            let p_mid = p(12.0, 12.0);
            let p_left = p(5.0, 8.0);
            let p_bot_left = p(5.0, 16.0);
            let p_bot = p(12.0, 20.0);
            let p_bot_right = p(19.0, 16.0);

            let stroke = Stroke::new(1.5_f32, c_neutral);
            // Face superior
            painter.line_segment([p_top, p_right], stroke);
            painter.line_segment([p_right, p_mid], stroke);
            painter.line_segment([p_mid, p_left], stroke);
            painter.line_segment([p_left, p_top], stroke);

            // Arestas verticais e inferiores
            painter.line_segment([p_left, p_bot_left], stroke);
            painter.line_segment([p_mid, p_bot], stroke);
            painter.line_segment([p_right, p_bot_right], stroke);
            painter.line_segment([p_bot_left, p_bot], stroke);
            painter.line_segment([p_bot, p_bot_right], stroke);

            // Vértices amarelos destacados com núcleo brilhante
            let verts = [
                p_top,
                p_right,
                p_mid,
                p_left,
                p_bot_left,
                p_bot,
                p_bot_right,
            ];
            for v in verts {
                painter.circle_filled(v, 2.0, c_yellow);
                painter.circle_stroke(v, 2.0, Stroke::new(1.0_f32, c_dark));
            }
        }
        "select_vertex" => {
            // Alvo de Seleção de Vértice: malha wireframe discreta com ponto destacado no centro
            let stroke_bg = Stroke::new(1.2_f32, c_neutral.linear_multiply(0.4));
            painter.line_segment([p(4.0, 12.0), p(20.0, 12.0)], stroke_bg);
            painter.line_segment([p(12.0, 4.0), p(12.0, 20.0)], stroke_bg);
            painter.rect_stroke(
                Rect::from_min_max(p(6.0, 6.0), p(18.0, 18.0)),
                1.0,
                stroke_bg,
                egui::StrokeKind::Inside,
            );

            // Vértice central amarelo destacado
            let center = p(12.0, 12.0);
            painter.circle_filled(center, 3.2, c_yellow);
            painter.circle_stroke(center, 3.2, Stroke::new(1.2_f32, c_white));
        }
        "select_edge" => {
            // Alvo de Seleção de Aresta: malha discreta com aresta diagonal reforçada
            let stroke_bg = Stroke::new(1.2_f32, c_neutral.linear_multiply(0.4));
            painter.rect_stroke(
                Rect::from_min_max(p(6.0, 6.0), p(18.0, 18.0)),
                1.0,
                stroke_bg,
                egui::StrokeKind::Inside,
            );

            // Aresta diagonal reforçada com marcadores nas pontas
            let p1 = p(6.0, 18.0);
            let p2 = p(18.0, 6.0);
            painter.line_segment([p1, p2], Stroke::new(2.8_f32, c_yellow));
            painter.circle_filled(p1, 2.0, c_white);
            painter.circle_filled(p2, 2.0, c_white);
        }
        "select_face" => {
            // Alvo de Seleção de Face: polígono quad preenchido com realce
            let face_pts = vec![p(6.0, 6.0), p(18.0, 6.0), p(18.0, 18.0), p(6.0, 18.0)];
            painter.add(egui::Shape::convex_polygon(
                face_pts,
                c_yellow.linear_multiply(0.4),
                Stroke::new(2.0_f32, c_yellow),
            ));
            // Pequeno ponto no baricentro da face
            painter.circle_filled(p(12.0, 12.0), 1.8, c_white);
        }
        "shading_wireframe" => {
            // Esfera de Shading Wireframe estilo canônico do Blender
            let center = p(12.0, 12.0);
            let r = side * 0.38;
            painter.circle_stroke(center, r, Stroke::new(1.8_f32, c_neutral));

            // Equador e meridiano elípticos
            painter.line_segment(
                [p(4.0, 12.0), p(20.0, 12.0)],
                Stroke::new(1.2_f32, c_neutral),
            );
            let mut arc = Vec::new();
            for i in 0..=12 {
                let a = (i as f32 / 12.0) * std::f32::consts::PI;
                arc.push(Pos2::new(
                    center.x + a.cos() * (r * 0.35),
                    center.y - a.sin() * r,
                ));
            }
            for w in arc.windows(2) {
                painter.line_segment([w[0], w[1]], Stroke::new(1.2_f32, c_neutral));
            }
        }
        "shading_solid" => {
            // Esfera sólida preenchida com volume suave
            let center = p(12.0, 12.0);
            let r = side * 0.38;
            painter.circle_filled(center, r, c_neutral);
            // Ponto de luz superior esquerdo
            painter.circle_filled(p(10.0, 10.0), r * 0.45, c_white.linear_multiply(0.4));
            painter.circle_stroke(
                center,
                r,
                Stroke::new(1.5_f32, c_white.linear_multiply(0.6)),
            );
        }
        "shading_material" => {
            // Esfera Material Preview: metade sólida colorida, metade contorno
            let center = p(12.0, 12.0);
            let r = side * 0.38;
            // Base circular
            painter.circle_filled(center, r, c_dark);

            // Semicírculo direito preenchido com laranja quente
            let mut semi_pts = vec![center];
            for i in -6..=6 {
                let a = (i as f32 / 6.0) * (std::f32::consts::PI * 0.5);
                semi_pts.push(Pos2::new(center.x + a.cos() * r, center.y + a.sin() * r));
            }
            painter.add(egui::Shape::convex_polygon(
                semi_pts,
                c_orange,
                Stroke::NONE,
            ));

            painter.line_segment([p(12.0, 3.5), p(12.0, 20.5)], Stroke::new(1.2_f32, c_white));
            painter.circle_stroke(
                center,
                r,
                Stroke::new(1.5_f32, c_white.linear_multiply(0.8)),
            );
        }
        "shading_rendered" => {
            // Esfera Rendered: núcleo brilhante com raios solares sutis
            let center = p(12.0, 12.0);
            let r = side * 0.32;
            painter.circle_filled(center, r, c_yellow);
            painter.circle_filled(center, r * 0.5, c_white);

            // 4 raios cardeais
            let ray_stroke = Stroke::new(1.5_f32, c_yellow);
            painter.line_segment([p(12.0, 2.5), p(12.0, 5.0)], ray_stroke);
            painter.line_segment([p(12.0, 19.0), p(12.0, 21.5)], ray_stroke);
            painter.line_segment([p(2.5, 12.0), p(5.0, 12.0)], ray_stroke);
            painter.line_segment([p(19.0, 12.0), p(21.5, 12.0)], ray_stroke);
        }
        "snap_magnet" => {
            // Ímã de ferradura técnico com polos vermelho e azul
            let p_left_top = p(6.0, 6.0);
            let p_right_top = p(18.0, 6.0);

            // Arco em U
            let stroke_base = Stroke::new(3.2_f32, c_neutral);
            let mut u_pts = Vec::new();
            for i in 0..=12 {
                let a = (i as f32 / 12.0) * std::f32::consts::PI;
                u_pts.push(Pos2::new(
                    12.0 + (a.cos() * 6.0 / 24.0) * grid_rect.width(),
                    13.0 + (a.sin() * 6.0 / 24.0) * grid_rect.height(),
                ));
            }
            painter.line_segment([p(6.0, 9.0), p(6.0, 13.0)], stroke_base);
            painter.line_segment([p(18.0, 9.0), p(18.0, 13.0)], stroke_base);
            for w in u_pts.windows(2) {
                painter.line_segment([w[0], w[1]], stroke_base);
            }

            // Polos nas pontas: vermelho à esquerda, azul à direita
            painter.line_segment([p_left_top, p(6.0, 9.0)], Stroke::new(3.2_f32, c_red));
            painter.line_segment([p_right_top, p(18.0, 9.0)], Stroke::new(3.2_f32, c_blue));
        }
        "proportional_editing" => {
            // Edição proporcional: anéis concêntricos e curva de decaimento suave
            let center = p(12.0, 12.0);
            painter.circle_stroke(center, side * 0.38, Stroke::new(1.5_f32, c_neutral));
            painter.circle_stroke(center, side * 0.22, Stroke::new(1.2_f32, c_cyan));
            painter.circle_filled(center, 2.0, c_white);
        }
        "xray" => {
            // Dois planos translúcidos sobrepostos
            let b1 = Rect::from_min_max(p(4.0, 4.0), p(15.0, 15.0));
            let b2 = Rect::from_min_max(p(9.0, 9.0), p(20.0, 20.0));
            painter.rect_filled(b1, 1.0, c_cyan.linear_multiply(0.2));
            painter.rect_stroke(
                b1,
                1.0,
                Stroke::new(1.4_f32, c_cyan.linear_multiply(0.7)),
                egui::StrokeKind::Inside,
            );
            painter.rect_filled(b2, 1.0, c_cyan.linear_multiply(0.3));
            painter.rect_stroke(
                b2,
                1.0,
                Stroke::new(1.6_f32, c_cyan),
                egui::StrokeKind::Inside,
            );
        }
        "overlays" => {
            // Grade de eixos de viewport
            let stroke_grid = Stroke::new(1.0_f32, c_neutral.linear_multiply(0.5));
            painter.line_segment([p(4.0, 8.0), p(20.0, 8.0)], stroke_grid);
            painter.line_segment([p(4.0, 16.0), p(20.0, 16.0)], stroke_grid);
            painter.line_segment([p(8.0, 4.0), p(8.0, 20.0)], stroke_grid);
            painter.line_segment([p(16.0, 4.0), p(16.0, 20.0)], stroke_grid);

            // Eixo X vermelho e Y verde centrais
            painter.line_segment([p(4.0, 12.0), p(20.0, 12.0)], Stroke::new(1.8_f32, c_red));
            painter.line_segment([p(12.0, 4.0), p(12.0, 20.0)], Stroke::new(1.8_f32, c_green));
        }
        "orientation_global" => {
            // Três eixos coordenados XYZ saindo de uma origem
            let origin = p(10.0, 14.0);
            painter.line_segment([origin, p(19.0, 14.0)], Stroke::new(2.0_f32, c_red)); // X
            painter.line_segment([origin, p(14.0, 8.0)], Stroke::new(2.0_f32, c_green)); // Y
            painter.line_segment([origin, p(10.0, 4.0)], Stroke::new(2.0_f32, c_blue)); // Z
            painter.circle_filled(origin, 2.0, c_white);
        }
        "pivot_median" => {
            // Dois pontos e um marcador destacado no ponto médio
            let p1 = p(6.0, 12.0);
            let p2 = p(18.0, 12.0);
            let mid = p(12.0, 12.0);
            painter.line_segment(
                [p1, p2],
                Stroke::new(1.2_f32, c_neutral.linear_multiply(0.5)),
            );
            painter.circle_filled(p1, 2.0, c_neutral);
            painter.circle_filled(p2, 2.0, c_neutral);

            // Ponto médio destacado
            painter.circle_filled(mid, 3.0, c_yellow);
            painter.circle_stroke(mid, 3.0, Stroke::new(1.2_f32, c_white));
        }
        "object_mesh" => {
            // Malha de objeto 3D canônica: cubo isométrico laranja
            let p_top = p(12.0, 4.0);
            let p_right = p(19.0, 8.0);
            let p_mid = p(12.0, 12.0);
            let p_left = p(5.0, 8.0);
            let p_bot_left = p(5.0, 16.0);
            let p_bot = p(12.0, 20.0);
            let p_bot_right = p(19.0, 16.0);

            painter.add(egui::Shape::convex_polygon(
                vec![p_top, p_right, p_mid, p_left],
                c_orange.linear_multiply(0.35),
                Stroke::new(1.6_f32, c_orange),
            ));

            let stroke = Stroke::new(1.6_f32, c_orange);
            painter.line_segment([p_left, p_bot_left], stroke);
            painter.line_segment([p_mid, p_bot], stroke);
            painter.line_segment([p_right, p_bot_right], stroke);
            painter.line_segment([p_bot_left, p_bot], stroke);
            painter.line_segment([p_bot, p_bot_right], stroke);
        }
        "reference_image" => {
            // Moldura fotográfica com paisagem (sol e montanhas)
            let frame = Rect::from_min_max(p(4.0, 5.0), p(20.0, 19.0));
            painter.rect_filled(frame, 2.0, c_dark);
            painter.rect_stroke(
                frame,
                2.0,
                Stroke::new(1.5_f32, c_cyan),
                egui::StrokeKind::Inside,
            );

            // Sol
            painter.circle_filled(p(8.0, 9.0), 1.8, c_yellow);

            // Montanha
            let mtn = vec![
                p(5.5, 17.5),
                p(11.0, 11.5),
                p(15.0, 15.5),
                p(17.5, 13.5),
                p(18.5, 17.5),
            ];
            painter.add(egui::Shape::convex_polygon(
                mtn,
                c_cyan.linear_multiply(0.4),
                Stroke::new(1.2_f32, c_cyan),
            ));
        }
        "collection" => {
            // Pasta de Coleção técnica de Outliner
            let p_tab = vec![
                p(4.0, 7.0),
                p(10.0, 7.0),
                p(12.0, 9.0),
                p(20.0, 9.0),
                p(20.0, 18.0),
                p(4.0, 18.0),
            ];
            painter.add(egui::Shape::convex_polygon(
                p_tab,
                c_white.linear_multiply(0.12),
                Stroke::new(1.5_f32, c_neutral),
            ));
        }
        "duplicate" => {
            // Dois retângulos sobrepostos
            let r1 = Rect::from_min_max(p(4.0, 4.0), p(15.0, 15.0));
            let r2 = Rect::from_min_max(p(9.0, 9.0), p(20.0, 20.0));
            painter.rect_stroke(
                r1,
                1.5,
                Stroke::new(1.4_f32, c_neutral),
                egui::StrokeKind::Inside,
            );
            painter.rect_filled(r2, 1.5, c_dark);
            painter.rect_stroke(
                r2,
                1.5,
                Stroke::new(1.6_f32, c_white),
                egui::StrokeKind::Inside,
            );
        }
        "delete" | "trash" => {
            // Lixeira técnica
            let lid_stroke = Stroke::new(1.6_f32, c_red);
            painter.line_segment([p(5.0, 6.0), p(19.0, 6.0)], lid_stroke);
            painter.line_segment([p(10.0, 4.0), p(14.0, 4.0)], lid_stroke);

            let body = vec![p(7.0, 7.5), p(17.0, 7.5), p(15.5, 19.5), p(8.5, 19.5)];
            painter.add(egui::Shape::convex_polygon(
                body,
                c_red.linear_multiply(0.15),
                Stroke::new(1.5_f32, c_red),
            ));
            painter.line_segment([p(10.0, 10.0), p(9.5, 17.0)], Stroke::new(1.2_f32, c_red));
            painter.line_segment([p(14.0, 10.0), p(14.5, 17.0)], Stroke::new(1.2_f32, c_red));
        }
        "settings" => {
            // Engrenagem técnica
            let center = p(12.0, 12.0);
            let r_outer = side * 0.35;
            let r_inner = side * 0.16;
            painter.circle_stroke(center, r_outer, Stroke::new(2.0_f32, c_neutral));
            painter.circle_filled(center, r_inner, c_neutral);
            for i in 0..6 {
                let angle = (i as f32) * std::f32::consts::PI / 3.0;
                let dir = egui::vec2(angle.cos(), angle.sin());
                let p1 = center + dir * (r_outer - 1.5);
                let p2 = center + dir * (r_outer + 3.0);
                painter.line_segment([p1, p2], Stroke::new(2.4_f32, c_neutral));
            }
        }
        "search" => {
            // Lupa
            let center = p(10.0, 10.0);
            let r = side * 0.24;
            painter.circle_stroke(center, r, Stroke::new(2.0_f32, c_neutral));
            let handle_start = p(14.0, 14.0);
            let handle_end = p(20.0, 20.0);
            painter.line_segment([handle_start, handle_end], Stroke::new(2.4_f32, c_neutral));
        }
        "folder" => {
            // Pasta técnica com aba
            let tab = vec![p(4.0, 6.0), p(10.0, 6.0), p(12.0, 8.0), p(4.0, 8.0)];
            painter.add(egui::Shape::convex_polygon(
                tab,
                c_yellow.linear_multiply(0.3),
                Stroke::new(1.5_f32, c_yellow),
            ));
            let body = Rect::from_min_max(p(4.0, 8.0), p(20.0, 19.0));
            painter.rect_filled(body, 2.0, c_yellow.linear_multiply(0.15));
            painter.rect_stroke(
                body,
                2.0,
                Stroke::new(1.6_f32, c_yellow),
                egui::StrokeKind::Inside,
            );
        }
        "file" => {
            // Documento com canto dobrado
            let doc = vec![
                p(5.0, 4.0),
                p(14.0, 4.0),
                p(19.0, 9.0),
                p(19.0, 20.0),
                p(5.0, 20.0),
            ];
            painter.add(egui::Shape::convex_polygon(
                doc,
                c_neutral.linear_multiply(0.1),
                Stroke::new(1.5_f32, c_neutral),
            ));
            // Dobra do canto
            painter.line_segment(
                [p(14.0, 4.0), p(14.0, 9.0)],
                Stroke::new(1.4_f32, c_neutral),
            );
            painter.line_segment(
                [p(14.0, 9.0), p(19.0, 9.0)],
                Stroke::new(1.4_f32, c_neutral),
            );
        }
        "eye" => {
            // Olho canônico
            let center = p(12.0, 12.0);
            painter.circle_filled(center, 2.5, c_neutral);
            let mut top_pts = Vec::new();
            let mut bot_pts = Vec::new();
            for i in 0..=8 {
                let t = i as f32 / 8.0;
                let x = 4.0 + t * 16.0;
                let y_top = 12.0 - 5.5 * (std::f32::consts::PI * t).sin();
                let y_bot = 12.0 + 5.5 * (std::f32::consts::PI * t).sin();
                top_pts.push(p(x, y_top));
                bot_pts.push(p(x, y_bot));
            }
            for w in top_pts.windows(2) {
                painter.line_segment([w[0], w[1]], Stroke::new(1.6_f32, c_neutral));
            }
            for w in bot_pts.windows(2) {
                painter.line_segment([w[0], w[1]], Stroke::new(1.6_f32, c_neutral));
            }
        }
        "eye_hidden" => {
            // Olho oculto com barra diagonal
            let center = p(12.0, 12.0);
            painter.circle_filled(center, 2.0, c_neutral.linear_multiply(0.5));
            for t_val in 0..=8 {
                let t = t_val as f32 / 8.0;
                let x = 4.0 + t * 16.0;
                let y_top = 12.0 - 5.0 * (std::f32::consts::PI * t).sin();
                let y_bot = 12.0 + 5.0 * (std::f32::consts::PI * t).sin();
                if t_val > 0 {
                    let prev_t = (t_val - 1) as f32 / 8.0;
                    let prev_x = 4.0 + prev_t * 16.0;
                    painter.line_segment(
                        [
                            p(prev_x, 12.0 - 5.0 * (std::f32::consts::PI * prev_t).sin()),
                            p(x, y_top),
                        ],
                        Stroke::new(1.4_f32, c_neutral.linear_multiply(0.5)),
                    );
                    painter.line_segment(
                        [
                            p(prev_x, 12.0 + 5.0 * (std::f32::consts::PI * prev_t).sin()),
                            p(x, y_bot),
                        ],
                        Stroke::new(1.4_f32, c_neutral.linear_multiply(0.5)),
                    );
                }
            }
            painter.line_segment([p(5.0, 19.0), p(19.0, 5.0)], Stroke::new(1.8_f32, c_red));
        }
        "lock" => {
            // Cadeado fechado
            let body = Rect::from_min_max(p(6.0, 10.0), p(18.0, 20.0));
            painter.rect_filled(body, 2.0, c_neutral.linear_multiply(0.2));
            painter.rect_stroke(
                body,
                2.0,
                Stroke::new(1.6_f32, c_neutral),
                egui::StrokeKind::Inside,
            );
            // Arco superior
            let top_arc = [p(8.5, 10.0), p(8.5, 6.0), p(15.5, 6.0), p(15.5, 10.0)];
            painter.line_segment([top_arc[0], top_arc[1]], Stroke::new(1.6_f32, c_neutral));
            painter.line_segment([top_arc[1], top_arc[2]], Stroke::new(1.6_f32, c_neutral));
            painter.line_segment([top_arc[2], top_arc[3]], Stroke::new(1.6_f32, c_neutral));
        }
        "unlock" => {
            // Cadeado aberto
            let body = Rect::from_min_max(p(6.0, 10.0), p(18.0, 20.0));
            painter.rect_filled(body, 2.0, c_neutral.linear_multiply(0.2));
            painter.rect_stroke(
                body,
                2.0,
                Stroke::new(1.6_f32, c_neutral),
                egui::StrokeKind::Inside,
            );
            // Arco levantado
            let top_arc = [p(8.5, 10.0), p(8.5, 4.0), p(15.5, 4.0), p(15.5, 7.0)];
            painter.line_segment([top_arc[0], top_arc[1]], Stroke::new(1.6_f32, c_neutral));
            painter.line_segment([top_arc[1], top_arc[2]], Stroke::new(1.6_f32, c_neutral));
            painter.line_segment([top_arc[2], top_arc[3]], Stroke::new(1.6_f32, c_neutral));
        }
        "close" => {
            // Cruz / X
            painter.line_segment(
                [p(6.0, 6.0), p(18.0, 18.0)],
                Stroke::new(1.8_f32, c_neutral),
            );
            painter.line_segment(
                [p(18.0, 6.0), p(6.0, 18.0)],
                Stroke::new(1.8_f32, c_neutral),
            );
        }
        "plus" => {
            // Sinal de mais
            painter.line_segment(
                [p(12.0, 5.0), p(12.0, 19.0)],
                Stroke::new(2.0_f32, c_neutral),
            );
            painter.line_segment(
                [p(5.0, 12.0), p(19.0, 12.0)],
                Stroke::new(2.0_f32, c_neutral),
            );
        }
        "minimize" => {
            // Sinal de menos
            painter.line_segment(
                [p(6.0, 12.0), p(18.0, 12.0)],
                Stroke::new(2.0_f32, c_neutral),
            );
        }
        "maximize" => {
            // Quadrado de maximizar
            let r = Rect::from_min_max(p(6.0, 6.0), p(18.0, 18.0));
            painter.rect_stroke(
                r,
                1.5,
                Stroke::new(1.6_f32, c_neutral),
                egui::StrokeKind::Inside,
            );
        }
        "chevron_left" => {
            painter.line_segment(
                [p(15.0, 6.0), p(9.0, 12.0)],
                Stroke::new(2.0_f32, c_neutral),
            );
            painter.line_segment(
                [p(9.0, 12.0), p(15.0, 18.0)],
                Stroke::new(2.0_f32, c_neutral),
            );
        }
        "chevron_right" => {
            painter.line_segment(
                [p(9.0, 6.0), p(15.0, 12.0)],
                Stroke::new(2.0_f32, c_neutral),
            );
            painter.line_segment(
                [p(15.0, 12.0), p(9.0, 18.0)],
                Stroke::new(2.0_f32, c_neutral),
            );
        }
        "chevron_down" => {
            painter.line_segment(
                [p(6.0, 9.0), p(12.0, 15.0)],
                Stroke::new(2.0_f32, c_neutral),
            );
            painter.line_segment(
                [p(12.0, 15.0), p(18.0, 9.0)],
                Stroke::new(2.0_f32, c_neutral),
            );
        }
        "chevron_up" => {
            painter.line_segment(
                [p(6.0, 15.0), p(12.0, 9.0)],
                Stroke::new(2.0_f32, c_neutral),
            );
            painter.line_segment(
                [p(12.0, 9.0), p(18.0, 15.0)],
                Stroke::new(2.0_f32, c_neutral),
            );
        }
        "more_vert" => {
            painter.circle_filled(p(12.0, 6.0), 1.8, c_neutral);
            painter.circle_filled(p(12.0, 12.0), 1.8, c_neutral);
            painter.circle_filled(p(12.0, 18.0), 1.8, c_neutral);
        }
        "filter" => {
            let funnel = vec![
                p(5.0, 6.0),
                p(19.0, 6.0),
                p(14.0, 13.0),
                p(14.0, 19.0),
                p(10.0, 17.0),
                p(10.0, 13.0),
            ];
            painter.add(egui::Shape::convex_polygon(
                funnel,
                c_neutral.linear_multiply(0.15),
                Stroke::new(1.6_f32, c_neutral),
            ));
        }
        "play" => {
            let triangle = vec![p(8.0, 5.0), p(18.0, 12.0), p(8.0, 19.0)];
            painter.add(egui::Shape::convex_polygon(
                triangle,
                c_green,
                Stroke::new(1.2_f32, c_dark),
            ));
        }
        "pause" => {
            let bar1 = Rect::from_min_max(p(7.0, 5.0), p(10.5, 19.0));
            let bar2 = Rect::from_min_max(p(13.5, 5.0), p(17.0, 19.0));
            painter.rect_filled(bar1, 1.0, c_neutral);
            painter.rect_filled(bar2, 1.0, c_neutral);
        }
        _ => {
            // Fallback genérico: losango geométrico
            let top = p(12.0, 4.0);
            let right = p(20.0, 12.0);
            let bottom = p(12.0, 20.0);
            let left = p(4.0, 12.0);
            let stroke = Stroke::new(2.0_f32, c_neutral);
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
            "select_box",
            "cursor_3d",
            "move",
            "transform",
            "rotate",
            "scale",
            "annotate",
            "measure",
            "add_primitive",
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
            "mode_object",
            "mode_edit",
            "select_vertex",
            "select_edge",
            "select_face",
            "shading_wireframe",
            "shading_solid",
            "shading_material",
            "shading_rendered",
            "snap_magnet",
            "proportional_editing",
            "xray",
            "overlays",
            "orientation_global",
            "pivot_median",
            "object_mesh",
            "reference_image",
            "collection",
            "duplicate",
            "delete",
            "unknown_tool",
        ];

        let ctx = egui::Context::default();
        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                let r = Rect::from_min_size(Pos2::ZERO, vec2(24.0, 24.0));
                for id in tools {
                    paint(ui.painter(), id, r, Color32::WHITE);
                }
            });
        })
        .textures_delta
        .clear();
    }

    #[test]
    fn test_tool_button_compact_and_wide_rendering() {
        let ctx = egui::Context::default();
        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                let resp_compact = tool_button(ui, "select", "Selecionar", true, true);
                assert_eq!(resp_compact.rect.width(), 40.0);
                assert_eq!(resp_compact.rect.height(), 40.0);

                let resp_wide = tool_button(ui, "extrude", "Extrusão", false, false);
                assert!(resp_wide.rect.width() >= 136.0);
                assert_eq!(resp_wide.rect.height(), 34.0);
            });
        })
        .textures_delta
        .clear();
    }
}
