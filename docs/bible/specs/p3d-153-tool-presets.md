# P3D-153 — Tool Presets

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 12 (Pós-GA)
- **Status Canônico**: `ROADMAP PÓS-GA (Low-Poly Game Asset Toolkit)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


## Objetivo

Salvar e reaplicar parâmetros de ferramentas sem salvar geometria.

## Exemplos

Bevel Tiny PS1, Brush Pixel Hard, Primitive Low-Sides.

## Arquitetura

Presets referenciam ToolId/Command metadata e parâmetros serializáveis; UI apenas edita/seleciona presets.