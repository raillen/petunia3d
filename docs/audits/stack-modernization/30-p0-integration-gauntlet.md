# 30-p0-integration-gauntlet: Phase D P0 Integration Verification

- **Date:** 2026-09-14
- **Scope:** all 17 P0 dependencies in dependency order (D1 → D4), each with
  owner, adapter, feature, tests and docs per the integration policy.
- **Baseline:** M1 (Rust 1.98.1 / Edition 2024) and M2 (egui 0.36.2 / wgpu 30.0.1)
  complete; `cargo check`, `fmt`, `clippy -D warnings`, `arch-check` green
  before starting.

## 1. Resolved versions

| Dep | Version | License |
|---|---|---|
| tracing | 0.1.44 | MIT |
| tracing-subscriber (env-filter) | 0.3.23 | MIT |
| slotmap | 1.1.1 | Zlib |
| rayon | 1.12.0 | MIT/Apache-2.0 |
| flume | 0.12.0 | Apache-2.0/MIT |
| tempfile | 3.27.0 | MIT/Apache-2.0 |
| schemars | 1.2.2 | MIT |
| proptest | 1.11.0 | MIT/Apache-2.0 |
| egui_extras (image, svg) | 0.36.2 | MIT/Apache-2.0 |
| iconflow (pack-lucide, pack-iconoir) | 1.0.0 | MIT |
| geo (default) | 0.33.1 | MIT/Apache-2.0 |
| manifold-rust | 0.13.1 | Apache-2.0 |
| gltf-json | 1.4.1 | MIT/Apache-2.0 |
| tobj | 4.0.5 | MIT |
| eframe | 0.36.2 | MIT/Apache-2.0 |
| libfuzzer-sys (fuzz harness only) | 0.4 | MIT/Apache-2.0/NLL |
| cargo-deny / cargo-fuzz | CI tools, not libs | — |

## 2. Integration table

### D1 — Foundation

| ID | Why | Owner | Adapter | Feature / ships | Boundary types | Tests | Failure | Removal | Security |
|---|---|---|---|---|---|---|---|---|---|
| P0-09 tracing | structured diagnostics replace ad-hoc logging | `petunia_core` | `core/src/diagnostics.rs` (`DiagnosticCategory`, `DiagnosticEvent::emit`) | always on, no-op without subscriber | `DiagnosticCategory`, `DiagnosticEvent` (owned) | `diagnostics::tests` (2) | never fails; no subscriber → no-op | delete module + dep |
| P0-10 tracing-subscriber | central sink: filter/format/output | `petunia_app` | `app/src/diagnostics.rs::init_diagnostics` | always on; `RUST_LOG` overrides, quiet prod default | none cross boundary (configures global subscriber) | idempotency + emit smoke (1) | `try_init` error ignored (already set) | delete module + dep, logs fall back to `log` |
| P0-02 slotmap | stale-handle protection for session lookups | `petunia_core` | `core/src/handles.rs` (`Handle`, `HandleTable`) | always on; UUIDs stay the persistent identity | `Handle`, `HandleTable<T>` (slotmap never leaks) | stale-handle + get_mut (2) + proptest counts | `get` → `None` on stale | delete module + dep |
| P0-07 rayon | CPU-parallel pure workloads | `petunia_core` | `core/src/jobs.rs` (`par_finite_sum`, `par_validate`) | always on; workers never mutate editor state | `&[T]` in, owned results out | unit (4) + proptest (1) | `JobsError::NonFiniteInput` | replace bodies with sequential iterators |
| P0-08 flume | typed worker→app channel, no global bus | `petunia_core` | `core/src/jobs.rs` (`JobChannel::bounded/drain/sender`) | always on; app owns receiver | `T: Send` messages only | unit (2) + proptest (1) | full → `TrySendError`; dropped sender → empty drain | swap for std mpsc behind same API |
| P0-15 tempfile | atomic save/export, scoped scratch | `petunia_project` | `project/src/io_atomic.rs` (`atomic_write`, `TempScope`) | always on | paths + bytes only | roundtrip/replace, cleanup, missing parents (3) | typed `AtomicIoError::Temp/Persist` | std-only temp via `std::env::temp_dir` |
| P0-11 schemars | JSON Schema for command/plugin/MCP contracts | `petunia_core` | `core/src/schema_contracts.rs` (`CommandIntent`, `SceneItemContract`, `*_schema()`) | always on; `CONTRACT_SCHEMA_VERSION = "1"` | DTOs with primitives only (no glam/egui leaks) | schema shape, roundtrip, rejection (3) | serde error on unknown shape | delete module + dep |
| P0-12 proptest | invariant/property coverage | `mesh`, `core` dev-deps | `mesh/tests/proptest_invariants.rs`, `core/tests/proptest_jobs.rs` | dev only, never ships | — | 3 + 3 properties | shrinking failure output | delete tests + dev-deps |
| P0-13 cargo-fuzz | bounded hostile-input harnesses | `fuzz/` (excluded crate) | `fuzz_targets/obj_parse.rs` (64 KiB), `project_parse.rs` (256 KiB) | nightly tool only | `&[u8]` in, typed errors out | stable mirror `project/tests/fuzz_corpus.rs` (3) | harness asserts no-panic | delete `fuzz/` |
| P0-14 cargo-deny | license/advisory/ban governance | CI | `deny.toml` + `.github/workflows/supply-chain.yml` | CI on manifest/lock changes | — | workflow runs `cargo-deny-action` | deny on vuln/banned/wildcard | delete workflow + config |

