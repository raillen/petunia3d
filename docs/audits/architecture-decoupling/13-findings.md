# 13 — Catálogo de Achados de Auditoria (Findings Catalog)

> **Catálogo padronizado de achados arquiteturais, classificados por severidade, categoria, evidência física e impacto no desacoplamento.**

---

## 1. Índice Resumido dos Achados

| ID | Título do Achado | Severidade | Categoria | Confiança |
| :---: | :--- | :---: | :--- | :---: |
| **F-001** | `petunia_core` depende diretamente de `egui` e incorpora tipos visuais | **P0 (Crítica)** | UI-Leak / Inversão de Dependência | **ALTA** |
| **F-002** | `AppState` atua como God Object agregando 60+ campos heterogêneos | **P0 (Crítica)** | God State / Baixa Coesão | **ALTA** |
| **F-003** | Ausência de comandos semânticos e dispatcher no ecossistema | **P0 (Crítica)** | Command Architecture | **ALTA** |
| **F-004** | A camada de UI executa mutações de domínio e grava undo checkpoints | **P0 (Crítica)** | UI Coupling / Regra de Negócio na UI | **ALTA** |
| **F-005** | Trait `Tool` não executa comportamento e acopla módulos ao egui | **P1 (Alta)** | Tool Coupling / Abstração Falha | **ALTA** |
| **F-006** | Máquinas de estado interativas de ferramentas vivem dentro do egui | **P1 (Alta)** | Sessão Confinada na UI | **ALTA** |
| **F-007** | `petunia_config` depende de egui para aplicar temas visuais | **P1 (Alta)** | Inversão de Dependência | **ALTA** |
| **F-008** | Crates `module-*` trazem dependência direta de egui em seus manifestos | **P1 (Alta)** | Acoplamento de Apresentação | **ALTA** |
| **F-009** | I/O de arquivos e diálogos do SO são acionados diretamente na UI | **P1 (Alta)** | Infrastructure Leak | **ALTA** |
| **F-010** | Contratos de seleção e assets dependem de índices efêmeros `usize` | **P2 (Média)** | Cross-Language Blocker | **ALTA** |
| **F-011** | 44% dos testes automatizados dependem de simulação de eventos egui | **P2 (Média)** | Testing Gap | **ALTA** |
| **F-012** | Cálculo físico de viewport no core consome `Option<egui::Rect>` | **P2 (Média)** | UI-Leak | **ALTA** |
| **F-013** | Despacho de teclado em `app` faz match de strings duplicando regras da UI | **P2 (Média)** | Duplicação de Comportamento | **ALTA** |
| **F-014** | Ausência de pipeline offscreen de geração de miniaturas de assets | **P3 (Baixa)** | Lacuna de Infraestrutura | **MÉDIA** |

---

## 2. Detalhamento dos Achados

### F-001 — `petunia_core` depende diretamente de `egui` e incorpora tipos visuais
* **Severidade**: P0 (Crítica)
* **Confiança**: Alta
* **Categoria**: UI-Leak / Inversão de Dependência

#### Evidências:
* `crates/core/Cargo.toml:13`: `egui = { workspace = true }`
* `crates/core/src/state.rs:46`: `pub texture: Option<egui::TextureHandle>`
* `crates/core/src/state.rs:189`: `pub viewport_rect: Option<egui::Rect>`
* `crates/core/src/state.rs:202`: `pub canvas_tex: Option<egui::TextureHandle>`
* `crates/core/src/module.rs:10`: `fn ui(&mut self, _ctx: &egui::Context, _ui: &mut egui::Ui, ...)`

#### Comportamento Atual:
O núcleo do editor não compila sem que a biblioteca gráfica `egui` esteja presente e compilada na árvore de dependências. Tipos de apresentação como retângulos de tela e handles de textura de GPU são propriedades de primeira classe no estado central do domínio.

