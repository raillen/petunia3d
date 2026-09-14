# P3D-040 — Sistema de Snap

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Selection Domain, Transform & Modeling Core)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **precisa auditoria** · Prioridade: P1.

</aside>

## Objetivo

Snap reutilizável para Grid/Increment/Vertex/Edge/Face conforme suporte real.

## Auditoria

Mapear targets, prioridade, tolerance, visual feedback e integração com Move/Rotate/Scale/Extrude. Identificar stubs.

## Arquitetura

`SnapQuery/SnapResult` neutros, consumidos por tools; input/UI apenas escolhe política. Evitar queries geométricas duplicadas em cada tool.

## Dependências

P3D-009, P3D-021–029.

## Testes / DoD

Cada target, conflito entre targets, zoom/DPI, cancel e ausência de jitter.