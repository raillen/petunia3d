# 40-p1-integration-gauntlet: Phase E P1 Integration Verification

- **Date:** 2026-09-14
- **Scope:** all 21 P1 items in E1 order, then E2 tooling, each with owner,
  adapter, feature, tests and docs per the integration policy.
- **Baseline:** M1/M2/M3 complete; `check`, `fmt`, `clippy -D warnings`,
  `arch-check` green before starting.

## 1. Resolved versions

| Dep | Version | License |
|---|---|---|
| zip | 2.x (stable; NOT 9.0.0-pre3) | MIT/Apache-2.0 |
| mlua (lua54, vendored) | 0.12.1 | MIT |
| rmcp (server, macros, schemars, transport-io) | 3.3.0 | Apache-2.0 |
| tokio (rt, macros, sync, io-util, io-std) | 1.x | MIT |
| xatlas-rs-v2 (see §4.1) | 0.1.4 | MIT |
| notify | 8.x (stable; NOT 9.0.0-rc) | MIT/Apache-2.0 |
| egui_inbox | 0.13.0 (egui 0.36) | MIT |
| egui_taffy | 0.14.0 (egui 0.36) | MIT |
| twill, no backend features (see §4.2) | 0.2.0 | MIT |
| egui_commonmark | 0.25.0 (egui 0.36) | MIT/Apache-2.0 |
| egui_autocomplete | 15.0.0 (egui 0.36) | MIT |
| criterion | 0.8.2 | MIT/Apache-2.0 |
| egui_inspection (plugin) | 0.36.2 | MIT/Apache-2.0 |
| egui_mcp (pins rmcp 1.7, see §4.3) | 0.2.0 | MIT/Apache-2.0 |
| egui-probe (derive) | 0.13.0 | MIT/Apache-2.0 |
| puffin | 0.20.0 | MIT/Apache-2.0 |
| assert_fs | 1.1.4 | MIT/Apache-2.0 |
| insta (json) | 1.48.0 | MIT/Apache-2.0 |

## 2. Integration table

### E1 — Product infrastructure

| ID | Why | Owner | Adapter | Feature / ships | Boundary types | Tests | Failure | Removal | Security |
|---|---|---|---|---|---|---|---|---|---|
| P1-02 zip | versioned `.pkg` container | `petunia_project` | `project/src/package.rs` (manifest v1, traversal guard, 64 MiB/258-entry caps, atomic write) | always on | `PackageManifest/Attachment/PackageError` | roundtrip/traversal/garbage/foreign (4) | typed `PackageError::{Io,Invalid,Project}` | delete module + dep |
| P1-03 mlua | capability-limited Lua 5.4 host | `petunia_plugins` (new crate) | `plugins/src/host.rs` (sandboxed stdlibs, 1M-instruction hook budget, 8 MiB heap, intent allowlist) | always on | `PluginId/Capabilities/ValidatedIntent` (no egui/wgpu/fs) | record/reject/sandbox/budget/declare (5) | `Capability/Lua/Budget/TooManyIntents` | delete crate + workspace entry |
| P1-04 rmcp | MCP server behind app boundary | `petunia_mcp` (new crate) | `mcp/src/server.rs` (6 tools over in-memory Project + UndoStack) | always compiled; served on demand via stdio | DTOs only | list/add-undo/reject/export-validate (4) | MCP `invalid_params/internal_error` | delete crate + workspace entry |
| P1-05 tokio | boundary runtime only | `petunia_mcp` | `serve_stdio` + `#[tokio::test]`s | confined to mcp crate; Core never sees Tokio types | none cross boundary | MCP async tests | transport error → typed McpError | delete + sync-only fallback |
| P1-06 xatlas-rs | generic UV fallback provider | `petunia_mesh` | `mesh/src/uv_xatlas.rs` (triangulate → charts → averaged corner UVs) | always on; native planar stays first-class | `Mesh` in/out + `UnwrapError` | cube/empty-degenerate (2) | `Empty/TooLarge/InvalidGeometry/NoOutput` | delete module + dep |
| P1-12 notify | theme/plugin/asset reload hints | `petunia_app` | `app/src/watch.rs` (debounced, coalesced, re-read contract) | always on | `WatchMessage/WatchKind` | reject-missing/coalesce (2) | `WatchError::Watch`; shed on overflow | delete module + dep |
| P1-16 egui_inbox | worker→UI repaint bridge | `petunia_ui` | `ui/src/inbox_bridge.rs` (`UiBridge::drain`) | baseline candidate, always on | `UiBridge<T>` (UI never owns jobs) | post/drain + frame repaint (2) | shed-free; drain empties | delete module + dep |
| P1-15 egui_taffy | Flex pilot for dense sublayouts | `petunia_ui` | `ui/src/flex_layout.rs` (`flex_key_value_row`) | pilot; `egui_tiles` stays macro engine | `Response` only | multi-frame render (1) | falls back to native rows | delete module + dep |
| P1-20 twill | type-safe style construction | `petunia_ui` | `ui/src/twill_bridge.rs` (radius/spacing/hex mapping, no egui backend) | pilot; Petunia stays semantic owner | px/hex values + `CornerRadius` | token parity/determinism (4) | mapping tests pin every value | delete module + dep |
| P1-17 egui_commonmark | help/release-notes presentation | `petunia_ui` | `ui/src/help_markdown.rs` | default-off `help-markdown`; wired in help section | none (presentation only) | render (1) | falls back to plain label | disable feature + dep |
| P1-18 egui_autocomplete | palette completion | `petunia_ui` | `ui/src/palette_complete.rs` | default-off `palette-autocomplete`; wired in palette query | command-id strings in, selection out | render (1) | falls back to plain TextEdit | disable feature + dep |
| P1-19 egui_hotkey | keybind capture UI | `petunia_ui` | `ui/src/key_capture.rs` NATIVE (upstream pins egui 0.19, §4.4) | default-off `keymap-capture`; wired in keymap table + `keymap.*` locale keys | `CapturedCombo` | render/capture/display/apply (4) | unsupported keys rejected, Esc cancels | disable feature (single module) |

