//! Painel de Edição e Projeção UV (`uv_ui`).
//! Renderiza controles de projeção/escala UV e canvas interativo de edição 0..1.

use egui::{Color32, Pos2, Shape, Stroke, Ui};
use petunia_config::text_id;
use petunia_core::AppState;
use petunia_module_uv::UvModule;

pub fn draw_uv_panel(ui: &mut Ui, state: &mut AppState) {
    let l_uv = state.t_id(text_id::UV_TITLE);
    let l_reproj = state.t("uv.reproject");
    let l_scale = state.t("uv.scale");
    egui::CollapsingHeader::new(l_uv)
        .default_open(true)
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui
                    .button(l_reproj)
                    .on_hover_text("Projeção planar no eixo dominante")
                    .clicked()
                {
                    UvModule::reproject(state);
                }
                if ui
                    .button("Cúbica (Box)")
                    .on_hover_text("Projeção cúbica nos 6 eixos")
                    .clicked()
                {
                    UvModule::project_cube(state);
                }
                if ui
                    .button("Auto Unwrap")
                    .on_hover_text("Desdobramento automático de malha via xatlas")
                    .clicked()
                {
                    match UvModule::unwrap_auto(state) {
                        Ok(charts) => {
                            state.set_status(format!("Auto Unwrap: {charts} ilhas geradas"))
                        }
                        Err(e) => state.set_status(format!("Erro no Unwrap: {e}")),
                    }
                }
                if ui.button(format!("{l_scale} +")).clicked() {
                    state.checkpoint("uv scale");
                    UvModule::scale_selected(state, 1.1);
                }
                if ui.button(format!("{l_scale} −")).clicked() {
                    state.checkpoint("uv scale");
                    UvModule::scale_selected(state, 1.0 / 1.1);
                }
                if ui
                    .button("90°")
                    .on_hover_text("Gira seleção 90 graus")
                    .clicked()
                {
                    state.checkpoint("uv rotate");
                    UvModule::rotate_selected(state, std::f32::consts::FRAC_PI_2);
                    state.emit_mesh_changed();
                }
            });
            ui.small(format!(
                "{}: {}",
                state.t_id(text_id::UV_SELECTED),
                state.uv_selected.len()
            ));
            // canvas UV 0..1 (preenche o pai: inspector estreito ou editor central).
            let size = ui.available_width().clamp(200.0, 720.0);
            let (rect, resp) =
                ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::click_and_drag());
            let p = ui.painter_at(rect);
            p.rect_filled(rect, 4.0, Color32::from_rgb(22, 22, 26));
            let to_screen =
                |u: f32, v: f32| egui::pos2(rect.min.x + u * size, rect.min.y + (1.0 - v) * size);
            if let Some(o) = state.project.assets.get(state.project.active) {
                for (fi, f) in o.mesh.faces.iter().enumerate() {
                    let pts: Vec<Pos2> = f.uv.iter().map(|q| to_screen(q[0], q[1])).collect();
                    if pts.len() < 3 {
                        continue;
                    }
                    let sel = state.uv_selected.contains(&fi) || f.selected;
                    let col = if sel {
                        Color32::from_rgb(255, 150, 50)
                    } else {
                        Color32::from_rgb(120, 170, 255)
                    };
                    p.add(Shape::convex_polygon(
                        pts,
                        if sel {
                            Color32::from_rgba_unmultiplied(255, 150, 50, 40)
                        } else {
                            Color32::TRANSPARENT
                        },
                        Stroke::new(1.0_f32, col),
                    ));
                }
            }
            // clique seleciona face (hit test); drag move selecionadas
            if resp.clicked()
                && let Some(pos) = resp.interact_pointer_pos()
            {
                let u = (pos.x - rect.min.x) / size;
                let v = 1.0 - (pos.y - rect.min.y) / size;
                if let Some(fi) = UvModule::uv_hit(state, u, v) {
                    state.checkpoint("uv select");
                    if let Some(o) = state.project.active_mut()
                        && let Some(f) = o.mesh.faces.get_mut(fi)
                    {
                        f.selected = !f.selected;
                    }
                    state.sync_selection();
                }
            }
            if resp.drag_started() {
                state.checkpoint("uv move");
            }
            if resp.dragged() {
                let d = resp.drag_delta();
                UvModule::move_selected(state, d.x / size, -d.y / size);
            }
            if resp.drag_stopped() {
                state.emit_mesh_changed();
            }
            if resp.hovered() {
                let z = ui.input(|i| i.smooth_scroll_delta.y);
                if z.abs() > 0.5 {
                    state.checkpoint("uv scale");
                    UvModule::scale_selected(state, if z > 0.0 { 1.1 } else { 1.0 / 1.1 });
                    state.emit_mesh_changed();
                }
            }
            ui.small(state.t_id(text_id::UV_HINT));
        });
}

/// Resumo UV para o inspector do workspace UV (Wave 3).
///
/// O editor interativo mora no centro (§6.3); aqui só o essencial de contexto,
/// sem duplicar widgets interativos.
pub fn draw_uv_summary(ui: &mut Ui, state: &mut AppState) {
    ui.label(
        egui::RichText::new(state.t_id(text_id::UV_TITLE))
            .size(12.0)
            .strong(),
    );
    if let Some(asset) = state.project.assets.get(state.project.active) {
        let selected = asset.mesh.faces.iter().filter(|f| f.selected).count();
        ui.label(format!(
            "{}: {}",
            state.t_id(text_id::UI_ASSETS),
            asset.name
        ));
        ui.label(format!(
            "{}: {} / {}",
            state.t_id(text_id::UV_FACES),
            selected,
            asset.mesh.faces.len()
        ));
    }
    ui.label(format!(
        "{}: {}",
        state.t_id(text_id::UV_SELECTED),
        state.uv_selected.len()
    ));
    ui.small(state.t_id(text_id::UV_HINT));
}
