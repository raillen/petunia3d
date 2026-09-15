//! Barra lateral vertical de ferramentas do Petunia3D (`Toolbar`).
//!
//! A toolbar é deliberadamente compacta e agrupada: seleção e transformação
//! são famílias (split buttons) em vez de uma lista de botões concorrentes.
//! Operações que pertencem ao Modifier Stack (Mirror/Symmetry) não aparecem
//! aqui. A configuração do usuário continua controlando ordem/visibilidade
//! dos grupos e das ferramentas de modelagem destrutivas.

use egui::{ScrollArea, Ui, vec2};
use petunia_core::{AppState, ModalKind, Workspace};
use petunia_module_model::ToolRegistry;

use crate::icon_registry::PetuniaIcon;
use crate::tokens;
use crate::widgets::PetuniaToolbarButton;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarSection {
    Primary,
    Inspect,
    Mesh,
}

pub struct ToolbarEntry {
    pub id: &'static str,
    pub icon: PetuniaIcon,
    pub key: &'static str,
    pub label_key: &'static str,
    pub section: ToolbarSection,
    pub edit_only: bool,
}

/// Ordem canônica da paleta Model.
///
/// `select` e `transform` representam grupos. Os filhos ficam no popover do
/// grupo, portanto não consomem quatro/cinco linhas da toolbar.
pub fn canonical_toolbar_entries() -> Vec<ToolbarEntry> {
    vec![
        ToolbarEntry {
            id: "select",
            icon: PetuniaIcon::SelectBox,
            key: "Q/B",
            label_key: "tools.select",
            section: ToolbarSection::Primary,
            edit_only: false,
        },
        ToolbarEntry {
            id: "cursor_3d",
            icon: PetuniaIcon::Cursor3D,
            key: "Shift+RMB",
            label_key: "tools.cursor_3d",
            section: ToolbarSection::Primary,
            edit_only: false,
        },
        ToolbarEntry {
            id: "transform",
            icon: PetuniaIcon::Transform,
            key: "T · G/R/S",
            label_key: "tools.transform",
            section: ToolbarSection::Primary,
            edit_only: false,
        },
        ToolbarEntry {
            id: "measure",
            icon: PetuniaIcon::Measure,
            key: "M",
            label_key: "tools.measure",
            section: ToolbarSection::Inspect,
            edit_only: false,
        },
        ToolbarEntry {
            id: "annotate",
            icon: PetuniaIcon::Annotate,
            key: "D",
            label_key: "tools.annotate",
            section: ToolbarSection::Inspect,
            edit_only: false,
        },
        ToolbarEntry {
            id: "extrude",
            icon: PetuniaIcon::Extrude,
            key: "E",
            label_key: "tools.extrude",
            section: ToolbarSection::Mesh,
            edit_only: true,
        },
        ToolbarEntry {
            id: "inset",
            icon: PetuniaIcon::Inset,
            key: "I",
            label_key: "tools.inset",
            section: ToolbarSection::Mesh,
            edit_only: true,
        },
        ToolbarEntry {
            id: "bevel",
            icon: PetuniaIcon::Bevel,
            key: "Ctrl+B",
            label_key: "tools.bevel",
            section: ToolbarSection::Mesh,
            edit_only: true,
        },
        ToolbarEntry {
            id: "loop_cut",
            icon: PetuniaIcon::LoopCut,
            key: "Ctrl+R",
            label_key: "tools.loop_cut",
            section: ToolbarSection::Mesh,
            edit_only: true,
        },
        ToolbarEntry {
            id: "knife",
            icon: PetuniaIcon::Knife,
            key: "K",
            label_key: "tools.knife",
            section: ToolbarSection::Mesh,
            edit_only: true,
        },
        ToolbarEntry {
            id: "pushpull",
            icon: PetuniaIcon::PushPull,
            key: "P",
            label_key: "tools.pushpull",
            section: ToolbarSection::Mesh,
            edit_only: true,
        },
        ToolbarEntry {
            id: "slice",
            icon: PetuniaIcon::Slice,
            key: "",
            label_key: "tools.slice",
            section: ToolbarSection::Mesh,
            edit_only: true,
        },
        ToolbarEntry {
            id: "subdivide",
            icon: PetuniaIcon::Subdivide,
            key: "W",
            label_key: "tools.subdivide",
            section: ToolbarSection::Mesh,
            edit_only: true,
        },
        ToolbarEntry {
            id: "draw_profile",
            icon: PetuniaIcon::DrawProfile,
            key: "Shift+P",
            label_key: "tools.draw_profile",
            section: ToolbarSection::Mesh,
            edit_only: true,
        },
        ToolbarEntry {
            id: "merge",
            icon: PetuniaIcon::Custom("merge"),
            key: "M",
            label_key: "tools.merge",
            section: ToolbarSection::Mesh,
            edit_only: true,
        },
    ]
}

