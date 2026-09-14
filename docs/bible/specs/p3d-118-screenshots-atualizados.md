# P3D-118 — Screenshots atualizados

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 9
- **Status Canônico**: `PLANEJADA (QA, Docs, Release & GA Candidate)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **processo precisa ser institucionalizado** · Prioridade: P1.

</aside>

## Objetivo

Screenshots reais e atuais do app para manual/releases, organizados semanticamente.

## Política

Mockup nunca é apresentado como UI implementada. Definir configuração oficial de captura (theme/icon pack/language/resolution) e atualizar quando controles/layout mudam.

## Automação futura

`cargo xtask docs-screenshots` pode criar estados determinísticos, mas não bloquear docs iniciais.

## Dependências

P3D-116, P3D-121.

## Testes / DoD

Todos assets referenciados existem, before/after quando útil e docs não contêm imagens claramente obsoletas.