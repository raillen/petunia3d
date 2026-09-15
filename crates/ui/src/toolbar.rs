//! Barra lateral vertical de ferramentas do Petunia3D (`Toolbar`).
//! Renderiza as ferramentas com os ícones extraídos da Golden Reference (`assets/ui/icons/toolbar/`)
//! com 40px de largura e suporte a destaque ativo e atalhos de teclado.

use egui::{ScrollArea, Ui, vec2};
use petunia_core::{AppState, ModalKind, Workspace};
use petunia_module_model::ToolRegistry;

use crate::icon_registry::PetuniaIcon;
use crate::tokens;
use crate::widgets::PetuniaToolbarButton;

/// Seção da paleta Model na toolbar esquerda.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarSection {
    Primary,
    Inspect,
    Mesh,
}

/// Entrada configurável da toolbar (dono único da ordem/atalhos do Model).
pub struct ToolbarEntry {
    /// Id da ferramenta (`active_tool`).
    pub id: &'static str,
    pub icon: PetuniaIcon,
    /// Chave de atalho exibida na dica (vazia = sem atalho).
    pub key: &'static str,
    /// Chave de tradução do rótulo (`tools.*`).
    pub label_key: &'static str,
    pub section: ToolbarSection,
    /// Só aparece no modo de edição de malha.
    pub edit_only: bool,
    /// Gizmo associado (trio Move/Rotate/Scale).
    pub gizmo: Option<ModalKind>,
}

/// Ordem canônica da paleta Model (usada quando o usuário nunca configurou).
pub fn canonical_toolbar_entries() -> Vec<ToolbarEntry> {
    vec![
        ToolbarEntry {
            id: "select",
            icon: PetuniaIcon::SelectBox,
            key: "B",
            label_key: "tools.select_box",
            section: ToolbarSection::Primary,
            edit_only: false,
            gizmo: None,
        },
        ToolbarEntry {
            id: "cursor_3d",
            icon: PetuniaIcon::Cursor3D,
            key: "Shift+RMB",
            label_key: "tools.cursor_3d",
            section: ToolbarSection::Primary,
            edit_only: false,
            gizmo: None,
        },
        ToolbarEntry {
            id: "move",
            icon: PetuniaIcon::Move,
            key: "G",
            label_key: "tools.move",
            section: ToolbarSection::Primary,
            edit_only: false,
            gizmo: Some(ModalKind::Move),
        },
        ToolbarEntry {
            id: "rotate",
            icon: PetuniaIcon::Rotate,
            key: "R",
            label_key: "tools.rotate",
            section: ToolbarSection::Primary,
            edit_only: false,
            gizmo: Some(ModalKind::Rotate),
        },
        ToolbarEntry {
            id: "scale",
            icon: PetuniaIcon::Scale,
            key: "S",
            label_key: "tools.scale",
            section: ToolbarSection::Primary,
            edit_only: false,
            gizmo: Some(ModalKind::Scale),
        },
        ToolbarEntry {
            id: "transform",
            icon: PetuniaIcon::Transform,
            key: "T",
            label_key: "tools.transform",
            section: ToolbarSection::Primary,
            edit_only: false,
            gizmo: None,
        },
        ToolbarEntry {
            id: "measure",
            icon: PetuniaIcon::Measure,
            key: "M",
            label_key: "tools.measure",
            section: ToolbarSection::Inspect,
            edit_only: false,
            gizmo: None,
        },
        ToolbarEntry {
            id: "annotate",
            icon: PetuniaIcon::Annotate,
            key: "D",
            label_key: "tools.annotate",
            section: ToolbarSection::Inspect,
            edit_only: false,
            gizmo: None,
        },
        ToolbarEntry {
            id: "extrude",
            icon: PetuniaIcon::Extrude,
            key: "E",
            label_key: "tools.extrude",
            section: ToolbarSection::Mesh,
            edit_only: true,
            gizmo: None,
        },
        ToolbarEntry {
            id: "inset",
            icon: PetuniaIcon::Inset,
            key: "I",
            label_key: "tools.inset",
            section: ToolbarSection::Mesh,
            edit_only: true,
            gizmo: None,
        },
        ToolbarEntry {
            id: "bevel",
            icon: PetuniaIcon::Bevel,
            key: "Ctrl+B",
            label_key: "tools.bevel",
            section: ToolbarSection::Mesh,
            edit_only: true,
            gizmo: None,
        },
        ToolbarEntry {
            id: "loop_cut",
            icon: PetuniaIcon::LoopCut,
            key: "Ctrl+R",
            label_key: "tools.loop_cut",
            section: ToolbarSection::Mesh,
            edit_only: true,
            gizmo: None,
        },
        ToolbarEntry {
            id: "knife",
            icon: PetuniaIcon::Knife,
            key: "K",
            label_key: "tools.knife",
            section: ToolbarSection::Mesh,
            edit_only: true,
            gizmo: None,
        },
        ToolbarEntry {
            id: "pushpull",
            icon: PetuniaIcon::PushPull,
            key: "",
            label_key: "tools.pushpull",
            section: ToolbarSection::Mesh,
            edit_only: true,
            gizmo: None,
        },
        ToolbarEntry {
            id: "slice",
            icon: PetuniaIcon::Slice,
            key: "",
            label_key: "tools.slice",
            section: ToolbarSection::Mesh,
            edit_only: true,
            gizmo: None,
        },
        ToolbarEntry {
            id: "subdivide",
            icon: PetuniaIcon::Subdivide,
            key: "",
            label_key: "tools.subdivide",
            section: ToolbarSection::Mesh,
            edit_only: true,
            gizmo: None,
        },
        ToolbarEntry {
            id: "draw_profile",
            icon: PetuniaIcon::DrawProfile,
            key: "",
            label_key: "tools.draw_profile",
            section: ToolbarSection::Mesh,
            edit_only: true,
            gizmo: None,
        },
        ToolbarEntry {
            id: "mirror",
            icon: PetuniaIcon::Custom("mirror"),
            key: "Ctrl+M",
            label_key: "tools.mirror",
            section: ToolbarSection::Mesh,
            edit_only: true,
            gizmo: None,
        },
        ToolbarEntry {
            id: "merge",
            icon: PetuniaIcon::Custom("merge"),
            key: "M",
            label_key: "tools.merge",
            section: ToolbarSection::Mesh,
            edit_only: true,
            gizmo: None,
        },
        ToolbarEntry {
            id: "symmetrize",
            icon: PetuniaIcon::Custom("symmetrize"),
            key: "Alt+M",
            label_key: "tools.symmetrize",
            section: ToolbarSection::Mesh,
            edit_only: true,
            gizmo: None,
        },
    ]
}

