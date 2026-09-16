# 40 — Pós-V1: Spline Core e Procedural Path Generators

<aside>
〰️

Splines são candidatas a uma das infraestruturas pós-V1 com maior capacidade de reaproveitamento. Um **Spline Core pequeno** pode alimentar Hair Guides, cabos, canos, cercas, paredes, corrimãos, trilhos, trims e vários generators sem exigir um sistema equivalente a Geometry Nodes.

</aside>

# Tese

O Petunia não precisa implementar um sistema procedural genérico como Geometry Nodes. Precisa de uma primitiva simples e previsível:

```
Path + Profile + Placement Rules + Parameters → Procedural Asset → Bake to Mesh
```

Essa primitiva é suficiente para uma grande família de assets usados em games.

# O que fica no Core

O **Spline Core** deve ser infraestrutura, não um plugin opcional, porque será reutilizado por vários módulos oficiais.

Capacidades mínimas:

- Polyline path;
- Bézier path;
- control points e handles simples;
- open/closed path;
- arc-length sampling;
- tangent evaluation;
- stable frame / orientation;
- per-point width/scale opcional;
- resampling por distância;
- snapping;
- attach/conform opcional a superfície;
- serialization no `.petunia`;
- Commands + Undo/Redo;
- convert/bake to mesh.

# Operadores genéricos

## Sweep Profile Along Path

Transforma um perfil 2D em geometria ao longo da spline.

Uso:

- fios;
- cabos;
- canos;
- mangueiras;
- trims;
- molduras;
- corrimãos;
- gutters.

## Repeat Asset Along Path

Distribui um asset repetido em espaçamento constante.

Uso:

- postes;
- estacas;
- cercas;
- lâmpadas;
- sleepers de trilho;
- munição em belt;
- barreiras.

## Stretch Segment Along Path

Deforma segmentos entre dois pontos da spline.

Uso:

- guardrails;
- rails;
- paredes modulares;
- segmentos de tubo especiais.

## End / Corner Rules

Permite assets específicos em início, fim e quinas:

- fence corner post;
- pipe elbow;
- cap;
- gate;
- junction.

# Generator Object

Generators devem permanecer procedurais até o usuário selecionar `Bake to Mesh`.

```rust
pub struct PathGenerator {
    pub path: SplineId,
    pub generator: GeneratorKind,
    pub parameters: GeneratorParameters,
    pub sources: Vec<AssetReference>,
}
```

# Geradores candidatos

| Generator | Fundação | Dificuldade | Valor |
| --- | --- | --- | --- |
| Cable / Wire | Sweep | 3/10 | Muito alto |
| Pipe / Hose | Sweep | 3–4/10 | Muito alto |
| Trim / Molding | Sweep | 4/10 | Alto |
| Fence | Repeat + rails | 4/10 | Muito alto |
| Simple Wall | Extrude/Sweep | 4/10 | Muito alto |
| Railing | Repeat + rails | 5/10 | Alto |
| Guardrail | Stretch + repeat | 5/10 | Alto |
| Chain / linked belt | Repeat oriented asset | 5/10 | Médio/Alto |
| Road curb | Stretch/Repeat | 5/10 | Alto |
| Road strip / path | Sweep surface | 6/10 | Alto |
| Rail track | Sweep + repeat | 6/10 | Alto |
| Pipe fittings/junctions | Corner rules | 6/10 | Médio |
| Barbed wire | Sweep + repeated detail | 6/10 | Médio |
| Terrain conform | Surface projection | 6/10 | Alto |

# Props além de personagens

O mesmo ecossistema abre workflows rápidos para:

- cercas de madeira/metal;
- paredes e muretas;
- tubos industriais;
- fios elétricos;
- cabos de poste;
- corrimãos;
- mangueiras;
- rodapés e molduras;
- estradas simples;
- calçadas/curbs;
- trilhos;
- barriers;
- vines estilizadas;
- neon tubes;
- cordas;
- correntes;
- cabos de ponte;
- gutters/downspouts;
- conveyor/belts simples.

# Relação com Hair Designer

Hair Guides e Path Generators devem reutilizar a mesma camada matemática de curves/sampling/frames.

```
petunia-spline-core
├── hair guides
├── cable generator
├── pipe generator
├── fence generator
├── wall generator
└── future community generators
```

Isso evita manter dois sistemas de curva independentes.

# Core vs módulo oficial vs plugin

## Core

- Spline data model;
- editing handles;
- sampling;
- frame/orientation;
- sweep/repeat primitives;
- surface attachment;
- bake to mesh;
- serialization;
- Commands/API.

## Módulos oficiais opcionais

- Hair Designer;
- Decal authoring tools avançados;
- Fence/Wall Generator;
- Cable/Pipe Generator;
- Character Morphs;
- Collision/LOD tools.

Esses módulos podem ser desligados da UI sem remover suporte ao formato de projeto.

## Community Plugins

- estilos específicos de cerca;
- geradores temáticos;
- arquiteturas especializadas;
- import/export de engines específicos;
- randomizers e presets de terceiros;
- pipelines de estúdios.

# Regra anti-bloat

**Pluginizar tudo não é desejável.** Uma feature deve ir para o Core quando várias outras features precisam dela para existir ou quando o formato de projeto/export precisa compreendê-la.

Uma feature deve ser módulo oficial quando:

- tem alto valor para o público-alvo;
- é relativamente grande na UI;
- não precisa estar visível para todos;
- reutiliza o Core sem criar arquitetura paralela.

Community plugin é preferível quando:

- é nicho;
- depende de workflow de uma engine/estúdio;
- adiciona presets ou geradores muito específicos;
- pode evoluir sem alterar invariantes do Petunia.

# Petunia Generator SDK — direção futura

A Extension API pode expor uma API declarativa limitada:

```
Generator
├── source path
├── one or more source assets/profiles
├── numeric parameters
├── placement rules
├── material slots
└── bake result
```

Plugins Lua não devem receber raw GPU/egui access; generators sofisticados de geometria podem inicialmente permanecer módulos Rust oficiais, enquanto plugins Lua coordenam operações e parâmetros aprovados pela API.

# Evidência externa

Unity mantém um pacote dedicado de Splines para paths, object generation e trajectories. Unreal possui Spline Mesh Components capazes de deformar meshes ao longo de splines. Godot permite extrudar CSGPolygon3D ao longo de Path3D. O ecossistema Blender possui vários generators comerciais/populares de fence, cable, pipe e wall baseados justamente em curves/Geometry Nodes.

# Referências externas de pesquisa

- [Unity Splines](https://docs.unity3d.com/6000.0/Documentation/Manual/com.unity.splines.html)
- [Unreal Spline Mesh Component](https://dev.epicgames.com/documentation/unreal-engine/API/Runtime/Engine/USplineMeshComponent)
- [Godot CSGPolygon3D / Path](https://docs.godotengine.org/en/stable/tutorials/3d/csg_tools.html)
- [Blender Curve Properties](https://docs.blender.org/manual/en/latest/modeling/curves/properties/index.html)
- [BagaPie — Cable, Fence, Array Along Shape etc.](https://extensions.blender.org/add-ons/bagapie/)

# Primeira ordem recomendada

1. Spline Core.
2. Cable / Pipe Generator.
3. Fence / Wall Generator.
4. Hair Guides migram/reutilizam o mesmo Core.
5. Generator SDK limitado.
6. Community generators.

# Decisão preliminar

**Spline Core deve ser infraestrutura oficial pós-V1; generators específicos devem ser módulos oficiais opcionais ou plugins.** Esse desenho maximiza reaproveitamento sem transformar o Petunia em um Geometry Nodes generalista.