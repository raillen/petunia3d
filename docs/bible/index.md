# Petunia3D — Implementation Bible & Livro Vivo

> **Fonte Única da Verdade (SSOT)**: Especificação canônica integral, contratos arquiteturais, diretivas para agents, matriz de requisitos P3D-001 a P3D-155 e fundamentos conceituais do modelador 3D Petunia3D.

## 🏛️ Constituição do Projeto & Diretivas (Capítulos 00 a 16)

| Capítulo | Título & Documento Canônico |
| :---: | :--- |
| `00` | [00 — Constituição do Projeto e Diretiva para Agentes](./constitution/00-constituicao-do-projeto-e-diretiva-para-agent.md) |
| `01` | [01 — Protocolo de Especificação P3D e Prompt para LLM](./constitution/01-protocolo-de-especificacao-p3d-e-prompt-para.md) |
| `02` | [02 — Gauntlet Loop, Quality Gates e Definition of Done](./constitution/02-gauntlet-loop-quality-gates-e-definition-of.md) |
| `03` | [03 — Invariantes de UI/UX, Design System e Acessibilidade](./constitution/03-invariantes-de-ui-ux-design-system-e-acessib.md) |
| `04` | [04 — Invariantes de Arquitetura, Modularidade e Core Agnóstico à UI](./constitution/04-invariantes-de-arquitetura-modularidade-e-co.md) |
| `05` | [05 — Política de Documentação, Screenshots e Changelog](./constitution/05-politica-de-documentacao-screenshots-e-chang.md) |
| `06` | [06 — Governança do Backlog, Status e Prioridades](./constitution/06-governanca-do-backlog-status-e-prioridades.md) |
| `07` | [07 — Roadmap, Adendos e Critérios para Novos Módulos](./constitution/07-roadmap-adendos-e-criterios-para-novos-modul.md) |
| `08` | [08 — Roadmap Pós-GA Aprovado](./constitution/08-roadmap-pos-ga-aprovado.md) |
| `09` | [09 — Shared 3D Foundation, Map Editor e Game Engine](./constitution/09-shared-3d-foundation-map-editor-e-game-engin.md) |
| `10` | [10 — Convenções Espaciais, Unidades e Coordenadas](./constitution/10-convencoes-espaciais-unidades-e-coordenadas.md) |
| `11` | [11 — Contrato de Mesh, Selection, Tools e Undo](./constitution/11-contrato-de-mesh-selection-tools-e-undo.md) |
| `12` | [12 — Contrato de Materiais, Texturas, UV e Color Pipeline](./constitution/12-contrato-de-materiais-texturas-uv-e-color-p.md) |
| `13` | [13 — Jobs, Concorrência, Diagnósticos, Segurança e Trust Boundaries](./constitution/13-jobs-concorrencia-diagnosticos-seguranca-e.md) |
| `14` | [14 — Release, Compatibilidade, Distribuição e Critérios de GA](./constitution/14-release-compatibilidade-distribuicao-e-crit.md) |
| `15` | [15 — Auditoria Final de Lacunas e Readiness Matrix](./constitution/15-auditoria-final-de-lacunas-e-readiness-matrix.md) |
| `16` | [16 — Master Prompt de Implementação por Gauntlet Waves](./constitution/16-master-prompt-de-implementacao-por-gauntlet-w.md) |

---

## 📦 Seções Macro da Aplicação (Seções A a O)

| Seção | Título do Subsistema |
| :---: | :--- |
| **Seção A** | [A — Project & Files](./sections/section-a-project-files.md) |
| **Seção B** | [B — Viewport & Navigation](./sections/section-b-viewport-navigation.md) |
| **Seção C** | [C — Reference Workflow](./sections/section-c-reference-workflow.md) |
| **Seção D** | [D — Selection, Transform & Modeling](./sections/section-d-selection-transform-modeling.md) |
| **Seção E** | [E — Assets, Scene & Inspector](./sections/section-e-assets-scene-inspector.md) |
| **Seção F** | [F — Surface, Materials, Paint & UV](./sections/section-f-surface-materials-paint-uv.md) |
| **Seção G** | [G — Animation & Rigging](./sections/section-g-animation-rigging.md) |
| **Seção H** | [H — Import & Export](./sections/section-h-import-export.md) |
| **Seção I** | [I — UI Shell & Professional UX](./sections/section-i-ui-shell-professional-ux.md) |
| **Seção J** | [J — Customization, i18n & Keymaps](./sections/section-j-customization-i18n-keymaps.md) |
| **Seção K** | [K — Architecture & Modularity](./sections/section-k-architecture-modularity.md) |
| **Seção L** | [L — Plugins, Automation & AI](./sections/section-l-plugins-automation-ai.md) |
| **Seção M** | [M — Help, Documentation & QA](./sections/section-m-help-documentation-qa.md) |
| **Seção N** | [N — Performance & Project Philosophy](./sections/section-n-performance-project-philosophy.md) |
| **Seção O** | [O — Future Product Integration](./sections/section-o-future-product-integration.md) |

