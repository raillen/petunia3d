# Análise e classificação dos SVGs exportados do Figma

## Referência principal

`Blender(1).svg` foi usado como mapa de montagem da interface. Ele representa a tela completa em 1920×1080 e ajuda a localizar a função dos SVGs menores.

A composição visual é, em alto nível:

1. `windows_bar.svg` — barra de título da janela/sistema.
2. `header.svg` — header global do aplicativo, menus e workspaces.
3. Área principal:
   - painel lateral esquerdo / Outliner;
   - viewport 3D central;
   - painel Properties/Data à direita.
4. Timeline na parte inferior.
5. `status_bar.svg` no rodapé.

`UI Buttons.svg` e `UI Elements.svg` funcionam como pranchas/bibliotecas de componentes e não como uma região única da tela final.

---

## Classificação por arquivo

| Arquivo | Dimensão | Classificação | Conteúdo principal |
|---|---:|---|---|
| `Blender(1).svg` | 1920×1080 | composição / referência | tela completa, viewport, painéis, timeline, headers e status |
| `windows_bar.svg` | 1920×40 | window chrome | logo/título da janela e controles minimizar/maximizar/fechar |
| `header.svg` | 1920×37 | application header | logo, File/Edit/Render/Window/Help, workspaces em pills e seletores da direita |
| `3d-view_bar.svg` | 1200×30 | viewport header | Object Mode, View/Select/Add/Object/Face, orientação, snapping, proportional editing, gizmos/overlays/shading |
| `toolbar.svg` | 140×416 | viewport tool shelf | duas apresentações da toolbar vertical; 9 ferramentas principais |
| `outliner_panel_header.svg` | 340×30 | panel header | seletor/editor, pesquisa, filtro e ações do Outliner |
| `properties_panel_header.svg` | 340×30 | panel header | seletor/editor, pesquisa e menu do painel Properties |
| `data_panel_vertical_menu.svg` | 30×646 | vertical tab rail | 15 botões/tabs verticais para categorias do painel Properties/Data |
| `properties.svg` | 2900×716 | component gallery | 8 telas/estados do painel Properties: Metadata, Select Box, Render, Output, View Layer, Scene, World, Collection |
| `timeline_panel_header.svg` | 1900×30 | timeline header | Playback/View/Marker, transport controls, snapping e campos de frame |
| `timeline_list_panel.svg` | 160×234 | timeline sidebar | pesquisa e árvore/lista `Summary` |
| `timeline_cursor.svg` | 50×234 | timeline overlay | playhead/cursor azul e marcador de frame |
| `status_bar.svg` | 1920×30 | status/footer | hints de mouse à esquerda e tempo/frame/memória/versão à direita |
| `UI Buttons.svg` | 836×400 | component library | 11 famílias de botões/controles e variantes de estado |
| `UI Elements.svg` | 400×400 | component library | 2 famílias de avatares/imagens circulares em tamanhos/estados diferentes |

---

## UI Buttons — 11 famílias detectadas

As caixas roxas tracejadas do próprio SVG foram usadas como limites de componente. Os nomes abaixo são classificação visual/funcional, não nomes de layers recuperados do Figma (os nomes semânticos originais não foram preservados no SVG exportado).

1. Toggle + dropdown de gizmo/ferramenta de viewport.
2. Toggle + dropdown de overlays/visualização.
3. Snapping com tipo/modo de snap e dropdown.
4. Proportional Editing com seletor de falloff.
5. X-Ray / visualização através da geometria.
6. Viewport Shading: Wireframe, Solid, Material Preview, Rendered + menu.
7. Auto Keying / Record, estados desligado/ligado + menu.
8. Checkbox simples, off/on.
9. Controle segmentado de navegação/playback + dropdown.
10. Snapping compacto (variante usada em editor/timeline) + dropdown.
11. Controle segmentado de operação de seleção, com 5 variantes.

---

## Toolbar — ícones identificados

O `toolbar.svg` contém **9 imagens PNG RGBA de 256×256 embutidas no próprio SVG**. Elas foram extraídas byte a byte, portanto são os assets originais presentes no export, sem recorte e sem recompressão.

1. `select_box` — Box Select.
2. `cursor_3d` — 3D Cursor.
3. `move` — Move / Translate.
4. `rotate` — Rotate.
5. `scale` — Scale.
6. `transform` — transform combinado.
7. `annotate` — Annotate / Draw.
8. `measure` — Measure.
9. `add_primitive` — adicionar primitiva/ferramenta custom de adição.

Foram geradas duas formas:

- `toolbar/icons_exact_png/` — fontes PNG exatas embutidas no SVG;
- `toolbar/buttons/svg/` — botões completos extraídos como SVG, incluindo fundo/borda/ícone.

---

## Properties — 8 painéis separados

A prancha `properties.svg` tem oito painéis de 340 px de largura, separados por gaps de 20 px. Foram extraídos individualmente como SVG e preview PNG:

1. `metadata`
2. `select_box_tool`
3. `render`
4. `output`
5. `view_layer`
6. `scene`
7. `world`
8. `collection`

---

## Vertical Properties/Data tabs

Foram detectados e separados 15 botões de 22×22 dentro de `data_panel_vertical_menu.svg`.

Os últimos itens seguem visualmente a linguagem de categorias conhecidas de Properties (por exemplo Object, Modifiers, dados do objeto e Material), mas o SVG exportado perdeu os nomes originais das layers; por isso o pacote usa nomes neutros `data_tab_01` … `data_tab_15` em vez de inventar nomes canônicos.

---

## Elementos rasterizados encontrados dentro dos SVGs

Nem tudo pode ser recuperado como vetor porque o Figma já exportou alguns conteúdos como `<image>` embutido.

- `toolbar.svg`: 9 PNG RGBA, 256×256 — ícones das ferramentas.
- `UI Elements.svg`: 1 JPEG, 736×736 — imagem usada nos avatares/previews circulares.
- `header.svg`: 1 JPEG, 736×736 — a mesma imagem/asset visual de avatar.
- `properties.svg`: 1 PNG, 60×60 — asset usado em uma área de ferramenta.
- `Blender(1).svg`: 11 imagens embutidas, incluindo a imagem do viewport e cópias dos assets da toolbar/avatar.

Essas imagens foram extraídas para `embedded/`.

---

## Método de extração usado

O extrator não corta paths arbitrariamente.

1. Faz parse XML do SVG.
2. Usa Inkscape para calcular bounding boxes reais dos nós renderizados.
3. Detecta as caixas-guia roxas do Figma (`#8A38F5`, tracejadas) quando existem.
4. Seleciona somente os nós visuais pertencentes à região.
5. Preserva `defs`, masks, patterns, clip paths e filters necessários.
6. Remove as caixas-guia roxas do asset extraído.
7. Mantém o `viewBox` correspondente à área original para não quebrar transforms, masks ou patterns.
8. Extrai também qualquer `<image data:...>` embutida sem recompressão.
9. Gera previews PNG para conferência visual.

Isso é mais seguro do que separar cada `<path>` isoladamente.

---

## Limitação importante

Os SVGs exportados não preservaram nomes semânticos de todos os grupos/layers do Figma. Muitos IDs restantes pertencem a masks, patterns e clip paths gerados automaticamente. Portanto:

- a separação geométrica/visual pode ser exata;
- a classificação semântica de alguns pequenos ícones precisa ser inferida pela aparência e posição na referência;
- para recuperar nomes de componentes/layers com 100% de fidelidade, o melhor caminho ainda é ler o arquivo original via Figma API/plugin antes da exportação.

O pacote atual evita atribuir nomes específicos quando a evidência visual não é suficiente.
