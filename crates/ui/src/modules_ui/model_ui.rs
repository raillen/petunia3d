//! Painel contextual das ferramentas de modelagem (`model_ui`).
//! Renderiza sliders, botões e controles de acordo com o `tool_id` ativo.

use egui::Ui;
use petunia_core::{AppState, ClearSelectionCmd, InvertSelectionCmd, SelectAllCmd, SelectMode};

pub fn draw_tool_panel(ui: &mut Ui, state: &mut AppState, tool_id: &str) {
    match tool_id {
        "select" => draw_select(ui, state),
        "paint" => draw_paint(ui, state),
        "primitives" => draw_primitives(ui, state),
        "transform" => draw_transform(ui, state),
        "extrude" => draw_extrude(ui, state),
        "inset" => draw_inset(ui, state),
        "bevel" => draw_bevel(ui, state),
        "pushpull" => draw_pushpull(ui, state),
        "subdivide" => draw_subdivide(ui, state),
        "slice" => draw_slice(ui, state),
        "mirror" => draw_mirror(ui, state),
        "connect" => draw_connect(ui, state),
        "merge" => draw_merge(ui, state),
        "dissolve" => draw_dissolve(ui, state),
        "draw_profile" => draw_profile(ui, state),
        _ => {
            ui.label(state.t("ui.no_tool"));
        }
    }
}

fn draw_select(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.select");
    let l_v = state.t("modes.vertex");
    let l_e = state.t("modes.edge");
    let l_f = state.t("modes.face");
    let l_all = state.t("actions.select_all");
    let l_none = state.t("actions.deselect");
    ui.label(l_title);
    ui.horizontal(|ui| {
        if ui
            .selectable_label(state.select_mode == SelectMode::Vertex, l_v)
            .clicked()
        {
            state.select_mode = SelectMode::Vertex;
            state.mark_dirty();
        }
        if ui
            .selectable_label(state.select_mode == SelectMode::Edge, l_e)
            .clicked()
        {
            state.select_mode = SelectMode::Edge;
            if let Some(m) = state.project.active_mesh_mut() {
                m.sync_edge_selection_from_verts();
            }
            state.mark_dirty();
        }
        if ui
            .selectable_label(state.select_mode == SelectMode::Face, l_f)
            .clicked()
        {
            state.select_mode = SelectMode::Face;
            state.mark_dirty();
        }
    });
    ui.horizontal(|ui| {
        if ui.button(l_all).clicked() {
            let _ = state.dispatch(&SelectAllCmd);
        }
        if ui.button(l_none).clicked() {
            let _ = state.dispatch(&ClearSelectionCmd);
        }
        if ui.button(state.t("actions.invert")).clicked() {
            let _ = state.dispatch(&InvertSelectionCmd);
        }
    });
}

fn draw_paint(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.paint");
    let l_radius = state.t("paint.radius");
    let l_strength = state.t("paint.strength");
    let l_hint = state.t("hints.paint");
    ui.label(l_title);
    let mut c = state.paint_color;
    if ui.color_edit_button_rgb(&mut c).changed() {
        state.checkpoint("brush color");
        state.paint_color = c;
        if let Some(o) = state.project.active_mut() {
            o.base_color = c;
        }
        state.mark_dirty();
    }
    if ui
        .add(egui::Slider::new(&mut state.paint_radius, 0.1..=3.0).text(l_radius))
        .changed()
    {
        state.mark_dirty();
    }
    if ui
        .add(egui::Slider::new(&mut state.paint_strength, 0.05..=1.0).text(l_strength))
        .changed()
    {
        state.mark_dirty();
    }
    ui.label(l_hint);
}

fn draw_primitives(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.primitives");
    ui.label(l_title);
    // Dez espécies (grupos do menu Add); cada uma abre a sessão de criação.
    let items: Vec<(&str, String)> = vec![
        ("Cube", state.t("prims.cube")),
        ("Plane", state.t("prims.plane")),
        ("Wedge", state.t("prims.wedge")),
        ("Cylinder8", state.t("prims.cylinder")),
        ("Cone", state.t("prims.cone")),
        ("Circle", state.t("prims.circle")),
        ("Torus", state.t("prims.torus")),
        ("Sphere", state.t("prims.sphere")),
        ("Icosphere", state.t("prims.icosphere")),
        ("Capsule", state.t("prims.capsule")),
    ];
    for (name, label) in items {
        if ui.button(label).clicked() {
            petunia_module_model::PrimitivesTool::add_primitive(state, name);
        }
    }
}

