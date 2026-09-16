# P3D-036 — Merge

<aside>
🧩

Estado: **precisa auditoria de correção/funcionalidade** · Prioridade: P1.

</aside>

## Objetivo

Unir componentes selecionados com semântica previsível.

## Auditoria

Descobrir quais variantes existem (at center, first/last, by distance etc.) e quais realmente funcionam. Não copiar opções do Blender sem necessidade.

## Arquitetura

Merge opera em topology IDs, atualiza/remapeia selection e attributes, integra undo e retorna erro estruturado para seleção inválida.

## Dependências

P3D-017–019, P3D-041.

## Testes / DoD

Seleções mínimas, duplicatas, boundaries, remapeamento de IDs/UV e undo.