> [← Voltar ao Índice de Modernização](/modernization/) | [Documento Integral Monolítico](/modernization/full-gauntlet)

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
