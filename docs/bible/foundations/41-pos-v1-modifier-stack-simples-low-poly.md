# 41 — Pós-V1: Modifier Stack Simples e Low-Poly

<aside>
🧱

**Modifiers são uma lacuna real** na documentação atual. O Petunia já possui operações diretas como Mirror, Bevel e Sweep, mas não possui um contrato consolidado para efeitos geométricos não destrutivos e reordenáveis. Esta página define uma pilha deliberadamente pequena, previsível e voltada a low-poly.

</aside>

# Princípio

Modifiers existem para evitar trabalho repetitivo e preservar edição não destrutiva. Não devemos reproduzir a lista extensa do Blender. O usuário deve aprender poucos modificadores com nomes concretos e previsíveis.

```
Base Mesh
  ↓
Mirror
  ↓
Array
  ↓
Thickness
  ↓
Bend
  ↓
Result Preview
  ↓
Apply / Bake when desired
```

# Arquitetura

A pilha deve ser parte do modelo de authoring oficial, não um plugin. Cada item é um `GeometryModifier` serializável, determinístico e avaliável fora da UI.

```rust
pub struct ModifierStack {
    pub modifiers: Vec<ModifierInstance>,
}

pub struct ModifierInstance {
    pub id: ModifierId,
    pub enabled: bool,
    pub kind: ModifierKind,
}
```

Requisitos:

- reorder;
- enable/disable;
- duplicate;
- apply/bake;
- remove;
- preview;
- Undo/Redo;
- deterministic evaluation;
- cache por revision;
- diagnostics em resultado inválido;
- export sempre recebe geometria avaliada/baked.

# Modifiers recomendados

| Modifier | Uso | Dificuldade | Prioridade |
| --- | --- | --- | --- |
| Mirror | simetria não destrutiva | 3/10 | Muito alta |
| Array | repetição linear/radial simples | 3–4/10 | Muito alta |
| Thickness | adicionar espessura a superfícies | 4–5/10 | Muito alta |
| Bend | curvar asset ao longo de um eixo | 4/10 | Alta |
| Taper | afunilar ao longo de um eixo | 3/10 | Alta |
| Twist | torção controlada | 3–4/10 | Média/Alta |
| Surface Conform | aproximar/projetar mesh sobre outra superfície | 5/10 | Alta |
| Repeat on Path | distribuir asset sobre spline | 4/10 após Spline Core | Alta |
| Curve/Bend on Path | deformar segmento ao longo de spline | 5–6/10 | Média |
| Simple Decimate | redução controlada para LOD/cleanup | 6/10 | Média |

# Mirror

Promover o Mirror atual para implementação não destrutiva opcional. O usuário continua podendo executar `Mirror` como ação rápida, mas pode escolher `Keep Live` para adicioná-lo à stack.

# Array

Escopo inicial:

- Linear X/Y/Z;
- count;
- spacing absoluto ou relative bounds;
- optional merge-at-boundary somente quando seguro;
- Radial simples como evolução.

Não implementar array procedural arbitrário, fields ou expressions.

# Thickness

Equivalente conceitual a um Solidify simples: offset da superfície e criação de side walls. Deve ter `Thickness`, `Direction` e `Keep Rim`. Casos não-manifold devem gerar diagnóstico claro em vez de heurísticas silenciosas.

# Simple Deform

Em vez de quatro ferramentas separadas na UI, um único modifier `Deform` pode oferecer:

- Bend;
- Taper;
- Twist;
- Stretch.

Inputs públicos: mode, axis, amount, limits, origin e preserve-volume quando aplicável.

# Visual-first Deformer UX

<aside>
👁️

**Regra normativa:** Bend/Twist/Taper/Stretch não podem existir apenas como sliders. Ao selecionar o modifier, o viewport deve mostrar de forma didática a região capturada e a transformação esperada.

</aside>

## Deformer Cage / Capture Box

Uma cage ajustada aos bounds afetados representa a região de influência. Ela inclui spine/eixo central, origin, Start/End bounds e handles diretos. A cage pode ser movida/rotacionada quando o origin/orientation forem editáveis.

## Bend

Mostrar arco-guia dentro/ao redor da cage. Um handle sobre o arco controla Angle/Curvature. Start/End limitam onde a curva começa e termina. O usuário deve enxergar a trajetória antes de confirmar.

## Twist

Mostrar eixo central e handles circulares nas extremidades, com uma indicação helicoidal simplificada da progressão. O modo avançado pode expor Start Angle e End Angle.

## Stretch / Squash

Mostrar handles longitudinais nos extremos e preview da mudança de seção transversal. Se `Preserve Volume` estiver ativo, a cage deve deixar claro que alongar também estreita e comprimir também engrossa.

## Taper

Mostrar frames/seções no início e fim da cage; arrastar a seção final controla o taper. Midpoint/squish fica em Advanced.

## Outros modifiers

- Mirror: plane visível + ghost result + merge seam;
- Array: ghost copies + spacing handle + count label; radial mostra arco/eixo;
- Thickness: shell ghost + normal offset direction;
- Surface Conform: target surface highlight + offset guide.

## Visibilidade e acessibilidade

`Show Guides: Always / When Selected / Never`. Overlays usam tokens do design system, não valores hardcoded; eixos/bounds não dependem só de cor. Todo handle possui equivalente numérico, foco por teclado e descrição semântica.

A especificação implementation-ready vive em [P3D-157 — Modifier Stack & Visual Deformer UX](../specs/p3d-157-modifier-stack-visual-deformer-ux.md).

# Surface Conform

Modifier inspirado conceitualmente em shrinkwrap, porém limitado:

- nearest surface;
- project along normal/axis;
- offset;
- selected region/mask opcional posteriormente.

Útil para roupas simples, patches geométricos, placas aderidas, detalhes e adaptação de peças.

# UX simples

```
MODIFIERS
────────────────────
[ + Add Modifier ]

☰ Mirror          ✓
  Axis X
  [ Apply ]

☰ Array           ✓
  Count     4
  Spacing   1.5m
  [ Apply ]

☰ Thickness       ✓
  0.03m
  [ Apply ]
```

Advanced fica recolhido. O usuário não vê dezenas de parâmetros por padrão.

# Modifiers que NÃO entram inicialmente

- Subdivision Surface como centro do workflow;
- remesh;
- skin deformation como modifier genérico;
- volume/voxel modifiers;
- cloth/physics;
- boolean stack avançada;
- procedural displacement pesado;
- arbitrary Geometry Nodes modifier.

# Relação com ferramentas diretas

Uma ação direta pode opcionalmente gerar modifier live quando houver benefício:

```
Mirror → Apply now | Keep Live
Array  → Apply now | Keep Live
Bend   → Apply now | Keep Live
```

Bevel Core V1 permanece operação direta por enquanto; uma versão live só entra se houver demanda real.

# Referências externas

- Blender Modifier Stack: operações não destrutivas reordenáveis.
- Blender Simple Deform: Bend/Taper/Twist/Stretch, Axis, Origin e Limits.
- Houdini Bend: capture region/spine e handles diferentes para Bend, Twist, Length Scale e Taper — principal referência para a visualização didática simplificada do Petunia.
- Maya nonlinear deformers: handles posicionados/orientados a partir dos bounds, Low/High Bound e manipulação interativa.
- Solidify: espessura sobre superfícies.
- Shrinkwrap: projeção/conformação sobre superfície.

# Decisão

Criar um **Modifier Stack pequeno pós-V1**, começando por Mirror, Array, Thickness e Simple Deform. Isso fecha um gap importante sem aproximar o produto da complexidade de um DCC generalista.