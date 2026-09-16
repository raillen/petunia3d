//! Interactive Navigation Orientation Gizmo, 3D Cursor overlay, and contextual menu.
//! Follows the Blender.svg Golden Reference specification for viewport ergonomics.

#![forbid(unsafe_code)]

use egui::{Color32, PointerButton, Pos2, Rect, Stroke, Vec2};
use glam::Vec3;
use petunia_core::{
    AppState, DuplicateSelectionCmd, FlipNormalsCmd, ModalKind, Projection, SelectMode,
    SelectionDomain, SubdivideSelectionCmd, ViewPreset, picking::pick_mesh,
};

use crate::icon_registry::PetuniaIcon;
use crate::tokens;
use crate::widgets::PetuniaMenuItem;

/// Screen coordinates of the 6 canonical world axes on the orientation sphere.
#[derive(Debug, Clone, Copy)]
pub struct ProjectedAxis {
    pub axis_index: usize,
    pub sign: f32,
    pub screen_pos: Pos2,
    pub depth: f32,
    pub label: &'static str,
    pub color: Color32,
    pub preset: ViewPreset,
}

const AXIS_COLORS: [Color32; 3] = [
    Color32::from_rgb(235, 75, 75),   // X: Red
    Color32::from_rgb(100, 215, 105), // Y: Green
    Color32::from_rgb(75, 140, 245),  // Z: Blue
];

const AXIS_DATA: [(usize, f32, &str, ViewPreset); 6] = [
    (0, 1.0, "X", ViewPreset::Right),
    (0, -1.0, "-X", ViewPreset::Left),
    (1, 1.0, "Y", ViewPreset::Top),
    (1, -1.0, "-Y", ViewPreset::Bottom),
    (2, 1.0, "Z", ViewPreset::Front),
    (2, -1.0, "-Z", ViewPreset::Back),
];

