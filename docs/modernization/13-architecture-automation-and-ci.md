> [← Voltar ao Índice de Modernização](/modernization/) | [Documento Integral Monolítico](/modernization/full-gauntlet)

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
