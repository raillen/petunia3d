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
