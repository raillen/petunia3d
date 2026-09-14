> [← Voltar ao Índice de Modernização](/modernization/) | [Documento Integral Monolítico](/modernization/full-gauntlet)

# 7. P0→P3 dependency integration policy

**All libraries below must be addressed.**

However, “integrated” does not mean “unconditionally enabled everywhere.”

The correct integration depends on class:

| Class | Cargo/default behavior |
|---|---|
| Production baseline | direct dependency in the owning crate/workspace and exercised by real product paths |
| Production support | direct dependency only in the boundary that needs it |
| DEV TOOL | dev dependency or default-off `dev-*` feature; disabled in distribution |
| OPTIONAL | default-off or only enabled by a real product feature |
| SECURITY CANDIDATE | isolated behind explicit security/provider boundary |
| FUTURE | default-off feature with working adapter/smoke test; must not block GA |
| PERF CANDIDATE | default-off/provider until benchmarks prove activation |

For every integration, document:

```text
Why this dependency exists
Who owns it
What adapter/provider isolates it
Which feature enables it
Whether it ships by default
What data/types may cross its boundary
What tests prove it works
How it fails
How it is removed/replaced
License
Version
Security considerations
```


## P0 integration matrix

### P0-01 — `egui_extras`
- **Adoption class:** Production baseline / UI adapter
- **Intended owner/boundary:** `petunia-ui`
- **Required integration contract:** Use only behind Petunia adapters/components for image loaders, tables, strips, and layout helpers. Never expose egui_extras types to Core/Application APIs.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P0-02 — `slotmap`
- **Adoption class:** Production baseline / typed generational IDs
- **Intended owner/boundary:** `core/domain`
- **Required integration contract:** Introduce typed generational handles where object/resource identity benefits from stale-handle protection. Do not replace UUIDs used for persistent/external identity without an explicit migration strategy.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P0-03 — `geo`
- **Adoption class:** Production baseline / 2D geometry
- **Intended owner/boundary:** `geometry/profile`
- **Required integration contract:** Use behind Petunia profile/planar geometry abstractions for predicates, polygon operations, intersections, and booleans where appropriate. Keep public domain types Petunia-owned.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P0-04 — `manifold-rust`
- **Adoption class:** Production baseline / 3D boolean provider
- **Intended owner/boundary:** `geometry/boolean provider`
- **Required integration contract:** Isolate behind a BooleanProvider-style boundary. Convert Petunia mesh -> provider input -> validated Petunia mesh. Never leak provider mesh types through the domain.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P0-05 — `gltf-json`
- **Adoption class:** Production baseline / glTF IO
- **Intended owner/boundary:** `import-export`
- **Required integration contract:** Promote glTF support from test-only experimentation to modular production import/export. Keep importer/exporter headless and independent from file dialogs/UI.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P0-06 — `tobj`
- **Adoption class:** Production baseline / OBJ import
- **Intended owner/boundary:** `import-export`
- **Required integration contract:** Use for OBJ input behind the Importer contract. Keep Petunia-controlled OBJ writing if that remains simpler and more deterministic.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P0-07 — `rayon`
- **Adoption class:** Production baseline / CPU jobs
- **Intended owner/boundary:** `jobs/compute`
- **Required integration contract:** Use for CPU-parallel pure workloads such as validation, heavy geometry, UV, thumbnails, and export preprocessing. Do not mutate editor state from Rayon workers.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P0-08 — `flume`
- **Adoption class:** Production baseline / channels
- **Intended owner/boundary:** `jobs/application boundary`
- **Required integration contract:** Use typed channels for worker/service -> single-writer application communication. No global event bus and no `Arc<Mutex<Everything>>` architecture.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P0-09 — `tracing`
- **Adoption class:** Production baseline / diagnostics
- **Intended owner/boundary:** `cross-cutting diagnostics`
- **Required integration contract:** Replace ad-hoc application logging with structured events/spans and stable categories. Domain errors remain typed; logs never replace user-facing diagnostics.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P0-10 — `tracing-subscriber`
- **Adoption class:** Production support / diagnostics sink
- **Intended owner/boundary:** `app/diagnostics`
- **Required integration contract:** Configure filters/format/output centrally. Keep production defaults quiet and useful; allow dev verbosity without recompiling business logic.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P0-11 — `schemars`
- **Adoption class:** Production baseline / JSON Schema
- **Intended owner/boundary:** `commands/plugins/MCP`
- **Required integration contract:** Generate schemas from Petunia-owned DTOs/contracts for Commands, plugins, MCP, package/config boundaries where stable schemas are valuable.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P0-12 — `proptest`
- **Adoption class:** Development baseline / property tests
- **Intended owner/boundary:** `mesh/core/tests`
- **Required integration contract:** Add invariant/property tests for topology, serialization, IDs, transforms, command reversibility, and geometry properties where suitable.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P0-13 — `cargo-fuzz`
- **Adoption class:** Development baseline / fuzzing
- **Intended owner/boundary:** `fuzz targets`
- **Required integration contract:** Create bounded fuzz targets for loaders, project/package parsing, geometry boundaries, malformed external inputs, and other trust boundaries.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P0-14 — `cargo-deny`
- **Adoption class:** CI baseline / supply-chain governance
- **Intended owner/boundary:** `CI`
- **Required integration contract:** Enforce license allowlist, advisories, banned/duplicate dependency policy as appropriate, and approved sources. The Petunia project license constraint remains MIT-compatible/permissive.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P0-15 — `tempfile`
- **Adoption class:** Production/dev support / atomic IO
- **Intended owner/boundary:** `project/io`
- **Required integration contract:** Use safe temporary files/directories for atomic save/export, recovery, cancellation cleanup, and filesystem tests.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P0-16 — `eframe`
- **Adoption class:** Production baseline / desktop host
- **Intended owner/boundary:** `app/frontend host`
- **Required integration contract:** Migrate or adapt the desktop host to eframe 0.36.2 without collapsing renderer boundaries. WGPU remains the canonical primary path; OpenGL fallback must not be silently removed. If host migration conflicts with explicit product requirements, document the architectural issue and solve it behind a narrow host adapter rather than leaking host concerns into Core.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.