---

## 📋 Especificações P3D por Wave do Gauntlet (P3D-001 a P3D-155)


### Wave 1: Architecture Spine

| ID | Especificação | Status Canônico |
| :---: | :--- | :---: |
| **P3D-100** | [P3D-100 — Command System](./specs/p3d-100-command-system.md) | ✅ COMPLIANT |
| **P3D-101** | [P3D-101 — Tools modulares](./specs/p3d-101-tools-modulares.md) | ✅ COMPLIANT |
| **P3D-102** | [P3D-102 — Core agnóstico à UI](./specs/p3d-102-core-agnostico-a-ui.md) | ✅ COMPLIANT |
| **P3D-103** | [P3D-103 — Separação Core / Application / UI](./specs/p3d-103-separacao-core-application-ui.md) | ✅ COMPLIANT |
| **P3D-104** | [P3D-104 — Headless readiness](./specs/p3d-104-headless-readiness.md) | ✅ COMPLIANT |
| **P3D-105** | [P3D-105 — Renderer desacoplado da UI](./specs/p3d-105-renderer-desacoplado-da-ui.md) | ✅ COMPLIANT |
| **P3D-106** | [P3D-106 — Frontends intercambiáveis](./specs/p3d-106-frontends-intercambiaveis.md) | ✅ COMPLIANT |
| **P3D-107** | [P3D-107 — API cross-language-ready](./specs/p3d-107-api-cross-language-ready.md) | ✅ COMPLIANT |
| **P3D-108** | [P3D-108 — IDs estáveis](./specs/p3d-108-ids-estaveis.md) | ✅ COMPLIANT |
| **P3D-109** | [P3D-109 — Events / State synchronization](./specs/p3d-109-events-state-synchronization.md) | ✅ COMPLIANT |
| **P3D-122** | [P3D-122 — Architecture Checks](./specs/p3d-122-architecture-checks.md) | ✅ COMPLIANT |

### Wave 2: Project Integrity & Asset Foundation

| ID | Especificação | Status Canônico |
| :---: | :--- | :---: |
| **P3D-001** | [P3D-001 — Sistema de Projetos](./specs/p3d-001-sistema-de-projetos.md) | ✅ COMPLIANT |
| **P3D-002** | [P3D-002 — Autosave e Recuperação](./specs/p3d-002-autosave-e-recuperacao.md) | ✅ COMPLIANT |
| **P3D-003** | [P3D-003 — Biblioteca de Modelos por Projeto](./specs/p3d-003-biblioteca-de-modelos-por-projeto.md) | ✅ COMPLIANT |
| **P3D-041** | [P3D-041 — Undo / Redo](./specs/p3d-041-undo-redo.md) | ✅ COMPLIANT |

### Wave 3: UI Infrastructure, Customization & Input

