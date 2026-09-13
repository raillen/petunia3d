# 03 — Acoplamento da Interface (UI Coupling Report)

> **Inventário de vazamento do egui fora da camada de apresentação, mutações diretas de estado na UI e controle de regras de negócio por widgets.**

---

## 1. Inventário de Dependências do egui Fora da Camada UI

A tabela abaixo detalha todas as ocorrências em que tipos e módulos do `egui` foram introduzidos em crates fora de `crates/ui`:

| Arquivo / Crate | Ocorrência / Símbolo egui | Camada | Justificada? | Diagnóstico e Problema |
| :--- | :--- | :---: | :---: | :--- |
| `crates/core/Cargo.toml:13` | `egui = { workspace = true }` | Core | **NÃO** | Core não pode compilar sem a biblioteca de UI. |
| `crates/core/src/state.rs:46` | `Option<egui::TextureHandle>` | Core | **NÃO** | Handle de textura de GPU do egui armazenado no modelo de imagem de referência. |
| `crates/core/src/state.rs:76` | `ensure_texture(&mut self, ctx: &egui::Context)` | Core | **NÃO** | Método de domínio exigindo contexto da UI para upload de textura. |
| `crates/core/src/state.rs:189` | `pub viewport_rect: Option<egui::Rect>` | Core | **NÃO** | Retângulo de tela da UI armazenado diretamente no estado do editor. |
| `crates/core/src/state.rs:202` | `pub canvas_tex: Option<egui::TextureHandle>` | Core | **NÃO** | Handle de textura da UI armazenado no estado do editor. |
| `crates/core/src/module.rs:10` | `fn ui(&mut self, _ctx: &egui::Context, _ui: &mut egui::Ui, ...)` | Core | **NÃO** | Trait central `Module` acoplada à biblioteca egui. |
| `crates/core/src/viewport.rs:11` | `rect: Option<egui::Rect>` | Core | **NÃO** | Conversão matemática de coordenadas usando struct de retângulo da UI em vez de `[f32; 4]` neutro. |
| `crates/config/Cargo.toml:10` | `egui = { workspace = true }` | Config | **NÃO** | Crate de configuração depende de UI para aplicar estilos. |
| `crates/config/src/theme.rs:9` | `use egui::Color32;` | Config | **NÃO** | Cores de tema armazenadas como tipos da UI em vez de hex ou `[u8; 4]`. |
| `crates/config/src/theme.rs:222`| `pub fn apply(&self, ctx: &egui::Context)` | Config | **NÃO** | Mutação do estilo do egui embutida no crate de configuração. |
| `crates/module-model/Cargo.toml:10` | `egui = { workspace = true }` | Módulo | **NÃO** | Módulo de modelagem depende de egui para desenhar sliders. |
| `crates/module-model/src/lib.rs:55` | `fn ui(&self, _ctx: &egui::Context, _ui: &mut egui::Ui, ...)` | Módulo | **NÃO** | Trait `Tool` exige egui para exibir controles na sidebar. |
| `crates/module-model/src/*.rs` | Sliders e botões em 14 arquivos de ferramentas | Módulo | **NÃO** | Código algorítmico e widgets egui coexistem no mesmo arquivo. |
| `crates/module-paint/Cargo.toml:10` | `egui = { workspace = true }` | Módulo | **NÃO** | Ferramenta de pintura depende de egui para paleta e canvas. |
| `crates/module-paint/src/lib.rs:189`| `fn ui(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, ...)` | Módulo | **NÃO** | Desenho de canvas e sliders egui misturados com pintura de textura. |
| `crates/module-paint/src/lib.rs:65` | `rfd::FileDialog::new()` | Módulo | **NÃO** | Módulo de pintura invoca diálogos de arquivo diretamente. |
| `crates/module-uv/Cargo.toml:10` | `egui = { workspace = true }` | Módulo | **NÃO** | Editor UV depende de egui para desenho 2D. |
| `crates/module-uv/src/lib.rs:97` | `p.add(egui::Shape::convex_polygon(...))` | Módulo | **NÃO** | Renderização vetorial da malha UV feita diretamente com `egui::Painter`. |
| `crates/module-assets/Cargo.toml:9` | `egui = { workspace = true }` | Módulo | **NÃO** | Módulo de assets depende de egui para lista de cartões. |
| `crates/render-gl/Cargo.toml:11` | `egui_glow = { workspace = true }` | Render | **QUESTIONÁVEL** | Pipeline GL herda tipos de glow através do egui. |
| `crates/app/src/lib.rs:18-21` | `egui`, `egui-winit`, `egui-wgpu`, `egui_glow` | App | **SIM** | Legítimo na camada de runner desktop para composição final. |

---

## 2. Mutações Diretas de Estado na Camada UI

Em uma arquitetura desacoplada, a UI atua estritamente como despachante de comandos semânticos (`dispatch(Command::ExtrudeFace { distance })`). No Petunia3D atual, a UI atua como **executora direta de regras de negócio**.

Foram catalogadas **mais de 1.000 ocorrências** de acesso e mutação direta a campos internos de `state`:

```text
crates/ui/src/outliner.rs:             179 ocorrências
crates/ui/src/viewport_bar.rs:         157 ocorrências
crates/ui/src/properties_panel.rs:     133 ocorrências
crates/ui/src/lib.rs:                  110 ocorrências
crates/ui/src/viewport_interaction.rs:  75 ocorrências
crates/ui/src/nav_gizmo.rs:             58 ocorrências
crates/ui/src/cutting_tests.rs:         50 ocorrências
crates/ui/src/modal_tests.rs:           46 ocorrências
crates/ui/src/annotation.rs:            45 ocorrências
crates/ui/src/measurement.rs:           45 ocorrências
crates/ui/src/toolbar.rs:               44 ocorrências
crates/ui/src/contextual_shelf.rs:      38 ocorrências
crates/ui/src/modal_viewport.rs:        36 ocorrências
crates/ui/src/timeline.rs:              31 ocorrências
crates/ui/src/asset_library_drawer.rs:  29 ocorrências
crates/ui/src/main_header.rs:           29 ocorrências
crates/ui/src/file_dialog_service.rs:   28 ocorrências
crates/ui/src/camera_controls.rs:       27 ocorrências
crates/ui/src/tool_fields.rs:           26 ocorrências
crates/ui/src/settings_modal.rs:        26 ocorrências
crates/ui/src/asset_browser.rs:         25 ocorrências
crates/ui/src/cutting.rs:               22 ocorrências
crates/ui/src/status_bar.rs:            20 ocorrências
```

### Exemplos Críticos de Violação

#### Exemplo 1: Exclusão de Geometria e Gestão Manual de Undo na UI
Arquivo: `crates/ui/src/properties_panel.rs` (linhas 45-48 e 337-340):
```rust
// A UI decide o rótulo do checkpoint, altera a topologia da malha,
// sincroniza seleção e emite eventos de mutação manualmente:
state.checkpoint("delete");
if let Some(mesh) = state.project.active_mesh_mut() {
    mesh.delete_selected();
}
state.sync_selection();
state.emit_mesh_changed();
```

#### Exemplo 2: Carregamento de Arquivo e Reset de Estado de Aplicação na UI
Arquivo: `crates/ui/src/lib.rs` (linhas 125-141):
```rust
// A UI abre o diálogo nativo do SO, lê o arquivo em disco e realiza
// o reset manual de 7 campos internos de estado do núcleo:
if let Some(path) = rfd::FileDialog::new().add_filter("Petunia", &["petunia"]).pick_file() {
    match format::load(&path) {
        Ok(p) => {
            state.palette = p.palette.clone();
            state.project = p;
            state.undo.clear();
            state.uv_selected.clear();
            state.project_path = Some(path.to_string_lossy().to_string());
            state.events.emit(petunia_core::AppEvent::ProjectLoaded);
            state.sync_selection();
            state.set_status(format!("open {}", path.display()));
        }
        Err(e) => state.set_status(format!("open err: {e}")),
    }
}
```

---

## 3. Máquinas de Estado Interativas Apropriadas pela UI

Quatro subsistemas fundamentais de modelagem e anotação têm suas **máquinas de estado e sessões interativas implementadas inteiramente dentro de widgets egui**:

1. **Sessão de Corte (`crates/ui/src/cutting.rs`)**:
   * A struct `CutSession` (que armazena a malha original, ponto de início, anel de loop cut e contagem de cortes) é guardada na memória temporária do egui (`ctx.data_mut()`).
   * A detecção de confirmação (`Enter`), cancelamento (`Escape`), arraste e cálculo de plano de corte ocorre exclusivamente dentro do método `draw` do egui.
2. **Sessão Modal de Transformação (`crates/ui/src/modal_viewport.rs`)**:
   * A struct `PointerSession` (âncora, buffer numérico digitado, estado de arraste de gizmo) é gerida via `egui::Id::new("modal.pointer")`.
   * A conversão de movimento do mouse em deltas de translação/rotação/escala depende de `Pos2` e `Rect` do egui.
3. **Sessão de Anotação 3D (`crates/ui/src/annotation.rs`)**:
   * O rascunho de traços livres depende de funções locais `project_to_screen` e `unproject_to_surface_or_plane` que recebem `egui::Pos2`, `egui::Rect` e `egui::Response`.
   * A projeção em tela e o desenho dos segmentos são realizados chamando `painter.line_segment()`.
4. **Sessão de Medição 3D (`crates/ui/src/measurement.rs`)**:
   * O snapping magnético a vértices e o traçado da régua com graduações métricas e badges dependem do `egui::Painter`.

### Conclusão do Diagnóstico de UI
Se a camada `petunia_ui` for excluída, **desaparecem junto com ela**:
* Todos os fluxos de corte de malha (faca, bissecção e loop cut);
* Todas as transformações interativas por mouse e teclado numérico;
* A capacidade de desenhar anotações e réguas 3D;
* A lógica de abrir, salvar, importar e exportar arquivos.