/// Computes the depth-sorted projection of the 6 navigation axes for the given camera.
pub fn project_nav_axes(
    camera: &petunia_core::Camera,
    center: Pos2,
    radius: f32,
) -> Vec<ProjectedAxis> {
    let right = camera.right();
    let up = camera.up();
    let forward = camera.forward();

    let world_vectors = [Vec3::X, Vec3::Y, Vec3::Z];

    let mut axes = Vec::with_capacity(6);

    for &(idx, sign, label, preset) in &AXIS_DATA {
        let v = world_vectors[idx] * sign;
        let dx = v.dot(right);
        let dy = -v.dot(up); // Inverted because screen Y coordinates go down
        let depth = v.dot(forward);

        let screen_pos = center + Vec2::new(dx, dy) * (radius * 0.72);
        let base_color = AXIS_COLORS[idx];
        let color = if sign > 0.0 {
            base_color
        } else {
            // Muted/darker tone for negative hemisphere axes
            Color32::from_rgb(
                (base_color.r() as f32 * 0.45) as u8,
                (base_color.g() as f32 * 0.45) as u8,
                (base_color.b() as f32 * 0.45) as u8,
            )
        };

        axes.push(ProjectedAxis {
            axis_index: idx,
            sign,
            screen_pos,
            depth,
            label,
            color,
            preset,
        });
    }

    // Sort by depth ascending: background axes drawn first, foreground axes drawn last.
    axes.sort_by(|a, b| {
        a.depth
            .partial_cmp(&b.depth)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    axes
}

/// Draws the interactive Navigation Gizmo in the top-right corner of the viewport.
/// Returns true if an interaction (click or drag) consumed the input.
pub fn draw_nav_gizmo(
    ctx: &egui::Context,
    state: &mut AppState,
    viewport_rect: Rect,
    painter: &egui::Painter,
) -> bool {
    if !state.show_nav_hud || !state.show_overlays {
        return false;
    }
    if viewport_rect.width() < 120.0 || viewport_rect.height() < 120.0 {
        return false;
    }

    let gizmo_radius = 38.0_f32;
    let center = Pos2::new(
        viewport_rect.right() - gizmo_radius - 14.0,
        viewport_rect.top() + gizmo_radius + 14.0,
    );

    let pointer_pos = ctx.pointer_hover_pos();
    let hovered = pointer_pos.is_some_and(|p| center.distance(p) <= gizmo_radius);

    // Draw base disc
    painter.circle(
        center,
        gizmo_radius,
        Color32::from_black_alpha(150),
        Stroke::new(1.0_f32, Color32::from_white_alpha(40)),
    );

    let axes = project_nav_axes(&state.camera, center, gizmo_radius);

    let mut clicked_preset = None;
    let mut hovered_handle = false;

    // First pass: draw back axes lines and negative dots
    for axis in &axes {
        if axis.depth <= 0.0 {
            let handle_radius = 6.0_f32;
            let is_handle_hovered =
                pointer_pos.is_some_and(|p| axis.screen_pos.distance(p) <= handle_radius + 3.0);

            if is_handle_hovered {
                hovered_handle = true;
            }

            // Line from center to axis handle
            painter.line_segment(
                [center, axis.screen_pos],
                Stroke::new(1.5_f32, axis.color.gamma_multiply(0.5)),
            );

            // Back dot
            let fill = if is_handle_hovered {
                Color32::WHITE
            } else {
                axis.color
            };
            painter.circle(
                axis.screen_pos,
                handle_radius,
                fill,
                Stroke::new(1.0_f32, Color32::from_black_alpha(180)),
            );

            if is_handle_hovered && ctx.input(|i| i.pointer.button_clicked(PointerButton::Primary))
            {
                clicked_preset = Some(axis.preset);
            }
        }
    }

    // Second pass: draw front axes lines, spheres, and labels
    for axis in &axes {
        if axis.depth > 0.0 {
            let handle_radius = 8.5_f32;
            let is_handle_hovered =
                pointer_pos.is_some_and(|p| axis.screen_pos.distance(p) <= handle_radius + 3.0);

            if is_handle_hovered {
                hovered_handle = true;
            }

            // Line from center to axis handle
            painter.line_segment([center, axis.screen_pos], Stroke::new(2.0_f32, axis.color));

            // Front sphere
            let fill = if is_handle_hovered {
                Color32::WHITE
            } else {
                axis.color
            };
            painter.circle(
                axis.screen_pos,
                handle_radius,
                fill,
                Stroke::new(1.2_f32, Color32::from_black_alpha(200)),
            );

            // Text label on sphere
            let text_color = if is_handle_hovered {
                Color32::BLACK
            } else {
                Color32::WHITE
            };
            painter.text(
                axis.screen_pos,
                egui::Align2::CENTER_CENTER,
                axis.label,
                egui::FontId::monospace(10.0),
                text_color,
            );

            if is_handle_hovered && ctx.input(|i| i.pointer.button_clicked(PointerButton::Primary))
            {
                clicked_preset = Some(axis.preset);
            }
        }
    }

    // If an axis sphere was clicked, align camera to that preset
    if let Some(preset) = clicked_preset {
        state.camera_frame = None;
        state.camera.set_preset(preset);
        state.mark_dirty();
        return true;
    }

    // Dragging on the base circle (not on an axis handle) smoothly orbits the camera
    if hovered && !hovered_handle && ctx.input(|i| i.pointer.button_down(PointerButton::Primary)) {
        let delta = ctx.input(|i| i.pointer.delta());
        if delta != Vec2::ZERO {
            state.camera_frame = None;
            state.camera.orbit(delta.x * 1.2, delta.y * 1.2);
            state.mark_dirty();
            ctx.set_cursor_icon(egui::CursorIcon::Grab);
            return true;
        }
    }

    // Draw navigation action buttons vertically below the sphere
    let button_radius = 12.0_f32;
    let button_spacing = 28.0_f32;
    let btn_center_x = center.x;
    let start_y = center.y + gizmo_radius + 16.0;

    // 1. Zoom Button
    let zoom_center = Pos2::new(btn_center_x, start_y);
    let zoom_hovered = pointer_pos.is_some_and(|p| zoom_center.distance(p) <= button_radius);
    painter.circle(
        zoom_center,
        button_radius,
        if zoom_hovered {
            Color32::from_rgb(60, 65, 75)
        } else {
            Color32::from_black_alpha(150)
        },
        Stroke::new(1.0_f32, Color32::from_white_alpha(50)),
    );
    // Draw magnifying glass icon
    painter.circle_stroke(
        zoom_center + Vec2::new(-1.5, -1.5),
        4.0,
        Stroke::new(1.5_f32, Color32::WHITE),
    );
    painter.line_segment(
        [
            zoom_center + Vec2::new(1.5, 1.5),
            zoom_center + Vec2::new(4.5, 4.5),
        ],
        Stroke::new(1.8_f32, Color32::WHITE),
    );
    if zoom_hovered && ctx.input(|i| i.pointer.button_down(PointerButton::Primary)) {
        let delta = ctx.input(|i| i.pointer.delta());
        if delta.y != 0.0 {
            state.camera_frame = None;
            state.camera.zoom(-delta.y * 4.0);
            state.mark_dirty();
            return true;
        }
    }

    // 2. Pan Button
    let pan_center = Pos2::new(btn_center_x, start_y + button_spacing);
    let pan_hovered = pointer_pos.is_some_and(|p| pan_center.distance(p) <= button_radius);
    painter.circle(
        pan_center,
        button_radius,
        if pan_hovered {
            Color32::from_rgb(60, 65, 75)
        } else {
            Color32::from_black_alpha(150)
        },
        Stroke::new(1.0_f32, Color32::from_white_alpha(50)),
    );
    // Draw 4-way pan icon
    painter.line_segment(
        [
            pan_center + Vec2::new(-4.0, 0.0),
            pan_center + Vec2::new(4.0, 0.0),
        ],
        Stroke::new(1.5_f32, Color32::WHITE),
    );
    painter.line_segment(
        [
            pan_center + Vec2::new(0.0, -4.0),
            pan_center + Vec2::new(0.0, 4.0),
        ],
        Stroke::new(1.5_f32, Color32::WHITE),
    );
    if pan_hovered && ctx.input(|i| i.pointer.button_down(PointerButton::Primary)) {
        let delta = ctx.input(|i| i.pointer.delta());
        if delta != Vec2::ZERO {
            state.camera_frame = None;
            state.camera.pan(delta.x, delta.y);
            state.mark_dirty();
            return true;
        }
    }

    // 3. Perspective/Ortho Toggle Button
    let ortho_center = Pos2::new(btn_center_x, start_y + button_spacing * 2.0);
    let ortho_hovered = pointer_pos.is_some_and(|p| ortho_center.distance(p) <= button_radius);
    let is_ortho = state.camera.proj == Projection::Ortho;
    painter.circle(
        ortho_center,
        button_radius,
        if is_ortho {
            Color32::from_rgb(50, 90, 150)
        } else if ortho_hovered {
            Color32::from_rgb(60, 65, 75)
        } else {
            Color32::from_black_alpha(150)
        },
        Stroke::new(1.0_f32, Color32::from_white_alpha(50)),
    );
    // Ícone do tema Petunia (arte curada): perspectiva/ortográfica conforme o
    // estado real da câmera — nunca um quadrado genérico.
    let proj_icon = if is_ortho {
        PetuniaIcon::ViewOrthographic
    } else {
        PetuniaIcon::ViewPerspective
    };
    let proj_rect = Rect::from_center_size(ortho_center, Vec2::splat(button_radius * 1.15));
    crate::icon_registry::IconRegistry::paint(ctx, painter, &proj_icon, proj_rect, Color32::WHITE);
    if ortho_hovered && ctx.input(|i| i.pointer.button_clicked(PointerButton::Primary)) {
        state.camera_frame = None;
        let new_proj = if is_ortho {
            Projection::Perspective
        } else {
            Projection::Ortho
        };
        state.camera.set_projection(new_proj);
        state.mark_dirty();
        return true;
    }

    hovered || zoom_hovered || pan_hovered || ortho_hovered
}

/// Renders the Navigation HUD badge (nominal view name + pitch/yaw orientation) in the top-left of the viewport (P3D-005).
pub fn draw_nav_hud(state: &AppState, viewport_rect: Rect, painter: &egui::Painter) {
    if !state.show_nav_hud || !state.show_overlays {
        return;
    }
    if viewport_rect.width() < 160.0 || viewport_rect.height() < 100.0 {
        return;
    }

    let (nominal_name, yaw_deg, pitch_deg) = state.camera.nominal_view();
    let text = format!("{nominal_name}  ·  Pitch {pitch_deg:+.0}°  Yaw {yaw_deg:+.0}°");

    let font_id = egui::FontId::monospace(11.0);
    let galley = painter.layout_no_wrap(text, font_id, Color32::from_rgb(220, 224, 230));

    let pad = Vec2::new(8.0, 4.0);
    let badge_pos = Pos2::new(viewport_rect.left() + 14.0, viewport_rect.top() + 14.0);
    let badge_rect = Rect::from_min_size(badge_pos, galley.size() + pad * 2.0);

    // Subtle translucent dark pill background with border
    painter.rect_filled(
        badge_rect,
        crate::tokens::RADIUS_PILL,
        Color32::from_black_alpha(160),
    );
    painter.rect_stroke(
        badge_rect,
        crate::tokens::RADIUS_PILL,
        Stroke::new(1.0_f32, Color32::from_white_alpha(40)),
        egui::StrokeKind::Inside,
    );

    // Text inside badge padding
    let text_pos = badge_pos + pad;
    painter.galley(text_pos, galley, Color32::from_rgb(220, 224, 230));
}

/// Renders the 3D Cursor overlay at `state.cursor_3d`.
pub fn draw_3d_cursor(state: &AppState, viewport_rect: Rect, painter: &egui::Painter) {
    if !state.show_cursor || !state.show_overlays {
        return;
    }
    let cursor_pos = Vec3::from_array(state.cursor_3d);
    let clip = state.camera.view_proj() * cursor_pos.extend(1.0);
    if !clip.is_finite() || clip.w <= 0.0 || clip.z < 0.0 || clip.z > clip.w {
        return;
    }

    let screen_x = viewport_rect.center().x + clip.x / clip.w * viewport_rect.width() * 0.5;
    let screen_y = viewport_rect.center().y - clip.y / clip.w * viewport_rect.height() * 0.5;
    let center = Pos2::new(screen_x, screen_y);

    if !viewport_rect.contains(center) {
        return;
    }

    let radius = 12.0_f32;
    let stroke_red = Stroke::new(1.5_f32, Color32::from_rgb(235, 60, 60));
    let stroke_white = Stroke::new(1.5_f32, Color32::WHITE);

    // Draw dashed red-and-white circle in 4 quadrants
    let segments = 32;
    for i in 0..segments {
        let a0 = i as f32 / segments as f32 * std::f32::consts::TAU;
        let a1 = (i + 1) as f32 / segments as f32 * std::f32::consts::TAU;
        let p0 = center + Vec2::new(a0.cos(), a0.sin()) * radius;
        let p1 = center + Vec2::new(a1.cos(), a1.sin()) * radius;
        let stroke = if (i / 4) % 2 == 0 {
            stroke_red
        } else {
            stroke_white
        };
        painter.line_segment([p0, p1], stroke);
    }

    // Draw 4 orthogonal crosshair ticks
    let tick_len = 5.0_f32;
    let tick_offsets = [
        (Vec2::new(0.0, -radius - tick_len), Vec2::new(0.0, -radius)),
        (Vec2::new(0.0, radius), Vec2::new(0.0, radius + tick_len)),
        (Vec2::new(-radius - tick_len, 0.0), Vec2::new(-radius, 0.0)),
        (Vec2::new(radius, 0.0), Vec2::new(radius + tick_len, 0.0)),
    ];
    for (start, end) in tick_offsets {
        painter.line_segment([center + start, center + end], stroke_red);
    }

    // Central anchor dot
    painter.circle_filled(center, 1.5, Color32::WHITE);
}

/// Handles Shift + RMB to position the 3D Cursor on geometry or ground plane.
pub fn handle_3d_cursor_placement(
    ctx: &egui::Context,
    state: &mut AppState,
    viewport_rect: Rect,
) -> bool {
    let pointer = ctx
        .pointer_hover_pos()
        .filter(|p| viewport_rect.contains(*p));
    let Some(pos) = pointer else {
        return false;
    };

    let cursor_tool_click = state.active_tool == "cursor_3d"
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
        let nx = (pos.x - viewport_rect.left()) / viewport_rect.width() * 2.0 - 1.0;
        let ny = 1.0 - (pos.y - viewport_rect.top()) / viewport_rect.height() * 2.0;

        // Try raycasting against active mesh faces first
        let hit = state.project.active_mesh().and_then(|mesh| {
            pick_mesh(
                mesh,
                &state.camera,
                glam::Vec2::new(viewport_rect.width(), viewport_rect.height())
                    * ctx.pixels_per_point(),
                glam::Vec2::new(nx, ny),
                SelectMode::Face,
                false,
            )
        });

        if let Some(hit) = hit {
            state.cursor_3d = hit.position.to_array();
            state.mark_dirty();
            return true;
        }

        // Fallback: intersect ray with ground plane Y = 0
        let (origin, dir) = state.camera.ray(nx, ny);
        if dir.y.abs() > 1e-4 {
            let t = -origin.y / dir.y;
            if t > 0.0 {
                let hit_pt = origin + dir * t;
                state.cursor_3d = hit_pt.to_array();
                state.mark_dirty();
                return true;
            }
        }

        // Parallel fallback: set near camera target
        state.cursor_3d = state.camera.target.to_array();
        state.mark_dirty();
        return true;
    }

    false
}

/// Desenha o menu de contexto do RMB sobre o viewport.
///
/// O conteúdo é derivado do **domínio de seleção ativo** (`SelectionDomain`,
/// P3D-015) — nunca de um "Modo de Edição" separado — e todo rótulo vem do i18n
/// (`[ctx]`), sem string hardcoded.
pub fn draw_context_menu(ctx: &egui::Context, state: &mut AppState) {
    let Some(menu_pos) = state.ui.context_menu_pos else {
        return;
    };

    let pos = Pos2::new(menu_pos[0], menu_pos[1]);
    let mut close_menu = false;
    let domain = state.selection_domain();
    let is_locked = state.is_active_locked();

    // Rótulos resolvidos antes do closure: `PetuniaMenuItem::new` toma `&str`, e
    // o corpo do menu precisa de `&mut state` para despachar comandos.
    let title = state.t(domain.key());
    let close_label = state.t("ui.close");
    let locked_label = state.t("context.object_locked");
    let move_label = state.t("actions.move");
    let rotate_label = state.t("tools.rotate");
    let scale_label = state.t("actions.scale");
    let extrude_label = state.t("actions.extrude");
    let extrude_region_label = state.t("ctx.extrude_region");
    let inset_label = state.t("actions.inset");
    let bevel_label = state.t("actions.bevel");
    let push_pull_label = state.t("actions.pushpull");
    let loop_cut_label = state.t("tools.loop_cut");
    let merge_label = state.t("actions.merge_center");
    let subdivide_label = state.t("actions.subdivide");
    let flip_normals_label = state.t("actions.flip_normals");
    let separate_label = state.t("ctx.separate");
    let duplicate_label = state.t("actions.duplicate");
    let delete_label = state.t("actions.delete");
    let cursor_origin_label = state.t("actions.cursor_to_origin");
    let cursor_origin_status = state.t("actions.cursor_to_origin_status");

    egui::Area::new(egui::Id::new("viewport.context_menu"))
        .fixed_pos(pos)
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            egui::Frame::menu(ui.style()).show(ui, |ui| {
                ui.set_min_width(150.0);

                if is_locked {
                    ui.label(
                        egui::RichText::new(&locked_label)
                            .color(tokens::TEXT_MUTED)
                            .italics(),
                    );
                    ui.separator();
                    if PetuniaMenuItem::new(&close_label)
                        .shortcut(Some("Esc"))
                        .show(ui)
                        .clicked()
                    {
                        close_menu = true;
                    }
                    return;
                }

                ui.label(
                    egui::RichText::new(&title)
                        .strong()
                        .color(tokens::TEXT_PRIMARY),
                );
                ui.separator();

                match domain {
                    SelectionDomain::Object => {
                        if PetuniaMenuItem::new(&move_label)
                            .icon(PetuniaIcon::Move)
                            .shortcut(Some("G"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.begin_modal(ModalKind::Move);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&rotate_label)
                            .icon(PetuniaIcon::Rotate)
                            .shortcut(Some("R"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.begin_modal(ModalKind::Rotate);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&scale_label)
                            .icon(PetuniaIcon::Scale)
                            .shortcut(Some("S"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.begin_modal(ModalKind::Scale);
                            close_menu = true;
                        }
                        ui.separator();
                        if PetuniaMenuItem::new(&duplicate_label)
                            .icon(PetuniaIcon::Duplicate)
                            .shortcut(Some("Shift+D"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.dispatch(&DuplicateSelectionCmd);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&cursor_origin_label)
                            .icon(PetuniaIcon::Cursor3D)
                            .show(ui)
                            .clicked()
                        {
                            state.session.cursor_3d = [0.0, 0.0, 0.0];
                            state.set_status(cursor_origin_status.clone());
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&delete_label)
                            .icon(PetuniaIcon::Trash)
                            .shortcut(Some("Delete"))
                            .show(ui)
                            .clicked()
                        {
                            let _ =
                                state.dispatch(&petunia_core::DeleteAssetCmd { asset_index: None });
                            close_menu = true;
                        }
                    }
                    SelectionDomain::Vertex => {
                        if PetuniaMenuItem::new(&move_label)
                            .icon(PetuniaIcon::Move)
                            .shortcut(Some("G"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.begin_modal(ModalKind::Move);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&extrude_label)
                            .icon(PetuniaIcon::Extrude)
                            .shortcut(Some("E"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.begin_modal(ModalKind::Extrude);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&merge_label)
                            .shortcut(Some("M"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.dispatch(&petunia_core::MergeCenterCmd);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&subdivide_label)
                            .icon(PetuniaIcon::Subdivide)
                            .shortcut(Some("W"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.dispatch(&SubdivideSelectionCmd);
                            close_menu = true;
                        }
                        ui.separator();
                        if PetuniaMenuItem::new(&delete_label)
                            .icon(PetuniaIcon::Trash)
                            .shortcut(Some("Delete"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.dispatch(&petunia_core::DeleteSelectionCmd);
                            close_menu = true;
                        }
                    }
                    SelectionDomain::Edge => {
                        if PetuniaMenuItem::new(&move_label)
                            .icon(PetuniaIcon::Move)
                            .shortcut(Some("G"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.begin_modal(ModalKind::Move);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&rotate_label)
                            .icon(PetuniaIcon::Rotate)
                            .shortcut(Some("R"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.begin_modal(ModalKind::Rotate);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&scale_label)
                            .icon(PetuniaIcon::Scale)
                            .shortcut(Some("S"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.begin_modal(ModalKind::Scale);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&bevel_label)
                            .icon(PetuniaIcon::Bevel)
                            .shortcut(Some("Ctrl+B"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.begin_modal(ModalKind::Bevel);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&loop_cut_label)
                            .icon(PetuniaIcon::LoopCut)
                            .shortcut(Some("Ctrl+R"))
                            .show(ui)
                            .clicked()
                        {
                            state.active_tool = "loop_cut".to_string();
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&merge_label)
                            .shortcut(Some("M"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.dispatch(&petunia_core::MergeCenterCmd);
                            close_menu = true;
                        }
                        ui.separator();
                        if PetuniaMenuItem::new(&delete_label)
                            .icon(PetuniaIcon::Trash)
                            .shortcut(Some("Delete"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.dispatch(&petunia_core::DeleteSelectionCmd);
                            close_menu = true;
                        }
                    }
                    SelectionDomain::Face => {
                        if PetuniaMenuItem::new(&extrude_region_label)
                            .icon(PetuniaIcon::Extrude)
                            .shortcut(Some("E"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.begin_modal(ModalKind::Extrude);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&inset_label)
                            .icon(PetuniaIcon::Inset)
                            .shortcut(Some("I"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.begin_modal(ModalKind::Inset);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&bevel_label)
                            .icon(PetuniaIcon::Bevel)
                            .shortcut(Some("Ctrl+B"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.begin_modal(ModalKind::Bevel);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&push_pull_label)
                            .icon(PetuniaIcon::PushPull)
                            .shortcut(Some("P"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.begin_modal(ModalKind::PushPull);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&subdivide_label)
                            .icon(PetuniaIcon::Subdivide)
                            .shortcut(Some("W"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.dispatch(&SubdivideSelectionCmd);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&flip_normals_label).show(ui).clicked() {
                            let _ = state.dispatch(&FlipNormalsCmd);
                            close_menu = true;
                        }
                        if PetuniaMenuItem::new(&separate_label)
                            .shortcut(Some("Shift+P"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.dispatch(&petunia_core::SeparateSelectionCmd);
                            close_menu = true;
                        }
                        ui.separator();
                        if PetuniaMenuItem::new(&delete_label)
                            .icon(PetuniaIcon::Trash)
                            .shortcut(Some("Delete"))
                            .show(ui)
                            .clicked()
                        {
                            let _ = state.dispatch(&petunia_core::DeleteSelectionCmd);
                            close_menu = true;
                        }
                    }
                }

                ui.separator();
                if PetuniaMenuItem::new(&close_label)
                    .shortcut(Some("Esc"))
                    .show(ui)
                    .clicked()
                {
                    close_menu = true;
                }
            });
        });

    if close_menu
        || ctx.input(|i| {
            i.key_pressed(egui::Key::Escape) || i.pointer.button_pressed(PointerButton::Primary)
        })
    {
        state.ui.context_menu_pos = None;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use petunia_core::Camera;

    #[test]
    fn test_project_nav_axes_produces_six_depth_sorted_axes() {
        let camera = Camera::default();
        let center = Pos2::new(100.0, 100.0);
        let radius = 40.0;

        let axes = project_nav_axes(&camera, center, radius);
        assert_eq!(axes.len(), 6);

        // Verify sorted by depth
        for w in axes.windows(2) {
            assert!(w[0].depth <= w[1].depth);
        }

        // Verify all 6 axes are accounted for
        let labels: Vec<&str> = axes.iter().map(|a| a.label).collect();
        assert!(labels.contains(&"X"));
        assert!(labels.contains(&"-X"));
        assert!(labels.contains(&"Y"));
        assert!(labels.contains(&"-Y"));
        assert!(labels.contains(&"Z"));
        assert!(labels.contains(&"-Z"));
    }

    #[test]
    fn test_nav_axes_preset_mapping() {
        let camera = Camera::default();
        let center = Pos2::new(100.0, 100.0);
        let axes = project_nav_axes(&camera, center, 40.0);

        let x_axis = axes.iter().find(|a| a.label == "X").unwrap();
        assert_eq!(x_axis.preset, ViewPreset::Right);

        let y_axis = axes.iter().find(|a| a.label == "Y").unwrap();
        assert_eq!(y_axis.preset, ViewPreset::Top);

        let z_axis = axes.iter().find(|a| a.label == "Z").unwrap();
        assert_eq!(z_axis.preset, ViewPreset::Front);
    }

    #[test]
    fn test_3d_cursor_placement_fallback_ground_plane() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("pt-BR");
        let rect = Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(800.0, 600.0));

        // Initial cursor at origin
        assert_eq!(state.cursor_3d, [0.0, 0.0, 0.0]);

        // Trigger Shift + RMB input
        let mut raw_input = egui::RawInput {
            screen_rect: Some(rect),
            ..Default::default()
        };
        let shift_mods = egui::Modifiers {
            shift: true,
            ..Default::default()
        };
        raw_input
            .events
            .push(egui::Event::PointerMoved(Pos2::new(400.0, 300.0)));
        raw_input
            .events
            .push(egui::Event::ModifiersChanged(shift_mods));
        raw_input.events.push(egui::Event::PointerButton {
            pos: Pos2::new(400.0, 300.0),
            button: PointerButton::Secondary,
            pressed: true,
            modifiers: shift_mods,
        });

        ctx.run_ui(raw_input, |_ui| {
            let handled = handle_3d_cursor_placement(&ctx, &mut state, rect);
            assert!(handled);
        })
        .textures_delta
        .clear();

        // The cursor should have been placed
        assert!(state.cursor_3d[0].is_finite());
        assert!(state.cursor_3d[1].is_finite());
        assert!(state.cursor_3d[2].is_finite());
    }

    #[test]
    fn test_context_menu_pos_lifecycle() {
        let ctx = egui::Context::default();
        let mut state = AppState::new("pt-BR");
        state.ui.context_menu_pos = Some([200.0, 200.0]);

        // Simulate Escape key
        let mut raw_input = egui::RawInput::default();
        raw_input.events.push(egui::Event::Key {
            key: egui::Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        });

        ctx.run_ui(raw_input, |_ui| {
            draw_context_menu(&ctx, &mut state);
        })
        .textures_delta
        .clear();

        // Menu should be closed by Escape
        assert_eq!(state.ui.context_menu_pos, None);
    }
}
