# P3D-012 — Shading Modes

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 4
- **Status Canônico**: `COMPLIANT (Viewport, Navigation & Reference Workflow)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado inicial: **implementado e funcional; validar/polir** · Prioridade: P2.

</aside>

## Objetivo

Preservar Wireframe, Solid, Material Preview e modo equivalente a Rendered quando realmente suportado.

## Auditoria

Validar que os quatro estados correspondem a comportamento real do renderer, não apenas ícones. Verificar transições, materiais ausentes e performance.

## UX

Segmented control com IconIds claros e tooltip; estado ativo inequívoco. Não usar círculos/quadrados genéricos sem semântica.

## Dependências

P3D-050, P3D-088, P3D-105.

## Testes / DoD

Troca de shading não quebra seleção/overlays, Material Preview usa o mesmo Material System e documentação/screenshots refletem os modos reais.