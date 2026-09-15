# Mapa de Componentes da UI

> Fonte legível por humanos do inventário canônico da interface.
> A fonte legível por máquina (para code agents) é [`/ui-map.json`](/ui-map.json):
> 193 nós com `kind`, arquivo, símbolo de entrada, região do shell, filhos e
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
| `Header` | menus (File/Edit/Window/Help) à esquerda, pílulas de workspace centralizadas (3 colunas), Config/Assets à direita — sem brand |
| `ViewportToolbar` | seleção, View/Select/Add/Object (seta vetorial, sem glifo `▾`), transform/pivot, snap, overlays, shading, câmera; overflow sob a seta + modo 2 linhas |
| `StatusBar` | 3 zonas rígidas (identidade+dica truncada, mensagem central truncada, telemetria+undo/redo) — nada escapa da coluna |
| `LeftTools` | paleta por workspace (Model configurável: ordem/visibilidade/1–2 colunas, incl. mirror/merge/symmetrize); Paint/UV/Animate fixas |
| `AssetBrowser` | busca, categorias, cartões (rolagem com altura limitada + reserva do rodapé), instanciação (quando aberto) |
| `RightDock` | lado esquerdo/direito + empilhado/lado a lado (cabeçalho do dock), divisor arrastável, colapso independente |
| `RightOutliner` | árvore (Anotações, Medidas, Cena, Referências) |
| `RightInspector` | barra do objeto + abas textuais contextuais + seções (só Model); painéis de workspace ocupam tudo fora do Model |
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
├── status_bar (3 zonas rígidas + undo/redo)
├── asset_browser ──→ DuplicateAssetCmd, DeleteAssetCmd
├── toolbar (por workspace; Model configurável ──→ toolbar_config)
├── right_dock (lado E/D, empilhado/lado-a-lado) ──┬─ right_outliner ──→ outliner_* (dispatches de coleção/asset)
│                └─ right_inspector ──→ properties_panel_root
│                    ├── inspector_object_bar (colapso + rename + busca + pin + olho + cadeado)
│                    ├── inspector_tabs (Object/Modify/Material por contexto)
│                    ├── transform_section + geometry/modifiers/display
│                    ├── material_tab_pbr, selection_tab, modify_tool_panel
│                    └── scene vazio intencional + painéis Paint/UV/Animate
├── viewport_bar ──┬─ primitive_menu ──→ begin_primitive() ──→ primitive_last_op_card
│                  ├─ view/select/object_mesh menus (seta vetorial) ──→ comandos de cena
│                  ├─ viewport_overflow (clusters ocultos, sempre operáveis)
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
`PetuniaIconButton`, `PetuniaMenuItem(/Checkbox/Radio)`, `petunia_action_button`,
`PetuniaMenuButton` (rótulo + seta vetorial via `Popup::menu`, nunca glifo `▾`/`›`),
`paint_chevron`, `chevron_toggle(dir)`, `paint_check`.
Ícones via `IconRegistry` (pacotes iconflow para chrome, arte Petunia para
ferramentas); tokens em `tokens.rs`; tradução por `TextId`.

## Revisão do shell (header/dock/responsivo)

- **Header sem brand**: 3 colunas (menus | pills centralizadas | Config/Assets).
- **Setas vetoriais**: nenhum `menu_button("... ▾")`, `›`, `↑/↓`, `✕`, `✓`, `●`
  ou `│` em rótulo — tudo passa por `PetuniaMenuButton`/`paint_chevron`/
  `chevron_toggle(dir)`/`paint_check` ou ASCII/Latin-1 seguro (`×`, `•`, `|`).
  (Blender: fileiras com toggles de restrição sempre visíveis; Plasticity:
  minimalismo + dot de material por objeto + seções por tipo; C4D: dois
  "semáforos" visibilidade editor/render + Attribute Manager em abas.)
- **Viewport medida, não estimada**: `measured_widths()` soma botões fixos +
  galleys reais; fluxo único centralizado (sem vão à direita); overflow preciso.
- **Outliner**: cabeçalho único com colapso (`Outliner (N)` + busca; dobra
  redundante com o dock removida); linhas só-nome (contagens no tooltip);
  delete sob demanda; dot de material → aba Material; rename de coleções +
  rename de asset no cabeçalho do inspector; vazio com quick-add real.
- **Inspector**: barra do objeto fixa (colapso + rename + olho + cadeado,
  em toda aba); cabeçalhos de seção do dock removidos; vazio com quick-add.
- **Viewport responsiva**: conjunto visível progressivo por largura (núcleo
  Domínio/Menus/Display nunca esconde); excedente no overflow sob a seta com
  os mesmos controles operáveis; altura > 40px divide em 2 linhas.
