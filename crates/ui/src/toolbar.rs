//! Barra lateral vertical de ferramentas do Petunia3D (`Toolbar`).
//! Renderiza as ferramentas com os ícones extraídos da Golden Reference (`assets/ui/icons/toolbar/`)
//! com 40px de largura e suporte a destaque ativo e atalhos de teclado.

use egui::{vec2, Context, ScrollArea, SidePanel};
use petunia_core::{AppState, ModalKind, Workspace};
use petunia_module_model::ToolRegistry;

use crate::app_icons;
use crate::tokens;

/// Renderiza a barra lateral vertical de ferramentas.
pub fn draw(ctx: &Context, state: &mut AppState, tools: &ToolRegistry) {
    let width = tokens::TOOLBAR_WIDTH + 8.0;

    SidePanel::left("main_toolbar")
        .exact_width(width)
        .resizable(false)
        .frame(
            egui::Frame::new()
                .fill(tokens::BG_PANEL)
                .stroke(tokens::stroke_border())
                .inner_margin(egui::Margin::symmetric(4, 4)),
        )
        .show(ctx, |ui| {
            ui.add_enabled_ui(!state.is_interacting(), |ui| {
                ScrollArea::vertical()
                    .id_salt("toolbar_scroll_area")
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing = vec2(0.0, 3.0);

                        match state.workspace {
                            Workspace::Model => draw_model_tools(ui, state, tools),
                            Workspace::Paint => draw_paint_tools(ui, state),
                            Workspace::Uv => draw_uv_tools(ui, state),
                            Workspace::Export => {}
                        }
                    });
            });
        });
}

fn draw_model_tools(ui: &mut egui::Ui, state: &mut AppState, tools: &ToolRegistry) {
    // 1. Ferramentas Primárias de Interação e Transformação (Ícones Rasterizados do Figma)
    let primary_tools = [
        ("select_box", "Select Box", "B", "Seleção em Caixa · B"),
        (
            "cursor_3d",
            "3D Cursor",
            "Shift+RMB",
            "3D Cursor · Shift+RMB",
        ),
        ("move", "Move", "G", "Transladar · G"),
        ("rotate", "Rotate", "R", "Rotacionar · R"),
        ("scale", "Scale", "S", "Escalar · S"),
        (
            "transform",
            "Transform",
            "T",
            "Gizmo de Transformação Combinado · T",
        ),
        ("annotate", "Annotate", "D", "Anotação e Rascunho · D"),
        ("measure", "Measure", "M", "Régua e Medição · M"),
        (
            "add_primitive",
            "Add Cube",
            "Shift+A",
            "Adicionar Primitiva · Shift+A",
        ),
    ];

    for (id, label, _key, hint) in primary_tools {
        let is_active = match id {
            "move" => state.active_tool == "transform" && state.gizmo_mode == ModalKind::Move,
            "rotate" => state.active_tool == "transform" && state.gizmo_mode == ModalKind::Rotate,
            "scale" => state.active_tool == "transform" && state.gizmo_mode == ModalKind::Scale,
            "select_box" => state.active_tool == "select" || state.active_tool == "select_box",
            "add_primitive" => state.active_tool == "primitives",
            _ => state.active_tool == id,
        };

        if app_icons::toolbar_button(ui, id, label, is_active, true)
            .on_hover_text(hint)
            .clicked()
        {
            match id {
                "move" => {
                    state.active_tool = "transform".into();
                    state.gizmo_mode = ModalKind::Move;
                }
                "rotate" => {
                    state.active_tool = "transform".into();
                    state.gizmo_mode = ModalKind::Rotate;
                }
                "scale" => {
                    state.active_tool = "transform".into();
                    state.gizmo_mode = ModalKind::Scale;
                }
                "select_box" => {
                    state.active_tool = "select".into();
                }
                "add_primitive" => {
                    state.active_tool = "primitives".into();
                }
                _ => {
                    state.active_tool = id.into();
                }
            }
            state.pending_modal = None;
            state.mark_dirty();
        }
    }

    ui.add_space(3.0);
    ui.separator();
    ui.add_space(3.0);

    // 2. Ferramentas de Modelagem Poligonal (Extrude, Inset, Bevel, Loop Cut, Knife, etc.)
    let mesh_tools = [
        ("extrude", "Extrude", "Extrusão de Faces · E"),
        ("inset", "Inset", "Inserção de Faces (Inset) · I"),
        ("bevel", "Bevel", "Chanfro / Bisel (Bevel) · Ctrl+B"),
        ("loop_cut", "Loop Cut", "Corte em Anel (Loop Cut) · Ctrl+R"),
        ("knife", "Knife", "Faca de Corte (Knife) · K"),
        ("pushpull", "Push/Pull", "Empurrar / Puxar Faces"),
        ("slice", "Slice", "Fatiar Geometria (Slice)"),
        ("subdivide", "Subdivide", "Subdividir Polígonos"),
        ("draw_profile", "Draw Profile", "Desenhar Perfil 2D"),
    ];

    for (id, label, hint) in mesh_tools {
        let is_active = state.active_tool == id;
        let final_label = if let Some(tool) = tools.get(id) {
            state.t(tool.label_key())
        } else {
            label.to_string()
        };

        if app_icons::toolbar_button(ui, id, &final_label, is_active, true)
            .on_hover_text(hint)
            .clicked()
        {
            state.active_tool = id.into();
            state.pending_modal = None;
            state.mark_dirty();
        }
    }
}

fn draw_paint_tools(ui: &mut egui::Ui, state: &mut AppState) {
    let is_active = state.active_tool == "paint";
    if app_icons::toolbar_button(ui, "paint", "Brush", is_active, true)
        .on_hover_text("Pincel de Textura e Vértices")
        .clicked()
    {
        state.active_tool = "paint".into();
        state.mark_dirty();
    }
}

fn draw_uv_tools(ui: &mut egui::Ui, state: &mut AppState) {
    let is_active = state.active_tool == "select";
    if app_icons::toolbar_button(ui, "select_box", "Select", is_active, true)
        .on_hover_text("Seleção de UVs")
        .clicked()
    {
        state.active_tool = "select".into();
        state.mark_dirty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolbar_renders_without_panic() {
        let ctx = Context::default();
        let mut state = AppState::new("en");
        let tools = ToolRegistry::default();

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            draw(ctx, &mut state, &tools);
        });
    }
}
