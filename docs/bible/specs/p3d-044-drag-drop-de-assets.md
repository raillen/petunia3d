# P3D-044 — Drag & Drop de Assets

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 6
- **Status Canônico**: `COMPLIANT (Scene, Assets, Outliner & Inspector)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **precisa auditoria** · Prioridade: P1.

</aside>

## Objetivo

Arrastar assets para viewport ou contextos compatíveis com feedback de target válido/inválido.

## Auditoria

Verificar DnD atual, ownership do payload, criação de ObjectId e integração com undo.

## Contrato

Payload carrega AssetId/descriptor neutro; UI decide target e application executa command de instanciar/aplicar. Não transportar ponteiro de widget/texture handle como identidade do asset.

## Dependências

P3D-003, P3D-042, P3D-020, P3D-041.

## Testes / DoD

Drop válido/inválido, cancel, asset missing, multi-window/panel quando aplicável e undo.