fn clone_entry(entry: &ToolbarEntry) -> ToolbarEntry {
    ToolbarEntry {
        id: entry.id,
        icon: entry.icon,
        key: entry.key,
        label_key: entry.label_key,
        section: entry.section,
        edit_only: entry.edit_only,
    }
}

fn display_entries(state: &AppState) -> Vec<ToolbarEntry> {
    let canonical = canonical_toolbar_entries();
    let mut ordered = Vec::with_capacity(canonical.len());
    if state.ui.toolbar_order.is_empty() {
        ordered = canonical;
    } else {
        for id in &state.ui.toolbar_order {
            if let Some(entry) = canonical.iter().find(|entry| entry.id == id.as_str()) {
                ordered.push(clone_entry(entry));
            }
        }
        for entry in canonical {
            if !ordered.iter().any(|candidate| candidate.id == entry.id) {
                ordered.push(entry);
            }
        }
    }
    let in_edit = state.mode == petunia_core::EditMode::Edit;
    ordered
        .into_iter()
        .filter(|entry| {
            !state
                .ui
                .toolbar_hidden
                .iter()
                .any(|hidden| hidden == entry.id)
                && (!entry.edit_only || in_edit)
        })
        .collect()
}

fn apply_toolbar_entry(state: &mut AppState, tools: &ToolRegistry, entry: &ToolbarEntry) {
    state.active_tool = entry.id.into();
    state.pending_modal = None;
    if let Some(tool) = tools.get(entry.id) {
        tool.on_activate(state);
    }
    state.mark_dirty();
}

fn entry_is_active(state: &AppState, entry: &ToolbarEntry) -> bool {
    match entry.id {
        "select" => matches!(state.active_tool.as_str(), "select" | "select_box"),
        "transform" => matches!(
            state.active_tool.as_str(),
            "transform" | "move" | "rotate" | "scale"
        ),
        _ => state.active_tool == entry.id,
    }
}

const TOOLBAR_FOOTER: f32 = 44.0;

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
                    .max_height((ui.available_height() - TOOLBAR_FOOTER).max(80.0))
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing = vec2(0.0, 3.0);
                        match state.workspace {
                            Workspace::Model => draw_model_tools(ui, state, tools, compact),
                            Workspace::Paint => draw_paint_tools(ui, state, compact),
                            Workspace::Uv => draw_uv_tools(ui, state, compact),
                            Workspace::Animate => draw_animate_tools(ui, state, compact),
                        }
                    });
                ui.separator();
                ui.vertical_centered(|ui| draw_toolbar_config(ui, state));
            });
        });
    crate::regions::record(
        ui.ctx(),
        crate::regions::RegionSlot::LeftTools,
        toolbar_resp.response.rect,
    );
}