/// Ordem de exibição = configuração do usuário (ou canônica), menos ocultos,
/// mais ferramentas novas ao final (compatibilidade futura).
fn display_entries(state: &AppState) -> Vec<ToolbarEntry> {
    let canonical = canonical_toolbar_entries();
    let mut ordered: Vec<ToolbarEntry> = Vec::with_capacity(canonical.len());
    if state.ui.toolbar_order.is_empty() {
        ordered = canonical;
    } else {
        for id in &state.ui.toolbar_order {
            if let Some(pos) = canonical.iter().position(|e| e.id == id.as_str()) {
                ordered.push(ToolbarEntry {
                    id: canonical[pos].id,
                    icon: canonical[pos].icon,
                    key: canonical[pos].key,
                    label_key: canonical[pos].label_key,
                    section: canonical[pos].section,
                    edit_only: canonical[pos].edit_only,
                    gizmo: canonical[pos].gizmo,
                });
            }
        }
        for entry in canonical {
            if !ordered.iter().any(|e| e.id == entry.id) {
                ordered.push(entry);
            }
        }
    }
    let in_edit = state.mode == petunia_core::EditMode::Edit;
    ordered
        .into_iter()
        .filter(|e| !state.ui.toolbar_hidden.iter().any(|h| h == e.id) && (!e.edit_only || in_edit))
        .collect()
}

/// Ativa a entrada da toolbar (único ponto de mutação).
fn apply_toolbar_entry(state: &mut AppState, entry: &ToolbarEntry) {
    match entry.id {
        "select" => state.active_tool = "select".into(),
        "move" | "rotate" | "scale" => {
            state.active_tool = "transform".into();
            if let Some(gizmo) = entry.gizmo {
                state.gizmo_mode = gizmo;
            }
        }
        _ => state.active_tool = entry.id.into(),
    }
    state.pending_modal = None;
    state.mark_dirty();
}

fn entry_is_active(state: &AppState, entry: &ToolbarEntry) -> bool {
    match entry.id {
        "select" => state.active_tool == "select" || state.active_tool == "select_box",
        "move" | "rotate" | "scale" => {
            state.active_tool == "transform" && entry.gizmo.is_some_and(|g| state.gizmo_mode == g)
        }
        "transform" => state.active_tool == "transform",
        _ => state.active_tool == entry.id,
    }
}

/// Reserva vertical do rodapé (engrenagem de configuração fixada).
const TOOLBAR_FOOTER: f32 = 44.0;

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
                // Engrenagem modesta, sempre fixada no rodapé (fora da rolagem).
                ui.separator();
                ui.vertical_centered(|ui| {
                    draw_toolbar_config(ui, state);
                });
            });
        });
    crate::regions::record(
        ui.ctx(),
        crate::regions::RegionSlot::LeftTools,
        toolbar_resp.response.rect,
    );
}

