# 24 — Design System Visual: Tokens, Hierarquia e Estados

<aside>
🧱

Este capítulo transforma a análise do Figma em uma **estrutura de design tokens e regras visuais para Petunia3D**. Valores observados na referência são registrados separadamente de valores finais ainda sujeitos a validação de acessibilidade e protótipo.

</aside>

# Fonte de verdade

Figma define intenção visual e medidas de referência. Notion define comportamento/semântica. O código implementa os componentes reais.

```
Figma = visual truth
Notion = product/behavior truth
Code = implementation truth
```

# Princípio de tokenização

Nenhum componente principal deve depender de hex, radius e spacing espalhados. Converter a linguagem visual para tokens semânticos.

Namespaces sugeridos:

```
surface.*
border.*
text.*
accent.*
state.*
spacing.*
radius.*
size.*
typography.*
motion.*
```

# Surface tokens

Base inicial derivada do Figma de referência:

```
surface.deep      ≈ #121212
surface.field     ≈ #1D1D1D
surface.chrome    ≈ #202020
surface.secondary ≈ #282828
surface.control   ≈ #303030
surface.raised    ≈ #313131
```

Esses valores são **baseline visual de referência**, não compromisso irrevogável de tema. Devem ser testados em monitor real e com contraste adequado.

# Border tokens

```
border.subtle  ≈ #313131
border.default ≈ #424242
border.active  ≈ #5B8EFF na referência
```

Bordas geralmente têm 1 px e são parte fundamental da hierarquia. Evitar remover borders e compensar com sombras pesadas.

# Text tokens

Referência:

```
text.secondary ≈ #D3D3D3
text.primary   ≈ #FFFFFF
```

Petunia deverá acrescentar tokens específicos para disabled/muted/warning/error/success conforme a acessibilidade for fechada.

# Accent semantics

Na referência, azul identifica estado ativo. Regra de Petunia:

**accent deve significar principalmente active / selected / focus / current**, evitando usá-lo simultaneamente como warning, categoria decorativa ou status semântico não relacionado.

O estado ativo não deve depender exclusivamente da cor do glyph; combinar superfície/border/indicador de estado.

A baseline Petunia usa **violeta floral em torno de `#B58CFF`** como ponto inicial de implementação. Pequenos ajustes de luminância/contraste são tuning permitido, mantendo essa semântica.

# Spacing scale

A referência usa principalmente gaps pequenos e regulares. Escala inicial recomendada para protótipo:

```
2
4
6
8
10
12
16
20
```

Não criar dezenas de valores semelhantes (`7`, `9`, `11`, `13`) sem necessidade comprovada.

# Radius scale

Referência observada: ~4 px em controles e ~12 px no chrome externo da janela.

Para Petunia, trabalhar com uma escala pequena:

```
radius.control
radius.segment
radius.panel
radius.window
radius.pill
```

Baseline V1:

```
radius.control = 4
radius.segment = 5
radius.panel   = 8
radius.window  = 10
radius.pill    = full
```

A regra permanece **radius discreto, não estética de card arredondado excessivo**.

# Hierarquia de altura

A referência estabelece níveis claros:

```
application chrome ~40
main header ~37
panel/editor header ~30
workspace control ~20
micro controls ~17
```

Petunia deve preservar a ideia de níveis dimensionais, mas ajustar tamanhos para legibilidade/escala real em vez de copiar 1:1.

# Tipografia

Referência: Inter Regular com 8/10/12 px.

Decisão de Petunia: **não adotar 8 px como baseline de texto interativo**. A escala final será definida junto com acessibilidade e UI scaling.

Baseline V1 congelada:

```
Caption      10
UI Small     11
UI Default   12
Panel/Strong 13
Exceptional  14
```

O toolkit deve permitir escala global e HiDPI sem quebrar layout.

# Tipografia como hierarquia, não decoração

Evitar títulos grandes dentro do editor. Usar peso, contraste e spacing para distinguir:

- application title;
- panel title;
- field label;
- secondary metadata;
- hint/status.

# Sombras

