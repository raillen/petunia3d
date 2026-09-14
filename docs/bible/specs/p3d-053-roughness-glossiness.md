# P3D-053 — Roughness / Glossiness

::: warning STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 7
- **Status Canônico**: `ACTIVE / PRÓXIMA (Materials, Texture, UV & Paint)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


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