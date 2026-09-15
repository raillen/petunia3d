#!/usr/bin/env python3
"""Apply Wave 3 visual QA / viewport UX polish on the convergence branch.

This staging script is intentionally removed by the validation workflow after
all Rust/i18n gates pass. The resulting commit contains source changes only.
"""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def read(path: str) -> str:
    return (ROOT / path).read_text()


def write(path: str, content: str) -> None:
    (ROOT / path).write_text(content)


def replace_once(path: str, old: str, new: str) -> None:
    content = read(path)
    count = content.count(old)
    if count != 1:
        raise RuntimeError(f"{path}: expected one anchor, found {count}: {old[:80]!r}")
    write(path, content.replace(old, new, 1))


# ---------------------------------------------------------------------------
# 1. Responsive, theme-aware viewport Tool Properties with a measurable rect.
# ---------------------------------------------------------------------------
write(
    "crates/ui/src/tool_properties_popover.rs",
    r'''//! Responsive viewport-local Tool Properties.
//!
//! This surface is deliberately separate from Object Properties and the
//! Modifier Stack. Wave 3 makes the card viewport-bounded and returns its
//! actual rectangle so viewport hit-testing can never click through it.

use egui::{RichText, Ui, Vec2};
use petunia_core::{AppState, Workspace};

use crate::tokens;

const VIEWPORT_MARGIN: f32 = 12.0;
const MIN_USABLE_WIDTH: f32 = 112.0;
const MIN_USABLE_HEIGHT: f32 = 96.0;
const REGULAR_CONTENT_WIDTH: f32 = 280.0;
const COMPACT_CONTENT_WIDTH: f32 = 220.0;
const REGULAR_MAX_HEIGHT: f32 = 520.0;
const COMPACT_MAX_HEIGHT: f32 = 320.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PopoverLayout {
    pub position: egui::Pos2,
    /// Width available to the content; frame margins are reserved separately.
    pub content_width: f32,
    /// Maximum total height budget inside the viewport.
    pub max_height: f32,
    pub compact: bool,
}

/// Computes a viewport-safe layout without reading global screen geometry.
pub fn layout_for_viewport(viewport: egui::Rect) -> Option<PopoverLayout> {
    let usable_width = (viewport.width() - VIEWPORT_MARGIN * 2.0).max(0.0);
    let usable_height = (viewport.height() - VIEWPORT_MARGIN * 2.0).max(0.0);
    if usable_width < MIN_USABLE_WIDTH || usable_height < MIN_USABLE_HEIGHT {
        return None;
    }

    let compact = usable_width < 300.0 || usable_height < 360.0;
    // Reserve 22 points for frame margins/stroke so the measured window stays
    // inside the usable viewport even at narrow sizes.
    let content_cap = if compact {
        COMPACT_CONTENT_WIDTH
    } else {
        REGULAR_CONTENT_WIDTH
    };
    let content_width = (usable_width - 22.0).min(content_cap).max(80.0);
    let max_height = usable_height
        .min(if compact {
            COMPACT_MAX_HEIGHT
        } else {
            REGULAR_MAX_HEIGHT
        })
        .max(80.0);

    Some(PopoverLayout {
        position: viewport.left_top() + Vec2::splat(VIEWPORT_MARGIN),
        content_width,
        max_height,
        compact,
    })
}

/// Draws canonical Tool Properties and returns the occupied viewport rectangle.
pub fn draw(ui: &mut Ui, state: &mut AppState, viewport: egui::Rect) -> Option<egui::Rect> {
    if state.workspace != Workspace::Model
        || state.is_active_locked()
        || state.session.primitive_session.is_some()
        || !crate::modeling_tool_properties::supports(state)
    {
        return None;
    }
    let layout = layout_for_viewport(viewport)?;

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
                if translated == key {
                    active.clone()
                } else {
                    translated
                }
            }
        });

    let frame = egui::Frame::new()
        .fill(tokens::bg_panel(state))
        .stroke(tokens::stroke_border_dyn(state))
        .corner_radius(tokens::RADIUS_CONTAINER)
        .inner_margin(egui::Margin::symmetric(10, 8));

    let window = egui::Window::new("viewport.tool_properties.window")
        .id(egui::Id::new("viewport.tool_properties"))
        .title_bar(false)
        .fixed_pos(layout.position)
        .default_width(layout.content_width)
        .max_width(layout.content_width + 20.0)
        .max_height(layout.max_height)
        .constrain_to(viewport)
        .collapsible(false)
        .resizable(false)
        .frame(frame)
        .show(ui.ctx(), |ui| {
            ui.set_width(layout.content_width);
            ui.spacing_mut().item_spacing = if layout.compact {
                egui::vec2(5.0, 4.0)
            } else {
                egui::vec2(6.0, 5.0)
            };

            ui.label(
                RichText::new(&title)
                    .strong()
                    .size(if layout.compact { 11.5 } else { 12.5 })
                    .color(tokens::text_primary(state)),
            );
            ui.label(
                RichText::new(state.t("tool_properties.title"))
                    .size(10.0)
                    .color(tokens::text_muted(state)),
            );
            ui.separator();

            // Header + frame consume roughly 48 pt. The inner ScrollArea keeps
            // long tool forms usable instead of escaping a short viewport.
            let body_height = (layout.max_height - 48.0).max(48.0);
            egui::ScrollArea::vertical()
                .id_salt("viewport.tool_properties.scroll")
                .max_height(body_height)
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    crate::modeling_tool_properties::draw(ui, state);
                });
        });

    window.map(|window| window.response.rect.intersect(viewport))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_is_bounded_and_regular_on_large_viewports() {
        let viewport = egui::Rect::from_min_size(
            egui::pos2(40.0, 60.0),
            egui::vec2(1280.0, 800.0),
        );
        let layout = layout_for_viewport(viewport).expect("large viewport");
        assert!(!layout.compact);
        assert_eq!(layout.position, viewport.left_top() + Vec2::splat(VIEWPORT_MARGIN));
        assert!(layout.content_width <= REGULAR_CONTENT_WIDTH);
        assert!(layout.max_height <= REGULAR_MAX_HEIGHT);
    }

    #[test]
    fn layout_compacts_on_narrow_viewports() {
        let viewport = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(260.0, 260.0));
        let layout = layout_for_viewport(viewport).expect("compact viewport");
        assert!(layout.compact);
        assert!(layout.content_width + 22.0 <= viewport.width() - VIEWPORT_MARGIN * 2.0 + 0.01);
        assert!(layout.max_height <= viewport.height() - VIEWPORT_MARGIN * 2.0 + 0.01);
    }

    #[test]
    fn unusably_small_viewport_suppresses_overlay() {
        let viewport = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(100.0, 80.0));
        assert!(layout_for_viewport(viewport).is_none());
    }

    #[test]
    fn primitive_session_owns_contextual_surface() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("en");
        state.workspace = Workspace::Model;
        state.active_tool = "primitives".to_owned();
        assert!(state.begin_primitive(petunia_core::PrimitiveKind::Cube, None));
        let viewport = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
        ctx.run_ui(egui::RawInput::default(), |ui| {
            assert!(draw(ui, &mut state, viewport).is_none());
        })
        .textures_delta
        .clear();
    }
}
''',
)

