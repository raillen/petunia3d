# Petunia3D — UI/UX, Responsiveness, Performance & Architecture Remediation Gauntlet

> **Implementation directive for Muse Spark 1.3**  
> Repository: `https://github.com/raillen/petunia3d`  
> Audit target: `main` at the current audited revision (`0a12dd1ebef3b8ba8c0ad9f7329540f625529c0b`)  
> Primary objective: **stabilize and finish the existing V1 UI/UX before adding more feature surface**.  
> Product constraint: Petunia3D remains a **simple, low-poly, shape-first, game-asset authoring application**, not a Blender clone, not a level editor, and not a general-purpose DCC.

---

## 0. Mission for Muse Spark 1.3

You are not being asked to make isolated cosmetic fixes. Treat the current problems as a **cross-cutting UI architecture regression** involving layout ownership, responsive behavior, duplicated icon infrastructure, incomplete i18n adoption, workspace composition, viewport overlays, renderer invalidation, and performance.

Your task is to:

1. reproduce every issue in this document;
2. trace it to its actual code path;
3. fix the underlying architecture rather than applying per-screen pixel patches;
4. preserve all valid existing functionality;
5. make the interface coherent in both Petunia Dark and Petunia Light;
6. make all four canonical workspaces visibly and functionally distinct;
7. make layout behave correctly across window resize and DPI changes;
8. restore good performance on low-end computers;
9. add tests and automated gates so the same regressions cannot return;
10. continue in **Gauntlet Loop** until the complete remediation reaches release quality.

Do **not** declare success because the project compiles or because one screenshot looks correct. Success requires the acceptance matrix at the end of this document.

---

# 1. Evidence reviewed

This directive was produced from:

- the current `raillen/petunia3d` repository;
- the UI screenshots supplied by the project owner;
- the supplied screen recording showing the current interaction/layout state;
- the project’s own modernization/audit documentation;
- current egui/iconflow/egui_taffy API behavior.

The supplied visual evidence shows, among other things:

- the right sidebar compressing Outliner and the Inspector/Properties into one narrow vertical area;
- tiny Properties tab icons that are difficult to identify;
- Material controls squeezed under the Outliner;
- the Animate workspace still looking almost identical to Model except for narrow controls at the right and a shelf at the bottom;
- the contextual bottom shelf being clipped, incorrectly sized, or visually colliding with the bottom application area;
- the Reference Set Manager producing a broken staggered/wrapped arrangement with large unused regions;
- icon package previews that do not reliably represent the selected package;
- square/tofu-like glyphs appearing in controls;
- themes switching while several interface strings remain in the wrong language;
- large menus/dropdowns relative to their actual content;
- weak/inconsistent contextual visual feedback.

These are not independent defects. Several originate from shared infrastructure.

---

# 2. Executive diagnosis

## 2.1 Severity P0 — architecture/performance blockers

### P0-A — WGPU renderer rebuilds mesh/edge/reference GPU buffers on every render update

`crates/render-wgpu/src/lib.rs` explicitly states that buffers are rebuilt per frame and `Renderer::update()` regenerates CPU vectors by walking all visible assets, triangulating meshes, collecting edges, and creating new WGPU vertex buffers.

This is unacceptable as the editor grows. Low-poly does not make per-frame re-triangulation, Vec allocation, CPU conversion, and GPU buffer allocation free.

**Required correction:** introduce revision-based render caches. Camera changes must update camera uniforms only. Mesh GPU data must only rebuild when geometry/material/selection/render-relevant revisions change.

### P0-B — OpenGL renderer creates/deletes VBOs during every draw

`crates/render-gl/src/lib.rs` creates a buffer inside `draw_mesh()`, uploads all vertex data, draws, then deletes the buffer. `draw_edges()` does the same for line geometry.

This is a major likely source of the reported performance deterioration, particularly on low-end systems and drivers.

**Required correction:** persistent per-asset GPU resources keyed by stable asset IDs and revision counters. Update with `buffer_sub_data`/replacement only when data changes. Camera movement must never force mesh reconstruction.

### P0-C — right panel architecture squeezes unrelated panes into one narrow column

`crates/ui/src/lib.rs::right_panel()` currently renders:

```text
Outliner
separator
Properties Panel
```

inside the same `Panel::right("props")`.

The user’s complaint that Active Tool, Transform, Modifier Stack, Geometry Data, and Materials are all squeezed under the Outliner is therefore the expected result of the current architecture.

There is already an `egui_tiles` pilot in `crates/ui/src/tiles_workspace.rs` with independent `Outliner` and `Properties` panes. The implementation has not been promoted into the canonical layout.

**Required correction:** make Outliner and Properties two independent panes with a resizable splitter, minimum heights, collapsing behavior, persistence, and workspace-aware defaults.

### P0-D — workspaces only switch state/content fragments, not actual workspace composition

`main_header.rs::draw_workspace_pills()` changes only `state.workspace`.

The top-level `petunia_ui::draw()` still executes the same macro-layout for all workspaces:

```text
main_header
status_bar
asset_browser
toolbar
right_panel
viewport_bar
viewport
...
```

The right Properties content and bottom contextual shelf react to `state.workspace`, but the application shell is fundamentally unchanged. This explains why clicking MODEL / PINTURA / UV / ANIMATE does not feel like changing workspace.

**Required correction:** create explicit **WorkspaceLayoutProfile** objects. Workspaces share the same scene/document state, but each workspace composes the shell differently.

### P0-E — two competing icon infrastructures exist, and the intended `iconflow` provider is not the production path

The repository currently has:

- `icon_registry.rs` — production-facing PetuniaIcon registry using embedded PNGs, native vector icons, and `egui_phosphor`;
- `icon_provider.rs` — the modernization-era semantic provider using `iconflow`.

Search of current code shows `GenericIcon`/`PetuniaIconProvider` is effectively used only inside its own tests, while production widgets continue to call `IconRegistry::paint()`.

Worse, the old `paint_pack()` treats `phosphor`, `tabler`, `lucide`, and `iconoir` as the same Phosphor glyph source. The Settings UI advertises packages that the actual renderer does not faithfully use.

This explains the owner’s observation that packages are not applied correctly.

**Required correction:** one canonical semantic icon service only.

### P0-F — incomplete i18n is structural, not merely missing TOML strings

