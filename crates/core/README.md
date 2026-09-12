# Crate `petunia_core` (`crates/core/`)

Núcleo central de estado e contratos do Petunia3D:
- `AppState`: Detentor canônico do estado de execução em tempo real (malha ativa, seleções, ferramentas, câmera 3D).
- Câmera 3D: Controle de projeção em perspectiva e ortográfica, órbita, pan e zoom.
- Barramento de Eventos: Emissão e despacho de mutações (`MeshChanged`, `SelectionChanged`, `ToolActivated`).
- Contrato `dyn Module`: Interface padronizada que permite desacoplar completamente os módulos funcionais da apresentação.


## Sessões de edição do viewport

`modal` guarda as prévias de transformação e ferramentas; `mesh_preview` controla
cortes em vários estágios; `state` agrupa pintura por traço. Sessões se excluem
mutuamente e não entram no formato persistido. `picking` resolve componentes em
pixels com oclusão na malha, e `loop_cut` adapta o algoritmo puro do crate mesh.
Regressões cobrem estado, UV, seleção e histórico após cancel/commit/undo.
