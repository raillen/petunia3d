# P3D-027 — Pivot System

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Selection Domain, Transform & Modeling Core)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **implementação precisa correção** · Prioridade: P0.

</aside>

## Objetivo

Definir pivô de transformação previsível para objeto/componentes e contexto da cena.

## Auditoria

Mapear pivôs atuais, median/center/origin/cursor equivalentes e divergências entre Move/Rotate/Scale.

## Arquitetura

Pivot policy é estado semântico do editor. UI mostra selector; transform core recebe pivot calculado de forma determinística.

## Dependências

P3D-021–026, P3D-049.

## Testes / DoD

Seleção simples/múltipla, origem do objeto, median/center aprovados, pivot após delete/merge e persistência apenas quando apropriada.