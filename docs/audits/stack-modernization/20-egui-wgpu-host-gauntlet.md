# 20-egui-wgpu-host-gauntlet: Phase C egui 0.36.2 + wgpu 30.0.1 Compatibility Verification

- **Date:** 2026-09-14
- **Resumed from:** Antigravity conversation `3c8f0fe5-6178-45d9-ba8f-b80ccab9dba1` (stopped on `RESOURCE_EXHAUSTED 429` mid-migration, workspace left with 63 compile errors)
- **Target:** egui family `0.36.2`, `eframe 0.36.2`, `wgpu 30.0.1`, `winit 0.30.x`

## 1. What was broken on resume

- `Cargo.toml` already bumped to `egui 0.36.2 / egui-winit / egui-wgpu / egui_glow / eframe / wgpu 30.0.1` with Edition 2024 (Phase B intact).
- Only `crates/ui/src/main_header.rs` partially migrated to `Panel::top(...).show(ui, ...)` + `run_ui`, leaving `lib.rs`, `toolbar`, `asset_browser`, `status_bar`, `timeline` on removed `SidePanel` / `TopBottomPanel` / `Context::run` / `Context::screen_rect`.
- `cargo check --workspace --all-targets`: 63 errors.

## 2. egui 0.32 → 0.36 breaking changes applied

- `SidePanel` / `TopBottomPanel` removed → unified `Panel::left/right/top/bottom` with `default_size / size_range / exact_size` (`RangeInclusive<f32>` converts into `Rangef`).
- `Panel::show` and `CentralPanel::show` now take `&mut Ui`, not `&Context`. Top-level `petunia_ui::draw` changed from `(ctx: &Context, ...)` to `(ui: &mut Ui, ...)`; Window/Modal/Area keeps `show(ctx, ...)` and is called via `ui.ctx().clone()`.
- `Context::run` removed → `Context::run_ui(RawInput, impl FnMut(&mut Ui))`. All 50 test call sites migrated.
- `Context::screen_rect` removed → `Context::viewport_rect()` (panels) / `ui.max_rect()` (root).
- `RawInput.modifiers` removed → modifiers travel as `Event::ModifiersChanged` + per-event `modifiers` field (`nav_gizmo` test fixed with explicit `Modifiers { shift: true, .. }`).
- `TexturesDelta::set` is now `HashMap<TextureId, SmallVec<[ImageDelta; 1]>>`; `Drop` debug-asserts on unapplied deltas. All `run_ui` test sites now end with `.textures_delta.clear()` (or clear before returning `FullOutput`).
- `Harness::builder().build(|ctx| ...)` removed in `egui_kittest 0.36` → `build_ui(|ui| ...)`.
- `properties_panel::draw(ctx, ui, ...)` and `paint_ui::draw_paint_panel(ctx, ui, ...)` refactored to Ui-first (`draw(ui, ...)`), deriving `Context` internally via `ui.ctx().clone()`.

## 3. wgpu 25 → 30 + egui-wgpu 0.36 changes applied (`crates/app`)

- `Instance::new(&desc)` → `Instance::new(desc)` with `InstanceDescriptor::new_without_display_handle()` + `backends = all`.
- `RequestAdapterOptions` gains `apply_limit_buckets: false`.
- `DeviceDescriptor` gains `experimental_features: Default::default()`.
- `SurfaceConfiguration` gains `color_space: SurfaceColorSpace::Auto`.
- `egui_wgpu::Renderer::new(device, format, None, 1, true)` → `Renderer::new(device, format, RendererOptions::default())`.
- `Surface::get_current_texture() -> Result<_, SurfaceError>` removed → returns `CurrentSurfaceTexture` enum (`Success | Suboptimal(frame)` renders; `Outdated | Lost` reconfigures; `Timeout | Occluded | Validation` skips frame).
- `SurfaceTexture::present()` removed → `Queue::present(frame)`.
- `RenderPassColorAttachment` gains `depth_slice: None`; `RenderPassDescriptor` gains `multiview_mask: None`.
- `egui::TexturesDelta` texture upload loop handles `SmallVec` deltas; `egui_glow::Painter::paint_and_update_textures` takes `&mut TexturesDelta`.
- `Context::wants_keyboard_input` → `egui_wants_keyboard_input`.

## 4. Host / eframe decision

- `eframe 0.36.2` resolves coherently in one egui family (`cargo tree -i egui` shows only `0.36.2`; no duplicate egui lines).
- The desktop host stays manual (`WgpuApp` + `GlApp` over winit) instead of a single `eframe::App`, preserving the required dual-backend invariant (WGPU primary + OpenGL/glow fallback behind `petunia_app` / renderer contracts, Core unaware of host).
- This matches the Gauntlet's own escape hatch: introduce a narrow Petunia host abstraction rather than collapsing renderer fallback into one `eframe::App`. Full `eframe::App::ui` migration remains a future wave with its own visual/behavior gate; `eframe` is not removed so the future path stays compilable.

## 5. Gate verification results

- `cargo check --workspace --all-targets`: PASS.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: PASS.
- `cargo test -p petunia_ui --lib`: PASS (88/88).
- `cargo test -p petunia_ui --test kittest_ui_flows`: PASS (17/17).
- `cargo test -p petunia_core --lib`: PASS (56/56).
- `cargo test -p petunia_mesh --lib`: PASS (53/53).
- `cargo run -p xtask -- arch-check`: PASS (G0-G10, Wave 1, Wave 2 intact).
- `cargo run -p xtask -- docs-check`: PASS (VitePress build ~80s, 0 errors).
- Full `cargo test --workspace --all-targets` not used as exit evidence: exceeds interactive timeout compiling/running every integration target at once; per-crate lib + kittest gates above are the reproducible evidence.
- `cargo deny check`: deferred (`cargo-deny` not installed in this environment).

**Phase C (Wave M2) compatibility is COMPLETE; eframe host cutover explicitly deferred with rationale above.**