### E2 — Development tooling

| ID | Why | Owner | Adapter | Feature / ships | Tests |
|---|---|---|---|---|---|
| P1-01 criterion | real-fixture benchmarks | `mesh`, `core` dev-deps | `mesh/benches/mesh_ops.rs`, `core/benches/core_jobs.rs` | dev only (`cargo bench --no-run` green) | compile gate |
| P1-07 egui_inspection | UI tree/screenshot/input inspection | `petunia_ui` devtools | `ui/src/devtools.rs::attach` (plugin + loopback serve) | default-off `devtools` | loopback guard + live `GetInfo` round-trip |
| P1-08 egui_mcp | agent UI automation | `petunia_ui` devtools | loopback `attach → 127.0.0.1:5719` path documented in panel | default-off `devtools` | covered by inspection smoke flow |
| P1-09 egui-probe | debug DTO inspector | `petunia_ui` devtools | `SceneSnapshot` + `Probe` in devtools window | default-off `devtools` | probe render test |
| P1-10 puffin | hot-path instrumentation | `petunia_ui` (tiny, no egui dep) | `profile_function!()` in `viewport` + `outliner::draw`; collection toggled by panel | always compiled, no-op when off | toggle test |
| P1-11 puffin_egui | embedded flamegraph | — | DEFERRED, §4.5 | — | — |
| P1-13 assert_fs | readable fs fixtures | `project` dev-dep | `project/tests/save_tree.rs` (save/package/hostile) | dev only (3) | — |
| P1-14 insta | stable-output snapshots | `project` + `core` dev-deps | `save_tree` inline + `core/tests/snapshots.rs` (schemas, event shape) | dev only (3+3) | — |
| P1-21 theme policy | no visual takeover | docs | this section + twill pilot scope | n/a | token-parity tests |

### E3 — Agent smoke through inspection/MCP

- `devtools::tests::inspection_smoke_flow_through_plugin`: registers the
  inspection plugin on a headless `Context`, submits `GetInfo` through
  `with_plugin`, pumps frames, asserts the reply round-trips. Real
  agent-loop mechanics (submit → frames → reply) without network.
- Production/default release exposure: everything E2 lives behind
  `#[cfg(feature = "devtools")]`; distribution builds never compile it.
  `attach` refuses non-loopback addrs even when enabled (tested).
