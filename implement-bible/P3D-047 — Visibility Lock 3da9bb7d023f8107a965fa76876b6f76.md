# P3D-047 — Visibility / Lock

<aside>
🧩

Estado: **precisa auditoria** · Prioridade: P1.

</aside>

## Objetivo

Mostrar/ocultar e bloquear itens de forma consistente entre Outliner, viewport e Inspector.

## Contrato

`Visible` e `Locked` são estados semânticos; ícones eye/eye-off e lock/unlock refletem a mesma fonte. Lock impede manipulação/seleção conforme política documentada.

## Arquitetura

Não armazenar visibility somente no row widget. Se for estado de projeto, serializar; se for sessão temporária, justificar explicitamente.

## Dependências

P3D-046, P3D-020.

## Testes / DoD

Hidden/locked selection, restore, save/load e comandos via UI/API.