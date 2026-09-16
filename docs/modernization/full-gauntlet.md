
# Petunia3D — Full Stack Modernization & P0→P3 Integration Gauntlet

> **Execution directive for a coding agent**
>
> Repository: `https://github.com/raillen/petunia3d`
>
> Mission: modernize the Rust/egui/wgpu foundation and correctly integrate every dependency classified P0 through P3, while preserving the exact Petunia3D product philosophy, architectural boundaries, design-system sovereignty, accessibility expectations, performance goals, and Gauntlet Loop discipline.
>
> **Primary language of implementation documentation: English.**
>
> This is an execution plan, not a brainstorming document. Audit first, implement incrementally, prove every claim with evidence, and continue the Gauntlet Loop until the wave exit criteria are objectively satisfied.

---

## 0. Authoritative target verified on 2026-09-14

Use these as the initial modernization target unless the repository has changed after this document was generated and a newer compatible stable patch release is available:

| Layer | Current audited state | Modernization target |
|---|---:|---:|
| Rust toolchain | not pinned in the audited root manifest; packages use Edition 2021 | **Rust 1.98.1 stable** |
| Rust Edition | `2021` | **2024** |
| egui | `0.32` | **0.36.2** |
| egui-winit | `0.32` | **0.36.2** |
| egui-wgpu | `0.32` | **0.36.2** |
| egui_glow | `0.32` | **0.36.2** |
| eframe | absent | **0.36.2** |
| wgpu | `25` | **30.0.1** |
| winit | `0.30` | remain on the **0.30.x** line compatible with egui 0.36.2; prefer the patch version resolved by the official egui 0.36.2 stack |
| AccessKit integration | through `egui-winit` feature | preserve and revalidate |
| OpenGL fallback | present through `petunia_render_gl` + glutin/egui_glow | preserve behind the renderer/host boundary unless an explicit ADR proves removal is correct |

### Verified public sources

- Rust 1.98.1 release: https://blog.rust-lang.org/releases/latest/
- egui 0.36.2: https://docs.rs/crate/egui/latest
- eframe 0.36.2: https://docs.rs/crate/eframe/latest
- egui-winit 0.36.2: https://docs.rs/crate/egui-winit/latest
- egui-wgpu 0.36.2: https://docs.rs/crate/egui-wgpu/latest
- egui_glow 0.36.2: https://docs.rs/crate/egui_glow/latest
- wgpu 30.0.1: https://docs.rs/crate/wgpu/latest

`egui 0.36.x` requires Rust 1.95 or newer. Pin the development/CI toolchain to Rust 1.98.1 for this migration. Do **not** lie about MSRV: if `package.rust-version` is declared, set it to the lowest version actually verified by CI/tests, not simply to a convenient number.

Additional verified ecosystem facts for this directive:

- `iconflow 1.0` unifies 14 open-source icon packs and exposes an egui integration through embedded icon fonts generated from the upstream SVG icon sources;
- `egui_extras` can install an `.svg` image loader when its `svg` feature is enabled;
- `egui::DragValue` natively supports changing a numeric value by dragging the number, which makes it a suitable implementation primitive for the Petunia scrubbable-number-field contract;
- `twill` provides type-safe design tokens/style composition and an egui backend, but Petunia must remain the semantic owner of its design system.

---

# 1. Non-negotiable Petunia3D philosophy

The migration is successful only if these invariants remain true.

## 1.1 Product philosophy

Petunia3D is a focused low-poly authoring/modeling tool. It must favor:

1. ease of use;
2. predictability;
3. modular architecture;
4. robustness;
5. sufficient performance for modest PCs;
6. extensibility;
7. visual polish.

Do not turn Petunia3D into a generic engine/framework merely because a dependency exposes more features.

## 1.2 Dependency direction

The required direction is:

```text
Frontend / UI
      ↓
Application
      ↓
Core / Domain

Renderer and infrastructure stay behind explicit boundaries.
```

Absolute prohibitions:

```text
Core !→ egui
Core !→ eframe
Core !→ localized strings
Core !→ icons
Core !→ physical keycodes
Tools !→ physical keycodes
Application !→ file dialog widget state
Renderer Core !→ egui widgets
```

Frontend/adapters may depend on Application/Core. The reverse is forbidden.

## 1.3 State ownership

Every meaningful state field must belong to exactly one category:

- **Project/Document State** — persistent, versioned, participates in dirty/save semantics;
- **Editor/Application State** — selection, active tool, history, logical session state;
- **UI State** — popup, focus, scroll, panel layout, hover, widget-local state;
- **Cache/Derived State** — reconstructible;
- **Infrastructure State** — GPU, worker pools, filesystem watchers, channels, MCP runtime.

Never move a field to the wrong category merely because a library makes that convenient.

## 1.4 Commands and tools

`CommandId` represents semantic intent.

A command must be dispatchable, where relevant, by:

- product UI;
- keymap;
- CLI;
- Lua plugin;
- MCP;
- future frontends.

No dependency-specific button callback becomes the domain command architecture.

Tools receive explicit context. Do not introduce a giant service locator.

## 1.5 UI sovereignty

Normative UI chain:

```text
Petunia workspaces/screens
        ↓
Petunia Components
        ↓
Petunia UI adapters
        ↓
egui / eframe / egui-wgpu / selected ecosystem crates
```

Third-party widgets must not spread directly throughout workspaces.

The Petunia Design System remains authoritative for:

- colors;
- typography;
- spacing;
- radii;
- strokes;
- metrics;
- motion;
- icons;
- interaction states;
- focus visuals;
- component anatomy.

`egui::Style`, `Visuals`, `Spacing`, `WidgetVisuals`, third-party themes, and painters are implementation details, not the product design system.

### 1.5.1 Absolute token-coverage rule

The current implementation must be treated as **incomplete** until the entire public UI is mapped to semantic Petunia tokens.

Raw visual constants are forbidden in product UI/workspace code when they represent themeable UI presentation.

At minimum, the token system must cover:

```text
surface.*
text.*
border.*
accent.*
status.*
selection.*
focus.*
overlay.*
axis.*
viewport_overlay.*
tool.*
property.*
outliner.*
asset_browser.*
menu.*
tooltip.*
toast.*
modal.*

typography.family.*
typography.size.*
typography.weight.*
typography.line_height.*

spacing.*
size.*
metric.*
radius.*
stroke.*
opacity.*
motion.*
density.*
```

Examples of values that must not remain scattered through workspace/component code:

```rust
Color32::from_rgb(...)
Color32::from_rgba_*
Color32::WHITE
Color32::YELLOW
FontId::proportional(14.0)
Frame::new().fill(...)
Stroke::new(2.0, ...)
rounding/radius numeric literals
panel widths/heights that are part of the design system
widget padding/spacing literals
hover/selected/focus colors
axis/selection/highlight colors
```

Exceptions are allowed only for values that are **not theme presentation** (for example actual model/material colors or image content). Every exception must be obvious from the type/boundary and must not be an excuse to bypass the theme system.

The target architecture is:

```text
assets/themes/*.toml
        ↓
PetuniaThemeSchema (versioned + validated)
        ↓
PetuniaThemeTokens / semantic ThemeToken IDs
        ↓
PetuniaThemeAdapter
        ↓
egui Style/Visuals + Petunia Components + viewport overlay styling
```

A theme is not valid merely because it parses. It must pass a **completeness validator**: all required semantic tokens are defined either by the theme or by an explicit canonical fallback chain.

### 1.5.2 Icon sovereignty and vector-first policy

All public icons must be addressed through semantic `IconId` values.

Required resolution chain:

```text
semantic IconId
    ↓
PetuniaIconRegistry
    ├── Petunia-owned domain icon → bundled SVG
    ├── generic icon → iconflow selected pack
    ├── pack-specific fallback → canonical Petunia/default pack
    └── explicit missing-icon diagnostic
```

