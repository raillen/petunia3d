# P3D-060 — Color Picker

::: warning STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 7
- **Status Canônico**: `ACTIVE / PRÓXIMA (Materials, Texture, UV & Paint)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **implementação incerta; auditar** · Prioridade: P1.

</aside>

## Objetivo

Selecionar cor existente da textura/material de forma rápida e previsível.

## Contrato

Picker consulta pixel/result layer apropriado conforme política clara; não depende de screenshot do widget se há texture data disponível.

## UX

Shortcut configurável, cursor/tooltip adequado e opção de sample do composite ou layer ativa somente se implementada corretamente.

## Dependências

P3D-055, P3D-061.

## Testes / DoD

Sample em diferentes channels/layers, transparência, outside texture e integração com current color.