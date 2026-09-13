//! Contrato de módulo (§23): features conhecem abstrações, não módulos.

use std::any::Any;

use super::events::AppEvent;
use super::state::AppState;

/// Ciclo de vida mínimo de módulo do editor.
pub trait Module: 'static + Send + Sync {
    fn id(&self) -> &'static str;
    fn on_event(&mut self, _event: &AppEvent, _state: &mut AppState) {}
    fn as_any(&self) -> &(dyn Any + 'static);
    fn as_any_mut(&mut self) -> &mut (dyn Any + 'static);
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
    pub fn get(&self, id: &str) -> Option<&(dyn Module + 'static)> {
        for m in self.modules.iter() {
            if m.id() == id {
                return Some(&**m);
            }
        }
        None
    }
    pub fn get_mut(&mut self, id: &str) -> Option<&mut (dyn Module + 'static)> {
        for m in self.modules.iter_mut() {
            if m.id() == id {
                return Some(&mut **m);
            }
        }
        None
    }
}