# ---------------------------------------------------------------------------
# 2. Last Operation card: return hit rect + respect narrow viewport budgets.
# ---------------------------------------------------------------------------
primitive_path = "crates/ui/src/primitive_card.rs"
primitive = read(primitive_path)
primitive = primitive.replace(
    "pub fn draw_primitive_card(ui: &mut Ui, state: &mut AppState, viewport_rect: Rect) {\n",
    "pub fn draw_primitive_card(\n    ui: &mut Ui,\n    state: &mut AppState,\n    viewport_rect: Rect,\n) -> Option<Rect> {\n",
    1,
)
primitive = primitive.replace(
    "        state.finalize_primitive_session();\n        return;\n",
    "        state.finalize_primitive_session();\n        return None;\n",
    1,
)
primitive = primitive.replace(
    "    let Some(session) = state.session.primitive_session.clone() else {\n        return;\n    };\n",
    "    let Some(session) = state.session.primitive_session.clone() else {\n        return None;\n    };\n",
    1,
)
primitive = primitive.replace(
    "    let anchor = creation_anchor(state, viewport_rect, session.asset_id);\n\n    let mut next",
    "    let anchor = creation_anchor(state, viewport_rect, session.asset_id);\n    let max_card_width = (viewport_rect.width() - 24.0).max(96.0).min(300.0);\n    let max_card_height = (viewport_rect.height() - 24.0).max(96.0).min(520.0);\n\n    let mut next",
    1,
)
primitive = primitive.replace(
    "        .default_pos(anchor)\n        .constrain_to(viewport_rect)",
    "        .default_pos(anchor)\n        .default_width(max_card_width.min(260.0))\n        .max_width(max_card_width)\n        .max_height(max_card_height)\n        .constrain_to(viewport_rect)",
    1,
)
primitive = primitive.replace(
    "    if cancel {\n        state.cancel_primitive();\n    } else if confirm {\n        state.confirm_primitive();\n    }\n}\n\n/// Campos por espécie.",
    "    if cancel {\n        state.cancel_primitive();\n    } else if confirm {\n        state.confirm_primitive();\n    }\n\n    win.map(|window| window.response.rect.intersect(viewport_rect))\n}\n\n/// Campos por espécie.",
    1,
)
primitive = primitive.replace(
    "vec2(110.0, 20.0),",
    "vec2(ui.available_width().min(110.0).max(40.0), 20.0),",
)
primitive = primitive.replace(
    ".width(ui.available_width().clamp(96.0, 160.0))",
    ".width(ui.available_width().min(160.0).max(64.0))",
)
primitive = primitive.replace(
    "                draw_primitive_card(ui, &mut state, viewport);",
    "                let _ = draw_primitive_card(ui, &mut state, viewport);",
)
write(primitive_path, primitive)

