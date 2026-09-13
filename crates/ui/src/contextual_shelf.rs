//! Barra Contextual Horizontal do Viewport (`Contextual Modeling Shelf`).
//! Posicionada na base inferior do Viewport 3D, reagindo dinamicamente
//! ao Workspace e ao Modo de Edição ativo (Object, Edit, Paint, UV, Animate).

use egui::{pos2, vec2, Rect, Response, RichText, Ui};
use petunia_core::{
    AppState, DuplicateSelectionCmd, EditMode, MergeCenterCmd, ModalKind, SubdivideSelectionCmd,
    Workspace,
};

use crate::icon_registry::{IconRegistry, PetuniaIcon};
use crate::outliner::add_primitive_to_scene;
use crate::tokens;

/// Renderiza a barra contextual horizontal flutuante na base do Viewport 3D.
/// Retorna o `Rect` ocupado pela shelf para permitir bloqueio de eventos na cena 3D.
pub fn draw(ui: &mut Ui, state: &mut AppState, viewport_rect: Rect) -> Option<Rect> {
    let screen_w = viewport_rect.width();
    if screen_w < 380.0 {
        return None;
    }

    // Calcula largura dinâmica estimada da shelf
    let shelf_height = 34.0;
    let bottom_margin = 12.0;

    // Constrói o conteúdo em container flutuante arredondado
    let (shelf_w, content_closure) = match state.workspace {
        Workspace::Model => {
            if state.mode == EditMode::Edit {
                (560.0_f32.min(screen_w - 32.0), ShelfContent::ModelEdit)
            } else {
                (510.0_f32.min(screen_w - 32.0), ShelfContent::ModelObject)
            }
        }
        Workspace::Paint => (420.0_f32.min(screen_w - 32.0), ShelfContent::Paint),
        Workspace::Uv => (380.0_f32.min(screen_w - 32.0), ShelfContent::Uv),
        Workspace::Animate => (440.0_f32.min(screen_w - 32.0), ShelfContent::Animate),
    };

    let shelf_rect = Rect::from_center_size(
        egui::pos2(
            viewport_rect.center().x,
            viewport_rect.max.y - bottom_margin - shelf_height * 0.5,
        ),
        vec2(shelf_w, shelf_height),
    );

    // Fundo da cápsula com sombra sutil e borda técnica
    let painter = ui.painter();
    painter.rect_filled(shelf_rect, tokens::RADIUS_PILL, tokens::BG_SHELF);
    painter.rect_stroke(
        shelf_rect,
        tokens::RADIUS_PILL,
        tokens::stroke_border(),
        egui::StrokeKind::Inside,
    );

    // Escudo de eventos: impede que cliques na shelf atravessem para o raycasting da cena 3D
    let _ = ui.allocate_rect(shelf_rect, egui::Sense::click_and_drag());

    // Renderiza controles horizontais centralizados dentro da cápsula
    let mut shelf_ui = ui.new_child(egui::UiBuilder::new().max_rect(shelf_rect));
    shelf_ui.horizontal_centered(|ui| {
        ui.spacing_mut().item_spacing = vec2(4.0, 0.0);
        ui.add_space(8.0);

        match content_closure {
            ShelfContent::ModelEdit => draw_model_edit_shelf(ui, state),
            ShelfContent::ModelObject => draw_model_object_shelf(ui, state),
            ShelfContent::Paint => draw_paint_shelf(ui, state),
            ShelfContent::Uv => draw_uv_shelf(ui, state),
            ShelfContent::Animate => draw_animate_shelf(ui, state),
        }

        ui.add_space(8.0);
    });

    Some(shelf_rect)
}

enum ShelfContent {
    ModelEdit,
    ModelObject,
    Paint,
    Uv,
    Animate,
}

