# P3D-054 — Displacement / Height

<aside>
🧩

Estado: **precisa auditoria; dependente do Material System** · Prioridade: P1.

</aside>

## Objetivo

Canal Height/Displacement voltado a textura/material, sem implicar remesh/sculpt.

## Escopo

Priorizar height/bump visual ou export de mapa. Geometrical displacement real só entra se houver implementação barata, previsível e coerente com low-poly.

## Dependências

P3D-050, P3D-052, P3D-062, P3D-127.

## Testes / DoD

Map loading, intensidade se suportada, fallback e garantia de que nenhum remesh é introduzido silenciosamente.