### P0-17 — `iconflow`
- **Adoption class:** Production baseline / unified generic icon provider
- **Intended owner/boundary:** `petunia-ui/adapters/icons`
- **Required integration contract:** Replace ad-hoc/manual third-party icon-pack acquisition with a single `PetuniaIconProvider` backed by `iconflow`. Enable the Lucide, Tabler, Iconoir, and Phosphor packs required by the existing Petunia customization promise. Resolve icons by semantic `IconId`, never by random glyph/name in workspace code. Petunia-owned domain icons remain Petunia assets and should be SVG-first. Do not enable `all-packs` by default unless a measured product requirement justifies binary-size cost.
- **Vector policy:** `iconflow` uses embedded icon fonts generated from the upstream SVG sources and therefore provides scalable/vector generic icons without downloading PNG copies. It does **not** eliminate the requirement that Petunia-owned/default domain icons be stored as canonical SVG assets.
- **Migration requirement:** replace the current canonical PNG toolbar/property icon path after screenshot/behavior parity; keep raster UI icons only as temporary migration fixtures until the new registry passes the icon Gauntlet.
- **Definition of integrated:** `PetuniaIconRegistry` can switch among the enabled packs; all generic toolbar/menu/settings icons resolve through `IconId`; missing icons have deterministic fallback/diagnostics; accessibility labels remain independent of glyphs; no workspace imports `iconflow` directly; obsolete PNG generic-icon code/assets are removed after parity; relevant visual tests pass.


## P1 integration matrix

