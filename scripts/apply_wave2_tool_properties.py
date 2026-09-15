#!/usr/bin/env python3
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(path: str) -> str:
    return (ROOT / path).read_text()


def write(path: str, content: str) -> None:
    p = ROOT / path
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(content)


def replace_once(path: str, old: str, new: str) -> None:
    content = read(path)
    if old not in content:
        raise SystemExit(f"wave2 anchor not found in {path}: {old[:80]!r}")
    content = content.replace(old, new, 1)
    write(path, content)


# ---------------------------------------------------------------------------
# Canonical viewport-owned modeling properties. This replaces model_ui as the
# execution surface for modeling operations. Modal tools use one transaction;
# non-modal operations keep a single contextual surface in the viewport.
# ---------------------------------------------------------------------------
write(
    "crates/ui/src/modeling_tool_properties.rs",
    r'''//! Canonical viewport-local controls for modeling tools.
//!
//! Tool Properties are intentionally separate from Object Properties and the
//! Modifier Stack. Modal tools delegate to `tool_fields`, which owns the same
//! transaction used by viewport interaction. Non-modal modeling operations are
//! rendered here so there is no second/legacy execution path in the Inspector.

use egui::Ui;
use petunia_core::{
    AppState, Command, EditMode, MergeCenterCmd, ModalKind, RevolveCmd, WeldCmd,
};

/// Whether the current modeling state owns a contextual Tool Properties card.
pub fn supports(state: &AppState) -> bool {
    if state.modal.is_some() || state.pending_modal.is_some() {
        return true;
    }
    matches!(
        state.active_tool.as_str(),
        "transform"
            | "move"
            | "rotate"
            | "scale"
            | "primitives"
            | "extrude"
            | "inset"
            | "bevel"
            | "pushpull"
            | "slice"
            | "subdivide"
            | "draw_profile"
            | "merge"
            | "connect"
            | "dissolve"
            | "revolve"
    )
}

/// Draw the canonical controls for the current tool.
pub fn draw(ui: &mut Ui, state: &mut AppState) {
    // A modal/pending modal always wins: one transaction, one preview, one
    // commit/cancel path, whether it came from shortcut, toolbar or property UI.
    if (state.modal.is_some() || state.pending_modal.is_some())
        && crate::tool_fields::draw(ui, state)
    {
        return;
    }

    match state.active_tool.as_str() {
        "move" | "rotate" | "scale" | "extrude" | "inset" | "bevel" | "pushpull" => {
            let _ = crate::tool_fields::draw(ui, state);
        }
        "transform" => draw_universal_transform(ui, state),
        "primitives" => draw_primitives(ui, state),
        "subdivide" => draw_subdivide(ui, state),
        "slice" => draw_slice(ui, state),
        "connect" => draw_connect(ui, state),
        "merge" => draw_merge(ui, state),
        "dissolve" => draw_dissolve(ui, state),
        "revolve" => draw_revolve(ui, state),
        "draw_profile" => draw_profile(ui, state),
        _ => {}
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
    edit_guard(state).or_else(|| {
        state
            .selection
            .is_empty()
            .then(|| state.t("actions.need_selection"))
    })
}

fn disabled_reason(ui: &mut Ui, reason: &Option<String>) {
    if let Some(reason) = reason {
        ui.small(reason);
    }
}

fn activate_transform(state: &mut AppState, id: &str, kind: ModalKind) {
    state.active_tool = id.to_owned();
    state.gizmo_mode = kind;
    state.pending_modal = Some(kind);
    state.mark_dirty();
}

fn draw_universal_transform(ui: &mut Ui, state: &mut AppState) {
    ui.small(state.t("tool_properties.choose_transform"));
    ui.horizontal(|ui| {
        if ui.button(state.t("tools.move")).clicked() {
            activate_transform(state, "move", ModalKind::Move);
        }
        if ui.button(state.t("tools.rotate")).clicked() {
            activate_transform(state, "rotate", ModalKind::Rotate);
        }
        if ui.button(state.t("tools.scale")).clicked() {
            activate_transform(state, "scale", ModalKind::Scale);
        }
    });
    ui.small(state.t("hints.transform"));
}

fn draw_primitives(ui: &mut Ui, state: &mut AppState) {
    ui.small(state.t("hints.primitives"));
    let items = [
        ("Cube", "prims.cube"),
        ("Plane", "prims.plane"),
        ("Wedge", "prims.wedge"),
        ("Cylinder", "prims.cylinder"),
        ("Cone", "prims.cone"),
        ("Circle", "prims.circle"),
        ("Torus", "prims.torus"),
        ("Sphere", "prims.sphere"),
        ("Icosphere", "prims.icosphere"),
        ("Capsule", "prims.capsule"),
    ];
    egui::Grid::new("tool_properties.primitive_grid")
        .num_columns(2)
        .spacing([6.0, 4.0])
        .show(ui, |ui| {
            for (index, (name, text_id)) in items.into_iter().enumerate() {
                if ui.button(state.t(text_id)).clicked() {
                    petunia_module_model::PrimitivesTool::add_primitive(state, name);
                }
                if index % 2 == 1 {
                    ui.end_row();
                }
            }
        });
}

fn draw_subdivide(ui: &mut Ui, state: &mut AppState) {
    ui.small(state.t("hints.subdivide"));
    let mut cuts = state.subdivide_cuts as i64;
    if ui
        .add(
            egui::Slider::new(&mut cuts, 1..=6)
                .text(state.t("tool_properties.cuts"))
                .step_by(1.0),
        )
        .changed()
    {
        state.subdivide_cuts = cuts.clamp(1, 6) as u32;
        state.mark_dirty();
    }

    let reason = selection_guard(state);
    ui.horizontal(|ui| {
        let subdivide = ui
            .add_enabled(
                reason.is_none(),
                egui::Button::new(state.t("actions.subdivide")),
            )
            .on_hover_text(reason.clone().unwrap_or_else(|| "W".to_owned()));
        if subdivide.clicked() {
            petunia_module_model::SubdivideTool::apply_subdivide(state);
        }
        let triangulate = ui.add_enabled(
            reason.is_none(),
            egui::Button::new(state.t("actions.triangulate")),
        );
        if triangulate.clicked() {
            petunia_module_model::SubdivideTool::apply_triangulate(state);
        }
    });
    disabled_reason(ui, &reason);
}

fn draw_slice(ui: &mut Ui, state: &mut AppState) {
    ui.small(state.t("hints.slice"));
    let reason = edit_guard(state);
    ui.horizontal(|ui| {
        for (label, axis) in [
            (state.t("actions.slice_x"), glam::Vec3::X),
            (state.t("actions.slice_y"), glam::Vec3::Y),
            (state.t("actions.slice_z"), glam::Vec3::Z),
        ] {
            let response = ui
                .add_enabled(reason.is_none(), egui::Button::new(label))
                .on_hover_text(reason.clone().unwrap_or_default());
            if response.clicked() {
                petunia_module_model::SliceTool::apply_slice(state, axis, false);
            }
        }
    });
    let cap = state.t("actions.slice_cap");
    let response = ui
        .add_enabled(reason.is_none(), egui::Button::new(cap.clone()))
        .on_hover_text(reason.clone().unwrap_or(cap));
    if response.clicked() {
        let camera_direction = state.camera.forward();
        petunia_module_model::SliceTool::apply_slice(state, camera_direction, true);
    }
    disabled_reason(ui, &reason);
}

fn draw_connect(ui: &mut Ui, state: &mut AppState) {
    ui.small(state.t("hints.connect"));
    let reason = selection_guard(state);
    let response = ui
        .add_enabled(
            reason.is_none(),
            egui::Button::new(state.t("actions.connect")),
        )
        .on_hover_text(reason.clone().unwrap_or_else(|| "B".to_owned()));
    if response.clicked() {
        petunia_module_model::ConnectTool::apply(state);
    }
    disabled_reason(ui, &reason);
}

fn draw_merge(ui: &mut Ui, state: &mut AppState) {
    ui.small(state.t("hints.merge"));
    let center_command = MergeCenterCmd;
    let center_reason = center_command.can_execute(state).err().map(|e| e.to_string());
    let response = ui
        .add_enabled(
            center_reason.is_none(),
            egui::Button::new(state.t("actions.merge_center")),
        )
        .on_hover_text(center_reason.clone().unwrap_or_else(|| "M".to_owned()));
    if response.clicked() {
        petunia_module_model::MergeTool::apply(state);
    }
    disabled_reason(ui, &center_reason);

    ui.separator();
    ui.label(state.t("actions.merge_by_distance"));
    if ui
        .add(
            egui::Slider::new(&mut state.merge_dist, 0.0005..=0.1)
                .text(state.t("actions.merge_distance"))
                .logarithmic(true),
        )
        .changed()
    {
        state.mark_dirty();
    }
    let weld_command = WeldCmd {
        eps: state.merge_dist,
    };
    let weld_reason = weld_command.can_execute(state).err().map(|e| e.to_string());
    let response = ui
        .add_enabled(
            weld_reason.is_none(),
            egui::Button::new(state.t("actions.merge_by_distance")),
        )
        .on_hover_text(weld_reason.clone().unwrap_or_default());
    if response.clicked() {
        petunia_module_model::MergeTool::apply_by_distance(state);
    }
    disabled_reason(ui, &weld_reason);
}

fn draw_dissolve(ui: &mut Ui, state: &mut AppState) {
    ui.small(state.t("hints.dissolve"));
    let reason = selection_guard(state);
    let response = ui
        .add_enabled(
            reason.is_none(),
            egui::Button::new(state.t("actions.dissolve")),
        )
        .on_hover_text(reason.clone().unwrap_or_else(|| "X".to_owned()));
    if response.clicked() {
        petunia_module_model::DissolveTool::apply(state);
    }
    disabled_reason(ui, &reason);
}

fn draw_revolve(ui: &mut Ui, state: &mut AppState) {
    ui.small(state.t("hints.revolve"));
    let mut segments = state.revolve_segments as i64;
    if ui
        .add(
            egui::Slider::new(&mut segments, 3..=64)
                .text(state.t("prims.segments"))
                .step_by(1.0),
        )
        .changed()
    {
        state.revolve_segments = segments.clamp(3, 64) as u32;
        state.mark_dirty();
    }
    if ui
        .add(
            egui::Slider::new(&mut state.revolve_angle, 5.0..=360.0)
                .text(state.t("actions.angle")),
        )
        .changed()
    {
        state.mark_dirty();
    }
    if state.revolve_angle < 359.5 {
        ui.small(state.t("actions.revolve_open_hint"));
    }
    ui.horizontal(|ui| {
        ui.label(state.t("modifiers.axis"));
        for (axis, name) in [(0, "X"), (1, "Y"), (2, "Z")] {
            if ui
                .selectable_label(state.revolve_axis == axis, name)
                .clicked()
            {
                state.revolve_axis = axis;
                state.mark_dirty();
            }
        }
    });

    let reason = selection_guard(state);
    let response = ui
        .add_enabled(
            reason.is_none(),
            egui::Button::new(state.t("actions.revolve")),
        )
        .on_hover_text(reason.clone().unwrap_or_else(|| state.t("hints.revolve")));
    if response.clicked() {
        let center = state
            .project
            .active_mesh()
            .map(|mesh| mesh.selection_center())
            .unwrap_or([0.0, 0.0, 0.0]);
        let command = RevolveCmd {
            segments: state.revolve_segments,
            angle_deg: state.revolve_angle,
            axis: state.revolve_axis,
            center,
        };
        if let Err(error) = state.dispatch(&command) {
            state.set_status(format!("{}: {error}", state.t("tools.revolve")));
        }
    }
    if let Some(reason) = &reason {
        ui.small(reason);
    } else {
        ui.small(state.t("actions.revolve_need_edges"));
    }
}

fn draw_profile(ui: &mut Ui, state: &mut AppState) {
    ui.small(state.t("tool_properties.profile_usage"));
    ui.label(format!(
        "{}: {}",
        state.t("profile.points"),
        state.profile.points.len()
    ));

    let mut snap = state.profile.snap;
    if ui.checkbox(&mut snap, state.t("profile.snap")).changed() {
        state.profile.snap = snap;
        state.mark_dirty();
    }
    if ui
        .add(
            egui::Slider::new(&mut state.profile.depth, 0.05..=8.0)
                .text(state.t("profile.depth")),
        )
        .changed()
    {
        state.mark_dirty();
    }
    if ui
        .add(
            egui::Slider::new(&mut state.profile.revolve_segments, 3..=48)
                .text(state.t("profile.segments")),
        )
        .changed()
    {
        state.mark_dirty();
    }

    ui.horizontal(|ui| {
        if ui.button(state.t("profile.close")).clicked() {
            if state.profile.points.len() >= 3 {
                state.profile.closed = true;
            }
            state.mark_dirty();
        }
        if ui.button(state.t("profile.undo_pt")).clicked() {
            state.profile.points.pop();
            state.profile.closed = false;
            state.mark_dirty();
        }
        if ui.button(state.t("profile.clear")).clicked() {
            state.profile.clear();
            state.mark_dirty();
        }
    });
    ui.horizontal(|ui| {
        if ui.button(state.t("profile.gen_extrude")).clicked() {
            petunia_module_model::draw_profile::generate_extrude(state);
        }
        if ui.button(state.t("profile.gen_revolve")).clicked() {
            petunia_module_model::draw_profile::generate_revolve(state);
        }
    });

    if state.profile.closed {
        match petunia_mesh::triangulate::ear_clip(&state.profile.points) {
            Ok(triangles) => {
                ui.small(format!(
                    "{}: {}",
                    state.t("profile.tris"),
                    triangles.len()
                ));
            }
            Err(error) => {
                ui.colored_label(egui::Color32::LIGHT_RED, error);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_surface_supports_all_contextual_modeling_tools() {
        let mut state = AppState::new("en");
        for id in [
            "transform",
            "move",
            "rotate",
            "scale",
            "primitives",
            "extrude",
            "inset",
            "bevel",
            "pushpull",
            "slice",
            "subdivide",
            "draw_profile",
            "merge",
            "connect",
            "dissolve",
            "revolve",
        ] {
            state.active_tool = id.to_owned();
            assert!(supports(&state), "missing Tool Properties for {id}");
        }
        state.active_tool = "select".to_owned();
        assert!(!supports(&state));
    }

    #[test]
    fn non_modal_property_surfaces_render_without_panicking() {
        let context = egui::Context::default();
        for id in [
            "transform",
            "primitives",
            "slice",
            "subdivide",
            "draw_profile",
            "merge",
            "connect",
            "dissolve",
            "revolve",
        ] {
            let mut state = AppState::new("en");
            state.mode = EditMode::Edit;
            state.active_tool = id.to_owned();
            context
                .run_ui(egui::RawInput::default(), |ui| draw(ui, &mut state))
                .textures_delta
                .clear();
        }
    }
}
''',
)

