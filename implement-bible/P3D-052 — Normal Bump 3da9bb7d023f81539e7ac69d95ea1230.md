# P3D-052 — Normal / Bump

<aside>
🧩

Estado: **precisa auditoria; dependente do Material System** · Prioridade: P1.

</aside>

## Objetivo

Canal Normal/Bump simples para acabamento low-poly sem introduzir sculpt/remesh.

## Decisão

Distinguir internamente normal map de height/bump quando necessário; a UI pode simplificar conforme suporte real do renderer/exporter.

## Dependências

P3D-050, P3D-054, P3D-062.

## Testes / DoD

Sem mapa, normal válido/inválido, escala/intensidade se suportada, save/load, preview e export compatibility.