//! Barra lateral vertical de ferramentas do Petunia3D (`Toolbar`).
//! Renderiza as ferramentas com os ícones extraídos da Golden Reference (`assets/ui/icons/toolbar/`)
//! com 40px de largura e suporte a destaque ativo e atalhos de teclado.

use egui::{ScrollArea, Ui, vec2};
use petunia_core::{AppState, ModalKind, Workspace};
use petunia_module_model::ToolRegistry;

use crate::icon_registry::PetuniaIcon;
use crate::tokens;
use crate::widgets::PetuniaToolbarButton;

/// Renderiza a barra lateral vertical de ferramentas.
pub fn draw(ui: &mut Ui, state: &mut AppState, tools: &ToolRegistry) {
    let min_width = tokens::TOOLBAR_MIN_WIDTH;
    let max_width = tokens::TOOLBAR_MAX_WIDTH;

    let toolbar_resp = egui::Panel::left("main_toolbar")
        .default_size(min_width)
        .size_range(min_width..=max_width)
        .resizable(true)
        .frame(
            egui::Frame::new()
                .fill(tokens::bg_panel(state))
                .stroke(tokens::stroke_border_dyn(state))
                .inner_margin(egui::Margin::symmetric(4, 4)),
        )
        .show(ui, |ui| {
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
                            Workspace::Animate => draw_animate_tools(ui, state, compact),
                        }
                    });
            });
        });
    crate::regions::record(
        ui.ctx(),
        crate::regions::RegionSlot::LeftTools,
        toolbar_resp.response.rect,
    );
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

    // 1. Ferramentas Primárias de Interação e Transformação (ordem canônica:
    // seleção, cursor, trio Move/Rotate/Scale, transform livre).
    select_box_button(ui, state, compact);
    cursor_3d_button(ui, state, compact);
    transform_gizmo_buttons(ui, state, compact);
    free_transform_button(ui, state, compact);

    ui.add_space(3.0);
    ui.separator();
    ui.add_space(3.0);

    // 2. Ferramentas de Inspeção e Anotação Tridimensional
    let inspection_tools = [
        (PetuniaIcon::Measure, "measure", "M"),
        (PetuniaIcon::Annotate, "annotate", "D"),
    ];

    for (icon, id, key) in inspection_tools {
        let is_active = state.active_tool == id;
        let label = state.t(&format!("tools.{id}"));
        let hint = format!("{label} · [{key}]");
        if PetuniaToolbarButton::new(icon, &label)
            .selected(is_active)
            .compact(compact)
            .tooltip(&hint)
            .show(ui)
            .clicked()
        {
            state.active_tool = id.into();
            state.pending_modal = None;
            state.mark_dirty();
        }
    }

    // 3. Ferramentas Especializadas de Modelagem de Malha (Exibidas no Modo de Edição)
    if state.mode == petunia_core::EditMode::Edit {
        ui.add_space(3.0);
        ui.separator();
        ui.add_space(3.0);

        for (icon, id, key) in [
            (PetuniaIcon::Extrude, "extrude", "E"),
            (PetuniaIcon::Inset, "inset", "I"),
            (PetuniaIcon::Bevel, "bevel", "Ctrl+B"),
            (PetuniaIcon::LoopCut, "loop_cut", "Ctrl+R"),
            (PetuniaIcon::Knife, "knife", "K"),
            (PetuniaIcon::PushPull, "pushpull", ""),
            (PetuniaIcon::Slice, "slice", ""),
            (PetuniaIcon::Subdivide, "subdivide", ""),
            (PetuniaIcon::DrawProfile, "draw_profile", ""),
        ] {
            let is_active = state.active_tool == id;
            let label = state.t(&format!("tools.{id}"));
            let hint = if key.is_empty() {
                label.clone()
            } else {
                format!("{label} · [{key}]")
            };
            if PetuniaToolbarButton::new(icon, &label)
                .selected(is_active)
                .compact(compact)
                .tooltip(&hint)
                .show(ui)
                .clicked()
            {
                state.active_tool = id.into();
                state.pending_modal = None;
                state.mark_dirty();
            }
        }
    }
}

/// Botão de seleção por caixa (semântica única, Model objeto + Animate).
fn select_box_button(ui: &mut egui::Ui, state: &mut AppState, compact: bool) {
    let is_active = state.active_tool == "select" || state.active_tool == "select_box";
    let label = state.t("tools.select_box");
    let hint = format!("{label} · [B]");
    if PetuniaToolbarButton::new(PetuniaIcon::SelectBox, &label)
        .selected(is_active)
        .compact(compact)
        .tooltip(&hint)
        .show(ui)
        .clicked()
    {
        state.active_tool = "select".into();
        state.pending_modal = None;
        state.mark_dirty();
    }
}

fn cursor_3d_button(ui: &mut egui::Ui, state: &mut AppState, compact: bool) {
    let label = state.t("tools.cursor_3d");
    let hint = format!("{label} · [Shift+RMB]");
    if PetuniaToolbarButton::new(PetuniaIcon::Cursor3D, &label)
        .selected(state.active_tool == "cursor_3d")
        .compact(compact)
        .tooltip(&hint)
        .show(ui)
        .clicked()
    {
        state.active_tool = "cursor_3d".into();
        state.pending_modal = None;
        state.mark_dirty();
    }
}

