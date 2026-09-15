#!/usr/bin/env python3
"""Apply the Petunia3D modeling/UI convergence batch on the feature branch.

This script is intentionally idempotent enough for CI retries. It exists only
as a staging aid and is removed by the validating workflow after a successful
application.
"""

from pathlib import Path
import re
import shutil

ROOT = Path(__file__).resolve().parents[1]


def read(path: str) -> str:
    return (ROOT / path).read_text()


def write(path: str, text: str) -> None:
    (ROOT / path).write_text(text)


def replace_once(path: str, old: str, new: str, *, required: bool = True) -> bool:
    text = read(path)
    if old not in text:
        if required:
            raise SystemExit(f"pattern not found in {path}: {old[:120]!r}")
        return False
    write(path, text.replace(old, new, 1))
    return True


def regex_once(path: str, pattern: str, replacement: str, *, flags: int = 0, required: bool = True) -> bool:
    text = read(path)
    updated, count = re.subn(pattern, replacement, text, count=1, flags=flags)
    if count != 1:
        if required:
            raise SystemExit(f"regex count {count} in {path}: {pattern[:120]!r}")
        return False
    write(path, updated)
    return True


# ---------------------------------------------------------------------------
# 1. Persistent non-destructive modifiers in the project domain.
# ---------------------------------------------------------------------------
project_path = "crates/project/src/lib.rs"
project = read(project_path)
if "pub enum ModifierKind" not in project:
    marker = "/// Um asset do projeto. `id` nunca muda (rename seguro).\n"
    block = r'''/// Operação não destrutiva persistente avaliada sobre a malha-base do asset.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ModifierKind {
    Mirror {
        axis: usize,
        weld: f32,
    },
    Symmetry {
        axis: usize,
        positive_to_negative: bool,
        weld: f32,
    },
}

/// Instância ordenada de modifier. O UUID mantém identidade estável para UI,
/// reordenação e futuras animações/serialization migrations.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ModifierInstance {
    pub id: Uuid,
    pub enabled: bool,
    pub kind: ModifierKind,
}

impl ModifierInstance {
    pub fn mirror(axis: usize, weld: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            enabled: true,
            kind: ModifierKind::Mirror {
                axis: axis.min(2),
                weld: weld.max(0.0),
            },
        }
    }

    pub fn symmetry(axis: usize, positive_to_negative: bool, weld: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            enabled: true,
            kind: ModifierKind::Symmetry {
                axis: axis.min(2),
                positive_to_negative,
                weld: weld.max(0.0),
            },
        }
    }
}

'''
    if marker not in project:
        raise SystemExit("Asset marker not found")
    project = project.replace(marker, block + marker, 1)

if "pub modifiers: Vec<ModifierInstance>" not in project:
    project = project.replace(
        "    #[serde(default)]\n    pub tags: Vec<String>,\n",
        "    #[serde(default)]\n    pub tags: Vec<String>,\n    #[serde(default)]\n    pub modifiers: Vec<ModifierInstance>,\n",
        1,
    )
    project = project.replace(
        "            favorite: false,\n            tags: Vec::new(),\n",
        "            favorite: false,\n            tags: Vec::new(),\n            modifiers: Vec::new(),\n",
        1,
    )

if "pub fn evaluated_mesh(&self) -> Mesh" not in project:
    marker = "    /// Duplicata com novo UUID.\n"
    method = r'''    /// Avalia a pilha de modifiers sem alterar a malha-base.
    /// Render, preview e export usam este resultado; edição continua operando
    /// sobre `mesh`, preservando a natureza não destrutiva da pilha.
    pub fn evaluated_mesh(&self) -> Mesh {
        let mut mesh = self.mesh.clone();
        for modifier in &self.modifiers {
            if !modifier.enabled {
                continue;
            }
            match modifier.kind {
                ModifierKind::Mirror { axis, weld } => mesh.mirror(axis, weld),
                ModifierKind::Symmetry {
                    axis,
                    positive_to_negative,
                    weld,
                } => {
                    mesh.symmetrize(axis, positive_to_negative, weld);
                }
            }
        }
        mesh
    }

'''
    if marker not in project:
        raise SystemExit("duplicate marker not found")
    project = project.replace(marker, method + marker, 1)
write(project_path, project)

# Struct literal used by hostile-format test.
format_path = "crates/project/src/format.rs"
format_text = read(format_path)
if "modifiers: vec![]" not in format_text and "modifiers: Vec::new()" not in format_text:
    format_text = format_text.replace(
        "                favorite: false,\n                tags: vec![],\n",
        "                favorite: false,\n                tags: vec![],\n                modifiers: vec![],\n",
        1,
    )
write(format_path, format_text)

# Old destructive Mirror/Symmetrize tools no longer belong to the default tool registry.
model_registry_path = "crates/module-model/src/lib.rs"
model_registry = read(model_registry_path)
model_registry = model_registry.replace("        r.register::<MirrorTool>(&enabled);\n", "")
model_registry = model_registry.replace("        r.register::<SymmetrizeTool>(&enabled);\n", "")
write(model_registry_path, model_registry)

# ---------------------------------------------------------------------------
# 2. Renderer/export evaluation uses the modifier stack.
# ---------------------------------------------------------------------------
wgpu_path = "crates/render-wgpu/src/lib.rs"
wgpu = read(wgpu_path)
loop_marker = "        for obj in &scene.assets {\n            if !obj.visible {\n                continue;\n            }\n"
if "let mesh = obj.evaluated_mesh();" not in wgpu:
    if loop_marker not in wgpu:
        raise SystemExit("WGPU asset loop not found")
    wgpu = wgpu.replace(loop_marker, loop_marker + "            let mesh = obj.evaluated_mesh();\n", 1)
