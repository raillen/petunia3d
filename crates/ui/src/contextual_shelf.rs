//! Barra Contextual Horizontal do Viewport (`Contextual Modeling Shelf`).
//! Posicionada na base inferior do Viewport 3D, reagindo dinamicamente
//! ao Workspace e ao Modo de Edição ativo (Object, Edit, Paint, UV, Animate).

use egui::{vec2, Color32, Rect, Response, RichText, Ui};
use petunia_core::{AppState, EditMode, ModalKind, SelectMode, Workspace};

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
                (540.0_f32.min(screen_w - 32.0), ShelfContent::ModelEdit)
            } else {
                (480.0_f32.min(screen_w - 32.0), ShelfContent::ModelObject)
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
    ui.painter()
        .rect_filled(shelf_rect, tokens::RADIUS_PILL, tokens::BG_PANEL);
    ui.painter().rect_stroke(
        shelf_rect,
        tokens::RADIUS_PILL,
        tokens::stroke_border(),
        egui::StrokeKind::Outside,
    );

    // Aloca sub-UI com contenção de eventos de mouse
    let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(shelf_rect));
    child_ui.horizontal_centered(|ui| {
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
    // 1. Alvos de Seleção do Modo de Edição
    let sel_targets = [
        (SelectMode::Vertex, "⬝ Vértice", "1"),
        (SelectMode::Edge, "╱ Aresta", "2"),
        (SelectMode::Face, "▨ Face", "3"),
    ];

    for (mode, label, shortcut) in sel_targets {
        let is_sel = state.select_mode == mode;
        if pill_button(
            ui,
            label,
            is_sel,
            &format!("Selecionar {label} · [{shortcut}]"),
        )
        .clicked()
        {
            state.select_mode = mode;
            state.sync_selection();
            state.mark_dirty();
        }
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    // 2. Comandos Principais de Modelagem de Malha
    let modeling_ops = [
        ("extrude", "Extrude", "E", "Extrusão de Faces"),
        ("inset", "Inset", "I", "Inserção de Faces"),
        ("bevel", "Bevel", "Ctrl+B", "Chanfro / Bisel"),
        ("loop_cut", "Loop Cut", "Ctrl+R", "Corte em Anel"),
        ("knife", "Knife", "K", "Faca Topológica"),
    ];

    for (tool_id, name, shortcut, desc) in modeling_ops {
        let is_active = state.active_tool == tool_id;
        if pill_button(ui, name, is_active, &format!("{desc} · [{shortcut}]")).clicked() {
            state.active_tool = tool_id.to_string();
            state.mark_dirty();
        }
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    // 3. Operações Topológicas de Malha
    if pill_button(ui, "Subdivide", false, "Subdividir malha selecionada").clicked() {
        state.checkpoint("subdivide");
        if let Some(m) = state.project.active_mesh_mut() {
            m.subdivide_selected();
        }
        state.sync_selection();
        state.emit_mesh_changed();
    }

    if pill_button(ui, "Merge", false, "Fundir vértices selecionados no centro").clicked() {
        state.checkpoint("merge");
        if let Some(m) = state.project.active_mesh_mut() {
            m.merge_center();
        }
        state.sync_selection();
        state.emit_mesh_changed();
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    if pill_button(ui, "🖼 Ref", false, "Carregar imagem de referência 3D").clicked() {
        crate::pick_and_add_reference_image(state);
    }
}

fn draw_model_object_shelf(ui: &mut Ui, state: &mut AppState) {
    // 1. Ferramentas de Transformação de Objeto
    let transforms = [
        (ModalKind::Move, "↔ Move", "G"),
        (ModalKind::Rotate, "🔄 Rotate", "R"),
        (ModalKind::Scale, "⤢ Scale", "S"),
    ];

    for (gizmo_m, label, shortcut) in transforms {
        let is_active = state.gizmo_mode == gizmo_m;
        if pill_button(ui, label, is_active, &format!("{label} · [{shortcut}]")).clicked() {
            state.gizmo_mode = gizmo_m;
            state.mark_dirty();
        }
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    // 2. Criação Rápida de Primitivas na coordenada do 3D Cursor
    let prims = [
        ("Cube", "🧊 Cubo", 0),
        ("Sphere", "⚪ Esfera", 1),
        ("Cylinder", "🛢 Cilindro", 2),
        ("Plane", "▭ Plano", 3),
    ];

    for (name, label, kind) in prims {
        if pill_button(ui, label, false, &format!("Criar {name} no 3D Cursor")).clicked() {
            add_primitive_to_scene(state, kind, name);
        }
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    // 3. Duplicar / Deletar Objeto
    if pill_button(
        ui,
        "📋 Duplicar",
        false,
        "Duplicar objeto ativo · [Shift+D]",
    )
    .clicked()
    {
        state.checkpoint("duplicate");
        if let Some(m) = state.project.active_mesh_mut() {
            m.duplicate_selected();
        }
        state.sync_selection();
        state.emit_mesh_changed();
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    if pill_button(
        ui,
        "🖼 Referência",
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
        ("paint", "🖌 Pincel"),
        ("eraser", "🧹 Apagador"),
        ("picker", "🎯 Conta-gotas"),
    ];

    for (id, label) in tools {
        let is_active = state.active_tool == id;
        if pill_button(ui, label, is_active, label).clicked() {
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
    ui.add(egui::DragValue::new(&mut state.canvas_brush).range(1..=32));

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    // Chip da cor ativa
    let active_color = state.palette.first().copied().unwrap_or([1.0, 1.0, 1.0]);
    let (rect, _) = ui.allocate_exact_size(vec2(16.0, 16.0), egui::Sense::hover());
    let col32 = Color32::from_rgb(
        (active_color[0] * 255.0) as u8,
        (active_color[1] * 255.0) as u8,
        (active_color[2] * 255.0) as u8,
    );
    ui.painter().rect_filled(rect, tokens::RADIUS_SMALL, col32);
    ui.painter().rect_stroke(
        rect,
        tokens::RADIUS_SMALL,
        tokens::stroke_border(),
        egui::StrokeKind::Outside,
    );
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
        if pill_button(ui, label, is_active, label).clicked() {
            state.active_tool = id.to_string();
            state.mark_dirty();
        }
    }
}

fn draw_animate_shelf(ui: &mut Ui, state: &mut AppState) {
    if pill_button(ui, "◀◀", false, "Primeiro Frame").clicked() {
        state.timeline_frame = state.timeline_start;
        state.mark_dirty();
    }

    let play_label = if state.timeline_playing {
        "⏸ Pausa"
    } else {
        "▶ Play"
    };
    if pill_button(
        ui,
        play_label,
        state.timeline_playing,
        "Iniciar/Pausar animação",
    )
    .clicked()
    {
        state.timeline_playing = !state.timeline_playing;
        state.mark_dirty();
    }

    if pill_button(ui, "▶▶", false, "Último Frame").clicked() {
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

fn pill_button(ui: &mut Ui, label: &str, is_active: bool, tooltip: &str) -> Response {
    let (bg, fg) = if is_active {
        (tokens::ACCENT_BLUE, tokens::TEXT_ACTIVE)
    } else {
        (tokens::BG_SURFACE, tokens::TEXT_PRIMARY)
    };

    let btn = egui::Button::new(RichText::new(label).size(10.5).color(fg))
        .fill(bg)
        .corner_radius(tokens::RADIUS_PILL);

    ui.add(btn).on_hover_text(tooltip)
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
