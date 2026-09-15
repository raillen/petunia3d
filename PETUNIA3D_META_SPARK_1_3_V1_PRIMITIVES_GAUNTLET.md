# Petunia3D — Meta Spark 1.3
# V1 Primitive System Validation, Remediation & Completion Gauntlet

Repository:

https://github.com/raillen/petunia3d

---

# 0. Mission

You are working directly on the current Petunia3D repository.

Your task is **not merely to add missing primitives**.

You must perform a complete implementation-level audit of the primitive creation system, validate every primitive that already exists, fix architectural/geometry/UI/UX/performance problems you find, implement the missing V1 primitives, and finish the complete primitive-authoring workflow to production quality.

You must continue implementation until the complete scope defined in this document is:

- working;
- tested;
- integrated;
- responsive;
- accessible;
- performant;
- localized;
- architecturally coherent;
- consistent with Petunia3D's product philosophy.

Do not stop after writing a report or plan.

Do not declare completion only because the repository compiles.

---

# 1. Product Context

Petunia3D is a low-poly, shape-first 3D asset creation tool.

It is **not** intended to become a smaller Blender.

Its core philosophy is:

- low cognitive load;
- beginner-friendly interaction;
- game-asset orientation;
- low-poly-first defaults;
- direct visual manipulation;
- strong feedback;
- predictable geometry;
- clean modular architecture;
- reversible operations where useful;
- minimal hidden state;
- good performance on modest computers;
- few fundamental concepts reused consistently;
- no unnecessary DCC complexity;
- no scene/world/map-editor creep.

The primitive system must reinforce this philosophy.

The objective is not to expose dozens of primitive types.

The objective is to provide a **small, extremely useful, well-designed vocabulary of shapes** from which most low-poly assets can begin.

---

# 2. Canonical V1 Primitive Set

The canonical Petunia3D V1 primitive set is:

## BASIC

1. Cube / Box
2. Plane
3. Wedge / Ramp

## ROUND

4. Cylinder
5. Cone / Frustum
6. Circle / Disc
7. Torus

## ORGANIC / ROUNDED

8. UV Sphere
9. Icosphere
10. Capsule

These ten primitives define the complete V1 scope.

Do not add unrelated primitive types unless required internally.

Do **not** add to the main V1 primitive list:

- Monkey / Suzanne;
- Terrain;
- Stairs;
- Gear;
- Arch;
- Spring;
- Text;
- Metaballs;
- Bézier surfaces;
- generalized procedural objects;
- map-building primitives;
- specialized architectural generators.

Those belong to future **Shape Presets**, **Procedural Generators**, or modules.

---

# 3. Why These Ten Primitives Exist

This section is normative product reasoning.

Do not remove a primitive because another primitive can theoretically be edited into it.

Petunia3D deliberately reduces unnecessary modeling steps for beginners.

## 3.1 Cube / Box

The Cube is the fundamental hard-surface primitive.

It is the starting point for:

- buildings;
- furniture;
- crates;
- weapons;
- machines;
- doors;
- windows;
- vehicles;
- modular environment pieces;
- stylized props.

Although the UI may call it `Cube`, its creation parameters should make it useful as a general Box.

Required dimensions:

```text
Width
Height
Depth
```

This is one of the most important primitives in the entire product.

## 3.2 Plane

The Plane is necessary for:

- floors;
- walls under construction;
- signs;
- cards;
- foliage;
- low-poly leaves;
- decals converted to geometry;
- reference modeling;
- surface patches;
- future cloth-shell workflows;
- simple environment surfaces.

Keep it extremely simple.

## 3.3 Wedge / Ramp

Wedge is intentionally included even though it can be built from a Cube.

It removes several unnecessary beginner operations.

Typical uses include:

- roofs;
- ramps;
- vehicle shapes;
- furniture;
- architectural props;
- stylized cliffs;
- support pieces;
- stairs/ramp components;
- hard-surface silhouettes.

Implementation difficulty is low and product value is high.

Do **not** turn it into a roof generator.

It remains a simple primitive.

## 3.4 Cylinder

Cylinder is fundamental for:

- pipes;
- weapon barrels;
- tree trunks;
- cans;
- bottles;
- wheels;
- mechanical shafts;
- pillars;
- stylized limbs;
- handles.

Petunia must default to a visibly low-poly Cylinder.

Recommended default:

```text
Sides = 8
```

Not 32 or 64.

## 3.5 Cone / Frustum

Cone should be exposed as a user-friendly primitive but internally designed as a more general **frustum/radial primitive**.

Typical uses:

- trees;
- roofs;
- horns;
- spikes;
- traffic cones;
- lamps;
- funnels;
- tapered props;
- stylized vegetation.

By allowing a non-zero top radius, the user can create a truncated cone without another primitive entry.

## 3.6 Circle / Disc

Circle is useful for:

- wheel profiles;
- caps;
- radial modeling starts;
- profile extrusion;
- flat discs;
- sockets;
- circular cut references;
- future spline/profile workflows.

Recommended fill options:

```text
None
Disc / Triangle Fan
```

Avoid excessive topology/fill modes.

## 3.7 Torus

Torus is useful enough for V1 because it appears frequently in game assets:

- tires;
- rings;
- handles;
- tubes;
- stylized pipes;
- bracelets;
- mechanical rings;
- circular trims.

Its parameters must remain understandable.

## 3.8 UV Sphere

UV Sphere and Icosphere are **not redundant**.

The UV Sphere is useful where predictable latitude/longitude loops matter.

Typical uses:

- heads;
- eyes;
- fruit;
- rounded props;
- objects that will be edited with horizontal loops;
- shapes where top/bottom regions should be structurally predictable.

Recommended low-poly default:

```text
Segments ≈ 12
Rings ≈ 6
```

## 3.9 Icosphere

The Icosphere better matches many stylized/low-poly aesthetics.

Typical uses:

- rocks;
- planets;
- low-poly heads;
- tree crowns;
- organic props;
- faceted objects;
- stylized natural assets.

Recommended default:

```text
Subdivision Level = 1
```

This primitive may become more commonly used than UV Sphere in Petunia's target style.

## 3.10 Capsule

Capsule is extremely valuable for:

- character bases;
- limbs;
- stylized bodies;
- handles;
- organic props;
- collision-like modeling;
- creature parts.

It also anticipates future character and collision workflows.

---

# 4. Cognitive Grouping in the UI

Do not present a long unstructured primitive list.

The canonical UI grouping is:

```text
ADD PRIMITIVE
│
├── BASIC
│   ├── Cube
│   ├── Plane
│   └── Wedge
│
├── ROUND
│   ├── Cylinder
│   ├── Cone
│   ├── Circle
│   └── Torus
│
└── ORGANIC
    ├── UV Sphere
    ├── Icosphere
    └── Capsule
```

The user sees **three understandable shape families**, not ten disconnected technical concepts.

Do not expose internal implementation concepts such as:

```text
RadialGenerator
FrustumGenerator
SphereTopologyBuilder
```

Those belong only to code.

---

# 5. Low-Poly Defaults Are a Product Requirement

Petunia3D should visually communicate:

> "This is a low-poly modeler."

through defaults.

Do not copy Blender's relatively dense defaults.

Recommended starting defaults:

```text
Cube:
1 × 1 × 1

Plane:
1 × 1

Cylinder:
8 sides

Cone:
8 sides

Circle:
8 or 12 vertices

UV Sphere:
12 segments
6 rings

Icosphere:
Subdivision 1

Capsule:
8 radial segments
low cap-segment count

Torus:
12 major segments
6 minor segments
```

Slight adjustments are acceptable if required for clean topology, but defaults must remain intentionally low-poly.

High-density topology should require a deliberate user choice.

---

# 6. First Phase — Audit Before Modification

Before changing implementation, inspect the current repository.

Determine:

- which of the ten primitives already exist;
- which are partially implemented;
- which exist in geometry code but are not exposed in UI;
- which appear in UI but have incomplete geometry;
- which use duplicated algorithms;
- which bypass canonical Commands/Undo;
- which mutate project state directly from UI code;
- which generate bad normals;
- which have inconsistent winding;
- which generate duplicate faces;
- which generate coincident vertices unnecessarily;
- which create non-manifold topology;
- which produce invalid caps;
- which use excessive polygon counts;
- which fail with minimum valid segment counts;
- which produce malformed UV data;
- which fail save/load;
- which fail export;
- which are not tested;
- which have incomplete i18n;
- which have missing tooltips;
- which have inconsistent icons;
- which trigger unnecessary project invalidation;
- which are incorrectly coupled to egui;
- which rebuild unrelated data during preview;
- which violate current architectural contracts.

Inspect implementation, not only documentation.

Search at minimum:

```text
crates/mesh
crates/core
crates/project
crates/ui
crates/module-model or equivalent
render backends
Command infrastructure
serialization
OBJ export
GLB/glTF export
tests
docs
```

Do not trust `COMPLIANT`, comments, changelogs, or previous implementation claims.

Verify behavior.

---

# 7. Required Audit Matrix

Before implementation, produce a working matrix:

| Primitive | Exists | Geometry | Parameters | Preview | Undo | Save/Load | Export | UI | i18n | Tests | Action |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Cube | | | | | | | | | | | |
| Plane | | | | | | | | | | | |
| Wedge | | | | | | | | | | | |
| Cylinder | | | | | | | | | | | |
| Cone | | | | | | | | | | | |
| Circle | | | | | | | | | | | |
| Torus | | | | | | | | | | | |
| UV Sphere | | | | | | | | | | | |
| Icosphere | | | | | | | | | | | |
| Capsule | | | | | | | | | | | |

This matrix guides implementation.

Do not stop after producing it.

---

# 8. Architecture — Do Not Create Ten Independent Geometry Systems

The UI exposes ten primitives.

The implementation should use a smaller number of robust generators where appropriate.

Conceptual architecture:

```text
PrimitiveGenerator
│
├── Box-family generator
│   ├── Cube / Box
│   └── Wedge
│
├── Plane/Profile generator
│   ├── Plane
│   └── Circle / Disc
│
├── Radial / Frustum generator
│   ├── Cylinder
│   ├── Cone
│   └── Frustum
│
├── Sphere generators
│   ├── UV Sphere
│   └── Icosphere
│
├── Capsule generator
│
└── Torus generator
```