Rules:

- integrate **`iconflow`** as the unified provider for generic icon packs;
- enable at least the packs already promised by Petunia customization: **Lucide, Tabler, Iconoir, and Phosphor**;
- do not manually download/duplicate PNG exports of those packs;
- generic pack icons may be rendered by `iconflow` as embedded vector-font glyphs generated from the upstream SVG icon sets;
- **Petunia-owned/default domain-specific icons must be stored and shipped as SVG whenever technically appropriate**;
- enable the `svg` loader in `egui_extras`;
- PNG/JPEG remain valid for raster content such as thumbnails, reference images, texture previews, screenshots, and genuinely raster artwork;
- PNG must not remain the canonical source for toolbar/menu/property UI icons merely because it already exists in the repository;
- remove or quarantine obsolete PNG UI-icon pipelines after SVG/iconflow parity is proven;
- icon tint, size, stroke intent, disabled/hover/active state, and spacing are controlled by Petunia tokens/components;
- icon packs may change glyph choice, but they must not change command meaning or accessibility labels;
- every icon-only button requires an accessible name and localized tooltip.

### 1.5.3 Complete localization rule

The current i18n mechanism may remain TOML-based, but its **coverage must become complete and enforceable**.

Every user-visible string must map through a semantic `TextId`/translation key, including:

- menu items;
- toolbar labels;
- tool names;
- tool descriptions;
- tooltips;
- panel titles;
- section titles;
- Properties/Inspector labels;
- model-information labels;
- validation errors;
- recoverable-error messages;
- confirmation dialogs;
- buttons;
- context menus;
- command palette entries/descriptions;
- settings;
- keymap editor labels;
- asset browser labels;
- status bar text;
- help text;
- empty states;
- onboarding hints;
- units/descriptions where localization is meaningful;
- accessibility names/descriptions;
- plugin/MCP user-facing diagnostics.

Forbidden in public UI code:

```rust
ui.label("Visible text")
ui.heading("Visible text")
Button::new("Visible text")
on_hover_text("Visible text")
format!("Visible user-facing {}", value)
```

unless the literal is a translation key, developer-only diagnostic, test fixture, or other explicitly non-user-facing value.

English is the canonical source locale. `pt-BR` must have full parity for all built-in public strings.

The translation system must support interpolation without building sentences from translated fragments. Add a typed or validated argument mechanism such as:

```text
TextId + TextArgs → localized String
```

Implement locale coverage checks that fail CI when:

- English key is missing;
- a required built-in locale is missing a key;
- an orphan key is detected beyond an explicit deprecation window;
- a public `TextId` has no canonical English string;
- public UI introduces a raw literal detected by the UI audit.

Add a pseudo-locale used only for testing that expands strings and introduces accented characters to expose clipping/layout assumptions.

### 1.5.4 UX completeness is part of implementation

A feature is not complete when the data field exists but interaction is awkward.

The migration must explicitly audit:

```text
Discoverability
Direct manipulation
Keyboard operation
Pointer operation
Undo/redo
Live preview
Cancel/commit semantics
Units
Precision
Defaults/reset
Error feedback
Disabled-state explanation
Focus behavior
Hit targets
Tooltips/help
Responsive layout
Visual hierarchy
Accessibility
```

For DCC-style numeric properties, text-only editing is not acceptable as the only interaction mode.

## 1.6 Accessibility

Every affected custom Petunia component must preserve or improve:

- semantic role;
- accessible name;
- description where needed;
- enabled/disabled state;
- selected/checked/expanded state;
- keyboard activation;
- focus state;
- visible focus;
- appropriate hit target;
- keyboard-only operation where practical;
- non-color-only state communication;
- DPI/UI-scale behavior;
- long-string/i18n behavior.

Do not regress AccessKit semantics while modernizing egui.

## 1.7 Renderer boundary

Preferred viewport chain:

```text
egui layout
  ↓
allocate viewport Rect
  ↓
Petunia viewport adapter
  ↓
egui-wgpu custom callback / renderer bridge
  ↓
PetuniaRenderer
  ↓
wgpu RenderPass/resources
  ↓
GPU
```

The Petunia renderer must not become an egui widget library.

Picking, camera, transforms, mesh, selection, and geometry use neutral Petunia types.

## 1.8 Security and trust boundaries

External data is untrusted by default.

For imports/packages/plugins/MCP/custom packs:

- validate paths;
- canonicalize/contain where appropriate;
- handle symlinks intentionally;
- impose reasonable resource limits;
- handle malformed/corrupt files;
- defend against archive traversal/zip bombs;
- do not silently execute code from data packs;
- plugins use explicit capabilities;
- MCP operations preserve Command/validation/undo/security boundaries;
- no plugin receives raw `egui::Context` or `wgpu::Device`.

## 1.9 Clean Code requirements

Mandatory:

- descriptive names; no cryptic abbreviations;
- small cohesive modules;
- explicit error types at domain boundaries;
- no catch-all `utils.rs` dumping grounds;
- no global mutable state;
- no duplicate implementation hidden behind a new dependency;
- no unused dependency accepted as “integrated”;
- no production `unwrap()`/`expect()` on recoverable external input paths without explicit justification;
- no `TODO` placeholder counted as completion;
- no comments that merely restate code;
- comments explain invariants, trade-offs, safety, and non-obvious constraints;
- prefer narrow adapters/providers over library types crossing layers.

---

# 2. Canonical Gauntlet Loop

Every wave and every major dependency group must execute:

```text
AUDIT
  ↓
TARGET
  ↓
SAFETY TESTS
  ↓
IMPLEMENT / REFACTOR
  ↓
BUILD
  ↓
TEST
  ↓
VISUAL / BEHAVIOR REVIEW
  ↓
ARCHITECTURE CHECK
  ↓
PERFORMANCE CHECK
  ↓
DOCS CHECK
  ↓
SCORE
  ↓
FIX
  ↓
REPEAT
```

## 2.1 Scoring

Score **0–10 without inflation** in these dimensions:

1. Functional Correctness
2. Architecture / Decoupling
3. Data Integrity
4. Tests
5. UX / Accessibility
6. Performance
7. Failure Handling / Security
8. Documentation

A score may rise only when supported by evidence:

- passing tests;
- real behavior;
- screenshots;
- dependency graph checks;
- architecture checks;
- benchmarks;
- fuzz/property results;
- diagnostics;
- reproducible commands.

“Implemented”, “compiled once”, or “agent says it looks correct” is not evidence.

## 2.2 Permanent gates

At minimum, for every meaningful wave:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Also run when introduced/applicable:

```bash
cargo deny check
cargo test --workspace --doc
cargo test -p <affected-crate>
cargo bench --no-run
cargo fuzz build
```

For feature-gated combinations, test the combinations deliberately rather than relying only on `--all-features`.

UI-affecting waves additionally require:

- `egui_kittest` flows;
- focus/keyboard interaction tests;
- accessibility inspection;
- screenshot/visual comparison;
- 1366×768 and 1920×1080 layout checks;
- DPI/scale checks;
- long-string locale checks;
- input routing checks;
- no shortcut leakage through focused text fields/modals.

## 2.3 Stop condition

A wave stops only when:

- its acceptance criteria are satisfied;
- no P0/P1 defects remain inside that wave’s scope;
- all regressions introduced by the wave are fixed;
- architecture rules pass;
- docs/changelog are synchronized;
- remaining limitations are explicitly accepted with impact/risk/next action.

If a wave breaks previously correct behavior, rollback the smallest offending change and reduce the migration step. Avoid big-bang rewrites.

---

# 3. Agent operating rules

