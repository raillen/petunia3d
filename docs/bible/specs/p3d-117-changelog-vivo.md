# P3D-117 — Changelog vivo

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 9
- **Status Canônico**: `PLANEJADA (QA, Docs, Release & GA Candidate)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **precisa confirmar integração no repo/site** · Prioridade: P1.

</aside>

## Objetivo

Registrar mudanças user-facing úteis por versão/Unreleased sem transformar commits em release notes.

## Estrutura

Categorias Added/Changed/Improved/Fixed/Performance/Deprecated/Removed/Security/Documentation; fragments por PR são recomendados para reduzir conflitos.

## Regras

Breaking change em project/schema/plugin/keymap exige migration note. Mudança interna sem impacto não precisa poluir changelog público.

## Dependências

P3D-116, P3D-120.

## Testes / DoD

Uma fonte canônica, release aggregation quando adotada, links/version/date corretos.