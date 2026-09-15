# Como Adicionar uma Ferramenta

Ferramentas de modelagem são plugins registrados em `crates/module-model`:

1. Crie `crates/module-model/src/minha.rs`:
   ```rust
   #[derive(Default)]
   pub struct MinhaTool;
   impl Tool for MinhaTool {
       fn id(&self) -> &'static str { "minha" }
       fn label_key(&self) -> &'static str { "tools.minha" }
       fn hint_key(&self) -> &'static str { "hints.minha" }
       fn icon(&self) -> &'static str { "⬆" }
       fn shortcut(&self) -> &'static str { "X" }
       fn on_activate(&self, state: &mut AppState) { /* sem geometria, sem histórico */ }
   }
   ```
2. Registre em `crates/module-model/src/lib.rs` (`register::<MinhaTool>()`).
3. Habilite em `assets/tools.toml` (`minha = true`).
4. Adicione os textos em `assets/locales/en.toml` e `pt-BR.toml`.
5. Regras: ativar nunca muta geometria nem histórico; mutação usa `state.checkpoint()` antes; erros retornam `CommandError`, nunca pânico em input externo.

Inspire-se em ferramentas pequenas existentes (`merge.rs`, `mirror.rs`).
