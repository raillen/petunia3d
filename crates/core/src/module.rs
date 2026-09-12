//! Contrato de módulo (§23): features conhecem abstrações, não módulos.

use super::events::AppEvent;
use super::state::AppState;

/// Ciclo de vida mínimo. `ui` desenha o painel do módulo na sidebar.
pub trait Module {
    fn id(&self) -> &'static str;
    fn on_event(&mut self, _event: &AppEvent, _state: &mut AppState) {}
    fn ui(&mut self, _ctx: &egui::Context, _ui: &mut egui::Ui, _state: &mut AppState) {}
}

/// Registro ordenado (usado p/ dispatch de eventos).
#[derive(Default)]
pub struct ModuleRegistry {
    modules: Vec<Box<dyn Module>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register<M: Module + 'static>(&mut self, m: M) {
        self.modules.push(Box::new(m));
    }
    pub fn dispatch(&mut self, ev: &AppEvent, state: &mut AppState) {
        for m in &mut self.modules {
            m.on_event(ev, state);
        }
    }
    pub fn get(&self, id: &str) -> Option<&(dyn Module + '_)> {
        for m in self.modules.iter() {
            if m.id() == id {
                return Some(&**m);
            }
        }
        None
    }
    pub fn get_mut(&mut self, id: &str) -> Option<&mut (dyn Module + '_)> {
        for m in self.modules.iter_mut() {
            if m.id() == id {
                return Some(&mut **m);
            }
        }
        None
    }
}
