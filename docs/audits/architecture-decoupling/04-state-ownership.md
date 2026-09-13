# 04 — Posse e Segregação de Estado (State Ownership Report)

> **Anatomia do God Object `AppState`, análise de ciclo de vida e mapeamento da segregação necessária entre Domínio, Aplicação e Apresentação.**

---

## 1. Anatomia do `AppState` (`crates/core/src/state.rs`)

O `AppState` é a estrutura central de dados do Petunia3D. Composta por **mais de 60 campos públicos**, ela acumula responsabilidades de cinco camadas arquiteturais distintas que deveriam possuir donos e ciclos de vida independentes:

```text
AppState (crates/core/src/state.rs:150-259)
│
├── [1. DOMÍNIO E PERSISTÊNCIA] (Vida longa, serializável em disco)
│   ├── project: Project (assets, malhas, anotações, medições, paleta)
│   └── undo: UndoStack<Project> (pilha de checkpoints de snapshots)
│
├── [2. SESSÃO DO EDITOR] (Sessão de trabalho, independe de widgets visuais)
│   ├── selection: Selection (vértices, arestas e faces selecionados)
│   ├── mode: EditMode (Object, Edit, TexturePaint)
│   ├── workspace: Workspace (Model, Paint, UV, Animate, Export)
│   ├── select_mode: SelectMode (Vertex, Edge, Face)
│   ├── camera: Camera (posição, rotação, zoom, projeção)
│   ├── camera_frame: Option<(Camera, Camera, f32)> (interpolação de enquadramento)
│   ├── cursor_3d: [f32; 3] (coordenada espacial do cursor de inserção)
│   ├── modal: Option<ModalOp> (operação modal ativa)
│   ├── pending_modal: Option<ModalKind> (intenção de operação modal)
│   ├── locked_axes: [bool; 3] (travamento de eixos X, Y, Z)
│   ├── isolate_active: bool (modo de isolamento de seleção)
│   └── dirty: bool (flag de redesenho da cena)
│
├── [3. PARÂMETROS VOLÁTEIS DE FERRAMENTAS] (Pertencem ao contexto da ferramenta)
│   ├── active_tool: String (identificador da ferramenta ativa)
│   ├── extrude_dist: f32 (distância da última extrusão)
│   ├── inset_factor: f32 (fator de inset)
│   ├── bevel_amount: f32 (largura do chanfro)
│   ├── mirror_axis: usize & mirror_weld: f32 (parâmetros de espelhamento)
│   ├── push_dist: f32 (deslocamento normal de push/pull)
│   ├── transform_delta: [f32; 3] & transform_rotation: [f32; 3] & transform_scale: f32
│   ├── paint_color: [f32; 3], paint_radius: f32, paint_strength: f32
│   ├── profile: ProfileState (pontos do traçado de perfil)
│   ├── active_measurement: Option<MeasurementItem> (medição em arrasto)
│   └── active_annotation: Option<AnnotationStroke> (traço de anotação em desenho)
│
├── [4. ESTADO VOLÁTIL DE APRESENTAÇÃO / WIDGETS] (Pertencem estritamente à UI)
│   ├── viewport_rect: Option<egui::Rect> (retângulo de pixels da área de visualização)
│   ├── viewport_pixels_per_point: f32 (fator de escala HiDPI)
│   ├── outliner_search: String (filtro de busca digitado na árvore)
│   ├── properties_tab: String (aba ativa no painel de propriedades)
│   ├── show_settings: bool & settings_tab: String (modal e aba de configurações)
│   ├── show_asset_library: bool (gaveta de assets aberta/fechada)
│   ├── show_asset_browser: bool (painel retrátil aberto/fechado)
│   ├── show_help: bool & show_perf: bool (modais de ajuda e telemetria)
│   ├── active_theme_id: String (tema visual selecionado)
│   ├── active_icon_pack_id: String (pacote de ícones selecionado)
│   ├── active_keymap_id: String (perfil de teclado selecionado)
│   ├── context_menu_pos: Option<[f32; 2]> (coordenada de abertura do menu RMB)
│   └── timeline_frame: i32, timeline_start: i32, timeline_end: i32, timeline_playing: bool
│
└── [5. INFRAESTRUTURA E HANDLES DE GPU] (Pertencem ao subsistema de renderização)
    ├── canvas_tex: Option<egui::TextureHandle> (textura alocada no backend egui)
    ├── canvas_dirty: bool (sinalizador de reenvio de textura)
    ├── backend_name: String (rótulo informativo da GPU)
    └── stats: RenderStats (contagem de triângulos, draws e tempo de quadro)
```

---

## 2. Mapeamento de Posse, Leitura e Mutação

| Entidade | Quem Cria? | Quem Possui? | Quem Lê? | Quem Modifica? |
| :--- | :--- | :--- | :--- | :--- |
| **`Project`** | `AppState::new` | `AppState` | Core, UI, Renderers | Core, Módulos, Widgets UI |
| **`UndoStack`** | `AppState::new` | `AppState` | UI (status), Core | Widgets UI diretamente (`checkpoint`) |
| **`Camera`** | `AppState::new` | `AppState` | Renderers, UI (view) | Winit, UI (controles de câmera), Core |
| **`ModalOp`** | `AppState::begin_modal`| `AppState` | UI (`modal_viewport`) | UI (`modal_viewport`), Core |
| **Campos de Widgets** | `AppState::new` | `AppState` | Widgets UI | Widgets UI diretamente |
| **Handles de GPU** | `AppState::new` | `AppState` | UI, Renderers | Widgets UI, App runner |

### Problemas Decorrentes da Estrutura Atual:
1. **Falta de Encapsulamento**: Todos os campos são `pub`. Qualquer linha de qualquer crate que possua `&mut AppState` pode alterar arbitrariamente a geometria, a seleção, a ferramenta ativa ou o tema, sem passar por validações de invariantes de domínio.
2. **Impossibilidade de Concorrência**: Como todo o estado está reunido em uma única struct monolítica, é impossível processar operações de malha em uma thread de background enquanto a UI responde na main thread, pois `&mut AppState` exige empréstimo exclusivo do universo inteiro.
3. **Invalidação Granular Inexistente**: O sistema conta apenas com duas flags genéricas (`dirty: bool` e `events: EventBus`). Uma alteração no texto de busca do Outliner (`outliner_search`) compartilha a mesma struct que uma alteração topológica destrutiva na malha.

---

## 3. Avaliação Positiva: Ausência de Estado Global Escondido

Apesar da concentração excessiva no `AppState`, **não foram identificados singletons estáticos, `static mut`, nem variáveis globais escondidas no domínio**.

O fato de todo o estado estar explicitamente enraizado em instâncias passadas por referência (`&mut AppState`) facilita enormemente a futura segregação. Para transformar o Petunia3D em uma arquitetura limpa, não será necessário desmontar uma teia de variáveis globais ocultas, mas sim decompor a struct monolítica em quatro contratos explícitos:
* `DomainState` (`Project`);
* `EditorSession` (`Selection`, `Camera`, `ModalOp`, `ToolContext`);
* `UiState` (`SearchQuery`, `ActiveTab`, `PanelWidths`, `OpenModals`);
* `RenderResources` (`TextureHandles`, `GpuBuffers`).