# ---------------------------------------------------------------------------
# 3. Canonical region bookkeeping for viewport overlays + tiny modal fix.
# ---------------------------------------------------------------------------
regions_path = "crates/ui/src/regions.rs"
regions = read(regions_path)
regions = regions.replace(
    "    Viewport,\n    Shelf,\n}",
    "    Viewport,\n    Shelf,\n    ToolProperties,\n    PrimitiveCard,\n}",
    1,
)
regions = regions.replace(
    "    pub viewport: Option<egui::Rect>,\n    pub shelf: Option<egui::Rect>,\n}",
    "    pub viewport: Option<egui::Rect>,\n    pub shelf: Option<egui::Rect>,\n    pub tool_properties: Option<egui::Rect>,\n    pub primitive_card: Option<egui::Rect>,\n}",
    1,
)
regions = regions.replace(
    "            RegionSlot::Viewport => &mut self.viewport,\n            RegionSlot::Shelf => &mut self.shelf,\n",
    "            RegionSlot::Viewport => &mut self.viewport,\n            RegionSlot::Shelf => &mut self.shelf,\n            RegionSlot::ToolProperties => &mut self.tool_properties,\n            RegionSlot::PrimitiveCard => &mut self.primitive_card,\n",
    1,
)
needle = '''    pub fn shelf_within_viewport(&self) -> bool {
        match (self.shelf, self.viewport) {
            (Some(shelf), Some(viewport)) => viewport.contains_rect(shelf),
            // Sem shelf visível: invariante trivialmente satisfeito.
            (None, _) => true,
            // Shelf sem viewport conhecida: não verificável.
            (Some(_), None) => false,
        }
    }
'''
replacement = needle + '''
    /// Every interactive overlay owned by the viewport must remain inside its
    /// canonical safe rectangle. Hidden overlays are ignored.
    pub fn viewport_overlays_within_viewport(&self) -> bool {
        let overlays = [self.shelf, self.tool_properties, self.primitive_card];
        if overlays.iter().all(Option::is_none) {
            return true;
        }
        let Some(viewport) = self.viewport else {
            return false;
        };
        overlays
            .into_iter()
            .flatten()
            .all(|overlay| viewport.contains_rect(overlay))
    }
'''
if regions.count(needle) != 1:
    raise RuntimeError("regions.rs: shelf invariant anchor changed")
