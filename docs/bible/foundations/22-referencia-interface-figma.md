# 22 — Referência de Interface: Análise do Figma Blender UI Redesign

<aside>
🎨

Este capítulo registra **o que foi efetivamente observado no arquivo Figma de referência**. Ele é evidência visual e estrutural, não autorização para copiar a arquitetura mental do Blender. Quando uma conclusão for inferência ou adaptação para Petunia3D, ela estará explicitamente marcada.

</aside>

# Fonte

Arquivo analisado: [Blender — UI Redesign Concept — Community](https://www.figma.com/design/nTwXeJiXClatWoGyspZIgO/Blender---UI-Redesign--Concept---Community-?node-id=0-1)

Nós inspecionados via Figma MCP:

- página `0:1` — **🖥️ UI**;
- página `202:9` — **🧩 Components**;
- frame principal `202:4` — **Blender**, 1920 × 1080;
- componente `202:28` — `windows_bar`;
- componente `202:10` — `header`;
- componente `202:568` — `3d-view_bar`;
- metadata dos principais painéis, toolbars e variants.

## Limitação da análise

O Figma MCP atingiu limite de leitura antes que todas as variantes internas do grande componente `properties` fossem abertas individualmente. Portanto dimensões, nomes, estrutura geral e componentes listados abaixo são confirmados; detalhes internos não inspecionados do painel Properties não devem ser tratados como especificação exata.

# Estrutura macro confirmada

O frame principal tem 1920 × 1080 e organiza a interface em quatro regiões de trabalho principais: Outliner à esquerda, 3D View ao centro, Properties à direita e Timeline abaixo, além das barras superiores e Status Bar.

```
Window bar: 40 px
Main header: 37 px
Editor region begins around y=77
Outliner: 340 × 676
3D View: 1200 × 676
Properties: 340 × 676
Timeline: 1900 × 264
Status bar: 30 px
Gutter externo recorrente entre regiões: ~10 px
```

A organização demonstra uma preferência por **canvas central dominante + painéis laterais claramente separados**, em vez de divisões sem respiro.

# Escala dimensional recorrente

| Elemento observado | Medida recorrente | Leitura para o design |
| --- | --- | --- |
| Window bar | 40 px | Nível de aplicação. |
| Main header | 37 px | Nível global/workspace. |
| Panel/editor header | 30 px | Nível de região. |
| Workspace pills | ~20 px | Context switch compacto. |
| Viewport controls | 17 px | Ferramentas de alta densidade. |
| Ícones pequenos | 8–12 px | Glyphs em controles compactos. |
| Ícones gerais | 10–20 px | Chrome e ações de aplicação. |
| Radius interno | ~4 px | Controles discretos. |
| Radius da janela | ~12 px | Separação entre chrome e controles. |

A escala visual diminui progressivamente de aplicação → workspace → região → controle. Essa hierarquia é um dos princípios mais valiosos da referência.

# Spacing observado

Gaps recorrentes incluem 2, 4, 5 e 10 px, com padding horizontal de aproximadamente 5 ou 10 px nos pequenos controles. Isso sugere uma escala compacta e sistemática, não valores arbitrários por componente.

# Paleta observada

As superfícies e bordas usam principalmente:

```
#121212 — fundo profundo / workspace pill inativa
#1D1D1D — superfícies de campo internas
#202020 — headers/chrome principal
#282828 — superfície secundária
#303030 — controles
#313131 — separadores/active surface em alguns contextos
#424242 — bordas e divisores
#D3D3D3 — texto secundário
#FFFFFF — texto de maior contraste
```

Accent ativo confirmado:

```
#3169E3 — background ativo
#5B8EFF — border ativo
```

O estado ativo altera superfície e borda, não apenas o glyph.

# Tipografia observada

A referência usa **Inter Regular**. Tamanhos frequentes observados: 8 px, 10 px e 12 px. Esses valores descrevem o concept Figma, **não constituem automaticamente a escala final de Petunia3D**, pois 8 px é inadequado como baseline acessível em muitas telas reais.

# Hierarquia visual

A profundidade é criada principalmente por:

- diferenças pequenas de luminância entre superfícies;
- borders de 1 px;
- spacing/gutters;
- radius discreto;
- sombras pequenas e pontuais em vez de grandes card shadows.

A referência evita a estética de dashboard web composto por cards independentes.

# Workspaces

O header contém pills para Layout, Modeling, UV Editing, Shading, Animation, Compositing e Scripting, além de `+`.

Comportamento visual observado:

- altura ~20 px;
- radius ~4 px;
- gap ~4 px;
- padding horizontal ~10 px;
- inativo sobre fundo muito escuro;
- ativo com superfície mais clara, texto branco e sombra discreta.

**Insight para Petunia:** adotar o princípio do switch compacto, não a quantidade nem os workspaces do Blender.

# 3D View Bar

O componente `3d-view_bar` tem 1200 × 30 e se divide visualmente em três agrupamentos.

## Esquerda

Editor/mode e menus contextuais, incluindo elementos equivalentes a `Object Mode`, View, Select, Add, Object e Face.

## Centro

Transform orientation, pivot, snapping e proportional editing.

## Direita

Visibility, Gizmo, Overlays, X-Ray e View3D Shading.

A organização pode ser entendida como:

```
WHAT am I editing?
HOW am I transforming?
HOW am I viewing?
```

Essa decomposição é altamente reutilizável em Petunia, mesmo com comandos diferentes.

# Split controls

Vários controles adotam o padrão:

```
[ toggle/action ][ dropdown ]
```

Exemplos observados: Snap, Gizmo, Overlays, shading e proportional editing. O clique principal alterna/aciona e a seta abre configuração secundária. É uma maneira eficiente de esconder opções sem criar painéis permanentes.

# Variants confirmadas na página Components

Foram observados components/variants para:

- `show_gizmo` — Default / Active;
- `show_overlays` — Default / Active;
- `use_snap` — Default / Active;
- `use_proportional_edit_objects` — Default / Active;
- `xray`;
- `view3d_shading` — Wire / Solid / Texture / Rendered;
- `select_options` — múltiplos modes;
- `toolbar` — ao menos duas variantes;
- `properties` — múltiplas tabs/variants;
- headers de Outliner, Properties, Timeline;
- status bar e timeline controls.

A referência portanto possui uma linguagem de **component variants**, não uma coleção de desenhos isolados.

# Toolbar vertical

No frame principal a toolbar do viewport fica dentro do canvas, aproximadamente em `x=10`, `y=70`, largura 40 px e altura na faixa de 360 px. É uma ferramenta **overlay/floating**, não uma coluna estrutural que reduza a largura do viewport.

# Outliner

O painel esquerdo observado tem 340 px de largura e rows bastante compactas, aproximadamente 16 px em partes da lista. A hierarquia apresenta chevron, tipo do objeto, nome e controles auxiliares como visibility/render flags.

**Leitura para Petunia:** a linguagem de árvore compacta é útil; os conceitos Scene/Collection/Data/Render não devem ser copiados automaticamente.

# Properties

O painel direito também tem 340 px. A página Components confirma variants para Project Info, Tool, Render, Output, View Layer, Scene, World e Collection. Isso evidencia que o redesign melhora a apresentação, porém preserva a arquitetura categórica do Blender.

**Leitura para Petunia:** aparência e proporção são referência; a taxonomia do Blender não é.

# Home screen

Foi observado um frame `home_screen` de 600 × 400 contendo banner, New project, Open project, Recover the last session, Changelog, Donation, Settings e uma lista de Last projects. O princípio de início simples e recuperação explícita é valioso; conteúdo específico do Blender não é requisito.

# Timeline

A Timeline ocupa 1900 × 264 quando presente. Como animação não faz parte da primeira experiência de modelagem do Petunia, essa região não deve ser copiada para o Model workspace apenas por existir na referência.

# Princípios visuais extraídos

1. Canvas first.
2. Chrome escuro e discreto.
3. Regiões separadas por gutters.
4. Headers compactos e consistentes.
5. Controles pequenos e agrupados.
6. Radius discreto.
7. Accent forte reservado a estado ativo.
8. Segmented controls para estados mutuamente exclusivos.
9. Split button para ação + configuração secundária.
10. Hierarquia por luminosidade, borders e spacing; evitar decoração excessiva.

# Regra de uso da referência

Petunia deve aprender a **linguagem visual** do redesign, não virar “Blender reorganizado”. A pergunta para cada padrão é: ele reduz carga cognitiva e preserva o viewport? Se sim, adaptar. Se apenas replica uma categoria necessária ao Blender generalista, descartar.