# P3D-023 — Scale

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

Escala uniforme e por eixo/plano em Object/Vertex/Edge/Face.

## Auditoria

Validar pivot, local/global, uniform scale, valores próximos de zero/negativos conforme política e integração com Inspector.

## Arquitetura

Gizmo/fields apenas produzem parâmetros da mesma operação central; evitar divergência entre drag e valores numéricos.

## Dependências

P3D-025–027, P3D-041.

## Testes / DoD

Uniform/axis/plane constraints, múltiplos componentes, cancelamento e undo transacional.