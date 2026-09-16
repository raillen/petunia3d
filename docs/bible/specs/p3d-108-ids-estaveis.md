# P3D-108 — IDs estáveis

<aside>
🧩

Estado: **precisa auditoria de índices/handles atuais** · Prioridade: P0.

</aside>

## Objetivo

ObjectId, MeshId, MaterialId, AssetId, TextureId, ReferenceId e futuros BoneId/AnimationAssetId estáveis o suficiente para undo, UI desacoplada, plugins e APIs.

## Auditoria

Localizar `usize`/Vec index/raw pointer/reference lifetime usados como identidade. Verificar invalidação após delete/reorder/merge.

## Regras

Display name/path não é identidade. IDs devem ter ownership/lifecycle claros e não ser reutilizados silenciosamente de forma perigosa.

## Dependências

P3D-001, P3D-020, P3D-102–107.

## Testes / DoD

Create/delete/recreate, save/load, undo/redo, missing references e APIs nunca observam objeto errado por index reuse.