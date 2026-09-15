//! Painel contextual das ferramentas de modelagem (`model_ui`).
//! Renderiza sliders, botões e controles de acordo com o `tool_id` ativo.

use egui::Ui;
use petunia_core::{
    AppState, ClearSelectionCmd, Command, EditMode, InvertSelectionCmd, MergeCenterCmd,
    SelectAllCmd, SelectMode, SymmetrizeCmd, WeldCmd,
};

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
        "symmetrize" => draw_symmetrize(ui, state),
        "dissolve" => draw_dissolve(ui, state),
        "revolve" => draw_revolve(ui, state),
        "draw_profile" => draw_profile(ui, state),
        _ => {
            ui.label(state.t("ui.no_tool"));
        }
    }
}

fn edit_guard(state: &AppState) -> Option<String> {
    if state.project.active_mesh().is_none() {
        Some(state.t("actions.no_mesh"))
    } else if state.mode != EditMode::Edit {
        Some(state.t("actions.need_edit"))
    } else {
        None
    }
}

fn selection_guard(state: &AppState) -> Option<String> {
    if let Some(reason) = edit_guard(state) {
        Some(reason)
    } else if state.selection.is_empty() {
        Some(state.t("actions.need_selection"))
    } else {
        None
    }
}

fn show_disabled_reason(ui: &mut Ui, reason: &str) {
    ui.small(reason);
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
        ("Cylinder", state.t("prims.cylinder")),
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
    ui.small(state.t("hints.extrude"));
    if ui
        .add(egui::Slider::new(&mut state.extrude_dist, -2.0..=2.0).text(l_dist))
        .changed()
    {
        state.mark_dirty();
    }
    let reason = selection_guard(state);
    ui.horizontal(|ui| {
        let resp = ui
            .add_enabled(reason.is_none(), egui::Button::new(l_go))
            .on_hover_text(reason.clone().unwrap_or_else(|| "E".to_string()));
        if resp.clicked() {
            petunia_module_model::ExtrudeTool::apply(state);
        }
        let resp_ind = ui
            .add_enabled(reason.is_none(), egui::Button::new(l_individual))
            .on_hover_text(reason.clone().unwrap_or_else(|| "Alt+E".to_string()));
        if resp_ind.clicked() {
            petunia_module_model::ExtrudeTool::apply_individual(state);
        }
    });
    if let Some(r) = reason {
        show_disabled_reason(ui, &r);
    }
}

fn draw_inset(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.inset");
    let l_factor = state.t("actions.factor");
    let l_go = state.t("actions.inset");
    ui.label(l_title);
    ui.small(state.t("hints.inset"));
    if ui
        .add(egui::Slider::new(&mut state.inset_factor, 0.0..=0.9).text(l_factor))
        .changed()
    {
        state.mark_dirty();
    }
    let reason = selection_guard(state);
    let resp = ui
        .add_enabled(reason.is_none(), egui::Button::new(l_go))
        .on_hover_text(reason.clone().unwrap_or_else(|| "I".to_string()));
    if resp.clicked() {
        petunia_module_model::InsetTool::apply(state);
    }
    if let Some(r) = reason {
        show_disabled_reason(ui, &r);
    }
}

fn draw_bevel(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.bevel");
    let l_amount = state.t("actions.amount");
    let l_go = state.t("actions.bevel");
    ui.label(l_title);
    ui.small(state.t("hints.bevel"));
    if ui
        .add(egui::Slider::new(&mut state.bevel_amount, 0.01..=1.0).text(l_amount))
        .changed()
    {
        state.mark_dirty();
    }
    let mut segs = state.bevel_segments as i64;
    if ui
        .add(
            egui::Slider::new(&mut segs, 1..=4)
                .text(state.t("prims.segments"))
                .step_by(1.0),
        )
        .changed()
    {
        state.bevel_segments = segs.clamp(1, 4) as u32;
        state.mark_dirty();
    }
    let reason = selection_guard(state);
    let resp = ui
        .add_enabled(reason.is_none(), egui::Button::new(l_go))
        .on_hover_text(reason.clone().unwrap_or_else(|| "Ctrl+B".to_string()));
    if resp.clicked() {
        petunia_module_model::BevelTool::apply(state);
    }
    if let Some(r) = reason {
        show_disabled_reason(ui, &r);
    }
}