- **Status sem invasão**: 3 colunas rígidas com truncamento; `asset_browser`
  limitava a rolagem sem reserva do rodapé e invadia a barra — corrigido com
  `FOOTER_RESERVE` + teste `shell_layout_tests` (3 resoluções × 4 configs de dock).
- **Toolbar configurável**: `canonical_toolbar_entries()` + `UiState`
  (`toolbar_order/_hidden/_columns`); engrenagem no fim da barra (visibilidade,
  ↑/↓, 1–2 colunas com fallback responsivo, Redefinir); inclui mirror/merge/
  symmetrize; resets em `settings_modal`.
- **Dock completo**: lado (E/D) + orientação (empilhado/lado a lado) no
  cabeçalho do dock; mesma fração de split, mesmos slots de região; Scene
  com altura AUTO/MANUAL; inspector contextual (sem rail).
- **Painéis por workspace**: rail e abas só no Model; Paint/UV/Animate rendem
  seu painel direto no inspector; memória de layout por workspace preservada.

## Redesign da sidebar direita (Scene + Inspector contextual)

- **Sem rail de ícones**: abas textuais contextuais (`Object | Modify |
  Material`, `Selection | Modify | Material`) via `context_tabs` — texto >
  memorização. Contextos resolvidos do estado real (`inspector_context`):
  cena vazia, objeto mesh (fixado ou ativo), seleção de componentes, anotação,
  medida. Sem multisseleção de objetos no modelo: sem contexto fake.
- **Scene adaptativa**: altura AUTO pelo conteúdo (até 38%) ou MANUAL pelo
  divisor; arrastar sai do AUTO, duplo-clique volta; `scene_panel_height` pura
  e testada. Cabeçalho compacto (`SCENE (N)` + busca expansível + funil de
  filtros por seção/estado + reset auto). Ctrl+F abre e foca a busca.
- **Árvore**: backend `egui_ltreeview` mantido (navegação por setas nativa ao
  focar; foco segue a seleção); fileiras só-nome com tooltip de contagens;
  Delete sob demanda; dot de material → aba Material; duplo-clique/Enter/F2
  renomeiam (buffer vazio + dica, undo com checkpoint); menu com Rename,
  Duplicate, Lock, Isolate, Move, Export real e Delete.
- **Header do inspector**: tipo + nome editável + busca + pin funcional (ref
  segura `Uuid`, fallback gracioso, limpeza ao deletar) + olho + cadeado;
  resumo `Mesh · 8 verts · 12 tris` no tooltip do ícone.
- **Transform redesenhado**: Position absoluta, Rotation relativa (rascunho;
  campos placebo removidos), Scale por eixo com link, Dimensions absolutas —
  campos vetoriais responsivos (linha cheia/coluna por eixo), `DragValue` com
  undo de 1 nível por gesto (`EditSession`), sem ruído decimal.
- **Seções leves**: tipografia + disclosure, sem cards aninhados; Geometry com
  resumo colapsado; Modifiers honesto (vazio + arme de ferramentas reais);
  Display só com o que existe; bloco da operação modal temporário com
  Aplicar/Cancelar reais; busca achata seções; vazios intencionais.
- **Densidade**: `UiDensity` (Compacta 28 / Confortável 32 / Espaçosa 36) em
  `UiState` + seletor na Interface; linhas, tabs, seções e campos consomem.
- **Sem dependências novas** (ltreeview já vendored; resto é egui built-in).

## Avaliação de crates externos (decisão)

Avaliados sem adicionar dependências: `egui_tiles` (removido na convergência —
sistema de dock próprio de ~120 linhas atende com slots/gates existentes),
`egui_dnd` (reordenação por ↑/↓ dispensa drag; sem custo de input),
`egui_css`/`egui_taffy`/`egui_flex` (padrões desta revisão usam egui built-in:
colunas, `horizontal_centered`, `Popup::menu`, `ScrollArea` limitado —
suficiente para centro/espaçamento/grades simples; `egui_taffy` segue
vendored para quando um grid complexo exigir), `egui-toast`/`egui-notify`
(status bar + `set_status` cobrem feedback), `egui-modal` (modais próprios com
`modal_sizes`), `egui-multiselect`/`egui_hotkey`/`egui-inspect` (cobertos por
widgets/keybinds/devtools). Motivos: invariantes de manifesto do `arch-check`,
risco de OOM de compilação e convergência (fronteira P0: cada dependência no
dono canônico). Skill `egui-layout-patterns` (skills.rest, por hafley66):
página pública consultada — conteúdo integral (GitHub) inacessível (404);
diretrizes aplicadas a partir do resumo publicado (built-ins primeiro,
terceiros só p/ flex avançado) + `UiBuilder`/`Layout` nativos.

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
- Versão anterior congelada em `ui-component-map.legacy.md` (backup legado).