# ---------------------------------------------------------------------------
# Tool-field transaction editor: i18n, explicit transform children, no stale
# Universal->last-gizmo alias, and no duplicated Apply/Cancel UI.
# ---------------------------------------------------------------------------
write(
    "crates/ui/src/tool_fields.rs",
    r'''//! Editable Tool Properties driving the same transaction as viewport transforms.
use egui::{Key, Modifiers};
use glam::Vec3;
use petunia_core::{AppState, ModalConstraint, ModalKind};

const SESSION_ID: &str = "tool.property_fields";

#[derive(Clone)]
struct FieldSession {
    kind: ModalKind,
    values: [String; 3],
    active: bool,
    error: Option<String>,
}

impl FieldSession {
    fn new(kind: ModalKind, state: &AppState) -> Self {
        let values = match kind {
            ModalKind::Scale => [1.0; 3],
            ModalKind::Extrude => [state.extrude_dist, 0.0, 0.0],
            ModalKind::Inset => [state.inset_factor, 0.0, 0.0],
            ModalKind::Bevel => [state.bevel_amount, 0.0, 0.0],
            ModalKind::PushPull => [state.push_dist, 0.0, 0.0],
            _ => [0.0; 3],
        };
        Self {
            kind,
            values: values.map(|value| value.to_string()),
            active: false,
            error: None,
        }
    }

    fn parsed(&self, state: &AppState) -> Result<Vec3, String> {
        let count = if transform(self.kind) { 3 } else { 1 };
        let mut values = [0.0; 3];
        for (index, slot) in values.iter_mut().enumerate().take(count) {
            *slot = self.values[index]
                .trim()
                .replace(',', ".")
                .parse::<f32>()
                .ok()
                .filter(|value| value.is_finite())
                .ok_or_else(|| state.t("tool_properties.error_number"))?;
        }
        Ok(Vec3::from_array(values))
    }

    fn preview(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.active {
            state.pending_modal = None;
            state
                .begin_modal(self.kind)
                .map_err(|error| error.to_string())?;
            self.active = true;
        }
        let values = self.parsed(state)?;
        if transform(self.kind) {
            state.update_modal_components(values)
        } else {
            state
                .set_modal_constraint(ModalConstraint::Free)
                .map_err(|error| error.to_string())?;
            state.update_modal(Vec3::ZERO, values.x)
        }
        .map_err(|error| error.to_string())
    }
}

fn transform(kind: ModalKind) -> bool {
    matches!(kind, ModalKind::Move | ModalKind::Rotate | ModalKind::Scale)
}

pub fn kind_label(state: &AppState, kind: ModalKind) -> String {
    state.t(match kind {
        ModalKind::Move => "tools.move",
        ModalKind::Rotate => "tools.rotate",
        ModalKind::Scale => "tools.scale",
        ModalKind::Extrude => "tools.extrude",
        ModalKind::Inset => "tools.inset",
        ModalKind::Bevel => "tools.bevel",
        ModalKind::PushPull => "tools.pushpull",
    })
}

/// Viewport input yields while the property editor owns the active transaction.
pub fn owns_modal(ctx: &egui::Context) -> bool {
    ctx.data_mut(|data| data.get_temp::<FieldSession>(egui::Id::new(SESSION_ID)))
        .is_some_and(|session| session.active)
}

fn selected_kind(state: &AppState) -> Option<ModalKind> {
    state
        .modal
        .as_ref()
        .map(|modal| modal.kind)
        .or(state.pending_modal)
        .or(match state.active_tool.as_str() {
            "move" => Some(ModalKind::Move),
            "rotate" => Some(ModalKind::Rotate),
            "scale" => Some(ModalKind::Scale),
            "extrude" => Some(ModalKind::Extrude),
            "inset" => Some(ModalKind::Inset),
            "bevel" => Some(ModalKind::Bevel),
            "pushpull" => Some(ModalKind::PushPull),
            // Universal Transform is intentionally not an implicit alias for
            // whichever gizmo mode happened to be selected previously.
            _ => None,
        })
}

fn field_id(ui: &egui::Ui, kind: ModalKind, index: usize) -> egui::Id {
    ui.id().with((SESSION_ID, kind.label(), index))
}

fn activate_transform_kind(
    state: &mut AppState,
    session: &mut FieldSession,
    kind: ModalKind,
) {
    let active_tool = match kind {
        ModalKind::Move => "move",
        ModalKind::Rotate => "rotate",
        ModalKind::Scale => "scale",
        _ => return,
    };
    state.active_tool = active_tool.to_owned();
    state.gizmo_mode = kind;
    state.pending_modal = Some(kind);
    *session = FieldSession::new(kind, state);
    state.mark_dirty();
}

/// Returns true when the selected tool has canonical editable numeric properties.
pub fn draw(ui: &mut egui::Ui, state: &mut AppState) -> bool {
    let ctx = ui.ctx().clone();
    let id = egui::Id::new(SESSION_ID);
    let Some(mut kind) = selected_kind(state) else {
        ctx.data_mut(|data| data.remove::<FieldSession>(id));
        return false;
    };
    let mut session = ctx
        .data_mut(|data| data.get_temp::<FieldSession>(id))
        .filter(|session| session.kind == kind)
        .unwrap_or_else(|| FieldSession::new(kind, state));
    if session.active && state.modal.is_none() {
        session = FieldSession::new(kind, state);
    }
    let blocked = state.mesh_preview.is_some()
        || state.paint_stroke.is_some()
        || (state.modal.is_some() && !session.active);

    if transform(kind) {
        ui.add_enabled_ui(!session.active && !blocked, |ui| {
            ui.horizontal(|ui| {
                for (candidate, label_id) in [
                    (ModalKind::Move, "tools.move"),
                    (ModalKind::Rotate, "tools.rotate"),
                    (ModalKind::Scale, "tools.scale"),
                ] {
                    if ui
                        .selectable_label(kind == candidate, state.t(label_id))
                        .clicked()
                    {
                        kind = candidate;
                        activate_transform_kind(state, &mut session, kind);
                    }
                }
            });
        });
    }

    if blocked {
        if let Some(modal) = &state.modal {
            session.values = if transform(kind) {
                modal.components.to_array()
            } else {
                [modal.value, 0.0, 0.0]
            }
            .map(|value| format!("{value:.4}"));
        }
        ui.small(state.t("tool_properties.blocked"));
    }

    let (label, unit) = match kind {
        ModalKind::Move => (state.t("tool_properties.displacement"), "m"),
        ModalKind::Rotate => (state.t("tool_properties.rotation_xyz"), "°"),
        ModalKind::Scale => (state.t("tool_properties.scale_xyz"), "×"),
        ModalKind::Extrude => (state.t("tool_properties.extrude_distance"), "m"),
        ModalKind::Inset => (state.t("tool_properties.inset_factor"), "0–0.95"),
        ModalKind::Bevel => (state.t("tool_properties.bevel_width"), "m"),
        ModalKind::PushPull => (state.t("tool_properties.pushpull_distance"), "m"),
    };
    ui.label(label);

    let mut changed = false;
    let mut focused = false;
    let mut apply = false;
    let mut cancel = false;
    ui.add_enabled_ui(!blocked, |ui| {
        egui::Grid::new(ui.id().with((SESSION_ID, "grid")))
            .num_columns(3)
            .show(ui, |ui| {
                for index in 0..if transform(kind) { 3 } else { 1 } {
                    ui.label(if transform(kind) {
                        ["X", "Y", "Z"][index].to_owned()
                    } else {
                        state.t("tool_properties.value")
                    });
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut session.values[index])
                            .id(field_id(ui, kind, index))
                            .desired_width(100.0)
                            .char_limit(64),
                    );
                    changed |= response.changed();
                    focused |= response.has_focus() || response.lost_focus();
                    ui.label(unit);
                    ui.end_row();
                }
            });
        if changed {
            session.error = session.preview(state).err();
            state.mark_dirty();
        }
        ui.horizontal(|ui| {
            apply = ui
                .add_enabled(
                    session.parsed(state).is_ok(),
                    egui::Button::new(state.t("actions.apply")),
                )
                .clicked();
            cancel = ui
                .add_enabled(
                    session.active || state.pending_modal.is_some(),
                    egui::Button::new(state.t("actions.cancel")),
                )
                .clicked();
        });
        if focused || session.active {
            apply |= ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Enter));
            cancel |= ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape));
        }
    });

    if cancel {
        state.cancel_modal();
        session = FieldSession::new(kind, state);
    } else if apply {
        match session.preview(state) {
            Ok(()) => {
                if let Ok(values) = session.parsed(state) {
                    match kind {
                        ModalKind::Extrude => state.extrude_dist = values.x,
                        ModalKind::Inset => state.inset_factor = values.x,
                        ModalKind::Bevel => state.bevel_amount = values.x,
                        ModalKind::PushPull => state.push_dist = values.x,
                        _ => {}
                    }
                }
                state.commit_modal();
                session = FieldSession::new(kind, state);
            }
            Err(error) => session.error = Some(error),
        }
    }

    if let Some(error) = &session.error {
        ui.colored_label(egui::Color32::LIGHT_RED, error);
    }
    ui.small(state.t("tool_properties.preview_hint"));
    if kind == ModalKind::Rotate {
        ui.small(state.t("tool_properties.rotation_hint"));
    }
    if kind == ModalKind::Bevel {
        ui.small(state.t("tool_properties.bevel_hint"));
    }
    ctx.data_mut(|data| data.insert_temp(id, session));
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_children_map_explicitly_and_universal_does_not_alias_stale_mode() {
        let mut state = AppState::new("en");
        state.gizmo_mode = ModalKind::Scale;
        state.active_tool = "transform".into();
        assert_eq!(selected_kind(&state), None);
        state.active_tool = "move".into();
        assert_eq!(selected_kind(&state), Some(ModalKind::Move));
        state.active_tool = "rotate".into();
        assert_eq!(selected_kind(&state), Some(ModalKind::Rotate));
        state.active_tool = "scale".into();
        assert_eq!(selected_kind(&state), Some(ModalKind::Scale));
    }

    #[test]
    fn modal_kind_labels_are_localized() {
        let state = AppState::new("pt-BR");
        assert_eq!(kind_label(&state, ModalKind::Move), "Mover");
        assert_eq!(kind_label(&state, ModalKind::Bevel), "Bevel");
    }
}
''',
)

