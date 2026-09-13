# P3D-041 — Undo / Redo

<aside>
🧩

Estado: **precisa auditoria crítica** · Prioridade: P0.

</aside>

## Objetivo

Histórico confiável para todas as mutações user-facing relevantes.

## Auditoria obrigatória

Inventariar mutações de mesh, object, material, paint, project, inspector e tools; localizar caminhos que alteram estado diretamente e bypassam history.

## Contrato

- gestos contínuos = uma transação;
- cancel não entra no histórico;
- redo reproduz estado sem depender de widget;
- operações não-undoable são raras e documentadas.

## Arquitetura

History pertence à application/editor layer, com Commands/transactions ou mecanismo equivalente coerente.

## Testes / DoD

Sequências longas, branches após undo, delete/create, transform drag, topology edits e persistência do dirty state.