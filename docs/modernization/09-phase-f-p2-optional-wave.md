> [← Voltar ao Índice de Modernização](/modernization/) | [Documento Integral Monolítico](/modernization/full-gauntlet)

# 11. Phase F — P2 optional integration wave

P2 dependencies are not permitted to bloat the default product without a reason.

For each P2 library:

1. implement the adapter;
2. create a focused real use or reference integration;
3. test it;
4. feature-gate it if not currently needed;
5. document activation;
6. prove no forbidden dependency leaks.

## F1. UI optionals

- `egui_dnd` → Petunia DnD adapter;
- `egui_animation` → Petunia Motion service/tokens;
- `egui_table` → Petunia large-table adapter;
- `egui_form` → Petunia form adapter;
- `egui_tool_windows` → Petunia controlled floating palette adapter;
- `egui-notify` → Petunia notification/toast adapter.

Do not replace already-correct specialized components purely to increase dependency use.

## F2. Platform/security support

- `directories` → central platform directories service;
- `semver` → plugin/package/API compatibility;
- `camino` → only explicit UTF-8 path boundaries;
- `cap-std` → plugin/MCP capability filesystem.

Security tests must include:

- denied path;
- outside-root path;
- traversal;
- missing permission;
- stale capability where applicable;
- malformed package/plugin metadata.

Record:
`docs/audits/stack-modernization/50-p2-integration-gauntlet.md`.

---