Many interface strings are still hardcoded in Portuguese or English inside Rust files, including Settings tabs/descriptions, status bar content, Reference Manager labels, animation labels, menu labels, tooltips, contextual shelf labels and errors.

Even perfectly complete locale files cannot translate those literals.

**Required correction:** no user-visible production UI literal may bypass the localization layer, except deliberately language-neutral symbols/numbers and debugging/dev-only UI.

---

# 3. Non-negotiable architecture rules

Implement all remediation under these rules.

## 3.1 Keep Core independent from egui

Do not move UI concepts into `petunia_core` merely to simplify widgets. Keep:

```text
Domain / Geometry / Commands
          ↑
Application state + semantic commands
          ↑
UI adapter (egui)
```

UI may own view/layout preferences. Domain owns meaningful editor/project data.

## 3.2 One state owner per concept

There must not be multiple parallel states for:

- current workspace;
- active icon pack;
- active theme;
- active language;
- panel visibility;
- selected tool;
- selected object;
- timeline state.

Widgets request changes through semantic helpers/commands; they do not shadow these states inside temporary egui memory unless the state is purely ephemeral and frame-local.

## 3.3 Responsive layout uses available rect, not assumed screen dimensions

Never size a child from `ctx.viewport_rect()` when what matters is the current panel/content rect.

Components must use:

```text
ui.available_width()
ui.available_height()
ui.available_rect_before_wrap()
```

or an explicit `UiRegion`/container rectangle provided by the canonical layout system.

## 3.4 No fixed-width architecture

Fixed widths are allowed for small atomic controls, icons, compact numeric fields and deliberate minimum targets.

They are not acceptable as the primary strategy for:

- whole shelves;
- whole settings cards;
- reference grids;
- inspector layouts;
- header rows;
- dropdown menus;
- workspace panes.

## 3.5 Every icon-only control requires semantic accessibility

Every icon-only action requires:

- tooltip;
- accessible widget info/label;
- keyboard path where applicable;
- focus ring;
- deterministic semantic icon ID;
- no meaning communicated by color alone.

## 3.6 No emoji as application icons

Remove UI usage such as:

```text
👁
🗑
✨
👤
↩
↪
⇲
```

where the character serves as an application icon.

Use semantic icon IDs through the canonical icon service. Emoji/Unicode may remain in user content or documentation, not as the primary product icon infrastructure.

---

# 4. Canonical UI shell redesign

## 4.1 Introduce `UiRegions`

Create a canonical shell result such as:

```rust
pub struct UiRegions {
    pub header: egui::Rect,
    pub viewport_toolbar: egui::Rect,
    pub left_tools: egui::Rect,
    pub content: egui::Rect,
    pub viewport: egui::Rect,
    pub right_dock: Option<egui::Rect>,
    pub bottom_dock: Option<egui::Rect>,
    pub status_bar: egui::Rect,
}
```

The exact ownership may differ, but the principle is mandatory: overlays and hit-testing need a single source of truth for safe UI rectangles.

## 4.2 Panel allocation order

Canonical order should reserve fixed shell areas before overlays:

```text
Top Header
↓
Workspace/View Toolbar
↓
Bottom Status Bar
↓
Optional Workspace Bottom Pane (Timeline/UV/etc.)
↓
Left Tools
↓
Right Dock
↓
Central Workspace Content / Viewport
↓
Viewport-local overlays
```

Never position a contextual shelf from the global screen bottom.

## 4.3 Promote or replace `tiles_workspace.rs` deliberately

`tiles_workspace.rs` already proves the repository has `egui_tiles` support and understands Outliner and Properties as separate panes.

Do one of two things, after a short spike:

### Preferred
Promote `egui_tiles` as the canonical pane composition layer for **workspace panes**, while retaining normal egui Panels for immutable shell areas (top header/status/left global toolbar).

### Acceptable alternative
Implement a small owned split-pane abstraction if `egui_tiles` causes integration complexity with the GPU viewport.

Do not leave both systems half-active.

## 4.4 Right dock target

Wide layout:

```text
┌────────────────────────────┐
│ Parts / Outliner           │
│                            │
├──────── draggable ─────────┤
│ Properties / Inspector     │
│ ┌ Tool                   ┐ │
│ ├ Transform              ┤ │
│ ├ Modifiers              ┤ │
│ ├ Mesh Data              ┤ │
│ └ Material               ┘ │
└────────────────────────────┘
```

The splitter position must persist in user UI preferences.

Minimum behavior:

- Outliner min height: enough for at least ~4 rows + search/header;
- Properties min height: enough for tab bar + one meaningful section;
- both scroll independently;
- either can collapse;
- Inspector detach remains optional, but is not the only workaround for bad layout.

---

# 5. Responsive Properties / Inspector redesign

The current 5 icon tabs in a narrow horizontal strip are hard to distinguish.

Implement three breakpoints based on **panel width**, not screen width.

## 5.1 Wide inspector

At sufficient width:

```text
[Tool] [Object] [Modifiers] [Mesh] [Material]
```

Use icon + translated text.

## 5.2 Medium inspector

Use semantic icons with robust hover tooltips and visually larger 28–32 px targets.

## 5.3 Narrow inspector

Do not squeeze all icons indefinitely.

Use one of:

```text
Properties: [ Material ▼ ]
```

or a compact vertical icon rail if measured accessibility remains acceptable.

## 5.4 Tool Properties separation

Do not let `Active Tool`, active modal Transform, generic Object settings, modifiers and material compete for the same unstructured vertical stream.

Define an explicit contextual priority:

```text
Tool Properties (contextual, near active tool)
Object / Geometry / Material Inspector (persistent)
```

A modal operation may temporarily pin a Tool Properties card without changing the selected persistent inspector category.

---

# 6. Workspace system must become real

Create:

```rust
pub struct WorkspaceLayoutProfile {
    pub id: Workspace,
    pub left_tools: ToolPaletteKind,
    pub right_panes: Vec<PaneKind>,
    pub bottom_panes: Vec<PaneKind>,
    pub viewport_overlay: ViewportOverlayProfile,
    pub default_active_inspector: InspectorTab,
}
```

Do not duplicate project state between workspaces.

## 6.1 MODEL

```text
Left: modeling tools
Center: 3D viewport
Right top: Parts
Right bottom: Tool/Object/Modifiers/Data/Material
Bottom overlay: compact contextual shelf
```

