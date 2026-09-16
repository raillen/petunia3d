# 05 — Wave 6 · toolbar lateral, Settings e Reference Manager (§48)

<aside>
🧭

Escopo da wave: **remover breakpoints manuais** da paleta lateral, do gerenciador
de referências e das linhas de campo de Settings, usando o motor de layout já
instalado (`egui_taffy`). Três contratos novos nasceram — e no caminho apareceu um
defeito silencioso de geometria que nenhum teste pegava.

</aside>

## Resumo

| Entrega | Antes | Depois |
| --- | --- | --- |
| Paleta lateral | `available_width() < 90.0`, `>= 100.0` | [`PetuniaToolGridSpec`] — colunas derivadas do mínimo da célula |
| Botão de ferramenta | media o container (`available_width()`) | recebe a largura resolvida (`.width(..)`) |
| Grupo (split button) | botão de célula inteira + seta desenhada fora do paine | [`plan_group`] — arranjo derivado; nenhum controle fora da célula |
| Coluna de ferramentas | 48px herdados do `Panel::left` | 74px derivados (ícone + vão + seta + moldura) |
| Settings | `ui.horizontal` + `size(13.0)` repetido por aba | `PetuniaForm` (`field`, `toggle`, `section`) |
| Reference Manager | `available_width() < 560.0`, `grid_columns`, `grid_card_width` | esteira flex com `SpaceBetween` + grade do adapter |
| `available_width` em product code | 18 | **14** |
| `spacing_mut` em product code | 48 | **37** |

```
✅ cargo fmt --check · clippy 0 warnings (workspace, --all-targets)
✅ 725 testes passando, 0 falhando
✅ ui-guard --strict exit 0 · bible-check · arch-check · docs-generate --check
⚠️ docs-check: falha apenas no passo `ui-map` (mapa congelado — ver Pendências)
```

---

## 1. O defeito que a wave encontrou: a seta do menu da família

O grupo de ferramentas (Seleção, Transformação) é um *split button*: botão
principal + seta que abre o menu da família. No modo compacto o botão ocupa
`TOOLBAR_WIDTH` (40px) e a seta ocupa **21.4px** medidos — juntos, 63.4px com o
vão. A coluna de ferramentas nascia em 48px (política herdada do `Panel::left`),
e a célula da paleta é a coluna menos 10px de moldura.

Resultado: **a seta era desenhada fora do paine e recortada**. Medido com a
paleta em 40/48/64px de coluna, o conteúdo ocupava 74px — a seta existia no
código, no tooltip e no teste, e era invisível e inalcançável na tela. O menu da
família só era acessível por atalho.

Isso não é um detalhe estético: é o mesmo tipo de falha que a Wave 5 encontrou na
Top Bar — geometria decidida por número herdado, nunca medida contra o conteúdo
que a coluna precisa hospedar.

**Correção (declarativa, sem breakpoint novo):**

1. `plan_group(cell_width)` decide o arranjo: com célula suficiente, botão + seta;
   sem, **só o botão** (o menu continua acessível pelo clique secundário, com a
   dica no tooltip). Nunca se desenha um controle fora da célula.
2. o mínimo da coluna passou a ser **derivado** do conteúdo:
   `TOOLBAR_WIDTH + TOOLBAR_CONTROL_GAP + TOOLBAR_CHEVRON_WIDTH + TOOLBAR_CHROME_WIDTH`
   = 40 + 2 + 22 + 10 = **74px** (`tokens::TOOLBAR_MIN_WIDTH`), com cada parcela
   documentada com o valor medido que a originou.
3. `core::TOOLBAR_DEFAULT_WIDTH` espelha o mesmo valor (o padrão é o mínimo
   utilizável, não um número independente).

**Evidência (teste `crates/ui/tests/toolbar_fit.rs`):** para colunas de 74 a
240px, a largura realmente ocupada pelo conteúdo é **exatamente** a da coluna.
Antes, o conteúdo estourava em 74px em toda a faixa de 40 a 72px.

