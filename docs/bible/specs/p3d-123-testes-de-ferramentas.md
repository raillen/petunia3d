# P3D-123 — Testes de ferramentas

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Testes de Ferramentas de Modelagem)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Parte de cada tool, não backlog de fim de projeto · Prioridade: P1.

</aside>

## Objetivo

Cobrir tools de modelagem/transform com testes de algoritmo, lifecycle, invalid input e undo.

## Matriz

Select/Move/Rotate/Scale/Extrude/Inset/Bevel/Knife/Loop Cut/Subdivide/Merge/Split/Mirror/Snap conforme implementados.

## Regra

Preferir testes headless de core/application; UI tests cobrem integração, não repetem toda matemática.

## Dependências

P3D-021–041, P3D-101, P3D-131.

## DoD

Cada tool estabilizada tem happy path, edge cases, cancel/undo e regression tests de bugs corrigidos.