#### Por que isso importa:
Impede qualquer execução ou teste headless puro e bloqueia a utilização do `petunia_core` em frontends alternativos (Qt, Slint, C#).

#### Direção Recomendada:
Remover `egui` do `crates/core/Cargo.toml`. Mover handles de textura para o frontend. Substituir `egui::Rect` por tipos geométricos neutros (`[f32; 4]` ou `LogicalRect`).

---

### F-002 — `AppState` atua como God Object agregando 60+ campos heterogêneos
* **Severidade**: P0 (Crítica)
* **Confiança**: Alta
* **Categoria**: God State / Baixa Coesão

#### Evidências:
* `crates/core/src/state.rs:150-259`: Struct monolítica com 60+ campos públicos.

#### Comportamento Atual:
Qualquer parte da aplicação que precise acessar a malha ou a câmera precisa receber `&mut AppState`, que também expõe filtros de texto do Outliner, abas abertas de configurações, contadores de frames e handles de textura.

#### Por que isso importa:
Destrói os limites de domínio e concorrência; impede a testabilidade isolada de subsistemas; impossibilita passar o estado para outras threads com segurança.

#### Direção Recomendada:
Segregar o `AppState` em quatro estruturas coesas com donos específicos: `DomainState` (`Project`), `EditorSession` (`Selection`, `Camera`, `ModalOp`), `UiState` (filtros, abas, modais) e `RenderResources` (texturas, buffers).

---

### F-003 — Ausência de comandos semânticos e dispatcher no ecossistema
* **Severidade**: P0 (Crítica)
* **Confiança**: Alta
* **Categoria**: Command Architecture

#### Evidências:
* `crates/commands/src/lib.rs:8-13`: O crate contém apenas `UndoStack<T: Clone>`.
* Não existem `trait Command`, `CommandId` ou `CommandDispatcher` em nenhum crate.

#### Comportamento Atual:
Ações como deletar geometria, duplicar modelos, aplicar extrusão e alterar materiais são executadas de forma descentralizada por closures anônimas em painéis de UI ou matches de strings no teclado.

#### Por que isso importa:
Impede a automação via CLI, scripts, plugins ou chamadas de frontends alternativos. Não há como invocar uma operação de edição sem simular cliques em widgets ou reimplementar a regra.

#### Direção Recomendada:
Implementar uma camada de aplicação soberana com DTOs de comando puros (`MeshExtrudeCmd`, `DeleteSelectionCmd`) e um `CommandDispatcher` centralizado que gerencie checkpoints de undo automaticamente.

---

### F-004 — A camada de UI executa mutações de domínio e grava undo checkpoints
* **Severidade**: P0 (Crítica)
* **Confiança**: Alta
* **Categoria**: UI Coupling / Regra de Negócio na UI

#### Evidências:
* Mais de 1.000 ocorrências de `state.*` direto espalhadas em 23 arquivos de `crates/ui`.
* `crates/ui/src/properties_panel.rs:45`: `state.checkpoint("delete"); mesh.delete_selected();`
* `crates/ui/src/lib.rs:125-141`: A UI gerencia o carregamento de arquivos e o reset manual de 7 campos do núcleo.

#### Comportamento Atual:
Os botões e menus da interface não apenas capturam intenções do usuário, mas decidem e executam as regras de negócio, alterando diretamente vértices, arestas, coleções e pilhas de histórico.

#### Por que isso importa:
A camada de apresentação está intimamente fundida ao modelo de domínio. Trocar a interface gráfica exige reescrever todas essas regras de manipulação de dados.

#### Direção Recomendada:
Inverter a dependência: os callbacks de UI devem apenas emitir comandos para o `CommandDispatcher` (`dispatcher.dispatch(Command::DeleteSelection)`).

---

### F-005 — Trait `Tool` não executa comportamento e acopla módulos ao egui
* **Severidade**: P1 (Alta)
* **Confiança**: Alta
* **Categoria**: Tool Coupling / Abstração Falha

#### Evidências:
* `crates/module-model/src/lib.rs:46-58`: A trait `Tool` possui apenas metadados e o método `ui(&self, &egui::Context, &mut egui::Ui, &mut AppState)`.

#### Comportamento Atual:
As ferramentas registradas em `ToolRegistry` não executam operações de modelagem no viewport; apenas exibem sliders em painéis da interface egui. A lógica real vive espalhada em `modal.rs` e `modal_viewport.rs`.

#### Por que isso importa:
A arquitetura de plugins/ferramentas é ilusória; adicionar uma ferramenta exige modificar 15 arquivos em 6 crates.

#### Direção Recomendada:
Redesenhar a trait de ferramentas para receber eventos de entrada neutros (`PointerEvent`, `KeyGesture`), gerenciar uma sessão de interação e produzir comandos semânticos de confirmação.

---

### F-006 — Máquinas de estado interativas de ferramentas vivem dentro do egui
* **Severidade**: P1 (Alta)
* **Confiança**: Alta
* **Categoria**: Sessão Confinada na UI

#### Evidências:
* `crates/ui/src/cutting.rs:9-16`: `CutSession` (faca, bissecção e loop cut) vive na memória temporária do egui.
* `crates/ui/src/modal_viewport.rs:8-14`: `PointerSession` gerida via `egui::Id::new("modal.pointer")`.
* `crates/ui/src/annotation.rs` e `measurement.rs`: Desenho e cálculos de régua e lápis acoplados a `egui::Painter`.

#### Comportamento Atual:
As sessões de edição interativa foram construídas como funções de desenho de widgets egui em vez de controladores de sessão do editor.

#### Por que isso importa:
Se a UI for trocada, todas as ferramentas interativas de corte, anotação e medição deixam de existir no projeto.

#### Direção Recomendada:
Extrair as máquinas de estado para `EditorSession` no núcleo/aplicação. A UI deve apenas fornecer as coordenadas do mouse e pintar primitivas de overlay geradas pela sessão.

---

### F-007 — `petunia_config` depende de egui para aplicar temas visuais
* **Severidade**: P1 (Alta)
* **Confiança**: Alta
* **Categoria**: Inversão de Dependência

#### Evidências:
* `crates/config/Cargo.toml:10`: `egui = { workspace = true }`
* `crates/config/src/theme.rs:222`: `pub fn apply(&self, ctx: &egui::Context)`

#### Comportamento Atual:
O crate de configuração carrega arquivos TOML, mas manipula diretamente structs internas de estilo do egui (`egui::Visuals`, `egui::Stroke`, `egui::FontId`).

#### Por que isso importa:
O crate `petunia_config` não pode ser consumido por um frontend em outra biblioteca sem arrastar o egui.

#### Direção Recomendada:
Manter `petunia_config` contendo apenas tokens semânticos neutros (strings hex, floats). Mover o método `apply(ctx)` para um adaptador em `crates/ui`.

---

### F-008 — Crates `module-*` trazem dependência direta de egui em seus manifestos
* **Severidade**: P1 (Alta)
* **Confiança**: Alta
* **Categoria**: Acoplamento de Apresentação

#### Evidências:
* `crates/module-model/Cargo.toml:10`, `module-paint/Cargo.toml:10`, `module-uv/Cargo.toml:10`, `module-assets/Cargo.toml:9`: Todos declaram `egui = { workspace = true }`.

#### Comportamento Atual:
Os módulos de modelagem, pintura, UV e assets não são bibliotecas puras de algoritmo, mas módulos híbridos contendo widgets gráficos egui.

#### Por que isso importa:
Impede a reutilização de funcionalidades de pintura de textura ou unwrapping UV em uma versão do Petunia com outra interface gráfica.

#### Direção Recomendada:
Separar o algoritmo de domínio dos painéis visuais. Módulos devem expor serviços de manipulação de dados; os painéis egui correspondentes devem viver em `crates/ui`.

---

### F-009 — I/O de arquivos e diálogos do SO são acionados diretamente na UI
* **Severidade**: P1 (Alta)
* **Confiança**: Alta
* **Categoria**: Infrastructure Leak

#### Evidências:
* `crates/ui/src/lib.rs:125, 153, 170`: Chamadas diretas a `rfd::FileDialog::new()`.
* `crates/module-paint/src/lib.rs:65, 89`: Chamadas a `rfd` dentro de módulo de pintura.

#### Comportamento Atual:
A abertura de janelas de diálogo do sistema operacional está codificada rigidamente dentro de callbacks de botões e módulos.

#### Por que isso importa:
Impede testes automatizados de carregamento e impede que interfaces headless ou remotas forneçam caminhos de arquivo programaticamente.

#### Direção Recomendada:
Criar uma porta de serviço de arquivos (`ApplicationService::open_project(path)`). O frontend escolhe como obter o caminho (via diálogo nativo, CLI argument ou prompt web) e passa o `Path` ao serviço.

---

### F-010 — Contratos de seleção e assets dependem de índices efêmeros `usize`
* **Severidade**: P2 (Média)
* **Confiança**: Alta
* **Categoria**: Cross-Language Blocker

#### Evidências:
* `crates/core/src/state.rs:205`: `pub export_selected: Vec<usize>`
* Mutações de seleção de assets usando índices de array em `outliner.rs` e `properties_panel.rs`.

#### Comportamento Atual:
Entidades são referenciadas predominantemente pelo seu índice no `Vec<Asset>`, e não pelo seu `Uuid` permanente.

#### Por que isso importa:
A exclusão de um objeto reordena o array e invalida todos os índices em clientes externos (como frontends em C# ou C++), gerando bugs sutis de sincronização.

#### Direção Recomendada:
Usar `AssetId(Uuid)` e `ObjectId(Uuid)` como identificadores canônicos universais em toda a API pública.

---

### F-011 — 44% dos testes automatizados dependem de simulação de eventos egui
* **Severidade**: P2 (Média)
* **Confiança**: Alta
* **Categoria**: Testing Gap

#### Evidências:
* 88 de 198 testes automatizados vivem em `crates/ui` e utilizam `egui::Context::run(...)`.

#### Comportamento Atual:
A maior parte dos testes de integração comportamental testa a lógica de modelagem através da simulação sintética de cliques em widgets egui.

#### Por que isso importa:
A substituição da UI resultará na perda de quase metade da cobertura de testes de regressão do projeto.

#### Direção Recomendada:
Migrar testes comportamentais para a futura camada de sessão headless (`EditorSession`), testando comandos diretamente sem depender do motor de layout da UI.

---

### F-012 — Cálculo físico de viewport no core consome `Option<egui::Rect>`
* **Severidade**: P2 (Média)
* **Confiança**: Alta
* **Categoria**: UI-Leak

#### Evidências:
* `crates/core/src/viewport.rs:11`: `pub fn from_logical(rect: Option<egui::Rect>, ...)`

#### Comportamento Atual:
Uma função matemática de conversão de coordenadas físicas de GPU no núcleo depende do tipo `egui::Rect`.

#### Por que isso importa:
Força o crate de núcleo a depender do egui apenas para ler 4 valores numéricos (`left`, `top`, `right`, `bottom`).

#### Direção Recomendada:
Substituir `Option<egui::Rect>` por `Option<[f32; 4]>` ou struct neutra `LogicalRect`.

---

### F-013 — Despacho de teclado em `app` faz match de strings duplicando regras da UI
* **Severidade**: P2 (Média)
* **Confiança**: Alta
* **Categoria**: Duplicação de Comportamento

#### Evidências:
* `crates/app/src/lib.rs:333-437`: Match de 30 strings de ação (`"model.extrude"`, `"model.delete"`).

#### Comportamento Atual:
O despachante de teclado converte teclas em strings e depois em mutações manuais, coexistindo com os botões da UI que realizam as mesmas mutações de forma independente.

#### Por que isso importa:
Manutenção duplicada e risco constante de divergência de comportamento entre atalhos de teclado e botões de tela.

#### Direção Recomendada:
Unificar o despacho de atalhos e cliques de tela no mesmo `CommandDispatcher`.

---

### F-014 — Ausência de pipeline offscreen de geração de miniaturas de assets
* **Severidade**: P3 (Baixa)
* **Confiança**: Média
* **Categoria**: Lacuna de Infraestrutura

#### Evidências:
* `crates/ui/src/asset_library_drawer.rs:170-179`: Miniaturas de assets são apenas chips quadrados de cor sólida.

#### Comportamento Atual:
O editor não gera pré-visualizações renderizadas em 3D dos modelos armazenados na biblioteca interna do projeto.

#### Por que isso importa:
Dificulta a criação de navegadores de assets profissionais em qualquer frontend.

#### Direção Recomendada:
Aproveitar a capacidade offscreen de `petunia_render_wgpu` para gerar texturas de miniatura sob demanda a partir do `Project`.