# Canonical popover delegates to the canonical modeling properties only.
write(
    "crates/ui/src/tool_properties_popover.rs",
    r'''//! Floating Tool Properties owned by the viewport, not the Object Inspector.

use egui::{Align2, RichText, Ui, Vec2};
use petunia_core::{AppState, Workspace};

pub fn draw(ui: &mut Ui, state: &mut AppState, viewport: egui::Rect) {
    if state.workspace != Workspace::Model
        || state.is_active_locked()
        || !crate::modeling_tool_properties::supports(state)
    {
        return;
    }

    let active = state.active_tool.clone();
    let title = state
        .modal
        .as_ref()
        .map(|modal| crate::tool_fields::kind_label(state, modal.kind))
        .or_else(|| {
            state
                .pending_modal
                .map(|kind| crate::tool_fields::kind_label(state, kind))
        })
        .unwrap_or_else(|| {
            if active == "transform" {
                state.t("tool_properties.universal_transform")
            } else {
                let key = format!("tools.{active}");
                let translated = state.t(&key);
                if translated == key { active.clone() } else { translated }
            }
        });

    let pos = viewport.left_top() + Vec2::new(18.0, 18.0);
    egui::Area::new(egui::Id::new("viewport.tool_properties"))
        .order(egui::Order::Foreground)
        .fixed_pos(pos)
        .pivot(Align2::LEFT_TOP)
        .show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style()).show(ui, |ui| {
                ui.set_min_width(220.0);
                ui.set_max_width(300.0);
                ui.label(RichText::new(&title).strong().size(12.0));
                ui.small(state.t("tool_properties.title"));
                ui.separator();
                crate::modeling_tool_properties::draw(ui, state);
            });
        });
}
''',
)

