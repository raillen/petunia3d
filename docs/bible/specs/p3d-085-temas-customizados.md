# P3D-085 — Temas customizados

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 3
- **Status Canônico**: `COMPLIANT (UI Infrastructure, Customization & Input)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


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