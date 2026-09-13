# P3D-006 — Vistas ortográficas

<aside>
🧩

Estado inicial: **funcional parcial / ampliar** · Prioridade: P1.

</aside>

## Objetivo

Garantir Front, Back, Left, Right, Top, Bottom e Perspective/Orthographic consistentes.

## Adendo aprovado

Adicionar presets isométricos úteis para workflows de games, preferencialmente NE/NW/SE/SW. Eles são **presets de câmera**, não novo modo de modelagem.

## Auditoria

Validar orientation, target/focus, transição perspective↔ortho, integração com Reference Sets e keymaps.

## Contrato

Vistas são comandos semânticos reaproveitáveis por menu, gizmo, shortcut, command palette e futura API. Não hardcodar Numpad no comportamento; o keymap resolve o input.

## Dependências

P3D-005, P3D-007, P3D-014, P3D-090.

## Testes / DoD

Cada preset produz orientação determinística, preserva enquadramento quando apropriado e funciona em perfil notebook/no-numpad.