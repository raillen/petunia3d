# 43 — Auditoria Pós-V1: Novos Gaps, Anti-Bloat e Asset Authoring Boundary

<aside>
🔎

Segunda auditoria de gaps após Hair, Morphs, Decals, Splines e pesquisa externa. O objetivo é identificar lacunas de alto valor sem deslocar o Petunia para scene editor, map editor ou DCC universal.

</aside>

# Achado principal

O maior gap novo é a ausência de um **contrato de edição não destrutiva de geometria**. Mirror existe, Array foi adiado e não havia Modifier Stack consolidado. Isso gera risco de implementar Hair/Spline/Generators com mecanismos ad hoc diferentes.

# Gaps P0 pós-V1

1. **Modifier Stack contract** — representação, ordem, cache, apply/bake, Undo e export.
2. **Surface Attachment contract compartilhado** — necessário para decals, hair roots, conform, accessories e projected details.
3. **Parametric Asset Properties** — mecanismo comum para expor `mouth`, `eyes`, `damage_state`, generator parameters, modifier inputs e material variants sem schemas paralelos.
4. **Bake/Flatten contract** — transformar decals, modifiers, generators e recipes em mesh/textures portáveis de maneira consistente.
5. **Asset State/Variant composition** — não implementar ainda como feature grande, mas definir compatibilidade entre Material Variant + Decal Set + Morph + visibility.
6. **Dependency graph / invalidation** — mudanças em Base Mesh, UV ou Material precisam invalidar corretamente decals, generators, bakes e caches derivados.

# Gaps P1 de UX

- Surface Manipulator compartilhado entre Decal, Conform e possivelmente Hair root placement.
- Drag-and-drop semantics por contexto: textura sobre viewport ≠ material sobre object ≠ decal sobre surface.
- Inspector de dependências: indicar quando aplicar um modifier pode invalidar UV/morph/decal attachment.
- `Apply / Keep Live / Bake` como vocabulário uniforme.
- Preview de custo: tris after modifiers/generators, decal overdraw e texture budget.
- Asset readiness deveria validar dados derivados também, não somente base mesh.

# Gaps de modelagem identificados

| Gap | Valor | Dificuldade | Decisão |
| --- | --- | --- | --- |
| Modifier Stack | Muito alto | 5–6/10 infra | Adicionar pós-V1 |
| Array simples | Muito alto | 3–4/10 | Adicionar |
| Thickness/Solidify | Muito alto | 4–5/10 | Adicionar |
| Simple Deform | Alto | 3–4/10 | Adicionar |
| Surface Conform | Alto | 5/10 | Adicionar após attachment foundation |
| Radial duplication | Alto | 3–4/10 | Array evolution |
| Instance/reference duplication | Alto | 4/10 | Estudar; útil para modular props |
| Simple pivot/hinge workflow | Muito alto | 2–3/10 | Quick win |

# Gaps de Surface/Paint

- Decal Tool/manipulator explícito;
- Projection/Stencil tool além do brush;
- Fill by Face/Material/Island;
- Clone/patch simples;
- Gradient tool;
- Line/Shape paint tool;
- Path Stroke reutilizando Spline Core posteriormente;
- channel-aware decal/effect stack;
- mask visualization/solo;
- flatten/bake preview.

A lista é inspirada em ferramentas comuns de texturing 3D, mas deve ser adicionada incrementalmente. O Paint não precisa copiar Substance Painter.

# Gaps de Nodes

P3D-113 é correto, mas subespecificado. Faltavam:

- node taxonomy;
- recipe/group asset semantics;
- cycle handling detalhado;
- exposed parameters;
- graph versioning;
- caching/invalidation;
- distinction Surface Recipe vs Generator Recipe;
- regra explícita contra arbitrary topology graph.

# Gaps de game-ready asset authoring

- hard-edge / smoothing guidance para normal baking;
- explicit tangent/normal preview quando normal maps entrarem;
- bake cage/basic high→low workflow pode ser futuro, mas não deve contaminar V1;
- texture size/budget presets por target platform;
- material slot count warning;
- draw-call estimate simples por asset;
- alpha/overdraw warnings para hair cards/decals;
- naming/export conventions per engine profile;
- origin/pivot validator;
- bounding dimensions target validator.

# Boundary: não virar editor de mapas

Petunia pode criar **assets que um map editor usa**, mas não deve montar o mapa.

## Permitido

- fence asset/generator;
- wall segment generator;
- road/rail mesh asset along local path;
- sockets;
- colliders;
- LODs;
- variants;
- decals pertencentes ao asset;
- local procedural arrangement dentro de um asset.

## Fora

- terrain/world sculpt;
- placement de milhares de scene objects;
- navmesh;
- level streaming;
- lighting de cena;
- gameplay triggers;
- spawn points de level;
- world foliage painting;
- scene hierarchy de jogo;
- runtime simulation;
- quest/gameplay authoring.

Teste simples: **se o resultado é um asset exportável isoladamente, provavelmente cabe; se o resultado é um level/world, não cabe.**

# Nova regra anti-bloat

Cada feature pós-V1 deve declarar `Scope Unit`:

- `Asset-local` → candidato Petunia;
- `Project-library` → candidato Petunia;
- `Scene/World` → outro produto/editor;
- `Runtime/Game Logic` → game engine.

# Ordem recomendada revisada

```
GA Hardening
→ Game Ready Inspector / quick wins
→ Decals + Surface Manipulator
→ Modifier Stack foundation
→ Mirror/Array/Thickness/Simple Deform
→ Spline Core
→ Cable/Pipe + Fence/Wall modules
→ Morphs
→ Hair
→ Surface Recipes / simple nodes
→ advanced parametric/animated decals
```

# Decisão

Adicionar Modifier Stack e Surface Attachment como fundações pós-V1 explícitas. Manter nodes restritos a recipes. Formalizar o limite asset-local para impedir deriva para map editor.

# Gap adicional — Parts Hierarchy e Linked Instances

A documentação possui Outliner/Parts, parent transforms na convenção espacial e hierarquia específica de Skeleton, porém a auditoria não encontrou um contrato de produto igualmente explícito para **parent/child entre Parts comuns** e **linked instances**.

Isso deve ser especificado antes de generators/modular assets avançarem, mas sem virar scene graph de level.

## Parts Hierarchy asset-local

Casos úteis:

- porta + maçaneta;
- arma + carregador + mira;
- veículo + rodas;
- personagem + acessórios;
- máquina com peças articuláveis.

Contrato mínimo a pesquisar:

- Parent Part / Child Part;
- local transform relativo ao parent;
- preserve world transform on reparent;
- visibility/lock inheritance claramente definida;
- export de hierarchy quando o formato suportar;
- restrição ao asset/documento atual.

## Linked Instance

Duplicação que referencia a mesma geometria/material source até `Make Unique`.

Usos:

- parafusos;
- rodas;
- dentes;
- partes repetidas;
- segmentos modulares dentro de um asset.

Dificuldade estimada: **4–5/10** incluindo persistência, selection e export/bake. Prioridade P1 pós-V1, especialmente útil junto a Array e generators.

Guardrail: isso é **asset assembly**, não placement de props em um mapa. Scene/world instancing pertence a outro produto/editor.