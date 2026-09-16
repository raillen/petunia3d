# 26 — Figma → Implementação, Assets e Fundamentos de Acessibilidade

<aside>
♿

Este capítulo define como transformar o Figma em implementação real sem repetir o problema de interfaces visualmente próximas mas frágeis, difíceis de personalizar ou pouco acessíveis. A regra principal é: **exportar assets do Figma; não exportar a interface como imagem/código descartável.**

</aside>

# Três fontes de verdade

```
Figma → intenção visual, medidas, tokens, assets
Notion → comportamento, UX, decisões e estados
Code → implementação executável e componentes reais
```

Nenhuma dessas fontes substitui silenciosamente as outras.

# O que extrair do Figma

Usar o Figma para:

- cores/tokens;
- spacing;
- radius;
- dimensões de referência;
- tipografia;
- estados visuais;
- layout/proporções;
- iconografia própria;
- branding/logo;
- ilustrações;
- assets gráficos reais.

# O que NÃO exportar como asset

Não transformar em PNG/SVG estático:

- buttons;
- inputs;
- dropdowns;
- tabs/workspace switch;
- panels;
- trees;
- scrollbars;
- menus;
- property rows;
- tooltips.

Esses itens precisam ser controles reais do toolkit/component library.

# Política para componentes do ecossistema

A baseline visual final usa **egui + eframe**, com `egui-wgpu` para a fronteira gráfica. Usar primitives do egui somente por trás de **Petunia Components** ou adapters claramente delimitados quando o componente for parte recorrente da linguagem visual do produto.

O vertical slice técnico do capítulo 31 valida conformance/integração da stack final; não mantém o toolkit em estado de escolha pendente.

Isso deve preservar:

- keyboard navigation;
- focus;
- AccessKit/accessibility semantics;
- DPI/HiDPI;
- text rendering;
- input method/editor support;
- menus/popups;
- states de mouse;
- platform integration.

Não escolher um toolkit ou UI kit apenas porque imita melhor o Figma em screenshots; avaliar custom styling, semântica, input, testes e comportamento real.

# Política de ícones

## Ícones comuns

Se o glyph do Figma vem de um icon set conhecido e a licença for compatível, preferir a fonte oficial desse icon set na aplicação em vez de reexportar uma cópia arbitrária.

## Ícones Petunia

Ferramentas específicas como Trace, Project Reference, Connect, Fuse, Silhouette e Smart Snap podem ter SVGs próprios desenhados no Figma e versionados em `assets/icons`.

## Branding

Logo, wordmark, splash artwork e ilustrações próprias são bons candidatos a export Figma.

## Licença

Não reutilizar branding, logos ou assets proprietários do Blender/design concept sem confirmação explícita de licença. A referência inspira; Petunia possui seus próprios assets.

# Viewport não é UI estática

Elementos espaciais devem ser renderizados pelo `ViewportRenderer`:

```
grid
mesh
wireframe
selection outline
points/edges/faces highlights
gizmos
extrude handles
profile points
snap hints
reference guides
triangulation overlay
```

Figma especifica aparência; renderer implementa geometria, hit testing e interação.

# Gizmos

Não exportar gizmo inteiro como SVG clicável. Pode haver assets auxiliares, mas arrows/rings/handles devem conhecer posição 3D, depth, occlusion, scale e hit zones. O renderer é a autoridade.

# Design token pipeline

Fluxo atual:

```
Figma styles/variables
→ optional extraction/export tooling
→ audited Petunia token table
→ source-controlled Rust token definitions
→ PetuniaThemeAdapter
→ Petunia Components sobre egui
```

Qualquer geração é intermediária e revisada. O build **não deve depender de uma chamada ao Figma MCP**. Tokens e assets aprovados são versionados no repositório.

# Figma Design-to-Code

Código React/Tailwind retornado por ferramentas de handoff é **referência**, não código de produção para Petunia. Não introduzir runtime web, Tailwind ou CEF apenas porque o extractor gera esse formato.

Converter intenção para o toolkit final e componentes do projeto.

# Estrutura recomendada no repositório

```
design/
  tokens/
    colors
    spacing
    radius
    typography
    motion
  components/
    specifications
  references/
    screenshots/notes
assets/
  icons/
  branding/
  images/
```

Na baseline Rust/egui, complementar com:

```
crates/petunia-ui/src/
  foundation/
  components/
  adapters/
  workspaces/
  devtools/
```

