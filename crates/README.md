# Crates da Workspace — Petunia3D (`crates/`)

Estrutura modular de 14 crates em Rust que compõem o motor e a aplicação Petunia3D, estruturados em grafo acíclico de dependências.

## Inventário de Crates

| Crate | Responsabilidade Principal | Dependências Internas |
|-------|-----------------------------|------------------------|
| [`mesh`](mesh/README.md) | Geometria de malha pura, primitivas, subdivisão, extrusão, triangulação e parser OBJ. | Nenhuma (domínio puro). |
| [`commands`](commands/README.md) | Histórico de comandos com snapshots para suporte completo a Desfazer e Refazer (Undo/Redo). | `mesh` |
| [`config`](config/README.md) | Internacionalização (i18n), atalhos de teclado (`petunia.toml`), temas visuais e ferramentas. | `mesh` |
| [`project`](project/README.md) | Gestão de documento de projeto `.petunia`, UUIDs de assets, exportação OBJ e glTF 2.0 (GLB). | `mesh` |
| [`core`](core/README.md) | `AppState` canônico, câmera 3D, referências ortográficas, barramento de eventos e trait `Module`. | `mesh`, `commands`, `config`, `project` |
| [`module-model`](module-model/README.md) | 12 ferramentas de modelagem 3D (Draw Profile, Extrude, Push/Pull, Inset, Bevel, Subdivide, Mirror, Merge). | `core` |
| [`module-paint`](module-paint/README.md) | Ferramentas de pintura em vértices (Vertex Color) e canvas albedo 2D. | `core` |
| [`module-uv`](module-uv/README.md) | Editor e visualizador de projeções UV sincronizado com a seleção 3D. | `core` |
| [`module-assets`](module-assets/README.md) | Gerenciador e biblioteca de múltiplos assets tridimensionais no projeto. | `core` |
| [`render`](render/README.md) | Matemática de iluminação compartilhada, grid infinito, quad de referência e tipos de renderização. | `mesh`, `core` |
| [`render-gl`](render-gl/README.md) | Backend OpenGL 3.3 Core Profile puro via `glow` e bootstrap de janela com `glutin`. | `render`, `core` |
| [`render-wgpu`](render-wgpu/README.md) | Backend moderno acelerado via `wgpu` com detecção de adaptadores e shaders WGSL/SPIR-V. | `render`, `core` |
| [`ui`](ui/README.md) | Layout da interface gráfica em modo imediato (`egui`), cabeçalho de pílulas, viewport e `ModuleRegistry`. | `core`, `module-*` |
| [`app`](app/README.md) | Ponto de entrada do loop de aplicação, render-on-demand e orquestração de backends. | `core`, `ui`, `render-gl`, `render-wgpu` |
