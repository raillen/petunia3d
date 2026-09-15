# UI Remediation — Wave 0 Baseline

- Revision: `0a12dd1` + Wave 1 (uncommitted at capture time)
- Date: 2026-09-15
- Directive: `PETUNIA3D_MUSE_SPARK_1_3_UI_ARCHITECTURE_REMEDIATION.md`

## P0 confirmation (code evidence)

- **P0-A confirmed**: `crates/render-wgpu/src/lib.rs::Renderer::update()` rebuilt
  `mesh_vb`/`line_vb`/`ref_vb` every call (triangulation + `create_buffer_init`
  per frame). Fixed in Wave 1 via `fingerprint_scene` early-return; camera writes
  only `cam_buffer`.
- **P0-B confirmed**: `crates/render-gl/src/lib.rs::draw_mesh/draw_edges/draw_refs`
  did `create_buffer → buffer_data → draw → delete_buffer` per asset/frame.
  Fixed in Wave 1 via persistent per-asset VBOs + edge/ref VBOs + CPU caches.
- **P0-C confirmed**: `crates/ui/src/lib.rs::right_panel()` renders Outliner +
  Properties inside single `Panel::right("props")`. Not yet fixed (Wave 2).
- **Shelf confirmed**: `crates/ui/src/contextual_shelf.rs` used workspace
  estimated widths (700/780/480/440/640) + `label.len() * 6.5` text measure.
  Not yet fixed (Wave 4).
- **Icons confirmed**: production path is `icon_registry.rs::IconRegistry::paint()`;
  `icon_provider.rs` (iconflow) used only in own tests; `paint_pack()` renders all
  third-party packs from the same Phosphor source. Not yet fixed (Wave 6).
- **i18n confirmed**: hardcoded literals in shelf/paint/UV/animate/settings paths
  (e.g. `"Pincel"`, `"Raio:"`, `"👤 Humanoide"`). Not yet fixed (Wave 7).

## Test baseline

- `cargo test -p petunia_core --lib`: **73 passed** (incl. 5 new
  `render_revision` fingerprint tests).
- `cargo clippy -p petunia_core -p petunia_render_gl -p petunia_render_wgpu
  --all-targets -- -D warnings`: clean.
- Full workspace gates + `arch-check`/`docs-check` pending re-run at Wave 1 close.

## Perf baseline (method; headless CI has no GPU)

- New counters: WGPU `mesh_rebuilds()/skipped_frames()`; GL
  `buffer_creations/skipped_uploads`; `upload_ref_pixels` already hash-gated.
- Render-on-demand preserved: `about_to_wait` redraws only on
  `consume_dirty()` (or `SIMPLE3D_SPIN`); camera orbit marks dirty via existing
  paths — camera-only frames now skip geometry rebuild/upload on both backends.
- Reference pixels: zero re-upload on unchanged hash (both backends).
- Profiling: `puffin::profile_function!()` now on UI draw, right dock, Outliner,
  Properties, shelf, Reference Manager, WGPU update/render, GL draw/mesh/edges/refs.
- Low-end scenario matrix (§16.7) requires hardware run before release; idle
  target = no continuous repaint without reason.

## Architecture Convergence Ledger (initial)

| Concept | Old production path | Pilot / new path | Canonical choice | Deletion criteria |
|---|---|---|---|---|
| GPU mesh cache | per-frame rebuild (removed) | `fingerprint_scene` + persistent buffers (Wave 1) | fingerprint + persistent buffers | old comment/code already removed |
| Workspace layout | static single right `Panel` (removed) | owned dock split in `ui::right_panel` (Wave 2) | **owned split-pane** (acceptable alternative §4.3: GPU viewport lives outside tiles; promoting `egui_tiles` would fork layout ownership) | `tiles_workspace.rs` stays as dev pilot only until Wave 9 verdict |
| GL VBOs | create/delete per draw (removed) | per-asset persistent VBOs (Wave 1) | persistent VBOs | no `create_buffer`/`delete_buffer` inside draw fns except prune/init |
| Icons | `icon_registry` PNG/Phosphor production | `icon_provider` iconflow pilot (tests only) | TBD Wave 6: single semantic service | remove non-canonical path after parity |
| Workspace layout | static Panels in `ui::draw` | `tiles_workspace.rs` pilot (inactive) | TBD Wave 2/3 | one canonical path only |
| Strings | raw literals + `I18n::t` | typed `TextId` (planned Wave 7) | TBD Wave 7 | zero raw user-facing literals |
| Dirty tracing | `mark_dirty()` everywhere | `DirtyReason` added (Wave 1), adoption pending | `mark_dirty_reason` on known paths | audit: no draw/hover-triggered dirty |