regions = regions.replace(needle, replacement, 1)
old_modal = '''    let min = egui::vec2(
        min_req.x.min(avail.x).max(200.0),
        min_req.y.min(avail.y).max(160.0),
    );
'''
new_modal = '''    // Floors themselves are capped by the available size; otherwise a tiny
    // application window could paradoxically produce a modal larger than its
    // viewport (the previous `max(200/160)` ordering did exactly that).
    let min = egui::vec2(
        min_req.x.min(avail.x).max(200.0_f32.min(avail.x)),
        min_req.y.min(avail.y).max(160.0_f32.min(avail.y)),
    );
'''
if regions.count(old_modal) != 1:
    raise RuntimeError("regions.rs: modal min anchor changed")
regions = regions.replace(old_modal, new_modal, 1)
regions = regions.replace(
    "        assert!(regions.shelf_within_viewport());\n        assert!(regions.dock_sections_disjoint());",
    "        assert!(regions.shelf_within_viewport());\n        assert!(regions.viewport_overlays_within_viewport());\n        assert!(regions.dock_sections_disjoint());",
    1,
)
insert = r'''

    #[test]
    fn viewport_overlay_invariant_covers_tool_and_primitive_cards() {
        let viewport = rect(0.0, 0.0, 800.0, 600.0);
        let valid = UiRegions {
            viewport: Some(viewport),
            tool_properties: Some(rect(12.0, 12.0, 280.0, 300.0)),
            primitive_card: Some(rect(420.0, 180.0, 690.0, 420.0)),
            ..Default::default()
        };
        assert!(valid.viewport_overlays_within_viewport());

        let invalid = UiRegions {
            primitive_card: Some(rect(700.0, 500.0, 900.0, 650.0)),
            ..valid
        };
        assert!(!invalid.viewport_overlays_within_viewport());
    }

    #[test]
    fn modal_sizes_stay_inside_tiny_viewports() {
        let viewport = rect(0.0, 0.0, 150.0, 120.0);
        let (default, min, max) = modal_sizes(
            viewport,
            egui::vec2(720.0, 560.0),
            egui::vec2(480.0, 360.0),
            egui::vec2(860.0, 720.0),
        );
        let avail = egui::vec2(126.0, 96.0);
        for size in [default, min, max] {
            assert!(size.x <= avail.x + f32::EPSILON);
            assert!(size.y <= avail.y + f32::EPSILON);
            assert!(size.x >= 0.0 && size.y >= 0.0);
        }
    }
'''
last = regions.rfind("\n}")
if last < 0:
    raise RuntimeError("regions.rs: test module closing brace missing")
regions = regions[:last] + insert + regions[last:]
write(regions_path, regions)

