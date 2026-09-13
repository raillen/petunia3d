# Current Project State

- Project: **Petunia3D**
- Prumo: **0.5.1**
- Current phase: **P01 — Premium viewport interaction**
- Canonical UI Golden Reference: [`docs/image-references/Blender.svg`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/image-references/Blender.svg) (component catalog in [`docs/image-references/extracted/`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/image-references/extracted/))
- Historical goal: **P01-G01**, recorded DONE before the premium specification.
- Current implementation and acceptance map: [premium interaction plan](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/development/premium-interaction-plan.md).
- Context methodology: **Lean Progressive Context (LPC)**
- Last updated: `2026-09-12`

## Current status

The premium audit supersedes the previous claim that all required operations were
complete. The visual target for final production is formalised as the 1920×1080
layout of [`Blender.svg`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/image-references/Blender.svg),
with 268 component SVGs cataloged and verified in [`docs/image-references/extracted/index.html`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/image-references/extracted/index.html).

The first implementation round delivers transactional modal transforms,
extrusion/inset/bevel previews, gizmos, component hover, quad loop cut, segmented
knife input, planar slicing, atomic paint strokes and corrected camera navigation.
The second implementation round delivers the native vector icon engine ([`crates/ui/src/icons.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/icons.rs)),
accessible `tool_button` with compact/expanded layouts, explicit orthographic camera
controls with 6 orthogonal presets, numeric tool property fields ([`crates/ui/src/tool_fields.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/tool_fields.rs)),
and responsive panel refactoring.
The third implementation round (Gauntlet Loop R2) delivers the interactive Navigation
Orientation Gizmo ([`crates/ui/src/nav_gizmo.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/nav_gizmo.rs)) with 6 depth-sorted world axes,
zoom/pan/perspective action handles, 3D Cursor overlay with `Shift+RMB` placement,
contextual RMB menus for sub-elements and objects, and direct Viewport Shading mode selectors.
The fourth implementation round delivers the Canonical Desktop UI Architecture matching [`Blender.svg`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/image-references/Blender.svg):
- Design tokens centralizados ([`crates/ui/src/tokens.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/tokens.rs))
- Gerenciador de ícones rasterizados embutidos com tingimento reativo ([`crates/ui/src/app_icons.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/app_icons.rs)) consumindo os PNGs reais do Figma em [`assets/ui/icons/toolbar/`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/assets/ui/icons/toolbar/)
- Macroestrutura de 6 níveis componentizada: Main Header ([`crates/ui/src/main_header.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/main_header.rs)), 3D View Bar ([`crates/ui/src/viewport_bar.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/viewport_bar.rs)), Toolbar 40px ([`crates/ui/src/toolbar.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/toolbar.rs)), Outliner hierárquico ([`crates/ui/src/outliner.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/outliner.rs)), Painel de Propriedades modular com abas ([`crates/ui/src/properties_panel.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/properties_panel.rs)), Timeline de animação ([`crates/ui/src/timeline.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/timeline.rs)) e Status Bar ([`crates/ui/src/status_bar.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/status_bar.rs)).
The fifth implementation round delivers modern egui ecosystem integration and specialized crates under strict Petunia Design System sovereignty:
- Registro centralizado de ícones (`IconRegistry`, `PetuniaIcon`) em [`crates/ui/src/icon_registry.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/icon_registry.rs) consumindo 9 PNGs de toolbar e 15 PNGs de abas de properties extraídos do Figma ([`assets/ui/icons/properties/`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/assets/ui/icons/properties/)), com decodificação alfa e tingimento dinâmico.
- Componentes soberanos de UI (`PetuniaToolbarButton`, `PetuniaPropertyTabButton`, `PetuniaWorkspacePill`, `PetuniaSearchBox`) em [`crates/ui/src/widgets.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/widgets.rs).
- Integração bidirecional de gizmos 3D via `transform-gizmo-egui` em [`crates/ui/src/transform_gizmo_integration.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/transform_gizmo_integration.rs).
- Hierarquia de cena moderna com `egui_ltreeview` em [`crates/ui/src/outliner.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/outliner.rs) com busca instantânea e toggles de visibilidade.
- Sistema de docking e layout de múltiplos painéis com `egui_tiles` e `PetuniaTilesBehavior` em [`crates/ui/src/tiles_workspace.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/tiles_workspace.rs).
- Diálogos de arquivos de sistema multiplataforma com `egui-file-dialog` em [`crates/ui/src/file_dialog_service.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/file_dialog_service.rs).
- Suíte de testes UI headless com `egui_kittest` em [`crates/ui/tests/kittest_ui_flows.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/tests/kittest_ui_flows.rs).
The sixth implementation round delivers UI consolidation, dead controls cleanup, Outliner redesign, and Blender-authentic high-visibility vector icons:
- Novo sistema vetorial de ícones na toolbar ([`crates/ui/src/icons.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/icons.rs), [`crates/ui/src/icon_registry.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/icon_registry.rs)) com traço reforçado (2.0px–2.6px) e paleta multicolorida autêntica do Blender (3D Cursor com anel vermelho e segmentos brancos, eixos cardeais RGB para Move/Scale/Rotate/Transform, lápis ciano com madeira dourada para Annotate, régua amarela com miras para Measure, cubo isométrico laranja com badge '+' para Add Primitive, e realces em amarelo/laranja vivo para ferramentas de modelagem).
- Redesenho completo do Outliner ([`crates/ui/src/outliner.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/outliner.rs)): árvore reativa aos assets reais do projeto (`project.assets`), toggle funcional de visibilidade `👁` sincronizado com pipelines de renderização WebGPU e OpenGL, menu de contexto (RMB) para duplicar/deletar e Galeria Rápida de Primitivas instanciadas na coordenada exata do 3D Cursor.
- Eliminação de botões sem funcionalidade: remoção dos botões mortos de abas de properties (`Render Engine`, `Output`, `View Layer`, `Scene`, `World`, `Collection`), pills estáticos de cena no header, menu fictício `Render` e painel de Timeline.
- Reorganização da Viewport Bar em 5 clusters semânticos com 4 botões esféricos de sombreamento (`○ Wire`, `● Solid`, `◐ Material`, `☼ Render`).
- Correção de atalhos e seleção: `Tab` e teclas numéricas `0..=4` com propagação sem perda no winit/egui e unificação para 4 alvos de seleção (`Objeto [Tab/0]`, `Vértice [1]`, `Aresta [2]`, `Face [3]`).
The seventh implementation round delivers interactive 3D measurement and grease pencil annotation, floating viewport bar refinement, dedicated asset library drawer, dynamic theme and icon pack systems, 8 canonical keymap profiles, and TOML i18n:
- Ferramenta interativa de régua e medição 3D ([`crates/ui/src/measurement.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/measurement.rs)): snapping magnético a vértices de malhas ativas, graduações métricas, e badge flutuante com distância euclidiana e deltas cartesianos ($\Delta X, \Delta Y, \Delta Z$).
- Ferramenta interativa de anotação e rascunho 3D ([`crates/ui/src/annotation.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/annotation.rs)): rascunho grease-pencil com projeção em superfícies de malhas ativas e plano do 3D Cursor.
- Barra da viewport aperfeiçoada ([`crates/ui/src/viewport_bar.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/viewport_bar.rs)): botão explícito `[➕ Add+ ▾]`, esferas flutuantes compactas de shading estilo Blender (`○`, `●`, `◐`, `☼`) e botões de seleção otimizados sem sobreposição.
- Gaveta dedicada da biblioteca de assets ([`crates/ui/src/asset_library_drawer.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/asset_library_drawer.rs)): gerenciamento de modelos, pré-visualizações, métricas de polígonos e instanciação no 3D Cursor, com clara distinção entre salvar asset (no projeto) e salvar o projeto (.petunia).
- Sistema de temas orientado a tokens ([`crates/config/src/theme.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/config/src/theme.rs), [`crates/ui/src/tokens.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/tokens.rs)): 4 temas nativos (`petunia-dark`, `petunia-light`, `petunia-capuccino`, `petunia-tokyo-nights`) e carregamento via TOML com fallback seguro.
- Sistema de pacotes de ícones ([`crates/ui/src/icon_registry.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/icon_registry.rs)): 5 pacotes (`Petunia`, `Phosphor`, `Tabler`, `Iconoir`, `Lucide`) com cascading fallback e espessura refinada para 1.8–1.9px.
- 8 perfis canônicos de teclado ([`crates/config/src/keybinds.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/config/src/keybinds.rs)): mapeamentos completos para Blender, Maya, 3ds Max, Cinema 4D, Notebooks e detecção automática de conflitos.
- Modal de configurações centralizado ([`crates/ui/src/settings_modal.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/settings_modal.rs)) e internacionalização em TOML (`pt-BR`, `en`).
The eighth implementation round delivers the UI Reorganization, Hierarchy Consolidation & Ergonomics Refinement:
- Contextual Modeling Shelf ([`crates/ui/src/contextual_shelf.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/contextual_shelf.rs)): cápsula flutuante na base da viewport reagindo ao modo ativo (`Model+Edit`, `Model+Object`, `Paint`, `UV`, `Animate`) com escudo de eventos de mouse para prevenir interferência no raycasting tridimensional.
- Retractable Asset Browser ([`crates/ui/src/asset_browser.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/asset_browser.rs)): painel retrátil à esquerda (220–340px) com filtro por categoria, busca padronizada com `PetuniaSearchBox`, cartões de modelo e salvamento direto na biblioteca interna.
- Viewport Bar em 6 clusters semânticos ([`crates/ui/src/viewport_bar.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/viewport_bar.rs)): dropdown de modo (`Object` vs `Edit`), alvos de seleção (`⬝ Vértice`, `╱ Aresta`, `▨ Face`) exibidos **exclusivamente** no modo de edição, menus com `➕ Add+ ▾` e menus contextuais de malha/objeto, transform/pivot, snapping/proportional editing, overlays/x-ray e esferas de shading.
The ninth implementation round delivers UI Enhancements & Interactions (Vertex Hover Demarcation, Toolbar Edit Tools, WGPU X-Ray, Reference Images, Outliner Collections/Lock/Isolate, and Vibrant Properties Tabs):
- Demarcação visual de vértices em modo de edição com hover dourado e anel halo ciano brilhante (`#64dcff`) em [`crates/ui/src/viewport_interaction.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/viewport_interaction.rs).
- Expansão dinâmica da toolbar esquerda ao entrar em Edit Mode com as 9 ferramentas de modelagem de malha em [`crates/ui/src/toolbar.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/toolbar.rs).
- Pipeline WebGPU e picking X-Ray com translucidez da malha (`alpha ~ 0.45`) e arestas sem oclusão de profundidade em [`crates/render-wgpu/src/lib.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/render-wgpu/src/lib.rs) e atalho `Alt+Z`.
- Reintegração de imagens de referência no menu `➕ Add+ ▾`, na contextual shelf (`🖼 Referência`) e no Outliner em [`crates/ui/src/outliner.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/outliner.rs).
- Suporte a pastas/coleções (`📁 Coleções`), bloqueio (`🔒 Lock`) e isolamento (`⌖ Isolar` / `Numpad /`) no Outliner e no motor de transformação modal.
- Abas de propriedades ampliadas para `32x28px` com container emoldurado e cores semânticas vibrantes do Blender (Tool `#3169e3`, Object `#e67e22`, Modifiers `#00a8ff`, Data `#2ecc71`, Material `#e84393`) em [`crates/ui/src/properties_panel.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/properties_panel.rs).
- Descongestionamento da sidebar direita: redução para estritamente `Outliner` e `Properties`, com inspector de transform em grid triaxial RGB e cor de material unificada com paleta do projeto.
- Especialização da toolbar vertical esquerda: restrita às 8 ferramentas primárias de interação contínua.
- Barra de status inferior reorganizada em 3 blocos: identidade de projeto (`● Salvo` / `○ Não salvo`) e dicas à esquerda, mensagens no centro, e telemetria de cena agregada (`Tris`, `Verts`, `Objs`, `ms`) à direita.
The tenth implementation round delivers complete Annotations & Measurements Architecture with Undo/Redo (Ctrl+Z), Dedicated Outliner Collections, Subgrouping, Strict Confinement, and Transform Properties:
- Migração de anotações e medidas para o domínio de dados persistente [`petunia_project::Project`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/project/src/lib.rs), eliminando o descarte no histórico e garantindo suporte total a Undo/Redo (`Ctrl+Z` e `Ctrl+Shift+Z`) com checkpoints transacionais.
- Coleção dedicada `📝 Anotações` no topo do Outliner ([`crates/ui/src/outliner.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/outliner.rs)) com cor ciano (`#00d2d3`), alternância coletiva e individual de visibilidade (`👁`) e bloqueio (`🔒`), criação de subgrupos internos (`📁 Subgrupo`) e confinamento estrito impedindo que anotações sejam movidas para coleções de malhas 3D.
- Coleção dedicada `📏 Medidas` no topo do Outliner ([`crates/ui/src/outliner.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/outliner.rs)) com cor amarela (`#feca57`) e controles estritamente limitados a visibilidade e exclusão.
- Inspetor de propriedades e transformações de anotações no Painel de Propriedades ([`crates/ui/src/properties_panel.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/properties_panel.rs)): campos de nome, visibilidade, bloqueio, subgrupo, aparência de traço (cor e espessura), e seção completa de Transformação (Location X/Y/Z, Rotation X/Y/Z em graus, Scale X/Y/Z e redefinição).
- Manipulação tridimensional por Gizmos no Viewport ([`crates/ui/src/viewport_interaction.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/viewport_interaction.rs)): cálculo em tempo real de matriz de transformação (`item.transform_matrix()`), renderização de gizmo no centro da anotação, arraste por eixos/planos, cancelamento por `Escape` e gravação de checkpoint no término.
The eleventh implementation round delivers Viewport Axis Locking Indications & Controls across 3 synchronized layers:
- Linhas-guia 3D infinitas projetadas no espaço da cena (`modal_viewport.rs`, `viewport_interaction.rs`): linhas atravessando o pivô com halo de brilho e cores canônicas do Blender (`AXIS_X` vermelho `#e03c42`, `AXIS_Y` verde `#62c934`, `AXIS_Z` azul `#3182f6`) e quad translúcido sombreado para planos (`Shift+X/Y/Z`).
- HUD flutuante dinâmico de alta visibilidade (`modal_viewport.rs`): pill com moldura na cor do eixo, badges semânticos `[ 🔒 EIXO X ]` e dicas contextuais de atalhos.
- Controles interativos na Viewport Bar (`viewport_bar.rs`): grupo `🔒 [ X ] [ Y ] [ Z ]` no cluster de transformação com preenchimento sólido colorido e badge dinâmico (`[ 🔒 Eixo X ]`), com suporte a travamento direto ou pré-configuração.
- Sincronização centralizada no domínio (`state.rs`, `modal.rs`): controle de estado unificado em `AppState.locked_axes`, herança em `begin_modal`, `toggle_axis_lock`, `is_axis_locked` e reset limpo em `commit_modal` e `cancel_modal`.
The twelfth implementation round delivers the Official Petunia3D Living Documentation Website, xtask Automation & Drift Prevention, and GitHub Actions CI/CD:
- Website estático completo com VitePress e Mermaid em [`docs/`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/): landing page oficial, Primeiros Passos (6 capítulos), Manual do Usuário (11 capítulos), Workspaces (5 guias), Catálogo de Ferramentas (19 ferramentas), Personalização (temas, ícones, i18n, keymaps), Atalhos (Cheatsheet e Blender), Portal do Desenvolvedor (8 guias) e Changelog sincronizado.
- Crate de automação [`crates/xtask`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/xtask) integrado no workspace (`cargo xtask docs` e `cargo xtask docs-check`) validando determinismo editorial e build do VitePress.
- Pipeline de deploy contínuo em [`.github/workflows/docs.yml`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/.github/workflows/docs.yml) publicando automaticamente no GitHub Pages a cada commit na branch `main`.
The thirteenth implementation round delivers Deep Interface Revision, Canonical Vector Iconography, Deduplicated Controls, and Unified Menus:
- Erradicação total de emojis Unicode em favor de mais de 20 ícones vetoriais canônicos desenhados via egui Painter ([`crates/ui/src/icons.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/icons.rs), [`crates/ui/src/icon_registry.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/icon_registry.rs)) com respeito estrito a DPI e tokens semânticos de tema.
- Fonte Única da Verdade para modos de seleção: remoção de botões duplicados da contextual shelf e centralização estrita no cabeçalho do viewport com atalhos numéricos (`1`, `2`, `3`).
- Sistema de menus unificado com `PetuniaMenuItem` ([`crates/ui/src/widgets.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/widgets.rs)) no padrão profissional do Blender `[Ícone] Rótulo ... [Atalho] ›` e busca dinâmica de atalhos (`Keybinds::shortcut_for`).
- Reorganização da Viewport Bar em 7 clusters responsivos com botões de visibilidade, travamento de eixos, projeção e esferas de sombreamento canônicas.
- Outliner e Properties refinados: substituição de botões textuais do Outliner por ícones vetoriais compactos, remoção das cores arco-íris de abas por tokens (`tokens::ACCENT_BLUE`), inspetor triaxial completo de Transform (Location, Rotation em graus, Scale), e padronização com `petunia_action_button`.
Final gates: 135 tests passing across the workspace (including 83 unit tests and 5 kittest tests in `petunia_ui`, and 42 tests in `petunia_core`); cargo fmt, strict Clippy (`-D warnings`), Prumo doctor e build estático do VitePress 100% verdes.
Evidence is preserved in `.prumo/history/premium/`, [`docs/GAUNTLET.md`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/GAUNTLET.md) and [`docs/GAUNTLET_HANDOFF.md`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/GAUNTLET_HANDOFF.md).

Premium convergence is **not established**. The human golden path and final GPU
performance targets remain unmeasured. Complex bevel, metric inset, inter-asset
occlusion, UV texture painting in the viewport and full GPU failure handling
remain acceptance gaps. See the plan and [`docs/GAUNTLET.md`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/GAUNTLET.md) for evidence and
limitations; do not reuse historical scores as proof of this specification.

## Next action

Use the remaining acceptance matrix in the premium plan. Validate the graphical
interaction and performance on both backends; extend unsupported geometry with
regression tests before raising any score or declaring convergence.

## Recovery order

1. `ENTRYPOINT.md` or platform adapter.
2. `prumo.json` and this state file.
3. `docs/PRUMO.md` and the premium interaction plan.
4. Historical goals in `.ai/goals/` (their DONE state does not close premium work).
5. Only relevant canonical docs, symbols and tests.
