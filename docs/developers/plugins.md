# Sistema de Plugins & Módulos

O Petunia3D foi desenhado com base no contrato dinâmico `dyn Module`, permitindo que novas ferramentas e abas sejam integradas sem modificar o núcleo do sistema.

---

## O Trait `Module`

Definido em `crates/core/src/module.rs`:

```rust
pub trait Module: Send + Sync {
    /// Nome identificador do módulo.
    fn name(&self) -> &'static str;
    
    /// Inicialização do módulo quando a aplicação sobe.
    fn initialize(&mut self, state: &mut AppState) -> Result<(), ModuleError>;
    
    /// Desenho de painéis customizados no egui Context.
    fn draw_ui(&mut self, ctx: &egui::Context, state: &mut AppState);
    
    /// Processamento de eventos do sistema.
    fn on_event(&mut self, event: &AppEvent, state: &mut AppState);
}
```

Cada workspace do Petunia3D (`Modeling`, `Paint`, `UV`, `Assets`) implementa essa trait, garantindo isolamento completo de responsabilidades.
