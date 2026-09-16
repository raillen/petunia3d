# P3D-085 — Temas customizados

<aside>
🧩

Estado: **precisa auditoria do loader/fallback** · Prioridade: P1.

</aside>

## Objetivo

Packs externos em `themes/<id>/{manifest.toml,theme.toml}` com schema version, validação e fallback.

## Regras

Carregar do diretório de usuário, offline, sem executar código. Paths são canonicalizados e confinados ao pack. Tema parcial é permitido quando tokens ausentes caem no default.

## UX

Settings mostra preview/nome/autor/versão e troca runtime sem restart quando viável.

## Dependências

P3D-084, P3D-089.

## Testes / DoD

Pack válido, token inválido, arquivo ausente, schema futuro, path traversal e fallback completo.