# P3D-086 — Icon Packs

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 3
- **Status Canônico**: `COMPLIANT (UI Infrastructure, Customization & Input)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **precisa auditoria** · Prioridade: P1.

</aside>

## Objetivo

Packs de ícones externos com `manifest.toml`, `icons.toml` e assets SVG/PNG, usando a mesma API runtime dos packs oficiais.

## Contrato

UI solicita `IconId`; resolver escolhe pack ativo → Petunia/default fallback → missing diagnostic. Nunca carregar path direto em widget.

## Segurança/performance

Validar paths/formatos, cachear parse/rasterização, nunca baixar assets em runtime.

## Dependências

P3D-084, P3D-087–088, P3D-125.

## Testes / DoD

SVG/PNG, pack parcial, asset corrupto, fallback, switch runtime e nenhuma rasterização por frame.