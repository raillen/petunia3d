# P3D-119 — Referência automática de tokens

<aside>
🧩

Estado: **a verificar/implementar** · Prioridade: P1.

</aside>

## Objetivo

Gerar catálogos de Commands, keybinds, IconIds, TextIds, ThemeTokens e formatos suportados a partir das fontes canônicas de código/metadata.

## Regra

Arquivos `docs/generated` têm aviso de generated e não são editados manualmente. Gerador deve ser determinístico.

## Integração

`cargo xtask docs`/equivalente produz referências; P3D-120 detecta drift.

## Dependências

P3D-084–100, P3D-116.

## Testes / DoD

Run twice = same output, todos IDs válidos, links/docs paths corretos e generated manifest opcional se útil.