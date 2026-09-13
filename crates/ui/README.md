# Crate `petunia_ui` (`crates/ui/`)

Camada de apresentação e interface gráfica construída com `egui`:
- Barra de cabeçalho superior com seleção de workspaces por pílulas (MODEL, PAINT, UV, EXPORT).
- Motor de iconografia vetorial procedural canônica (`icons.rs`, `icon_registry.rs`) e eliminação de emojis.
- Barra de viewport organizada em 7 clusters funcionais responsivos (`viewport_bar.rs`).
- Menus padronizados (`PetuniaMenuItem`) com layout profissional e atalhos dinâmicos.
- Orquestração de painéis laterais de ferramentas através do registro dinâmico `ModuleRegistry`.
- Área central de viewport 3D interativo com suporte a eventos de ponteiro, gestos e desenho de overlays de controle.


## Interação direta

`modal_viewport` traduz mouse/teclado em prévias absolutas de domínio e desenha HUD.
`gizmo` projeta e testa eixos, planos, anéis e escala. `viewport_interaction` arbitra
picking/hover, navegação e pintura; `cutting` implementa estágios de loop/knife/slice.
Testes egui nos módulos `*_tests` injetam eventos reais e verificam geometria e undo.
Consulte [o manual](../../docs/manual/usage.md) para atalhos e limitações efetivos.
