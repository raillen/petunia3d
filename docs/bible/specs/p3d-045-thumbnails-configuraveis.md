# P3D-045 — Thumbnails configuráveis

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 6
- **Status Canônico**: `COMPLIANT (Scene, Assets, Outliner & Inspector)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **precisa auditoria** · Prioridade: P1.

</aside>

## Objetivo

Thumbnails com tamanho ajustável no Asset Browser/Model Library sem regeneração por frame.

## Contrato

Slider/steps de tamanho alteram layout, não o asset. Grid recalcula colunas responsivamente. Cache pode manter variantes ou reusar source eficiente.

## Dependências

P3D-003, P3D-042, P3D-125.

## Performance

Lazy loading para itens visíveis/próximos, placeholder estável e política de cache limitada.

## Testes / DoD

Small/large, resize da janela, centenas de assets, missing thumbnail e persistência como UI setting.