1. **Do not begin by editing dependencies blindly.**
2. Audit the repository, current branch, manifests, feature flags, current test state, renderer paths, UI adapters, CI, docs, and current dependency graph.
3. Record a baseline report before edits.
4. Do not discard working implementations merely because a new crate exists.
5. If a requested library overlaps an existing working solution, integrate it at the correct boundary, feature-gate it, or use it for the intended future/dev path; do not create two uncontrolled implementations.
6. If a library is incompatible with egui 0.36/wgpu 30:
   - first check its current stable release;
   - then maintained upstream git if a release is imminent;
   - only then consider a minimal fork/patch;
   - keep the patch behind an adapter;
   - preserve license notices;
   - document the reason and upstream issue/commit;
   - do not downgrade the whole Petunia stack to satisfy one optional library.
7. P2 and P3 dependencies must be **properly integrated according to their adoption class**, not dumped into default production dependencies.
8. P3/FUTURE integration means:
   - real adapter/provider module exists;
   - feature is default-off;
   - it compiles and has a smoke/integration test or executable dev path;
   - documentation explains activation and future owner;
   - it does not block V1/GA;
   - no dead default production UI is resurrected.
9. DEV TOOL dependencies must not be enabled in distribution builds by default.
10. Security-sensitive services (`egui_mcp`, inspection endpoints, plugin host, MCP server) default to disabled/restricted and never bind broadly without explicit configuration.
11. Keep commits small and wave-scoped if repository workflow permits.
12. Do not hide failures by disabling tests, broad `allow` lints, removing assertions, or weakening acceptance criteria.
13. Do not artificially raise Gauntlet scores.
14. Continue fixing until the wave meets its exit gate.

---

# 4. Phase A — Pre-migration audit

Before changing code, produce `docs/audits/stack-modernization/00-baseline.md` containing:

## 4.1 Repository baseline

Capture:

```bash
git status --short
git branch --show-current
git rev-parse HEAD
rustc --version --verbose
cargo --version
cargo metadata --format-version 1
cargo tree --workspace
cargo tree -d
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features
```

Record:

- every workspace crate;
- every `edition`;
- current direct dependency versions;
- duplicate major/minor lines of egui/wgpu/winit;
- current features;
- current renderer ownership;
- current host lifecycle;
- current AccessKit integration;
- current file dialog paths;
- current UI test paths;
- current plugin/MCP state;
- current import/export state;
- current geometry/UV state;
- current CI jobs.

## 4.2 Baseline screenshots and behavior

Capture at least:

- initial workspace;
- Model workspace;
- Edit mode;
- Outliner;
- Properties;
- Settings;
- Asset Browser;
- file open/save flow;
- viewport shading modes;
- transform gizmo;
- reference-image flow.

Use the project’s existing golden references where applicable.

## 4.3 Baseline performance

Record representative current measurements before migration:

- cold startup;
- first frame;
- steady idle CPU;
- representative viewport frame time;
- large Outliner interaction;
- representative mesh operation;
- save/load fixture;
- import/export fixture where available.

Do not invent numbers. If a measurement cannot be produced, state why.

## 4.4 Baseline score

Score all eight Gauntlet dimensions with evidence.

Only then start modernization.

---

# 5. Phase B — Rust toolchain and Edition 2024 migration

## B1. Add/pin the toolchain

Create or update `rust-toolchain.toml`:

```toml
[toolchain]
channel = "1.98.1"
profile = "default"
components = ["rustfmt", "clippy"]
```

Add additional compilation targets only if the project/CI actually uses them.

## B2. Centralize workspace package metadata where sensible

Prefer a root pattern such as:

```toml
[workspace.package]
edition = "2024"
license = "MIT"
```

Then member crates can use:

```toml
edition.workspace = true
license.workspace = true
```

Do not perform this mechanical cleanup if it obscures meaningful per-crate metadata. Keep descriptions/versioning where appropriate.

## B3. `rust-version`

Do not set a false MSRV.

- The active migration toolchain is Rust 1.98.1.
- egui 0.36.x requires Rust >=1.95.
- If Petunia wants MSRV 1.95, prove it in CI.
- Otherwise set a truthful project MSRV or omit it until verified.

## B4. Edition migration process

For every crate:

1. migrate syntax/lints with compiler-supported tooling;
2. inspect edition-related changes;
3. resolve unsafe/trait/import behavior explicitly;
4. run crate-local tests;
5. run workspace tests;
6. do not bundle unrelated refactors into the same commit.

## B5. Exit gate

- all workspace members compile as Edition 2024;
- Rust 1.98.1 builds the entire workspace;
- rustfmt/clippy pass;
- baseline tests still pass;
- no architecture regression.

Execute the full Gauntlet and record `docs/audits/stack-modernization/10-rust-2024-gauntlet.md`.

---

# 6. Phase C — egui 0.36.2 + eframe 0.36.2 + wgpu 30.0.1 compatibility wave

Treat this as one coordinated compatibility wave.

## C1. Upgrade the official egui family together

Target:

```toml
egui = "0.36.2"
egui-winit = "0.36.2"
egui-wgpu = "0.36.2"
egui_glow = "0.36.2"
eframe = "0.36.2"
wgpu = "30.0.1"
```

Keep versions centralized under `[workspace.dependencies]` when practical.

Do not permit accidental multiple egui lines unless a temporarily incompatible third-party crate forces one during a short-lived migration step. The final state should avoid duplicate egui major/minor families.

## C2. Align winit and backend dependencies

`egui-winit 0.36.2` is built around the current `winit 0.30.x` line.

Upgrade/align:

- `winit`;
- `glutin`;
- `glutin-winit`;
- `raw-window-handle`;
- `transform-gizmo*`;
- `egui_tiles`;
- `egui_ltreeview`;
- `egui-file-dialog`;
- `egui-phosphor`;
- `egui_kittest`;
- and every egui-coupled crate

to versions compatible with the target line.

Do not guess compatibility. Verify with:

```bash
cargo tree -d
cargo tree -i egui
cargo tree -i wgpu
cargo tree -i winit
```

## C3. eframe host migration

The canonical architecture specifies eframe, but the current app manually owns winit/egui renderer integration.

Perform the migration through a narrow host boundary.

Required goals:

- `petunia_app` owns the desktop application lifecycle;
- Core remains unaware of eframe;
- UI remains a frontend;
- renderer remains behind Petunia renderer contracts;
- WGPU is the primary canonical viewport path;
- the existing OpenGL fallback is not silently deleted;
- host choice does not leak into mesh/project/commands.

If a single eframe `App` cannot cleanly represent the renderer fallback requirements, introduce a Petunia host abstraction rather than contaminating Core.

## C4. wgpu 25 → 30 migration

Audit and update:

- Instance creation;
- Adapter/device request API;
- Surface configuration;
- texture/buffer descriptors;
- pipeline descriptors;
- render pass descriptors;
- bind group layouts;
- shader/Naga behavior;
- limits/features;
- surface lifecycle;
- frame latency/vsync;
- capture/screenshot integration;
- staging/upload paths;
- X-Ray and picking passes;
- depth state;
- blending;
- MSAA if present.

Run visual and behavioral comparisons after each renderer subsystem change.

## C5. egui 0.32 → 0.36 migration

Audit breaking changes in:

- `Context`;
- input/event translation;
- textures;
- font/image loaders;
- panels/areas/windows;
- accessibility;
- `Response`;
- popup/menu APIs;
- painter/shape APIs;
- custom widgets;
- drag semantics;
- interaction IDs;
- viewport APIs;
- wgpu callback integration.

Do not fix compile errors with broad semantic changes. Preserve behavior unless the new API requires a documented intentional change.

## C6. Accessibility regression gate

Re-test:

- keyboard traversal;
- focus visuals;
- modal blocking;
- text-field shortcut suppression;
- menu Escape behavior;
- accessible names;
- selected/checked/expanded semantics;
- icon-only tooltip/labels;
- scaling.

## C7. Visual regression gate

No unexplained changes to:

- design tokens;
- spacing;
- panel metrics;
- color;
- tool hitboxes;
- Outliner hierarchy;
- workspace pills;
- shading controls;
- viewport overlays;
- Properties tabs;
- Asset Browser.

Any intentional visual delta must be documented and screenshot-approved.

## C8. Exit gate

- one coherent egui 0.36.x family;
- wgpu 30.x family;
- eframe integrated through the host boundary;
- both required renderer paths build according to project policy;
- all UI and renderer tests pass;
- baseline behaviors preserved;
- no forbidden UI dependency leaked into Core.

Record:
`docs/audits/stack-modernization/20-egui-wgpu-host-gauntlet.md`.

---

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

### P1-15 — `egui_taffy` (contrato revisado)

> **Superseded (2026-09-16).** O contrato abaixo descrevia Taffy como piloto
> condicional ("não substitui `egui_tiles` sem ADR"). A *Egui Ecosystem Final Push
> Directive* (§14, §15, §17, §44) substitui essa leitura: Tiles faz macro-layout
> controlado e Taffy faz layout responsivo dentro de um painel — são problemas
> diferentes e **ambos** são baseline. Responsividade manual em product code é
> proibida (§36) e medida por `cargo xtask ui-guard`.

- **Adoption class:** BASELINE CANDIDATE / advanced responsive sublayout
- **Intended owner/boundary:** `petunia-ui/adapters/layout`
- **Required integration contract:** Flex/Block/Grid responsivo dentro de painéis e sublayouts complexos, atrás de `PetuniaTaffyLayout`. Micro-layout simples permanece em built-in. Não é alternativa a `egui_tiles`: é a camada de layout responsivo abaixo dela.
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

# 9. Phase D — P0 integration wave

Integrate P0 in dependency-order, not arbitrary alphabetical order.

## D1. Foundation first

Recommended sequence:

```text
tracing + tracing-subscriber
        ↓
slotmap
        ↓
rayon + flume
        ↓
tempfile
        ↓
schemars
        ↓
cargo-deny
        ↓
proptest + cargo-fuzz
```

Why: observability, IDs, job ownership, safe IO, schemas, supply-chain policy, and test infrastructure should exist before heavy feature providers.

## D2. UI baseline, vector icons, and design-system foundation

Integrate:

- `eframe`;
- `egui_extras` with the required image features, including **`svg`**;
- **`iconflow`** with the selected built-in Petunia generic packs.

Create/finish adapters so workspaces do not import ecosystem crates directly.

### D2.1 Icon migration

Audit every icon source in:

```text
assets/ui/icons/**
crates/ui/src/app_icons.rs
crates/ui/src/icons.rs
crates/ui/src/icon_registry.rs
toolbar/menu/properties/outliner/settings widgets
```

Classify each icon as:

```text
GENERIC
DOMAIN_SPECIFIC
CONTENT_RASTER
TEMPORARY_MIGRATION
DEAD/UNUSED
```

Then migrate:

```text
GENERIC
→ semantic IconId
→ iconflow

DOMAIN_SPECIFIC
→ canonical Petunia SVG
→ SVG loader/cache
→ semantic IconId

CONTENT_RASTER
→ PNG/JPEG/WebP allowed when raster is appropriate
```

The current PNG toolbar/property UI-icon pipeline must not survive as the canonical production path merely to avoid migration work.

### D2.2 Icon visual and accessibility gates

For every built-in selectable pack:

- verify all required `IconId` mappings;
- verify fallback;
- verify no missing glyph/tofu;
- verify 16/18/20/24/32 px-equivalent readability where used;
- verify active/hover/disabled tinting;
- verify high-DPI rendering;
- verify light/dark theme contrast;
- verify icon-only accessible names/tooltips;
- verify command meaning does not change between packs.

### D2.3 Theme-token foundation

Before broad visual refactoring, generate a machine-readable **UI visual inventory** from the current code and map every themeable value to a semantic Petunia token.

Create a token schema/version and a completeness validator before declaring any theme fixed.


## D3. Geometry baseline

Integrate:

- `geo`;
- `manifold-rust`.

Add conversion validation around external provider boundaries:

- finite coordinates;
- valid indices;
- winding policy;
- manifold expectations;
- material/attribute preservation policy;
- failure diagnostics;
- cancellation if operation is jobified.

Add fixtures for:

- simple union;
- disjoint union;
- subtraction;
- intersecting faces;
- degenerate input;
- empty input;
- large low-poly fixture;
- deterministic/repeatability expectations where meaningful.

## D4. IO baseline

Integrate:

- `gltf-json`;
- `tobj`.

The Importer/Exporter contract remains headless.

Required tests:

- valid fixture;
- malformed file;
- unsupported feature;
- missing external resource;
- path traversal attempt where relevant;
- unit/axis conversion;
- normals/UV/material mapping;
- deterministic output where expected.

## D5. P0 Gauntlet

Run the full loop for each group, then a combined P0 loop.

No P0 dependency is complete merely because it appears in `Cargo.toml`.

Record:
`docs/audits/stack-modernization/30-p0-integration-gauntlet.md`.

---

# 10. Phase E — P1 integration wave

Split P1 into product infrastructure and development tooling.

## E1. Product infrastructure

Integrate in this order:

```text
zip
  ↓
mlua
  ↓
rmcp + tokio
  ↓
xatlas-rs
  ↓
notify
  ↓
egui_inbox
  ↓
egui_taffy
  ↓
twill
  ↓
egui_commonmark
  ↓
egui_autocomplete
  ↓
egui_hotkey
```

### Package container (`zip`)

Do not simply zip arbitrary project directories.

Define:

- package manifest;
- format version;
- contained paths;
- size limits;
- compression policy;
- atomic write;
- migration;
- extraction containment;
- cleanup on cancel/failure;
- test fixtures.

### Lua (`mlua`)

Define capabilities before exposing APIs.

At minimum:

```text
plugin identity
plugin API version
declared capabilities
commands allowed
queries allowed
filesystem roots if any
network policy
destructive-operation policy
diagnostics
timeout/resource strategy where possible
```

Plugin APIs operate through Petunia Commands/Queries/DTOs.

### MCP (`rmcp` + `tokio`)

MCP must not become a second application architecture.

Flow:

```text
MCP transport
    ↓
Petunia MCP adapter
    ↓
capability/validation
    ↓
Command / Query
    ↓
Application
    ↓
Core
```

### xatlas

Wrap as a provider.

Never make xatlas-specific UV structures the canonical project representation.

### notify

Route file events into controlled service messages. Re-read and validate the file after events; filesystem events are hints, not trusted content.

### egui_inbox

Use as a UI bridge, not as domain ownership.

### egui_taffy

Use for selected complex sublayouts. Compare complexity and performance against native egui layout before broad adoption.

### Twill and theme infrastructure

Use Twill only behind the Petunia theme/foundation adapter.

The objective is not “use Twill everywhere.” The objective is:

```text
semantic Petunia tokens
        ↓
validated theme data
        ↓
type-safe style/variant construction
        ↓
egui Style + custom Petunia Components
```

Run a before/after scan of hardcoded UI colors, spacing, radii, dimensions, font sizes, and state visuals. The count must trend toward zero outside approved foundation/theme files.

### Markdown/autocomplete/hotkey

Keep all semantics owned by Petunia:

- Markdown is presentation;
- autocomplete returns CommandIds/search items;
- hotkey capture edits the Petunia keymap.

## E2. Development tooling

Integrate:

- `criterion`;
- `egui_inspection`;
- `egui_mcp`;
- `egui_probe`;
- `puffin`;
- `puffin_egui`;
- `assert_fs`;
- `insta`.

Create a unified `petunia-ui/devtools` entry rather than scattered debug windows.

Development build behavior should allow:

```text
Inspect UI tree
Capture screenshot
Inject input
Run UI scenarios
Open profiler
Inspect selected debug state
Review structured logs
```

Distribution behavior:

```text
Remote UI inspection OFF
UI MCP OFF
Probe panels OFF
Profiler UI OFF unless explicitly enabled
No listening debug server
```

## E3. P1 Gauntlet

In addition to normal gates:

- run a real agent-driven UI smoke flow through the inspection/MCP stack if technically possible;
- verify it binds only to intended development interfaces;
- verify production/default release does not expose it;
- benchmark representative paths before/after taffy/inbox/profiling changes.

Record:
`docs/audits/stack-modernization/40-p1-integration-gauntlet.md`.

---

# 11. Phase F — P2 optional integration wave

P2 dependencies are not permitted to bloat the default product without a reason.

For each P2 library:

1. implement the adapter;
2. create a focused real use or reference integration;
3. test it;
4. feature-gate it if not currently needed;
5. document activation;
6. prove no forbidden dependency leaks.

## F1. UI optionals

- `egui_dnd` → Petunia DnD adapter;
- `egui_animation` → Petunia Motion service/tokens;
- `egui_table` → Petunia large-table adapter;
- `egui_form` → Petunia form adapter;
- `egui_tool_windows` → Petunia controlled floating palette adapter;
- `egui-notify` → Petunia notification/toast adapter.

Do not replace already-correct specialized components purely to increase dependency use.

## F2. Platform/security support

- `directories` → central platform directories service;
- `semver` → plugin/package/API compatibility;
- `camino` → only explicit UTF-8 path boundaries;
- `cap-std` → plugin/MCP capability filesystem.

Security tests must include:

- denied path;
- outside-root path;
- traversal;
- missing permission;
- stale capability where applicable;
- malformed package/plugin metadata.

Record:
`docs/audits/stack-modernization/50-p2-integration-gauntlet.md`.

---

# 12. Phase G — P3 future integration wave

P3 must be truly integrated but **must not distort the current product**.

## G1. `rstar`

Add a spatial-index provider with benchmarks against the current approach.

Provide fixtures for:

- nearest vertex;
- region query;
- snapping candidates;
- updates after transform/delete;
- stale index detection/rebuild.

Keep default activation evidence-based.

## G2. `egui_timeline`

Add a `future-animation` feature.

Provide:

- Petunia timeline adapter;
- minimal working track/playhead demo over Petunia-owned timeline DTOs;
- keyboard/focus/accessibility smoke tests;
- no default production Timeline panel until Animation/Rig scope is ready.

## G3. `egui-snarl`

Add a `future-nodes` feature.

Provide:

- Petunia node/port IDs;
- adapter conversions;
- minimal node graph smoke workflow;
- serialization only through Petunia-owned model if/when nodes become persistent.

No Snarl types in Core APIs.

## G4. `egui_code_editor`

Add a `future-script-editor` feature.

Provide:

- editor adapter;
- example Lua text editing;
- no requirement that plugin execution use this editor;
- syntax/editor state remains UI state;
- plugin source/project state classification explicitly documented.

## G5. P3 Gauntlet

Test default build with all P3 features disabled.

Then test each P3 feature individually and in the relevant combined dev feature set.

Record:
`docs/audits/stack-modernization/60-p3-future-integration-gauntlet.md`.

---

# 13. Existing dependency reconciliation

The repository already uses important ecosystem crates. Do not duplicate them.

Audit and update compatibility for at least:

```text
rfd
image
egui-phosphor
egui-file-dialog
egui_ltreeview
egui_tiles
mint
transform-gizmo
transform-gizmo-egui
egui_kittest
winit
pollster
glutin
glutin-winit
raw-window-handle
postcard
bytemuck
gltf (currently observed as dev dependency)
current PNG toolbar/property icon assets and loaders (migration debt, not permanent baseline)
current TOML i18n loader (retain but enforce complete TextId coverage)
current theme/tokens implementation (retain concepts, rebuild for complete semantic coverage)
serde
serde_json
toml
uuid
thiserror
anyhow
log
env_logger
glam
```

## 13.1 Logging consolidation

After `tracing` is integrated:

- application code should prefer `tracing`;
- remove direct `env_logger` if no longer needed;
- keep `log` only where dependency interoperability requires it;
- if a log→tracing compatibility bridge is needed, add the smallest justified supporting dependency and document it;
- do not maintain two unrelated logging configurations.

## 13.2 glTF reconciliation

The audited repository has `gltf` as a development dependency in `petunia_project`.

After P0:

- decide the production owning crate/module;
- add `gltf`/`gltf-json` intentionally;
- remove test-only duplication;
- build real importer/exporter integration;
- test round-trip expectations where appropriate.

---


# 13A. Mandatory UI/UX, Theme, Icon, i18n & Inspector Remediation Wave

This wave is **mandatory** even if the dependency migration already compiles.

The audited repository currently demonstrates that architecture/documentation intent and UI reality are not equivalent. The codebase contains:

- canonical PNG UI icon embedding paths;
- raw `Color32` values outside the token foundation;
- public Portuguese strings hardcoded directly into UI code;
- transform/tool property fields implemented as plain text editing;
- panels whose controls do not yet meet the expected DCC interaction quality.

The purpose of this wave is to close those implementation gaps systematically.

---

## 13A.1 Build a complete UI inventory

Create an inventory tool under `xtask` or an equivalent development tool.

Suggested command:

```bash
cargo xtask ui-audit
```

It must scan `petunia-ui` and produce:

```text
docs/audits/ui-system/
├── 00-ui-inventory.md
├── 01-hardcoded-visuals.md
├── 02-hardcoded-text.md
├── 03-icon-inventory.md
├── 04-number-field-inventory.md
├── 05-panel-usability-inventory.md
├── 06-theme-coverage.md
├── 07-i18n-coverage.md
└── 08-accessibility-coverage.md
```

The scanner should detect likely occurrences of:

```text
Color32::from_*
Color32 named presentation constants
raw FontId sizes
raw padding/spacing/radius/stroke values in public component/workspace code
include_bytes! of UI PNG icons
ui.label("...")
ui.heading("...")
Button::new("...")
on_hover_text("...")
user-facing format!("...")
TextEdit used for numeric domain/tool properties
```

Use an allowlist only for justified foundation files, tests, raster content, and non-user-facing diagnostics. An allowlist entry must contain a reason.

---

## 13A.2 Rebuild the theme model around semantic tokens

### Token groups

At minimum define semantic strongly-typed groups equivalent to:

```rust
PetuniaThemeTokens {
    surfaces,
    text,
    borders,
    accents,
    status,
    selection,
    focus,
    axes,
    viewport,
    tools,
    properties,
    outliner,
    assets,
    menus,
    tooltips,
    modals,
    typography,
    spacing,
    metrics,
    radii,
    strokes,
    opacity,
    motion,
    density,
}
```

Do not require this exact Rust syntax; preserve the semantic coverage.

### Theme schema requirements

Every built-in/custom theme must declare:

```text
schema_version
id
display_name
variant/light-dark classification
author/source metadata where applicable
semantic colors
typography
spacing/density overrides
radii
strokes
metrics
motion preference/defaults
icon-pack preference only as a preference, not command semantics
```

### Fallback

Fallback must be deterministic and visible to diagnostics.

Allowed chain:

```text
custom theme token
→ canonical base-theme token
→ schema default
→ hard failure in development for required token
```

Do not silently replace missing theme tokens with arbitrary egui defaults in production code.

### Theme coverage tests

Add tests for:

- every required token is resolvable;
- unknown token is rejected/diagnosed;
- schema version migration;
- malformed colors/numbers;
- contrast checks for critical text/control states;
- active/hover/disabled/focus visibility;
- long locale strings;
- all built-in themes;
- runtime theme switching without stale cached UI colors.

### Theme cache invalidation

