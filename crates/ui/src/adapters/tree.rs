//! Adapter de árvore hierárquica (`egui_ltreeview`).
//!
//! Único arquivo autorizado a mencionar `egui_ltreeview`. Painéis consomem os
//! itens reexportados daqui — antes, `outliner.rs` importava a crate
//! diretamente, o que violava a regra de dependência §22.1 (um panel conhecendo
//! a crate auxiliar).
//!
//! ## O que este adapter é hoje
//!
//! Uma **fachada de confinamento**: reexporta os tipos que a árvore de Parts
//! precisa e documenta o contrato. Product code depende de
//! `crate::adapters::tree::*`, então trocar de implementação é um diff de um
//! arquivo.
//!
//! ## O que ainda falta (Wave 2 em diante)
//!
//! O adapter ideal recebe um **view-model Petunia** (`PartsNode`) e não expõe
//! tipos da crate. Hoje `outliner.rs` ainda monta `NodeBuilder`/`TreeView`
//! diretamente, porque a árvore tem seleção múltipla, acessibilidade e
//! culling próprios que precisam ser preservados. A conversão para view-model é
//! incremental e não pode regredir interação — está registrada na checklist da
//! Wave 2, não silenciada aqui.

pub use egui_ltreeview::{Action, NodeBuilder, TreeView, TreeViewSettings, TreeViewState};

/// Seleção múltipla é parte do contrato da árvore de Parts (comando, Ctrl e
/// Shift); centralizada aqui para não ser reimplementada por painel.
pub fn multi_select_settings() -> TreeViewSettings {
    TreeViewSettings::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_reexports_the_tree_contract() {
        // Se a crate mudar de nome ou de API, este é o único arquivo a ajustar.
        let _settings: TreeViewSettings = multi_select_settings();
        let _ = std::any::type_name::<TreeViewState<u32>>();
    }
}
