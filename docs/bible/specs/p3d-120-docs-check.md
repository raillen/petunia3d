# P3D-120 — Docs Check

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 9
- **Status Canônico**: `COMPLIANT (Implementado e Verificado)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **implementado via quality gate permanente `cargo xtask docs-check`** · Prioridade: P1.

</aside>

## Objetivo

CI falha quando referências geradas estão stale, links/anchors/assets quebrados ou build do site falha.

## Pipeline

Generate temp → compare tracked generated docs → Markdown/build → link check → asset check. Mensagem de erro deve indicar comando de correção (`cargo xtask docs` ou equivalente).

## Dependências

P3D-116–119.

## Testes / DoD

Simular arquivo generated antigo, imagem faltante, link inválido e build quebrado; CI detecta cada classe sem falsos positivos excessivos.