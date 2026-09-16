# P3D-089 — Sistema completo de traduções

<aside>
🧩

Estado: **precisa auditoria de strings/fallback** · Prioridade: P1.

</aside>

## Objetivo

Toda string visível via `TextId`, com packs TOML externos, fallback e validação.

## Regras

Não exibir labels bilíngues como `Posição (Location)` na UI final. Locale escolhido governa o texto. IDs técnicos permanecem estáveis e não traduzidos.

## Validação

Missing/extra/obsolete keys, placeholders, locale metadata, coverage e malformed TOML. Fallback: selected locale → canonical locale → key/diagnostic sem crash.

## Dependências

P3D-084, P3D-114–115.

## Testes / DoD

Hot/runtime switch quando suportado, strings longas, placeholders, menus/tooltips e nenhum hardcode user-facing novo.