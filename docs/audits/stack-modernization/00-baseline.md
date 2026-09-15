# 00-baseline: Pre-Migration Stack & Ergonomics Baseline Audit

- **Date:** 2026-09-14
- **Host / Target:** Linux x86_64 / Arch Linux
- **Rust Toolchain:** `rustc 1.98.1 (48a229cea 2026-09-01)` / `cargo 1.98.1`
- **Baseline Git Rev:** `c38f2ac`
- **Initial Workspace Crates:** 18 (root `petunia3d`, 17 crates in `crates/`)

## 1. Direct Dependency Baseline
- `egui`: 0.32
- `egui-winit`: 0.32
- `egui-wgpu`: 0.32
- `egui_glow`: 0.32
- `wgpu`: 25.0.2 (via egui-wgpu 0.32.3)
- `glam`: 0.27
- `image`: 0.25
- `rfd`: 0.15
- `egui-file-dialog`: 0.8
- `transform-gizmo-egui`: 0.7
- `egui_tiles`: 0.12
- `egui_ltreeview`: 0.4

## 2. Test & Quality Baseline
- Automated Tests: 277 passed (88 unit ui, 17 kittest ui flows, 60 mesh, 42 core, 11 commands, etc.).
- Clippy: 0 warnings with `-D warnings`.
- Architecture & Manifest Invariants: 100% compliant (`arch-check` passed).
- Living Docs: 100% compliant (`docs-check` / VitePress build passed in 71.92s).

## 3. Subsystem Ownership Boundary Status
- Renderer: Dual backend (WGPU primary + OpenGL/glow fallback).
- UI Sovereignty: Strictly isolated in `crates/ui` and `crates/app`.
- Domain Sovereignty: Core, mesh, commands, project, modules are 100% headless.