fn draw_revolve(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.revolve");
    ui.label(l_title);
    ui.small(state.t("hints.revolve"));
    let mut segs = state.revolve_segments as i64;
    if ui
        .add(
            egui::Slider::new(&mut segs, 3..=64)
                .text(state.t("prims.segments"))
                .step_by(1.0),
        )
        .changed()
    {
        state.revolve_segments = segs.clamp(3, 64) as u32;
        state.mark_dirty();
    }
    let l_angle = state.t("actions.angle");
    if ui
        .add(egui::Slider::new(&mut state.revolve_angle, 5.0..=360.0).text(l_angle))
        .changed()
    {
        state.mark_dirty();
    }
    if state.revolve_angle < 359.5 {
        ui.small(state.t("actions.revolve_open_hint"));
    }
    ui.horizontal(|ui| {
        for (ax, name) in [(0, "X"), (1, "Y"), (2, "Z")] {
            if ui
                .selectable_label(state.revolve_axis == ax, name)
                .clicked()
            {
                state.revolve_axis = ax;
                state.mark_dirty();
            }
        }
    });
    let reason = selection_guard(state);
    let resp = ui.add_enabled(
        reason.is_none(),
        egui::Button::new(state.t("actions.revolve")),
    );
    let resp = resp.on_hover_text(reason.clone().unwrap_or_else(|| state.t("hints.revolve")));
    if resp.clicked() {
        let center = state
            .project
            .active_mesh()
            .map(|m| m.selection_center())
            .unwrap_or([0.0, 0.0, 0.0]);
        let cmd = petunia_core::RevolveCmd {
            segments: state.revolve_segments,
            angle_deg: state.revolve_angle,
            axis: state.revolve_axis,
            center,
        };
        if let Err(err) = state.dispatch(&cmd) {
            state.set_status(format!("revolve: {err}"));
        }
    }
    if let Some(r) = reason {
        show_disabled_reason(ui, &r);
    } else {
        ui.small(state.t("actions.revolve_need_edges"));
    }
}

fn draw_pushpull(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.pushpull");
    let l_dist = state.t("actions.distance");
    let l_go = state.t("actions.pushpull");
    ui.label(l_title);
    ui.small(state.t("hints.pushpull"));
    if ui
        .add(egui::Slider::new(&mut state.push_dist, -2.0..=2.0).text(l_dist))
        .changed()
    {
        state.mark_dirty();
    }
    let reason = selection_guard(state);
    let resp = ui
        .add_enabled(reason.is_none(), egui::Button::new(l_go))
        .on_hover_text(reason.clone().unwrap_or_else(|| "P".to_string()));
    if resp.clicked() {
        petunia_module_model::PushPullTool::apply(state);
    }
    if let Some(r) = reason {
        show_disabled_reason(ui, &r);
    }
}

fn draw_subdivide(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.subdivide");
    let l_sub = state.t("actions.subdivide");
    let l_tri = state.t("actions.triangulate");
    ui.label(l_title);
    ui.small(state.t("hints.subdivide"));
    let reason = selection_guard(state);
    ui.horizontal(|ui| {
        let r1 = ui
            .add_enabled(reason.is_none(), egui::Button::new(l_sub))
            .on_hover_text(reason.clone().unwrap_or_else(|| "W".to_string()));
        if r1.clicked() {
            petunia_module_model::SubdivideTool::apply_subdivide(state);
        }
        if ui.button(l_tri).clicked() {
            petunia_module_model::SubdivideTool::apply_triangulate(state);
        }
    });
    if let Some(r) = reason {
        show_disabled_reason(ui, &r);
    }
}

fn draw_slice(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.slice");
    ui.label(l_title);
    ui.small(state.t("hints.slice"));

    let reason = edit_guard(state);
    ui.horizontal(|ui| {
        for (label, axis) in [
            (state.t("actions.slice_x"), glam::Vec3::X),
            (state.t("actions.slice_y"), glam::Vec3::Y),
            (state.t("actions.slice_z"), glam::Vec3::Z),
        ] {
            let resp = ui
                .add_enabled(reason.is_none(), egui::Button::new(label))
                .on_hover_text(reason.clone().unwrap_or_default());
            if resp.clicked() {
                petunia_module_model::SliceTool::apply_slice(state, axis, false);
            }
        }
    });

    let cap_label = state.t("actions.slice_cap");
    let resp = ui
        .add_enabled(reason.is_none(), egui::Button::new(cap_label.clone()))
        .on_hover_text(reason.clone().unwrap_or_else(|| cap_label.clone()));
    if resp.clicked() {
        let cam_dir = state.camera.forward();
        petunia_module_model::SliceTool::apply_slice(state, cam_dir, true);
    }
    if let Some(r) = reason {
        show_disabled_reason(ui, &r);
    }
}

fn draw_mirror(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.mirror");
    let l_go = state.t("actions.mirror");
    let l_weld = state.t("actions.weld_eps");
    ui.label(l_title);
    ui.small(state.t("hints.mirror"));
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
    let reason = edit_guard(state);
    let resp = ui
        .add_enabled(reason.is_none(), egui::Button::new(l_go))
        .on_hover_text(reason.clone().unwrap_or_else(|| "Ctrl+M".to_string()));
    if resp.clicked() {
        petunia_module_model::MirrorTool::apply(state);
    }
    if let Some(r) = reason {
        show_disabled_reason(ui, &r);
    }
}