### P1-01 — `criterion`
- **Adoption class:** Development baseline / benchmarks
- **Intended owner/boundary:** `benches`
- **Required integration contract:** Build representative benchmarks around real fixtures/hot paths; do not benchmark toy code and call it performance proof.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-02 — `zip`
- **Adoption class:** Production baseline / project package container
- **Intended owner/boundary:** `project/package`
- **Required integration contract:** Implement versioned project/package container rules with safe extraction, traversal protection, limits, deterministic metadata where needed, and migration tests.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-03 — `mlua`
- **Adoption class:** Production baseline / Lua 5.4 plugin host
- **Intended owner/boundary:** `plugins`
- **Required integration contract:** Create a capability-limited Lua host. Plugins must receive Petunia API contracts, never raw egui::Context, wgpu::Device, arbitrary application state, or unrestricted filesystem by default.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-04 — `rmcp`
- **Adoption class:** Production baseline / MCP
- **Intended owner/boundary:** `MCP boundary`
- **Required integration contract:** Implement the MCP server behind an application service boundary using Petunia Commands/Queries. MCP must not bypass permissions, undo semantics, validation, or trust boundaries.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-05 — `tokio`
- **Adoption class:** Boundary runtime / MCP and async IO
- **Intended owner/boundary:** `MCP/network boundary only`
- **Required integration contract:** Keep Tokio isolated to MCP/network/async boundaries. Do not turn the whole editor ownership model into async or expose Tokio types through Core.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-06 — `xatlas-rs`
- **Adoption class:** Production fallback provider / UV
- **Intended owner/boundary:** `UV provider`
- **Required integration contract:** Use only behind a UV unwrap provider. Petunia native deterministic/projection workflows remain first-class; xatlas is the generic fallback.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-07 — `egui_inspection`
- **Adoption class:** DEV TOOL / UI inspection
- **Intended owner/boundary:** `petunia-ui/devtools`
- **Required integration contract:** Enable only in development/test builds. Expose accessibility tree, input/screenshot/resize inspection for automation without shipping remote inspection enabled in distribution builds.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-08 — `egui_mcp`
- **Adoption class:** DEV TOOL / agent UI automation
- **Intended owner/boundary:** `petunia-ui/devtools`
- **Required integration contract:** Integrate with the Gauntlet UI loop so agents can inspect/interact/capture screenshots. Bind to loopback/dev configuration only; never silently expose in production.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-09 — `egui_probe`
- **Adoption class:** DEV TOOL / state inspector
- **Intended owner/boundary:** `petunia-ui/devtools`
- **Required integration contract:** Use for debug/probe panels over selected debug DTOs; do not make product UI depend on derived debug inspectors.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-10 — `puffin`
- **Adoption class:** DEV TOOL / profiling instrumentation
- **Intended owner/boundary:** `profiling`
- **Required integration contract:** Instrument selected CPU/UI hot paths in development builds with low intrusion.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-11 — `puffin_egui`
- **Adoption class:** DEV TOOL / embedded profiler UI
- **Intended owner/boundary:** `petunia-ui/devtools`
- **Required integration contract:** Provide a Petunia devtools wrapper around the embedded profiler only if the release is compatible with egui 0.36.x; otherwise patch/fork behind the adapter with documented rationale.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-12 — `notify`
- **Adoption class:** Production support / file watching
- **Intended owner/boundary:** `filesystem/watch service`
- **Required integration contract:** Use for theme/icon/translation/plugin/assets reload and future Live Asset Link. Debounce/coalesce events and revalidate files before applying.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-13 — `assert_fs`
- **Adoption class:** DEV TOOL / filesystem tests
- **Intended owner/boundary:** `tests`
- **Required integration contract:** Use for readable file-tree fixtures and assertions around import/export/save/recovery/security.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-14 — `insta`
- **Adoption class:** DEV TOOL / snapshots
- **Intended owner/boundary:** `tests`
- **Required integration contract:** Use snapshots only for stable structured outputs: diagnostics, schemas, manifests, serialization samples, token references. Do not snapshot volatile noise.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-15 — `egui_taffy`
- **Adoption class:** BASELINE CANDIDATE / advanced responsive sublayout
- **Intended owner/boundary:** `petunia-ui/adapters/layout`
- **Required integration contract:** Pilot Flex/Grid/Block for complex responsive sublayouts. It must not replace egui_tiles as the controlled macro-layout engine unless an ADR proves the need.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-16 — `egui_inbox`
- **Adoption class:** BASELINE CANDIDATE / worker-to-UI bridge
- **Intended owner/boundary:** `petunia-ui/adapters/jobs`
- **Required integration contract:** Bridge controlled worker results/callbacks to egui and request repaint without making UI own domain concurrency.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-17 — `egui_commonmark`
- **Adoption class:** OPTIONAL / in-app help and release notes
- **Intended owner/boundary:** `petunia-ui/adapters/markdown`
- **Required integration contract:** Render Markdown help/release notes behind PetuniaHelp/PetuniaMarkdown adapter. No Markdown renderer types leak outside UI.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-18 — `egui_autocomplete`
- **Adoption class:** OPTIONAL / command palette and search
- **Intended owner/boundary:** `petunia-ui/components/search`
- **Required integration contract:** Use behind PetuniaCommandPalette/PetuniaSearchBox when it meaningfully improves completion without owning command semantics.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P1-19 — `egui_hotkey`
- **Adoption class:** OPTIONAL / keybind capture UI
- **Intended owner/boundary:** `petunia-ui/settings`
- **Required integration contract:** Use only to capture/edit key combinations. CommandId and the Petunia keymap remain authoritative; widgets never hard-code command execution.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.


