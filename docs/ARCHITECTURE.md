# Petunia3D — Arquitetura

## Grafo de crates (acíclico, verificado)

```text
                    ┌──────────┐
                    │  mesh    │  malha pura (sem UI, sem GL)
                    └────┬─────┘
                         │
┌────────┐  ┌────────────┼────────────┐
│commands│  │   config   │  project   │  fundações (sem core)
│ undo   │  │i18n/keys/  │ Asset/Project
└───┬────┘  │theme/tools │ .petunia/GLB
    │       └─────┬──────┘     │
    │             │            │
    └──────┬──────┴──────┬─────┘
           │    core     │  estado, câmera, eventos, Module
           └──────┬──────┘
                  │
     ┌────────────┼────────────────┬──────────────┐
     │            │                │              │
module-model module-paint      module-uv   module-assets
(tools)      (paint)            (uv)        (assets)
     │            │                │              │
     └────────────┼────────────────┴──────────────┘
                  │
        ┌─────────┴─────────┐
        │        ui         │  layout (chama módulos via registry)
        └─────────┬─────────┘
                  │
        ┌─────────┴─────────┐
        │        app        │  Core, backends, loop on-demand
        └──────────────────┘
                  │
        render ← render-gl / render-wgpu (só eles tocam GPU)
```

Regras:

- módulos conhecem `core`, nunca uns aos outros;
- `render-gl` é o único lugar com OpenGL/glutin (bootstrap incluído);
- `ui` chama módulos via `ModuleRegistry` (`dyn Module`), não tipos concretos
  (exceção: `ToolRegistry`, que é o contrato das ferramentas de MODEL);
- `core::AppState` centralizado é decisão deliberada (egui immediate-mode);
- input externo (OBJ, `.petunia`, imagens, TOML) retorna `Result` e é
  validado no trust boundary (`Mesh::validate`, `Project::validate`,
  `Canvas::validate`, export retorna `ExportError`).

## Eventos

```text
operação → checkpoint → mutação → sync_selection/emit → dispatch → módulos
```

`MeshChanged`, `SelectionChanged`, `ToolActivated`, `ActiveAssetChanged`,
`TextureChanged`, `ProjectLoaded`. UV sincroniza seleção via
`SelectionChanged`; assets via `ActiveAssetChanged`.

## Render-on-demand

`ControlFlow::Wait` + flag `dirty`: redesenha em eventos (input, resize,
hover), mutações e `consume_dirty` no `about_to_wait`. Idle = 0 frames.
`SIMPLE3D_SPIN=1` força loop contínuo (benchmark).


## Interações transacionais do viewport

`core::modal` possui snapshot do projeto e malha fonte. Cada preview deriva da
fonte; apenas commit grava undo. `core::mesh_preview` aplica a mesma fronteira às
ferramentas com vários estágios (loop, knife, slice). Pintura guarda um snapshot
por traço. O formato persistido não inclui sessões de interação.

`ui::{modal_viewport,viewport_interaction,cutting,gizmo}` traduz eventos egui,
arbitra ferramentas e desenha overlays. `app` roteia atalhos configuráveis e
bloqueia comandos concorrentes; `state.is_interacting()` desativa os painéis.
`core::picking` usa triangulação e projeção coerente com a câmera para hover/pick.
`mesh::{loop_cut,knife,bevel}` contém algoritmos sem dependência de UI.

O manifesto contém **14 crates internos**, mais o pacote executável raiz;
a referência anterior a 13 crates estava desatualizada. Não foram adicionados
novos crates nem dependências externas nesta rodada.
