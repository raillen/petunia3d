> **Documento Canônico**: Integrado à documentação oficial do Petunia3D. [Ver Documento Integral Monolítico](/modernization/full-gauntlet)

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