**Delta visual deliberado:** a coluna de ferramentas nasce 26px mais larga
(48 → 74). É o preço de a coluna comportar o que ela existe para hospedar; a
alternativa era manter um controle invisível. Continua sendo trilho de ícones
(o rótulo só aparece a partir de 120px de célula, ou seja, com a coluna arrastada).

---

## 2. `PetuniaToolGridSpec` — a grade da paleta (§48)

O produto não decide mais nada sobre colunas nem sobre "modo compacto":

```rust
static TOOL_GRID: PetuniaToolGridSpec =
    PetuniaToolGridSpec::new(tokens::TOOLBAR_WIDTH, 120.0, 3.0);
```

- `min_cell` (40) = célula só-ícone; `label_min_cell` (120) = largura a partir da
  qual o botão mostra o rótulo.
- `plan()` delega a `taffy_layout::fit_columns` — a **única** fórmula de divisão
  do projeto; a contagem sai do mínimo real do item, nunca de um limiar.
- cada item recebe `PetuniaToolCell { width, columns, labeled }`; o botão usa
  `width` (novo `PetuniaToolbarButton::width`) e `labeled` (que substitui
  `compact = available_width() < 90.0`).
- o teto de colunas continua vindo da preferência do usuário
  (`toolbar_columns`: lista ou par) — o que muda é que o número **efetivo** de
  colunas e a largura de cada uma são medidos.

Os três workspaces (Paint/UV/Animate) usam a mesma grade com 1 ou 2 itens, então
a paleta inteira fala um contrato só.

## 3. `PetuniaForm` — linhas de campo sem breakpoint (§48)

Contrato novo em `adapters/form.rs`, com a decisão pura e testável:

```text
Inline   ← o controle ainda cabe com `control_min` depois do rótulo
Stacked  ← não cabe; o rótulo vai para cima e o controle recebe a faixa inteira
```

- `plan_row(available, spec) -> PetuniaFormRow { placement, label_width, control_width }`
  é puro (sem `Ui`), e é assim que painel estreito, escala de UI maior e idioma
  longo ficam cobertos por teste.
- a geometria `Inline` vem da nova primitiva
  `taffy_layout::fixed_label_row`: **uma** linha taffy, rótulo com largura
  declarada e controle com o resto — e a mesma largura passada ao `Ui` do item,
  então o campo não mede o container. Em célula mais estreita que rótulo + vão, o
  rótulo cede até no máximo metade da faixa: o controle nunca recebe zero.
- `section()` centraliza título + descrição + respiro (antes cada aba repetia
  `strong().size(13.0)` + `add_space(8.0)`); `toggle()` centraliza o alternador
  rotulado.

**Por que sem `egui_form` ainda:** o contrato é do Petunia e o motor é o
`egui_taffy` já instalado. A crate de validação entra na Wave 9 (§51) como
**backend** do mesmo contrato (erro por campo, estado de validação), sem
reescrever Settings. A vitrine §38 passou a listar a pendência como
*"form validation"*, não como *"form"*.

### Migrado em Settings (834 linhas → mesmas telas, sem breakpoint)

| Onde | Antes | Depois |
| --- | --- | --- |
| Barra de abas | `ui.horizontal_wrapped` + `spacing_mut` | `wrap_row` do adapter |
| Seções (6 abas) | `strong().size(13.0)` + `add_space` | `PetuniaForm::section` |
| Densidade | `ui.horizontal` com rótulo `size(11.5)` | `PetuniaForm::field` (controle recebe a largura) |
| Checkboxes | `ui.checkbox` solto + `size` literais | `PetuniaForm::toggle` |
| Perfis de keymap | `ui.horizontal` com N botões | esteira flex (rótulo + 1 item por perfil) |