When a theme changes:

- invalidate relevant icon tint/cache;
- invalidate SVG rasterization cache when tint/size depends on theme;
- request repaint;
- do not mutate project/document dirty state;
- preserve UI state such as panel layout/focus where possible.

---

## 13A.3 Vector-first icon-system rewrite

### Required production policy

```text
Petunia-owned UI icons: SVG first
Generic icons: iconflow vector font glyphs
Raster content: PNG/JPEG/WebP when raster is semantically correct
```

Do not convert SVGs to PNG merely for convenience before shipping.

### Generic packs

Configure `iconflow` for the existing user-facing pack choices:

```text
Lucide
Tabler
Iconoir
Phosphor
```

Additional iconflow packs may be exposed later only if:
- licenses are approved by `cargo-deny`/third-party notice policy;
- binary impact is measured;
- mapping coverage is adequate.

Do not enable every pack by default without reason.

### Semantic mapping

Create a canonical mapping table such as:

```text
IconId::Move
IconId::Rotate
IconId::Scale
IconId::Transform
IconId::Extrude
IconId::Inset
IconId::Bevel
IconId::Knife
IconId::LoopCut
IconId::Measure
IconId::Annotate
IconId::Add
IconId::Delete
IconId::Duplicate
IconId::Visibility
IconId::Locked
IconId::Isolate
IconId::Search
IconId::Settings
IconId::Help
IconId::Warning
IconId::Error
IconId::Success
...
```

Each icon pack provides a mapping where a generic equivalent exists.

Domain-specific tools without a semantically correct generic pack icon use a Petunia SVG.

Never choose an approximately similar icon merely to avoid creating a correct domain icon.

### Remove raster icon debt

After parity is proven:

- delete obsolete toolbar/property PNG UI icons;
- delete obsolete PNG decoding/tint code used only for those icons;
- keep migration snapshots in tests/docs only if useful;
- ensure release packages do not carry dead duplicate icons.

### SVG requirements

- sanitize/validate custom SVG assets;
- cache parsed/rasterized results;
- avoid parsing SVG every frame;
- test light/dark tint rules;
- preserve viewBox/aspect ratio;
- keep source SVG optimized but editable;
- record upstream/source/license metadata for externally derived assets.

---

## 13A.4 Full i18n remediation

The existing TOML loader may be retained, but the calling discipline must change.

### Introduce semantic Text IDs

Prefer typed IDs or generated constants:

```rust
TextId::ToolMove
TextId::ToolRotate
TextId::ToolScale
TextId::ToolPropertiesTitle
TextId::TransformLocation
TextId::TransformRotation
TextId::TransformScale
TextId::Apply
TextId::Cancel
TextId::ErrorFiniteNumberRequired
...
```

Do not pass arbitrary English/Portuguese source strings as IDs.

### Inventory all UI strings

Search every public UI path, including:

```text
main_header
viewport_bar
toolbar
contextual_shelf
outliner
properties_panel
asset_browser
asset_library_drawer
settings_modal
command_palette
tool_fields
modal_viewport
measurement
annotation
paint UI
UV UI
file dialogs
notifications
help/tooltips
status bar
context menus
error dialogs
```

### Locale parity

English is canonical and complete.

`pt-BR` must provide every required built-in key.

Add pseudo-locale testing, e.g. a development-only locale that:

- expands text ~30–50%;
- adds accented characters;
- wraps text with visible delimiters.

The goal is to find width assumptions and untranslated strings.

### User-facing errors

Domain/core errors should expose structured codes/context.

UI maps them to `TextId`.

Do not localize inside the geometry/core layer.

### Number localization

`PetuniaNumberField` must:

- display locale-appropriate decimal formatting where practical;
- accept both `.` and `,` where that improves robustness for built-in locales;
- preserve exact internal numeric representation;
- never persist localized formatted strings as numeric project data.

---

## 13A.5 Replace text-only numeric editing with `PetuniaNumberField`

The current transform/tool-properties implementation must be migrated away from plain `TextEdit` as the sole interaction.

Use `egui::DragValue` (including `from_get_set` where useful) as a foundation, or implement equivalent behavior inside the Petunia wrapper.

The Petunia public component is:

```text
PetuniaNumberField
```

not `egui::DragValue`.

### Required interaction model

For numeric properties:

**Click**
- focuses/selects text for exact typed entry.

**Horizontal drag on value or explicit scrub handle**
- continuously adjusts the value;
- shows live preview;
- uses an appropriate resize/scrub cursor;
- captures pointer until release/cancel.

**Shift + drag**
- fine adjustment.

**Ctrl + drag**
- coarse adjustment.

**Enter**
- commits typed/modal value.

**Escape**
- restores/cancels the active transaction.

**Double click or explicit reset action**
- reset to context-defined default only where this is discoverable and safe.

**Right-click/context action**
- may expose Reset / Copy Value / Paste Value where appropriate.

Do not make mouse wheel over a field silently change values while the user is merely scrolling a panel.

### Numeric field metadata

Every numeric property declares a `NumberFieldSpec`-equivalent contract:

```text
semantic TextId label
optional IconId
unit
display precision
minimum/maximum or unbounded
normal drag speed
fine multiplier
coarse multiplier
step/snap behavior
default/reset policy
validation
accessible name
tooltip/help TextId
transaction/undo policy
```

### DCC transform controls

Create reusable:

```text
PetuniaVec2Field
PetuniaVec3Field
PetuniaTransformFields
```

For Location / Rotation / Scale:

- show X/Y/Z in a compact consistent grid;
- X/Y/Z colors come from semantic axis theme tokens;
- value areas are scrubbable;
- labels/tooltips are localized;
- units are visible but unobtrusive;
- exact typing remains available;
- gizmo and fields operate on the same underlying transaction;
- dragging must create one undo step per committed gesture, not hundreds;
- live preview must not dirty/save intermediate invalid data;
- cancel restores the pre-gesture value;
- scale may expose link/unlink axes only if behavior is clearly defined.

Recommended initial sensitivity should be expressed as configurable specs, not literals scattered in UI:

```text
Location: context/unit-aware, e.g. 0.01 display-unit per logical px
Rotation: e.g. 0.5° per logical px
Scale: e.g. 0.01 per logical px
```

Fine/coarse modifiers multiply the spec, rather than each field implementing its own math.

---

## 13A.6 Properties, Tool Properties, and model-information UX redesign

The agent must not treat this as a color-only redesign.

Audit the information architecture and interaction cost.

### Properties panel

Target hierarchy:

```text
Properties
├── Transform
│   ├── Location X Y Z
│   ├── Rotation X Y Z
│   └── Scale X Y Z
├── Object
│   ├── visibility
│   ├── lock
│   └── object/domain properties
├── Geometry / Mesh information
│   ├── vertices
│   ├── edges
│   ├── faces/triangles
│   └── read-only diagnostics
├── Material
└── context-specific sections
```

Rules:

- clearly distinguish editable controls from read-only information;
- do not show irrelevant sections;
- use compact label/value alignment;
- use consistent section headers;
- allow collapse where helpful;
- preserve keyboard traversal;
- show disabled reason where non-obvious;
- no dead controls;
- no placeholder control that does nothing.

### Tool Properties

Show only parameters relevant to the active tool/operation.

Examples:

```text
Extrude
├── Distance [scrubbable]
├── Axis/constraint
├── Individual/region mode if supported
└── contextual help

Bevel
├── Width [scrubbable]
├── Segments
├── Clamp/limits where implemented
└── contextual help
```

Do not expose future parameters as dead UI.

### Model information

Separate read-only metrics from modification controls.

Model stats should be glanceable, copyable where useful, and not disguised as editable text fields.

### Responsiveness

Validate at minimum:

```text
1366×768
1600×900
1920×1080
2560×1440
```

