# P3D-026 — Transform Orientation

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Selection Domain, Transform & Modeling Core)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **implementação precisa correção** · Prioridade: P0.

</aside>

## Objetivo

Global, Local e somente outras orientações realmente suportadas devem alterar transformações de modo matematicamente correto.

## Auditoria

Traçar onde orientation é armazenada, quem a lê e se gizmos/commands/Inspector usam a mesma fonte.

## Regras

Não expor Normal/View/Cursor apenas por semelhança com Blender se o core não as suporta corretamente. Dropdown é UI sobre enum/descriptor semântico.

## Dependências

P3D-021–025, P3D-027.

## Testes / DoD

Object e component selection, objetos rotacionados, múltiplas orientações e consistência entre gizmo e modal tools.