Sombras devem ser raras e discretas, usadas para separar overlays/pills/popup menus do plano imediatamente abaixo. Não criar cards profundos ou drop shadows grandes em todos os painéis.

# Separadores

Separadores de 1 px e pequenos spacers verticais/horizontais são preferidos a caixas extras. Exemplo conceitual:

```
File Edit Help | Workspaces | Project status
```

# Controle ativo

Baseline visual:

```
Default: surface.control + border.default
Hover:   leve aumento de contraste
Active:  accent surface/border + glyph/text legível
Focus:   indicação independente e acessível
Disabled: contraste reduzido sem tornar conteúdo ilegível
```

Hover/focus/disabled não estavam completamente especificados no Figma analisado; são requisitos que o Petunia precisa acrescentar.

# Component states obrigatórios

Todo componente interativo deve contemplar quando aplicável:

```
default
hover
pressed
active/checked/selected
disabled
keyboard focus
error/warning quando semântico
```

Não implementar apenas Default/Active porque o concept Figma só apresenta esses variants em muitos componentes.

# Segmented controls

Usar para estados mutuamente exclusivos e poucos:

```
[ Object | Face | Edge | Point ]
[ Wire | Solid | Texture | Silhouette ]
```

Segmentos compartilham borda/estrutura visual. O estado atual deve ser evidente por mais de uma pista visual.

# Split button

Padrão oficial candidato para ação/toggle com opções secundárias:

```
[ Mirror ][▼]
[ Snap   ][▼]
[ Overlay][▼]
```

Área principal executa/toggle; seta abre configuração. A hit area real deve ser maior que o glyph visual.

# Inputs compostos

Controles pertencentes ao mesmo dado podem formar grupos segmentados, evitando espaços e labels redundantes.

Exemplos:

```
[ X ][ Y ][ Z ][▼]
[ Front ][ reference.png ][×]
```

# Ícones e labels

Regra de aprendizado:

- ações universais e reconhecíveis podem ser icon-only: Undo, Redo, visibility, settings;
- ferramentas importantes para iniciantes devem preferir icon + label ao menos na apresentação inicial: Draw, Cut, Mirror, Connect;
- todo icon-only precisa de accessible name e tooltip;
- não depender de um glyph obscuro como única explicação.

# Motion

Motion deve explicar transição e estado, não decorar.

Baseline V1:

```
hover/menu       80–100 ms
collapse/panel   ~140 ms
structural       <= 180 ms
```

Reduced-motion é obrigatório e pode tornar transições praticamente instantâneas. Evitar qualquer animação contínua puramente decorativa.

# Temas

**Dark é o tema oficial completo da V1** e High Contrast é a variação oficial de acessibilidade. O Design System continua inteiramente baseado em tokens semânticos.

Usuários podem criar, editar, importar, exportar e compartilhar temas através de **Theme Extensions declarativas**, sem código executável. O formato compartilhável baseline é `.petunia-theme`, um ZIP versionado com `theme.toml`, `tokens.json` e assets opcionais validados.

Temas suportam herança (`extends = "petunia.dark"`) e overrides de famílias theme-safe como `surface.*`, `border.*`, `text.*`, `accent.*`, `selection.*`, `state.*`, `shadow.*`, radius dentro de ranges suportados, motion dentro de limites de acessibilidade, typography resources válidos e icon mappings tematizáveis.

Um tema **não pode** quebrar minimum hit areas, focus/keyboard semantics, accessible roles/names, layout topology, tamanho mínimo do viewport, indicadores de segurança/destructive actions ou requisitos do High Contrast. Spacing/metrics estruturais usam density profiles/ranges validados em vez de liberdade irrestrita.

Settings → Appearance deve oferecer theme picker, preview, `Create Theme from Current`, duplicate/rename, edição de tokens semânticos, import/export, reset de token e warnings de contraste. Tema nunca altera documento `.petunia`.

O contrato completo de `.petunia-theme`, herança, validação e integração com Plugin Panels está no capítulo 36.

# Regra final

**Design system é um contrato, não um arquivo de cores.** Cada token deve existir para reduzir divergência entre componentes e permitir que uma mudança visual seja corrigida em um único lugar.