wgpu = wgpu.replace("obj.mesh.to_triangles_unlit()", "mesh.to_triangles_unlit()")
wgpu = wgpu.replace("obj.mesh.to_triangles_smooth(smooth)", "mesh.to_triangles_smooth(smooth)")
wgpu = wgpu.replace("obj.mesh.to_edges()", "mesh.to_edges()")
wgpu = wgpu.replace("obj.mesh.triangulation_wireframe()", "mesh.triangulation_wireframe()")
write(wgpu_path, wgpu)

gl_path = "crates/render-gl/src/lib.rs"
gl = read(gl_path)
# Two loops build mesh data and edge data. Give each one its own evaluated mesh.
pattern = r"(        for obj in &state\.project\.assets \{\n            if !obj\.visible \{\n                continue;\n            \}\n)"
if gl.count("let mesh = obj.evaluated_mesh();") < 2:
    gl, count = re.subn(pattern, r"\1            let mesh = obj.evaluated_mesh();\n", gl)
    if count < 2:
        raise SystemExit(f"expected >=2 OpenGL asset loops, found {count}")
gl = gl.replace("obj.mesh.to_triangles_unlit()", "mesh.to_triangles_unlit()")
gl = gl.replace("obj.mesh.to_triangles_smooth(smooth)", "mesh.to_triangles_smooth(smooth)")
gl = gl.replace("obj.mesh.to_edges()", "mesh.to_edges()")
gl = gl.replace("obj.mesh.triangulation_wireframe()", "mesh.triangulation_wireframe()")
write(gl_path, gl)

export_path = "crates/project/src/export.rs"
export_text = read(export_path)
export_text = export_text.replace(
    "pub fn export_obj(asset: &Asset) -> String {\n    asset.mesh.to_obj()\n}",
    "pub fn export_obj(asset: &Asset) -> String {\n    asset.evaluated_mesh().to_obj()\n}",
)
export_text = export_text.replace("        let mut m = a.mesh.clone();", "        let mut m = a.evaluated_mesh();")
write(export_path, export_text)

pipeline_path = "crates/project/src/pipeline.rs"
pipeline = read(pipeline_path)
pipeline = pipeline.replace("let mut mesh = asset.mesh.clone();", "let mut mesh = asset.evaluated_mesh();")
pipeline = pipeline.replace("let mut m = asset.mesh.clone();", "let mut m = asset.evaluated_mesh();")
write(pipeline_path, pipeline)

fingerprint_path = "crates/core/src/render_revision.rs"
fingerprint = read(fingerprint_path)
if "asset.modifiers.len()" not in fingerprint:
    marker = "        h = mix(h, asset.mesh.selected_edges.len() as u64);\n"
    block = r'''        h = mix(h, asset.mesh.selected_edges.len() as u64);
        h = mix(h, asset.modifiers.len() as u64);
        for modifier in &asset.modifiers {
            h = hash_bytes(h, modifier.id.as_bytes());
            h = mix(h, modifier.enabled as u64);
            match modifier.kind {
                petunia_project::ModifierKind::Mirror { axis, weld } => {
                    h = mix(h, 1);
                    h = mix(h, axis as u64);
                    h = hash_f32(h, weld);
                }
                petunia_project::ModifierKind::Symmetry {
                    axis,
                    positive_to_negative,
                    weld,
                } => {
                    h = mix(h, 2);
                    h = mix(h, axis as u64);
                    h = mix(h, positive_to_negative as u64);
                    h = hash_f32(h, weld);
                }
            }
        }
'''
    if marker not in fingerprint:
        raise SystemExit("fingerprint asset marker not found")
    fingerprint = fingerprint.replace(marker, block, 1)
write(fingerprint_path, fingerprint)

# ---------------------------------------------------------------------------
# 3. Subdivide gets a real Cut Count parameter.
# ---------------------------------------------------------------------------
state_path = "crates/core/src/state.rs"
state = read(state_path)
if "pub subdivide_cuts: u32" not in state:
    state = state.replace(
        "    pub bevel_segments: u32,\n    pub revolve_segments: u32,\n",
        "    pub bevel_segments: u32,\n    pub subdivide_cuts: u32,\n    pub revolve_segments: u32,\n",
        1,
    )
    state = state.replace(
        "            bevel_segments: 1,\n            revolve_segments: 16,\n",
        "            bevel_segments: 1,\n            subdivide_cuts: 1,\n            revolve_segments: 16,\n",
        1,
    )
write(state_path, state)

subdivide_path = "crates/module-model/src/subdivide.rs"
subdivide = read(subdivide_path)
old = """        if let Some(m) = state.project.active_mesh_mut() {
            m.subdivide_selected();
        }
"""
new = """        let cuts = state.subdivide_cuts.clamp(1, 6);
        if let Some(m) = state.project.active_mesh_mut() {
            for _ in 0..cuts {
                m.subdivide_selected();
            }
        }
"""
if "let cuts = state.subdivide_cuts" not in subdivide:
    if old not in subdivide:
        raise SystemExit("subdivide apply block not found")
    subdivide = subdivide.replace(old, new, 1)
write(subdivide_path, subdivide)

