# P3D-021 — Move

<aside>
🧩

Estado reportado: **funcional; validar pivot/orientation** · Prioridade: P1.

</aside>

## Objetivo

Translação consistente de Object/Vertex/Edge/Face via gizmo, input modal, command e Inspector.

## Auditoria

Provar que Global/Local, pivot, axis constraints e selection domain produzem resultados coerentes. Verificar gizmo vs operação matemática e undo transacional.

## Arquitetura

Transform operation é independente do gizmo/egui. Gizmo apenas fornece delta/axis. O mesmo core deve poder ser usado por command/script/API.

## Dependências

P3D-025–027, P3D-041, P3D-131.

## Testes / DoD

Todos os domínios de seleção, diferentes pivôs/orientações, cancelamento, precisão e uma única entrada de undo por gesto.