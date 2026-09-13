# P3D-019 — Face Select

<aside>
🧩

Estado reportado: **implementado e funcional** · Prioridade: P1.

</aside>

## Objetivo

Selecionar/manipular faces com integração completa a Extrude, Inset, Paint isolation e materiais.

## Auditoria

Validar picking, multi-select, backfaces/X-Ray, seleção após triangulação/topology edits e sincronização central.

## Dependências

P3D-015, P3D-020, P3D-029, P3D-031, P3D-132.

## Testes / DoD

Faces adjacentes, seleção múltipla, faces ocultas, delete/extrude/inset e manutenção segura do selection state.