model_ui_path = "crates/ui/src/modules_ui/model_ui.rs"
model_ui = read(model_ui_path)
if "state.subdivide_cuts" not in model_ui:
    marker = "    ui.small(state.t(\"hints.subdivide\"));\n    let reason = selection_guard(state);\n"
    insertion = r'''    ui.small(state.t("hints.subdivide"));
    let mut cuts = state.subdivide_cuts as i64;
    if ui
        .add(
            egui::Slider::new(&mut cuts, 1..=6)
                .text("Cuts")
                .step_by(1.0),
        )
        .changed()
    {
        state.subdivide_cuts = cuts.clamp(1, 6) as u32;
        state.mark_dirty();
    }
    let reason = selection_guard(state);
'''
    if marker not in model_ui:
        raise SystemExit("subdivide UI marker not found")
    model_ui = model_ui.replace(marker, insertion, 1)
write(model_ui_path, model_ui)

# ---------------------------------------------------------------------------
# 4. Toolbar lifecycle and explicit transform family.
# ---------------------------------------------------------------------------
toolbar_path = "crates/ui/src/toolbar.rs"
toolbar = read(toolbar_path)
if "fn apply_toolbar_entry(state: &mut AppState, tools: &ToolRegistry" not in toolbar:
    toolbar = toolbar.replace(
        """fn apply_toolbar_entry(state: &mut AppState, entry: &ToolbarEntry) {
    state.active_tool = entry.id.into();
    state.pending_modal = None;
    state.mark_dirty();
}
""",
        """fn apply_toolbar_entry(state: &mut AppState, tools: &ToolRegistry, entry: &ToolbarEntry) {
    state.active_tool = entry.id.into();
    state.pending_modal = None;
    if let Some(tool) = tools.get(entry.id) {
        tool.on_activate(state);
    }
    state.mark_dirty();
}
""",
        1,
    )
    toolbar = toolbar.replace(
        "fn draw_model_tools(ui: &mut egui::Ui, state: &mut AppState, _tools: &ToolRegistry, compact: bool)",
        "fn draw_model_tools(ui: &mut egui::Ui, state: &mut AppState, tools: &ToolRegistry, compact: bool)",
        1,
    )
    toolbar = toolbar.replace("draw_model_entry(ui, state, entry, true);", "draw_model_entry(ui, state, tools, entry, true);")
    toolbar = toolbar.replace("draw_model_entry(ui, state, entry, compact);", "draw_model_entry(ui, state, tools, entry, compact);")
    toolbar = toolbar.replace(
        "fn draw_model_entry(ui: &mut egui::Ui, state: &mut AppState, entry: &ToolbarEntry, compact: bool)",
        "fn draw_model_entry(ui: &mut egui::Ui, state: &mut AppState, tools: &ToolRegistry, entry: &ToolbarEntry, compact: bool)",
        1,
    )
    toolbar = toolbar.replace(
        "_ => draw_entry_button(ui, state, entry, compact),",
        "_ => draw_entry_button(ui, state, tools, entry, compact),",
        1,
    )
    toolbar = toolbar.replace(
        "fn draw_entry_button(ui: &mut egui::Ui, state: &mut AppState, entry: &ToolbarEntry, compact: bool)",
        "fn draw_entry_button(ui: &mut egui::Ui, state: &mut AppState, tools: &ToolRegistry, entry: &ToolbarEntry, compact: bool)",
        1,
    )
    toolbar = toolbar.replace("apply_toolbar_entry(state, entry);", "apply_toolbar_entry(state, tools, entry);")
write(toolbar_path, toolbar)

# ---------------------------------------------------------------------------
# 5. Universal transform gizmo and transform-only gizmo ownership.
# ---------------------------------------------------------------------------
gizmo_path = "crates/ui/src/gizmo.rs"
gizmo = read(gizmo_path)
if "pub struct GizmoInteraction" not in gizmo:
    marker = r'''pub enum GizmoHandle {
    Center,
    Axis(u8),
    /// Normal axis: 0 = YZ, 1 = XZ, 2 = XY.
    Plane(u8),
}
'''
    replacement = marker + r'''
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GizmoInteraction {
    pub kind: GizmoKind,
    pub handle: GizmoHandle,
}
'''
    if marker not in gizmo:
        raise SystemExit("GizmoHandle marker not found")
    gizmo = gizmo.replace(marker, replacement, 1)