/// Trio Move/Rotate/Scale por gizmo: mesma semântica no Model (objeto) e no
/// Animate (pose). Dono único do trio (§3.2).
fn transform_gizmo_buttons(ui: &mut egui::Ui, state: &mut AppState, compact: bool) {
    for (icon, id, key, gizmo) in [
        (PetuniaIcon::Move, "move", "G", ModalKind::Move),
        (PetuniaIcon::Rotate, "rotate", "R", ModalKind::Rotate),
        (PetuniaIcon::Scale, "scale", "S", ModalKind::Scale),
    ] {
        let is_active = state.active_tool == "transform" && state.gizmo_mode == gizmo;
        let label = state.t(&format!("tools.{id}"));
        let hint = format!("{label} · [{key}]");
        if PetuniaToolbarButton::new(icon, &label)
            .selected(is_active)
            .compact(compact)
            .tooltip(&hint)
            .show(ui)
            .clicked()
        {
            state.active_tool = "transform".into();
            state.gizmo_mode = gizmo;
            state.pending_modal = None;
            state.mark_dirty();
        }
    }
}

fn free_transform_button(ui: &mut egui::Ui, state: &mut AppState, compact: bool) {
    let label = state.t("tools.transform");
    let hint = format!("{label} · [T]");
    if PetuniaToolbarButton::new(PetuniaIcon::Transform, &label)
        .selected(state.active_tool == "transform")
        .compact(compact)
        .tooltip(&hint)
        .show(ui)
        .clicked()
    {
        state.active_tool = "transform".into();
        state.pending_modal = None;
        state.mark_dirty();
    }
}

/// Paleta Animate (Wave 3): seleção + transform de pose. Sem ferramentas
/// fictícias: só ações que operam hoje (objetos/armatures via gizmo).
fn draw_animate_tools(ui: &mut egui::Ui, state: &mut AppState, compact: bool) {
    select_box_button(ui, state, compact);
    transform_gizmo_buttons(ui, state, compact);
}

fn draw_paint_tools(ui: &mut egui::Ui, state: &mut AppState, compact: bool) {
    let is_active = state.active_tool == "paint";
    let label = state.t("tools.paint");
    if PetuniaToolbarButton::new(PetuniaIcon::Custom("paint"), &label)
        .selected(is_active)
        .compact(compact)
        .tooltip(&label)
        .show(ui)
        .clicked()
    {
        state.active_tool = "paint".into();
        state.mark_dirty();
    }
}

fn draw_uv_tools(ui: &mut egui::Ui, state: &mut AppState, compact: bool) {
    let is_active = state.active_tool == "select";
    let label = state.t("tools.select");
    if PetuniaToolbarButton::new(PetuniaIcon::SelectBox, &label)
        .selected(is_active)
        .compact(compact)
        .tooltip(&label)
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
        let ctx = egui::Context::default();
        let tools = ToolRegistry::default();

        for mode in [EditMode::Object, EditMode::Edit, EditMode::TexturePaint] {
            let mut state = AppState::new("en");
            state.mode = mode;

            ctx.run_ui(egui::RawInput::default(), |ui| {
                draw(ui, &mut state, &tools);
            })
            .textures_delta
            .clear();
        }
    }

    #[test]
    fn test_mesh_tools_hidden_in_object_mode_and_normalizes_tool() {
        let ctx = egui::Context::default();
        let tools = ToolRegistry::default();
        let mut state = AppState::new("en");
        state.mode = EditMode::Object;
        state.active_tool = "extrude".into();

        ctx.run_ui(egui::RawInput::default(), |ui| {
            draw(ui, &mut state, &tools);
        })
        .textures_delta
        .clear();

        // Deveria normalizar para a ferramenta padrão de seleção em Object Mode
        assert_eq!(state.active_tool, "select");
    }

    #[test]
    fn test_mesh_tools_allowed_in_edit_mode() {
        let ctx = egui::Context::default();
        let tools = ToolRegistry::default();
        let mut state = AppState::new("en");
        state.mode = EditMode::Edit;
        state.active_tool = "extrude".into();

        ctx.run_ui(egui::RawInput::default(), |ui| {
            draw(ui, &mut state, &tools);
        })
        .textures_delta
        .clear();

        // No Edit Mode, a ferramenta de malha permanece ativa
        assert_eq!(state.active_tool, "extrude");
    }

    #[test]
    fn test_toolbar_renders_with_compact_and_expanded_widths() {
        let tools = ToolRegistry::default();

        // 1. Largura compacta padrão
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");
        state.mode = EditMode::Edit;
        ctx.run_ui(egui::RawInput::default(), |ui| {
            draw(ui, &mut state, &tools);
        })
        .textures_delta
        .clear();

        // 2. Toolbar com dimensões expandidas para exibir ícones + rótulos de texto
        let ctx = egui::Context::default();
        let raw_input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1920.0, 1080.0),
            )),
            ..Default::default()
        };
        ctx.run_ui(raw_input, |ui| {
            draw(ui, &mut state, &tools);
        })
        .textures_delta
        .clear();
    }
}
