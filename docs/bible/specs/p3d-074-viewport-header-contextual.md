# P3D-074 — Viewport Header contextual

<aside>
🧩

Estado: **parcial/rudimentar, com excesso e overlap** · Prioridade: P0.

</aside>

## Objetivo

Header organizado por domínios semânticos claros, responsivo e profissional.

## Estrutura

Selection Domain → menus (`View/Select/Add/Object|Mesh`) → Orientation/Pivot → Snap/Proportional/X-Ray/Overlays → Shading.

## Decisões

Com P3D-015, não exibir Object/Edit Mode separado. Vertex/Edge/Face ficam no header e não se repetem na shelf inferior. Em largura reduzida, remover labels secundárias/usar overflow em vez de esmagar controles.

## Visual

IconIds reais, chevrons, segmented controls e grupos/divisores. Sem quadrados genéricos.

## Dependências

P3D-015, P3D-077, P3D-079, P3D-088.

## Testes / DoD

Object/Vertex/Edge/Face, menus abertos, 1366×768/1600×900/1920×1080 e DPI variados sem overlap.