# P3D-149 — Texture Atlas Builder

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 12 (Pós-GA)
- **Status Canônico**: `ROADMAP PÓS-GA (Low-Poly Game Asset Toolkit)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


## Objetivo

Agrupar texturas de vários assets em atlas e remapear UVs de maneira previsível.

## Funções

Seleção múltipla, padding, tamanho do atlas, packing, preview, export e relatório de remapeamento.

## Segurança

Operação deve ser undoable ou gerar variante/cópia para não destruir trabalho original sem confirmação.