fn draw_model_tools(ui: &mut egui::Ui, state: &mut AppState, tools: &ToolRegistry, compact: bool) {
    if state.mode != petunia_core::EditMode::Edit
        && canonical_toolbar_entries()
            .iter()
            .any(|entry| entry.edit_only && entry.id == state.active_tool)
    {
        state.active_tool = "select".into();
        state.mark_dirty();
    }

    let entries = display_entries(state);
    let two_cols = state.ui.toolbar_columns.clamp(1, 2) == 2 && ui.available_width() >= 100.0;
    let compact = two_cols || compact;
    let mut seen_section = false;

    for section in [
        ToolbarSection::Primary,
        ToolbarSection::Inspect,
        ToolbarSection::Mesh,
    ] {
        let group: Vec<&ToolbarEntry> = entries
            .iter()
            .filter(|entry| entry.section == section)
            .collect();
        if group.is_empty() {
            continue;
        }
        if seen_section {
            ui.add_space(3.0);
            ui.separator();
            ui.add_space(3.0);
        }
        seen_section = true;

        if two_cols {
            for pair in group.chunks(2) {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = vec2(3.0, 3.0);
                    for entry in pair {
                        draw_model_entry(ui, state, tools, entry, true);
                    }
                });
            }
        } else {
            for entry in group {
                draw_model_entry(ui, state, tools, entry, compact);
            }
        }
    }
}

fn draw_model_entry(
    ui: &mut egui::Ui,
    state: &mut AppState,
    tools: &ToolRegistry,
    entry: &ToolbarEntry,
    compact: bool,
) {
    match entry.id {
        "select" => draw_select_group(ui, state, compact),
        "transform" => draw_transform_group(ui, state, compact),
        _ => draw_entry_button(ui, state, tools, entry, compact),
    }
}

fn draw_entry_button(
    ui: &mut egui::Ui,
    state: &mut AppState,
    tools: &ToolRegistry,
    entry: &ToolbarEntry,
    compact: bool,
) {
    let label = state.t(entry.label_key);
    let hint = if entry.key.is_empty() {
        label.clone()
    } else {
        format!("{label} · [{}]", entry.key)
    };
    if PetuniaToolbarButton::new(entry.icon, &label)
        .selected(entry_is_active(state, entry))
        .compact(compact)
        .tooltip(&hint)
        .show(ui)
        .clicked()
    {
        apply_toolbar_entry(state, tools, entry);
    }
}

fn draw_select_group(ui: &mut egui::Ui, state: &mut AppState, compact: bool) {
    let box_mode = state.active_tool == "select_box";
    let label_key = if box_mode {
        "tools.select_box"
    } else {
        "tools.select"
    };
    let label = state.t(label_key);
    let mut popup_response = None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(2.0, 0.0);
        if PetuniaToolbarButton::new(PetuniaIcon::SelectBox, &label)
            .selected(matches!(
                state.active_tool.as_str(),
                "select" | "select_box"
            ))
            .compact(compact)
            .tooltip(&format!("{label} · [Q/B]"))
            .show(ui)
            .clicked()
        {
            state.active_tool = if box_mode { "select_box" } else { "select" }.into();
            state.pending_modal = None;
            state.mark_dirty();
        }
        popup_response = Some(
            ui.small_button("▾")
                .on_hover_text(state.t("toolbar.configure")),
        );
    });
    if let Some(response) = popup_response {
        egui::Popup::menu(&response).show(|ui| {
            ui.set_max_width(190.0);
            if ui
                .selectable_label(state.active_tool == "select", state.t("tools.select"))
                .clicked()
            {
                state.active_tool = "select".into();
                state.ui.box_select_start = None;
                state.mark_dirty();
                ui.close();
            }
            if ui
                .selectable_label(
                    state.active_tool == "select_box",
                    state.t("tools.select_box"),
                )
                .clicked()
            {
                state.active_tool = "select_box".into();
                state.ui.box_select_start = None;
                state.mark_dirty();
                ui.close();
            }
            ui.add_enabled(false, egui::Button::new(state.t("tools.select_lasso")))
                .on_hover_text(state.t("toolbar.planned"));
        });
    }
}

fn transform_child(state: &AppState) -> (&'static str, PetuniaIcon, &'static str, &'static str) {
    match state.active_tool.as_str() {
        "move" => ("move", PetuniaIcon::Move, "tools.move", "G"),
        "rotate" => ("rotate", PetuniaIcon::Rotate, "tools.rotate", "R"),
        "scale" => ("scale", PetuniaIcon::Scale, "tools.scale", "S"),
        _ => ("transform", PetuniaIcon::Transform, "tools.transform", "T"),
    }
}