fn draw_model_tools(ui: &mut egui::Ui, state: &mut AppState, _tools: &ToolRegistry, compact: bool) {
    // Normalização defensiva: se não estiver no modo de edição e a ferramenta ativa
    // for exclusiva de malha, reverte para a seleção básica de objetos.
    if state.mode != petunia_core::EditMode::Edit
        && canonical_toolbar_entries()
            .iter()
            .any(|e| e.edit_only && e.id == state.active_tool)
    {
        state.active_tool = "select".into();
        state.mark_dirty();
    }

    let entries = display_entries(state);
    // 2 colunas exigem ~100px; abaixo disso volta a 1 (responsivo).
    let two_cols = state.ui.toolbar_columns.clamp(1, 2) == 2 && ui.available_width() >= 100.0;
    let compact = if two_cols { true } else { compact };
    let mut seen_section = false;
    for section in [
        ToolbarSection::Primary,
        ToolbarSection::Inspect,
        ToolbarSection::Mesh,
    ] {
        let group: Vec<&ToolbarEntry> = entries.iter().filter(|e| e.section == section).collect();
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
                        draw_entry_button(ui, state, entry, true);
                    }
                });
            }
        } else {
            for entry in group {
                draw_entry_button(ui, state, entry, compact);
            }
        }
    }
}

/// Botão de uma entrada da toolbar.
fn draw_entry_button(ui: &mut egui::Ui, state: &mut AppState, entry: &ToolbarEntry, compact: bool) {
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
        apply_toolbar_entry(state, entry);
    }
}

/// Engrenagem de configuração da toolbar: visibilidade, ordem e colunas.
///
/// Tudo opera de imediato sobre `UiState` (sem placebo): esconder remove o
/// atalho, ↑/↓ reordena, 1/2 alterna as colunas, Redefinir volta ao canônico.
fn draw_toolbar_config(ui: &mut egui::Ui, state: &mut AppState) {
    let gear_tip = state.t("toolbar.configure");
    let resp =
        crate::widgets::PetuniaIconButton::new(PetuniaIcon::Settings, &gear_tip, 24.0).show(ui);
    egui::Popup::menu(&resp).show(|ui| {
        ui.set_min_width(250.0);
        ui.label(
            egui::RichText::new(state.t("toolbar.configure"))
                .strong()
                .size(12.0)
                .color(tokens::TEXT_PRIMARY),
        );
        ui.separator();

        // Semeia a ordem editável a partir do exibido (sem mutar no draw).
        let mut order: Vec<String> = if state.ui.toolbar_order.is_empty() {
            canonical_toolbar_entries()
                .iter()
                .map(|e| e.id.to_string())
                .collect()
        } else {
            state.ui.toolbar_order.clone()
        };
        // Ferramentas novas entram ao final.
        for entry in canonical_toolbar_entries() {
            if !order.iter().any(|id| id == entry.id) {
                order.push(entry.id.to_string());
            }
        }

        let mut dirty = false;
        let mut move_up: Option<usize> = None;
        let mut move_down: Option<usize> = None;
        egui::ScrollArea::vertical()
            .id_salt("toolbar_config_scroll")
            .max_height(320.0)
            .show(ui, |ui| {
                for (idx, id) in order.iter().enumerate() {
                    let label = canonical_toolbar_entries()
                        .iter()
                        .find(|e| e.id == id.as_str())
                        .map(|e| state.t(e.label_key))
                        .unwrap_or_else(|| id.clone());
                    ui.horizontal(|ui| {
                        let mut visible = !state.ui.toolbar_hidden.iter().any(|h| h == id);
                        if ui.checkbox(&mut visible, "").changed() {
                            if visible {
                                state.ui.toolbar_hidden.retain(|h| h != id);
                            } else if !state.ui.toolbar_hidden.iter().any(|h| h == id) {
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
        // Persiste a ordem semeada mesmo sem reordenar (estabiliza a lista).
        if state.ui.toolbar_order.is_empty() {
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

/// Paleta Animate (Wave 3): seleção + transform de pose. Sem ferramentas
/// fictícias: só ações que operam hoje (objetos/armatures via gizmo).
fn draw_animate_tools(ui: &mut egui::Ui, state: &mut AppState, compact: bool) {
    let entries = canonical_toolbar_entries();
    for id in ["select", "move", "rotate", "scale"] {
        if let Some(entry) = entries.iter().find(|e| e.id == id) {
            draw_entry_button(ui, state, entry, compact);
        }
    }
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
    fn test_toolbar_config_hides_reorders_and_two_columns() {
        use petunia_core::EditMode;
        let ctx = egui::Context::default();
        let tools = ToolRegistry::default();
        let mut state = AppState::new("en");
        state.mode = EditMode::Edit;

        // Esconder remove da exibição.
        state.ui.toolbar_hidden = vec!["knife".to_string(), "slice".to_string()];
        let shown: Vec<&str> = display_entries(&state).iter().map(|e| e.id).collect();
        assert!(!shown.contains(&"knife") && !shown.contains(&"slice"));
        assert!(shown.contains(&"mirror") && shown.contains(&"symmetrize"));

        // Ordem customizada é respeitada.
        state.ui.toolbar_hidden.clear();
        state.ui.toolbar_order = vec!["symmetrize".to_string(), "select".to_string()];
        let shown: Vec<&str> = display_entries(&state).iter().map(|e| e.id).collect();
        assert_eq!(&shown[..2], &["symmetrize", "select"]);

        // 2 colunas renderizam sem pânico.
        state.ui.toolbar_columns = 2;
        ctx.run_ui(egui::RawInput::default(), |ui| {
            draw(ui, &mut state, &tools);
        })
        .textures_delta
        .clear();
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