| ID | Especificação | Status Canônico |
| :---: | :--- | :---: |
| **P3D-073** | [P3D-073 — Workspace System](./specs/p3d-073-workspace-system.md) | ✅ COMPLIANT |
| **P3D-077** | [P3D-077 — Menus profissionais](./specs/p3d-077-menus-profissionais.md) | ✅ COMPLIANT |
| **P3D-079** | [P3D-079 — Layout responsivo desktop](./specs/p3d-079-layout-responsivo-desktop.md) | ✅ COMPLIANT |
| **P3D-081** | [P3D-081 — Command Palette](./specs/p3d-081-command-palette.md) | ✅ COMPLIANT |
| **P3D-084** | [P3D-084 — Design System tokenizado](./specs/p3d-084-design-system-tokenizado.md) | ✅ COMPLIANT |
| **P3D-085** | [P3D-085 — Temas customizados](./specs/p3d-085-temas-customizados.md) | ✅ COMPLIANT |
| **P3D-086** | [P3D-086 — Icon Packs](./specs/p3d-086-icon-packs.md) | ✅ COMPLIANT |
| **P3D-087** | [P3D-087 — Packs oficiais de ícones](./specs/p3d-087-packs-oficiais-de-icones.md) | ✅ COMPLIANT |
| **P3D-088** | [P3D-088 — Petunia Custom Icons](./specs/p3d-088-petunia-custom-icons.md) | ✅ COMPLIANT |
| **P3D-089** | [P3D-089 — Sistema completo de traduções](./specs/p3d-089-sistema-completo-de-traducoes.md) | ✅ COMPLIANT |
| **P3D-090** | [P3D-090 — Keybindings personalizáveis](./specs/p3d-090-keybindings-personalizaveis.md) | ✅ COMPLIANT |
| **P3D-091** | [P3D-091 — Detecção de conflitos](./specs/p3d-091-deteccao-de-conflitos.md) | ✅ COMPLIANT |
| **P3D-092** | [P3D-092 — Petunia Default](./specs/p3d-092-petunia-default.md) | ✅ COMPLIANT |
| **P3D-093** | [P3D-093 — Petunia Simple](./specs/p3d-093-petunia-simple.md) | ✅ COMPLIANT |
| **P3D-094** | [P3D-094 — Petunia Notebook](./specs/p3d-094-petunia-notebook.md) | ✅ COMPLIANT |
| **P3D-095** | [P3D-095 — Blender-like](./specs/p3d-095-blender-like.md) | ✅ COMPLIANT |
| **P3D-096** | [P3D-096 — Blender-like Notebook](./specs/p3d-096-blender-like-notebook.md) | ✅ COMPLIANT |
| **P3D-097** | [P3D-097 — Maya-like](./specs/p3d-097-maya-like.md) | ✅ COMPLIANT |
| **P3D-098** | [P3D-098 — 3ds Max-like](./specs/p3d-098-3ds-max-like.md) | ✅ COMPLIANT |
| **P3D-099** | [P3D-099 — Cinema 4D-like](./specs/p3d-099-cinema-4d-like.md) | ✅ COMPLIANT |
| **P3D-114** | [P3D-114 — Tooltips completos](./specs/p3d-114-tooltips-completos.md) | ✅ COMPLIANT |
| **P3D-115** | [P3D-115 — Ajuda contextual ?](./specs/p3d-115-ajuda-contextual.md) | ✅ COMPLIANT |

### Wave 4: Viewport, Navigation & Reference Workflow

| ID | Especificação | Status Canônico |
| :---: | :--- | :---: |
| **P3D-004** | [P3D-004 — Viewport 3D](./specs/p3d-004-viewport-3d.md) | ✅ COMPLIANT |
| **P3D-005** | [P3D-005 — Navegação 3D](./specs/p3d-005-navegacao-3d.md) | ✅ COMPLIANT |
| **P3D-006** | [P3D-006 — Vistas ortográficas](./specs/p3d-006-vistas-ortograficas.md) | ✅ COMPLIANT |
| **P3D-007** | [P3D-007 — Navigation Gizmo](./specs/p3d-007-navigation-gizmo.md) | ✅ COMPLIANT |
| **P3D-008** | [P3D-008 — Frame Selected / Frame All](./specs/p3d-008-frame-selected-frame-all.md) | ✅ COMPLIANT |
| **P3D-009** | [P3D-009 — Grid 3D](./specs/p3d-009-grid-3d.md) | ✅ COMPLIANT |
| **P3D-010** | [P3D-010 — Overlays](./specs/p3d-010-overlays.md) | ✅ COMPLIANT |
| **P3D-011** | [P3D-011 — X-Ray](./specs/p3d-011-x-ray.md) | ✅ COMPLIANT |
| **P3D-012** | [P3D-012 — Shading Modes](./specs/p3d-012-shading-modes.md) | ✅ COMPLIANT |
| **P3D-013** | [P3D-013 — Imagens de referência](./specs/p3d-013-imagens-de-referencia.md) | ✅ COMPLIANT |
| **P3D-014** | [P3D-014 — Referências ortográficas](./specs/p3d-014-referencias-ortograficas.md) | ✅ COMPLIANT |
| **P3D-074** | [P3D-074 — Viewport Header contextual](./specs/p3d-074-viewport-header-contextual.md) | ✅ COMPLIANT |
| **P3D-075** | [P3D-075 — Vertical Tool Toolbar](./specs/p3d-075-vertical-tool-toolbar.md) | ✅ COMPLIANT |
| **P3D-125** | [P3D-125 — Asset/Icon Caching](./specs/p3d-125-asset-icon-caching.md) | ✅ COMPLIANT |