fn draw_model_edit_shelf(ui: &mut Ui, state: &mut AppState) {
    // 1. Comandos Principais de Modelagem de Malha (Alvos de Seleção residem exclusivamente no Viewport Bar)
    let modeling_ops = [
        (
            "extrude",
            PetuniaIcon::Extrude,
            "Extrude",
            "E",
            "Extrusão de Faces",
        ),
        (
            "inset",
            PetuniaIcon::Inset,
            "Inset",
            "I",
            "Inserção de Faces",
        ),
        (
            "bevel",
            PetuniaIcon::Bevel,
            "Bevel",
            "Ctrl+B",
            "Chanfro / Bisel",
        ),
        (
            "loop_cut",
            PetuniaIcon::LoopCut,
            "Loop Cut",
            "Ctrl+R",
            "Corte em Anel",
        ),
        ("knife", PetuniaIcon::Knife, "Knife", "K", "Faca Topológica"),
    ];

    for (tool_id, icon, name, shortcut, desc) in modeling_ops {
        let is_active = state.active_tool == tool_id;
        if pill_button(
            ui,
            Some(icon),
            name,
            is_active,
            &format!("{desc} · [{shortcut}]"),
        )
        .clicked()
        {
            state.active_tool = tool_id.to_string();
            state.mark_dirty();
        }
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    // 2. Operações Topológicas de Malha
    if pill_button(
        ui,
        Some(PetuniaIcon::Subdivide),
        "Subdivide",
        false,
        "Subdividir malha selecionada",
    )
    .clicked()
    {
        let _ = state.dispatch(&SubdivideSelectionCmd);
    }

    if pill_button(
        ui,
        Some(PetuniaIcon::Custom("merge")),
        "Merge",
        false,
        "Fundir vértices selecionados no centro",
    )
    .clicked()
    {
        let _ = state.dispatch(&MergeCenterCmd);
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    // 3. Imagem de Referência
    if pill_button(
        ui,
        Some(PetuniaIcon::ReferenceImage),
        "Reference Image",
        false,
        "Carregar imagem de referência 3D",
    )
    .clicked()
    {
        crate::pick_and_add_reference_image(state);
    }
}

fn draw_model_object_shelf(ui: &mut Ui, state: &mut AppState) {
    // 1. Ferramentas de Transformação de Objeto
    let transforms = [
        (ModalKind::Move, PetuniaIcon::Move, "Move", "G"),
        (ModalKind::Rotate, PetuniaIcon::Rotate, "Rotate", "R"),
        (ModalKind::Scale, PetuniaIcon::Scale, "Scale", "S"),
    ];

    for (gizmo_m, icon, label, shortcut) in transforms {
        let is_active = state.gizmo_mode == gizmo_m;
        if pill_button(
            ui,
            Some(icon),
            label,
            is_active,
            &format!("{label} · [{shortcut}]"),
        )
        .clicked()
        {
            state.gizmo_mode = gizmo_m;
            state.mark_dirty();
        }
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    // 2. Criação Rápida de Primitivas na coordenada do 3D Cursor
    let prims = [
        ("Cube", "Cube", 0),
        ("Sphere", "Sphere", 1),
        ("Cylinder", "Cylinder", 2),
        ("Plane", "Plane", 3),
    ];

    for (name, label, kind) in prims {
        if pill_button(
            ui,
            Some(PetuniaIcon::AddPrimitive),
            label,
            false,
            &format!("Criar {name} no 3D Cursor"),
        )
        .clicked()
        {
            add_primitive_to_scene(state, kind, name);
        }
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    // 3. Duplicar Objeto
    if pill_button(
        ui,
        Some(PetuniaIcon::Duplicate),
        "Duplicate",
        false,
        "Duplicar objeto ativo · [Shift+D]",
    )
    .clicked()
    {
        let _ = state.dispatch(&DuplicateSelectionCmd);
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    // 4. Imagem de Referência
    if pill_button(
        ui,
        Some(PetuniaIcon::ReferenceImage),
        "Reference Image",
        false,
        "Carregar imagem de referência 3D",
    )
    .clicked()
    {
        crate::pick_and_add_reference_image(state);
    }
}

fn draw_paint_shelf(ui: &mut Ui, state: &mut AppState) {
    let tools = [
        ("paint", Some(PetuniaIcon::Custom("paint")), "Pincel"),
        ("eraser", Some(PetuniaIcon::Custom("delete")), "Apagador"),
        ("picker", Some(PetuniaIcon::Cursor3D), "Conta-gotas"),
    ];

    for (id, icon, label) in tools {
        let is_active = state.active_tool == id;
        if pill_button(ui, icon, label, is_active, label).clicked() {
            state.active_tool = id.to_string();
            state.mark_dirty();
        }
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    ui.label(
        RichText::new("Raio:")
            .size(10.5)
            .color(tokens::TEXT_SECONDARY),
    );
    ui.add(
        egui::DragValue::new(&mut state.paint_radius)
            .range(0.01..=5.0)
            .speed(0.02),
    );

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    ui.label(
        RichText::new("Cor:")
            .size(11.0)
            .color(tokens::TEXT_SECONDARY),
    );
    ui.color_edit_button_rgb(&mut state.paint_color);
}

fn draw_uv_shelf(ui: &mut Ui, state: &mut AppState) {
    let uv_tools = [
        ("uv_select", "Seleção UV"),
        ("uv_unwrap", "Desdobrar (Unwrap)"),
        ("uv_project", "Projetar da Câmera"),
        ("uv_seam", "Marcar Costura"),
    ];

    for (id, label) in uv_tools {
        let is_active = state.active_tool == id;
        if pill_button(ui, None, label, is_active, label).clicked() {
            state.active_tool = id.to_string();
            state.mark_dirty();
        }
    }
}

fn draw_animate_shelf(ui: &mut Ui, state: &mut AppState) {
    if pill_button(
        ui,
        Some(PetuniaIcon::JumpStart),
        "",
        false,
        "Primeiro Frame",
    )
    .clicked()
    {
        state.timeline_frame = state.timeline_start;
        state.mark_dirty();
    }

    let (play_icon, play_label) = if state.timeline_playing {
        (PetuniaIcon::Pause, "Pause")
    } else {
        (PetuniaIcon::Play, "Play")
    };
    if pill_button(
        ui,
        Some(play_icon),
        play_label,
        state.timeline_playing,
        "Iniciar/Pausar animação",
    )
    .clicked()
    {
        state.timeline_playing = !state.timeline_playing;
        state.mark_dirty();
    }

    if pill_button(ui, Some(PetuniaIcon::JumpEnd), "", false, "Último Frame").clicked() {
        state.timeline_frame = state.timeline_end;
        state.mark_dirty();
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    ui.label(
        RichText::new("Frame:")
            .size(10.5)
            .color(tokens::TEXT_SECONDARY),
    );
    ui.add(
        egui::DragValue::new(&mut state.timeline_frame)
            .range(state.timeline_start..=state.timeline_end),
    );
}

fn pill_button(
    ui: &mut Ui,
    icon: Option<PetuniaIcon>,
    label: &str,
    is_active: bool,
    tooltip: &str,
) -> Response {
    let (bg, fg) = if is_active {
        (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
    } else {
        (tokens::BG_SURFACE, tokens::TEXT_PRIMARY)
    };

    let text_len = if label.is_empty() {
        0.0
    } else {
        label.len() as f32 * 6.5 + 4.0
    };
    let icon_w = if icon.is_some() { 16.0 } else { 0.0 };
    let padding = if icon.is_some() && !label.is_empty() {
        16.0
    } else {
        12.0
    };
    let desired_size = vec2((icon_w + text_len + padding).max(24.0), 22.0);

    let (rect, resp) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if ui.is_rect_visible(rect) {
        let fill = if is_active {
            bg
        } else if resp.hovered() {
            tokens::BG_SURFACE_HOVER
        } else {
            bg
        };
        ui.painter().rect_filled(rect, tokens::RADIUS_PILL, fill);

        let mut start_x = rect.min.x
            + if icon.is_some() && label.is_empty() {
                (rect.width() - 14.0) * 0.5
            } else {
                6.0
            };
        if let Some(ic) = icon {
            let icon_rect = Rect::from_min_size(
                pos2(start_x, rect.min.y + (rect.height() - 14.0) * 0.5),
                vec2(14.0, 14.0),
            );
            IconRegistry::paint(ui.ctx(), ui.painter(), &ic, icon_rect, fg);
            start_x += 17.0;
        }

        if !label.is_empty() {
            ui.painter().text(
                pos2(start_x, rect.min.y + (rect.height() - 13.0) * 0.5),
                egui::Align2::LEFT_TOP,
                label,
                egui::FontId::proportional(10.5),
                fg,
            );
        }
    }

    resp.on_hover_text(tooltip)
}

fn shelf_separator(ui: &mut Ui) {
    let (rect, _) = ui.allocate_exact_size(vec2(1.0, 16.0), egui::Sense::hover());
    ui.painter().line_segment(
        [rect.center_top(), rect.center_bottom()],
        tokens::stroke_border(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contextual_shelf_renders_without_panic() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let rect = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), vec2(800.0, 600.0));
                let _ = draw(ui, &mut state, rect);
            });
        });
    }
}
