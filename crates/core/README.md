# Crate `petunia_core` (`crates/core/`)

Núcleo central de estado, serviços de aplicação e contratos do Petunia3D (100% puro e agnóstico de UI):
- `AppState`: Detentor canônico do estado de execução em tempo real (malha ativa, seleções, ferramentas, câmera 3D, sessões ativas).
- `CommandDispatcher` & `Command`: Despachante transacional de comandos com auto-checkpointing de Undo/Redo, catálogo de metadados (`CommandMetadata`), validação contextual (`can_execute`) e busca para a Command Palette (`CommandPaletteItem`).
- `DocsTopic`: Resolução semântica centralizada de links e tópicos de documentação oficial (P3D-115).
- `ProjectService`: Serviço puro de aplicação para ciclo de vida de projeto, salvamento/carregamento (.petunia), importação/exportação (OBJ, GLB, paletas) e imagens de referência.
- `CutSession` (`cutting_session.rs`): Máquina de estado pura para ferramentas interativas de corte (`Knife`, `Slice` e `Loop Cut`).
- `PointerSession` (`modal.rs`): Controlador neutro de interação por ponteiro e entrada numérica para operações modais (G/R/S).
- `SelectionDomain` & `selection.rs`: Domínio unificado de interação (`Object`, `Vertex`, `Edge`, `Face`) e rastreamento canônico de seleção de arestas.
- `SnapSettings` & `snap.rs`: Motor matemático headless de snapping magnético com atração determinística a grade, incrementos, vértices, arestas e faces.
- `ProportionalSettings` & `proportional.rs`: Motor headless de edição proporcional com curvas de atenuação (Smooth, Linear, Sphere, Sharp, Constant).
- `ToolFeedback` & `modal_feedback.rs`: Sistema agnóstico de apresentação e telemetria para operações modais (linhas-guia, deltas e estado de snap).
- `LogicalRect` & `viewport.rs`: Geometria neutra de viewport com funções canônicas de projeção de tela e desprojeção.
- Câmera 3D: Controle de projeção em perspectiva, ortográfica e presets isométricos, órbita, pan e zoom.
- Barramento de Eventos: Emissão e despacho de mutações (`MeshChanged`, `SelectionChanged`, `ToolActivated`).
- Contrato `dyn Module`: Interface padronizada que permite desacoplar completamente os módulos funcionais da apresentação.

## Sessões de edição do viewport

`modal` guarda as prévias de transformação e ferramentas; `mesh_preview` e `CutSession` controlam
cortes em vários estágios; `state` agrupa pintura por traço. Sessões se excluem
mutuamente e não entram no formato persistido. `picking` resolve componentes em
pixels com oclusão na malha, e `loop_cut` adapta o algoritmo puro do crate mesh.
Regressões cobrem estado, UV, seleção e histórico após cancel/commit/undo.