This diagram is conceptual.

Adapt it to the existing codebase.

Do not force unnecessary abstraction.

However, duplicated geometry math should be eliminated where a common implementation is clearly appropriate.

---

# 9. Cylinder, Cone and Frustum Must Share a Radial Foundation

Do not maintain duplicated Cylinder and Cone algorithms unless technically unavoidable.

Prefer a common radial/frustum generator with parameters similar to:

```text
bottom_radius
top_radius
height
sides
cap_bottom
cap_top
```

Then:

```text
Cylinder
bottom_radius = 1
top_radius    = 1

Cone
bottom_radius = 1
top_radius    = 0

Frustum
bottom_radius = 1
top_radius    = 0.5
```

The UI still exposes:

```text
Cylinder
Cone
```

The user does not need a separate Frustum menu item.

Changing `Top Radius` on Cone naturally creates a frustum.

This provides more capability without increasing cognitive load.

---

# 10. Primitive Parameter Model

Primitive-specific creation fields must not be scattered randomly across UI code.

Use or create a typed authoring parameter model.

Conceptual example:

```rust
pub enum PrimitiveParameters {
    Box(BoxParameters),
    Plane(PlaneParameters),
    Wedge(WedgeParameters),
    Cylinder(CylinderParameters),
    Cone(ConeParameters),
    Circle(CircleParameters),
    Torus(TorusParameters),
    UvSphere(UvSphereParameters),
    Icosphere(IcosphereParameters),
    Capsule(CapsuleParameters),
}
```

Adapt this to existing Petunia architecture.

Do not create this exact enum if an existing canonical abstraction is better.

The important requirements are:

- typed parameters;
- clear ownership;
- deterministic regeneration;
- serializable creation state only when appropriate;
- no UI-owned geometry state;
- reusable validation.

---

# 11. Required Parameters — Cube / Box

At minimum:

```text
Width
Height
Depth
```

Optional convenience:

```text
Uniform Size
```

Default:

```text
1 × 1 × 1
```

Geometry must remain centered/predictable according to Petunia's origin contract.

---

# 12. Required Parameters — Plane

At minimum:

```text
Width
Depth
```

If subdivisions are already cleanly supported:

```text
Segments X
Segments Y
```

Do not introduce subdivisions merely because Blender has them.

Keep default topology minimal.

---

# 13. Required Parameters — Wedge

At minimum:

```text
Width
Height
Depth
```

Optional:

```text
Slope Direction
```

only if required by UX.

Generate clean topology with:

- correct normals;
- no degenerate slope face;
- no duplicate coplanar faces;
- predictable origin.

---

# 14. Required Parameters — Cylinder

At minimum:

```text
Radius
Height
Sides
Cap Top
Cap Bottom
```

Recommended default:

```text
Sides = 8
```

Test at minimum:

```text
Sides = 3
Sides = 6
Sides = 8
Sides = 32
```

---

# 15. Required Parameters — Cone / Frustum

At minimum:

```text
Bottom Radius
Top Radius
Height
Sides
Cap Bottom
Cap Top
```

Default:

```text
Top Radius = 0
Sides = 8
```

When:

```text
Top Radius > 0
```

the preview becomes a frustum.

Do not add a second primitive entry.

---

# 16. Required Parameters — Circle / Disc

At minimum:

```text
Radius
Vertices
Fill
```

Recommended fill modes:

```text
None
Disc
```

`Disc` can use a triangle fan or whichever clean topology fits Petunia's mesh representation.

Do not expose unnecessary triangulation modes.

---

# 17. Required Parameters — UV Sphere

At minimum:

```text
Radius
Segments
Rings
```

Recommended defaults:

```text
Segments = 12
Rings = 6
```

Audit carefully:

- poles;
- duplicated pole vertices;
- winding;
- UV seam behavior;
- smooth/flat normals;
- degenerate triangles.

---

# 18. Required Parameters — Icosphere

At minimum:

```text
Radius
Subdivision Level
```

Recommended default:

```text
Subdivision Level = 1
```

Use a reasonable upper bound.

The UI should communicate that each subdivision level increases topology rapidly.

Do not allow accidental extreme mesh generation.

---

# 19. Required Parameters — Capsule

At minimum:

```text
Radius
Body Length / Height
Radial Segments
Cap Segments
```

Recommended radial default:

```text
8
```

Audit especially:

- body/hemisphere seam;
- duplicated rings;
- normals;
- zero-length body edge cases;
- minimum cap segments;
- smooth shading behavior.

---

# 20. Required Parameters — Torus

At minimum:

```text
Major Radius
Minor Radius
Major Segments
Minor Segments
```

Recommended defaults:

```text
Major Segments = 12
Minor Segments = 6
```

Validate:

```text
major_radius > 0
minor_radius > 0
major_segments >= minimum
minor_segments >= minimum
```

Avoid NaNs and malformed topology.

---

# 21. Primitive Creation UX Is a Critical V1 Requirement

Primitive creation must not be:

```text
click primitive
→ final mesh appears
→ parameters are lost
→ user must delete/recreate to change basic dimensions
```

