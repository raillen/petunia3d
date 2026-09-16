# 42 — Pós-V1: Nodes Simples, Recipes e Procedural Surface

<aside>
🔗

Petunia pode ter nodes, mas eles devem servir para **compor operações simples já existentes**, não introduzir uma linguagem visual geral. O graph deve esconder complexidade, não criá-la.

</aside>

# Diagnóstico atual

A documentação já possui `P3D-113 — Texturing Nodes simples`, corretamente restrito a textura/filtros. A lacuna é definir **que tipos de graphs são permitidos**, qual relação possuem com Modifier Stack/Recipes e como impedir expansão para Geometry Nodes completo.

# Três superfícies, não um graph universal

## 1. Surface Recipe Graph — recomendado

Graph pequeno para textura/material/masks.

```
Texture
  ↓
Tint
  ↓
Noise Mask
  ↓
Wear
  ↓
Base Color Output
```

## 2. Generator Recipe — futuro controlado

Não é Geometry Nodes. Combina operadores de alto nível já existentes:

```
Spline
  ↓
Repeat Asset
  ↓
Random Rotate
  ↓
Bake Mesh
```

A biblioteca de nodes é minúscula e baseada em Commands/Core aprovados.

## 3. Modifier Recipe — opcional posterior

Composição visual de modifiers já existentes, equivalente a salvar uma stack como preset reutilizável. Não permite manipulação arbitrária por vértice.

# O que NÃO criar

- fields por face/edge/point;
- topology scripting arbitrário;
- loops visuais;
- simulation zones;
- arbitrary attributes;
- custom geometry kernels dentro do graph;
- gameplay scripting;
- compositor;
- shader graph geral;
- world/scene graph.

# Nodes de Texturing recomendados

| Categoria | Nodes | Dificuldade |
| --- | --- | --- |
| Input | Texture, Color, Value, UV, Vertex Color | 2–3/10 |
| Color | Tint, Brightness/Contrast, Hue/Saturation, Invert | 2–3/10 |
| Mix | Mix, Multiply, Add, Overlay simples | 2–3/10 |
| Mask | Threshold, Levels, Mask Input | 3/10 |
| Procedural | Noise, Checker, Gradient, Voronoi simples opcional | 3–4/10 |
| Geometry Maps | AO, Curvature, Normal Direction, Position | 4–5/10 + baking |
| Effects | Edge Wear, Dirt, Color Variation | 4–5/10 |
| Output | Base Color, Roughness, Normal, Height, Mask | 2–3/10 |

# Filosofia de presets antes de graphs

Usuário comum deveria primeiro enxergar:

```
[ Add Effect ]
  Dirt
  Edge Wear
  Color Variation
  Pixel Noise
  Gradient
```

Cada efeito pode internamente ser um node group/recipe, mas o graph só aparece em `Advanced > Edit Recipe`.

Isso segue a ideia de node groups: esconder um conjunto de operações atrás de uma interface pequena e parametrizável.

# Generator Recipe nodes — conjunto máximo inicial

- Path Input;
- Profile Input;
- Asset Input;
- Sweep;
- Repeat Along Path;
- Stretch Along Path;
- Random Rotate;
- Random Scale;
- Alternate Asset;
- Material Assign;
- Bake Output.

Não permitir acesso genérico a vertices/faces.

# Recipe Assets

Um graph pode ser salvo como asset/preset:

```
Rusty Metal
Stylized Wood
Anime Skin
Sci-Fi Cable
Wood Fence
Pipe Run
```

O usuário arrasta o recipe e vê apenas parâmetros expostos.

# Relação com Modifier Stack

Modifier Stack é linear e deve resolver 80–90% dos casos de geometria não destrutiva. Nodes só entram quando **ramificação/composição visual** realmente oferece benefício.

```
Normal user: Modifier Stack
Advanced user: Recipe Graph
Plugin author: declarative Generator API
```

# Implementação

Graph model independente do widget visual:

- typed node IDs;
- typed sockets;
- DAG somente;
- cycle rejection;
- deterministic evaluator;
- memoization/cache;
- serialized graph schema version;
- explicit errors;
- preview per node opcional apenas para textures.

# Relação com plugins

Community plugins podem registrar novos **Recipe Assets** e, futuramente, nodes de categorias aprovadas. Não expor raw Rust, raw GPU ou arbitrary geometry mutation via node runtime.

# Referências externas

Blender demonstra que Node Groups são úteis para esconder complexidade e expor somente parâmetros; Geometry Nodes também permite transformar node groups em Tools/Assets. Substance Painter demonstra abordagem mais simples para usuários: generators/filters aparecem como efeitos parametrizados em layers, sem exigir graph editing na rotina comum.

# Decisão

Manter P3D-113 e evoluí-lo para **Surface Recipes**. Generator Nodes ficam posteriores ao Spline Core e Modifier Stack. O Petunia nunca apresenta nodes como workflow obrigatório.