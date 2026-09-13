# Crate `petunia_ui` (`crates/ui/`)

Camada de apresentação e interface gráfica construída com `egui`:
- Barra de cabeçalho superior com seleção de workspaces por pílulas (MODEL, PAINT, UV, EXPORT).
- Motor de iconografia vetorial procedural canônica (`icons.rs`, `icon_registry.rs`) e eliminação de emojis.
- Barra de viewport organizada em 7 clusters funcionais responsivos (`viewport_bar.rs`).
- Menus padronizados (`PetuniaMenuItem`) com layout profissional e atalhos dinâmicos.
- Orquestração de painéis laterais de ferramentas através do registro dinâmico `ModuleRegistry`.
- Área central de viewport 3D interativo com suporte a eventos de ponteiro, gestos e desenho de overlays de controle.


## Interação direta e Camada Apresentacional
 
`modal_viewport` traduz mouse/teclado para a máquina de estados `PointerSession` e transações modais do domínio, desenhando o HUD e guias.
`gizmo` projeta e testa eixos, planos, anéis e escala. `viewport_interaction` arbitra picking/hover, navegação e pintura.
`cutting` delega o ciclo de corte e deslizamento diretamente para a máquina de estados `CutSession` em `AppState`.
`file_dialog_service` centraliza os diálogos nativos/in-canvas de arquivos e os encaminha ao `ProjectService`.
Testes egui nos módulos `*_tests` injetam eventos reais e verificam geometria, sessões e undo.
Consulte [o manual](../../docs/manual/usage.md) para atalhos e limitações efetivos.