Instead:

```text
Add Primitive
↓
temporary creation session
↓
live preview
↓
contextual parameters
↓
Done
```

or:

```text
Esc / Cancel
```

This is a temporary authoring workflow.

The final object becomes an ordinary editable mesh.

---

# 22. Contextual Primitive Panel

Immediately after adding a primitive, present a contextual panel near the primitive or in a reliably anchored viewport region.

Example:

```text
┌─ Cylinder ─────────────────┐
│ Radius             1.00    │
│ Height             2.00    │
│ Sides                 8    │
│ Cap                Both ▼  │
│                            │
│ Verts 16 · Faces 10        │
│                            │
│ [ Reset ]         [ Done ] │
└────────────────────────────┘
```

The panel must:

- follow viewport resizing;
- never overlap the footer;
- never become detached from its anchor;
- clamp to viewport boundaries;
- work in narrow windows;
- work with longer translated labels;
- avoid fixed width assumptions;
- avoid `label.len() * constant` sizing.

If there is insufficient space near the primitive, reposition intelligently.

---

# 23. Primitive Creation Session

Implement or consolidate a real creation session.

Conceptually:

```rust
PrimitiveCreationSession {
    primitive_kind,
    parameters,
    transform,
    preview_mesh,
}
```

Lifecycle:

```text
Begin
↓
Preview
↓
Edit Parameters
↓
Regenerate Preview
↓
Confirm
or
Cancel
```

Requirements:

- preview updates do not spam Undo history;
- one final commit creates one Undo transaction;
- Cancel leaves the document exactly as before;
- no hidden object remains after Cancel;
- no leaked preview resource;
- Reset restores defaults;
- Enter/Done confirms where appropriate;
- Escape cancels where appropriate;
- switching tools resolves the session predictably.

Reuse existing Petunia modal/transaction infrastructure where appropriate.

Do not invent a second unrelated interaction state machine.

---

# 24. Live Visual Feedback

Every parameter change must update the viewport immediately.

Examples:

```text
Cylinder Sides:
6 → 8 → 12
```

must visibly regenerate the cylinder.

```text
Cone Top Radius:
0 → 0.5
```

must visibly transform Cone → Frustum.

```text
Icosphere Subdivision:
0 → 1 → 2
```

must update geometry and statistics.

The user should never wonder whether a parameter has taken effect.

---

# 25. Geometry Statistics During Creation

Where layout allows, show lightweight geometry information:

```text
Verts: 16
Faces: 10
Tris: 28
```

This reinforces Petunia's low-poly identity.

On constrained layouts:

- collapse into one line;
- move into tooltip;
- move into expandable details.

Do not remove essential creation controls to preserve statistics.

---

# 26. Placement

Audit existing placement behavior.

Primitive insertion must follow one canonical rule.

Prefer Petunia's current intended:

```text
3D Cursor / insertion point
```

if that is the existing specification.

Do not inconsistently create some primitives at:

- world origin;
- selected object center;
- camera focus;
- viewport center;
- 3D cursor.

The insertion contract must be unified.

---

# 27. Pivot / Origin

All primitives need predictable local coordinates and origin.

Default should generally be geometric center unless canonical Petunia documentation defines otherwise.

For each primitive explicitly test:

- origin;
- bounds;
- dimensions;
- transform behavior;
- gizmo placement.

Do not introduce strange per-primitive offsets.

---

# 28. Geometry Quality Invariants

Every generated primitive must satisfy:

- deterministic vertex ordering;
- deterministic face ordering;
- valid indices;
- outward face winding;
- outward normals;
- no accidental duplicate faces;
- no accidental coincident vertices;
- no zero-area faces;
- no degenerate triangles above tolerance;
- no unexpected non-manifold topology;
- no invalid caps;
- no NaN coordinates;
- no infinity coordinates;
- finite bounds;
- valid minimum topology;
- stable triangulation;
- predictable orientation.

Create shared geometry validation helpers where appropriate.

---

# 29. Normals

Validate:

```text
Flat Shading
Smooth Shading
```

where applicable.

Do not rely on accidental geometry duplication to make shading appear correct.

Respect Petunia's existing normals/shading architecture.

---

# 30. UV Behavior

Do not derail this task into a UV rewrite.

However:

- do not create invalid UV arrays;
- preserve existing UV contracts;
- validate primitives that already generate UVs;
- generate minimal sane UVs only if that is already canonical behavior;
- do not invent a second UV system.

If primitive UV generation is intentionally deferred, document it clearly.

---

# 31. Undo / Redo

Expected workflow:

```text
Add Cylinder
→ Radius 1.0 → 0.8
→ Sides 8 → 6
→ Height 2 → 1.5
→ Done
```

Undo should perform:

```text
Remove Cylinder
```

in one coherent transaction.

Do not create an Undo history like:

```text
Undo Height
Undo Height
Undo Sides
Undo Radius
Undo Add Cylinder
```

for temporary creation preview.

Redo restores the final created primitive.

---

# 32. Cancel Behavior

Example:

```text
Add Torus
→ change parameters
→ Esc
```

Expected result:

```text
No Torus
No hidden asset
No Undo pollution
No leaked temporary state
No stale selection
No renderer resource leak
```

