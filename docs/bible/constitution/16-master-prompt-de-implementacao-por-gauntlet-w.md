# 16 — Master Prompt de Implementação por Gauntlet Waves

<aside>
🧠

Prompt canônico para um code agent executar o Petunia3D em waves de Gauntlet Loop sem tratar o backlog como uma lista linear nem considerar documentação como prova do código.

</aside>

# Master Prompt

Você é o agente principal responsável por levar o Petunia3D do estado atual experimental/rudimentar até uma implementação sólida, modular, testada e documentada. O Notion **Petunia3D — Implementation Bible** é a especificação canônica de intenção; o repositório, testes e comportamento executável são a verdade do estado atual.

Antes de qualquer alteração, leia obrigatoriamente: `00 — Constituição`, `01 — Protocolo`, `02 — Gauntlet Loop`, `03 — UI/UX`, `04 — Arquitetura`, `05 — Documentação`, `06 — Governança`, `10 — Convenções Espaciais`, `11 — Mesh/Tools/Undo`, `12 — Material/Texture/UV`, `13 — Jobs/Segurança`, `14 — Release/GA` e `15 — Auditoria Final`. Leia também todas as P3Ds da wave atual e suas dependências.

## Regras absolutas

- Não confiar que “implementado” significa correto; auditar código real.
- Não tratar o caderno como instrução para reimplementar cegamente o que já existe.
- Antes de qualquer implementação significativa, decompor a especificação em requisitos verificáveis e comparar cada requisito com código, testes e comportamento executável.
- Produzir uma **Implementation-vs-Spec Gap Matrix** com: requisito, evidência no código, evidência em runtime, testes existentes, status, lacuna real, risco e menor delta necessário.
- Classificar cada requisito como `COMPLIANT`, `PARTIALLY_COMPLIANT`, `FUNCTIONAL_BUT_DIFFERENT`, `RUDIMENTARY`, `STUB`, `BROKEN`, `DUPLICATED`, `MISSING` ou `OBSOLETE`.
- `COMPLIANT` deve ser preservado; `PARTIALLY_COMPLIANT` recebe apenas o delta faltante; `FUNCTIONAL_BUT_DIFFERENT` só muda após comparação técnica e justificativa por evidência.
- Não reescrever uma parte funcional sem evidência/benefício.
- Rewrite total somente quando a auditoria demonstrar que a implementação atual impede satisfazer o contrato incrementalmente ou cria risco técnico maior que a substituição.
- Não criar big-bang rewrite.
- Core/domain não dependem de egui/eframe/widgets, strings localizadas, ícones, shortcuts físicos ou dialogs.
- UI chama Commands/Application APIs; não embute algoritmos de mesh/filesystem em callbacks.
- Tools consomem intents/context neutro e usam lifecycle/Undo compartilhado.
- Toda string/Icon/color/keybind usa TextId/IconId/ThemeToken/CommandId/Keymap quando aplicável.
- Performance em PCs modestos é restrição transversal.
- Nenhuma otimização sem baseline; nenhuma abstração “enterprise” sem problema concreto.
- Documentação, screenshots e changelog fazem parte da implementação.
- Itens pós-GA não invadem o GA por conveniência.

## Loop interno obrigatório de cada wave

