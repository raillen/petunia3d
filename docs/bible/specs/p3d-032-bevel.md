# P3D-032 — Bevel

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Selection Domain, Transform & Modeling Core)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **implementado parcialmente; precisa correção e clareza** · Prioridade: P0.

</aside>

## Objetivo

Bevel/Chamfer low-poly previsível, com **1 segmento no Core V1** salvo decisão posterior.

## Correções

Definir explicitamente onde opera: vertex, edge e/ou face conforme implementação correta. UI não pode mostrar modos inexistentes. Integrar feedback modal P3D-131.

## Arquitetura

Bevel kernel independente da tool e do renderer; tool fornece amount/selection/mode. Validar topologia e limites antes de commit.

## Dependências

P3D-017–019, P3D-041, P3D-131.

## Testes / DoD

Boundary/non-manifold cases suportados ou rejeitados claramente, amount zero/limites, selection modes, cancel e undo.