Comportamento idêntico; o `changed()` do checkbox continua sendo o gatilho de
`mark_dirty` (o `toggle` devolve exatamente isso). O hint do checkbox de
exportação virou linha própria de legenda — antes vivia só no hover.

## 4. Reference Manager — grade e cabeçalho (§48)

- **Grade:** `grid_columns` + `grid_card_width` + laço de `chunks` foram
  substituídos por `taffy_layout::columns` com
  `PetuniaColumnSpec::responsive(SLOTS.len(), MIN_CARD, GRID_GAP)`. O cartão
  recebe a largura pronta; a quebra de linha é do motor. As fórmulas que viviam
  no produto (e as duas funções de teste que as guardavam) morreram.
- **Cabeçalho:** o par `if available_width() < 560.0 { empilhado } else { ... }`
  virou **uma** esteira `wrap_row` com `SpaceBetween`: descrição, limpar tudo e
  adicionar imagem são três itens; quando não cabem, as ações descem sozinhas e a
  ordem se mantém.

## 5. Cobertura nova

| Teste | O que garante |
| --- | --- |
| `toolbar_fit::the_palette_never_draws_past_its_column` | conteúdo ≤ coluna em 74/90/120/180/240px |
| `toolbar_fit::the_palette_keeps_the_whole_column_at_the_declared_minimum` | o mínimo entrega a linha de grupo inteira |
| `toolbar::the_declared_minimum_fits_a_whole_group_row` | idem, cruzando coluna × célula (via `TOOLBAR_CHROME_WIDTH`) |
| `toolbar::a_cell_too_narrow_for_the_arrow_keeps_the_menu_reachable` | degradação: sem seta, botão compacto inteiro |
| `toolbar::a_wide_palette_gives_the_group_button_the_label` | coluna larga volta a mostrar rótulo |
| `tool_grid::*` (5) | colunas, `labeled`, não-estouro, célula idêntica por item |
| `form::*` (5) | fronteira exata Inline/Stacked, largura nunca negativa, render nos dois arranjos |
| `taffy_layout::fixed_label_row_*` (2) | rótulo com a largura declarada e controle com o resto; controle nunca zerado |
| `reference_manager::grid_tests::*` (3) | colunas pelo mínimo do cartão, linha sem estouro, spec cobre os 6 slots |

**Nota de implementação que vale para as waves 7–9:** o callback de item de uma
grade taffy roda **mais de uma vez por item** (passagem de medida + passagem de
posicionamento). Ele é um desenho, não um acumulador. O teste
`tool_grid::every_item_receives_the_same_resolved_cell` mede exatamente isso, e a
doc pública de `PetuniaToolGridSpec::show` passou a declarar a regra.

## 6. Medições registradas em `docs/dependencies/ui-ecosystem-lock.md`

- seta do grupo (`small_button("▾")`) = **21.4px** no tema padrão ⇒ token 22;
- célula da paleta = coluna − **10px** (moldura 4+4 + arredondamento do motor);
- com a grade, a paleta ocupa **exatamente** a largura da coluna em 74–300px
  (antes: 74px fixos em qualquer coluna de 40 a 72px — o estouro da seta).

## Pendências geradas

1. **`docs/public/ui-map.json` congelado e desatualizado.** O gate `docs-check`
   valida o mapa contra os símbolos do código, e o mapa aponta para símbolos que
   as Waves 5b/6 aposentaram (`right_panel`, `draw_split_dock`,
   `animate_workspace_center`, `scene_panel_height`, `toolbar::draw`). O arquivo
   vive em `docs/public/`, que está **congelado** (AGENTS.md §1): atualizá-lo exige
   `cargo run -p xtask -- bible-lock` e a decisão explícita de descongelar.
   Decisão do projeto: **manter congelado** e registrar aqui. A correção está
   descrita e testada localmente (7 nós: arquivo + `entry` + `position`).
2. **Captura real de screenshots** (§40/§42) continua pendente, agora com um delta
   visual deliberado a fotografar: a coluna de ferramentas em 74px.
