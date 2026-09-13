# P3D-017 — Vertex Select

<aside>
🧩

Estado reportado: **implementado e funcional** · Trabalho: validar/regredir · Prioridade: P1.

</aside>

## Objetivo

Seleção e manipulação de vértices dentro do modelo unificado de seleção.

## Auditoria

Confirmar click, box selection quando existir, X-Ray, multi-select, visibility e sincronização com tools/undo. Adaptar ao P3D-015 sem reimplementar algoritmo correto.

## Arquitetura

Selection state usa IDs/handles estáveis da topologia, não estado do widget. Commands/tools consultam a seleção central.

## Integrações

P3D-015, P3D-020, P3D-011, P3D-021–023.

## Testes / DoD

Seleção única/múltipla, vértices sobrepostos, X-Ray, alteração topológica e seleção inválida após delete/merge.