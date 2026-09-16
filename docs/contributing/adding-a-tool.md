# Como Adicionar uma Ferramenta

Ferramentas de modelagem vivem em `crates/module-model` e seguem a cadeia funcional
canônica do projeto:

```
Tool → Command → Algorithm → Data
```

- **Tool** interpreta intenção do usuário e monta uma sessão interativa;
- **Command** é a unidade transacional registrada no Command Registry (undo, MCP, Lua e
  command palette enxergam a mesma ação);
- **Algorithm** é a operação pura sobre a malha, sem UI, sem Undo, sem Lua;
- **Data** é a malha/documento (single-writer, handles generacionais validados).

**Regra dura:** `Tool` não chama `Tool`. A coordenação acontece por Command.

## Passo a passo

1. **Crie o algoritmo puro** em `crates/mesh/src/<operacao>.rs`, testável sem UI:

   ```rust
   /// Aplica a operação pura. Não conhece UI, Undo nem CommandId.
   pub fn minha_operacao(mesh: &mut PetuniaMesh, params: MinhaParams) -> Result<(), MeshError> { .. }
   ```

2. **Registre o Command** no catálogo, com metadados semânticos — nunca com texto,
   ícone ou atalho literais:

   ```rust
   CommandSpec {
       id: CommandId::new("model.minha_operacao"),
       label_key: TextId::new("commands.model.minha_operacao"),
       category: CommandCategory::Model,
       destructive: true,
       docs_topic: Some("tools/minha-operacao".into()),
   }
   ```

   - O rótulo visível só existe como `TextId` em `assets/locales/{en,pt-BR}.toml`.
   - O ícone é um `IconId` resolvido pelo pack ativo (`PetuniaIcon::*`), **nunca** um
     caractere unicode ou emoji.
   - Não declare tecla física: o atalho é um bind de keymap (abaixo).

3. **Escreva a Tool** que monta a sessão interativa e delega ao Command:

   ```rust
   impl Tool for MinhaTool {
       fn id(&self) -> CommandId { CommandId::new("model.minha_operacao") }
       fn begin(&mut self, ctx: &mut ToolContext) -> ToolSession {
           // preview derivado da fonte; commit só no fim
           ToolSession::modal("model.minha_operacao")
       }
   }
   ```

   - Ativar a ferramenta **nunca** muta geometria nem histórico.
   - Preview é derivado do snapshot da fonte; o commit grava **uma** entrada de Undo.
   - Erros retornam erro tipado — nunca pânico em input externo.

4. **Declare o bind** no perfil de keymap canônico (`assets/keymaps/petunia-default.toml`)
   apontando para o `CommandId`, e valide conflitos.

5. **Registre** a Tool no módulo (`register::<MinhaTool>()`) e habilite em
   `assets/tools.toml` quando aplicável.

6. **Sincronize a documentação no mesmo PR**:
   - `docs/tools/<ferramenta>.md` (com vocabulário de usuário: `Point`, `Round Edge`);
   - regenere as referências derivadas: `cargo run -p xtask -- docs-generate`;
   - `CHANGELOG.md` em `[Unreleased]`;
   - nó novo em `docs/public/ui-map.json` se houver superfície de UI nova
     (`cargo run -p xtask -- ui-check`);
   - screenshots reais se a UI mudou.

7. **Valide** antes de abrir o PR:

   ```bash
   cargo fmt --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test -p petunia_mesh -p petunia_commands
   cargo run -p xtask -- bible-check
   cargo run -p xtask -- docs-generate --check
   ```

## Checklist de invariantes

- [ ] core não depende de `egui`/`wgpu`; a Tool não vaza tipo de UI para o algoritmo;
- [ ] nenhum texto, ícone, cor ou atalho hardcoded;
- [ ] operação transacional com preview, validação e rollback;
- [ ] `Tool` não chama `Tool`;
- [ ] documentação, changelog e referências geradas atualizados.