Panels must degrade gracefully via:
- responsive widths;
- wrapping where appropriate;
- truncation + tooltip when unavoidable;
- scroll only in deliberate regions;
- no overlapping controls;
- no clipped Apply/Cancel or numeric fields.

---

## 13A.7 Global UX audit checklist

Audit every major interaction against:

| Dimension | Required question |
|---|---|
| Discoverability | Can a new user tell what is interactive? |
| Direct manipulation | Can a value/tool be manipulated without unnecessary dialogs/typing? |
| Precision | Can an expert still enter an exact value? |
| Feedback | Is hover/active/preview/error state obvious? |
| Reversibility | Is Undo/Cancel predictable? |
| Consistency | Do equivalent controls behave the same everywhere? |
| Density | Is space used efficiently without becoming cramped? |
| Keyboard | Can core workflows be driven predictably? |
| Pointer | Are hit targets and drag ownership correct? |
| Accessibility | Are semantics/focus/contrast sufficient? |
| Localization | Does it work with long translated strings? |
| Theme | Is every visual state derived from tokens? |
| Performance | Does interaction remain responsive on modest PCs? |

Produce prioritized UX findings:

```text
P0 — blocks/corrupts/core unusability
P1 — major friction or inconsistency
P2 — polish and workflow speed
P3 — optional enhancement
```

Fix P0/P1 within this modernization scope.

---

## 13A.8 Theme × locale × icon-pack validation matrix

At minimum exercise:

```text
all built-in themes
× English
× pt-BR
× pseudo-locale
× canonical default icon pack
```

Then smoke-test every selectable iconflow pack with each light/dark class.

For the canonical themes, capture representative screenshots at:

```text
1366×768
1920×1080
```

Cover:

- Model workspace;
- Edit mode;
- Properties;
- Tool Properties;
- Settings;
- Asset Browser;
- Outliner;
- command palette;
- modal/tooltip/notification states.

Automate as much as possible using:

```text
egui_kittest
egui_inspection
egui_mcp
```

---

## 13A.9 UI hardcode quality gate

Final target:

```text
0 unapproved public UI text literals
0 unapproved themeable Color32/raw color literals outside foundation/theme
0 unapproved themeable font-size literals outside typography tokens
0 unapproved themeable radius/spacing/stroke literals outside token/foundation definitions
0 canonical PNG toolbar/menu/property icons when a vector source is appropriate
0 missing built-in translation keys
0 generic icon mappings bypassing IconId
0 numeric DCC fields that are text-only when scrub interaction is appropriate
```

The audit tool must make regressions visible in CI.

---

## 13A.10 UX Gauntlet

Run:

```text
AUDIT
→ TOKEN INVENTORY
→ TEXT INVENTORY
→ ICON INVENTORY
→ NUMERIC-CONTROL INVENTORY
→ PANEL UX INVENTORY
→ IMPLEMENT
→ BUILD
→ UI TEST
→ ACCESSIBILITY TEST
→ THEME MATRIX
→ LOCALE MATRIX
→ ICON-PACK MATRIX
→ VISUAL REVIEW
→ UX SCORE
→ FIX
→ REPEAT
```

Do not exit this wave with known P0/P1 UX defects.

Record:
`docs/audits/stack-modernization/65-ui-system-ux-gauntlet.md`.


# 14. Architecture automation

Add automated checks capable of failing CI when forbidden boundaries are introduced.

At minimum detect:

```text
petunia_core -> egui
petunia_core -> eframe
petunia_core -> UI icon/text/keycode modules
petunia_mesh -> egui
petunia_project -> file dialog UI
petunia_render -> egui widget APIs
domain/tool modules -> physical keycode handling
```

Implementation may use:

- workspace dependency inspection;
- a custom `xtask`;
- targeted source/import scanning where reliable;
- cargo metadata graph validation.

Prefer structural graph validation over fragile grep when possible.

Also add UI-system lint/audit checks for:

```text
public raw UI strings
raw themeable colors outside theme/foundation
raw UI metrics outside token definitions
canonical raster UI icons where vector is required
missing TextId keys
missing locale parity
missing IconId mappings
```

These checks may use AST-aware tooling where practical plus a small explicit allowlist for legitimate exceptions.

---

# 15. CI target matrix

Create/extend CI with meaningful jobs.

Recommended logical jobs:

```text
fmt
clippy-default
test-default
test-renderer-wgpu
test-renderer-opengl
test-ui
test-headless
test-plugins-lua
test-mcp
test-devtools-compile
test-future-animation
test-future-nodes
test-future-script-editor
ui-token-audit
ui-i18n-coverage
ui-icon-audit
ui-ux-interaction
ui-theme-matrix
deny
fuzz-build
bench-compile
docs
```

Run Windows and Linux for the critical production paths.

Do not explode the matrix combinatorially. Choose combinations based on ownership/risk.

---

# 16. Testing requirements by subsystem

## 16.1 Geometry

- deterministic fixtures where expected;
- topology invariants;
- no NaN/inf propagation;
- degenerate input;
- undo/redo behavior;
- property tests;
- fuzz boundaries;
- benchmark real low-poly fixtures.

## 16.2 Project / IO

- atomic save;
- canceled save/export cleanup;
- corrupt project;
- old format migration;
- package traversal;
- missing permission;
- read-only target;
- malformed glTF/OBJ;
- resource limit behavior.

## 16.3 Jobs

Every non-trivial job contract must define:

```text
start
progress (if useful)
cancel
completion
failure
cleanup
stale-result/version handling
```

Workers never arbitrarily mutate editor state.

## 16.4 UI

- semantic queries;
- clicks;
- keyboard;
- drag;
- focus;
- Escape/cancel;
- modal precedence;
- text-input shortcut blocking;
- long text;
- DPI;
- visual regression;
- screenshot evidence;
- complete token coverage;
- complete TextId/i18n coverage;
- SVG/iconflow icon mapping and fallback;
- icon-only accessibility;
- theme switching/cache invalidation;
- pseudo-locale layout;
- scrubbable number-field behavior;
- one undo transaction per drag gesture;
- fine/coarse drag modifiers;
- exact numeric typing;
- transform field ↔ gizmo synchronization;
- panel responsive behavior.

## 16.5 Plugins / MCP

- denied capability;
- allowed capability;
- destructive command policy;
- invalid schema;
- incompatible API version;
- timeout/resource behavior where available;
- filesystem containment;
- no raw GPU/UI access;
- command/undo semantics preserved.

---

# 17. Performance rules

Petunia must remain friendly to modest PCs.

Never accept a new library solely because it simplifies code if it measurably harms important hot paths without justification.

For each performance-relevant dependency:

- record baseline;
- record post-integration result;
- record fixture/hardware/software context;
- compare memory where meaningful;
- compare startup where meaningful;
- compare frame time where meaningful.

Particularly benchmark:

- `manifold-rust`;
- `geo` operations used interactively;
- `rayon` job thresholds;
- `egui_taffy`;
- `egui_table`;
- `rstar`;
- project ZIP save/load;
- xatlas unwrap.

Avoid parallelizing tiny workloads where Rayon overhead dominates.

---

# 18. Dependency quality rules

Every direct dependency must have:

- purpose;
- owner;
- compatible license;
- current version rationale;
- maintenance assessment;
- feature selection;
- default-feature review where important;
- security/trust impact;
- removal/replacement path.

Use `default-features = false` only when you understand and test the resulting feature set.

Do not minimize features blindly and accidentally remove Linux/Windows support, image codecs, accessibility, or renderer functionality.

Run:

```bash
cargo tree -e features
cargo tree -d
cargo deny check
```

Resolve unjustified duplicates.

---

# 19. Documentation changes required

Update the canonical documentation only after implementation evidence exists.

At minimum update:

```text
README / developer setup
PROJECT_STATE.md
CHANGELOG.md
canonical stack documentation
dependency/adapters documentation
architecture diagrams if changed
feature flags
plugin/MCP security docs
project/package format docs
import/export docs
UI devtools docs
ThemeToken schema and generated token reference
IconId mapping/reference and icon-pack licensing/source metadata
TextId/i18n key reference and locale coverage report
PetuniaNumberField interaction specification
Properties/Tool Properties UX specification
benchmark/fuzz instructions
```

Generate `docs/audits/stack-modernization/FINAL_REPORT.md` with:

1. before/after stack;
2. exact versions;
3. dependencies added;
4. dependencies removed;
5. feature flags;
6. architectural changes;
7. migration notes;
8. test results;
9. benchmark results;
10. fuzz/property results;
11. security findings;
12. accessibility findings;
13. screenshots;
14. known limitations;
15. final scorecard;
16. unresolved issues ranked P0/P1/P2/P3.

---

# 20. Required final dependency audit

At the end, generate a machine-verifiable table:

| Dependency | Version | Owner crate | Class | Default enabled? | Adapter/provider | Tests | License | Status |
|---|---|---|---|---|---|---|---|---|

Every P0→P3 dependency from this directive must appear.

Allowed status values:

```text
INTEGRATED
INTEGRATED_DEV_ONLY
INTEGRATED_OPTIONAL
INTEGRATED_FUTURE_DEFAULT_OFF
PATCHED_UPSTREAM_COMPAT
BLOCKED_EXTERNAL
```

`BLOCKED_EXTERNAL` is allowed only when:

- the agent attempted reasonable compatible integration;
- the blocker is external and evidenced;
- forcing it would violate architecture/security/build correctness;
- an ADR records the exact blocker;
- a next action is specified.

The goal remains zero blocked dependencies.

---

# 21. Final combined Gauntlet

After all waves, run a clean combined validation from repository root.

At minimum:

```bash
cargo clean
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo test --workspace --doc
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
cargo bench --no-run
cargo fuzz build
cargo tree -d
cargo xtask ui-audit
```

Then run the project-specific:

- UI interaction suite;
- token/theme hardcode audit;
- TextId/i18n coverage audit;
- icon SVG/iconflow mapping audit;
- scrubbable numeric-field interaction tests;
- theme × locale × icon-pack visual matrix;
- visual regression;
- renderer smoke tests;
- import/export fixtures;
- project save/load/recovery;
- Lua capability tests;
- MCP tests;
- headless CLI tests;
- architecture dependency checks;
- representative benchmarks.

Repeat:

```text
SCORE → FIX → RETEST → RESCORE
```

until:

- no in-scope P0/P1 defect remains;
- no failing required test remains;
- no unexplained visual regression remains;
- no forbidden dependency edge remains;
- no required P0→P3 item is silently absent;
- default production build contains no unintended dev/future service;
- documentation matches real behavior.

---

# 22. Final score requirements

Do not manufacture a 10/10.

A 10/10 in a dimension requires evidence that no known meaningful gap remains in the scoped modernization.

Final report must show:

| Dimension | Baseline | Final | Evidence |
|---|---:|---:|---|
| Functional Correctness | x/10 | x/10 | tests/behavior |
| Architecture / Decoupling | x/10 | x/10 | graph checks |
| Data Integrity | x/10 | x/10 | invariants/fixtures |
| Tests | x/10 | x/10 | suites/property/fuzz |
| UX / Accessibility | x/10 | x/10 | kittest/inspection/screens |
| Performance | x/10 | x/10 | benchmarks |
| Failure Handling / Security | x/10 | x/10 | injection/capability tests |
| Documentation | x/10 | x/10 | docs checks |

If any score is below the project’s accepted completion threshold, continue the Gauntlet Loop.

---

# 23. Mandatory agent completion report

When finished, return a concise but evidence-rich summary containing:

1. final Rust/Edition version;
2. final egui/eframe/wgpu versions;
3. renderer status;
4. every P0 library and integration status;
5. every P1 library and integration status;
6. every P2 library and integration status;
7. every P3 library and integration status;
8. new feature flags;
9. dependencies removed/replaced;
10. architecture changes;
11. test command results;
12. benchmark deltas;
13. security/fuzz/property-test results;
14. UI/accessibility results;
15. theme-token coverage and built-in theme validation;
16. full i18n/TextId coverage including en + pt-BR + pseudo-locale test;
17. iconflow/SVG migration status and remaining justified raster assets;
18. numeric-field and Properties/Tool Properties UX results;
19. final scorecard;
20. exact known limitations;
21. final commit SHA(s), if commits are part of the workflow.

Do not say “done” until the evidence exists.

---

# 24. Source-of-truth references the agent must read before implementation

Repository:

- `Cargo.toml`
- all workspace `Cargo.toml` files
- `PROJECT_STATE.md`
- `CHANGELOG.md`
- `docs/bible/foundations/27-stack-rust-canonica.md`
- `docs/bible/foundations/35-egui-components-adapters-tooling.md`
- current architecture audits
- current implementation plans
- relevant P3D specification mirrors in the repository

Canonical Implementation Bible / Notion, when available:

- **02 — Gauntlet Loop, Quality Gates e Definition of Done**
- **03 — Invariantes de UI/UX, Design System e Acessibilidade**
- **04 — Invariantes de Arquitetura, Modularidade e Core Agnóstico à UI**
- **05 — Política de Documentação, Screenshots e Changelog**
- **10 — Convenções Espaciais, Unidades e Coordenadas**
- **11 — Contrato de Mesh, Selection, Tools e Undo**
- **12 — Contrato de Materiais, Texturas, UV e Color Pipeline**
- **13 — Jobs, Concorrência, Diagnósticos, Segurança e Trust Boundaries**
- **14 — Release, Compatibilidade, Distribuição e Critérios de GA**
- **15 — Auditoria Final de Lacunas e Readiness Matrix**
- **16 — Master Prompt de Implementação por Gauntlet Waves**

Additional implementation references for this modernization:

- `iconflow`: https://github.com/FerrisMind/iconflow
- `egui_extras` SVG loader: https://docs.rs/egui_extras/latest/egui_extras/loaders/fn.install_image_loaders.html
- `egui::DragValue`: https://docs.rs/egui/latest/egui/widgets/struct.DragValue.html
- `twill`: https://docs.rs/crate/twill/latest
- theme architecture references only: `egui_elements`, `egui-elegance`, `egui_sauge`, `egui_colors`

When documentation and implementation disagree:

1. identify the conflict;
2. prefer newer explicit normative decisions;
3. verify behavior/tests;
4. do not silently rewrite history;
5. record the discrepancy and resolution.

---

# 25. Final command to the coding agent

Execute this modernization as a sequence of small, evidence-backed waves.

**Do not stop at dependency installation.**
A dependency counts only when it is correctly owned, isolated, exercised, tested, documented, and compatible with Petunia’s architecture.

**Do not sacrifice Petunia’s philosophy to satisfy a library.**
The library adapts to Petunia; Petunia does not reorganize its domain around library APIs.

**Do not turn optional/future dependencies into baseline bloat.**
Integrate P2/P3 completely at their proper boundary and lifecycle, using default-off features/dev tooling where their canonical adoption class requires it.

**Do not inflate Gauntlet scores.**
Fix defects and repeat until evidence supports the result.

Proceed continuously through:

```text
AUDIT
→ RUST 1.98.1 / EDITION 2024
→ EGUI 0.36.2 / EFRAME 0.36.2 / WGPU 30.0.1
→ P0
→ P1
→ P2
→ P3
→ UI/THEME/ICON/I18N/UX REMEDIATION
→ CONSOLIDATION
→ FULL GAUNTLET
→ FINAL REPORT
```

Preserve working behavior, improve architecture, and leave the repository in a cleaner, more testable, more observable, more secure, and easier-to-evolve state than before the migration.