Test this behavior explicitly.

---

# 33. Primitive Finalization

After `Done`, the primitive must become a normal Petunia mesh.

The user can immediately:

- select it;
- move;
- rotate;
- scale;
- enter Edit Mode;
- select vertices;
- select edges;
- select faces;
- Extrude;
- Inset;
- Bevel;
- assign materials;
- paint later;
- UV edit later;
- duplicate;
- export;
- save/load.

No special permanent primitive object should remain.

---

# 34. Do Not Turn V1 Primitives Into Permanent Procedural Objects

Canonical V1 model:

```text
primitive creation
→ temporary parametric creation session
→ regular editable mesh
```

Do not build a permanent procedural primitive dependency system as part of this task.

That is a separate future architectural decision.

---

# 35. Workspace Integration

Primitive creation belongs primarily in:

```text
MODEL
```

Audit workspace behavior.

Do not let primitive-specific creation UI accidentally leak into:

```text
PAINT
UV
ANIMATE
```

unless a global Add command is intentionally available.

The contextual primitive panel must respect current workspace switching.

---

# 36. Creation Menu UI

Provide an understandable primitive selector.

Canonical grouping:

```text
BASIC
Cube
Plane
Wedge

ROUND
Cylinder
Cone
Circle
Torus

ORGANIC
UV Sphere
Icosphere
Capsule
```

Do not overwhelm the user.

Do not place future procedural generators in this menu.

---

# 37. Responsive Behavior

Test at minimum:

```text
1024 × 600
1280 × 720
1366 × 768
1600 × 900
1920 × 1080
2560 × 1440
```

Also test UI scale where supported:

```text
100%
125%
150%
175%
```

The primitive selector and contextual panel must not:

- overlap footer;
- overlap Outliner;
- overlap Properties;
- clip essential controls;
- disappear offscreen;
- assume English-length labels;
- break after resizing smaller then larger.

---

# 38. i18n

No new production-facing primitive text may be hardcoded in Rust.

Add proper translation keys.

At minimum complete:

```text
English
Português do Brasil
```

for:

- primitive group names;
- primitive names;
- all parameters;
- tooltips;
- Reset;
- Done;
- Cancel;
- Fill;
- Cap;
- Radius;
- Width;
- Height;
- Depth;
- Sides;
- Segments;
- Rings;
- Subdivision Level;
- Major Radius;
- Minor Radius;
- Body Length;
- Top Radius;
- Bottom Radius;
- error/validation messages.

Audit old primitive-related hardcoded strings and migrate them.

Do not rely silently on English fallback.

---

# 39. Pseudo-Locale / Long Text

If Petunia already has or is introducing pseudo-locale testing, include the primitive UI.

The creation panel must survive significantly longer labels.

Do not size UI based on current Portuguese/English text lengths.

---

# 40. Icons

Use the canonical semantic icon architecture.

The repository currently has historical/overlapping icon implementations.

Do not add another one.

Audit and use the canonical provider being established by the UI remediation work.

Rules:

- Petunia-specific primitive/domain icons may use bundled canonical assets;
- generic controls use semantic generic icons;
- selected icon pack changes appearance, not command meaning;
- icon-only controls require tooltip/accessibility label;
- never use emoji as permanent replacement for a semantic icon when a real icon exists.

---

# 41. Tooltips

Every non-obvious control must explain itself.

Examples:

### Sides

> Number of sides around the shape. Lower values create a more visibly low-poly result.

### Top Radius

> Radius of the upper ring. Set it to zero to create a cone.

### Subdivision Level

> Each level significantly increases the number of faces in the Icosphere.

### Major Radius

> Distance from the Torus center to the center of its tube.

### Minor Radius

> Radius of the Torus tube.

Tooltips must be:

- concise;
- beginner-friendly;
- translated;
- technically correct.

---

# 42. Numeric Fields

Reuse Petunia's canonical numeric-field behavior.

Numeric input should support the canonical interactions already defined by the project, ideally:

```text
click → exact text edit
drag/scrub → adjust
keyboard → adjust
reset → default
```

Do not create a primitive-only numeric field implementation.

---

# 43. Error Handling and Valid Ranges

Invalid primitive parameters must never produce malformed geometry.

Examples:

```text
radius <= 0
height <= 0
sides < 3
rings below minimum
major_radius <= 0
minor_radius <= 0
subdivision beyond supported maximum
```

Use:

- clamping;
- disabled invalid actions;
- inline diagnostics;
- safe defaults.

Never silently generate corrupt geometry.

---

# 44. Sensible Complexity Limits

Protect beginners and low-end systems.

Set justified upper bounds for:

```text
Cylinder sides
Cone sides
Circle vertices
UV Sphere segments
UV Sphere rings
Icosphere subdivision
Capsule segments
Torus major segments
Torus minor segments
```

Limits should reflect:

- Petunia's low-poly purpose;
- preview responsiveness;
- memory;
- renderer cost.

Do not allow accidental multi-million-face primitives.

---

# 45. Performance — Primitive Preview

Primitive parameter scrubbing must feel immediate.

During preview:

