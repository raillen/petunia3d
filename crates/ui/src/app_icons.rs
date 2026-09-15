//! Gerenciamento de ativos de ícones rasterizados e vetoriais para a interface do Petunia3D.
//! Carrega ícones extraídos da Golden Reference (`assets/ui/icons/toolbar/*.png`)
//! e oferece renderização com tingimento dinâmico e fallback vetorial seamlessly integrado.

use egui::{
    Color32, ColorImage, Context, Rect, Response, Sense, TextureHandle, TextureOptions, Ui,
    WidgetInfo, WidgetType, vec2,
};

use crate::icons;
use crate::tokens;

// Inclusão dos bytes dos ícones transparentes de 256x256 em tempo de compilação
const ICON_SELECT_BOX: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/select_box.png");
const ICON_CURSOR_3D: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/cursor_3d.png");
const ICON_MOVE: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/move.png");
const ICON_ROTATE: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/rotate.png");
const ICON_SCALE: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/scale.png");
const ICON_TRANSFORM: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/transform.png");
const ICON_ANNOTATE: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/annotate.png");
const ICON_MEASURE: &[u8] = include_bytes!("../../../assets/ui/icons/toolbar/measure.png");
const ICON_ADD_PRIMITIVE: &[u8] =
    include_bytes!("../../../assets/ui/icons/toolbar/add_primitive.png");

/// Retorna ou carrega a textura rasterizada correspondente ao identificador de ferramenta.
pub fn get_icon_texture(ctx: &Context, name: &str) -> Option<TextureHandle> {
    let id = egui::Id::new("petunia_icon_texture").with(name);
    if let Some(handle) = ctx.data(|d| d.get_temp::<TextureHandle>(id)) {
        return Some(handle);
    }

    let raw_bytes: Option<&'static [u8]> = match name {
        "select" | "select_box" => Some(ICON_SELECT_BOX),
        "cursor_3d" => Some(ICON_CURSOR_3D),
        "move" => Some(ICON_MOVE),
        "rotate" => Some(ICON_ROTATE),
        "scale" => Some(ICON_SCALE),
        "transform" => Some(ICON_TRANSFORM),
        "annotate" => Some(ICON_ANNOTATE),
        "measure" => Some(ICON_MEASURE),
        "primitives" | "add_primitive" => Some(ICON_ADD_PRIMITIVE),
        _ => None,
    };

    if let Some(bytes) = raw_bytes
        && let Ok(img) = image::load_from_memory(bytes)
    {
        let rgba = img.to_rgba8();
        let size = [rgba.width() as usize, rgba.height() as usize];
        let color_img = ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
        let handle = ctx.load_texture(format!("tb_icon_{name}"), color_img, TextureOptions::LINEAR);
        ctx.data_mut(|d| d.insert_temp(id, handle.clone()));
        return Some(handle);
    }

    None
}

/// Pinta um ícone (rasterizado ou vetorial) no retângulo alvo com tingimento semântico.
pub fn paint_icon(
    ctx: &Context,
    painter: &egui::Painter,
    id: &str,
    target_rect: Rect,
    tint: Color32,
) {
    if crate::icon_registry::is_toolbar_vector_tool(id) {
        icons::paint(painter, id, target_rect, tint);
        return;
    }

    if let Some(tex) = get_icon_texture(ctx, id) {
        // Renderiza textura rasterizada centralizada e mantendo proporção 1:1
        let side = target_rect.width().min(target_rect.height());
        let icon_rect = Rect::from_center_size(target_rect.center(), vec2(side, side));
        let uv = Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
        painter.image(tex.id(), icon_rect, uv, tint);
    } else {
        // Fallback vetorial limpo de alta precisão
        icons::paint(painter, id, target_rect, tint);
    }
}

/// Botão de ferramenta com suporte a ícones reais da toolbar, destaque ativo `#3169E3`
/// e feedback tátil/visual fiel ao Blender.svg.
pub fn toolbar_button(
    ui: &mut Ui,
    id: &str,
    label: &str,
    selected: bool,
    compact: bool,
) -> Response {
    let desired_size = if compact {
        vec2(tokens::TOOLBAR_WIDTH, tokens::TOOLBAR_WIDTH)
    } else {
        vec2(ui.available_width().max(tokens::TOOLBAR_WIDTH), 32.0)
    };

    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

    response
        .widget_info(|| WidgetInfo::selected(WidgetType::Button, ui.is_enabled(), selected, label));

    if ui.is_rect_visible(rect) {
        let (bg_fill, fg_color) = if selected {
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

        // Borda de foco visual acessível
        if response.has_focus() {
            painter.rect_stroke(
                rect,
                tokens::RADIUS_CONTAINER,
                tokens::stroke_focus(),
                egui::StrokeKind::Inside,
            );
        }

        if compact {
            // Centraliza o ícone de 20x20 no botão de 40x40
            let icon_rect = Rect::from_center_size(rect.center(), vec2(20.0, 20.0));
            paint_icon(ui.ctx(), &painter, id, icon_rect, fg_color);
        } else {
            // Ícone na esquerda (20x20) + texto alinhado
            let icon_rect = Rect::from_min_size(
                egui::pos2(rect.min.x + 8.0, rect.min.y + (rect.height() - 20.0) * 0.5),
                vec2(20.0, 20.0),
            );
            paint_icon(ui.ctx(), &painter, id, icon_rect, fg_color);

            let text_pos = egui::pos2(rect.min.x + 36.0, rect.min.y + (rect.height() - 14.0) * 0.5);
            painter.text(
                text_pos,
                egui::Align2::LEFT_TOP,
                label,
                egui::FontId::proportional(13.0),
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
    fn test_embedded_raster_icons_load_validly() {
        let ctx = egui::Context::default();
        let raster_tools = [
            "select_box",
            "cursor_3d",
            "move",
            "rotate",
            "scale",
            "transform",
            "annotate",
            "measure",
            "add_primitive",
        ];

        for tool in raster_tools {
            let tex = get_icon_texture(&ctx, tool);
            assert!(
                tex.is_some(),
                "Ícone rasterizado '{}' deve carregar com sucesso dos assets embutidos",
                tool
            );
        }
    }

    #[test]
    fn test_paint_icon_with_both_raster_and_vector() {
        let ctx = egui::Context::default();
        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                let rect = Rect::from_min_size(egui::Pos2::ZERO, vec2(24.0, 24.0));
                // Raster tool
                paint_icon(&ctx, ui.painter(), "select_box", rect, Color32::WHITE);
                // Vector tool fallback
                paint_icon(&ctx, ui.painter(), "extrude", rect, Color32::WHITE);
            });
        })
        .textures_delta
        .clear();
    }

    #[test]
    fn test_toolbar_button_interaction() {
        let ctx = egui::Context::default();
        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                let resp = toolbar_button(ui, "select_box", "Select", true, true);
                assert_eq!(resp.rect.width(), tokens::TOOLBAR_WIDTH);
                assert_eq!(resp.rect.height(), tokens::TOOLBAR_WIDTH);
            });
        })
        .textures_delta
        .clear();
    }
}
