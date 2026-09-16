# P3D-018 — Edge Select

<aside>
🧩

Estado reportado: **implementado e funcional** · Prioridade: P1.

</aside>

## Objetivo

Selecionar/manipular arestas de maneira previsível no modelo unificado.

## Auditoria

Validar hit-testing, multi-select, X-Ray, integração com Bevel/Loop Cut/Merge/Split e invalidação após mudanças de topologia.

## Arquitetura

A seleção de edge é estado central, reutilizável por UI, tools, scripts e testes headless.

## Dependências

P3D-015, P3D-020, P3D-032, P3D-034.

## Testes / DoD

Arestas compartilhadas, boundary edges, hidden geometry e operações destrutivas sem IDs pendentes.