//! Barra lateral vertical de ferramentas do Petunia3D (`Toolbar`).
//! Renderiza as ferramentas com os ícones extraídos da Golden Reference (`assets/ui/icons/toolbar/`)
//! com 40px de largura e suporte a destaque ativo e atalhos de teclado.

use egui::{vec2, Context, ScrollArea, SidePanel};
use petunia_core::{AppState, ModalKind, Workspace};
use petunia_module_model::ToolRegistry;

use crate::icon_registry::PetuniaIcon;
use crate::tokens;
use crate::widgets::PetuniaToolbarButton;

/// Renderiza a barra lateral vertical de ferramentas.
pub fn draw(ctx: &Context, state: &mut AppState, tools: &ToolRegistry) {
    let min_width = tokens::TOOLBAR_MIN_WIDTH;
    let max_width = tokens::TOOLBAR_MAX_WIDTH;

    SidePanel::left("main_toolbar")
        .default_width(min_width)
        .width_range(min_width..=max_width)
        .resizable(true)
        .frame(
            egui::Frame::new()
                .fill(tokens::BG_PANEL)
                .stroke(tokens::stroke_border())
                .inner_margin(egui::Margin::symmetric(4, 4)),
        )
        .show(ctx, |ui| {
            let compact = ui.available_width() < 90.0;
            ui.add_enabled_ui(!state.is_interacting(), |ui| {
                ScrollArea::vertical()
                    .id_salt("toolbar_scroll_area")
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing = vec2(0.0, 3.0);

                        match state.workspace {
                            Workspace::Model => draw_model_tools(ui, state, tools, compact),
                            Workspace::Paint => draw_paint_tools(ui, state, compact),
                            Workspace::Uv => draw_uv_tools(ui, state, compact),
                            Workspace::Animate => {}
                        }
                    });
            });
        });
}

const MESH_TOOLS: &[(PetuniaIcon, &str, &str, &str)] = &[
    (
        PetuniaIcon::Extrude,
        "extrude",
        "Extrude",
        "Extrusão de Faces · E",
    ),
    (
        PetuniaIcon::Inset,
        "inset",
        "Inset",
        "Inserção de Faces (Inset) · I",
    ),
    (
        PetuniaIcon::Bevel,
        "bevel",
        "Bevel",
        "Chanfro / Bisel (Bevel) · Ctrl+B",
    ),
    (
        PetuniaIcon::LoopCut,
        "loop_cut",
        "Loop Cut",
        "Corte em Anel (Loop Cut) · Ctrl+R",
    ),
    (
        PetuniaIcon::Knife,
        "knife",
        "Knife",
        "Faca de Corte (Knife) · K",
    ),
    (
        PetuniaIcon::PushPull,
        "pushpull",
        "Push/Pull",
        "Empurrar / Puxar Faces",
    ),
    (
        PetuniaIcon::Slice,
        "slice",
        "Slice",
        "Fatiar Geometria (Slice)",
    ),
    (
        PetuniaIcon::Subdivide,
        "subdivide",
        "Subdivide",
        "Subdividir Polígonos",
    ),
    (
        PetuniaIcon::DrawProfile,
        "draw_profile",
        "Draw Profile",
        "Desenhar Perfil 2D",
    ),
];

