# P3D-018 — Edge Select

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Selection Domain, Transform & Modeling Core)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


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