# ---------------------------------------------------------------------------
# 4. Viewport owns overlay hit-testing: no primitive/tool card click-through.
# ---------------------------------------------------------------------------
lib_path = "crates/ui/src/lib.rs"
lib = read(lib_path)
old = '''        // Cartão Last Operation da criação ativa (Wave 8).
        let had_primitive_session = state.session.primitive_session.is_some();
        primitive_card::draw_primitive_card(ui, state, rect);
        tool_properties_popover::draw(ui, state, rect);
        let pointer_on_shelf = shelf_rect.is_some_and(|sr| {
            ui.input(|i| {
                i.pointer
                    .interact_pos()
                    .or(i.pointer.hover_pos())
                    .is_some_and(|pos| sr.contains(pos))
            })
        });

        if pointer_on_shelf {
            state.ui.box_select_start = None;
            return;
        }
'''
new = '''        // Viewport-local interactive chrome. Every surface returns its actual
        // rect, which becomes both QA evidence and a hit-test exclusion zone.
        let had_primitive_session = state.session.primitive_session.is_some();
        let primitive_card_rect = primitive_card::draw_primitive_card(ui, state, rect);
        if let Some(card_rect) = primitive_card_rect {
            regions::record(&ctx, regions::RegionSlot::PrimitiveCard, card_rect);
        }
        // Avoid flashing a second contextual surface on the same frame that a
        // primitive card confirms/cancels itself.
        let tool_properties_rect = if had_primitive_session {
            None
        } else {
            tool_properties_popover::draw(ui, state, rect)
        };
        if let Some(tool_rect) = tool_properties_rect {
            regions::record(&ctx, regions::RegionSlot::ToolProperties, tool_rect);
        }

        let pointer_on_viewport_chrome = ui.input(|i| {
            i.pointer
                .interact_pos()
                .or(i.pointer.hover_pos())
                .is_some_and(|pos| {
                    [shelf_rect, primitive_card_rect, tool_properties_rect]
                        .into_iter()
                        .flatten()
                        .any(|overlay| overlay.contains(pos))
                })
        });

        if pointer_on_viewport_chrome {
            // Critical Wave 3 invariant: buttons, fields and scroll gestures in
            // viewport chrome must never become box-select starts or GPU picks.
            state.ui.box_select_start = None;
            return;
        }
'''
if lib.count(old) != 1:
    raise RuntimeError("lib.rs: viewport overlay block changed")
lib = lib.replace(old, new, 1)
write(lib_path, lib)

# ---------------------------------------------------------------------------
# 5. Gizmo visual scale follows the usable viewport while preserving DPI size.
# ---------------------------------------------------------------------------
gizmo_path = "crates/ui/src/gizmo.rs"
gizmo = read(gizmo_path)
axis_anchor = "const AXES: [Vec3; 3] = [Vec3::X, Vec3::Y, Vec3::Z];\n"
if gizmo.count(axis_anchor) != 1:
    raise RuntimeError("gizmo.rs: axis anchor changed")
gizmo = gizmo.replace(
    axis_anchor,
    axis_anchor
    + '''\n/// Visual gizmo extent in logical points. Large viewports retain the familiar\n/// 80pt reach; narrow panes shrink the gizmo instead of covering the model.\nfn gizmo_extent_points(viewport: Rect) -> f32 {\n    (viewport.width().min(viewport.height()) * 0.14).clamp(56.0, 80.0)\n}\n''',
    1,
)
if gizmo.count("world_per_point * 80.0") != 2:
    raise RuntimeError("gizmo.rs: expected two fixed-length gizmos")
gizmo = gizmo.replace("world_per_point * 80.0", "world_per_point * gizmo_extent_points(viewport)")
gizmo_test = r'''

    #[test]
    fn gizmo_extent_is_responsive_but_keeps_accessible_floor() {
        let tiny = Rect::from_min_size(Pos2::ZERO, Vec2::new(220.0, 180.0));
        let medium = Rect::from_min_size(Pos2::ZERO, Vec2::new(520.0, 420.0));
        let large = Rect::from_min_size(Pos2::ZERO, Vec2::new(1280.0, 800.0));
        assert_eq!(gizmo_extent_points(tiny), 56.0);
        assert!(gizmo_extent_points(medium) > gizmo_extent_points(tiny));
        assert_eq!(gizmo_extent_points(large), 80.0);
    }
'''
last = gizmo.rfind("\n}")
if last < 0:
    raise RuntimeError("gizmo.rs: test module closing brace missing")