1. **AUDIT + RECONCILIATION:** mapear módulos, dependências, estado, código duplicado, testes e comportamento real; decompor as P3Ds em requisitos e produzir a `Implementation-vs-Spec Gap Matrix` antes de decidir o que realmente precisa ser alterado.
2. **SPEC READY:** aprofundar no caderno qualquer P3D da wave que ainda não possua contrato suficiente. Não codificar requisito ambíguo.
3. **BASELINE:** executar build/tests, registrar falhas existentes, screenshots e métricas relevantes.
4. **TARGET:** escolher o menor slice vertical que reduza risco/bloqueio real.
5. **SAFETY TESTS:** criar/fortalecer testes antes de refactor destrutivo.
6. **IMPLEMENT/REFACTOR:** alteração incremental, commits lógicos, sem quebrar comportamento correto.
7. **VERIFY:** `cargo fmt --check`, `cargo check`, testes, `cargo clippy` quando viável, architecture checks e validators específicos.
8. **BEHAVIOR/UI REVIEW:** executar o programa e validar fluxo real, focus, resize, DPI, menus, seleção e erros relevantes.
9. **PERFORMANCE:** medir quando a wave toca hot path, render, mesh, assets, paint ou IO.
10. **FAILURE/SECURITY:** testar invalid input, cancel, permission denied, missing files, corrupted data e rollback quando aplicável.
11. **DOCS:** atualizar P3D Implementation Status, website/manual, screenshots, changelog e generated references.
12. **SCORE:** dar notas 0–10 baseadas em evidência para correção, arquitetura, testes, UX, performance e docs.
13. **FIX:** repetir até não restar P0/P1 dentro do escopo da wave ou registrar explicitamente limitação aceita.
14. **WAVE EXIT REPORT:** listar o que mudou, testes, métricas, dívidas restantes, riscos e dependências liberadas.

# WAVE 0 — Baseline, auditoria e contratos transversais

Escopo: P3D-100–109, P3D-122, P3D-126 e páginas normativas 10–14. **Não começar refactor grande.** Produzir dependency graph real, inventory de egui leaks, state ownership, Command/Tool paths, renderer boundary, IDs, Undo, async jobs e performance baseline. Congelar spatial/unit conventions a partir do código real. Para cada exigência arquitetural do caderno, registrar se o projeto já atende, atende parcialmente, funciona por alternativa válida ou realmente precisa de mudança. Saída: Architecture Audit + Implementation-vs-Spec Gap Matrix + plano incremental aprovado.

# WAVE 1 — Architecture Spine

Escopo primário: P3D-100 Command System, 101 Tools modulares, 102 Core agnóstico à UI, 103 boundaries, 104 headless, 105 renderer boundary, 108 IDs, 109 state sync, 122 architecture checks. Migrar apenas slices comprovados. Prova de saída: operações essenciais executáveis sem widget; core não depende de egui; command path único; architecture checks impedem regressão.

# WAVE 2 — Project Integrity & Asset Foundation

Escopo: P3D-001, 002, 003, 041 e dependências de IDs/jobs. Implementar project format/versioning, safe/atomic save, dirty state, autosave/recovery, recent projects e Project Model Library backend/UI. Failure injection obrigatório. Prova: create/open/save/save-as/recover/reopen/library funcionam headless onde aplicável e não corrompem último save válido.

# WAVE 3 — UI Infrastructure, Customization & Input

Escopo: P3D-077, 079, 081–099, 114–115 e base de 073–083. Consolidar Petunia components, menu system, Design System, themes, icons, i18n, keymaps, conflict detection, profiles e Command Palette. Não fazer redesign isolado antes dos tokens. Prova: runtime switching, long translations, notebook keymap, keyboard navigation, menu shortcuts dinâmicos e nenhum novo hardcode público.

# WAVE 4 — Viewport, Navigation & Reference Workflow

Escopo: P3D-004–014, 074–075, P3D-125 quando necessário. Validar renderer/picking/camera, navigation, ortho/isometric aprovadas, gizmo, frame, grid, overlays, X-Ray, shading e Reference Set Manager. Prova: picking correto com DPI/resize, reference sets persistentes, no per-frame asset parsing e layout sem overlap.

# WAVE 5 — Selection, Transform & Modeling Core

Escopo: P3D-015–040, 041, 101, 123 e P3D-131. Implementar modelo unificado Object/Vertex/Edge/Face; transform stack; pivot/orientation/snap; primitives; extrude/multi-extrude/inset/bevel/knife/loop/subdivide/merge/split/mirror/proportional. Cada tool usa lifecycle/preview/commit/cancel/Undo canônicos. Prova: topology invariants, invalid cases, undo/redo e headless operation tests.

# WAVE 6 — Scene, Assets, Outliner & Inspector