fn activate_transform_child(state: &mut AppState, id: &str) {
    state.active_tool = id.into();
    state.pending_modal = None;
    match id {
        "move" => state.gizmo_mode = ModalKind::Move,
        "rotate" => state.gizmo_mode = ModalKind::Rotate,
        "scale" => state.gizmo_mode = ModalKind::Scale,
        _ => {}
    }
    state.mark_dirty();
}

fn draw_transform_group(ui: &mut egui::Ui, state: &mut AppState, compact: bool) {
    let (current_id, icon, label_key, key) = transform_child(state);
    let label = state.t(label_key);
    let active = matches!(
        state.active_tool.as_str(),
        "transform" | "move" | "rotate" | "scale"
    );
    let mut popup_response = None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(2.0, 0.0);
        if PetuniaToolbarButton::new(icon, &label)
            .selected(active)
            .compact(compact)
            .tooltip(&format!("{label} · [{key}]"))
            .show(ui)
            .clicked()
        {
            activate_transform_child(state, current_id);
        }
        popup_response = Some(
            ui.small_button("▾")
                .on_hover_text(state.t("tools.transform")),
        );
    });
    if let Some(response) = popup_response {
        egui::Popup::menu(&response).show(|ui| {
            ui.set_max_width(190.0);
            for (id, label_key, shortcut) in [
                ("transform", "tools.transform", "T"),
                ("move", "tools.move", "G"),
                ("rotate", "tools.rotate", "R"),
                ("scale", "tools.scale", "S"),
            ] {
                let text = format!("{}    {}", state.t(label_key), shortcut);
                if ui.selectable_label(state.active_tool == id, text).clicked() {
                    activate_transform_child(state, id);
                    ui.close();
                }
            }
        });
    }
}

fn draw_toolbar_config(ui: &mut egui::Ui, state: &mut AppState) {
    let gear_tip = state.t("toolbar.configure");
    let response =
        crate::widgets::PetuniaIconButton::new(PetuniaIcon::Settings, &gear_tip, 24.0).show(ui);
    egui::Popup::menu(&response).show(|ui| {
        // Não imponha 250px a um menu que normalmente precisa de ~180px.
        ui.set_min_width(176.0);
        ui.set_max_width(230.0);
        ui.label(
            egui::RichText::new(state.t("toolbar.configure"))
                .strong()
                .size(12.0)
                .color(tokens::TEXT_PRIMARY),
        );
        ui.separator();

        let mut order: Vec<String> = if state.ui.toolbar_order.is_empty() {
            canonical_toolbar_entries()
                .iter()
                .map(|entry| entry.id.to_string())
                .collect()
        } else {
            state.ui.toolbar_order.clone()
        };
        // Purga ids legados que agora pertencem a grupos/modifiers.
        order.retain(|id| {
            canonical_toolbar_entries()
                .iter()
                .any(|entry| entry.id == id)
        });
        for entry in canonical_toolbar_entries() {
            if !order.iter().any(|id| id == entry.id) {
                order.push(entry.id.to_string());
            }
        }
        state.ui.toolbar_hidden.retain(|id| {
            canonical_toolbar_entries()
                .iter()
                .any(|entry| entry.id == id)
        });

        let mut dirty = false;
        let mut move_up = None;
        let mut move_down = None;
        egui::ScrollArea::vertical()
            .id_salt("toolbar_config_scroll")
            .max_height(320.0)
            .show(ui, |ui| {
                for (idx, id) in order.iter().enumerate() {
                    let label = canonical_toolbar_entries()
                        .iter()
                        .find(|entry| entry.id == id.as_str())
                        .map(|entry| state.t(entry.label_key))
                        .unwrap_or_else(|| id.clone());
                    ui.horizontal(|ui| {
                        let mut visible =
                            !state.ui.toolbar_hidden.iter().any(|hidden| hidden == id);
                        if ui.checkbox(&mut visible, "").changed() {
                            if visible {
                                state.ui.toolbar_hidden.retain(|hidden| hidden != id);
                            } else if !state.ui.toolbar_hidden.iter().any(|hidden| hidden == id) {
                                state.ui.toolbar_hidden.push(id.clone());
                            }
                            dirty = true;
                        }
                        ui.label(
                            egui::RichText::new(&label)
                                .size(11.5)
                                .color(tokens::TEXT_PRIMARY),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if crate::widgets::chevron_toggle_dir(
                                ui,
                                &state.t("toolbar.move_down"),
                                crate::widgets::ChevronDir::Down,
                            )
                            .clicked()
                            {
                                move_down = Some(idx);
                            }
                            if crate::widgets::chevron_toggle_dir(
                                ui,
                                &state.t("toolbar.move_up"),
                                crate::widgets::ChevronDir::Up,
                            )
                            .clicked()
                            {
                                move_up = Some(idx);
                            }
                        });
                    });
                }
            });

        if let Some(idx) = move_up
            && idx > 0
        {
            order.swap(idx, idx - 1);
            state.ui.toolbar_order = order.clone();
            dirty = true;
        }
        if let Some(idx) = move_down
            && idx + 1 < order.len()
        {
            order.swap(idx, idx + 1);
            state.ui.toolbar_order = order.clone();
            dirty = true;
        }
        if state.ui.toolbar_order != order {
            state.ui.toolbar_order = order;
            dirty = true;
        }

        ui.separator();
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(state.t("toolbar.columns"))
                    .size(11.5)
                    .color(tokens::TEXT_SECONDARY),
            );
            for (cols, key) in [(1u8, "toolbar.one_column"), (2u8, "toolbar.two_columns")] {
                if ui
                    .selectable_label(state.ui.toolbar_columns.clamp(1, 2) == cols, state.t(key))
                    .clicked()
                {
                    state.ui.toolbar_columns = cols;
                    dirty = true;
                }
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(state.t("toolbar.reset")).clicked() {
                    state.ui.toolbar_order.clear();
                    state.ui.toolbar_hidden.clear();
                    state.ui.toolbar_columns = 1;
                    dirty = true;
                }
            });
        });
        if dirty {
            state.mark_dirty();
        }
    });
}

