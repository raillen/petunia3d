# 10 — Prontidão para Execução Headless (Headless Readiness Report)

> **Diagnóstico da capacidade de execução sem interface gráfica, análise de testes em modo headless e avaliação de viabilidade de uma CLI de automação.**

---

## 1. O que Funciona em Modo Headless Hoje

Três crates do workspace já são **100% desacoplados de interface gráfica e rodam perfeitamente em modo headless**:

1. **`petunia_mesh`**:
   * Executa 53 testes automatizados em menos de 1 segundo;
   * Cria primitivas 3D, calcula topologia half-edge, executa extrusões, insets, chanfros, loop cuts, bissecções e triangulações;
   * Importa e exporta strings de arquivos Wavefront OBJ;
   * Não referencia nenhuma biblioteca de UI ou sistema de janelas.
2. **`petunia_project`**:
   * Executa 13 testes automatizados;
   * Salva e carrega projetos `.petunia` com serialização compacta `postcard`;
   * Exporta malhas e materiais para arquivos GLB (glTF 2.0 binário) e OBJ;
   * Valida paletas de cores (GPL, HEX) e coleções de dados;
   * Zero dependências de egui ou GPU.
3. **`petunia_commands`**:
   * Executa testes unitários de empilhamento e desempilhamento de snapshots de undo/redo.

---

## 2. O que IMPEDE a Execução do Core em Modo Headless Hoje

Apesar da independência dos crates de geometria e persistência, **o núcleo do editor (`petunia_core`) NÃO pode ser executado nem compilado de forma headless**:

1. **Dependência Mandatória do `egui` no Manifesto**:
   * `crates/core/Cargo.toml` declara `egui = { workspace = true }`. Qualquer projeto ou binário que importe `petunia_core` puxará compilação de toda a árvore do egui.
2. **Campos da UI Embutidos em `AppState`**:
   * `AppState` contém `Option<egui::Rect>` e `Option<egui::TextureHandle>`.
3. **Ausência de um Orquestrador de Sessão Headless**:
   * Não existe uma struct como `HeadlessEditor` ou `EditorSession`.
   * A struct `Core` atual vive exclusivamente dentro de `crates/app/src/lib.rs` e exige `winit::window::Window`, `egui-winit`, superfícies gráficas nativas e loop de eventos de janela.
4. **Ferramentas Interativas Confinadas ao Loop Visual**:
   * Operações modais com mouse (arrasto de vértices, snap, corte por faca, bissecção) só têm lógica de interpretação de deltas dentro de métodos de desenho de widgets do egui (`modal_viewport.rs`, `cutting.rs`).

---

## 3. Experimento Mental: A Criação de uma `petunia-cli`

Imagine o seguinte caso de uso: uma ferramenta de linha de comando para automação em pipelines de jogos (CI/CD) que gere modelos parametricamente:

```bash
petunia-cli new --primitive cube
petunia-cli select --face 0
petunia-cli extrude --distance 1.5
petunia-cli undo
petunia-cli save my_model.petunia
petunia-cli export my_model.glb
```

### O que acontece se tentarmos construir essa CLI hoje?
* A CLI **não conseguirá reutilizar `petunia_core` nem `petunia_app`** sem arrastar dependências de janela e egui;
* A CLI **não terá um `CommandDispatcher`** para despachar `select`, `extrude` ou `undo`;
* O desenvolvedor da CLI seria forçado a reimplementar manualmente a orquestração de seleção e histórico chamando `petunia_mesh` diretamente.

---

## 4. Requisitos para Habilitar 100% de Prontidão Headless

1. **Remover o `egui` do `petunia_core`**:
   * Substituir `egui::Rect` por `[f32; 4]` ou `LogicalRect { min: [f32; 2], max: [f32; 2] }`;
   * Mover `TextureHandle` para o frontend egui;
   * Mover `Module::ui` da trait central `Module` para uma trait de extensão de apresentação.
2. **Criar a Camada `EditorSession` / `CommandDispatcher`**:
   * Uma struct pura que mantém `Project`, `Selection`, `Camera` e `UndoStack`, e expõe `dispatch(Command) -> Result<(), CommandError>`.
3. **Testes Headless do Editor**:
   * Criar suíte de testes de integração em Rust que instancie a sessão, adicione um cubo, selecione uma face, extrua, desfaça via `undo()`, salve o arquivo e verifique o resultado — **sem carregar nenhuma janela, GPU ou widget visual**.