if "pub fn draw_universal_gizmo_oriented" not in gizmo:
    universal = r'''
/// Combined Move/Rotate/Scale gizmo for the Universal Transform tool.
pub fn draw_universal_gizmo_oriented(
    painter: &Painter,
    camera: &Camera,
    viewport: Rect,
    pivot: Vec3,
    pointer: Option<Pos2>,
    axes: [Vec3; 3],
) -> Option<GizmoInteraction> {
    if viewport.height() <= 0.0 || !pivot.is_finite() {
        return None;
    }
    let depth = (pivot - camera.eye()).dot(camera.forward());
    if depth <= camera.near || depth >= camera.far {
        return None;
    }
    let world_per_point = match camera.proj {
        Projection::Ortho => 2.0 * camera.ortho_half_h / viewport.height(),
        Projection::Perspective => 2.0 * depth * (camera.fov_y * 0.5).tan() / viewport.height(),
    };
    let length = world_per_point * 80.0;
    let project = |point: Vec3| -> Option<Pos2> {
        let clip = camera.view_proj() * point.extend(1.0);
        if !clip.is_finite() || clip.w <= 0.0 || clip.z < 0.0 || clip.z > clip.w {
            return None;
        }
        Some(Pos2::new(
            viewport.center().x + clip.x / clip.w * viewport.width() * 0.5,
            viewport.center().y - clip.y / clip.w * viewport.height() * 0.5,
        ))
    };
    let center = project(pivot)?;
    let mut best: Option<(f32, GizmoInteraction)> = None;
    let mut consider = |distance: f32, kind: GizmoKind, handle: GizmoHandle| {
        if best.is_none_or(|(previous, _)| distance < previous) {
            best = Some((distance, GizmoInteraction { kind, handle }));
        }
    };

    let center_distance = pointer.map_or(f32::INFINITY, |point| point.distance(center));
    let center_hovered = center_distance <= 7.0;
    painter.circle(
        center,
        7.0,
        Color32::from_black_alpha(115),
        Stroke::new(
            1.5,
            if center_hovered { Color32::WHITE } else { Color32::from_white_alpha(190) },
        ),
    );
    painter.circle_filled(center, 2.5, Color32::from_white_alpha(210));
    if center_hovered {
        consider(center_distance, GizmoKind::Translate, GizmoHandle::Center);
    }

    for (normal, &base_color) in COLORS.iter().enumerate() {
        let a = axes[(normal + 1) % 3] * length;
        let b = axes[(normal + 2) % 3] * length;
        let corners: Option<Vec<_>> = [(0.18, 0.18), (0.34, 0.18), (0.34, 0.34), (0.18, 0.34)]
            .into_iter()
            .map(|(u, v)| project(pivot + a * u + b * v))
            .collect();
        let Some(corners) = corners else { continue; };
        let hovered = pointer.is_some_and(|point| inside_convex(&corners, point));
        let color = if hovered { Color32::WHITE } else { base_color };
        painter.add(Shape::convex_polygon(
            corners,
            color.gamma_multiply(0.20),
            Stroke::new(1.0, color),
        ));
        if hovered {
            consider(5.0, GizmoKind::Translate, GizmoHandle::Plane(normal as u8));
        }
    }

    for (axis, &base_color) in COLORS.iter().enumerate() {
        let a = axes[(axis + 1) % 3] * length * 0.58;
        let b = axes[(axis + 2) % 3] * length * 0.58;
        let mut previous = None;
        let mut ring_lines = Vec::new();
        let mut ring_distance = f32::INFINITY;
        for step in 0..=48 {
            let angle = step as f32 / 48.0 * std::f32::consts::TAU;
            let current = project(pivot + a * angle.cos() + b * angle.sin());
            if let (Some(start), Some(end)) = (previous, current) {
                if let Some(pointer) = pointer {
                    ring_distance = ring_distance.min(segment_distance(pointer, start, end));
                }
                ring_lines.push([start, end]);
            }
            previous = current;
        }
        let ring_hovered = ring_distance <= 5.0;
        for line in ring_lines {
            painter.line_segment(
                line,
                Stroke::new(
                    if ring_hovered { 3.0 } else { 1.5 },
                    if ring_hovered { Color32::WHITE } else { base_color.gamma_multiply(0.75) },
                ),
            );
        }
        if ring_hovered {
            consider(ring_distance, GizmoKind::Rotate, GizmoHandle::Axis(axis as u8));
        }

        let (Some(start), Some(move_end)) = (
            project(pivot + axes[axis] * length * 0.12),
            project(pivot + axes[axis] * length * 0.82),
        ) else {
            continue;
        };
        if start.distance(move_end) < 10.0 {
            continue;
        }
        let move_distance = pointer.map_or(f32::INFINITY, |point| segment_distance(point, start, move_end));
        let move_hovered = move_distance <= 6.0;
        let move_color = if move_hovered { Color32::WHITE } else { base_color };
        painter.line_segment([start, move_end], Stroke::new(5.0, Color32::from_black_alpha(170)));
        painter.line_segment([start, move_end], Stroke::new(2.5, move_color));
        let direction = (move_end - start).normalized();
        let perpendicular = Vec2::new(-direction.y, direction.x);
        painter.add(Shape::convex_polygon(
            vec![
                move_end,
                move_end - direction * 11.0 + perpendicular * 4.5,
                move_end - direction * 11.0 - perpendicular * 4.5,
            ],
            move_color,
            Stroke::NONE,
        ));
        if move_hovered {
            consider(move_distance, GizmoKind::Translate, GizmoHandle::Axis(axis as u8));
        }

        if let Some(scale_end) = project(pivot + axes[axis] * length * 1.04) {
            let scale_rect = Rect::from_center_size(scale_end, Vec2::splat(9.0));
            let scale_hovered = pointer.is_some_and(|point| scale_rect.expand(3.0).contains(point));
            let scale_color = if scale_hovered { Color32::WHITE } else { base_color };
            painter.line_segment([move_end, scale_end], Stroke::new(1.5, base_color.gamma_multiply(0.65)));
            painter.rect_filled(scale_rect, 1.5, scale_color);
            if scale_hovered {
                consider(1.0, GizmoKind::Scale, GizmoHandle::Axis(axis as u8));
            }
        }
    }

    best.map(|(_, interaction)| interaction)
}

'''
    marker = "fn segment_distance(point: Pos2, start: Pos2, end: Pos2) -> f32 {"
    if marker not in gizmo:
        raise SystemExit("segment_distance marker not found")
    gizmo = gizmo.replace(marker, universal + marker, 1)
write(gizmo_path, gizmo)

