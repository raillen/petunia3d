//! module-assets — Asset Library (§13): múltiplos assets com UUID,
//! rename/duplicate/delete/search. Reage a `ActiveAssetChanged`.

use petunia_core::{AppEvent, AppState, Module, Workspace};
use petunia_mesh::Mesh;

#[derive(Default)]
pub struct AssetsModule {
    pub filter: String,
}

impl AssetsModule {
    pub fn new() -> Self {
        Self {
            filter: String::new(),
        }
    }

    pub fn add_primitive(state: &mut AppState, name: &str, mesh: Mesh) {
        state.checkpoint("add asset");
        state.project.add(name, mesh);
        let id = state.project.assets[state.project.active].id;
        state
            .events
            .emit(AppEvent::ActiveAssetChanged { asset_id: id });
        state.sync_selection();
        state.emit_mesh_changed();
    }

    pub fn duplicate_active(state: &mut AppState) {
        let idx = state.project.active;
        if let Some(a) = state.project.assets.get(idx).cloned() {
            state.checkpoint("duplicate asset");
            let dup = a.duplicate();
            let id = dup.id;
            state.project.assets.push(dup);
            state.project.active = state.project.assets.len() - 1;
            state
                .events
                .emit(AppEvent::ActiveAssetChanged { asset_id: id });
            state.sync_selection();
            state.emit_mesh_changed();
        }
    }

    pub fn delete_active(state: &mut AppState) {
        state.checkpoint("delete asset");
        let active = state.project.active;
        state.project.remove(active);
        let id = state
            .project
            .assets
            .get(state.project.active)
            .map(|a| a.id)
            .unwrap_or_default();
        state
            .events
            .emit(AppEvent::ActiveAssetChanged { asset_id: id });
        state.sync_selection();
        state.emit_mesh_changed();
    }

    /// Mescla a malha de outro asset com o asset ativo e remove o asset de origem.
    pub fn join_asset(state: &mut AppState, source_idx: usize) {
        if source_idx == state.project.active || source_idx >= state.project.assets.len() {
            return;
        }
        state.checkpoint("join asset");
        let source_mesh = state.project.assets[source_idx].mesh.clone();
        if let Some(active_asset) = state.project.active_mut() {
            active_asset.mesh.join(&source_mesh);
        }
        state.project.remove(source_idx);
        state.sync_selection();
        state.emit_mesh_changed();
        state.set_status("asset joined into active");
    }

    /// Assets visíveis após o filtro (índices reais).
    pub fn filtered(state: &AppState, filter: &str) -> Vec<usize> {
        let f = filter.to_lowercase();
        state
            .project
            .assets
            .iter()
            .enumerate()
            .filter(|(_, a)| f.is_empty() || a.name.to_lowercase().contains(&f))
            .map(|(i, _)| i)
            .collect()
    }
}

impl Module for AssetsModule {
    fn id(&self) -> &'static str {
        "assets"
    }
    fn on_event(&mut self, _event: &AppEvent, _state: &mut AppState) {}

    fn as_any(&self) -> &(dyn std::any::Any + 'static) {
        self
    }

    fn as_any_mut(&mut self) -> &mut (dyn std::any::Any + 'static) {
        self
    }
}

impl AssetsModule {
    pub fn ui(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui, state: &mut AppState) {
        let l_lib = state.t("ui.assets");
        let l_search = state.t("ui.search");
        egui::CollapsingHeader::new(l_lib)
            .default_open(true)
            .show(ui, |ui| {
                let total_assets = state.project.assets.len();
                let total_tris: usize = state
                    .project
                    .assets
                    .iter()
                    .map(|a| a.mesh.tri_count())
                    .sum();
                ui.small(format!("{total_assets} assets | {total_tris} tris"));

                ui.horizontal(|ui| {
                    ui.label(l_search);
                    if ui.text_edit_singleline(&mut self.filter).changed() {
                        state.mark_dirty();
                    }
                });
                let list = AssetsModule::filtered(state, &self.filter.clone());
                let mut del: Option<usize> = None;
                let mut activate: Option<usize> = None;
                let mut join_target: Option<usize> = None;
                for i in list {
                    let (name, vis, tris, is_active) = {
                        let a = &state.project.assets[i];
                        (
                            a.name.clone(),
                            a.visible,
                            a.mesh.tri_count(),
                            state.project.active == i,
                        )
                    };
                    ui.horizontal(|ui| {
                        let mut v = vis;
                        if ui.checkbox(&mut v, "").changed() {
                            if let Some(o) = state.project.assets.get_mut(i) {
                                o.visible = v;
                            }
                            state.mark_dirty();
                        }
                        if ui
                            .selectable_label(is_active, format!("{name} ({tris})"))
                            .clicked()
                        {
                            activate = Some(i);
                        }
                        if !is_active
                            && ui
                                .small_button("⚯")
                                .on_hover_text("Join into active asset")
                                .clicked()
                        {
                            join_target = Some(i);
                        }
                        if ui
                            .small_button("⧉")
                            .on_hover_text(state.t("ui.duplicate"))
                            .clicked()
                        {
                            state.project.active = i;
                            AssetsModule::duplicate_active(state);
                        }
                        if ui.small_button("✕").clicked() {
                            del = Some(i);
                        }
                    });
                }
                if let Some(i) = activate {
                    state.project.active = i;
                    state.uv_selected.clear();
                    let id = state.project.assets[i].id;
                    state
                        .events
                        .emit(petunia_core::AppEvent::ActiveAssetChanged { asset_id: id });
                    state.sync_selection();
                }
                if let Some(i) = join_target {
                    AssetsModule::join_asset(state, i);
                }
                if let Some(i) = del {
                    state.project.active = i;
                    AssetsModule::delete_active(state);
                }
                ui.horizontal(|ui| {
                    if ui.small_button(state.t("ui.rename")).clicked() {
                        // rename inline: usa o painel de propriedades do MODEL
                        state.workspace = Workspace::Model;
                    }
                });
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assets_filter_and_operations() {
        let mut state = AppState::new("en");
        assert_eq!(state.project.assets.len(), 1);

        // Add primitives
        AssetsModule::add_primitive(&mut state, "Sphere", Mesh::cube(1.0));
        AssetsModule::add_primitive(&mut state, "Cylinder", Mesh::cube(1.0));
        assert_eq!(state.project.assets.len(), 3);

        // Filter
        let all = AssetsModule::filtered(&state, "");
        assert_eq!(all.len(), 3);

        let cyl = AssetsModule::filtered(&state, "cyl");
        assert_eq!(cyl.len(), 1);
        assert_eq!(state.project.assets[cyl[0]].name, "Cylinder");

        // Duplicate active
        state.project.active = 1;
        AssetsModule::duplicate_active(&mut state);
        assert_eq!(state.project.assets.len(), 4);
        assert!(state.project.assets.last().unwrap().name.contains("copy"));

        // Join asset into active
        state.project.active = 0;
        let v0 = state.project.assets[0].mesh.verts.len();
        let v1 = state.project.assets[1].mesh.verts.len();
        AssetsModule::join_asset(&mut state, 1);
        assert_eq!(state.project.assets.len(), 3);
        assert_eq!(state.project.assets[0].mesh.verts.len(), v0 + v1);
    }
}