- regenerate only preview geometry;
- do not rebuild all project meshes;
- do not rebuild unrelated scene state;
- do not upload unrelated GPU buffers;
- do not invalidate all render resources;
- do not cause full project serialization;
- do not run expensive validators every frame.

Use revision-based invalidation where available.

A preview update should have a narrowly scoped cost.

---

# 46. Important Performance Context

The broader Petunia audit already identified renderer performance concerns involving repeated geometry/buffer rebuilding.

Do not amplify those problems.

When possible:

```text
parameter change
↓
preview mesh revision changes
↓
only preview GPU resource changes
```

Camera movement or unrelated UI changes must not regenerate primitive geometry.

---

# 47. Unit Tests — Cube

Test:

- expected bounds;
- expected dimensions;
- correct face orientation;
- manifold topology;
- finite positions;
- correct origin.

---

# 48. Unit Tests — Plane

Test:

- orientation;
- width/depth;
- face winding;
- finite coordinates;
- minimal topology.

---

# 49. Unit Tests — Wedge

Test:

- expected topology;
- slope face;
- outward normals;
- no zero-area faces;
- expected bounds.

---

# 50. Unit Tests — Cylinder

Test at minimum:

```text
3 sides
6 sides
8 sides
32 sides
cap top on/off
cap bottom on/off
```

Validate counts and topology.

---

# 51. Unit Tests — Cone / Frustum

Test:

```text
top_radius = 0
top_radius > 0
equal top/bottom radius where shared generator allows it
caps combinations
minimum sides
```

---

# 52. Unit Tests — Circle

Test:

```text
unfilled
filled
minimum valid vertices
default vertices
```

---

# 53. Unit Tests — UV Sphere

Test:

```text
minimum valid segments/rings
default topology
pole handling
normals
finite positions
face winding
```

---

# 54. Unit Tests — Icosphere

Test:

```text
subdivision 0
subdivision 1
subdivision 2
```

Verify predictable topology growth.

Also validate projected vertices remain on the expected radius within tolerance.

---

# 55. Unit Tests — Capsule

Test:

```text
short body
normal body
minimum radial segments
different cap segments
```

Validate body/cap seam.

---

# 56. Unit Tests — Torus

Test:

```text
minimum major/minor segments
default topology
different radii
```

Validate ring closure and seam topology.

---

# 57. Property-Based Geometry Tests

Where appropriate, use property-based tests over valid parameter ranges.

Assert:

```text
all positions finite
all indices in bounds
all faces valid
no zero-area triangle above tolerance
bounds finite
normals finite
deterministic result for same input
```

Use deterministic fixture tests as well.

Property-based tests supplement rather than replace known-case tests.

---

# 58. UI Tests

Use Petunia's egui testing infrastructure.

Test:

- primitive menu renders;
- all ten primitives exist;
- grouping is correct;
- each entry starts creation;
- parameters appear;
- parameter changes update session state;
- Reset restores defaults;
- Done commits;
- Cancel cancels;
- Enter behavior if supported;
- Escape behavior;
- narrow window;
- long translation labels;
- icon-only tooltip presence;
- keyboard focus;
- switching workspace resolves creation correctly.

---

# 59. Integration Tests

For each primitive verify:

```text
Create
↓
Confirm
↓
Select
↓
Transform
↓
Enter Edit Mode
↓
Save
↓
Load
↓
Export OBJ / GLB where supported
```

Do not assume a valid in-memory mesh guarantees pipeline compatibility.

---

# 60. Screenshot / Visual Gauntlet

Where project tooling supports it, capture evidence for:

```text
Cube
Wedge
Cylinder 6 sides
Cone
Frustum
UV Sphere
Icosphere
Capsule
Torus
Primitive menu
Creation panel
Narrow screen
Portuguese
English
```

Visual testing is supplemental.

Do not use screenshots as the only source of correctness.

---

# 61. Accessibility

Primitive creation controls must provide:

- visible focus;
- keyboard navigation;
- accessible names;
- tooltips for icon-only buttons;
- editable numeric values;
- text alternatives;
- active state not communicated only by color;
- sufficient click targets;
- understandable disabled states.

Avoid tiny square icon buttons with no explanation.

---

# 62. UI Consistency

Use Petunia's existing design system:

- spacing tokens;
- radius tokens;
- typography tokens;
- semantic colors;
- canonical buttons;
- canonical numeric fields;
- canonical dropdowns;
- canonical icon rendering.

Do not create a local visual language specifically for primitive creation.

---

# 63. Dropdowns

Primitive dropdowns such as:

```text
Cap
Fill
```

must not become unnecessarily wide.

Use content-aware/local widths.

Test in both languages.

Avoid inheriting oversized parent widths accidentally.

---

# 64. Contextual UI Behavior

The contextual primitive panel should be considered part of a more general future **Contextual Operation Panel** pattern.

Design it so the interaction model could later be reused by:

- Add Collider;
- Add Modifier;
- Generator creation;
- Decal placement;
- Spline creation;
- potentially other creation operations.

Do not prematurely generalize into a huge framework.

But avoid a one-off primitive popup architecture that cannot be reused.

---

# 65. Architecture Review During Implementation

Fix primitive-related architectural problems discovered during the audit.

