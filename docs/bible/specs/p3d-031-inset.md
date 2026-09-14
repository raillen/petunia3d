# P3D-031 — Inset

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Selection Domain, Transform & Modeling Core)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado reportado: **funcional; falta feedback modal** · Prioridade: P1.

</aside>

## Objetivo

Inset previsível de faces/regiões selecionadas com preview interativo.

## UX

Reutilizar P3D-131 para guia visual, valor/delta, confirm/cancel e status hints. Tool Properties pode conter modo/offset somente se já houver semântica implementada.

## Arquitetura

Operação geométrica headless; preview não deve duplicar mesh permanentemente a cada frame de forma cara.

## Dependências

P3D-019, P3D-041, P3D-131.

## Testes / DoD

Faces convexas/côncavas suportadas, regiões, limites inválidos, cancel e undo.