## 6.2 PAINT

```text
Left: Paint tools
Center: 3D Paint viewport
Right top: Layers / Surface Details
Right bottom: Brush / Material / Channel properties
Optional 2D texture editor: user-opened pane, not forced
```

## 6.3 UV

UV must visibly change the working environment.

Suggested default:

```text
┌───────────────┬───────────────┐
│ UV Editor     │ 3D Preview    │
│               │               │
└───────────────┴───────────────┘
Right: UV properties / islands / material context
```

For small windows, expose a toggle between `UV` and `3D Preview` instead of crushing both.

## 6.4 ANIMATE

```text
Center: 3D viewport
Right: Rig/Bone/Clip inspector
Bottom: real Timeline / Dope-sheet-lite pane
Left: Pose/animation tools
```

The current behavior where Animate mostly changes narrow sidebar content is insufficient.

## 6.5 Workspace tests

For every workspace assert:

- button click changes `state.workspace`;
- expected pane profile becomes active;
- toolbar tools change;
- expected bottom pane visibility changes;
- keyboard focus is not lost unexpectedly;
- returning to a workspace restores its split sizes and selected inspector tab.

---

# 7. Contextual floating shelf remediation

Current `contextual_shelf.rs` uses workspace-dependent estimated widths such as `700`, `780`, etc. and then forces:

```rust
.max(estimated_w)
.min(screen_w - 24.0)
```

It also estimates text width with `label.len() * 6.5`, which is not a valid text measurement strategy and is especially unreliable across translations.

Remove this architecture.

## 7.1 New responsive shelf model

Represent content as priority groups:

```rust
pub struct ShelfGroup {
    pub priority: ShelfPriority,
    pub commands: Vec<ShelfCommand>,
}
```

Each command has:

- semantic icon;
- localized label;
- localized tooltip;
- shortcut;
- compact label optional;
- icon-only representation allowed;
- overflow eligibility.

## 7.2 Breakpoint behavior

### Wide

Show icon + full label.

### Medium

Show icon + shortened label for high-priority tools; secondary tools become icon-only.

### Narrow

Show essential tools as icons and place the remainder in a `More` overflow popover.

### Very narrow

Replace shelf with a small `Tools` pill/popover rather than allowing clipping.

## 7.3 Geometry

The shelf must be positioned within the final `viewport_rect` returned by the layout shell.

Use:

```text
bottom = viewport_rect.bottom - safe_margin
```

The status bar is already outside that rectangle and must never be overlapped.

## 7.4 Measurement

Do not estimate text width from `String::len()`.

Use egui galley/layout measurement or natural layout pass. If advanced responsive composition becomes unwieldy, use the repository’s existing `egui_taffy` adapter for this sublayout only.

`egui_taffy` is already declared specifically as a responsive sublayout candidate; do not migrate the whole UI to it.

---

# 8. Menus and dropdowns are too wide

There are two distinct problems.

## 8.1 Custom Petunia menu items

`PetuniaMenuItem::show()` currently uses:

```rust
let width = ui.available_width().max(148.0);
```

and radio items use a 180 px minimum.

This effectively encourages each row to fill a popup’s available width rather than making the popup match content within sane bounds.

### Required fix

Build a menu sizing policy:

```rust
pub struct MenuMetrics {
    pub min_width: f32,
    pub preferred_width: f32,
    pub max_width: f32,
}
```

Measure:

```text
left padding
+ optional icon
+ localized label galley width
+ gap
+ shortcut width
+ submenu indicator
+ right padding
```

Clamp to a sane max relative to current content region.

Long labels should truncate or wrap only under explicit policy.

## 8.2 egui ComboBox

egui’s `ComboBox::width()` controls the outer button/menu width and defaults to `Spacing::combo_width`.

Do not depend on one global width for every context.

Implement local responsive widths such as:

```rust
let width = ui.available_width().clamp(96.0, 180.0);
egui::ComboBox::from_id_salt(id).width(width)
```

For compact toolbar combos use smaller explicit widths. For Inspector fields use available row width.

---

# 9. Reference Set Manager complete redesign

The current manager:

- has six fixed 260 px cards;
- puts them in `horizontal_wrapped`;
- cards can have different heights;
- computes window max size from screen values using minimums that can exceed a small viewport;
- combines a long descriptive label and global actions in one horizontal row;
- still contains hardcoded text and emoji-like action symbols.

This produces the broken staggered result seen in the supplied screenshot.

## 9.1 New layout

Use a deterministic responsive grid.

Calculate:

```text
available = ui.available_width()
minimum_card = 220–240 logical px
columns = clamp(floor((available + gap) / (minimum_card + gap)), 1, 3)
card_width = (available - gap * (columns - 1)) / columns
```

On typical desktop:

- >= 1050 content px: 3 columns × 2 rows;
- ~720–1050: 2 columns × 3 rows;
- smaller: 1 column × 6 rows.

Do not hardcode those exact thresholds if measured content indicates better values.

`egui_taffy` Grid/Flex is appropriate here because it is already in the dependency graph and this screen is a classic adaptive-card case.

## 9.2 Card invariants

Every reference slot card always reserves the same major zones:

```text
Header
Preview
Status/filename
Controls summary
```

When empty, the preview area remains the same height.

Loaded advanced controls may live in a collapsible section rather than expanding one card unpredictably.

## 9.3 Header

Do not force description and actions into a single line.

Wide:

```text
Title/description                    Add Custom | Clear All
```

Narrow:

```text
Title/description
Add Custom | Clear All
```

## 9.4 Window sizing

Never compute a `max_size` larger than the usable viewport by applying `.max(minimum)` after subtraction.

Instead:

```text
available_w = viewport_w - margins
requested_min_w = 460
actual_min_w = min(requested_min_w, available_w)
```

Same for height.

## 9.5 Additional fixes

- all text through i18n;
- all actions through semantic icons;
- keyboard focus traversal in natural slot order;
- `Front/Back/Left/Right/Top/Bottom` translated consistently;
- clear drop targets and hover state;
- image replacement and file errors shown inline;
- preview loading must be cached and must not decode on every frame.

---

# 10. Primitive creation needs contextual “Last Operation” editing