### P1-20 — `twill`
- **Adoption class:** BASELINE CANDIDATE / type-safe design-token and styling infrastructure
- **Intended owner/boundary:** `petunia-ui/foundation/theme` and `petunia-ui/adapters/theme`
- **Required integration contract:** Use Twill only as an implementation aid for type-safe/composable styling and token/variant mapping. **PetuniaThemeTokens and Petunia Components remain the product API and source of semantic meaning.** Workspaces must never depend directly on Twill classes/types. Do not copy Tailwind aesthetics into the product.
- **Required evaluation:** prototype mapping of semantic Petunia color/spacing/radius/size tokens and representative component variants to the egui backend. Measure compile/runtime impact and verify egui 0.36 compatibility. If a small compatibility patch is needed, isolate it behind the theme adapter and document it.
- **Definition of integrated:** at least representative PetuniaButton, PetuniaNumberField, PetuniaPanel, and PetuniaTooltip variants are driven through the Petunia token layer with Twill assistance; raw styling values are reduced rather than duplicated; replacement remains possible inside the theme adapter; token-completeness tests pass.

### P1-21 — Theme/reference tooling policy (no visual takeover)
The following projects are **REFERENCE/MONITOR**, not product design systems: `egui_elements`, `egui-elegance`, `egui_sauge`, and `egui_colors`.

The agent must study them for:
- semantic palette organization;
- typography/spacing/radius models;
- live theme-editor ideas;
- component visual-state anatomy;
- WCAG/contrast testing approaches;
- density/theme-switch patterns.

Do **not** install their themes over Petunia or make Petunia Components wrappers around their visual identity. If a small dev-only theme-preview utility from one of them is adopted, keep it behind `dev-theme-tools`, separate from production themes, and document the exact purpose.


## P2 integration matrix

### P2-01 — `egui_dnd`
- **Adoption class:** OPTIONAL / list reorder
- **Intended owner/boundary:** `petunia-ui/adapters/dnd`
- **Required integration contract:** Use for reorderable lists/presets/assets where tree DnD does not already solve the problem. Centralize pointer capture/ownership semantics.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P2-02 — `egui_animation`
- **Adoption class:** OPTIONAL / motion
- **Intended owner/boundary:** `petunia-ui/foundation/motion`
- **Required integration contract:** Use behind Petunia motion tokens. Respect reduced/disabled motion and never make critical feedback understandable only through animation.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P2-03 — `egui_table`
- **Adoption class:** OPTIONAL / large virtualized tables
- **Intended owner/boundary:** `petunia-ui/adapters/table`
- **Required integration contract:** Use for Asset Validator, batch processing, reports, and large data sets. Do not replace simple tables unnecessarily.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P2-04 — `egui_form`
- **Adoption class:** OPTIONAL / form validation
- **Intended owner/boundary:** `petunia-ui/adapters/forms`
- **Required integration contract:** Use for genuinely complex settings/plugin forms. Validation rules remain Petunia/domain-owned where they affect semantics.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P2-05 — `egui_tool_windows`
- **Adoption class:** OPTIONAL / floating tool palettes
- **Intended owner/boundary:** `petunia-ui/adapters/tool_windows`
- **Required integration contract:** Use only for product-approved semi-floating palettes. It must not introduce unrestricted IDE docking or bypass Petunia layout constraints.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P2-06 — `egui-notify`
- **Adoption class:** OPTIONAL / transient notifications
- **Intended owner/boundary:** `petunia-ui/components/toast`
- **Required integration contract:** Place behind PetuniaToast/notification service. Critical errors must also have persistent/recoverable representation; never rely exclusively on ephemeral toasts.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P2-07 — `directories`
- **Adoption class:** Production support / platform paths
- **Intended owner/boundary:** `config/filesystem`
- **Required integration contract:** Centralize cross-platform config/cache/data paths for Windows/Linux. Never scatter OS path assumptions.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P2-08 — `semver`
- **Adoption class:** Production support / compatibility
- **Intended owner/boundary:** `plugins/packages/API`
- **Required integration contract:** Use explicit SemVer constraints for plugin API, packages, and other versioned contracts. Reject incompatible versions with structured diagnostics.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P2-09 — `camino`
- **Adoption class:** Support / UTF-8 paths where appropriate
- **Intended owner/boundary:** `serialized path boundaries`
- **Required integration contract:** Use only where UTF-8 path semantics are explicitly required. Keep std::path::Path/PathBuf at OS-facing boundaries where non-UTF8 must remain representable.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P2-10 — `cap-std`
- **Adoption class:** SECURITY CANDIDATE / capability filesystem
- **Intended owner/boundary:** `plugins/MCP sandbox`
- **Required integration contract:** Use for capability-oriented filesystem access for plugins/MCP. Default to least privilege; no ambient filesystem authority in plugin APIs.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

