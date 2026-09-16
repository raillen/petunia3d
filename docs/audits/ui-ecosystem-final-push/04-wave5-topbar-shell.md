# 04 — Wave 5 · Top Bar e shell

<aside>
🧱

Terceira *product wave* (§47), executada em **duas partes** de propósito. A parte
1 (Top Bar em grid/flex) está fechada e verificada; a parte 2 (shell atrás de
`PetuniaLayoutAdapter` + `egui_tiles`) tem a dependência, o adapter e as regras
prontos e testados, com a troca do shell declarada como o passo seguinte. Nada
aqui é parcial *por acidente*: o que não foi trocado está listado em "Dívida
declarada", com o caminho exato.

</aside>

## Aceitação da §47

| Critério | Estado | Onde |
| --- | --- | --- |
| "Implementar Top Bar em grid/flex" | ✅ | `adapters::top_bar` + `main_header.rs` |
| CENTER geometricamente centralizado quando possível | ✅ por distribuição (caixas laterais iguais), com fallback declarado | `plan_top_bar` + `three_zone` |
| LEFT/RIGHT não empurram as pílulas sem fallback | ✅ `Drifting` é escolhido por **medição** e testado | `narrow_bar_falls_back_to_drifting_without_losing_access` |
| largura insuficiente → ações de menor prioridade em overflow | ✅ rank declarado; seta sempre alcançável | `low_rank_actions_go_to_overflow_instead_of_cutting_the_bar` |
| "não usar `add_space` repetido para fingir centralização" | ✅ `ui.columns(3, …)` e os três `spacing_mut` do header removidos | `spacing_mut` de produto 51 → 48 |
| "Reintroduzir `PetuniaLayoutAdapter` com egui_tiles" | ✅ adapter, topologia, restrições e testes | `adapters::tile_layout` |
| Migrar Left · Center · Right · Bottom | ⏳ passo seguinte (Wave 5b) | dívida abaixo |
| viewport-first · sem docking livre · min viewport · persistência por workspace | ✅ regras implementadas e testadas no adapter; persistência por workspace já existe (`WorkspaceUiMemory`) | `PetuniaShellLayout::clamped`, `ShellBehavior` |

## Parte 1 — Top Bar em grid/flex (§25)

### O que existia

```rust
ui.columns(3, |cols| {
    cols[0].horizontal_centered(|ui| { ui.spacing_mut().item_spacing = vec2(6.0, 0.0); /* menus */ });
    cols[1].horizontal_centered(|ui| { ui.spacing_mut().item_spacing = vec2(6.0, 0.0); /* pills */ });
    cols[2].horizontal_centered(|ui| { … ui.with_layout(right_to_left, …) });
});
```

Colunas iguais **não** centralizam: as pílulas ficam no meio do terço do meio,
que só coincide com o meio da barra quando as laterais têm a mesma largura. Com
idioma longo de um lado (ou com o botão `Config` + `Assets` do outro), as
pílulas deslizam — e nenhum teste pegava isso porque nenhum teste media o centro.

### O que existe agora

```text
PetuniaTopBarSpec        left / center / right (faixas + ranks) + overflow
        ↓
sonda offscreen           largura REAL de cada faixa (por zona)
        ↓
plan_top_bar              aritmética pura do centro, da queda e do fallback
        ↓
three_zone (taffy)        laterais com base 0 + flex_grow igual ⇒ centro geométrico
```

| Peça | Papel |
| --- | --- |
| `PetuniaTopBarSpec` | zonas declaradas pelo produto (`left`, `center`, `right`, `overflow`) |
| `plan_top_bar(...)` | pura: modo de centralização + o que cai, a partir de larguras medidas |
| `centered_side_width(available, center, gap)` | a metade que sobra para uma lateral |
| `three_zone(ui, id, gap, centering, zone, after)` | caixas laterais iguais (taffy) com um único callback por zona |
| `plan_row_width(...)` | largura do desenho do plano (visíveis + seta + vãos) |

O centro deixa de ser uma coincidência de terços: com `flex_basis: 0` + o **mesmo**
`flex_grow` nas duas laterais, o espaço livre é dividido em partes iguais e o
centro cai no meio geométrico por construção. Quando a esquerda não cabe na
metade, o plano assume o fallback `Drifting` (laterais empacotadas nas bordas) —
nada é cortado e o centro simplesmente deixa de ser geométrico. A escolha é por
medição, não por palpite, e está em teste.

### Bônus: um bug de prioridade corrigido no `plan_row`

`plan_row` (§46) decidia a queda **item a item, na ordem de rank**: uma faixa de
rank alto que não cabia era escondida, e depois uma de rank baixo que caberia
ficava. Com larguras diferentes isso contradizia a própria regra documentada
("rank menor cai primeiro"). O algoritmo passou a ser iterativo — derruba a menos
protegida **até caber** — e o invariante virou teste:

```text
rank de todo visível >= rank de todo oculto   (em 6 larguras diferentes)
```

Consequência prática: em alguns casos a barra agora esconde **menos** que antes.
Também foi corrigida a seta fantasma: quando nada saiu da linha, a faixa de
acesso não é desenhada (antes era incluída no plano mesmo com `hidden` vazio).

## Parte 2 — `PetuniaLayoutAdapter` + `egui_tiles` (§31, §47)

### O contrato

```text
PetuniaShellLayout        DTO Petunia (workspace, larguras, split, visibilidade)
        ↓
PetuniaLayoutAdapter      topologia + mínimos + o que pode fechar/arrastar
        ↓
egui_tiles::Tree          mecânica de layout
```