gizmo = gizmo[:last] + gizmo_test + gizmo[last:]
write(gizmo_path, gizmo)

# ---------------------------------------------------------------------------
# 6. Viewport context menu and DnD feedback: remove visible hard-coded language.
# ---------------------------------------------------------------------------
interaction_path = "crates/ui/src/viewport_interaction.rs"
interaction = read(interaction_path)
if interaction.count('"Soltar para Instanciar"') != 1:
    raise RuntimeError("viewport_interaction.rs: DnD label anchor changed")
interaction = interaction.replace(
    '"Soltar para Instanciar"',
    'state.t("viewport.drop_to_instantiate")',
    1,
)
start = interaction.index("    response.context_menu(|ui| {")
end = interaction.index("    false\n}", start)
context_block = r'''    let object_locked_label = state.t("context.object_locked");
    let modeling_label = state.t("context.modeling");
    let move_label = state.t("tools.move");
    let rotate_label = state.t("tools.rotate");
    let scale_label = state.t("tools.scale");
    let extrude_label = state.t("tools.extrude");
    let inset_label = state.t("tools.inset");
    let pushpull_label = state.t("tools.pushpull");
    let bevel_label = state.t("tools.bevel");
    response.context_menu(|ui| {
        if state.is_active_locked() {
            ui.label(
                egui::RichText::new(object_locked_label.as_str())
                    .italics()
                    .color(tokens::TEXT_MUTED),
            );
            return;
        }
        ui.label(
            egui::RichText::new(modeling_label.as_str())
                .strong()
                .color(tokens::TEXT_PRIMARY),
        );
        ui.separator();
        if widgets::PetuniaMenuItem::new(move_label.as_str())
            .icon(PetuniaIcon::Move)
            .shortcut(Some("G"))
            .show(ui)
            .clicked()
        {
            state.pending_modal = Some(ModalKind::Move);
            ui.close();
        }
        if widgets::PetuniaMenuItem::new(rotate_label.as_str())
            .icon(PetuniaIcon::Rotate)
            .shortcut(Some("R"))
            .show(ui)
            .clicked()
        {
            state.pending_modal = Some(ModalKind::Rotate);
            ui.close();
        }
        if widgets::PetuniaMenuItem::new(scale_label.as_str())
            .icon(PetuniaIcon::Scale)
            .shortcut(Some("S"))
            .show(ui)
            .clicked()
        {
            state.pending_modal = Some(ModalKind::Scale);
            ui.close();
        }
        let faces = state
            .project
            .active_mesh()
            .is_some_and(|m| m.selected_face_count() > 0);
        if faces {
            ui.separator();
            if widgets::PetuniaMenuItem::new(extrude_label.as_str())
                .icon(PetuniaIcon::Extrude)
                .shortcut(Some("E"))
                .show(ui)
                .clicked()
            {
                state.pending_modal = Some(ModalKind::Extrude);
                ui.close();
            }
            if widgets::PetuniaMenuItem::new(inset_label.as_str())
                .icon(PetuniaIcon::Inset)
                .shortcut(Some("I"))
                .show(ui)
                .clicked()
            {
                state.pending_modal = Some(ModalKind::Inset);
                ui.close();
            }
            if widgets::PetuniaMenuItem::new(pushpull_label.as_str())
                .icon(PetuniaIcon::PushPull)
                .shortcut(Some("P"))
                .show(ui)
                .clicked()
            {
                state.pending_modal = Some(ModalKind::PushPull);
                ui.close();
            }
        }
        if state
            .project
            .active_mesh()
            .is_some_and(|m| !m.selected_edges.is_empty())
            && widgets::PetuniaMenuItem::new(bevel_label.as_str())
                .icon(PetuniaIcon::Bevel)
                .shortcut(Some("Ctrl+B"))
                .show(ui)
                .clicked()
        {
            state.pending_modal = Some(ModalKind::Bevel);
            ui.close();
        }
    });
'''
interaction = interaction[:start] + context_block + interaction[end:]
write(interaction_path, interaction)