## P3 integration matrix

### P3-01 — `rstar`
- **Adoption class:** FUTURE/PERF CANDIDATE / spatial index
- **Intended owner/boundary:** `spatial query provider`
- **Required integration contract:** Integrate behind a feature/provider with representative benchmarks. Activate in production paths only when profiling shows benefit for snapping/picking/nearest-neighbor workloads.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P3-02 — `egui_timeline`
- **Adoption class:** FUTURE / animation timeline
- **Intended owner/boundary:** `future animation workspace`
- **Required integration contract:** Integrate behind a default-off future-animation feature with a working adapter/demo/test. Do not resurrect dead timeline UI in the default product before the Animation/Rig feature is ready.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P3-03 — `egui-snarl`
- **Adoption class:** FUTURE / node editor
- **Intended owner/boundary:** `future nodes workspace`
- **Required integration contract:** Integrate behind a default-off future-nodes feature and Petunia node adapter. Product node semantics remain Petunia-owned.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.

### P3-04 — `egui_code_editor`
- **Adoption class:** FUTURE / Lua/dev editor
- **Intended owner/boundary:** `future scripting/dev console`
- **Required integration contract:** Integrate behind a default-off future-script-editor feature with syntax/editor adapter. It must not become a prerequisite for the Lua plugin host.
- **Definition of integrated:** dependency/version resolved; ownership boundary implemented; at least one real call path or intentionally default-off feature path compiles; relevant tests exist; architecture checks pass; documentation states why/where it is used; there is no unused dependency or direct third-party leakage across forbidden layers.


---

# 8. Recommended Cargo feature architecture

Do not copy this mechanically if the audited architecture already has a better equivalent; preserve the intent.

A reasonable target feature map:

```toml
[features]
default = [
    "renderer-wgpu",
]

renderer-wgpu = []
renderer-opengl = []

plugins-lua = []
mcp = []
uv-xatlas = []

dev-inspection = []
dev-ui-mcp = []
dev-probe = []
dev-profiler = []

ui-advanced-layout = []
ui-large-tables = []
ui-rich-forms = []
ui-floating-tools = []
ui-motion = []

icon-pack-lucide = []
icon-pack-tabler = []
icon-pack-iconoir = []
icon-pack-phosphor = []
dev-theme-tools = []

fs-capabilities = []
spatial-rstar = []

future-animation = []
future-nodes = []
future-script-editor = []
```

Rules:

- Core must not use UI features.
- Product release CI must explicitly test the default feature set.
- CI must test meaningful optional feature combinations.
- `--all-features` is useful but insufficient because mutually exclusive renderers/configurations may need separate jobs.
- Dev remote-control features are never default.
- P3 future UI must never appear merely because `cargo build` was run.

---