Examples:

- UI directly creates topology;
- duplicated mesh algorithms;
- primitives bypass Commands;
- egui types leak into geometry/core;
- renderer contains primitive generation;
- parameter state duplicated in UI and domain;
- hardcoded strings;
- icon system bypass;
- creation preview stored as normal permanent project object;
- whole-scene dirty invalidation for local preview.

Do not use this task as an excuse to rewrite unrelated systems.

---

# 66. Clean Code Requirements

Use idiomatic Rust.

Requirements:

- descriptive names;
- no cryptic abbreviations;
- focused functions;
- explicit data ownership;
- minimal unnecessary cloning;
- no hidden mutable global state;
- no unnecessary `unwrap`;
- no duplicated geometry math;
- no magic numbers without named meaning;
- no swallowed errors;
- no UI/domain coupling;
- no comments pretending unfinished code is complete.

Optimize for maintainability, not minimum line count.

---

# 67. Recommended Internal Modules

Adapt to the existing structure, but a healthy conceptual separation could look like:

```text
mesh/
└── primitives/
    ├── mod.rs
    ├── box.rs
    ├── plane.rs
    ├── radial.rs
    ├── uv_sphere.rs
    ├── icosphere.rs
    ├── capsule.rs
    ├── torus.rs
    └── validation.rs

core/application/
└── primitive_creation.rs

ui/
├── primitive_menu.rs
└── primitive_creation_panel.rs
```

Do not force these exact file names.

The separation matters more than the names.

---

# 68. Command / Transaction Semantics

The final confirmed primitive creation must use canonical Petunia state mutation.

Conceptually:

```text
BeginPrimitiveCreation
UpdatePrimitivePreview
ConfirmPrimitiveCreation
CancelPrimitiveCreation
```

Preview-only changes do not become history entries.

Confirmation creates one document mutation.

Avoid direct mutation from egui widgets when a canonical Command/application path exists.

---

# 69. Save / Load

Only the final confirmed mesh needs normal project persistence for V1.

If creation-session state is not intentionally persisted across application restarts, do not serialize it unnecessarily.

However:

- no partially confirmed preview should accidentally enter project save;
- no canceled preview should survive save/load;
- confirmed mesh must round-trip correctly.

---

# 70. Export

Every confirmed primitive should behave like a normal mesh.

Verify:

```text
OBJ
GLB / glTF
```

where those exporters are supported.

Check:

- scale;
- normals;
- triangulation;
- material association;
- UV data if present.

---

# 71. Documentation

After implementation, update canonical Petunia documentation.

Document:

- canonical ten-primitive V1 list;
- why each exists;
- UI grouping;
- low-poly defaults;
- shared generator architecture;
- Cone/Frustum behavior;
- Primitive Creation Session;
- contextual panel;
- Undo semantics;
- validation limits;
- performance behavior;
- tests;
- non-goals.

Documentation must match implementation reality.

---

# 72. Do Not Implement These as V1 Primitives

Explicit non-goals:

```text
Grid as separate primitive
Monkey / Suzanne
Gear
Star
Pyramid as separate primitive
Tube as separate primitive
Pipe generator
Arch generator
Spring
Terrain
Stairs
Text
Metaballs
Bézier Surface
Procedural building elements
```

Many of these can later become:

```text
Shape Presets
Procedural Generators
Spline-based Generators
Official Modules
```

---

# 73. Future Shape Presets / Generators

Do not implement these now, but keep architecture compatible with the future idea:

```text
Procedural Shapes
├── Stair
├── Arch
├── Pipe
├── Gear
├── Fence
├── Roof
├── Door
├── Window
└── Tree Trunk
```

These are conceptually different from fundamental primitives.

Do not mix them into the V1 primitive menu.

---

# 74. Why the Contextual Creation Panel Matters More Than More Primitives

Petunia's product value comes from reducing cognitive cost.

A polished workflow for ten primitives provides more value than a menu with thirty poorly designed primitives.

The most important interaction is:

```text
Add
↓
see shape
↓
understand parameters
↓
adjust visually
↓
confirm
```

This is more important than maximizing primitive count.

---

# 75. Compiler and Test Validation

Run the applicable repository checks.

At minimum:

```bash
cargo fmt --all -- --check

cargo check --workspace --all-targets

cargo clippy --workspace --all-targets --all-features -- -D warnings

cargo test --workspace
```

Also run relevant:

- xtask checks;
- architecture checks;
- UI conformance tests;
- export tests;
- screenshot tests;
- performance probes;

that already exist in Petunia.

Adapt commands to project tooling where required.

---

# 76. No Fake Completion

Do not claim success because:

```text
cargo check
```

passes.

Completion requires actual behavior.

Do not hide:

- broken preview;
- untranslated strings;
- missing primitive;
- failed tests;
- poor performance;
- invalid normals;
- broken export;
- incomplete cancellation;
- UI overlap.

---

# 77. Gauntlet Loop

Use an iterative Gauntlet Loop.

After every major wave, independently score:

```text
Geometry Correctness
Architecture
UX
Visual Feedback
Responsiveness
Accessibility
i18n
Performance
Undo/Redo
Persistence
Export Compatibility
Test Coverage
Code Quality
Documentation
```