viewport_interaction_path = "crates/ui/src/viewport_interaction.rs"
viewport_interaction = read(viewport_interaction_path)
if "let transform_tool_active = matches!" not in viewport_interaction:
    pattern = re.compile(
        r"    if !state\.is_active_locked\(\)\n        && let Some\(mesh\) = state\.project\.active_mesh\(\)\.filter\(\|m\| \{.*?\n    \}\n    response\.context_menu",
        re.S,
    )
    replacement = r'''    let transform_tool_active = matches!(
        state.active_tool.as_str(),
        "transform" | "move" | "rotate" | "scale"
    );
    if transform_tool_active
        && !state.is_active_locked()
        && let Some(mesh) = state.project.active_mesh().filter(|m| {
            m.has_selection() || state.selection_domain() == petunia_core::SelectionDomain::Object
        })
    {
        let pivot = state.calculate_pivot(state.session.pivot_point);
        let axes = if state.transform_orientation == petunia_core::TransformOrientation::Local {
            crate::gizmo::local_axes_for_mesh(mesh)
        } else {
            [Vec3::X, Vec3::Y, Vec3::Z]
        };
        let interaction = if state.active_tool == "transform" {
            crate::gizmo::draw_universal_gizmo_oriented(
                painter,
                &state.camera,
                rect,
                pivot,
                pointer,
                axes,
            )
        } else {
            let kind = match state.active_tool.as_str() {
                "rotate" => GizmoKind::Rotate,
                "scale" => GizmoKind::Scale,
                _ => GizmoKind::Translate,
            };
            crate::gizmo::draw_gizmo_oriented(
                painter,
                &state.camera,
                rect,
                pivot,
                kind,
                pointer,
                axes,
            )
            .map(|handle| crate::gizmo::GizmoInteraction { kind, handle })
        };
        if let Some(interaction) = interaction {
            ctx.set_cursor_icon(egui::CursorIcon::Grab);
            if ctx.input(|i| i.pointer.button_pressed(PointerButton::Primary))
                && let Some(pos) = pointer
            {
                let constraint = match interaction.handle {
                    GizmoHandle::Center => ModalConstraint::Free,
                    GizmoHandle::Axis(a) => ModalConstraint::Axis(a as usize),
                    GizmoHandle::Plane(a) => ModalConstraint::Plane(a as usize),
                };
                let kind = match interaction.kind {
                    GizmoKind::Translate => ModalKind::Move,
                    GizmoKind::Rotate => ModalKind::Rotate,
                    GizmoKind::Scale => ModalKind::Scale,
                };
                modal_viewport::start_handle(ctx, state, kind, constraint, pos);
                state.ui.box_select_start = None;
                return true;
            }
        }
    }
    response.context_menu'''
    viewport_interaction, count = pattern.subn(replacement, viewport_interaction, count=1)
    if count != 1:
        raise SystemExit(f"viewport gizmo block replacement count: {count}")
write(viewport_interaction_path, viewport_interaction)

# ---------------------------------------------------------------------------
# 6. App shortcut routing + deterministic first window size.
# ---------------------------------------------------------------------------
app_path = "crates/app/src/lib.rs"
app = read(app_path)
old_route = r'''            "model.transform" => self.set_tool("transform", None),
            "model.rotate" | "model.scale" => {
                self.state.pending_modal = Some(if action == "model.rotate" {
                    petunia_core::modal::ModalKind::Rotate
                } else {
                    petunia_core::modal::ModalKind::Scale
                });
                self.state.active_tool = "transform".into();
                self.state.mark_dirty();
            }
'''
new_route = r'''            "model.transform" => self.set_tool("transform", None),
            "model.move" | "model.rotate" | "model.scale" => {
                let (tool_id, kind) = match action.as_str() {
                    "model.move" => ("move", petunia_core::modal::ModalKind::Move),
                    "model.rotate" => ("rotate", petunia_core::modal::ModalKind::Rotate),
                    _ => ("scale", petunia_core::modal::ModalKind::Scale),
                };
                self.state.pending_modal = Some(kind);
                self.state.active_tool = tool_id.into();
                self.state.mark_dirty();
            }
'''
if old_route in app:
    app = app.replace(old_route, new_route, 1)
app = app.replace('            "model.mirror" => self.set_tool("mirror", None),\n', '')
app = app.replace(
    'create_window(Window::default_attributes().with_title("Petunia3D"))',
    'create_window(\n                    Window::default_attributes()\n                        .with_title("Petunia3D")\n                        .with_inner_size(PhysicalSize::new(1280, 800)),\n                )',
    1,
)
app = app.replace(
    'Window::default_attributes().with_title("Petunia3D (OpenGL)")',
    'Window::default_attributes()\n                .with_title("Petunia3D (OpenGL)")\n                .with_inner_size(PhysicalSize::new(1280, 800))',
    1,
)
write(app_path, app)

# ---------------------------------------------------------------------------
# 7. Box Select only owns primary drag while active; viewport tool properties.
# ---------------------------------------------------------------------------
ui_lib_path = "crates/ui/src/lib.rs"
ui_lib = read(ui_lib_path)
if "pub mod tool_properties_popover;" not in ui_lib:
    ui_lib = ui_lib.replace("mod tool_fields;\n", "pub mod tool_fields;\npub mod tool_properties_popover;\n", 1)
if "tool_properties_popover::draw(ui, state, rect);" not in ui_lib:
    ui_lib = ui_lib.replace(
        "        primitive_card::draw_primitive_card(ui, state, rect);\n",
        "        primitive_card::draw_primitive_card(ui, state, rect);\n        tool_properties_popover::draw(ui, state, rect);\n",
        1,
    )
for needle in [
    "if resp.drag_started_by(egui::PointerButton::Primary)",
    "if resp.dragged_by(egui::PointerButton::Primary)",
    "if resp.drag_stopped_by(egui::PointerButton::Primary)",
]:
    guarded = 'if state.active_tool == "select_box"\n            && ' + needle[3:]
    if guarded not in ui_lib and needle in ui_lib:
        ui_lib = ui_lib.replace(needle, guarded, 1)
