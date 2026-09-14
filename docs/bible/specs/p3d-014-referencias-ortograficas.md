# P3D-014 — Referências ortográficas

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 4
- **Status Canônico**: `COMPLIANT (Viewport, Navigation & Reference Workflow)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado inicial: **parcial, deve ser integrado ao Reference Set Manager** · Prioridade: P1.

</aside>

## Objetivo

Associar referências às vistas ortográficas corretas e permitir modelar sobre elas com alinhamento previsível.

## Contrato

- Front/Back/Left/Right/Top/Bottom consultam o Reference Set do projeto.
- Cada vista pode ocultar/mostrar sua referência e respeita opacity/lock/transform.
- Trocar de vista não modifica a geometria nem o transform da referência.
- Perspective pode mostrar referências apenas quando a política definida permitir, sem poluir o viewport.

## Arquitetura

Câmera/vista e referência são subsistemas separados ligados por IDs/roles. Não armazenar referência dentro do widget de viewport.

## Dependências

P3D-006 e P3D-013.

## Testes / DoD

Todas as seis vistas, ausência parcial de imagens, transforms persistidos, zoom/frame e mudanças de resolução sem drift de alinhamento.