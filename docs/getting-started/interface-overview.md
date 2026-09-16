# Visão Geral da Interface

O Petunia3D usa um shell **viewport-first** e profissional simplificado: `Parts` à
esquerda, `Context` à direita, `Asset Library` embaixo e o viewport no centro. Não é
um "Blender amputado" nem um aplicativo infantil.

Medidas em **logical px**, antes de UI scaling.

## 1. Top Bar (`40 px`)

- **Menus/projeto** à esquerda (`File`, `Edit`, `View`, `Help`).
- **Workspace pills** ao centro: `MODEL`, `PAINT`, `UV`. Workspace não implementado
  **não aparece** como pill desabilitada.
- **Ações globais** à direita (salvar, validação, settings).

## 2. Viewport (centro)

O centro é o viewport/editor do workspace e mantém cerca de `480 × 360` logical px
antes de ceder espaço a painéis.

- Navegação: `MMB` orbita, `Shift+MMB` faz pan, roda/pinch faz zoom.
- **Toolbar contextual dentro do viewport** — as ferramentas aparecem conforme o
  domínio de seleção (`Object` / `Face` / `Edge` / `Point`) e a ferramenta ativa.
  Não existe uma coluna permanente de 20+ ícones.
- **Domínio de seleção**: `Object` para malhas inteiras e o último domínio de
  componente usado. Os alvos `Point` (`1`), `Edge` (`2`) e `Face` (`3`) ficam
  disponíveis quando você trabalha nos componentes da malha.
- **Modos-base**: `Wireframe`, `Solid`, `Textured`, `Silhouette`. Overlays (grid,
  eixos, contagem de geometria, wireframe sobre superfície) são composição sobre o
  modo-base.

## 3. Parts (`248 px` default, `200–400`)

Hierarquia da cena: collections, objetos, overlays e anotações, com visibilidade,
bloqueio e isolamento por item. É recolhível e redimensionável em divisor autorizado.
Extension panels autorizados podem aparecer nesta região.

## 4. Context (`288 px` default, `240–440`)

Painel **selection/tool-centric**: parâmetros da ferramenta ou da operação em foco,
propriedades do objeto selecionado e material. Campos numéricos aceitam arrasto
horizontal. Extension panels autorizados podem aparecer aqui.

## 5. Asset Library (`176 px` default, `120–360`)

Gaveta inferior de assets reutilizáveis do projeto: modelos, paletas e presets.
Tem resize vertical e collapse; ao colapsar, o viewport recupera a área.

## 6. Status Strip (`~22 px`)

Dicas contextuais de mouse/teclado, estado de save, indicador de validação e
contagem discreta de pontos, faces e triângulos.

## 7. Regras de layout

- Panel header tem um contrato único de `28 px`; controles `28 px` (30–32 apenas
  quando a hierarquia justificar); gutter estrutural de `8 px` (`4` só entre
  controles intimamente relacionados).
- **Nada de docking irrestrito na V1**: resize existe apenas em divisores
  autorizados e double-click no divisor restaura a medida default. Não há janelas
  flutuantes arbitrárias.
- O estado de layout é persistido **por workspace**.
- Breakpoints: `≥ 1280` shell completo · `1024–1279` Asset Library inicia recolhida ·
  `< 1024` Parts/Context podem virar drawers/overlays temporários.

## 8. Personalização

Aparência vem de **tokens semânticos** (`ThemeToken`), então você pode trocar de
tema — inclusive criar e compartilhar o seu `.petunia-theme` — sem alterar o
documento. Ícones são endereçados por `IconId` e atalhos por `CommandId`, nunca por
texto, cor, glifo ou tecla fixa no código.

> **Acessibilidade é contrato:** foco visível, navegação por teclado
> (`F6`/`Shift+F6` entre regiões, `Tab`/`Shift+Tab` dentro da região) e
> `reduced-motion` fazem parte da V1, não são acabamento tardio.
