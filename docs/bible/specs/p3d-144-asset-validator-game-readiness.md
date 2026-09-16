# P3D-144 — Asset Validator & Game Readiness

<aside>
✅

Estado: **pós-V1 aprovado** · Função: transformar requisitos técnicos de game assets em feedback legível para iniciantes e checks reproduzíveis por export/batch/API.

</aside>

# Objetivo

Validar um asset antes de export/entrega, usando regras configuráveis e perfis de budget. O Validator deve funcionar também como **Game Ready Inspector**: não apenas dizer que algo está errado, mas explicar o problema no contexto do asset e oferecer `Locate`, `Explain` e, quando seguro, `Fix`.

# Princípio de UX

```
GAME READY
────────────────────────
✓ Scale
✓ Origin
✓ Normals
✓ UV overlap
! Texel density
✓ Materials
! Collider missing
✓ Naming
✓ Polygon budget
✓ Texture sizes
✓ LOD0
! No LOD1
✓ Export profile

Readiness: 82%
```

O score é informativo; erros bloqueadores continuam explícitos e não devem ser mascarados por média.

# Categorias de validação

## Geometry

- manifold/non-manifold;
- degenerate triangles/faces;
- duplicate/coincident faces;
- flipped/inconsistent normals;
- zero-area UV/geometry quando detectável;
- unexpected negative/non-uniform scale conforme profile;
- triangle/vertex budget;
- evaluated geometry budget **após modifiers/generators**, não apenas base mesh.

## Transform / dimensions

- pivot/origin;
- bounding dimensions;
- target scale/unit profile;
- optional Game Scale Guide comparison;
- hinge/pivot hints para doors/wheels quando metadata existir.

## UV / textures

- UV presence;
- overlaps quando proibidos;
- bounds/packing warnings;
- texel/pixel density target;
- texture dimensions e max budget;
- channel presence conforme material;
- alpha usage.

## Materials / rendering cost

- material-slot count;
- estimated draw-call contribution por asset quando calculável;
- texture memory estimate simples;
- alpha/overdraw warning para Decals e Hair Cards;
- normal/tangent requirements quando normal maps forem usados.

## Game metadata

- collision presence/type;
- sockets requeridos;
- LOD chain;
- naming rules;
- export-profile compatibility;
- Parts hierarchy/reference validity;
- broken Parametric Property bindings.

## Derived authoring data

- invalid Surface Attachments;
- stale Decals após topology change;
- invalid Modifier targets;
- stale generator/path dependencies;
- Morph topology mismatch;
- missing source for Linked Instance;
- pending Bake/Flatten requirements do export profile.

# Perfis

Generic, PS1-like, N64-like, PS2-like, Mobile e Custom. Esses nomes representam **budgets configuráveis**, não emulação histórica rígida. Futuramente export adapters podem fornecer engine profiles sem misturar lógica de engine com Geometry Core.

# Severity

- `Info` — contexto/otimização opcional;
- `Warning` — asset funciona, mas há risco/custo;
- `Error` — resultado provavelmente incorreto;
- `Blocking` — export profile exige correção.

# Ações

`Locate` seleciona/realça a origem do problema. `Explain` abre explicação curta e neurodivergent-friendly com consequência concreta. `Fix` só existe quando a transformação é determinística, segura, previewable e undoable.

Nunca oferecer auto-fix silencioso para topology, UV remapping ou attachment reprojection ambígua.

# Arquitetura

Validator opera em dados neutros e não em widgets. Regras devem ser registráveis e reutilizáveis por exportadores, Batch Processor, MCP e futura engine bridge.

```rust
pub trait AssetRule {
    fn evaluate(&self, snapshot: &AssetSnapshot, profile: &ReadinessProfile) -> Vec<Diagnostic>;
}
```

Diagnostics possuem stable code, severity, target reference, explanation key, optional locate descriptor e optional safe-fix command.

# Performance

Validação incremental por revision quando possível. Checks pesados rodam em jobs canceláveis sobre snapshots imutáveis. A UI deve permitir `Quick Check` e `Full Check` se profiling justificar a separação.

# Batch

Project Model Library pode validar vários assets e ordenar por blocking/error count. Export em lote reutiliza exatamente as mesmas regras.

# Testes / DoD

- fixtures válidas/inválidas por regra;
- stable diagnostic codes;
- profile-specific thresholds;
- evaluated modifiers/generators considerados;
- broken derived dependencies detectadas;
- Locate aponta target correto;
- safe Fix é undoable e idempotente quando aplicável;
- batch e single-asset produzem diagnósticos equivalentes;
- no false pass após topology/material/UV revision.

# Non-goals

Não virar profiler de engine, scene analyzer, navmesh checker ou level validator. O escopo permanece **asset-local / project-library**.