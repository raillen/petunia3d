# P3D-091 — Detecção de conflitos

<aside>
🧩

Parte obrigatória do sistema de keymaps · Prioridade: P1.

</aside>

## Objetivo

Detectar conflitos reais sem marcar como erro bindings válidos em contextos diferentes.

## Tipos

Exact conflict, context overlap, shadowed binding, reserved binding e platform conflict.

## UX

Ao capturar binding, oferecer Replace/Keep both/Cancel somente quando semanticamente válido. Mostrar command/context afetado.

## Arquitetura

Conflict detector opera sobre keymap compilado/context graph, independente do widget Settings.

## Dependências

P3D-090, P3D-100.

## Testes / DoD

Mesmo input+mesmo context, contextos disjuntos, duplicata do mesmo command, inheritance e reserved escape path.