| Peça | Papel |
| --- | --- |
| `PetuniaPane` | os 5 paines reais do shell: `Tools`, `Viewport`, `Parts`, `Context`, `Bottom` |
| `PetuniaPaneSlot` | slot autorizado de cada paine (`Left`, `Center`, `Right`, `Bottom`) |
| `PetuniaShellLayout` | DTO persistível: workspace, `left_width`, `right_width`, `bottom_height`, `right_dock_split`, orientação, visibilidade |
| `PetuniaShellLayout::clamped(available)` | **min viewport**: as laterais cedem antes de a viewport esmagar |
| `PetuniaLayoutAdapter::tree(layout, available)` | topologia fixa (`vertical[horizontal[Tools, Viewport, dock], Bottom]`) |
| `PetuniaLayoutAdapter::show(ui, layout, ctx)` | desenha e devolve larguras **em pixels** para o produto persistir |
| `ShellBehavior` | regras do shell: nada fecha, nada arrasta, só as colunas redimensionam |

Restrições de §31.2 implementadas **e testadas**:

- paines núcleo nunca fecham (`is_tab_closable == false`);
- **nenhum** paine é arrastável (`is_tile_draggable == false`) — não existe
  docking livre, a árvore não pode ser rearranjada pela UI;
- um paine só existe no slot autorizado (`PetuniaPane::slot` + `Slot::allows`);
- `retain_pane` sempre `true`: nenhum paine some por GC silencioso;
- só containers lineares redimensionam (abas não).

Nada de `Tree` serializado: o que o usuário mexe (larguras, split) volta em
pixels para o estado Petunia, que já tem dono único (`UiState` /
`WorkspaceUiMemory` por workspace).

### Por que a migração do shell ficou para a parte 2

Duas medições mudaram o plano:

1. **Um `Panel` do egui 0.36 funciona dentro do `Ui` de um tile** (medido: tile de
   199.8px, painel interno carvando 8..51px **dentro do tile**). Isso permite
   trocar região por região, sem reescrever de uma vez o conteúdo de todos os
   painéis — que é o que a §60 exige.
2. **As shares são normalizadas pela soma.** Pedir dock de 300px sem escrever o
   share do centro devolvia 231.8px. Com as três colunas declaradas, o DTO é
   honrado (teste `real_frame_reports_pixel_widths_close_to_the_dto`, tolerância
   de 24px/40px).

Ambas estão registradas em
[`docs/dependencies/ui-ecosystem-lock.md`](../../dependencies/ui-ecosystem-lock.md)
para ninguém cair de novo.

## Medidas

| Regra (produto) | Wave 0 | Wave 3 | Wave 4 | Agora |
| --- | ---: | ---: | ---: | ---: |
| `available_width` | 24 | 18 | 18 | 18 |
| `spacing_mut` | 54 | 54 | 51 | **48** |
| `allocate_exact_size` | 33 | 33 | 33 | 33 |
| `painter().rect_filled` | — | — | 15 | 15 |

`ui-guard --strict`: **exit 0** (333 candidatos em produto, 231 em
foundation/adapter, 190 arquivos varridos, 17 regras).

## Testes novos

Adapter da Top Bar (`adapters::top_bar::tests`):

- `wide_bar_centers_exactly_and_hides_nothing`
- `center_is_geometric_even_when_sides_differ_wildly`
- `low_rank_actions_go_to_overflow_instead_of_cutting_the_bar`
- `long_locale_keeps_the_core_and_sends_the_rest_to_overflow`
- `narrow_bar_falls_back_to_drifting_without_losing_access`
- `every_action_is_either_visible_or_reachable`
- `empty_right_zone_still_plans_the_center`

Adapter de linha (`adapters::toolbar::tests`, regressão da §46):

- `a_more_protected_cluster_never_falls_before_a_less_protected_one`
- `planned_width_counts_visible_plus_arrow_plus_dividers`

Adapter de macro-layout (`adapters::tile_layout::tests`):

- `tree_contains_exactly_the_declared_panes`
- `core_panes_cannot_be_closed_and_nothing_can_be_dragged`
- `every_pane_lives_only_in_its_authorized_slot`
- `hidden_panes_stay_in_the_tree_but_out_of_the_layout`
- `min_viewport_wins_over_the_side_columns`
- `min_viewport_never_grows_beyond_a_tiny_window`
- `bottom_dock_only_exists_when_the_workspace_enables_it`
- `real_frame_reports_pixel_widths_close_to_the_dto`
- `resize_is_authorized_only_for_the_shell_columns`

Nenhuma asserção de teste existente mudou.

## Delta visual deliberado

Para o passe de screenshot — não são regressões acidentais:

- as pílulas de workspace passam a ficar no **meio geométrico** da barra
  (antes: meio do terço do meio, deslocado conforme o conteúdo das laterais);
- `Assets` e `Config` mantêm a ordem relativa, mas ganham a seta no fim da linha
  quando não cabem (antes simplesmente espremiam);
- os menus deixam de ter `item_spacing` próprio por coluna (o vão vem do gap do
  layout, `CONTROL`), então o espaçamento entre menus é o mesmo do resto da UI.

## Dívida declarada que sai desta wave

| Item | Onde | Wave |
| --- | --- | --- |
| troca do shell para a árvore (Left/Center/Right/Bottom) | `lib.rs` (`draw`, `right_panel`, `draw_split_dock`, `draw_side_by_side_dock`) | 5b |
| `dock_split` manual e colapsos por coluna (`split_widths`, `scene_panel_height`) | `regions.rs` → passam a ser shares/visibilidade do adapter | 5b |
| breakpoints manuais na toolbar lateral | `toolbar.rs` | §48 |
| captura real de screenshots e comparação | pendente por decisão | §40, §42 |
