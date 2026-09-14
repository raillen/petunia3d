# P3D-024 — Universal Transform

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Selection Domain, Transform & Modeling Core)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **parcial / precisa correção** · Prioridade: P1.

</aside>

## Objetivo

Gizmo combinado Move/Rotate/Scale útil sem sacrificar precisão ou hit-testing.

## Auditoria

Revisar handles, prioridade de picking, overlap visual, active operation, transform state e cancelamento. Não duplicar matemática de P3D-021–023.

## UX

Ícones/handles claros, estados hover/active e possibilidade de voltar a gizmos individuais. Não sobrecarregar viewport em telas pequenas.

## Dependências

P3D-021–023, P3D-084.

## Testes / DoD

Cada handle executa a operação correta, nenhum handle “rouba” interação indevida e o gizmo respeita pivot/orientation.