# ---------------------------------------------------------------------------
# 7. Modifier Stack visual/responsive polish using canonical theme tokens.
# ---------------------------------------------------------------------------
properties_path = "crates/ui/src/properties_panel.rs"
properties = read(properties_path)
fn_start = properties.index("fn draw_modifiers_section(")
fn_end = properties.index("/// Seção Display", fn_start)
modifier = properties[fn_start:fn_end]
frame_old = "                    egui::Frame::group(ui.style()).show(ui, |ui| {\n"
frame_new = '''                    egui::Frame::new()
                        .fill(tokens::bg_surface(state))
                        .stroke(tokens::stroke_border_dyn(state))
                        .corner_radius(tokens::RADIUS_CONTAINER)
                        .inner_margin(egui::Margin::symmetric(8, 6))
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
'''
if modifier.count(frame_old) != 1:
    raise RuntimeError("properties_panel.rs: modifier frame anchor changed")
modifier = modifier.replace(frame_old, frame_new, 1)
add_old = '''            ui.horizontal(|ui| {
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
'''
add_new = '''            let narrow_actions = ui.available_width() < 220.0;
            if narrow_actions {
                if ui.button(&add_mirror).clicked() {
                    state.project.assets[asset_idx]
                        .modifiers
                        .push(ModifierInstance::mirror(0, 0.001));
                    changed = true;
                }
                if ui.button(&add_symmetry).clicked() {
                    state.project.assets[asset_idx]
                        .modifiers
                        .push(ModifierInstance::symmetry(0, true, 0.001));
                    changed = true;
                }
            } else {
                ui.horizontal(|ui| {
                    if ui.button(&add_mirror).clicked() {
                        state.project.assets[asset_idx]
                            .modifiers
                            .push(ModifierInstance::mirror(0, 0.001));
                        changed = true;
                    }
                    if ui.button(&add_symmetry).clicked() {
                        state.project.assets[asset_idx]
                            .modifiers
                            .push(ModifierInstance::symmetry(0, true, 0.001));
                        changed = true;
                    }
                });
            }
'''
if modifier.count(add_old) != 1:
    raise RuntimeError("properties_panel.rs: modifier add-actions anchor changed")
modifier = modifier.replace(add_old, add_new, 1)
properties = properties[:fn_start] + modifier + properties[fn_end:]
write(properties_path, properties)

# ---------------------------------------------------------------------------
# 8. Locale parity for the remaining viewport-visible Wave 3 strings.
# ---------------------------------------------------------------------------
for locale, drop, modeling, locked in [
    ("assets/locales/en.toml", "Drop to Instantiate", "Modeling", "Object Locked"),
    ("assets/locales/pt-BR.toml", "Soltar para Instanciar", "Modelagem", "Objeto Bloqueado"),
]:
    text = read(locale)
    pivot_anchor = 'pivot = "Pivot Point"\n' if locale.endswith("en.toml") else 'pivot = "Ponto de Pivô"\n'
    if text.count(pivot_anchor) != 1:
        raise RuntimeError(f"{locale}: viewport pivot anchor changed")
    text = text.replace(
        pivot_anchor,
        pivot_anchor + f'drop_to_instantiate = "{drop}"\n',
        1,
    )
    context_anchor = '[context]\n'
    context_pos = text.index(context_anchor) + len(context_anchor)
    text = text[:context_pos] + f'modeling = "{modeling}"\nobject_locked = "{locked}"\n' + text[context_pos:]
    write(locale, text)

print("Wave 3 UX polish staged successfully")
