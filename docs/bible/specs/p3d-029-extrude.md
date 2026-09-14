# P3D-029 — Extrude

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

Extrudar faces/arestas conforme contexto com restrição por eixo e feedback claro durante o gesto.

## UX aprovado

Integrar P3D-131: linha/guia discreta da origem ao cursor, delta numérico quando útil, axis lock, confirm/cancel e hints na status bar. O feedback não deve cobrir a geometria.

## Arquitetura

Extrude core é operação topológica headless; Tool apenas mantém estado modal e preview. Não acoplar cálculo ao mouse egui.

## Dependências

P3D-025, P3D-041, P3D-131.

## Testes / DoD

Face/edge cases suportados, múltiplas seleções válidas, axis constraints, cancel sem alteração, undo único e topologia válida.