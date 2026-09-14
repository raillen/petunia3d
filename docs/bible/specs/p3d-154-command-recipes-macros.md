# P3D-154 — Command Recipes / Macros

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 11 (Pós-GA)
- **Status Canônico**: `ROADMAP PÓS-GA (Plugins, Automation & AI)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


## Objetivo

Registrar/salvar/reproduzir sequências semânticas de Commands para automação do usuário.

## Princípio

Recipe opera sobre `CommandId` e parâmetros, nunca coordenadas de UI.

## Futuro

Lua, MCP e AI podem criar/usar Recipes pela mesma Application API.

## Segurança

Execução deve respeitar undo/redo, contexto, validação de parâmetros e permissões de ações destrutivas.