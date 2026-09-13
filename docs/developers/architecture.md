# Macroarquitetura de Crates

O Petunia3D é construído sob uma arquitetura modular estrita em Cargo Workspace composta por 14 crates altamente coesos e desacoplados, garantindo limites explícitos de domínio:

```mermaid
graph TD
    App["petunia_app (Runtime / Windowing / Winit)"]
    UI["petunia_ui (egui Presentation Layer)"]
    RenderWGPU["petunia_render_wgpu (WebGPU Pipeline)"]
    RenderGL["petunia_render_gl (OpenGL Glow Pipeline)"]
    RenderCommon["petunia_render (Abstrações de Renderização)"]
    ModuleModel["petunia_module_model (Modelagem)"]
    ModulePaint["petunia_module_paint (Pintura)"]
    ModuleUV["petunia_module_uv (Unwrapping)"]
    ModuleAssets["petunia_module_assets (Biblioteca de Assets)"]
    Commands["petunia_commands (Command Pattern / Undo)"]
    Core["petunia_core (AppState / Câmera / Eventos)"]
    Project["petunia_project (Estruturas Persistidas / IO)"]
    Mesh["petunia_mesh (Geometria Half-Edge / Topologia)"]
    Config["petunia_config (Temas / Atalhos / i18n)"]

    App --> UI
    UI --> Core
    UI --> Config
    UI --> ModuleModel
    UI --> ModulePaint
    UI --> ModuleUV
    UI --> ModuleAssets
    ModuleModel --> Commands
    Commands --> Core
    Core --> Project
    Project --> Mesh
    RenderWGPU --> RenderCommon
    RenderGL --> RenderCommon
    RenderCommon --> Core
    App --> RenderWGPU
    App --> RenderGL
```

## Descrição dos Crates

| Crate | Responsabilidade |
| :--- | :--- |
| `crates/mesh` | Estruturas de dados geométricas fundamentais, representação de vértices, arestas e faces poligonais. |
| `crates/project` | Formato persistente de projeto (`.petunia`), gerenciador de coleções, anotações e medidas. |
| `crates/core` | O coração de estado em tempo de execução: `AppState`, matemática de câmera orbital, barramento de eventos e operações modais. |
| `crates/commands` | Pilha transacional de comandos atômicos (`execute`, `undo`, `redo`). |
| `crates/config` | Carregador de temas visuais, keymaps configuráveis e dicionários de tradução TOML. |
| `crates/render` | Traits e contratos agnósticos para os renderizadores gráficos. |
| `crates/render-wgpu` | Backend gráfico primário construído com `wgpu` e shaders WGSL modernos. |
| `crates/render-gl` | Backend de compatibilidade universal construído com `glow` (OpenGL ES 3.0 / Desktop OpenGL 3.3). |
| `crates/ui` | Toda a interface gráfica, layout de painéis, gizmos 3D, ícones vetoriais e controles com `egui`. |
| `crates/app` | Orquestração da janela desktop, loop de eventos do `winit` e inicialização de contexto gráfico. |
