# P3D-046 — Outliner

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 6
- **Status Canônico**: `COMPLIANT (Scene, Assets, Outliner & Inspector)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **precisa auditoria / provável parcial** · Prioridade: P1.

</aside>

## Objetivo

Hierarquia clara de objetos, coleções e referências com seleção central sincronizada.

## Auditoria

Mapear tree model, expand/collapse, search, selection, context menu, DnD e qualquer estado duplicado.

## Contrato UX

Rows com icon type, nome, visibility/lock quando aplicável, hover/selected states e indentation consistente. Não colorir categorias aleatoriamente sem semântica.

## Arquitetura

Tree widget é projection do Scene/Editor state. Selection e operações usam ObjectId/ReferenceId e Commands.

## Dependências

P3D-020, P3D-047, P3D-077, P3D-082.

## Testes / DoD

Seleção bidirecional, rename/delete, referências, search, expand/collapse e context actions.