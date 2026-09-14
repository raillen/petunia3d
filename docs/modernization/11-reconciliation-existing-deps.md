> [← Voltar ao Índice de Modernização](/modernization/) | [Documento Integral Monolítico](/modernization/full-gauntlet)

# 13. Existing dependency reconciliation

The repository already uses important ecosystem crates. Do not duplicate them.

Audit and update compatibility for at least:

```text
rfd
image
egui-phosphor
egui-file-dialog
egui_ltreeview
egui_tiles
mint
transform-gizmo
transform-gizmo-egui
egui_kittest
winit
pollster
glutin
glutin-winit
raw-window-handle
postcard
bytemuck
gltf (currently observed as dev dependency)
current PNG toolbar/property icon assets and loaders (migration debt, not permanent baseline)
current TOML i18n loader (retain but enforce complete TextId coverage)
current theme/tokens implementation (retain concepts, rebuild for complete semantic coverage)
serde
serde_json
toml
uuid
thiserror
anyhow
log
env_logger
glam
```

## 13.1 Logging consolidation

After `tracing` is integrated:

- application code should prefer `tracing`;
- remove direct `env_logger` if no longer needed;
- keep `log` only where dependency interoperability requires it;
- if a log→tracing compatibility bridge is needed, add the smallest justified supporting dependency and document it;
- do not maintain two unrelated logging configurations.

## 13.2 glTF reconciliation

The audited repository has `gltf` as a development dependency in `petunia_project`.

After P0:

- decide the production owning crate/module;
- add `gltf`/`gltf-json` intentionally;
- remove test-only duplication;
- build real importer/exporter integration;
- test round-trip expectations where appropriate.

---