The current Object shelf directly calls `add_primitive_to_scene(...)`; once inserted, the primitive is essentially already a regular mesh.

Implement an **Add Primitive Session**, not a permanent procedural-object system.

This preserves Petunia’s simplicity while delivering the expected direct feedback.

## 10.1 Model

```rust
pub struct PrimitiveCreationSession {
    pub asset_id: AssetId,
    pub descriptor: PrimitiveDescriptor,
    pub anchor: PrimitivePopupAnchor,
    pub original_selection: SelectionSnapshot,
}

pub enum PrimitiveDescriptor {
    Cube(CubeParams),
    LowSphere(SphereParams),
    Cylinder(CylinderParams),
    Plane(PlaneParams),
}
```

## 10.2 Parameters

### Cube
- size or dimensions;

### Low Sphere
- radius;
- segments/rings or low-poly resolution control;

### Cylinder
- radius;
- depth;
- sides;
- cap mode if already supported;

### Plane
- width/height or size;
- optional simple divisions if justified.

## 10.3 Interaction

After insertion:

```text
[primitive appears]
       ↳ small contextual card near projected bounds
```

Example:

```text
CYLINDER
Radius      1.00
Height      2.00
Sides       8

[Confirm] [Cancel]
```

Changes regenerate from the descriptor instead of incrementally deforming the previous result.

The entire insertion/parameterization should become **one undoable transaction** once confirmed.

## 10.4 Popup anchoring

Anchor to:

1. projected asset bounds corner when visible;
2. insertion cursor/mouse location as fallback;
3. Tool Properties pane if no safe viewport area exists.

Clamp to viewport safe rect. Never overlap status bar/right dock by absolute screen math.

## 10.5 Lifecycle

- Enter / click elsewhere: Confirm;
- Esc: Cancel and remove created primitive;
- F9 or explicit “Last Operation” may reopen while descriptor is still valid;
- any topology-editing operation converts/finalizes it into normal mesh editing.

No permanent Blender-style procedural primitive object is required.

---

# 11. Canonical icon architecture

This is a high-priority cleanup.

## 11.1 Delete the parallel meaning systems

Target conceptual architecture:

```text
SemanticIconId
      │
      ├── Petunia domain icon → bundled Petunia SVG/vector source
      │
      └── Generic icon → iconflow active GenericPack
                         │
                         └── deterministic fallback to default pack
```

Keep compatibility adapters temporarily while migrating call sites, then remove them.

## 11.2 Use `iconflow` correctly

Current `icon_provider.rs` resolves a family/codepoint correctly but is not wired into production rendering.

The `iconflow` API exposes `fonts()` and requires fonts to be registered into egui `FontDefinitions`. Render resolved glyphs with the named icon family, e.g. conceptually:

```rust
FontFamily::Name(icon.family.into())
```

Do not render every third-party pack using Phosphor glyphs.

## 11.3 Built-in pack availability must reflect compiled features

The current old registry advertises Tabler and Phosphor even when `extended-icon-packs` is disabled.

Settings must list only packages actually available in the build, or explicitly display a disabled package with a truthful build-state explanation. Do not show a “Use this pack” button for a pack that cannot actually render.

## 11.4 “Download pack” semantics

Do not pretend built-in packs are downloaded at runtime. Built-in `iconflow` packs are build-time assets/features.

If future custom packs can be installed from disk, that is a separate plugin/package-management path.

## 11.5 Remove tofu/squares

Add an icon audit:

```text
for each SemanticIconId
    for each enabled pack
        resolve glyph or approved fallback
        assert no missing family
        assert valid Unicode codepoint
```

If resolution fails in a dev build:

- draw a visible debug missing-icon marker;
- log semantic ID and pack;
- never silently draw an unknown tofu box.

## 11.6 SVG-first Petunia domain icons

The repository already enables `egui_extras` with SVG support.

Use SVG for Petunia-owned domain/tool icons where vector scalability matters. Keep raster images for actual raster content, not generic UI controls.

---

# 12. Complete i18n remediation

## 12.1 Eliminate hardcoded production strings

Audit all files under:

```text
crates/ui/src/**
```

Every user-facing literal becomes a translation ID.

Known problem files include at minimum:

- `main_header.rs`;
- `settings_modal.rs`;
- `status_bar.rs`;
- `reference_manager.rs`;
- `contextual_shelf.rs`;
- `properties_panel.rs`;
- `tiles_workspace.rs`;
- `modules_ui/animation_ui.rs`;
- `modules_ui/paint_ui.rs`;
- `modules_ui/uv_ui.rs`;
- context menus and viewport overlays.

## 12.2 Introduce typed translation IDs

Avoid arbitrary stringly-typed translation keys across dozens of files.

Preferred:

```rust
pub enum TextId {
    MenuFile,
    MenuEdit,
    SettingsAppearance,
    ToolMove,
    TooltipMove,
    ...
}
```

or generated constants from a canonical locale schema.

A transitional wrapper can map to the existing TOML implementation.

## 12.3 Locale parity CI

Add a deterministic tool/test that verifies:

- every canonical English key exists in pt-BR;
- pt-BR does not contain orphan keys;
- no duplicate/invalid hierarchy;
- all `TextId`s resolve;
- missing keys fail CI instead of silently shipping key names.

Development builds can still fallback to English to remain usable, but CI must catch the incomplete translation.

## 12.4 Pseudo-locale

Generate a pseudo-locale automatically, for example:

```text
Settings → [ Šééţţïñğš — LONG ]
```

Use it to catch clipping, fixed-width assumptions, shelf overflow and bad dropdown sizes.

## 12.5 Switch language live

Changing language must:

- rebuild all visible labels immediately;
- invalidate layout where appropriate;
- update tooltips, menu entries and status text;
- preserve selected commands, panel state and focus where reasonable.

---

# 13. Tooltips and help feedback

The project needs a universal rule rather than manually noticing missing tooltips.

## 13.1 Tooltip contract

Every icon-only or ambiguous tool must expose:

```text
Localized title
One-sentence action description
Shortcut if present
Optional state explanation
```

Example:

```text
Extrude
Extend the selected face or edge outward.
Shortcut: E
```

Keep tooltips short by default. A help system can contain deeper explanations.

## 13.2 Canonical widget wrappers

Create/complete wrappers:

```text
PetuniaIconButton
PetuniaToolButton
PetuniaToggleButton
PetuniaMenuItem
PetuniaComboField
PetuniaNumberField
```

They should require or derive semantic tooltip/accessibility metadata.

This allows an automated audit: production code should not create ad hoc icon-only `ui.button("symbol")` controls.

## 13.3 Interaction feedback

All tools need clear states:

- hover;
- pressed;
- active;
- disabled;
- keyboard focus;
- modal operation;
- success/error/invalid target where relevant.

Selected/active controls should not rely solely on blue fill; include shape/border/icon/state change usable under color-vision deficiencies.

---

# 14. Accessibility remediation

Implement against pragmatic desktop-editor requirements.

## 14.1 Target sizes

Use a default interactive target of approximately 28–32 logical px for compact desktop controls, with enough spacing to avoid accidental clicks. Tiny 18–20 px clickable controls should be exceptional and still receive a larger hit target where possible.

## 14.2 Keyboard navigation

Ensure:

- tab navigation through dialogs/settings/reference manager;
- arrow navigation where semantic;
- Space/Enter activation;
- Esc closes popovers/modal operations predictably;
- focus is visible;
- workspace switching has command/keymap hooks.

## 14.3 Screen-reader/semantic metadata

Use egui WidgetInfo/accessibility output consistently for custom-painted widgets.

A custom-painted rectangle is not automatically an understandable button.

## 14.4 Reduced visual motion

Add an Interface/Accessibility preference to disable/reduce UI animations if any are introduced for panel/shelf transitions.

## 14.5 UI scale

Expose UI scale using supported egui pixels-per-point strategy. Verify at least:

```text
100%
125%
150%
175%
```

Do not use scale-specific hardcoded coordinates.

---

# 15. Settings redesign

Current Settings only covers Appearance, Icons, Language and Keymap, while multiple meaningful application preferences are scattered or absent.

Do not turn Settings into an enormous control panel. Use a clear set of categories.

Recommended:

```text
General
Interface
Viewport
Appearance
Icons
Language
Keymap
Performance
Autosave & Recovery
Import / Export
Developer   (only dev builds / explicit enable)
```

## 15.1 General

- startup behavior;
- reopen last project if supported;
- confirmation preferences where appropriate.

## 15.2 Interface

- UI scale;
- tooltip delay / enable explanations;
- show contextual shelf;
- compact mode policy;
- show guides behavior;
- reduced motion;
- panel reset to defaults.

## 15.3 Viewport

Move persistent user preferences such as grid/axis display defaults here when they are application preferences rather than project data.

## 15.4 Performance

- renderer backend selection if already supported safely;
- viewport anti-aliasing/sample setting only if existing renderer supports it;
- low-power mode if meaningful;
- diagnostics/profiling shortcut in dev builds.

Do not add placebo toggles that have no implementation.

## 15.5 Autosave & Recovery

Expose only settings already supported by the project lifecycle or implement them fully before showing them.

## 15.6 Reset layout

Add:

```text
Reset Current Workspace Layout
Reset All UI Layouts
```

This is essential after introducing persistent split positions.

---

# 16. Performance remediation — mandatory

Do not optimize by intuition only. First establish repeatable profiling scenarios, then remove confirmed hotspots.

## 16.1 Existing good architecture to preserve

The project already has a render-on-demand concept based on `ControlFlow::Wait` and dirty tracking. Keep this model.

Continuous repaint is appropriate only while:

- camera is moving;
- transform/modal drag is active;
- timeline animation is playing;
- a UI animation is active;
- an async job completion requires redraw.

For low-frequency timed UI, prefer scheduled repaint (`request_repaint_after`) instead of a global polling loop.

## 16.2 WGPU revision cache

Replace “rebuild every frame” with something like:

```rust
struct GpuAssetCache {
    mesh_revision: u64,
    material_revision: u64,
    selection_revision: u64,
    vertex_buffer: wgpu::Buffer,
    edge_buffer: wgpu::Buffer,
    mesh_count: u32,
    edge_count: u32,
}
```

Separate revisions so selection changes do not necessarily rebuild solid triangles if only edge/selection overlay changes.

Camera updates only `cam_buffer`.

Reference geometry only rebuilds when reference transform/visibility changes. Reference pixels only upload when image revision/hash changes.

## 16.3 OpenGL persistent resources

Create persistent VAO/VBO resources per asset/cache entry.

Do not call:

```text
create_buffer → buffer_data → draw → delete_buffer
```

for every object every frame.

Use revision-driven buffer updates.

## 16.4 Avoid per-frame geometry conversion

Calls such as:

```text
to_triangles_smooth
to_triangles_unlit
to_edges
triangulation_wireframe
```

should be computed when relevant source state changes, not during every camera movement.

## 16.5 UI cache audit

Cache or invalidate on events rather than scanning each frame:

- available icon packs;
- theme manifests;
- locale manifests;
- filesystem resources;
- project-library metadata;
- reference thumbnails;
- text-heavy derived diagnostics.

File watcher events can trigger registry refresh.

## 16.6 Avoid accidental continuous dirtying

Audit every `mark_dirty()` call.

`mark_dirty()` belongs on state changes, not merely because a widget was drawn or hovered.

Add a debug `DirtyReason` trace in development builds:

```rust
state.mark_dirty_reason(DirtyReason::CameraOrbit);
```

so continuous redraw loops can be identified.

## 16.7 Measure both render backends

Do not fix WGPU and leave OpenGL pathological. Low-end support makes OpenGL particularly important.

Scenarios:

1. idle, one cube;
2. continuous hover, no camera motion;
3. orbit/pan;
4. gizmo drag;
5. 10k triangles;
6. 100k triangles;
7. reference images loaded;
8. Paint stroke;
9. Animate timeline playback;
10. settings/reference modal open.

Measure:

- UI pass CPU ms;
- geometry preparation CPU ms;
- renderer upload ms;
- GPU render ms where available;
- allocations/frame;
- buffer creations/frame;
- texture uploads/frame;
- redraws/sec when idle.

## 16.8 Performance acceptance targets

Use these as initial targets, then document actual reference hardware:

- idle: **no continuous redraw** without a reason;
- buffer creations after warm-up while only orbiting camera: **0 mesh/edge buffer recreations/frame**;
- unchanged reference textures: **0 pixel uploads/frame**;
- no filesystem scanning in steady-state frame loop;
- UI p95 target: <= 8 ms at 1080p in normal editor state;
- complete frame target: sustain 60 Hz for ordinary low-poly assets on the project’s declared low-end reference profile, with no recurring >33 ms spikes from UI/layout/buffer allocation.

If low-end hardware cannot hit 60 FPS, retain responsiveness with adaptive quality rather than rebuilding unchanged geometry.

---

# 17. Use the libraries already adopted — but only where they fit

## `egui_tiles`

Use for persistent workspace pane splitting/docking if the viewport integration spike succeeds.

Do not use it for every little widget.

## `egui_taffy`

Use selectively for difficult responsive sublayouts:

- Reference Manager adaptive card grid;
- settings cards/header rows;
- potentially contextual shelf priority wrapping.

Do not rewrite all ordinary egui layout in Taffy.

## `iconflow`

Promote to canonical provider for generic icon packs. Actually install its font assets and use named families.

## `egui_extras` SVG loader

Use for Petunia-owned SVG domain icons/resources.

## `puffin`

Use it as a real gate, not merely an optional dependency. Add profiling scopes around:

```text
whole UI
right dock
Outliner
Properties
Reference Manager
contextual shelf
geometry preparation
GPU uploads
renderer draw
```

## `egui_inbox`

Use where background task/event delivery needs safe UI wakeups. Do not poll asynchronous job state aggressively each frame.

## `twill`

Do not force adoption merely because it is in Cargo. Keep only if the existing bridge has a real maintained purpose. Unused dependencies should not dictate architecture.

---

# 18. Architecture cleanup / duplicate systems

The current repo contains modernization pilots that are not authoritative while old paths remain production paths.

Create an **Architecture Convergence Ledger**.

For each duplicated concept record:

```text
Old implementation
New/pilot implementation
Chosen canonical owner
Migration steps
Deletion criteria
```

At minimum evaluate:

| Concept | Existing competing paths |
|---|---|
| Icons | `icon_registry.rs`, `icon_provider.rs`, `app_icons.rs`, `icons.rs` |
| Workspace layout | static Panels in `lib.rs`, `tiles_workspace.rs` |
| Responsive layout | ad-hoc fixed widths, `flex_layout.rs` / egui_taffy |
| Toolbar/tool rendering | toolbar widgets, contextual shelf, viewport bar custom controls |
| Strings | raw literals, `I18n::t` |
| GPU state | WGPU per-frame rebuild vs intended dirty/revision architecture |

After migration, delete dead paths. Do not leave “new architecture” files that are never called by production.

---

# 19. Geometry statistics and Material UI

The owner specifically reports geometry statistics and materials being compressed.

## 19.1 Geometry statistics

Do not make statistics a permanently expanded block fighting for vertical space.

Use a compact summary:

```text
Mesh
1,248 verts · 2,312 tris · 1 material
```

Expand for detailed topology diagnostics.

Statistics should be cached by geometry revision.

## 19.2 Material panel

The current narrow Material PBR panel requires horizontal space that the stacked right sidebar does not provide.

After split-pane correction:

- use 2-column field rows when width allows;
- stack label/value when narrow;
- never clip numeric values;
- keep slider + numeric input aligned;
- translated labels must fit pseudo-locale tests;
- Alpha mode ComboBox gets local responsive width.

---

# 20. Visual consistency and component tokens

Audit all raw values in UI files.

Centralize semantic metrics, not just colors:

```rust
pub struct UiMetrics {
    pub control_height_small: f32,
    pub control_height: f32,
    pub icon_small: f32,
    pub icon: f32,
    pub panel_padding: f32,
    pub section_gap: f32,
    pub popover_max_width: f32,
    pub tooltip_max_width: f32,
    ...
}
```

Avoid magic numbers repeated across panels.

Support density modes only if there is a real user need; default should remain compact desktop-professional, not touch-first.

---

# 21. Responsive test matrix

UI remediation is incomplete without automated size coverage.

Test at least:

```text
1024 × 600       stress/minimum-supported candidate
1280 × 720
1366 × 768       common low-end laptop
1440 × 900
1920 × 1080
2560 × 1440
```

and DPI/UI scales:

```text
1.00
1.25
1.50
1.75
```

For each:

- Model workspace;
- Paint workspace;
- UV workspace;
- Animate workspace;
- Settings modal;
- Reference Manager;
- one open top menu;
- one ComboBox;
- narrow right dock;
- wide right dock.

Required invariant:

**No control may render outside the content rectangle it owns unless it is a deliberate popover/tooltip whose placement is clamped by egui.**

---

# 22. UI regression tests

The repository already uses `egui_kittest`. Expand it substantially.

## 22.1 Structural tests

Assert:

- right Outliner and Properties have independent rects;
- splitter changes both rects without overlap;
- status bar does not intersect viewport shelf;
- shelf rect stays within viewport rect;
- Reference Manager card rects do not intersect and maintain row/column order;
- Properties tab targets exceed minimum hit size;
- workspace switch produces expected pane IDs.

## 22.2 Tooltip coverage test

Build a test registry of semantic tools/actions and require tooltip text IDs for every icon-only command.

## 22.3 i18n coverage test

Run UI smoke tests in:

```text
en
pt-BR
pseudo
```

## 22.4 Icon coverage test

Each enabled icon pack must resolve required generic semantic icons or approved deterministic fallback.

## 22.5 Screenshot/golden tests

Use golden screenshots only for stable, high-level layout states:

- Model 1366×768 dark;
- Model 1366×768 light;
- UV 1366×768;
- Animate 1366×768;
- Reference Manager 1280×720;
- Settings 1280×720;
- pseudo-locale narrow layout.

Do not snapshot dynamic timing or noisy raster data.

---

# 23. Implementation waves

Do not attempt all fixes in one huge rewrite.

## Wave 0 — Baseline and instrumentation

Before UI refactor:

- capture current screenshots for comparison;
- record current default and low-end performance metrics;
- enable useful puffin zones;
- record idle redraw rate;
- record buffer creations/uploads per frame;
- document currently passing tests.

**Exit gate:** reproducible baseline exists.

## Wave 1 — Performance critical renderer correction

- WGPU persistent/revision-driven mesh and line buffers;
- OpenGL persistent VBO/VAO resources;
- geometry conversion caches;
- reference geometry/image upload revision tracking;
- dirty-reason tracing.