write(ui_lib_path, ui_lib)

# ---------------------------------------------------------------------------
# 8. 3D Cursor is a real explicit toolbar tool (Shift+RMB remains shortcut).
# ---------------------------------------------------------------------------
nav_path = "crates/ui/src/nav_gizmo.rs"
nav = read(nav_path)
old_cursor = r'''    if ctx.input(|i| {
        i.modifiers.shift
            && (i.pointer.button_pressed(PointerButton::Secondary)
                || i.pointer.button_clicked(PointerButton::Secondary))
    }) {
'''
new_cursor = r'''    let cursor_tool_click = state.active_tool == "cursor_3d"
        && ctx.input(|i| {
            i.pointer.button_pressed(PointerButton::Primary)
                || i.pointer.button_clicked(PointerButton::Primary)
        });
    let shortcut_click = ctx.input(|i| {
        i.modifiers.shift
            && (i.pointer.button_pressed(PointerButton::Secondary)
                || i.pointer.button_clicked(PointerButton::Secondary))
    });
    if state.active_tool == "cursor_3d" {
        ctx.set_cursor_icon(egui::CursorIcon::Crosshair);
    }
    if cursor_tool_click || shortcut_click {
'''
if "let cursor_tool_click = state.active_tool" not in nav:
    if old_cursor not in nav:
        raise SystemExit("3D cursor input block not found")
    nav = nav.replace(old_cursor, new_cursor, 1)
nav = nav.replace("ui.set_min_width(190.0);", "ui.set_min_width(150.0);", 1)
write(nav_path, nav)

# ---------------------------------------------------------------------------
# 9. Inspector owns persistent object data; transient tool params live viewport-local.
# ---------------------------------------------------------------------------
props_path = "crates/ui/src/properties_panel.rs"
props = read(props_path)
props = props.replace("use crate::tool_fields;\n", "")
props = props.replace(
    "    draw_geometry_section(ui, state, idx, false);\n    ui.add_space(gap);\n    draw_modifiers_section(ui, state, false);\n    ui.add_space(gap);\n    draw_display_section(ui, state, idx, false);",
    "    draw_geometry_section(ui, state, idx, false);\n    ui.add_space(gap);\n    draw_display_section(ui, state, idx, false);",
    1,
)
if "/// Aba Modifiers: propriedades persistentes" not in props:
    pattern = re.compile(r"/// Aba Modify:.*?/// Cena vazia:", re.S)
    replacement = r'''/// Aba Modifiers: propriedades persistentes do objeto. Tool Properties vivem no viewport.
fn draw_modify_tab(ui: &mut Ui, state: &mut AppState) {
    draw_modifiers_section(ui, state, true);
}

/// Cena vazia:'''
    props, count = pattern.subn(replacement, props, count=1)
    if count != 1:
        raise SystemExit(f"Modify tab replacement count: {count}")

# Replace placeholder modifiers section with a real persistent stack.
modifier_section_pattern = re.compile(r"/// Seção Modifiers:.*?/// Seção Display:", re.S)
if "modifier_stack_rows" not in props:
    modifier_section = r'''/// Seção Modifiers: pilha persistente, não destrutiva e reordenável.
fn draw_modifiers_section(ui: &mut Ui, state: &mut AppState, force_open: bool) {
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
            let Some(asset_idx) = inspected_asset_idx(state).filter(|idx| *idx < state.project.assets.len()) else {
                ui.label(RichText::new(state.t("empty.no_selection")).color(tokens::TEXT_MUTED));
                return;
            };

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
                            if ui.checkbox(&mut enabled, "").on_hover_text("Enable modifier").changed() {
                                state.project.assets[asset_idx].modifiers[modifier_idx].enabled = enabled;
                                changed = true;
                            }
                            let title = match snapshot.kind {
                                ModifierKind::Mirror { .. } => "Mirror",
                                ModifierKind::Symmetry { .. } => "Symmetry",
                            };
                            ui.strong(title);
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.small_button("×").on_hover_text("Remove modifier").clicked() {
                                    remove = Some(modifier_idx);
                                }
                                if ui.add_enabled(modifier_idx + 1 < modifier_count, egui::Button::new("↓")).clicked() {
                                    move_down = Some(modifier_idx);
                                }
                                if ui.add_enabled(modifier_idx > 0, egui::Button::new("↑")).clicked() {
                                    move_up = Some(modifier_idx);
                                }
                            });
                        });

                        match &mut state.project.assets[asset_idx].modifiers[modifier_idx].kind {
                            ModifierKind::Mirror { axis, weld } => {
                                ui.horizontal(|ui| {
                                    ui.label("Axis");
                                    for (candidate, name) in [(0, "X"), (1, "Y"), (2, "Z")] {
                                        if ui.selectable_label(*axis == candidate, name).clicked() {
                                            *axis = candidate;
                                            changed = true;
                                        }
                                    }
                                });
                                changed |= ui.add(egui::Slider::new(weld, 0.0..=0.05).text("Weld")).changed();
                            }
                            ModifierKind::Symmetry { axis, positive_to_negative, weld } => {
                                ui.horizontal(|ui| {
                                    ui.label("Axis");
                                    for (candidate, name) in [(0, "X"), (1, "Y"), (2, "Z")] {
                                        if ui.selectable_label(*axis == candidate, name).clicked() {
                                            *axis = candidate;
                                            changed = true;
                                        }
                                    }
                                });
                                changed |= ui.checkbox(positive_to_negative, "+ → −").changed();
                                changed |= ui.add(egui::Slider::new(weld, 0.0..=0.05).text("Weld")).changed();
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
                state.project.assets[asset_idx].modifiers.swap(index, index - 1);
                changed = true;
            } else if let Some(index) = move_down {
                state.project.assets[asset_idx].modifiers.swap(index, index + 1);
                changed = true;
            }

            if modifier_count == 0 {
                ui.label(RichText::new(state.t("modifiers.empty")).size(11.0).color(tokens::TEXT_MUTED));
            }

            ui.horizontal(|ui| {
                if ui.button("+ Mirror").clicked() {
                    state.project.assets[asset_idx].modifiers.push(ModifierInstance::mirror(0, 0.001));
                    changed = true;
                }
                if ui.button("+ Symmetry").clicked() {
                    state.project.assets[asset_idx].modifiers.push(ModifierInstance::symmetry(0, true, 0.001));
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

/// Seção Display:'''
    props, count = modifier_section_pattern.subn(modifier_section, props, count=1)
    if count != 1:
        raise SystemExit(f"modifier section replacement count: {count}")

