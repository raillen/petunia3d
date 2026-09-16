# P3D-053 — Roughness / Glossiness

<aside>
🧩

Estado: **precisa auditoria; dependente do Material System** · Prioridade: P1.

</aside>

## Objetivo

Controlar aparência de superfície por roughness e, quando necessário, converter/interpretar glossiness de forma explícita.

## Regras

Escolher representação interna canônica; não manter dois valores independentes que possam divergir. Import/export converte formatos externos.

## Dependências

P3D-050, P3D-062, P3D-140.

## Testes / DoD

Constant value, texture map, conversion glossiness↔roughness, preview e serialization.