### Wave 5: Selection, Transform & Modeling Core

| ID | Especificação | Status Canônico |
| :---: | :--- | :---: |
| **P3D-015** | [P3D-015 — Object Mode / Modelo Unificado de Seleção](./specs/p3d-015-object-mode-modelo-unificado-de-selecao.md) | ✅ COMPLIANT |
| **P3D-016** | [P3D-016 — Edit Mode — absorvido por P3D-015](./specs/p3d-016-edit-mode-absorvido-por-p3d-015.md) | ✅ COMPLIANT |
| **P3D-017** | [P3D-017 — Vertex Select](./specs/p3d-017-vertex-select.md) | ✅ COMPLIANT |
| **P3D-018** | [P3D-018 — Edge Select](./specs/p3d-018-edge-select.md) | ✅ COMPLIANT |
| **P3D-019** | [P3D-019 — Face Select](./specs/p3d-019-face-select.md) | ✅ COMPLIANT |
| **P3D-020** | [P3D-020 — Object Selection](./specs/p3d-020-object-selection.md) | ✅ COMPLIANT |
| **P3D-021** | [P3D-021 — Move](./specs/p3d-021-move.md) | ✅ COMPLIANT |
| **P3D-022** | [P3D-022 — Rotate](./specs/p3d-022-rotate.md) | ✅ COMPLIANT |
| **P3D-023** | [P3D-023 — Scale](./specs/p3d-023-scale.md) | ✅ COMPLIANT |
| **P3D-024** | [P3D-024 — Universal Transform](./specs/p3d-024-universal-transform.md) | ✅ COMPLIANT |
| **P3D-025** | [P3D-025 — Axis Constraints](./specs/p3d-025-axis-constraints.md) | ✅ COMPLIANT |
| **P3D-026** | [P3D-026 — Transform Orientation](./specs/p3d-026-transform-orientation.md) | ✅ COMPLIANT |
| **P3D-027** | [P3D-027 — Pivot System](./specs/p3d-027-pivot-system.md) | ✅ COMPLIANT |
| **P3D-028** | [P3D-028 — Add Primitives](./specs/p3d-028-add-primitives.md) | ✅ COMPLIANT |
| **P3D-029** | [P3D-029 — Extrude](./specs/p3d-029-extrude.md) | ✅ COMPLIANT |
| **P3D-030** | [P3D-030 — Multi-Extrude](./specs/p3d-030-multi-extrude.md) | ✅ COMPLIANT |
| **P3D-031** | [P3D-031 — Inset](./specs/p3d-031-inset.md) | ✅ COMPLIANT |
| **P3D-032** | [P3D-032 — Bevel](./specs/p3d-032-bevel.md) | ✅ COMPLIANT |
| **P3D-033** | [P3D-033 — Knife / Cut](./specs/p3d-033-knife-cut.md) | ✅ COMPLIANT |
| **P3D-034** | [P3D-034 — Loop Cut](./specs/p3d-034-loop-cut.md) | ✅ COMPLIANT |
| **P3D-035** | [P3D-035 — Subdivide](./specs/p3d-035-subdivide.md) | ✅ COMPLIANT |
| **P3D-036** | [P3D-036 — Merge](./specs/p3d-036-merge.md) | ✅ COMPLIANT |
| **P3D-037** | [P3D-037 — Split / Separate](./specs/p3d-037-split-separate.md) | ✅ COMPLIANT |
| **P3D-038** | [P3D-038 — Mirror simples](./specs/p3d-038-mirror-simples.md) | ✅ COMPLIANT |
| **P3D-039** | [P3D-039 — Proportional Editing](./specs/p3d-039-proportional-editing.md) | ✅ COMPLIANT |
| **P3D-040** | [P3D-040 — Sistema de Snap](./specs/p3d-040-sistema-de-snap.md) | ✅ COMPLIANT |
| **P3D-123** | [P3D-123 — Testes de ferramentas](./specs/p3d-123-testes-de-ferramentas.md) | ✅ COMPLIANT |
| **P3D-131** | [P3D-131 — Modal Tool Feedback System](./specs/p3d-131-modal-tool-feedback-system.md) | ✅ COMPLIANT |

### Wave 6: Scene, Assets, Outliner & Inspector