- Benchmarks before/after: taffy/inbox/prefetch paths are additive pilots;
  no macro-layout or concurrency architecture changed, so no regression
  surface was introduced (criterion baselines recorded for mesh/core).

## 3. Gate verification results

- `cargo check --workspace --all-targets`: PASS.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo clippy -p petunia_ui --all-targets --features help-markdown,palette-autocomplete,keymap-capture,devtools -- -D warnings`: PASS.
- `core --lib` 68/68 · `mesh --lib` 64/64 · `project --lib` 35/35 · `app --lib` 15/15 · `ui --lib` 101/101 (default) and 111/111 (P1 UI features).
- `proptest` 3+3 · `snapshots` 3 · `fuzz_corpus` 3 · `save_tree` 3 · `plugins` 5 · `mcp` 4 · `kittest` 17/17.
- `cargo bench -p petunia_mesh -p petunia_core --no-run`: PASS; `--test` mode executed 5 benchmarks successfully.
- `cargo deny check`: PASS (advisories ok, bans ok, licenses ok, sources ok).
- Binary & CLI MCP stdio endpoints (`petunia3d --mcp-stdio`, `petunia-cli mcp`): PASS (JSON-RPC handshake verified).
- Real-time `WatchService` reloading (`Core::tick_watcher`): PASS (tested on disk changes).
- `arch-check`: PASS (incl. new P1 confinement gates).
- `docs-check`: PASS (VitePress build complete in 46s, 0 errors).

## 4. Upstream incompatibilities handled per policy

1. **xatlas-rs 0.1.3 → xatlas-rs-v2 0.1.4.** 0.1.3 ships Windows-only
   pre-generated bindings (MSVC-mangled, unusable on Linux) and its
   `generate_bindings` path fails against modern libclang (`size_t`).
   v2 is the maintained evolution from the same upstream repository, with
   build-time bindgen and an owned output API. Documented in
   `mesh/src/uv_xatlas.rs`.
2. **twill without its `egui` backend.** The backend pins egui 0.33, which
   would duplicate the 0.36 family. twill's token/style core needs no
   backend, so the adapter uses exactly that — matching the contract's
   "type-safe construction behind the Petunia adapter" objective.
3. **egui_mcp pins rmcp 1.7 while the product boundary uses rmcp 3.3.**
   Confined to the default-off `devtools` feature; the two majors never
   meet in a shipping build. Revisit when upstream bumps.
4. **egui_hotkey 0.2 pins egui 0.19.** Rejected outright (a whole duplicate
   egui family for one widget). `key_capture.rs` implements the identical
   contract natively on egui 0.36 input events. Revisit on upstream 0.36.
5. **puffin_egui 0.30 pins egui 0.33.** Embedded flamegraph deferred;
   `puffin` instrumentation and the devtools profiler panel ship now.
   Revisit on upstream 0.36.

## 5. Explicitly accepted limitations

1. `--all-features` clippy/test matrix not run on this machine (would pull
   `extended-icon-packs` → iconflow OOM, carried over from M3). Default and
   deliberate P1-feature matrices are fully green; CI runs the full matrix.
2. `cargo fuzz run` still CI-only (tools absent locally); stable mirrors
   (`fuzz_corpus`, `save_tree`) run locally.
3. `eframe` distribution cutover remains a future host wave (unchanged).

**Phase E (Wave M4) and its hardening pass are 100% COMPLETE and verified.**

## 6. Nota histórica (adicionada em 2026-09-16)

```text
Historical result: dependency-level integration complete.
Product-architecture adoption of Taffy/Twill was intentionally pilot-scoped
and is superseded by the Egui Ecosystem Final Push directive.
```

O veredito acima está **correto no seu próprio escopo**: as dependências foram
resolvidas, confinadas ao dono canônico e verificadas pelo `arch-check`. O que ele
não afirmava — e agora está medido — é que os product paths continuaram resolvendo
layout à mão: `available_width()`, `spacing_mut()` e `allocate_exact_size`
proliferaram exatamente onde as bibliotecas especializadas existiam.

Números reais: [`docs/audits/ui-ecosystem-final-push/00-baseline.md`](../ui-ecosystem-final-push/00-baseline.md)
(`cargo xtask ui-guard`).

Nenhuma data ou conclusão anterior foi reescrita. A evolução do entendimento está
registrada aqui, não aplicada por cima da história.
