# P3D-059 — Fill

<aside>
🧩

Estado: **implementação incerta; auditar** · Prioridade: P1.

</aside>

## Objetivo

Preenchimento útil para pixel/texture workflow sem algoritmo excessivamente pesado.

## Escopo

Começar por fill da área/ilha/seleção conforme representation disponível. Flood fill por similaridade só se trouxer benefício claro e for determinístico.

## Dependências

P3D-055, P3D-061, P3D-064, P3D-132.

## Testes / DoD

Boundaries, UV islands, mask, selection isolation, undo e grandes áreas sem travar UI.