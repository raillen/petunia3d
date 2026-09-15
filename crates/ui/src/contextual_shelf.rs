//! Barra Contextual Horizontal do Viewport (`Contextual Modeling Shelf`).
//! Posicionada na base inferior do Viewport 3D, reagindo dinamicamente
//! ao Workspace e ao Modo de Edição ativo (Object, Edit, Paint, UV, Animate).

use egui::{Rect, Response, RichText, Ui, pos2, vec2};
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

    // Calcula largura dinâmica da shelf com medição real do frame anterior
    let shelf_height = 36.0;
    let bottom_margin = 12.0;

    let (estimated_w, content_closure) = match state.workspace {
        Workspace::Model => {
            if state.mode == EditMode::Edit {
                (780.0_f32, ShelfContent::ModelEdit)
            } else {
                (700.0_f32, ShelfContent::ModelObject)
            }
        }
        Workspace::Paint => (480.0_f32, ShelfContent::Paint),
        Workspace::Uv => (440.0_f32, ShelfContent::Uv),
        Workspace::Animate => (640.0_f32, ShelfContent::Animate),
    };

    let shelf_id = ui.make_persistent_id("contextual_shelf_width");
    let shelf_w = ui.ctx().data(|d| {
        d.get_temp::<f32>(shelf_id)
            .unwrap_or(estimated_w)
            .max(estimated_w)
            .min(screen_w - 24.0)
    });

    let shelf_rect = Rect::from_center_size(
        egui::pos2(
            viewport_rect.center().x,
            viewport_rect.max.y - bottom_margin - shelf_height * 0.5,
        ),
        vec2(shelf_w, shelf_height),
    );

    // Fundo da cápsula com tokens dinâmicos do tema
    let painter = ui.painter();
    painter.rect_filled(
        shelf_rect,
        tokens::RADIUS_PILL,
        tokens::bg_panel_header(state),
    );
    painter.rect_stroke(
        shelf_rect,
        tokens::RADIUS_PILL,
        tokens::stroke_border_dyn(state),
        egui::StrokeKind::Inside,
    );

    // Escudo de eventos: impede que cliques na shelf atravessem para o raycasting da cena 3D
    let _ = ui.allocate_rect(shelf_rect, egui::Sense::click_and_drag());

    // Renderiza controles horizontais centralizados dentro da cápsula com espaço para medição natural
    let max_avail_rect = Rect::from_center_size(
        shelf_rect.center(),
        vec2((screen_w - 24.0).max(shelf_w), shelf_height),
    );
    let mut shelf_ui = ui.new_child(egui::UiBuilder::new().max_rect(max_avail_rect));
    let content_resp = shelf_ui.horizontal_centered(|ui| {
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

    let measured_w = (content_resp.response.rect.width() + 24.0).min(screen_w - 24.0);
    ui.ctx().data_mut(|d| {
        d.insert_temp(shelf_id, measured_w);
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
    // 1. Comandos Principais de Modelagem de Malha
    let modeling_ops = [
        ("extrude", PetuniaIcon::Extrude, "E"),
        ("inset", PetuniaIcon::Inset, "I"),
        ("bevel", PetuniaIcon::Bevel, "Ctrl+B"),
        ("loop_cut", PetuniaIcon::LoopCut, "Ctrl+R"),
        ("knife", PetuniaIcon::Knife, "K"),
    ];

    for (tool_id, icon, shortcut) in modeling_ops {
        let is_active = state.active_tool == tool_id;
        let name = state.t(&format!("tools.{tool_id}"));
        let hint = format!("{name} · [{shortcut}]");
        if pill_button(ui, Some(icon), &name, is_active, &hint).clicked() {
            state.active_tool = tool_id.to_string();
            state.mark_dirty();
        }
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    // 2. Operações Topológicas de Malha
    let sub_name = state.t("actions.subdivide");
    if pill_button(
        ui,
        Some(PetuniaIcon::Subdivide),
        &sub_name,
        false,
        &sub_name,
    )
    .clicked()
    {
        let _ = state.dispatch(&SubdivideSelectionCmd);
    }

    let merge_name = state.t("actions.merge_center");
    if pill_button(
        ui,
        Some(PetuniaIcon::Custom("merge")),
        &merge_name,
        false,
        &merge_name,
    )
    .clicked()
    {
        let _ = state.dispatch(&MergeCenterCmd);
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    // 3. Imagem de Referência
    let ref_name = state.t("ui.refs");
    if pill_button(
        ui,
        Some(PetuniaIcon::ReferenceImage),
        &ref_name,
        false,
        &ref_name,
    )
    .clicked()
    {
        state.ui.show_reference_manager = true;
    }
}

fn draw_model_object_shelf(ui: &mut Ui, state: &mut AppState) {
    // 1. Ferramentas de Transformação de Objeto
    let transforms = [
        (ModalKind::Move, PetuniaIcon::Move, "tools.move", "G"),
        (ModalKind::Rotate, PetuniaIcon::Rotate, "tools.rotate", "R"),
        (ModalKind::Scale, PetuniaIcon::Scale, "tools.scale", "S"),
    ];

    for (gizmo_m, icon, key, shortcut) in transforms {
        let is_active = state.gizmo_mode == gizmo_m;
        let label = state.t(key);
        let hint = format!("{label} · [{shortcut}]");
        if pill_button(ui, Some(icon), &label, is_active, &hint).clicked() {
            state.gizmo_mode = gizmo_m;
            state.mark_dirty();
        }
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    // 2. Criação Rápida de Primitivas na coordenada do 3D Cursor
    let prims = [
        ("Cube", "prims.cube", 0),
        ("Sphere", "prims.sphere", 1),
        ("Cylinder", "prims.cylinder", 2),
        ("Plane", "prims.plane", 3),
    ];

    for (name, key, kind) in prims {
        let label = state.t(key);
        let hint = format!("{} no 3D Cursor", label);
        if pill_button(ui, Some(PetuniaIcon::AddPrimitive), &label, false, &hint).clicked() {
            add_primitive_to_scene(state, kind, name);
        }
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    // 3. Duplicar Objeto
    let dup_label = state.t("ui.duplicate");
    let dup_hint = format!("{dup_label} · [Shift+D]");
    if pill_button(
        ui,
        Some(PetuniaIcon::Duplicate),
        &dup_label,
        false,
        &dup_hint,
    )
    .clicked()
    {
        let _ = state.dispatch(&DuplicateSelectionCmd);
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    // 4. Imagem de Referência
    let ref_label = state.t("ui.refs");
    if pill_button(
        ui,
        Some(PetuniaIcon::ReferenceImage),
        &ref_label,
        false,
        &ref_label,
    )
    .clicked()
    {
        state.ui.show_reference_manager = true;
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
        None,
        "👤 Humanoide",
        false,
        "Adicionar Armature Humanoide",
    )
    .clicked()
    {
        let skel = petunia_project::animation::RigPreset::humanoid(1.0);
        state.project.add_skeleton(skel);
        state.mark_dirty();
    }
    if pill_button(
        ui,
        None,
        "✨ Auto-Rig",
        false,
        "Auto-Rig sobre o modelo ativo",
    )
    .clicked()
    {
        let active = state.project.active;
        if let Some(asset) = state.project.assets.get_mut(active) {
            let skel = petunia_project::animation::auto_fit_humanoid(&asset.mesh);
            let skel_id = skel.id;
            let skin = petunia_project::animation::compute_auto_skin_weights(&asset.mesh, &skel);
            asset.skeleton_id = Some(skel_id);
            asset.skin_data = Some(skin);
            state.project.add_skeleton(skel);
            state.mark_dirty();
        }
    }

    ui.add_space(2.0);
    shelf_separator(ui);
    ui.add_space(2.0);

    if pill_button(
        ui,
        Some(PetuniaIcon::JumpStart),
        "",
        false,
        "Primeiro Frame",
    )
    .clicked()
    {
        state.ui.timeline_frame = state.ui.timeline_start;
        state.mark_dirty();
    }

    let (play_icon, play_label) = if state.ui.timeline_playing {
        (PetuniaIcon::Pause, "Pause")
    } else {
        (PetuniaIcon::Play, "Play")
    };
    if pill_button(
        ui,
        Some(play_icon),
        play_label,
        state.ui.timeline_playing,
        "Iniciar/Pausar animação",
    )
    .clicked()
    {
        state.ui.timeline_playing = !state.ui.timeline_playing;
        state.mark_dirty();
    }

    if pill_button(ui, Some(PetuniaIcon::JumpEnd), "", false, "Último Frame").clicked() {
        state.ui.timeline_frame = state.ui.timeline_end;
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
        egui::DragValue::new(&mut state.ui.timeline_frame)
            .range(state.ui.timeline_start..=state.ui.timeline_end),
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
        ctx.run_ui(egui::RawInput::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                let rect = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), vec2(800.0, 600.0));
                let _ = draw(ui, &mut state, rect);
            });
        })
        .textures_delta
        .clear();
    }
}
