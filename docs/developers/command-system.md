# Sistema de Comandos (Undo/Redo)

Operações destrutivas são comandos transacionais sobre snapshots clonados
(suficiente e correto para low-poly, cap de 100 níveis).

## Regras

- **Checkpoint antes de mutar**: `state.checkpoint("rótulo")` salva o estado
  atual; mutações sem checkpoint prévio quebram o undo (1 `Ctrl+Z` deve
  desfazer 1 gesto — nunca 2, nunca 0).
- **Edição contínua** (arrastar/scrub): 1 checkpoint por gesto (no início do
  arrasto ou na primeira mudança por teclado), nunca por frame.
- **Comandos** (`crates/commands` + `CommandDispatcher`): `is_destructive()`
  decide checkpoint automático; erros via `CommandError`, sem pânico.
- **Sessões interativas** (modais, criação de primitivas): 1 transação por
  operação; `Esc` cancela restaurando o original.

Desfazer: `Ctrl+Z`. Refazer: `Ctrl+Shift+Z`. Rótulos aparecem na barra de status.
