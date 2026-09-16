# 03 — Wave 4 · Viewport Toolbar

<aside>
🧱

Segunda *product wave* (§46). A barra da viewport era o exemplo mais forte da
auditoria: o arquivo **sabia** que estava reimplementando CSS e traduziu o
problema para aritmética manual. Contrato, achados e verificação abaixo; a API
pública está em
[`docs/developers/ui-architecture.md`](../../developers/ui-architecture.md#barra-responsiva-wave-4).

</aside>

## Aceitação da §46

| Critério | Estado |
| --- | --- |
| remover/encapsular `measured_widths` | ✅ função deletada; a medição virou sonda no adapter |
| sem somas manuais de pixels no product module | ✅ nenhuma constante de largura em `viewport_bar.rs` |
| overflow semantics preservadas | ✅ só faixas de rank declarado caem; núcleo e faixa de acesso sempre alcançáveis |
| Domain/Menus/Display permanecem acessíveis | ✅ pinados; testado de 180px a 1600px e nos dois idiomas |
| long locale tested | ✅ `en` × `pt-BR` medidos na mesma largura |
| 1/2 row behavior estável | ✅ decide pela **altura medida** da linha, não por breakpoint |

## O que a Wave 4 mudou

### 1. `adapters::toolbar` — o modelo

| Item | Papel |
| --- | --- |
| `PetuniaToolbarSpec` | declaração: faixas da linha principal, da segunda, teto de linhas e faixa de acesso |
| `PetuniaToolbarCluster::{pinned, overflowable}` | semântica de permanência (`Pinned` nunca cai) |
| `plan_row(clusters, widths, available, divider, overflow)` | **aritmética pura** de overflow — sem egui, testável com larguras arbitrárias |
| `rows_that_fit(available_h, line_height, row_gap, max_rows)` | substitui o breakpoint `available_height() > 40.0` |
| `PetuniaResponsiveToolbar::{plan, show}` | mede (sonda), decide e desenha a linha via taffy |
| `PetuniaToolbarPlan` | resultado: linhas, faixas visíveis/ocultas, larguras medidas |

A ordem ficou explícita e unidirecional:

```text
PetuniaToolbarSpec (declaração do produto)
      ↓
sonda offscreen (largura/altura REAIS por faixa)
      ↓
plan_row (aritmética pura)
      ↓
taffy flex row (gap, alinhamento, distribuição)
```

Nenhum caminho alternativo: o produto não soma larguras nem decide quem cai.

### 2. Como a medição funciona (a parte não óbvia)

A sonda desenha a faixa **uma vez** num `Ui` em coordenadas muito negativas, com
`id_salt` próprio:

- nada é interativo (o ponteiro nunca está ali) e nenhum popup abre, porque o
  `Id` do menu difere do `Id` real;
- o retângulo tem extensão **cruzada zero**: num layout de linha o `min_rect` do
  egui cresce até a extensão cruzada do `max_rect`, então um retângulo alto
  devolveria a altura da sonda. Com a cruz em zero o cursor cresce exatamente até
  o conteúdo — e a altura medida (24px) é a altura real da faixa;
- a largura do divisor e a da seta também são medidas, não estimadas.

### 3. O bug de layout que a wave revelou

`egui_taffy` entrega a cada item um `Ui` que **herda o layout do pai**. Numa linha
dentro de um painel vertical, as faixas que desenham vários filhos direto no `Ui`
recebido (`draw_viewport_actions_cluster` desenha os quatro botões de menu na
mesma passagem) empilhavam: a linha media **101px** de altura em vez de 24.

Medido antes da correção (uma faixa por vez, mesma montagem do produto):

| Faixa | Altura como item de taffy |
| --- | ---: |
| `viewport.domain` | 22 |
| `viewport.menus` | **97** |
| `viewport.transform` | **41** |
| `viewport.snap-prop` | 22 |
| `viewport.display` | **101** |

Correção no adapter de layout: `PetuniaItemLayout::{Inherit, Row}` +
`PetuniaResponsiveLayout::with_item_layout`. É correção de **contrato**, não
paliativo: a barra declara que cada item é uma linha. Registrado em
[`docs/dependencies/ui-ecosystem-lock.md`](../../dependencies/ui-ecosystem-lock.md).

### 4. Delta visual deliberado

Para o passe de screenshot — não são regressões acidentais:

- a seta de overflow fica **no fim** da linha em ambos os modos (antes ficava
  entre Snap/Prop e Display quando havia uma linha só): posição única e
  previsível para o mesmo controle;
- o espaço entre faixas vem do gap do layout (6px) + divisor medido, em vez de
  `item_spacing` mais `add_space(2.0)` em volta de cada separador;
- uma barra de 41–46px de altura passa a ficar em **uma** linha em vez de
  espremer duas (a decisão usa a altura medida, não um limiar de 40px).

Comportamento e undo preservados: nenhuma asserção de teste existente mudou.

### 5. Medida antes/depois

| Regra | Wave 0 (produto) | Wave 3 (produto) | Agora (produto) |
| --- | ---: | ---: | ---: |
| `available_width` | 24 | 18 | 18 |
| `spacing_mut` | 54 | 54 | 51 |
| `allocate_exact_size` | 33 | 33 | 33 |

`viewport_bar.rs` sai de **27** para **24** ocorrências totais, e as que ficam são
desenho de controle de baixo nível (ícone, toggles, badge de eixo) — a §36 permite
isso como implementação de componente. `available_width` no arquivo: **1 → 0**.

## Testes novos que falham com a implementação anterior

No adapter (`adapters::toolbar::tests`):

- `wide_line_keeps_everything_and_spends_no_overflow_reserve`
- `pinned_clusters_never_leave_the_line`
- `lower_rank_falls_first_and_the_rightmost_breaks_the_tie`
- `every_occluded_cluster_is_still_reachable_through_the_plan`
- `long_locale_labels_push_clusters_to_overflow_instead_of_cutting_the_bar`
- `rows_that_fit_uses_the_measured_line_height`
- `probe_measures_real_widths_off_screen`

No produto (`viewport_bar::tests`):

- `toolbar_keeps_the_core_visible_no_matter_how_narrow` (180px → 1600px)
- `only_pinned_clusters_may_exceed_the_available_width` (monotonicidade + nenhuma
  faixa ocultável desenhada quando a linha estoura)
- `long_locale_pushes_more_clusters_to_overflow_than_en`
- `second_row_appears_only_when_the_measured_line_height_fits`
- `snap_and_prop_stay_reachable_through_the_overflow_menu`

No shell (`crates/ui/tests/panel_stability.rs`):

- `viewport_toolbar_height_is_stable_and_inside_the_panel_band` — a altura da
  barra não muda em 50 frames e fica na faixa do painel (26–56px). Protege contra
  a barra "pulando" de uma para duas linhas entre frames.

## Dívida declarada que sai desta wave

| Item | Onde | Wave |
| --- | --- | --- |
| `measured_widths` do mesmo tipo em outras barras | `toolbar.rs` (lateral) e `status_bar.rs` | §48 |
| largura mínima suportada pela barra | o núcleo (Domain/Menus/Display) é largo por natureza: abaixo de ~660px a linha passa da largura em vez de ocultar o núcleo. Decisão de produto pendente (encolher o núcleo ou reduzir o mínimo de janela) | §47/§59 |
| captura real de screenshots e comparação | pendente por decisão | §40, §42 |