Score:

```text
1–10
```

Do not inflate scores.

For every score below 10:

1. identify concrete reasons;
2. prioritize impact;
3. fix what is within scope;
4. rerun tests;
5. reevaluate.

A score of 10 means:

> No known meaningful defect remains within the evaluated scope.

It does not mean:

> "The code compiled."

---

# 78. Implementation Waves

## Wave 0 — Repository Audit

Inventory every primitive implementation and architectural path.

Produce the audit matrix.

## Wave 1 — Primitive Geometry Foundation

Consolidate:

- shared generators;
- typed parameters;
- common validation;
- low-poly defaults;
- deterministic topology.

## Wave 2 — Existing Primitive Remediation

Repair existing:

```text
Cube
Plane
and any other current primitive
```

before adding new systems on broken foundations.

## Wave 3 — Basic and Radial Completion

Implement/fix:

```text
Wedge
Cylinder
Cone / Frustum
Circle / Disc
```

## Wave 4 — Rounded / Organic Completion

Implement/fix:

```text
UV Sphere
Icosphere
Capsule
Torus
```

## Wave 5 — Primitive Creation Session

Implement:

```text
Begin
Preview
Edit
Reset
Confirm
Cancel
Undo transaction
```

## Wave 6 — Contextual Creation UX

Implement:

- creation panel;
- viewport anchoring;
- boundary clamping;
- live statistics;
- responsive behavior;
- no footer overlap.

## Wave 7 — Primitive Menu & Workspace Integration

Implement:

- BASIC;
- ROUND;
- ORGANIC groups;
- Model workspace integration;
- active tool behavior;
- semantic icons.

## Wave 8 — i18n & Accessibility

Complete:

```text
English
pt-BR
```

and accessibility semantics.

## Wave 9 — Performance

Profile:

- primitive regeneration;
- GPU updates;
- unnecessary invalidation;
- resizing;
- high-segment cases.

## Wave 10 — Persistence & Export

Validate:

- Save/Load;
- OBJ;
- GLB/glTF;
- regular mesh behavior.

## Wave 11 — Final Testing & Documentation

Run full tests.

Synchronize documentation.

Run final Gauntlet.

---

# 79. Definition of Done — Primitive Coverage

All ten must exist and be validated:

- Cube / Box
- Plane
- Wedge / Ramp
- Cylinder
- Cone / Frustum
- Circle / Disc
- Torus
- UV Sphere
- Icosphere
- Capsule

---

# 80. Definition of Done — Geometry

For every primitive:

- finite geometry;
- valid indices;
- valid winding;
- valid normals;
- valid bounds;
- deterministic output;
- no accidental degeneracy;
- correct minimum parameter handling;
- intentionally low-poly defaults.

---

# 81. Definition of Done — UX

- grouped primitive menu exists;
- creation panel exists;
- preview is live;
- Reset works;
- Done works;
- Cancel works;
- no footer overlap;
- resizing works;
- parameters are understandable;
- tooltips exist;
- geometry statistics are available where practical.

---

# 82. Definition of Done — Architecture

- geometry does not depend on egui;
- UI does not duplicate geometry algorithms;
- primitive creation has one canonical path;
- preview does not pollute Undo;
- final creation creates one coherent transaction;
- no unnecessary parallel icon/i18n/numeric systems were created.

---

# 83. Definition of Done — Pipeline

- selection works after creation;
- transforms work;
- Edit Mode works;
- modeling tools work;
- materials work;
- save/load works;
- export works.

---

# 84. Definition of Done — Quality

- unit tests exist;
- UI tests exist;
- integration tests exist;
- relevant property tests exist;
- PT-BR complete;
- English complete;
- accessibility validated;
- performance regression absent;
- documentation synchronized;
- no known P0/P1 primitive defect remains.

---

# 85. Final Report

When the implementation is genuinely complete, provide:

```text
1. Initial audit findings
2. Existing primitives found
3. Existing primitives repaired
4. Missing primitives implemented
5. Geometry architecture changes
6. Primitive Creation Session changes
7. UI/UX changes
8. Responsive layout changes
9. Performance fixes
10. i18n changes
11. Accessibility changes
12. Tests added
13. Test commands executed
14. Export/save-load validation
15. Remaining limitations
16. Final Gauntlet scores
```

Reference concrete file paths and test evidence.

Do not hide unresolved issues.

---

# 86. Product Boundary

Keep this statement visible during implementation:

> **Petunia3D should become exceptionally good at creating individual game-ready low-poly assets, not become a Blender clone or a game-engine level editor.**

The primitive system must remain:

```text
small
powerful
predictable
low-poly-first
visually understandable
easy to edit
easy to learn
fast
```

Prefer making these ten primitives excellent over adding thirty mediocre ones.

---

# 87. Final Instruction

Begin by auditing the **current repository implementation**.

Do not assume previous implementation claims are accurate.

Then proceed through the implementation waves.

Do not stop at planning.

Do not ask for confirmation between waves unless a genuinely destructive or ambiguous architectural choice cannot be resolved from the repository and canonical Petunia documentation.

Continue until the primitive system satisfies the complete Definition of Done in this document.