fn draw_animate_tools(ui: &mut egui::Ui, state: &mut AppState, compact: bool) {
    draw_select_group(ui, state, compact);
    draw_transform_group(ui, state, compact);
}

fn draw_paint_tools(ui: &mut egui::Ui, state: &mut AppState, compact: bool) {
    let active = state.active_tool == "paint";
    let label = state.t("tools.paint");
    if PetuniaToolbarButton::new(PetuniaIcon::Custom("paint"), &label)
        .selected(active)
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
    draw_select_group(ui, state, compact);
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
            ctx.run_ui(egui::RawInput::default(), |ui| draw(ui, &mut state, &tools))
                .textures_delta
                .clear();
        }
    }

    #[test]
    fn mesh_tools_are_hidden_in_object_mode() {
        let mut state = AppState::new("en");
        state.mode = EditMode::Object;
        state.active_tool = "extrude".into();
        let ctx = egui::Context::default();
        let tools = ToolRegistry::default();
        ctx.run_ui(egui::RawInput::default(), |ui| draw(ui, &mut state, &tools))
            .textures_delta
            .clear();
        assert_eq!(state.active_tool, "select");
    }

    #[test]
    fn modifier_operations_are_not_toolbar_tools() {
        let ids: Vec<&str> = canonical_toolbar_entries()
            .iter()
            .map(|entry| entry.id)
            .collect();
        assert!(!ids.contains(&"mirror"));
        assert!(!ids.contains(&"symmetrize"));
        assert!(ids.contains(&"transform"));
        assert!(!ids.contains(&"move"));
        assert!(!ids.contains(&"rotate"));
        assert!(!ids.contains(&"scale"));
    }

    #[test]
    fn toolbar_config_filters_legacy_entries() {
        let mut state = AppState::new("en");
        state.mode = EditMode::Edit;
        state.ui.toolbar_hidden = vec!["knife".to_string(), "mirror".to_string()];
        state.ui.toolbar_order = vec![
            "symmetrize".to_string(),
            "select".to_string(),
            "merge".to_string(),
        ];
        let shown: Vec<&str> = display_entries(&state)
            .iter()
            .map(|entry| entry.id)
            .collect();
        assert_eq!(&shown[..2], &["select", "merge"]);
        assert!(!shown.contains(&"knife"));
    }
}