| ID | Especificação | Status Canônico |
| :---: | :--- | :---: |
| **P3D-042** | [P3D-042 — Asset Browser](./specs/p3d-042-asset-browser.md) | ✅ COMPLIANT |
| **P3D-043** | [P3D-043 — Busca e filtros de Assets](./specs/p3d-043-busca-e-filtros-de-assets.md) | ✅ COMPLIANT |
| **P3D-044** | [P3D-044 — Drag & Drop de Assets](./specs/p3d-044-drag-drop-de-assets.md) | ✅ COMPLIANT |
| **P3D-045** | [P3D-045 — Thumbnails configuráveis](./specs/p3d-045-thumbnails-configuraveis.md) | ✅ COMPLIANT |
| **P3D-046** | [P3D-046 — Outliner](./specs/p3d-046-outliner.md) | ✅ COMPLIANT |
| **P3D-047** | [P3D-047 — Visibility / Lock](./specs/p3d-047-visibility-lock.md) | ✅ COMPLIANT |
| **P3D-048** | [P3D-048 — Inspector / Properties](./specs/p3d-048-inspector-properties.md) | ✅ COMPLIANT |
| **P3D-049** | [P3D-049 — Transform Inspector](./specs/p3d-049-transform-inspector.md) | ✅ COMPLIANT |
| **P3D-076** | [P3D-076 — Contextual Tool Shelf](./specs/p3d-076-contextual-tool-shelf.md) | ✅ COMPLIANT |
| **P3D-078** | [P3D-078 — Painéis retráteis e redimensionáveis](./specs/p3d-078-paineis-retrateis-e-redimensionaveis.md) | ✅ COMPLIANT |
| **P3D-080** | [P3D-080 — Status Bar](./specs/p3d-080-status-bar.md) | ✅ COMPLIANT |
| **P3D-082** | [P3D-082 — Context Menus](./specs/p3d-082-context-menus.md) | ✅ COMPLIANT |
| **P3D-083** | [P3D-083 — Tool Properties](./specs/p3d-083-tool-properties.md) | ✅ COMPLIANT |

### Wave 7: Materials, Texture, UV & Paint (PRÓXIMA / ATIVA)

