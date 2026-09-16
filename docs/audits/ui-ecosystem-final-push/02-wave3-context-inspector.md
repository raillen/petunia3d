# 02 — Wave 3 · Context/Inspector

<aside>
🧱

Primeira *product wave* da diretiva (§45) por ser uma das áreas de maior dor: o
inspector concentrava as fórmulas manuais de layout. Contrato, evidência e
verificação abaixo; o que mudou na API pública está em
[`docs/developers/ui-architecture.md`](../../developers/ui-architecture.md#contrato-de-colunas-wave-3).

</aside>

## Aceitação da §45

| Critério | Estado |
| --- | --- |
| sem `avail >= 300` / `190` em `Vector3Field` | ✅ removidos — o arranjo vem de `PetuniaColumnSpec::responsive` |
| sem cálculo manual de `tab_w` | ✅ removido — `PetuniaColumnSpec::fixed` |
| layout passa todos os tamanhos/escalas | ✅ testes paramétricos no adapter (0 → 800px) + 5 escalas na vitrine |
| screenshot comparado | ⏳ adiado por decisão explícita (passe de captura após as correções) |
| behavior/undo preservados | ✅ suítes de UI sem alteração de asserção |

## O que a Wave 3 mudou

### 1. O adapter ganhou o contrato de colunas

| Item | Papel |
| --- | --- |
| `column_width(available, columns, gap)` | única fórmula de divisão, pura e truncada para baixo |
| `fit_columns(available, max, min_item, gap)` | quantas colunas cabem **a partir do mínimo do item** |
| `columns(ui, id, spec, items, after)` | renderiza e entrega a largura resolvida a cada item |
| `PetuniaColumnSpec::{fixed, responsive}` | descrição declarativa do arranjo |
| `clamped_width(ui, min, desired)` | substitui `available_width().clamp(a, b)` |
| `fill_remaining(ui, reserved, min)` | substitui `(available - reservado).floor().max(min)` |

`responsive` em modo `Grid` passou a delegar para `columns` — antes o modo
existia mas não dividia a linha (ver `A10` em
[`01-code-vs-doc-findings.md`](01-code-vs-doc-findings.md)).

### 2. O inspector deixou de calcular layout

| Arquivo · local | Antes | Depois |
| --- | --- | --- |
| `inspector_widgets.rs` · `Vector3Field` | `avail >= 300` / `avail >= 190` + `(avail - 3*label - 2*gap)/3` | `PetuniaColumnSpec::responsive(3, AXIS_UNIT_MIN_W, AXIS_GAP)` |
| `inspector_widgets.rs` · `context_tabs` | `tab_w = ((avail - gap*(n-1))/n).floor()` | `PetuniaColumnSpec::fixed(n, TAB_GAP)` |
| `inspector_widgets.rs` · `section` / `block_header` | `FontId::proportional(11.5)`, `.size(12.5)`, `add_space(2.0)` | `TextRole::SectionTitle` / `TextRole::Caption` + `foundation::spacing` |
| `properties_panel.rs` · barra do objeto | `(available_width() - 108.0).floor().max(60.0)` | `fill_remaining(ui, object_bar_actions_width(), OBJECT_BAR_NAME_MIN_W)` |
| `properties_panel.rs` · ações de modificador | `narrow_actions = available_width() < 220.0` | `PetuniaResponsiveLayout::wrap_row()` |
| `properties_panel.rs` · combos de grupo (×4) | `available_width().clamp(96.0, 180.0)` | `clamped_width(ui, 96.0, 180.0)` |

`AXIS_UNIT_MIN_W` e `object_bar_actions_width()` são derivados do conteúdo
(rótulo + vão + campo utilizável; lado do botão × quantidade + vãos), não números
escolhidos a dedo.

### 3. Delta visual deliberado

Para o passe de screenshot — não são regressões acidentais:

- título de seção `11.5 → 11.0` px e título de bloco `12.5 → 11.0` px (um papel
  para cabeçalho em vez de dois literais);
- a barra do objeto perde 12px de folga que existiam só porque o `108.0` era uma
  estimativa; a faixa de ações agora é derivada e cabe exata;
- em painel estreito, os botões de modificador passam a quebrar linha por wrap do
  layout em vez de por um limiar fixo de 220px.

### 4. Medida antes/depois

Comparado à [baseline congelada da Wave 0](00-baseline.md) (a tabela da Wave 0
não é reescrita: ela é a foto *anterior*).

| Regra | Wave 0 (produto) | Agora (produto) | Δ |
| --- | ---: | ---: | ---: |
| `available_width` | 24 | 18 | **−6** |
| `spacing_mut` | 54 | 54 | 0 |
| `allocate_exact_size` | 33 | 33 | 0 |

A queda de `spacing_mut`/`allocate_exact_size` não é esperada nesta wave: o
arranjo responsivo era o problema do inspector, não o desenho de controle.
`properties_panel.rs` saiu de 7 para **1** ocorrência de `available_width` (a que
resta é `set_min_width` dentro de um `egui::Frame`, sem alternativa no adapter).

## Verificação

```bash
cargo test -p petunia_ui            # 189 lib + 35 kittest + 4 estabilidade + 6 fluxos
cargo clippy -p petunia_ui --all-targets   # sem warnings
cargo run -p xtask -- ui-guard --strict    # exit 0
cargo run -p petunia_ui --example component_gallery
```

Testes novos no adapter (falham com a implementação anterior):

- `grid_mode_now_really_splits_the_row`
- `columns_hands_each_item_its_resolved_width_on_one_line`
- `columns_stacks_full_width_when_only_one_fits`
- `fit_columns_derives_the_count_from_the_item_minimum`
- `column_width_never_overflows_and_floors`
- `clamped_width_respects_the_floor_and_the_ceiling`
- `fill_remaining_never_goes_below_the_minimum`

## Dívida declarada que sai desta wave

| Item | Onde | Wave |
| --- | --- | --- |
| breakpoints remanescentes fora do inspector | `reference_manager.rs`, `toolbar.rs`, `viewport_bar.rs`, `primitive_card.rs`, `status_bar.rs`, `animation_ui.rs` | §46–§50 |
| `PetuniaContextMenu` / `PetuniaPopup` / `PetuniaModal` | `nav_gizmo.rs` ainda monta o menu de contexto | §34, §44 |
| captura real de screenshots e comparação | pendente por decisão | §40, §42 |
