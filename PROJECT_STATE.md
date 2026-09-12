# Current Project State

- Project: **Petunia3D**
- Prumo: **0.5.1**
- Current phase: **P01 — Premium viewport interaction**
- Historical goal: **P01-G01**, recorded DONE before the premium specification.
- Current implementation and acceptance map: [premium interaction plan](docs/development/premium-interaction-plan.md).
- Context methodology: **Lean Progressive Context (LPC)**
- Last updated: `2026-09-12`

## Current status

The premium audit supersedes the previous claim that all required operations were
complete. The first implementation round delivers transactional modal transforms,
extrusion/inset/bevel previews, gizmos, component hover, quad loop cut, segmented
knife input, planar slicing, atomic paint strokes and corrected camera navigation.
Independent reviewers and deterministic domain/egui tests cover these changes.
Final gates: 128 tests passed; fmt, strict Clippy and smoke passed. A real GL
startup capture on Intel HD Graphics 4000 verified the corrected central viewport.
Evidence is preserved in `.prumo/history/premium/`.

Premium convergence is **not established**. The human golden path and final GPU
performance targets remain unmeasured. Complex bevel, metric inset, inter-asset
occlusion, UV texture painting in the viewport and full GPU failure handling
remain acceptance gaps. See the plan and `docs/GAUNTLET.md` for evidence and
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