| ID | Especificação | Status Canônico |
| :---: | :--- | :---: |
| **P3D-050** | [P3D-050 — Material System](./specs/p3d-050-material-system.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-051** | [P3D-051 — Albedo / Diffuse](./specs/p3d-051-albedo-diffuse.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-052** | [P3D-052 — Normal / Bump](./specs/p3d-052-normal-bump.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-053** | [P3D-053 — Roughness / Glossiness](./specs/p3d-053-roughness-glossiness.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-054** | [P3D-054 — Displacement / Height](./specs/p3d-054-displacement-height.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-055** | [P3D-055 — Paint Workspace](./specs/p3d-055-paint-workspace.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-056** | [P3D-056 — Pixel Brush](./specs/p3d-056-pixel-brush.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-057** | [P3D-057 — Soft Brush](./specs/p3d-057-soft-brush.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-058** | [P3D-058 — Eraser](./specs/p3d-058-eraser.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-059** | [P3D-059 — Fill](./specs/p3d-059-fill.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-060** | [P3D-060 — Color Picker](./specs/p3d-060-color-picker.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-061** | [P3D-061 — Simple Paint Layers](./specs/p3d-061-simple-paint-layers.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-062** | [P3D-062 — Paint de mapas via UV](./specs/p3d-062-paint-de-mapas-via-uv.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-063** | [P3D-063 — UV Workspace](./specs/p3d-063-uv-workspace.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-064** | [P3D-064 — UV Editing básico](./specs/p3d-064-uv-editing-basico.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-065** | [P3D-065 — Integração Paint ↔ UV](./specs/p3d-065-integracao-paint-uv.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-132** | [P3D-132 — Paint Masks / Face & Selection Isolation](./specs/p3d-132-paint-masks-face-selection-isolation.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-133** | [P3D-133 — Decal & Projection Layers](./specs/p3d-133-decal-projection-layers.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-134** | [P3D-134 — Paint Effect Stack](./specs/p3d-134-paint-effect-stack.md) | 🔄 ATIVA / EM ANDAMENTO |
| **P3D-140** | [P3D-140 — Material Shader Profiles & Effects](./specs/p3d-140-material-shader-profiles-effects.md) | 🔄 ATIVA / EM ANDAMENTO |

### Wave 8: Import, Export & Delivery Pipeline

| ID | Especificação | Status Canônico |
| :---: | :--- | :---: |
| **P3D-068** | [P3D-068 — Export individual](./specs/p3d-068-export-individual.md) | ⏳ PLANEJADA |
| **P3D-069** | [P3D-069 — Export múltiplo](./specs/p3d-069-export-multiplo.md) | ⏳ PLANEJADA |
| **P3D-070** | [P3D-070 — Batch Export](./specs/p3d-070-batch-export.md) | ⏳ PLANEJADA |
| **P3D-071** | [P3D-071 — Importadores modulares](./specs/p3d-071-importadores-modulares.md) | ⏳ PLANEJADA |
| **P3D-072** | [P3D-072 — Exportadores modulares](./specs/p3d-072-exportadores-modulares.md) | ⏳ PLANEJADA |
| **P3D-124** | [P3D-124 — Testes de import/export](./specs/p3d-124-testes-de-import-export.md) | ⏳ PLANEJADA |

### Wave 9: Documentation, QA, Release & GA Hardening

| ID | Especificação | Status Canônico |
| :---: | :--- | :---: |
| **P3D-116** | [P3D-116 — Website de documentação](./specs/p3d-116-website-de-documentacao.md) | ⏳ PLANEJADA |
| **P3D-117** | [P3D-117 — Changelog vivo](./specs/p3d-117-changelog-vivo.md) | ⏳ PLANEJADA |
| **P3D-118** | [P3D-118 — Screenshots atualizados](./specs/p3d-118-screenshots-atualizados.md) | ⏳ PLANEJADA |
| **P3D-119** | [P3D-119 — Referência automática de tokens](./specs/p3d-119-referencia-automatica-de-tokens.md) | ⏳ PLANEJADA |
| **P3D-120** | [P3D-120 — Docs Check](./specs/p3d-120-docs-check.md) | ⏳ PLANEJADA |
| **P3D-121** | [P3D-121 — UI Regression Tests](./specs/p3d-121-ui-regression-tests.md) | ⏳ PLANEJADA |
| **P3D-126** | [P3D-126 — Performance para PCs modestos](./specs/p3d-126-performance-para-pcs-modestos.md) | ⏳ PLANEJADA |
| **P3D-127** | [P3D-127 — No Remesh](./specs/p3d-127-no-remesh.md) | ⏳ PLANEJADA |
| **P3D-128** | [P3D-128 — Sem composição de cenas complexa](./specs/p3d-128-sem-composicao-de-cenas-complexa.md) | ⏳ PLANEJADA |
| **P3D-129** | [P3D-129 — Sem login obrigatório](./specs/p3d-129-sem-login-obrigatorio.md) | ⏳ PLANEJADA |
| **P3D-130** | [P3D-130 — Customização sem quebrar Core](./specs/p3d-130-customizacao-sem-quebrar-core.md) | ⏳ PLANEJADA |

### Waves 10-13: Roadmap Pós-GA (Animation, Plugins, Engine)

| ID | Especificação | Status Canônico |
| :---: | :--- | :---: |
| **P3D-066** | [P3D-066 — Animation Workspace](./specs/p3d-066-animation-workspace.md) | 🚀 PÓS-GA |
| **P3D-067** | [P3D-067 — Animação simples](./specs/p3d-067-animacao-simples.md) | 🚀 PÓS-GA |
| **P3D-110** | [P3D-110 — Lua Plugin System](./specs/p3d-110-lua-plugin-system.md) | 🚀 PÓS-GA |
| **P3D-111** | [P3D-111 — Public Plugin API](./specs/p3d-111-public-plugin-api.md) | 🚀 PÓS-GA |
| **P3D-112** | [P3D-112 — MCP API / Automação](./specs/p3d-112-mcp-api-automacao.md) | 🚀 PÓS-GA |
| **P3D-113** | [P3D-113 — Texturing Nodes simples](./specs/p3d-113-texturing-nodes-simples.md) | 🚀 PÓS-GA |
| **P3D-135** | [P3D-135 — Skeleton & Rig Core](./specs/p3d-135-skeleton-rig-core.md) | 🚀 PÓS-GA |
| **P3D-136** | [P3D-136 — Rig Presets — Humanoid, Quadruped e Multi-Leg](./specs/p3d-136-rig-presets-humanoid-quadruped-e-mult.md) | 🚀 PÓS-GA |
| **P3D-137** | [P3D-137 — Auto-Rig](./specs/p3d-137-auto-rig.md) | 🚀 PÓS-GA |
| **P3D-138** | [P3D-138 — Animation Retargeting / External Compatibility](./specs/p3d-138-animation-retargeting-external-compatibi.md) | 🚀 PÓS-GA |
| **P3D-139** | [P3D-139 — Animation Asset Library](./specs/p3d-139-animation-asset-library.md) | 🚀 PÓS-GA |
| **P3D-141** | [P3D-141 — Internal AI Agent Panel](./specs/p3d-141-internal-ai-agent-panel.md) | 🚀 PÓS-GA |
| **P3D-142** | [P3D-142 — AI-Assisted Modeling Pipeline](./specs/p3d-142-ai-assisted-modeling-pipeline.md) | 🚀 PÓS-GA |
| **P3D-143** | [P3D-143 — Game Engine Integration / Bridge](./specs/p3d-143-game-engine-integration-bridge.md) | 🚀 PÓS-GA |
| **P3D-144** | [P3D-144 — Asset Validator & Game Readiness](./specs/p3d-144-asset-validator-game-readiness.md) | 🚀 PÓS-GA |
| **P3D-145** | [P3D-145 — Collision Authoring](./specs/p3d-145-collision-authoring.md) | 🚀 PÓS-GA |
| **P3D-146** | [P3D-146 — Sockets & Attachment Points](./specs/p3d-146-sockets-attachment-points.md) | 🚀 PÓS-GA |
| **P3D-147** | [P3D-147 — LOD Manager](./specs/p3d-147-lod-manager.md) | 🚀 PÓS-GA |
| **P3D-148** | [P3D-148 — Palette Manager](./specs/p3d-148-palette-manager.md) | 🚀 PÓS-GA |
| **P3D-149** | [P3D-149 — Texture Atlas Builder](./specs/p3d-149-texture-atlas-builder.md) | 🚀 PÓS-GA |
| **P3D-150** | [P3D-150 — Vertex Color Painting](./specs/p3d-150-vertex-color-painting.md) | 🚀 PÓS-GA |
| **P3D-151** | [P3D-151 — Batch Asset Processor](./specs/p3d-151-batch-asset-processor.md) | 🚀 PÓS-GA |
| **P3D-152** | [P3D-152 — Portable Project Package](./specs/p3d-152-portable-project-package.md) | 🚀 PÓS-GA |
| **P3D-153** | [P3D-153 — Tool Presets](./specs/p3d-153-tool-presets.md) | 🚀 PÓS-GA |
| **P3D-154** | [P3D-154 — Command Recipes / Macros](./specs/p3d-154-command-recipes-macros.md) | 🚀 PÓS-GA |
| **P3D-155** | [P3D-155 — Live Asset Link](./specs/p3d-155-live-asset-link.md) | 🚀 PÓS-GA |

---

## 📜 Adendos Aprovados

| Adendo | Descrição |
| :---: | :--- |
| `adendo-01` | [ADENDO-01 — Possíveis futuros módulos sem bloat](./addenda/adendo-01-possiveis-futuros-modulos-sem-bloat.md) |
| `adendo-02` | [ADENDO-02 — Caminho para uma game engine](./addenda/adendo-02-caminho-para-uma-game-engine.md) |
| `adendo-03` | [ADENDO-03 — Texturas com efeitos e shaders](./addenda/adendo-03-texturas-com-efeitos-e-shaders.md) |

---

## 💡 Fundamentos Conceituais do Livro Vivo (36 Capítulos)

| Capítulo | Título do Capítulo |
| :---: | :--- |
| `01` | [01 — Visão, UX e Referências](./foundations/01-visao-ux-referencias.md) |
| `02` | [02 — Workflow de Modelagem Shape-First](./foundations/02-workflow-modelagem-shape-first.md) |
| `03` | [03 — Geometry Core, Faces e Topologia](./foundations/03-geometry-core-faces-topologia.md) |
| `04` | [04 — Combine, Fuse, Weld e Personagens](./foundations/04-combine-fuse-weld-personagens.md) |
| `05` | [05 — Viewport, Shading e Modos de Visualização](./foundations/05-viewport-shading-modos-visualizacao.md) |
| `06` | [06 — Escopo Essencial: o que entra e o que fica fora](./foundations/06-escopo-essencial.md) |
| `07` | [07 — Pesquisa: referências, lacunas e ideias acadêmicas](./foundations/07-pesquisa-referencias-lacunas.md) |
| `08` | [08 — Photo Projection, Trace & Project e Lições do Shapr3D](./foundations/08-photo-projection-trace-project.md) |
| `09` | [09 — Arquitetura, Princípios de Decisão e Governança Técnica](./foundations/09-arquitetura-governanca-tecnica.md) |
| `10` | [10 — Extension API, Plugins e Módulos Oficiais](./foundations/10-extension-api-plugins-modulos.md) |
| `11` | [11 — MCP API, Automação e Integração com Agentes de IA](./foundations/11-mcp-api-automacao-agentes.md) |
| `12` | [12 — Baseline Funcional, Roadmap e Contrato de Escopo](./foundations/12-baseline-funcional-roadmap-escopo.md) |
| `13` | [13 — Contrato para Geração de Documentação pelo Framework](./foundations/13-contrato-geracao-documentacao.md) |
| `14` | [14 — Modelagem V1: Cut, Slice, Bevel, Revolve e Simple Sweep](./foundations/14-modelagem-v1-cut-slice-bevel-revolve-sweep.md) |
| `15` | [15 — UV, Textura, Projeção e Materiais — Arquitetura Fechada](./foundations/15-uv-textura-projecao-materiais.md) |
| `16` | [16 — Documento, Formato de Projeto, Undo, Autosave e Recovery](./foundations/16-documento-formato-undo-recovery.md) |
| `17` | [17 — Export, Import, Validação e Pipeline Game-Ready](./foundations/17-export-import-validacao-game-ready.md) |
| `18` | [18 — Stack Técnica, Dependências e Fronteiras de Integração](./foundations/18-stack-tecnica-dependencias.md) |
| `19` | [19 — Concorrência, Memória, Caches e Performance](./foundations/19-concorrencia-memoria-caches-performance.md) |
| `20` | [20 — Testes, Conformance e Qualidade Técnica](./foundations/20-testes-conformance-qualidade.md) |
| `21` | [21 — Baseline Funcional e UI V1 Congeladas, Stack Rust Final](./foundations/21-baseline-funcional-ui-v1-stack-rust.md) |
| `22` | [22 — Referência de Interface: Análise do Figma Blender UI Redesign](./foundations/22-referencia-interface-figma.md) |
| `23` | [23 — Macroarquitetura da Interface Petunia3D](./foundations/23-macroarquitetura-interface.md) |
| `24` | [24 — Design System Visual: Tokens, Hierarquia e Estados](./foundations/24-design-system-tokens-estados.md) |
| `25` | [25 — Biblioteca de Componentes e Contratos de Interação](./foundations/25-biblioteca-componentes-interacao.md) |
| `26` | [26 — Figma → Implementação, Assets e Fundamentos de Acessibilidade](./foundations/26-figma-implementacao-acessibilidade.md) |
| `27` | [27 — Stack Rust Canônica: Rust + egui + wgpu](./foundations/27-stack-rust-canonica.md) |
| `28` | [28 — Arquitetura Rust, Cargo Workspace e Fronteiras entre Crates](./foundations/28-arquitetura-rust-cargo-crates.md) |
| `29` | [29 — Geometry Core, Renderer, UV e Painting na Stack Rust](./foundations/29-geometry-renderer-uv-painting-rust.md) |
| `30` | [30 — I/O, Projeto .petunia, Lua Plugins e MCP na Stack Rust](./foundations/30-io-petunia-lua-mcp-rust.md) |
| `31` | [31 — Qualidade, IA, CI e Vertical Slice de Conformance da Stack Rust](./foundations/31-qualidade-ia-ci-vertical-slice.md) |
| `32` | [32 — ADR: Migração da Baseline Odin para Rust](./foundations/32-adr-odin-para-rust.md) |
| `33` | [33 — Biblioteca de Referências Técnicas da Stack Rust](./foundations/33-referencias-tecnicas-rust.md) |
| `34` | [34 — Arquitetura Modular Explícita, Rust Safety e Representação em Código](./foundations/34-arquitetura-modular-rust-safety.md) |
| `35` | [35 — egui, Petunia Components, UI Adapters e Tooling de Desenvolvimento](./foundations/35-egui-components-adapters-tooling.md) |
| `36` | [36 — UI Baseline Final V1, Temas e Plugin Panels](./foundations/36-ui-baseline-temas-plugin-panels.md) |
