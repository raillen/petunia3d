# P3D-081 — Command Palette

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 3
- **Status Canônico**: `COMPLIANT (UI Infrastructure, Customization & Input)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **implementação não comprovada** · Prioridade: P1.

</aside>

## Objetivo

Pesquisar/executar Commands sem depender de localizar botão/menu.

## Busca

Nome localizado, descrição, CommandId, categoria e keybind atual. Resultados respeitam contexts/availability e indicam disabled reason quando útil.

## Arquitetura

Palette consulta CommandRegistry; não possui implementações próprias. Shortcut padrão é configurável pelo keymap.

## Dependências

P3D-089, P3D-090, P3D-100.

## Testes / DoD

Busca fuzzy/normal, idioma, command disabled, execução/undo e keyboard-only navigation.