## Exit gate

Reproducible baseline exists: this file + fingerprint unit tests + counters +
profiler scopes. Wave 1 implements the renderer correction on top.

## Wave 2 close-out (2026-09-15)

- `UiRegions` (`crates/ui/src/regions.rs`): shell rects recorded per frame by
  each pane owner; `status_overlaps` / `shelf_within_viewport` /
  `dock_sections_disjoint` invariants + 9 unit tests.
- Right dock split: Outliner/Inspector independent panes, draggable separator
  (`right_dock_split` 0.25–0.75), independent collapse, independent scroll, no
  more 280px outliner cap (`draw_body` fills parent).
- Findings fixed along the way:
  - `allocate_*` advances cursor by `item_spacing.y` per section: 3 dock
    allocations overflowed the panel by 12px into the status bar. Dock stacks
    with `item_spacing.y = 0`, interiors restore the inherited value.
  - egui `Rect::intersects` uses `<=`: adjacent tiled panes always "overlap".
    Invariants require positive-area intersection (>0.5px).
  - Detached inspector `max_size` used `.max()` after subtraction and could
    exceed small viewports; now `min(requested, available)` (§9.4).
- Persistence scope: `UiState` session fields (no prefs-file infra exists in
  repo; disk persistence noted as Settings Wave 5 follow-up).
- Gates: 111 lib + 24 kittest green (4 new structural dock tests at fixed
  1280×800), workspace clippy clean, arch-check green, docs regenerated.

## Wave 3 close-out (2026-09-15)

- `WorkspaceLayoutProfile` (`crates/ui/src/workspaces.rs`): tabela canônica
  Model/Paint/Uv/Animate (paleta, centro, painel inferior, shelf) + 4 testes.
- Transição com memória: `AppState::switch_workspace` (dono único §3.2) salva e
  restaura split/aba/colapsos por workspace; pílulas do header roteiam por ele;
  3 testes de unidade no core.
- Animate real: paleta de pose na toolbar (Select + trio Move/Rotate/Scale via
  helper compartilhado, sem ferramenta fictícia) + faixa de Timeline de 56px
  dentro da área central + painel de animação no inspector (já existia).
- UV real: editor UV interativo (existente, antes espremido no inspector) no
  centro + prévia 3D lado a lado; janela <760px alterna em vez de esmagar;
  inspector mostra só resumo (`draw_uv_summary`). Canvas preenche o pai.
- Findings:
  - `timeline::draw` criava `Panel::bottom` próprio mas nunca era chamado no
    `draw()` — o Animate não tinha timeline visível. Causa raiz do "Animate
    idêntico ao Model".
  - Dois `Panel::bottom` empilhados deslocam o segundo em ~11px (sharp edge do
    egui, reproduzível nas duas ordens). Timeline vive como faixa fixa central
    (`animate_workspace_center`, soma exata); a status bar segue como único
    painel bottom do shell.
- Gates: core 76+ lib/integração, ui 115 lib + 27 kittest (3 novos de workspace),
  workspace clippy clean, arch-check verde, `docs-generate` (chaves `uv.preview_3d`,
  `uv.faces` en+pt-BR).

## Wave 4 close-out (2026-09-15)

- Shelf como dados (`contextual_shelf.rs`): `ShelfCommand` (ícone, rótulo,
  tooltip, `ShelfPriority`, `ShelfAction`) + widgets finos de largura fixa.
- Sem estimativa: pílulas medidas por galley (`pill_width` idêntica na medição
  e na renderização); cápsula com largura exata do conteúdo — fim do "fundo
  menor que os elementos".
- Modos determinísticos por frame (Full → Compact → Overflow → Pill → Hidden),
  sem feedback de frame anterior; "More…" com `PetuniaMenuItem` reaproveitado.