fn draw_model_tools(ui: &mut egui::Ui, state: &mut AppState, _tools: &ToolRegistry, compact: bool) {
    // Normalização defensiva: se não estiver no modo de edição e a ferramenta ativa
    // for exclusiva de malha, reverte para a seleção básica de objetos.
    if state.mode != petunia_core::EditMode::Edit
        && MESH_TOOLS
            .iter()
            .any(|(_, id, _, _)| *id == state.active_tool)
    {
        state.active_tool = "select".into();
        state.mark_dirty();
    }

    // 1. Ferramentas Primárias de Interação e Transformação
    let primary_tools = [
        (
            PetuniaIcon::SelectBox,
            "select_box",
            "Select Box",
            "B",
            "Seleção em Caixa · B",
        ),
        (
            PetuniaIcon::Cursor3D,
            "cursor_3d",
            "3D Cursor",
            "Shift+RMB",
            "3D Cursor · Shift+RMB",
        ),
        (PetuniaIcon::Move, "move", "Move", "G", "Transladar · G"),
        (
            PetuniaIcon::Rotate,
            "rotate",
            "Rotate",
            "R",
            "Rotacionar · R",
        ),
        (PetuniaIcon::Scale, "scale", "Scale", "S", "Escalar · S"),
        (
            PetuniaIcon::Transform,
            "transform",
            "Transform",
            "T",
            "Gizmo de Transformação Combinado · T",
        ),
    ];

    for (icon, id, label, _key, hint) in primary_tools {
        let is_active = match id {
            "move" => state.active_tool == "transform" && state.gizmo_mode == ModalKind::Move,
            "rotate" => state.active_tool == "transform" && state.gizmo_mode == ModalKind::Rotate,
            "scale" => state.active_tool == "transform" && state.gizmo_mode == ModalKind::Scale,
            "select_box" => state.active_tool == "select" || state.active_tool == "select_box",
            _ => state.active_tool == id,
        };

        if PetuniaToolbarButton::new(icon, label)
            .selected(is_active)
            .compact(compact)
            .tooltip(hint)
            .show(ui)
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

    // 2. Ferramentas de Inspeção e Anotação Tridimensional
    let inspection_tools = [
        (
            PetuniaIcon::Measure,
            "measure",
            "Measure",
            "M",
            "Régua e Medição 3D · M",
        ),
        (
            PetuniaIcon::Annotate,
            "annotate",
            "Annotate",
            "D",
            "Anotação e Rascunho 3D · D",
        ),
    ];

    for (icon, id, label, _key, hint) in inspection_tools {
        let is_active = state.active_tool == id;
        if PetuniaToolbarButton::new(icon, label)
            .selected(is_active)
            .compact(compact)
            .tooltip(hint)
            .show(ui)
            .clicked()
        {
            state.active_tool = id.into();
            state.pending_modal = None;
            state.mark_dirty();
        }
    }
}

fn draw_paint_tools(ui: &mut egui::Ui, state: &mut AppState, compact: bool) {
    let is_active = state.active_tool == "paint";
    if PetuniaToolbarButton::new(PetuniaIcon::Custom("paint"), "Brush")
        .selected(is_active)
        .compact(compact)
        .tooltip("Pincel de Textura e Vértices")
        .show(ui)
        .clicked()
    {
        state.active_tool = "paint".into();
        state.mark_dirty();
    }
}

fn draw_uv_tools(ui: &mut egui::Ui, state: &mut AppState, compact: bool) {
    let is_active = state.active_tool == "select";
    if PetuniaToolbarButton::new(PetuniaIcon::SelectBox, "Select")
        .selected(is_active)
        .compact(compact)
        .tooltip("Seleção de UVs")
        .show(ui)
        .clicked()
    {
        state.active_tool = "select".into();
        state.mark_dirty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petunia_core::EditMode;

    #[test]
    fn test_toolbar_renders_without_panic_in_all_modes() {
        let ctx = Context::default();
        let tools = ToolRegistry::default();

        for mode in [EditMode::Object, EditMode::Edit, EditMode::TexturePaint] {
            let mut state = AppState::new("en");
            state.mode = mode;

            let _ = ctx.run(egui::RawInput::default(), |ctx| {
                draw(ctx, &mut state, &tools);
            });
        }
    }

    #[test]
    fn test_mesh_tools_hidden_in_object_mode_and_normalizes_tool() {
        let ctx = Context::default();
        let tools = ToolRegistry::default();
        let mut state = AppState::new("en");
        state.mode = EditMode::Object;
        state.active_tool = "extrude".into();

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            draw(ctx, &mut state, &tools);
        });

        // Deveria normalizar para a ferramenta padrão de seleção em Object Mode
        assert_eq!(state.active_tool, "select");
    }

    #[test]
    fn test_mesh_tools_allowed_in_edit_mode() {
        let ctx = Context::default();
        let tools = ToolRegistry::default();
        let mut state = AppState::new("en");
        state.mode = EditMode::Edit;
        state.active_tool = "extrude".into();

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            draw(ctx, &mut state, &tools);
        });

        // No Edit Mode, a ferramenta de malha permanece ativa
        assert_eq!(state.active_tool, "extrude");
    }

    #[test]
    fn test_toolbar_renders_with_compact_and_expanded_widths() {
        let tools = ToolRegistry::default();

        // 1. Largura compacta padrão
        let ctx = Context::default();
        let mut state = AppState::new("en");
        state.mode = EditMode::Edit;
        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            draw(ctx, &mut state, &tools);
        });

        // 2. Toolbar com dimensões expandidas para exibir ícones + rótulos de texto
        let ctx = Context::default();
        let raw_input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1920.0, 1080.0),
            )),
            ..Default::default()
        };
        let _ = ctx.run(raw_input, |ctx| {
            draw(ctx, &mut state, &tools);
        });
    }
}
