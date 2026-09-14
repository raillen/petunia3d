> [← Voltar ao Índice de Modernização](/modernization/) | [Documento Integral Monolítico](/modernization/full-gauntlet)

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