# Material is authoritative; don't mirror resource values into legacy asset fields.
material_sync = r'''            if should_emit_change {
                if let Some(mat) = state.project.project.get_material(mat_id).cloned()
                    && let Some(o) = state.project.assets.get_mut(active_idx)
                {
                    o.base_color = [mat.base_color[0], mat.base_color[1], mat.base_color[2]];
                    if let Some(tex) = mat.albedo_texture {
                        o.texture = Some(tex);
                    }
                }
                state.render.canvas_dirty = true;
                state.emit_mesh_changed();
'''
if material_sync in props:
    props = props.replace(
        material_sync,
        """            if should_emit_change {
                state.render.canvas_dirty = true;
                state.emit_mesh_changed();
""",
        1,
    )

# Add a delete action next to duplicate when the current implementation has none.
if 'on_hover_text("Excluir material ativo")' not in props:
    duplicate_marker = r'''                if ui
                    .button("⧉")
                    .on_hover_text("Duplicar material ativo")
                    .clicked()
                    && let Some(cur_mat) = state.project.project.active_material().cloned()
                {
                    state.checkpoint("duplicate material");
                    let dup = cur_mat.duplicate();
                    let dup_id = state.project.project.add_material(dup);
                    if let Some(a) = state.project.assets.get_mut(active_idx) {
                        a.material_id = Some(dup_id);
                    }
                    state.mark_dirty();
                }
'''
    if duplicate_marker in props:
        props = props.replace(
            duplicate_marker,
            duplicate_marker + r'''
                if let Some(material_id) = active_mat_id
                    && ui.button("×").on_hover_text("Excluir material ativo").clicked()
                {
                    state.checkpoint("delete material");
                    if state.project.project.remove_material(material_id) {
                        state.render.canvas_dirty = true;
                        state.emit_mesh_changed();
                        state.mark_dirty();
                    }
                }
''',
            1,
        )
write(props_path, props)

for locale_path, old_value, new_value in [
    ("assets/locales/en.toml", 'tab_modify = "Modify"', 'tab_modify = "Modifiers"'),
    ("assets/locales/pt-BR.toml", 'tab_modify = "Modificar"', 'tab_modify = "Modificadores"'),
]:
    locale = read(locale_path)
    locale = locale.replace(old_value, new_value)
    write(locale_path, locale)

# Narrow a few known over-wide popup minimums.
for path, old, new in [
    ("crates/ui/src/viewport_bar.rs", "ui.set_min_width(210.0);", "ui.set_min_width(168.0);"),
    ("crates/ui/src/viewport_bar.rs", "ui.set_min_width(160.0);", "ui.set_min_width(144.0);"),
    ("crates/ui/src/outliner.rs", "ui.set_min_width(200.0);", "ui.set_min_width(160.0);"),
    ("crates/ui/src/contextual_shelf.rs", "ui.set_min_width(165.0);", "ui.set_min_width(148.0);"),
]:
    text = read(path)
    if old in text:
        text = text.replace(old, new, 1)
        write(path, text)

# ---------------------------------------------------------------------------
# 10. Toolbar Golden Reference SVGs, rasterized once and cached by egui.
# ---------------------------------------------------------------------------
ui_cargo_path = "crates/ui/Cargo.toml"
ui_cargo = read(ui_cargo_path)
if 'resvg = "0.45"' not in ui_cargo:
    ui_cargo = ui_cargo.replace('image = { version = "0.25", default-features = false, features = ["png", "jpeg"] }\n', 'image = { version = "0.25", default-features = false, features = ["png", "jpeg"] }\nresvg = "0.45"\n', 1)
write(ui_cargo_path, ui_cargo)

svg_sources = {
    "select_box": "tool_01_select_box_40x40.svg",
    "cursor_3d": "tool_02_cursor_40x40.svg",
    "move": "tool_03_move_40x40.svg",
    "rotate": "tool_04_rotate_40x40.svg",
    "scale": "tool_05_scale_40x40.svg",
    "transform": "tool_06_transform_40x40.svg",
    "annotate": "tool_07_annotate_40x40.svg",
    "measure": "tool_08_measure_40x40.svg",
    "add_primitive": "tool_09_add_primitive_40x40.svg",
}
svg_dest = ROOT / "assets/ui/icons/toolbar"
svg_dest.mkdir(parents=True, exist_ok=True)
for icon_id, filename in svg_sources.items():
    source = ROOT / "docs/image-references/extracted/toolbar/buttons" / filename
    if not source.exists():
        raise SystemExit(f"missing Golden Reference SVG: {source}")
    shutil.copyfile(source, svg_dest / f"{icon_id}.svg")

