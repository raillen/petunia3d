# Interface & Layout

O Petunia3D tem uma hierarquia de painéis estável e determinística. A V1 é
**viewport-first** e **não** oferece docking irrestrito nem janelas flutuantes
arbitrárias: o que muda entre usuários é a visibilidade e o tamanho de regiões
**autorizadas**, sempre dentro de limites explícitos.

## Anatomia da V1

Medidas em logical px. O viewport mantém ~`480 × 360` antes de ceder espaço.

```
+------------------------------------------------------------------------------+
| 1. Top Bar (40): menus/projeto | pills MODEL · PAINT · UV | ações globais     |
+------------------+------------------------------------------+----------------+
| 2. Parts (248)   | 3. Viewport + toolbar contextual         | 4. Context     |
| 200–400          |    (Objects / Face / Edge / Point)       | 240–440        |
| recolhível       |                                          | selection/tool |
|                  |                                          |                |
+------------------+------------------------------------------+----------------+
| 5. Asset Library (176; 120–360, collapse)                                    |
+------------------------------------------------------------------------------+
| 6. Status Strip (~22): hints | save | validação | points/faces/tris          |
+------------------------------------------------------------------------------+
```

Cada painel usa **panel header de `28 px`** e **controls de `28 px`** (30–32 apenas
quando a hierarquia justificar). O gutter estrutural é de `8 px`.

## Como o layout é calculado

O grafo das quatro regiões é decidido por um **motor de macro-layout controlado**,
não por regras soltas em cada painel. Dentro de um painel, o arranjo é responsivo
por um motor de layout dedicado (`flex`, `wrap`, `grid`) — nunca por comparação de
largura escrita à mão.

Consequência prática para quem usa o editor: o arranjo dos controles é
**determinístico** e independente da resolução. Redimensionar a janela reorganiza
os campos em coluna ou linha por regra declarada, não por ponto de quebra
improvisado, e nada depende de a janela ter exatamente um certo número de pixels.

## Manipulação de layout

- **Divisores autorizados**: arraste o divisor para redimensionar dentro da faixa
  permitida (Parts `200–400`, Context `240–440`, Asset Library `120–360`).
- **Double-click no divisor** restaura a medida default.
- **Collapse**: Parts e Asset Library podem ser recolhidos; o viewport recupera a área.
- **Persistência**: o layout é salvo **por workspace** — `MODEL`, `PAINT` e `UV` têm
  estados independentes.
- **Densidade e escala**: presets de UI scaling `100% / 125% / 150% / 175% / 200%`,
  além da escala do sistema/HiDPI. O layout trabalha em unidades lógicas.

### O que a V1 não faz

- não permite remover permanentemente painéis do core;
- não permite docking livre nem janelas flutuantes arbitrárias;
- não permite que plugin reposicione o shell — Plugin Panels entram apenas em
  **extension slots** controlados (`left`, `right`, `bottom`).

Mudanças dessa natureza exigem decisão arquitetural explícita (ADR) porque alteram o
grafo estrutural do shell.

O detalhamento técnico (motores, adapters e o que é proibido em código de produto)
está em [`docs/developers/ui-architecture.md`](../developers/ui-architecture.md).

## Painéis e regiões

| Região | Conteúdo do core | Extension slots |
| :--- | :--- | :--- |
| Esquerda | Parts (hierarquia da cena) | plugin panels autorizados |
| Centro | Viewport / editor do workspace | substituir o centro **não** é permitido na V1 |
| Direita | Context (seleção/ferramenta/material) | plugin panels autorizados |
| Embaixo | Asset Library | plugin panels autorizados |

O usuário controla visibilidade de painéis em `View → Panels` ou pela Command Palette,
e pode mover um plugin panel somente entre as regiões permitidas por ele.

## Aparência

Toda aparência vem de tokens semânticos (`ThemeToken`): superfícies, bordas, texto,
accent, seleção, estado, sombra, radius, tipografia e motion. Trocar de tema — incluindo
temas criados por usuários via `.petunia-theme` — **nunca** altera o documento.

Tema oficial da V1: **Dark**. **High Contrast** é variação oficial de acessibilidade.
Semântica de accent: `selected`, `active`, `current`, `focus emphasis`; warning/error/
success têm famílias próprias.

## Acessibilidade

- Foco sempre visível e navegável por teclado.
- `F6` / `Shift+F6` navega as regiões principais; `Tab` / `Shift+Tab` navega controles
  dentro da região; setas navegam listas/segmented; `Space`/`Enter` ativam o controle focado.
- `reduced-motion` é obrigatório e nenhuma animação é puramente decorativa.
- Clicar fora **nunca** confirma silenciosamente operação destrutiva.

> Alterações de toolkit, grafo do shell, docking irrestrito ou exposição de UI/GPU a
> plugins são decisão arquitetural, não tuning. Pequenas calibrações visuais são tuning.
