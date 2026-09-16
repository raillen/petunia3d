# P3D-168 — Procedural Path Generators

<aside>
🏗️

Estado: **pós-V1 aprovado após P3D-161**. Geradores oficiais transformam Path + Profile/Asset + regras simples em um **asset local**, nunca em conteúdo de level/world.

</aside>

# Objetivo

Entregar alto ganho de produtividade para props e environment assets repetitivos sem introduzir Geometry Nodes genérico.

# Primitives compartilhadas

`Sweep Profile Along Path`, `Repeat Asset Along Path`, `Stretch Segment Along Path` e `End/Corner Rules` vêm da application layer sobre o Spline Core.

# Módulos iniciais

## Cable / Wire

Circle/low-poly profile, radius, sides, material, optional sag manual por control points. Dificuldade 3/10 após Spline Core.

## Pipe / Hose

Profile + sweep, caps, optional elbow/corner handling limitado. Dificuldade 3–4/10.

## Fence

Post asset + spacing + rails + corner/end posts + optional gate slot. Dificuldade 4–5/10.

## Wall

Path + height + thickness + segment length, optional corner behavior e material slots. Dificuldade 4–5/10.

## Railing / Guardrail

Repeat posts + stretch rails; evolução após Fence. Dificuldade 5/10.

# UX

```
FENCE GENERATOR
Path        FencePath
Post        Wood_Post_01
Spacing     1.50 m
Height      1.20 m
Rails       2
Variation   0.10

[✓] Follow Path
[ ] Conform to Surface

[ Bake to Parts ]
```

Viewport mostra path, ghost positions e contagem prevista antes do bake.

# Randomização controlada

Seed determinístico; small rotate/scale/offset ranges; alternate assets. Sem scatter em world space.

# Surface conform

Somente após P3D-158. Serve para o asset seguir uma superfície local de referência; não é terrain/world placement system.

# Budget

Mostrar estimated Parts/instances, triangles após bake e warnings quando o gerador exceder budgets configurados.

# Plugins

Community plugins podem registrar presets/geradores declarativos sobre primitives aprovadas. Raw topology scripting permanece fora.

# Boundary anti-map-editor

Permitido: gerar uma cerca completa como asset exportável, road curb mesh, trilho local, cabo, pipe run. Fora: desenhar estradas no mapa, espalhar assets pelo level, terrain, navmesh, gameplay/lighting.

# Testes / DoD

Deterministic seed, open/closed paths, sharp corners, endpoints, material assignment, live preview, bake equivalence, save/load, Undo e budget diagnostics.