# Remove the legacy module from compilation and delete its source.
replace_once(
    "crates/ui/src/modules_ui/mod.rs",
    "pub mod animation_ui;\npub mod model_ui;\npub mod paint_ui;\npub mod uv_ui;",
    "pub mod animation_ui;\npub mod paint_ui;\npub mod uv_ui;",
)
legacy = ROOT / "crates/ui/src/modules_ui/model_ui.rs"
if legacy.exists():
    legacy.unlink()

# Register the canonical module.
replace_once(
    "crates/ui/src/lib.rs",
    "pub mod measurement;\n#[cfg(test)]",
    "pub mod measurement;\npub mod modeling_tool_properties;\n#[cfg(test)]",
)

# Shortcut and command-palette path for Revolve now opens the same contextual
# tool workflow instead of executing a hidden default operation immediately.
replace_once(
    "crates/app/src/lib.rs",
    '''            "model.revolve" => {\n                let _ = self.state.dispatch(&petunia_core::RevolveCmd::default());\n            }''',
    '''            "model.revolve" => self.set_tool("revolve", None),''',
)

# ---------------------------------------------------------------------------
# Modifier stack: remove hard-coded visible strings and make direction explicit.
# ---------------------------------------------------------------------------
properties_path = "crates/ui/src/properties_panel.rs"
properties = read(properties_path)
start = properties.index("fn draw_modifiers_section(")
end = properties.index("/// Seção Display", start)
new_modifier_fn = r'''fn draw_modifiers_section(ui: &mut Ui, state: &mut AppState, force_open: bool) {
    use petunia_project::{ModifierInstance, ModifierKind};

    let section_title = state.t("modifiers.title");
    inspector_widgets::section(
        ui,
        state.ui.density,
        "modifiers",
        inspector_widgets::SectionOpts {
            title: &section_title,
            summary: None,
            default_open: false,
            force_open,
        },
        |ui| {
            let Some(asset_idx) =
                inspected_asset_idx(state).filter(|idx| *idx < state.project.assets.len())
            else {
                ui.label(RichText::new(state.t("empty.no_selection")).color(tokens::TEXT_MUTED));
                return;
            };

            // Precompute localized strings before mutably borrowing a modifier.
            let enable_tip = state.t("modifiers.enable");
            let remove_tip = state.t("modifiers.remove");
            let axis_label = state.t("modifiers.axis");
            let weld_label = state.t("actions.weld_eps");
            let mirror_title = state.t("tools.mirror");
            let symmetry_title = state.t("tools.symmetrize");
            let add_mirror = state.t("modifiers.add_mirror");
            let add_symmetry = state.t("modifiers.add_symmetry");
            let move_up_tip = state.t("toolbar.move_up");
            let move_down_tip = state.t("toolbar.move_down");
            let positive_to_negative = state.t("actions.symmetrize_dir_pos");
            let negative_to_positive = state.t("actions.symmetrize_dir_neg");

            let mut changed = false;
            let mut remove = None;
            let mut move_up = None;
            let mut move_down = None;
            let modifier_count = state.project.assets[asset_idx].modifiers.len();
            ui.push_id("modifier_stack_rows", |ui| {
                for modifier_idx in 0..modifier_count {
                    let snapshot = state.project.assets[asset_idx].modifiers[modifier_idx].clone();
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let mut enabled = snapshot.enabled;
                            if ui
                                .checkbox(&mut enabled, "")
                                .on_hover_text(&enable_tip)
                                .changed()
                            {
                                state.project.assets[asset_idx].modifiers[modifier_idx].enabled =
                                    enabled;
                                changed = true;
                            }
                            let title = match snapshot.kind {
                                ModifierKind::Mirror { .. } => &mirror_title,
                                ModifierKind::Symmetry { .. } => &symmetry_title,
                            };
                            ui.strong(title);
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui
                                        .small_button("×")
                                        .on_hover_text(&remove_tip)
                                        .clicked()
                                    {
                                        remove = Some(modifier_idx);
                                    }
                                    if ui
                                        .add_enabled(
                                            modifier_idx + 1 < modifier_count,
                                            egui::Button::new("↓"),
                                        )
                                        .on_hover_text(&move_down_tip)
                                        .clicked()
                                    {
                                        move_down = Some(modifier_idx);
                                    }
                                    if ui
                                        .add_enabled(modifier_idx > 0, egui::Button::new("↑"))
                                        .on_hover_text(&move_up_tip)
                                        .clicked()
                                    {
                                        move_up = Some(modifier_idx);
                                    }
                                },
                            );
                        });

                        match &mut state.project.assets[asset_idx].modifiers[modifier_idx].kind {
                            ModifierKind::Mirror { axis, weld } => {
                                ui.horizontal(|ui| {
                                    ui.label(&axis_label);
                                    for (candidate, name) in [(0, "X"), (1, "Y"), (2, "Z")] {
                                        if ui.selectable_label(*axis == candidate, name).clicked() {
                                            *axis = candidate;
                                            changed = true;
                                        }
                                    }
                                });
                                changed |= ui
                                    .add(egui::Slider::new(weld, 0.0..=0.05).text(&weld_label))
                                    .changed();
                            }
                            ModifierKind::Symmetry {
                                axis,
                                positive_to_negative: direction,
                                weld,
                            } => {
                                ui.horizontal(|ui| {
                                    ui.label(&axis_label);
                                    for (candidate, name) in [(0, "X"), (1, "Y"), (2, "Z")] {
                                        if ui.selectable_label(*axis == candidate, name).clicked() {
                                            *axis = candidate;
                                            changed = true;
                                        }
                                    }
                                });
                                ui.horizontal(|ui| {
                                    if ui
                                        .selectable_label(*direction, &positive_to_negative)
                                        .clicked()
                                    {
                                        *direction = true;
                                        changed = true;
                                    }
                                    if ui
                                        .selectable_label(!*direction, &negative_to_positive)
                                        .clicked()
                                    {
                                        *direction = false;
                                        changed = true;
                                    }
                                });
                                changed |= ui
                                    .add(egui::Slider::new(weld, 0.0..=0.05).text(&weld_label))
                                    .changed();
                            }
                        }
                    });
                    ui.add_space(4.0);
                }
            });

            if let Some(index) = remove {
                state.project.assets[asset_idx].modifiers.remove(index);
                changed = true;
            } else if let Some(index) = move_up {
                state.project.assets[asset_idx]
                    .modifiers
                    .swap(index, index - 1);
                changed = true;
            } else if let Some(index) = move_down {
                state.project.assets[asset_idx]
                    .modifiers
                    .swap(index, index + 1);
                changed = true;
            }

            if modifier_count == 0 {
                ui.label(
                    RichText::new(state.t("modifiers.empty"))
                        .size(11.0)
                        .color(tokens::TEXT_MUTED),
                );
            }

            ui.horizontal(|ui| {
                if ui.button(add_mirror).clicked() {
                    state.project.assets[asset_idx]
                        .modifiers
                        .push(ModifierInstance::mirror(0, 0.001));
                    changed = true;
                }
                if ui.button(add_symmetry).clicked() {
                    state.project.assets[asset_idx]
                        .modifiers
                        .push(ModifierInstance::symmetry(0, true, 0.001));
                    changed = true;
                }
            });

            if changed {
                state.emit_mesh_changed();
                state.mark_dirty();
            }
        },
    );
}

'''
properties = properties[:start] + new_modifier_fn + properties[end:]
properties = properties.replace(
    'ui.label(format!("Tris: {}", mesh.tri_count()));',
    'ui.label(format!("{}: {}", state.t("geometry.tris"), mesh.tri_count()));',
)
properties = properties.replace(
    '            state.t("props.verts"),\n            "edges",\n            state.t("props.faces"),',
    '            state.t("props.verts"),\n            state.t("props.edges"),\n            state.t("props.faces"),',
)
write(properties_path, properties)