fn draw_transform(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.transform");
    let l_move = state.t("actions.move");
    let l_scale = state.t("actions.scale");
    let l_apply = state.t("actions.apply_scale");
    let l_dup = state.t("actions.duplicate");
    let l_del = state.t("actions.delete");
    ui.label(l_title);
    let mut moved = false;
    moved |= ui
        .add(egui::Slider::new(&mut state.transform_delta[0], -3.0..=3.0).text("X"))
        .changed();
    moved |= ui
        .add(egui::Slider::new(&mut state.transform_delta[1], -3.0..=3.0).text("Y"))
        .changed();
    moved |= ui
        .add(egui::Slider::new(&mut state.transform_delta[2], -3.0..=3.0).text("Z"))
        .changed();
    if moved {
        state.mark_dirty();
    }
    if ui.button(l_move).clicked() {
        petunia_module_model::TransformTool::apply_move(state);
    }
    ui.add(egui::Slider::new(&mut state.transform_scale, 0.1..=3.0).text(l_scale));
    if ui.button(l_apply).clicked() {
        petunia_module_model::TransformTool::apply_scale(state);
    }
    ui.separator();
    ui.horizontal(|ui| {
        if ui.button(l_dup).clicked() {
            petunia_module_model::TransformTool::apply_duplicate(state);
        }
        if ui.button(l_del).clicked() {
            petunia_module_model::TransformTool::apply_delete(state);
        }
    });
}

fn draw_extrude(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.extrude");
    let l_dist = state.t("actions.distance");
    let l_go = state.t("actions.extrude");
    let l_individual = state.t("actions.extrude_individual");
    ui.label(l_title);
    if ui
        .add(egui::Slider::new(&mut state.extrude_dist, -2.0..=2.0).text(l_dist))
        .changed()
    {
        state.mark_dirty();
    }
    ui.horizontal(|ui| {
        if ui.button(l_go).clicked() {
            petunia_module_model::ExtrudeTool::apply(state);
        }
        if ui.button(l_individual).on_hover_text("Alt+E").clicked() {
            petunia_module_model::ExtrudeTool::apply_individual(state);
        }
    });
}

fn draw_inset(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.inset");
    let l_factor = state.t("actions.factor");
    let l_go = state.t("actions.inset");
    ui.label(l_title);
    if ui
        .add(egui::Slider::new(&mut state.inset_factor, 0.0..=0.9).text(l_factor))
        .changed()
    {
        state.mark_dirty();
    }
    if ui.button(l_go).clicked() {
        petunia_module_model::InsetTool::apply(state);
    }
}

fn draw_bevel(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.bevel");
    let l_amount = state.t("actions.amount");
    let l_go = state.t("actions.bevel");
    ui.label(l_title);
    if ui
        .add(egui::Slider::new(&mut state.bevel_amount, 0.01..=1.0).text(l_amount))
        .changed()
    {
        state.mark_dirty();
    }
    if ui.button(l_go).clicked() {
        petunia_module_model::BevelTool::apply(state);
    }
}

fn draw_pushpull(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.pushpull");
    let l_dist = state.t("actions.distance");
    let l_go = state.t("actions.pushpull");
    ui.label(l_title);
    if ui
        .add(egui::Slider::new(&mut state.push_dist, -2.0..=2.0).text(l_dist))
        .changed()
    {
        state.mark_dirty();
    }
    if ui.button(l_go).clicked() {
        petunia_module_model::PushPullTool::apply(state);
    }
}

fn draw_subdivide(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.subdivide");
    let l_sub = state.t("actions.subdivide");
    let l_tri = state.t("actions.triangulate");
    ui.label(l_title);
    if ui.button(l_sub).clicked() {
        petunia_module_model::SubdivideTool::apply_subdivide(state);
    }
    if ui.button(l_tri).clicked() {
        petunia_module_model::SubdivideTool::apply_triangulate(state);
    }
}