**Exit gate:** camera orbit no longer rebuilds unchanged mesh buffers.

## Wave 2 — Canonical UI regions + right dock

- introduce shell regions;
- split Outliner and Properties;
- promote `egui_tiles` or chosen canonical splitter;
- make footer/status safe area authoritative;
- persist panel proportions.

**Exit gate:** right panel no longer squeezes all content and all major rects remain non-overlapping through resize.

## Wave 3 — Real workspace profiles

- Model profile;
- Paint profile;
- UV profile;
- Animate profile;
- workspace-local pane persistence.

**Exit gate:** workspace switching is visibly and functionally meaningful.

## Wave 4 — Contextual shelf + menus/dropdowns

- remove fixed estimated widths;
- priority/overflow shelf layout;
- actual text measurement;
- menu content-aware sizing;
- ComboBox local sizing;
- footer collision tests.

**Exit gate:** shelf survives resize and pseudo-locale without clipping.

## Wave 5 — Reference Manager + Settings

- adaptive Reference Manager grid;
- responsive headers/cards;
- Settings information architecture;
- safe modal sizing;
- reset layout actions.

**Exit gate:** 1024×600 through 2560×1440 works without broken modal layout.

## Wave 6 — Icon convergence

- make semantic icon provider canonical;
- wire iconflow font assets;
- SVG domain icon path;
- remove fake pack rendering;
- remove emoji UI icons;
- icon audit and build-feature-aware Settings.

**Exit gate:** each advertised package visibly changes generic icons and zero tofu squares remain.

## Wave 7 — i18n + accessibility

- typed TextId/generator;
- remove raw user-facing literals;
- locale parity;
- pseudo-locale;
- tooltips;
- keyboard/focus semantics;
- UI scale validation.

**Exit gate:** language switch is complete and icon-only actions are discoverable.

## Wave 8 — Primitive creation contextual UI

- PrimitiveCreationSession;
- descriptor-based regeneration;
- anchored Last Operation card;
- one undo transaction;
- keyboard accept/cancel;
- viewport-safe placement.

**Exit gate:** Cube/Sphere/Cylinder/Plane have live, contextual creation controls without introducing permanent procedural-object complexity.

## Wave 9 — Final convergence/dead code deletion

- remove old icon paths no longer required;
- remove inactive layout pilots;
- update docs to match implementation;
- check Cargo dependencies for genuinely unused libraries;
- run all quality gates.

---

# 24. Gauntlet Loop protocol for Muse Spark 1.3

Repeat until all critical scores are >= 9.5 and all binary acceptance gates pass.

## Loop A — Inspect

For the current wave:

1. identify exact source ownership;
2. list observed symptoms;
3. list root causes with code references;
4. identify relevant tests;
5. identify risk to unrelated features.

## Loop B — Implement

Rules:

- small cohesive commits;
- Clean Code;
- no abbreviated function/variable names without strong domain convention;
- no UI behavior hidden in magic numbers;
- no unsafe code introduced into UI/domain layers;
- preserve Command/Undo semantics;
- preserve WGPU/OpenGL parity where applicable.

## Loop C — Test

At minimum run the repository’s applicable equivalents of:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test -p petunia_ui
```

Run existing `xtask` modernization/architecture/GA gates discovered in the repository. Do not invent an xtask command name—inspect `cargo xtask --help` / the xtask source first.

Run extended icon-pack CI in an environment with sufficient memory rather than forcing the low-memory developer machine to OOM.

## Loop D — Visual regression

Capture the required viewport matrix and compare against the previous pass.

Do not improve one screen by breaking another theme/language/size.

## Loop E — Performance

Compare to Wave 0 baseline and previous wave.

Reject a change if it produces a significant unexplained regression.

## Loop F — Score honestly

Score 1–10 independently:

```text
Architecture
Responsiveness
Visual hierarchy
Accessibility
i18n
Icon consistency
Workspace coherence
Performance CPU
Performance GPU/upload
Low-end suitability
Test coverage
Maintainability
```

Do not artificially raise a score because implementation effort was high.

Any score < 8 blocks progression to final polish.

---

# 25. Explicit Definition of Done

The remediation is not complete until all conditions below are true.

## Layout / responsiveness

- [ ] No panel overlaps Status Bar/Footer.
- [ ] Contextual shelf remains fully inside the viewport safe region.
- [ ] Shelf reflows/compacts when resizing both smaller and larger.
- [ ] Outliner and Properties are independently usable and resizable.
- [ ] Properties tabs are identifiable at all supported widths.
- [ ] Material panel remains readable at minimum supported inspector width.
- [ ] Reference Manager produces deterministic aligned rows/columns.
- [ ] Settings remains usable at minimum supported window size.
- [ ] Dropdowns/menu widths match content within sensible caps.

## Workspaces

- [ ] MODEL changes to Model profile.
- [ ] PINTURA changes to Paint profile.
- [ ] UV changes to UV profile.
- [ ] ANIMATE changes to Animate profile.
- [ ] Each profile changes tools and meaningful panes, not just a highlighted pill.
- [ ] Workspace layouts persist independently.

## Icons

- [ ] No tofu/square glyphs in normal operation.
- [ ] Petunia domain icons use the canonical domain path.
- [ ] Generic icons use the canonical iconflow provider.
- [ ] Lucide and Iconoir render their real package glyphs in normal builds.
- [ ] Tabler/Phosphor are only offered when compiled/available or clearly disabled.
- [ ] Icon package selection changes production UI, not only preview examples.
- [ ] Literal emoji product icons are removed.

## i18n

- [ ] 100% production UI text uses localization IDs.
- [ ] English and pt-BR key sets match.
- [ ] No mixed-language settings/menu/reference panel.
- [ ] Tooltips translate.
- [ ] pseudo-locale passes responsive tests.

## Accessibility

- [ ] Icon-only controls have semantic labels/tooltips.
- [ ] Focus is visible.
- [ ] Settings/dialogs are keyboard navigable.
- [ ] UI scale passes tested factors.
- [ ] active/disabled/error states do not rely only on color.

## Primitive insertion

- [ ] newly created primitive exposes contextual parameters;
- [ ] controls are anchored safely near the primitive or fall back to Tool Properties;
- [ ] parameter edits update live preview;
- [ ] insertion is one undo transaction;
- [ ] Esc cancels cleanly;
- [ ] no permanent procedural object system is introduced accidentally.

## Performance

- [ ] WGPU does not rebuild unchanged geometry buffers on camera-only frames.
- [ ] OpenGL does not create/delete asset VBOs every draw.
- [ ] reference images do not upload unchanged pixels each frame.
- [ ] idle mode does not repaint continuously without reason.
- [ ] no filesystem registry scan occurs in steady-state frame loop.
- [ ] profiler confirms no major uninvestigated regression.

## Architecture

- [ ] one canonical icon system;
- [ ] one canonical workspace layout path;
- [ ] one localization path;
- [ ] responsive layout helpers have defined ownership;
- [ ] obsolete pilots/duplicate paths are removed or explicitly documented as intentional;
- [ ] architecture docs match actual production code.

---

# 26. Files that require priority inspection

This list is not exhaustive; it is the minimum hotspot set.

```text
Cargo.toml
crates/ui/Cargo.toml