A separação conceitual entre design reference, assets e implementation permanece obrigatória.

# Acessibilidade: controles reais

Um desenho de botão exportado não é semanticamente um botão. Um controle real deve expor quando aplicável:

```
role
accessible name
enabled/disabled
checked/selected/expanded state
keyboard activation
focus order
```

Isso é motivo arquitetural para preferir componentes do toolkit em vez de arte estática.

# Não depender somente de cor

Active/selected/focus/error não devem ser comunicados exclusivamente por hue. Usar combinação apropriada de fill, border, indicator, icon/shape ou text state. A referência Figma já usa background + border para active; Petunia deve evoluir isso para focus/accessibility completos.

# Tamanho visual versus hit target

Controles podem manter aparência compacta, mas a área interativa não precisa ser tão pequena quanto um glyph de 8–10 px. O toolkit deve permitir hit areas adequadas sem destruir a densidade visual.

# Tipografia e legibilidade

Os 8 px observados no Figma são referência estética, não baseline final. A escala tipográfica e de UI está congelada no capítulo 36 e deve ser validada em:

- 100%, 125%, 150%, 175%, 200% UI scale;
- HiDPI;
- Linux e Windows;
- labels longos;
- i18n;
- monitores com densidades diferentes.

# Keyboard-first

Toda operação frequente deve poder ser acessada sem mouse, mesmo que a interface seja fortemente visual. Requisitos gerais:

- focus visível;
- navegação previsível;
- shortcuts configuráveis;
- Escape cancela operação temporária quando aplicável;
- Enter/confirm tem comportamento consistente;
- toolbars e segmented controls não podem ser traps de foco.

Os templates Blender/Maya/3ds Max/C4D previamente previstos são uma camada de mapping; a apresentação e remapping serão definidos na fase específica de atalhos.

# Screen reader / accessibility tree

Com egui como baseline final, mapear explicitamente roles/names/states na árvore **AccessKit** para:

- menus;
- toolbars;
- tree de Parts;
- Context fields;
- dialogs;
- workspace switch;
- status/validation messages.

Cada Petunia Component customizado deve preservar/inserir `WidgetInfo`/semântica equivalente apropriada: role, accessible name/description, enabled/selected/checked/expanded e comportamento de keyboard/focus quando aplicável. O vertical slice deve validar a accessibility tree em Windows/Linux na medida em que adapters/plataformas permitirem. O viewport 3D exigirá estratégia própria para seleção/estado, mas isso não justifica tornar o shell inacessível.

# Tooltips e manual

Tooltips devem ensinar nome e função; `?`/Help pode abrir documentação detalhada. Não esconder parâmetros críticos exclusivamente em hover.

# High-DPI e scaling

A UI precisa ser desenhada em unidades lógicas e testada em múltiplos scales. Evitar layout baseado em screenshots fixas de 1920×1080. O frame Figma é referência proporcional, não canvas rígido.

# Responsividade desktop

Petunia é desktop, portanto “responsive” significa adaptar-se a tamanhos de janela e densidade, não transformar-se em mobile layout. Prioridades:

1. preservar viewport mínimo útil;
2. permitir collapse de Parts/Context/Assets;
3. reduzir/overflow controls de baixa prioridade;
4. nunca simplesmente sobrepor conteúdo crítico sem regra.

# Checklist de handoff Figma → egui/Petunia Components

Para cada component/tela aprovada:

1. identificar node/component/variant de referência;
2. registrar medidas/tokens e marcar FIGMA-CONFIRMED vs PETUNIA-ADOPTED;
3. separar assets reais de controles;
4. mapear estado semântico;
5. mapear keyboard/focus/accessibility;
6. implementar/reutilizar Petunia Component em Rust;
7. `cargo check`/build;
8. executar `egui_kittest` quando aplicável;
9. executar a aplicação development build;
10. inspecionar AccessKit tree via tooling de inspection quando aplicável;
11. capturar screenshot e comparar com referência aprovada;
12. exercitar mouse/keyboard;
13. testar scaling/i18n;
14. registrar delta se a implementação precisar divergir do Figma.

A definição de pronto nunca é apenas “o código compila”.

# Regra final

**Fidelidade visual não pode custar comportamento, manutenção ou acessibilidade.** O melhor resultado é o controle real do ecossistema escolhido, estilizado para parecer Petunia, e não uma captura do Figma transformada em software.