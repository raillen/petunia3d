# P3D-022 — Rotate

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Selection Domain, Transform & Modeling Core)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado reportado: **funcional; validar pivot/orientation** · Prioridade: P1.

</aside>

## Objetivo

Rotação consistente de objetos/componentes sem depender do gizmo concreto.

## Auditoria

Validar pivot, Global/Local, eixos, snapping angular se existir, transform inspector e comportamento em seleção múltipla.

## Arquitetura

Rotação recebe quaternion/axis-angle/delta neutro apropriado; UI não armazena regra de domínio.

## Dependências

P3D-025–027, P3D-041, P3D-131.

## Testes / DoD

Cancel/confirm, 360°, valores extremos, local/global, undo único e equivalência gizmo/command.