crates/ui/src/lib.rs
crates/ui/src/main_header.rs
crates/ui/src/contextual_shelf.rs
crates/ui/src/properties_panel.rs
crates/ui/src/outliner.rs
crates/ui/src/widgets.rs
crates/ui/src/toolbar.rs
crates/ui/src/viewport_bar.rs
crates/ui/src/status_bar.rs
crates/ui/src/reference_manager.rs
crates/ui/src/settings_modal.rs
crates/ui/src/tiles_workspace.rs
crates/ui/src/flex_layout.rs
crates/ui/src/icon_registry.rs
crates/ui/src/icon_provider.rs
crates/ui/src/app_icons.rs
crates/ui/src/icons.rs
crates/ui/src/tokens.rs
crates/ui/src/modules_ui/model_ui.rs
crates/ui/src/modules_ui/paint_ui.rs
crates/ui/src/modules_ui/uv_ui.rs
crates/ui/src/modules_ui/animation_ui.rs
crates/ui/tests/kittest_ui_flows.rs

crates/config/src/i18n.rs
assets/locales/en.toml
assets/locales/pt-BR.toml

crates/core/src/state.rs
crates/core/src/viewport.rs

crates/app/src/lib.rs
crates/app/src/eframe_host.rs
crates/app/src/watch.rs

crates/render-wgpu/src/lib.rs
crates/render-gl/src/lib.rs

crates/xtask/src/main.rs
crates/xtask/src/generator.rs

docs/modernization/**
docs/audits/**
PETUNIA3D_STACK_MODERNIZATION_P0_P3_GAUNTLET.md
```

---

# 27. Architectural observations that Muse must verify, not ignore

## Observation 1 — documentation/code drift

The project’s modernization documentation already requires concepts such as:

- SVG-first Petunia icons;
- `iconflow` for generic icon packs;
- complete TextId/i18n coverage;
- pseudo-locale responsive validation;
- icon-only accessibility;
- responsive sublayout candidates.

Current production code still bypasses several of these contracts.

Treat this as a release-quality bug: **a feature is not integrated merely because its dependency and pilot file exist.**

## Observation 2 — “library present” is not “library used”

The workspace contains `egui_taffy`, `iconflow`, `egui_tiles`, `puffin`, `egui_inbox`, and other modernization dependencies.

Do not mark them as integrated based on Cargo alone. Demonstrate actual production call paths and measurable benefit.

## Observation 3 — current WGPU comment encodes an obsolete assumption

The statement that rebuilding buffers every frame is “ok for low-poly” must be removed as an architectural assumption.

Petunia’s target includes low-end PCs. “Low-poly” is a content style, not permission to allocate/rebuild unchanged GPU resources every frame.

## Observation 4 — UI correctness must be translation-safe

Any layout that only works because Portuguese/English happens to fit at current text length is not responsive.

Pseudo-locale is mandatory.

---

# 28. External API references to use during implementation

## egui ComboBox

Official egui source/docs confirm that `ComboBox::width()` controls outer button/menu width and defaults to `Spacing::combo_width`. Use contextual widths instead of one uncontrolled global sizing policy.

- https://docs.rs/egui/0.36.2/egui/containers/struct.ComboBox.html
- https://docs.rs/crate/egui/0.36.2/source/src/containers/combo_box.rs

## egui repaint scheduling

Use `Context::request_repaint()` for immediate animation/state changes and `request_repaint_after()` for scheduled low-frequency refresh, while preserving the application’s sleeping event loop.

- https://docs.rs/egui/0.36.2/egui/struct.Context.html

## iconflow

`fonts()` returns enabled font assets; `try_icon()` resolves an icon; egui integration requires registering font assets and rendering with the returned named font family.

- https://docs.rs/iconflow/1.0.0/iconflow/

## egui_taffy

Use selectively for Flex/Grid responsive sublayouts.

- https://docs.rs/egui_taffy/

---

# 29. Final instruction to Muse Spark 1.3

Do not treat this document as a checklist of independent visual bugs.

The correct implementation order is:

```text
MEASURE
   ↓
FIX FRAME / GPU LIFECYCLE
   ↓
FIX CANONICAL UI REGION OWNERSHIP
   ↓
FIX WORKSPACE COMPOSITION
   ↓
FIX RESPONSIVE COMPONENTS
   ↓
CONVERGE ICON / I18N INFRASTRUCTURE
   ↓
ADD ACCESSIBILITY + PRIMITIVE CONTEXT UI
   ↓
REGRESSION + PERFORMANCE GAUNTLET
```

When a symptom can be fixed either by adding another `if width < ...` or by repairing the container/ownership model, prefer the ownership model.

When a feature exists in two implementations, converge them instead of adding a third.

When a UI action needs a string, icon, or shortcut, use semantic IDs rather than literals.

When a frame has not changed geometry, do not rebuild geometry.

When a workspace changes, the user must **feel that the workspace changed**.

When the window changes size, the UI must respond instead of merely clipping.

When a tool is represented only by an icon, the user must still be able to discover what it does.

And throughout the remediation, preserve the core Petunia principle:

> **A growing number of useful tools is acceptable; a growing number of unrelated concepts and inconsistent interaction models is not.**

The result should feel smaller and simpler than Blender even as it becomes substantially more capable.

