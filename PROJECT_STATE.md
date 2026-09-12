# Current Project State

- Project: **Petunia3D**
- Prumo: **0.5.1**
- Current phase: **P01 — Premium viewport interaction**
- Canonical UI Golden Reference: [`docs/image-references/Blender.svg`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/image-references/Blender.svg) (component catalog in [`docs/image-references/extracted/`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/image-references/extracted/))
- Historical goal: **P01-G01**, recorded DONE before the premium specification.
- Current implementation and acceptance map: [premium interaction plan](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/development/premium-interaction-plan.md).
- Context methodology: **Lean Progressive Context (LPC)**
- Last updated: `2026-09-12`

## Current status

The premium audit supersedes the previous claim that all required operations were
complete. The visual target for final production is formalised as the 1920×1080
layout of [`Blender.svg`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/image-references/Blender.svg),
with 268 component SVGs cataloged and verified in [`docs/image-references/extracted/index.html`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/image-references/extracted/index.html).

The first implementation round delivers transactional modal transforms,
extrusion/inset/bevel previews, gizmos, component hover, quad loop cut, segmented
knife input, planar slicing, atomic paint strokes and corrected camera navigation.
The second implementation round delivers the native vector icon engine ([`crates/ui/src/icons.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/icons.rs)),
accessible `tool_button` with compact/expanded layouts, explicit orthographic camera
controls with 6 orthogonal presets, numeric tool property fields ([`crates/ui/src/tool_fields.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/tool_fields.rs)),
and responsive panel refactoring.
The third implementation round (Gauntlet Loop R2) delivers the interactive Navigation
Orientation Gizmo ([`crates/ui/src/nav_gizmo.rs`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/crates/ui/src/nav_gizmo.rs)) with 6 depth-sorted world axes,
zoom/pan/perspective action handles, 3D Cursor overlay with `Shift+RMB` placement,
contextual RMB menus for sub-elements and objects, and direct Viewport Shading mode selectors.
Final gates: 132 tests passed; fmt, strict Clippy (-D warnings) and headless smoke test passed.
Evidence is preserved in `.prumo/history/premium/`, [`docs/GAUNTLET.md`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/GAUNTLET.md) and [`docs/GAUNTLET_HANDOFF.md`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/GAUNTLET_HANDOFF.md).

Premium convergence is **not established**. The human golden path and final GPU
performance targets remain unmeasured. Complex bevel, metric inset, inter-asset
occlusion, UV texture painting in the viewport and full GPU failure handling
remain acceptance gaps. See the plan and [`docs/GAUNTLET.md`](file:///home/raillen/Documentos/Projetos/simple3d-modeling/docs/GAUNTLET.md) for evidence and
limitations; do not reuse historical scores as proof of this specification.

## Next action

Use the remaining acceptance matrix in the premium plan. Validate the graphical
interaction and performance on both backends; extend unsupported geometry with
regression tests before raising any score or declaring convergence.

## Recovery order

1. `ENTRYPOINT.md` or platform adapter.
2. `prumo.json` and this state file.
3. `docs/PRUMO.md` and the premium interaction plan.
4. Historical goals in `.ai/goals/` (their DONE state does not close premium work).
5. Only relevant canonical docs, symbols and tests.
