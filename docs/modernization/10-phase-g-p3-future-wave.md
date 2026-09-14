> [← Voltar ao Índice de Modernização](/modernization/) | [Documento Integral Monolítico](/modernization/full-gauntlet)

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
