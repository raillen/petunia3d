# P3D-072 — Exportadores modulares

<aside>
🧩

Estado: **precisa auditoria arquitetural** · Prioridade: P0.

</aside>

## Objetivo

Exporters plugáveis/registráveis por formato, compartilhados por export individual/multi/batch.

## Interface esperada

Capabilities, validation, export options/profile, serialize/write e report. O application layer coordena destino e errors.

## Regras

Não conhecer widgets/file dialogs; não depender de global state; format-specific quirks ficam no adapter.

## Dependências

P3D-068–071, P3D-102/103/108, P3D-124.

## Testes / DoD

Golden files/round-trip onde possível, capabilities, unsupported material/profile e registro de novo exporter com baixo extension cost.