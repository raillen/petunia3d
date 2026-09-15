# Mapa de Componentes da UI

> Fonte legível por humanos do inventário canônico da interface.
> A fonte legível por máquina (para code agents) é [`/ui-map.json`](/ui-map.json):
> 190 nós com `kind`, arquivo, símbolo de entrada, região do shell, filhos e
> conexões (`reads`/`writes`/`dispatches`/`opens`).
> Ambos são verificados contra o código por `cargo xtask ui-check`
> (parte do `docs-check`): ids únicos, arquivos e símbolos existentes, regiões
> dentro de `RegionSlot`, filhos acíclicos.

## Ordem de composição do shell (`petunia_ui::draw`)

```text
Header (main_header)
StatusBar (status_bar — único Panel::bottom do shell)
[Timeline — faixa central, só Animate]
AssetBrowser (se aberto) → Toolbar → RightDock → ViewportToolbar
CentralPanel → Viewport 3D | UV split | Animate + strip
Overlays: shelf, primitive card, gizmos, HUDs
Modais: settings, reference manager, asset library, palette, recovery, file dialogs
```

Painéis `top`/`bottom` não competem entre si; laterais ocupam o que sobra;
a toolbar da viewport é central (padrão Blender). Retângulos canônicos em
`UiRegions` (`crates/ui/src/regions.rs`), um escritor por slot.

## Regiões e quem mora em cada uma

| Região | Conteúdo |
|---|---|
| `Header` | brand, menus (File/Edit/Window/Help), pílulas de workspace, Config/Assets |
| `ViewportToolbar` | seleção, View/Select/Add/Object, transform/pivot, snap, overlays, shading, câmera |
| `StatusBar` | identidade do projeto, dicas, telemetria, undo/redo, mensagem central |
| `LeftTools` | paleta por workspace (Model/Paint/UV/Animate) |
| `AssetBrowser` | busca, categorias, cartões, instanciação (quando aberto) |
| `RightDock` | divisor Outliner/Inspector com colapso independente |
| `RightOutliner` | árvore (Anotações, Medidas, Cena, Referências) |
| `RightInspector` | abas Tool/Object/Modifiers/Data/Material + painéis de workspace |
| `UvEditor` | editor UV 2D (split central no workspace UV) |
| `Viewport` | cena 3D + todos os overlays e gizmos |
| `Shelf` | barra contextual flutuante por workspace/modo |
| `BottomDock` | faixa Timeline (só Animate) |

## Árvore de interdependência (resumo)

```text
shell (lib.rs draw)
├── main_header ──┬─ file_menu ──→ file_service_host, asset_library_modal
│                 ├─ edit_menu ──→ command_palette, settings_modal
│                 ├─ window_menu ──→ asset_browser, reference_manager_modal
│                 ├─ workspace_pills ──→ switch_workspace() → perfis (workspaces.rs)
│                 └─ header_right_actions ──→ settings_modal, asset_browser
├── status_bar (undo/redo)
├── asset_browser ──→ DuplicateAssetCmd, DeleteAssetCmd
├── toolbar (por workspace)
├── right_dock ──┬─ right_outliner ──→ outliner_* (dispatches de coleção/asset)
│                └─ right_inspector ──→ properties_panel_root
│                    ├── property_tab_bar + tool_property_fields
│                    ├── model_* (16 forms via model_tool_dispatcher)
│                    ├── paint_vertex_section + paint_canvas_section
│                    ├── uv_summary_section (+ uv_editor_panel no centro)
│                    ├── animation_panel_root (4 seções)
│                    └── material_tab_pbr, object_tab_transform, ...
├── viewport_bar ──┬─ primitive_menu ──→ begin_primitive() ──→ primitive_last_op_card
│                  ├─ view/select/object_mesh menus ──→ comandos de cena
│                  └─ transform/snap/shading/câmera
├── central_viewport ──┬─ viewport_arbitrator ──→ gizmos, HUDs, tools 3D
│                      ├─ shelf_overlay_bar (por workspace)
│                      ├─ primitive_last_op_card (transação única de undo)
│                      ├─ animate_center_strip ──→ timeline
│                      └─ uv_center_split ──→ uv_editor_panel
└── modais: settings_modal (6 abas) · reference_manager_modal (grade 1–3)
    asset_library_modal · command_palette · recovery_dialog · file_service_host (9 pickers)
```

Widgets reutilizáveis (`widgets.rs`): `PetuniaToolbarButton`,
`PetuniaPropertyTabButton`, `PetuniaWorkspacePill`, `petunia_search_box`,
`PetuniaIconButton`, `PetuniaMenuItem(/Checkbox/Radio)`, `petunia_action_button`.
Ícones via `IconRegistry` (pacotes iconflow para chrome, arte Petunia para
ferramentas); tokens em `tokens.rs`; tradução por `TextId`.

## Fluxos de interligação principais

- **Criação de primitiva**: `primitive_menu`/`shelf`/`add_primitive_entry` →
  `begin_primitive()` → `primitive_last_op_card` (regen sem checkpoint) →
  Confirm (1 txn) / Esc (remove) / outra operação (finaliza em malha comum).
- **Comando via paleta**: `command_palette` → `dispatch_command(id)` → undo transacional.
- **Arquivo**: menus/File → `file_service_host` (in-canvas) → `ProjectService`.
- **Workspace**: pílulas → `switch_workspace()` (memória de layout) → perfis recompõem shell.
- **Render**: `viewport_rect` posiciona a superfície GPU; fingerprint evita rebuild.

## Manutenção

- Novo componente visual → adicionar nó em `docs/public/ui-map.json`.
- `cargo xtask ui-check` valida; `cargo xtask docs-check` exige o `.md` desta página + mapa válido.