- Menus (`widgets.rs`): `menu_row_width` por galley + `MENU_MIN/MAX_W`
  (120/320) nos 3 tipos de item; popup acompanha o conteúdo.
- ComboBox: inspector com `available.clamp(96, 180)`; toolbar já tinha
  larguras locais (62/88/116) — mantidas.
- Chaves novas: `tools.eraser/picker/uv_*`, `paint.color`, `ui.more/tools_menu`,
  `[animate]` (7) en+pt-BR. Restante hardcoded da shelf vai para Wave 7.
- Gates: ui 117→122 lib + 28→30 kittest (modos narrow/medium/tiny + menu),
  clippy limpo.

## Wave 5 close-out (2026-09-15)

- Reference Manager: grade determinística (`grid_columns`, `grid_card_width`,
  1–3 cols, testes), header empilha <560px, cartões com zonas fixas
  (header/prévia 140/status) + ajuste fino colapsável, `✕` no lugar do 🗑,
  títulos via `refs.*`, janela com `modal_sizes`.
- `regions::modal_sizes` (dono único §9.4): aplicado em Settings, Reference
  Manager, Asset Library, Recovery; paleta com clamp seguro.
- Texturas de ref: chave nome+dims+len (sem pixels obsoletos) + poda de
  removidas (sem leak de VRAM de UI).
- Settings: abas Interface (shelf toggle real + 2 resets reais §15.7) e
  Import/Export (`export_gltf` real); sem placebo (Performance/Autosave sem
  contraparte real ficam de fora — ledger).
- Sem escopo: glyphs restantes (👁📁↺) → Wave 6; i18n residual → Wave 7;
  prefs em disco → follow-up (sem infra no repo).

## Wave 6 close-out (2026-09-15)

- Serviço canônico único: `IconRegistry::paint_pack` resolve `PetuniaIcon` →
  glifo REAL do pacote (iconflow) para 38 ícones utilitários/chrome, arte de
  domínio Petunia (vetor/PNG) para ferramentas, Phosphor como fallback,
  losango vetorial como último recurso. Fim da renderização fictícia (todos os
  pacotes usavam glifos Phosphor).
- Tabela de nomes por pacote verificada por teste (dumps de `iconflow::list`):
  lucide/iconoir 100% auditados; tabler/phosphor com candidatos + aliases —
  o mesmo teste de auditoria roda no CI com `extended-icon-packs`.
- Fontes: `install_fonts` (iconflow) + phosphor em `ensure_fonts`; pintura por
  família nomeada com guarda `icon_fonts_ready` (pass boundary) — sem pânico
  em ctx fresco, sem deadlock (pass lido fora do lock de dados).
- Settings: só pacotes compilados selecionáveis; tabler/phosphor sem feature
  mostram nota honesta; prévia com utilitários; sem promessa de download.
- `PetuniaIconButton` (§13.2): botão só-ícone canônico (tooltip, widget_info,
  foco, selected, disabled). Emoji erradicado do código UI (timeline,
  status, dock, refmgr, animation, paint, uv, settings, paleta, viewport).
  Restam só glifos tipográficos (› ✓ … + – ✕ ↑↓ ⚠); Wave 7 revisa tooltips.
- Escopo documentado: ferramentas de domínio NÃO seguem o pacote (regressão
  de design); PNGs das abas de properties permanecem (arte raster sem fonte
  SVG); `app_icons.rs` sem nenhum uso → candidato à remoção na Wave 9.
- Gates: ui 124 lib + 30 kittest, clippy limpo (após corrigir clone_on_copy e
  deadlock de lock em `ensure_fonts`).
- Gates: ui 122 lib + 30 kittest, workspace clippy limpo, arch-check verde,
  `docs-check` (VitePress build OK).

## Wave 6 close-out (2026-09-15)

- Serviço canônico único: `IconRegistry::paint_pack` resolve `PetuniaIcon` →
  glifo REAL do pacote (iconflow) para 38 ícones utilitários/chrome, arte de
  domínio Petunia (vetor/PNG) para ferramentas, Phosphor como fallback,
  losango vetorial como último recurso. Fim da renderização fictícia.
- Tabela de nomes verificada por dumps de `iconflow::list` + teste de
  auditoria (38 ícones × pacotes habilitados; CI cobre tabler/phosphor).
