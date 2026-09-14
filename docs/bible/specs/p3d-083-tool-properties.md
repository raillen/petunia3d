# P3D-083 — Tool Properties

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 6
- **Status Canônico**: `COMPLIANT (Scene, Assets, Outliner & Inspector)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **boundary ainda fraca/incompleta** · Prioridade: P0.

</aside>

## Objetivo

Configurações da tool ativa separadas de propriedades do objeto.

## Exemplos

Primitive parameters, Bevel amount, Extrude mode, brush size/strength. Elas podem aparecer como popover, small panel ou região contextual, sem entrar no Object Inspector.

## Arquitetura

Tool metadata/state expõe parâmetros editáveis por descriptor/API; UI não acessa internals arbitrários. Mudanças que afetam operação modal integram preview/undo corretamente.

## Dependências

P3D-048, P3D-101, P3D-131.

## Testes / DoD

Troca de tool, defaults, cancel/confirm, persistência somente quando apropriada e nenhum vazamento de object properties.