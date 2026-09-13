# 05 — Arquitetura de Comandos (Command System Report)

> **Auditoria do sistema de comandos, análise da ausência de dispatcher semântico e mapeamento da execução ad-hoc de regras de negócio.**

---

## 1. A Realidade do Crate `petunia_commands`

A documentação conceitual e comentários do projeto sugerem o uso do *Command Pattern*. No entanto, a inspeção do código-fonte em `crates/commands/src/lib.rs` revela a seguinte implementação:

```rust
pub struct UndoStack<T: Clone> {
    undo: Vec<(String, T)>,
    redo: Vec<(String, T)>,
    cap: usize,
}
```

### O que o crate `petunia_commands` realmente é:
* Um buffer genérico de **snapshots em memória** baseado em clonagem integral do estado (`T: Clone`, onde `T` é `Project`);
* Uma fila com capacidade máxima de 100 níveis que empilha tuplas `(String, Project)` a cada chamada de `checkpoint(label, &current)`;
* Métodos utilitários de navegação: `can_undo()`, `can_redo()`, `undo(current)`, `redo(current)` e `clear()`.

### O que NÃO existe no crate nem em nenhum lugar do projeto:
* **NÃO existe** `trait Command`;
* **NÃO existe** `enum CommandId`;
* **NÃO existe** `struct CommandDispatcher` ou barramento de comandos;
* **NÃO existe** tipagem estruturada de parâmetros de comando;
* **NÃO existe** retorno semântico (`Result<CommandSuccess, DomainError>`).

---

## 2. Como as Ações são Realmente Executadas Hoje

Hoje, qualquer operação de edição de malha, duplicação, exclusão ou salvamento é executada de duas formas ad-hoc:

### 2.1. Via Match de Strings no Tratador de Teclado (`crates/app/src/lib.rs`)
O método `Core::on_key` consulta o mapa de atalhos e faz um casamento com strings literais:
```rust
match action.as_str() {
    "model.delete" => {
        self.state.checkpoint("delete");
        if let Some(m) = self.state.project.active_mesh_mut() {
            m.delete_selected();
        }
        self.state.sync_selection();
        self.state.emit_mesh_changed();
    }
    "model.invert_selection" => {
        if let Some(m) = self.state.project.active_mesh_mut() {
            m.invert_selection();
        }
        self.state.sync_selection();
        self.state.mark_dirty();
    }
    // ...
}
```

### 2.2. Via Closures Anônimas Diretamente dentro dos Widgets da UI
Nos painéis de interface, a ação é executada diretamente quando o botão egui é clicado:
```rust
// crates/ui/src/properties_panel.rs:
if widgets::petunia_action_button(ui, Some(PetuniaIcon::Trash), "Delete", true).clicked() {
    state.checkpoint("delete asset");
    if active_idx < state.project.assets.len() {
        state.project.assets.remove(active_idx);
        // Ajuste manual de índice ativo, sincronização de seleção e dirty flag:
        if state.project.assets.is_empty() {
            state.project.assets.push(Asset::new("Cube", Mesh::cube(1.0)));
        }
        state.project.active = state.project.active.min(state.project.assets.len() - 1);
        state.sync_selection();
        state.emit_mesh_changed();
        state.mark_dirty();
    }
}
```

---

## 3. Duplicação de Lógica de Operações Críticas

Como não existe um despacho centralizado, a mesma operação de negócio é **reimplementada repetidamente** em múltiplos arquivos da UI com ligeiras variações:

| Operação de Domínio | Arquivos onde está Reimplementada Ad-Hoc |
| :--- | :--- |
| **Duplicar Objeto / Asset** | `properties_panel.rs`, `outliner.rs`, `asset_browser.rs`, `asset_library_drawer.rs`, `nav_gizmo.rs`, `viewport_bar.rs` (6 lugares) |
| **Excluir Objeto / Asset** | `properties_panel.rs`, `outliner.rs`, `asset_browser.rs`, `asset_library_drawer.rs`, `viewport_bar.rs` (5 lugares) |
| **Inserir Primitiva 3D** | `viewport_bar.rs`, `outliner.rs`, `nav_gizmo.rs`, `module-model/src/primitives.rs` (4 lugares) |
| **Subdividir Seleção** | `viewport_bar.rs`, `contextual_shelf.rs`, `nav_gizmo.rs`, `module-model/src/subdivide.rs` (4 lugares) |
| **Unir Vértices (Merge)** | `viewport_bar.rs`, `contextual_shelf.rs`, `module-model/src/merge.rs` (3 lugares) |

Se a regra de negócios de "Duplicar Objeto" precisar mudar (por exemplo, gerando um novo nome `Objeto.001` ou disparando um evento de telemetria), o desenvolvedor precisará localizar e atualizar **6 arquivos diferentes**.

---

## 4. Resposta à Pergunta Crítica da Auditoria

> **"É possível executar uma operação como `Extrude` sem egui, teclado, mouse ou widget?"**

### Diagnóstico:
* **Algoritmicamente:** SIM. O método `mesh.extrude_selected(dist)` em `petunia_mesh` é puro e aceita chamadas diretas.
* **Arquiteturalmente no Sistema do Editor:** **NÃO**.
  * Não existe como uma aplicação externa, um script de teste, um plugin ou um frontend secundário solicitar:
    ```rust
    editor.dispatch(Command::Extrude { distance: 1.5 })
    ```
  * O código que lê o input, inicia a transação modal, aplica pré-visualizações, decide o momento do checkpoint de undo e emite os eventos de notificação só existe espalhado entre `modal.rs`, `modal_viewport.rs` e `module-model/src/extrude.rs`.

---

## 5. Requisitos para a Camada de Aplicação Futura

Para que qualquer frontend ou teste headless opere com paridade comportamental, o Petunia3D precisa introduzir uma camada `petunia_app_api` ou `petunia_application` contendo:
1. **Tipos de Comando Semânticos**: DTOs claros contendo apenas dados puros (`AddPrimitiveCmd { kind, position }`, `DeleteSelectionCmd`, `TransformSelectionCmd { delta, mode }`);
2. **`CommandDispatcher` Central**: Ponto único de entrada para todas as mutações;
3. **Pipeline Automático de Undo**: O dispatcher deve ser responsável por invocar `undo.checkpoint()` antes de despachar mutações destrutivas, eliminando a gravação manual de checkpoints pela UI;
4. **Respostas Estruturadas**: Retorno de `Result<(), CommandError>` permitindo que a interface ou cliente reporte falhas sem que o domínio decida o formato da mensagem ao usuário.