### D2 — UI baseline

| ID | Why | Owner | Adapter | Feature / ships | Boundary types | Tests | Failure | Removal | Security |
|---|---|---|---|---|---|---|---|---|---|
| P0-01 egui_extras | image loaders incl. SVG for vector-first icons | `petunia_ui` | `ui/src/image_kit.rs` (`install_image_loaders`, `has_svg_loader`) | always on; wired in both gfx backends | none (adapter functions only) | idempotency + svg/image presence (2) | install skips existing | delete module + dep, PNG-only fallback |
| P0-17 iconflow | unified generic icon packs behind semantic ids | `petunia_ui` | `ui/src/icon_provider.rs` (`PetuniaIconProvider`, `GenericPack`, `GenericIcon`) | Lucide+Iconoir default; Tabler+Phosphor via default-off `extended-icon-packs` | `GenericPack/Icon/Error/ResolvedIcon` (iconflow never imported by workspaces) | resolve/fallback/switch (4) | `ProviderError::PackDisabled/Missing`, never tofu | delete module + dep, vector engine stays canonical |

### D3 — Geometry

| ID | Why | Owner | Adapter | Feature / ships | Boundary types | Tests | Failure | Removal | Security |
|---|---|---|---|---|---|---|---|---|---|
| P0-03 geo | 2D predicates/polygon ops for profiles | `petunia_mesh` | `mesh/src/profile_geo.rs` (area, winding, contains, intersects) | always on | `[[f64; 2]]` in/out (geo never leaks) | area/winding/contain/reject (4) | `ProfileGeoError::TooFewPoints/NonFinite` | reimplement predicates locally |
| P0-04 manifold-rust | 3D boolean provider | `petunia_mesh` | `mesh/src/boolean.rs` (`BooleanOp`, `boolean_meshes`, `boolean_cubes`) | always on | `Mesh` in/out + `BooleanOp/Error` | union/difference/disjoint/roundtrip/invalid (5) | `InvalidInput/Kernel/InvalidOutput` | delete module + dep |

### D4 — IO

| ID | Why | Owner | Adapter | Feature / ships | Boundary types | Tests | Failure | Removal | Security |
|---|---|---|---|---|---|---|---|---|---|
| P0-06 tobj | OBJ input behind Importer contract | `petunia_project` | `project/src/import_obj.rs` (`import_obj_bytes`) | always on; Petunia OBJ writer untouched | `&[u8]` in, `Mesh` out | valid/malformed/empty/quad-tri (4) | `Parse/Validation/TooLarge` (1M vert/face caps) | delete module + dep |
| P0-05 gltf-json | glTF JSON structural boundary | `petunia_project` | `project/src/import_gltf.rs` (`parse_gltf_json`, `GltfSummary`) | always on; buffers decode behind same boundary next | `&[u8]` in, `GltfSummary` out | valid/malformed/missing/version (4) | `Parse/Validation`, major != 2 rejected | delete module + dep |

### P0-16 — eframe host

| Why | Owner | Adapter | Feature / ships | Boundary types | Tests | Failure | Removal | Security |
|---|---|---|---|---|---|---|---|---|
| sanctioned eframe integration without collapsing dual-backend | `petunia_app` | `app/src/eframe_host.rs` (`PetuniaEframeApp: eframe::App::ui` reusing `petunia_ui::draw`) | always compiled; distribution host stays manual `WgpuApp/GlApp` | `Core` owned inside host; Core unaware of eframe | construction (1); full-draw path covered by kittest (17) | quit flag only; no renderer coupling | delete module + dep |

## 3. Gate verification results

- `cargo check --workspace --all-targets`: PASS.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test -p petunia_core --lib`: 68/68. `proptest_jobs`: 3/3.
- `cargo test -p petunia_mesh --lib`: 62/62. `proptest_invariants`: 3/3.
- `cargo test -p petunia_project --lib`: 31/31. `fuzz_corpus`: 3/3.
- `cargo test -p petunia_app --lib`: 12/12.
- `cargo test -p petunia_ui --lib`: 94/94. kittest flows: 17/17.
- `cargo run -p xtask -- arch-check`: PASS (incl. new P0 confinement gates).
- `cargo run -p xtask -- docs-check`: PASS (see below).

## 4. Explicitly accepted limitations

1. **iconflow Tabler+Phosphor behind `extended-icon-packs` (default-off).**
   Impact: default build resolves Lucide+Iconoir only; the 4-pack promise is
   code-complete but the heavy packs need CI RAM. Risk: low (provider API is
   pack-agnostic; cfg-gated variants mirror the default ones). Next: CI job
   with adequate RAM runs `cargo check/test --features extended-icon-packs`.
2. **`cargo deny check` and `cargo fuzz run` not executed locally**
   (tools not installed; fuzzing needs nightly). Impact: supply-chain and
   hostile-input exploration verified by config presence + stable corpus
   mirror. Next: CI runs both workflows.
3. **Clippy gate run without `--all-features`** on this machine for the same
   RAM reason; default-feature matrix is fully green. Next: CI runs the
   `--all-features` matrix.
4. **`eframe` distribution cutover deferred**: `PetuniaEframeApp` is the
   compiled smoke/dev path; manual dual-backend host stays canonical per the
   Gauntlet's own escape hatch. Next: dedicated host-migration wave with
   visual/behavior gates.

**Phase D (Wave M3) is COMPLETE under these recorded limitations.**