fn draw_slice(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.slice");
    let l_go = state.t("actions.slice");
    ui.label(l_title);

    ui.horizontal(|ui| {
        if ui.button("Slice X").clicked() {
            petunia_module_model::SliceTool::apply_slice(state, glam::Vec3::X, false);
        }
        if ui.button("Slice Y").clicked() {
            petunia_module_model::SliceTool::apply_slice(state, glam::Vec3::Y, false);
        }
        if ui.button("Slice Z").clicked() {
            petunia_module_model::SliceTool::apply_slice(state, glam::Vec3::Z, false);
        }
    });

    if ui.button(format!("{l_go} (Cap)")).clicked() {
        let cam_dir = state.camera.forward();
        petunia_module_model::SliceTool::apply_slice(state, cam_dir, true);
    }
}

fn draw_mirror(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.mirror");
    let l_go = state.t("actions.mirror");
    let l_weld = state.t("actions.weld_eps");
    ui.label(l_title);
    ui.horizontal(|ui| {
        for (ax, name) in [(0, "X"), (1, "Y"), (2, "Z")] {
            if ui.selectable_label(state.mirror_axis == ax, name).clicked() {
                state.mirror_axis = ax;
                state.mark_dirty();
            }
        }
    });
    if ui
        .add(egui::Slider::new(&mut state.mirror_weld, 0.0..=0.05).text(l_weld))
        .changed()
    {
        state.mark_dirty();
    }
    if ui.button(l_go).clicked() {
        petunia_module_model::MirrorTool::apply(state);
    }
}

fn draw_connect(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.connect");
    let l_go = state.t("actions.connect");
    ui.label(l_title);
    if ui.button(l_go).clicked() {
        petunia_module_model::ConnectTool::apply(state);
    }
}

fn draw_merge(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.merge");
    let l_go = state.t("actions.merge_center");
    ui.label(l_title);
    if ui.button(l_go).clicked() {
        petunia_module_model::MergeTool::apply(state);
    }
}

fn draw_dissolve(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.dissolve");
    let l_go = state.t("actions.dissolve");
    ui.label(l_title);
    if ui.button(l_go).clicked() {
        petunia_module_model::DissolveTool::apply(state);
    }
}

fn draw_profile(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.draw_profile");
    let l_snap = state.t("profile.snap");
    let l_depth = state.t("profile.depth");
    let l_seg = state.t("profile.segments");
    let l_close = state.t("profile.close");
    let l_ext = state.t("profile.gen_extrude");
    let l_rev = state.t("profile.gen_revolve");
    let l_clear = state.t("profile.clear");
    let l_undo_pt = state.t("profile.undo_pt");
    ui.label(l_title);
    ui.label(format!(
        "{}: {}",
        state.t("profile.points"),
        state.profile.points.len()
    ));
    let mut snap = state.profile.snap;
    if ui.checkbox(&mut snap, l_snap).changed() {
        state.profile.snap = snap;
        state.mark_dirty();
    }
    if ui
        .add(egui::Slider::new(&mut state.profile.depth, 0.05..=8.0).text(l_depth))
        .changed()
    {
        state.mark_dirty();
    }
    ui.add(egui::Slider::new(&mut state.profile.revolve_segments, 3..=48).text(l_seg));
    ui.horizontal(|ui| {
        if ui.button(l_close).clicked() {
            if state.profile.points.len() >= 3 {
                state.profile.closed = true;
            }
            state.mark_dirty();
        }
        if ui.button(l_undo_pt).clicked() {
            state.profile.points.pop();
            state.profile.closed = false;
            state.mark_dirty();
        }
        if ui.button(l_clear).clicked() {
            state.profile.clear();
            state.mark_dirty();
        }
    });
    ui.horizontal(|ui| {
        if ui.button(l_ext).clicked() {
            petunia_module_model::draw_profile::generate_extrude(state);
        }
        if ui.button(l_rev).clicked() {
            petunia_module_model::draw_profile::generate_revolve(state);
        }
    });
    if state.profile.closed {
        match petunia_mesh::triangulate::ear_clip(&state.profile.points) {
            Ok(t) => {
                ui.small(format!("{}: {}", state.t("profile.tris"), t.len()));
            }
            Err(e) => {
                ui.colored_label(egui::Color32::LIGHT_RED, e);
            }
        }
    }
}