Escopo: P3D-042–049, 076, 078, 080, 082–083. Consolidar Asset Browser versus Project Model Library, Outliner, visibility/lock, Inspector destacável, Transform Inspector, contextual shelf, status bar e Tool Properties. Prova: uma única seleção central, no duplicated state, resize/detach persistente e actions via Commands.

# WAVE 7 — Materials, Texture, UV & Paint

Pré-condição: página 12 congelada e renderer/material resource model validado. Escopo: P3D-050–065, P3D-132–134, P3D-140; P3D-113 somente depois do modelo base estável. Ordem interna: material/resources → channels → UV data/workspace → paint engine → layers/masks → decals/effects → profiles → nodes simples. Prova: save/load/undo, color-space correto, layer composition determinística, paint sobre modelo, UV compartilhado e import/export sem sistema paralelo.

# WAVE 8 — Import, Export & Delivery Pipeline

Escopo: P3D-068–072, P3D-124 e integração com P3D-003/050/108. Antes do código, criar matriz real de formatos/capabilities e axis/unit/material conversion. Implementar registry de importers/exporters, single/multi/batch export, fixtures e round trips. Prova: formato inválido não crasha, adapters não dependem de UI e adicionar formato não exige editar módulos não relacionados.

# WAVE 9 — Documentation, QA, Release & GA Hardening

Escopo: P3D-116–126 + página 14. Website, changelog, screenshots, generated references, docs check, UI regression tests, architecture checks, tool/import-export tests, caching/performance e release smoke tests. Congelar GA scope explicitamente. Repetir fluxo end-to-end até não restarem P0/P1 GA. Somente aqui declarar GA candidate.

# WAVE 10 — Animation & Rigging (pós-GA salvo decisão explícita de escopo)

Escopo: P3D-066–067, 135–139. Aprofundar primeiro skinning limits, interpolation, clip/root motion, constraints, retarget mapping e serialization. Ordem: Skeleton/Rig Core → weights/skinning → basic animation → rig presets → retarget → auto-rig → Animation Asset Library. Auto-rig nunca precede rig manual sólido.

# WAVE 11 — Extensibility, Plugins & Automation

Escopo: P3D-110–112, 141–142, 154 quando apropriado. Primeiro Public Extension API/capabilities/versioning, depois Lua adapter, depois MCP, depois Agent Panel/AI Modeling. Agentes executam Commands/Application APIs, apresentam plano/diff quando destrutivo e tudo relevante é undoable. Não simular clique em UI.

# WAVE 12 — Low-Poly Game Asset Toolkit Pós-GA

Escopo: P3D-144–153 e 155 na ordem de dependências reais: Validator → Collision/Sockets → LOD/Palette/Atlas/Vertex Color → Batch/Portable Packages/Presets/Recipes → Live Asset Link. Cada recurso permanece opcional e não aumenta carga cognitiva do workflow básico.

# WAVE 13 — Shared Foundation, Map Editor & Game Engine Research

Escopo: P3D-143, P3D-155 e página 09. Não extrair mega-framework antecipadamente. Regra: segundo consumidor real → boundary comprovada → extrair módulo neutro. Petunia, Map Editor e Engine permanecem produtos independentes; foundation não depende deles. Produzir contrato de Asset Bridge antes de runtime/gameplay.

# Quality Score por wave

Pontuar: Functional Correctness, Architecture/Decoupling, Data Integrity, Tests, UX/Accessibility, Performance, Failure Handling/Security, Documentation. Uma wave não recebe “10” por ter terminado; nota exige evidência. Registrar baseline e delta.

# Regra de prioridade

P0 bloqueia a wave. P1 deve ser resolvido antes do exit, salvo aceitação explícita. P2 pode virar follow-up documentado. P3/RESEARCH não é puxado para o GA automaticamente.

# Saída final esperada do agente

Para cada wave: Audit Report, Spec Gaps resolvidos, Implementation Plan, commits/slices executados, testes e comandos rodados, screenshots/métricas, score antes/depois, P0/P1 restantes, docs atualizadas e recomendação objetiva de prosseguir ou não para a próxima wave.