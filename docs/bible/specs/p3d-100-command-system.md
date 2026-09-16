# P3D-100 — Command System

<aside>
🧩

Estado: **precisa auditoria arquitetural profunda** · Prioridade: P0.

</aside>

## Objetivo

`CommandId` deve ser a linguagem semântica única para ações disparáveis por menu, toolbar, keymap, command palette, Lua, MCP, testes e futuros frontends.

## Auditoria obrigatória

Inventariar handlers atuais, ações sem Command, duplicações, callbacks que mutam mesh/state diretamente e atalhos que bypassam dispatcher.

## Contrato

- Commands não conhecem widgets/teclas;
- availability/context é consultável;
- erros são estruturados;
- mutações relevantes integram Undo/Redo;
- `CommandMetadata` pode associar TextId/IconId/DocsTopic/context sem contaminar domain algorithms.

## Arquitetura

Application layer possui registry/dispatcher ou mecanismo equivalente. Core recebe operações/dados sem saber a origem do comando.

## Testes / DoD

Executar commands equivalentes por UI/headless, invalid context, disabled reason, undo/redo e architecture checks impedindo UI logic no core.