icon_registry_path = "crates/ui/src/icon_registry.rs"
icons = read(icon_registry_path)
if "SVG_TOOL_SELECT_BOX" not in icons:
    marker = "// -------------------------------------------------- Bytes Embutidos dos PNGs de Properties Tabs\n"
    svg_consts = r'''// -------------------------------------------------- Golden Reference SVG toolbar assets
const SVG_TOOL_SELECT_BOX: &str = include_str!("../../../assets/ui/icons/toolbar/select_box.svg");
const SVG_TOOL_CURSOR_3D: &str = include_str!("../../../assets/ui/icons/toolbar/cursor_3d.svg");
const SVG_TOOL_MOVE: &str = include_str!("../../../assets/ui/icons/toolbar/move.svg");
const SVG_TOOL_ROTATE: &str = include_str!("../../../assets/ui/icons/toolbar/rotate.svg");
const SVG_TOOL_SCALE: &str = include_str!("../../../assets/ui/icons/toolbar/scale.svg");
const SVG_TOOL_TRANSFORM: &str = include_str!("../../../assets/ui/icons/toolbar/transform.svg");
const SVG_TOOL_ANNOTATE: &str = include_str!("../../../assets/ui/icons/toolbar/annotate.svg");
const SVG_TOOL_MEASURE: &str = include_str!("../../../assets/ui/icons/toolbar/measure.svg");
const SVG_TOOL_ADD_PRIMITIVE: &str = include_str!("../../../assets/ui/icons/toolbar/add_primitive.svg");

fn embedded_toolbar_svg(id: &str) -> Option<&'static str> {
    match id {
        "select_box" => Some(SVG_TOOL_SELECT_BOX),
        "cursor_3d" => Some(SVG_TOOL_CURSOR_3D),
        "move" => Some(SVG_TOOL_MOVE),
        "rotate" => Some(SVG_TOOL_ROTATE),
        "scale" => Some(SVG_TOOL_SCALE),
        "transform" => Some(SVG_TOOL_TRANSFORM),
        "annotate" => Some(SVG_TOOL_ANNOTATE),
        "measure" => Some(SVG_TOOL_MEASURE),
        "add_primitive" => Some(SVG_TOOL_ADD_PRIMITIVE),
        _ => None,
    }
}

fn rasterize_svg(svg: &str, size: u32) -> Option<ColorImage> {
    let tree = resvg::usvg::Tree::from_str(svg, &resvg::usvg::Options::default()).ok()?;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(size, size)?;
    let source = tree.size();
    let scale_x = size as f32 / source.width();
    let scale_y = size as f32 / source.height();
    let scale = scale_x.min(scale_y);
    let transform = resvg::tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    Some(ColorImage::from_rgba_unmultiplied(
        [size as usize, size as usize],
        pixmap.data(),
    ))
}

fn get_or_load_toolbar_svg(ctx: &Context, id: &str) -> Option<TextureHandle> {
    let cache_key = format!("svg-toolbar:{id}");
    if let Ok(cache) = TEXTURE_CACHE.read()
        && let Some(handle) = cache.get(&cache_key)
    {
        return Some(handle.clone());
    }
    let image = rasterize_svg(embedded_toolbar_svg(id)?, 64)?;
    let handle = ctx.load_texture(cache_key.clone(), image, TextureOptions::LINEAR);
    if let Ok(mut cache) = TEXTURE_CACHE.write() {
        cache.insert(cache_key, handle.clone());
    }
    Some(handle)
}

'''
    if marker not in icons:
        raise SystemExit("icon registry PNG marker not found")
    icons = icons.replace(marker, svg_consts + marker, 1)

old_vector = r'''        if is_toolbar_vector_tool(&id) {
            icons::paint(painter, &id, target_rect, tint);
            return;
        }
'''
new_vector = r'''        if is_toolbar_vector_tool(&id) {
            if let Some(texture) = get_or_load_toolbar_svg(ctx, &id) {
                let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
                painter.image(texture.id(), target_rect, uv, tint);
            } else {
                icons::paint(painter, &id, target_rect, tint);
            }
            return;
        }
'''
if old_vector in icons:
    icons = icons.replace(old_vector, new_vector, 1)
write(icon_registry_path, icons)

# Petunia package mapping documents the shipped toolbar semantics.
pack_map = ROOT / "assets/icons/petunia/icons.toml"
pack_map.parent.mkdir(parents=True, exist_ok=True)
pack_map.write_text("""# Canonical Petunia icon pack mapping.\n# Toolbar assets come from the extracted Golden Reference SVGs.\n\n[icons]\nselect_box = \"../../ui/icons/toolbar/select_box.svg\"\ncursor_3d = \"../../ui/icons/toolbar/cursor_3d.svg\"\nmove = \"../../ui/icons/toolbar/move.svg\"\nrotate = \"../../ui/icons/toolbar/rotate.svg\"\nscale = \"../../ui/icons/toolbar/scale.svg\"\ntransform = \"../../ui/icons/toolbar/transform.svg\"\nannotate = \"../../ui/icons/toolbar/annotate.svg\"\nmeasure = \"../../ui/icons/toolbar/measure.svg\"\nadd_primitive = \"../../ui/icons/toolbar/add_primitive.svg\"\n""")

print("modeling convergence patch applied")
