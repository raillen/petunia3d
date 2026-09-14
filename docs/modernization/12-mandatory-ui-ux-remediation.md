> [← Voltar ao Índice de Modernização](/modernization/) | [Documento Integral Monolítico](/modernization/full-gauntlet)

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