fn draw_connect(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.connect");
    let l_go = state.t("actions.connect");
    ui.label(l_title);
    ui.small(state.t("hints.connect"));
    let reason = selection_guard(state);
    let resp = ui
        .add_enabled(reason.is_none(), egui::Button::new(l_go))
        .on_hover_text(reason.clone().unwrap_or_else(|| "B".to_string()));
    if resp.clicked() {
        petunia_module_model::ConnectTool::apply(state);
    }
    if let Some(r) = reason {
        show_disabled_reason(ui, &r);
    }
}

fn draw_merge(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.merge");
    ui.label(l_title);
    ui.small(state.t("hints.merge"));
    // Merge center exige seleção.
    let center_cmd = MergeCenterCmd;
    let center_reason = center_cmd.can_execute(state).err().map(|e| e.to_string());
    let resp = ui.add_enabled(
        center_reason.is_none(),
        egui::Button::new(state.t("actions.merge_center")),
    );
    let resp = resp.on_hover_text(center_reason.clone().unwrap_or_else(|| "M".to_string()));
    if resp.clicked() {
        petunia_module_model::MergeTool::apply(state);
    }
    if let Some(r) = center_reason {
        show_disabled_reason(ui, &r);
    }
    ui.separator();
    // Merge by distance usa weld global com threshold.
    ui.label(state.t("actions.merge_by_distance"));
    let dist_label = state.t("actions.merge_distance");
    if ui
        .add(
            egui::Slider::new(&mut state.merge_dist, 0.0005..=0.1)
                .text(dist_label)
                .logarithmic(true),
        )
        .changed()
    {
        state.mark_dirty();
    }
    let weld_cmd = WeldCmd {
        eps: state.merge_dist,
    };
    let weld_reason = weld_cmd.can_execute(state).err().map(|e| e.to_string());
    let resp = ui.add_enabled(
        weld_reason.is_none(),
        egui::Button::new(state.t("actions.merge_by_distance")),
    );
    let resp = resp.on_hover_text(weld_reason.clone().unwrap_or_default());
    if resp.clicked() {
        petunia_module_model::MergeTool::apply_by_distance(state);
    }
    if let Some(r) = weld_reason {
        show_disabled_reason(ui, &r);
    }
}

fn draw_symmetrize(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.symmetrize");
    ui.label(l_title);
    ui.small(state.t("hints.symmetrize"));
    ui.horizontal(|ui| {
        for (ax, name) in [(0, "X"), (1, "Y"), (2, "Z")] {
            if ui
                .selectable_label(state.symmetrize_axis == ax, name)
                .clicked()
            {
                state.symmetrize_axis = ax;
                state.mark_dirty();
            }
        }
    });
    ui.horizontal(|ui| {
        let pos_label = state.t("actions.symmetrize_dir_pos");
        let neg_label = state.t("actions.symmetrize_dir_neg");
        if ui
            .selectable_label(state.symmetrize_pos_to_neg, pos_label)
            .clicked()
        {
            state.symmetrize_pos_to_neg = true;
            state.mark_dirty();
        }
        if ui
            .selectable_label(!state.symmetrize_pos_to_neg, neg_label)
            .clicked()
        {
            state.symmetrize_pos_to_neg = false;
            state.mark_dirty();
        }
    });
    ui.small(state.t("actions.symmetrize_dir"));
    let cmd = SymmetrizeCmd {
        axis: state.symmetrize_axis,
        positive_to_negative: state.symmetrize_pos_to_neg,
        eps: state.mirror_weld,
    };
    let reason = cmd.can_execute(state).err().map(|e| e.to_string());
    let resp = ui.add_enabled(
        reason.is_none(),
        egui::Button::new(state.t("actions.symmetrize")),
    );
    let resp = resp.on_hover_text(reason.clone().unwrap_or_else(|| "Alt+M".to_string()));
    if resp.clicked() {
        petunia_module_model::SymmetrizeTool::apply(state);
    }
    if let Some(r) = reason {
        show_disabled_reason(ui, &r);
    }
}

fn draw_dissolve(ui: &mut Ui, state: &mut AppState) {
    let l_title = state.t("tools.dissolve");
    let l_go = state.t("actions.dissolve");
    ui.label(l_title);
    ui.small(state.t("hints.dissolve"));
    let reason = selection_guard(state);
    let resp = ui
        .add_enabled(reason.is_none(), egui::Button::new(l_go))
        .on_hover_text(reason.clone().unwrap_or_else(|| "X".to_string()));
    if resp.clicked() {
        petunia_module_model::DissolveTool::apply(state);
    }
    if let Some(r) = reason {
        show_disabled_reason(ui, &r);
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