- Fontes via `install_fonts` + guarda `icon_fonts_ready` (fronteira de pass;
  sem pânico em ctx fresco, sem deadlock com locks do egui).
- Settings: só compilados selecionáveis + notas honestas + prévia de
  utilitários. `PetuniaIconButton` canônico (§13.2). Emoji erradicado do
  código UI (restam glifos tipográficos › ✓ … + – ✕ ↑↓ ⚠).
- Escopo: domínio NÃO segue pacote (decisão de design); PNGs das abas ficam
  (sem fonte SVG); `app_icons.rs` sem uso → Wave 9.
- Gates: ui 124 lib + 30 kittest, workspace clippy limpo.

## Wave 7 close-out (2026-09-15)

- `TextId` + catálogo (`text_id`, 54 ids) + `t_id` + 64 call sites das Waves
  2–6 migrados; chaves pré-existentes seguem compatíveis.
- Paridade CI: `locale_parity_en_ptbr` + `text_id_catalog_resolves` com TOMLs
  embutidos (determinístico em qualquer CWD).
- Pseudo-locale (`I18n::pseudo_from`, +40% com acentos): full draw 1280+700
  mantém invariantes de regiões — pega clipping real.
- Switch en↔pt-BR testado com redesenho; locales embutidos como reserva
  (binário instalado sem dir de locales nunca exibe chaves).
- Cobertura: hint_keys de todas as ferramentas traduzidos; shelf com tooltips;
  Tab alcança controles do Settings; matriz de escala 1.0–1.75.
- Chaves novas: `animate.tip_*` (5), `ui.floating_inspector/redock/at_3d_cursor`.
- HONESTO / NÃO ATINGIDO: migração 100% dos literais. Backlog inventariado
  (literais visíveis aparentes por arquivo): properties_panel ~60,
  animation_ui ~35, viewport_bar ~30, toolbar ~25, outliner ~25, settings
  descrições ~20, asset_browser ~15, main_header ~15, paint_ui ~10, uv_ui ~8,
  status_bar ~8, outros ~15. Mecanismo (TextId + paridade + pseudo) é o caminho;
  cada arquivo vira tarefa incremental com o pseudo-teste como rede.
- Gates: config 9, core 76+, ui 125 lib + 36 kittest, workspace clippy limpo,
  arch-check verde, docs regenerados.

## Wave 8 close-out (2026-09-15)

- `PrimitiveCreationSession` no Core (`primitive_session.rs` + métodos em
  `AppState`): 6 espécies com parâmetros, regeneração sem checkpoint, UMA
  transação de undo (testado: N edições = 1 checkpoint), Esc cancela e remove,
  validade por asset+topo-do-undo (op interveniente finaliza sozinha).
- Cartão Last Operation (`primitive_card.rs`): âncora no bbox projetado +
  `constrain_to`, Enter/Esc só com hover (Esc global nunca apaga), clique na
  viewport confirma, reabertura explícita na shelf (F9 global fora de escopo:
  sem pipeline de atalhos para ele).
- Shelf/outliner roteiam criação pela sessão (Cone/Capsule diretos por ora —
  cartão cobre as 4 do exit gate + Cone/Capsule de brinde no core).
- Gates: 7 testes de sessão + 1 kittest; clippy limpo.

## Wave 9 close-out (2026-09-15)

- Removidos: `tiles_workspace.rs` (pilot inativo; split próprio venceu na Wave 2),
  `app_icons.rs` (0 usos), PNGs inalcançáveis da toolbar (~2MB de binário),
  dep `egui_tiles`, teste kittest do tiles; READMEs atualizados.
- Mantidos com veredito: `flex_layout`/`inbox_bridge` (exigidos pelo arch-check
  P1 como adapters testados) e PNGs das abas (sem fonte SVG).
- CHANGELOG com seção Unreleased da remediação.
- AMBIENTE: volume `/home/raillen/Documentos` chegou a 100% (target/ com 91GB
  de acúmulo) — `cargo clean` liberou 94GB; o "linker bus error" era falta de
  disco, não RAM. Rebuild limpo + suíte total: **902 passed, 0 failed**
  (367 lib + 535 integração, bins incluídos).
- Gates finais: fmt --check, clippy workspace `-D warnings`, cargo-deny,
  arch-check, docs-check (VitePress build) — todos verdes.