# Toolbar planned Lasso entry is localized too.
replace_once(
    "crates/ui/src/toolbar.rs",
    '''            ui.add_enabled(false, egui::Button::new("Lasso"))\n                .on_hover_text("Planned selection behavior");''',
    '''            ui.add_enabled(false, egui::Button::new(state.t("tools.select_lasso")))\n                .on_hover_text(state.t("toolbar.planned"));''',
)

# ---------------------------------------------------------------------------
# Locales: all new/changed visible Tool Properties + Modifier Stack strings.
# ---------------------------------------------------------------------------
for locale_path, values in [
    (
        "assets/locales/en.toml",
        {
            "toolbar_planned": 'planned = "Planned feature"',
            "lasso": 'select_lasso = "Lasso Select"',
            "geometry_tris": 'tris = "Triangles"',
            "modifiers": '''[modifiers]\ntitle = "Modifiers"\nempty = "No modifiers in stack."\nadd = "Add"\nenable = "Enable modifier"\nremove = "Remove modifier"\naxis = "Axis"\nadd_mirror = "Add Mirror"\nadd_symmetry = "Add Symmetry"''',
            "tool_props": '''[tool_properties]\ntitle = "Tool Properties"\nuniversal_transform = "Universal Transform"\nchoose_transform = "Choose a transform handle or use G/R/S."\nblocked = "Confirm or cancel the viewport operation before editing these fields."\nerror_number = "Enter a finite number in every field."\nvalue = "Value"\ndisplacement = "Displacement"\nrotation_xyz = "XYZ Rotation"\nscale_xyz = "Scale per axis"\nextrude_distance = "Extrude distance"\ninset_factor = "Inset factor"\nbevel_width = "Bevel width"\npushpull_distance = "Push/Pull distance"\npreview_hint = "Changes preview live. Enter applies; Esc cancels."\nrotation_hint = "Angles are degrees, Euler XYZ, around the selection center."\nbevel_hint = "Supports one manifold convex edge with simple corners; one segment."\ncuts = "Cuts"\nprofile_usage = "Click in an orthographic view to add points. Click near the first point to close the profile."''',
        },
    ),
    (
        "assets/locales/pt-BR.toml",
        {
            "toolbar_planned": 'planned = "Recurso planejado"',
            "lasso": 'select_lasso = "Seleção em laço"',
            "geometry_tris": 'tris = "Triângulos"',
            "modifiers": '''[modifiers]\ntitle = "Modificadores"\nempty = "Sem modificadores na pilha."\nadd = "Adicionar"\nenable = "Ativar modificador"\nremove = "Remover modificador"\naxis = "Eixo"\nadd_mirror = "Adicionar Espelho"\nadd_symmetry = "Adicionar Simetria"''',
            "tool_props": '''[tool_properties]\ntitle = "Propriedades da ferramenta"\nuniversal_transform = "Transformação unificada"\nchoose_transform = "Escolha uma alça de transformação ou use G/R/S."\nblocked = "Confirme ou cancele a operação na viewport antes de editar estes campos."\nerror_number = "Informe um número finito em cada campo."\nvalue = "Valor"\ndisplacement = "Deslocamento"\nrotation_xyz = "Rotação XYZ"\nscale_xyz = "Escala por eixo"\nextrude_distance = "Distância de extrusão"\ninset_factor = "Fator de inset"\nbevel_width = "Largura do bevel"\npushpull_distance = "Distância de Push/Pull"\npreview_hint = "As alterações são pré-visualizadas em tempo real. Enter aplica; Esc cancela."\nrotation_hint = "Ângulos em graus, Euler XYZ, em torno do centro da seleção."\nbevel_hint = "Suporta uma aresta convexa manifold com cantos simples; um segmento."\ncuts = "Cortes"\nprofile_usage = "Clique em uma vista ortográfica para adicionar pontos. Clique perto do primeiro ponto para fechar o perfil."''',
        },
    ),
]:
    text = read(locale_path)
    text = text.replace(
        'visible = "Visible"\n\n[dock]' if locale_path.endswith("en.toml") else 'visible = "Visível"\n\n[dock]',
        ('visible = "Visible"\n' if locale_path.endswith("en.toml") else 'visible = "Visível"\n')
        + values["toolbar_planned"]
        + '\n\n[dock]',
        1,
    )
    text = text.replace(
        'select_box = "Box Select"' if locale_path.endswith("en.toml") else 'select_box = "Seleção em caixa"',
        ('select_box = "Box Select"' if locale_path.endswith("en.toml") else 'select_box = "Seleção em caixa"')
        + '\n'
        + values["lasso"],
        1,
    )
    old_mod = (
        '[modifiers]\ntitle = "Modifiers"\nempty = "No modifiers in stack."\nadd = "Add"'
        if locale_path.endswith("en.toml")
        else '[modifiers]\ntitle = "Modificadores"\nempty = "Sem modificadores na pilha."\nadd = "Adicionar"'
    )
    if old_mod not in text:
        raise SystemExit(f"modifier locale anchor missing: {locale_path}")
    text = text.replace(old_mod, values["modifiers"], 1)
    old_geom = '[geometry]\ntitle = "Geometry"' if locale_path.endswith("en.toml") else '[geometry]\ntitle = "Geometria"'
    text = text.replace(old_geom, old_geom + '\n' + values["geometry_tris"], 1)
    props_anchor = '\n[props]\n'
    if props_anchor not in text:
        raise SystemExit(f"props anchor missing: {locale_path}")
    text = text.replace(props_anchor, '\n' + values["tool_props"] + '\n\n[props]\n', 1)
    write(locale_path, text)

# Guard against accidentally leaving the legacy execution surface referenced.
for path in [
    "crates/ui/src/tool_properties_popover.rs",
    "crates/ui/src/properties_panel.rs",
    "crates/ui/src/lib.rs",
    "crates/ui/src/modules_ui/mod.rs",
]:
    if "model_ui" in read(path):
        raise SystemExit(f"legacy model_ui reference remains in {path}")
