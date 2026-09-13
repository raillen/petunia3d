# P3D-040 — Sistema de Snap

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