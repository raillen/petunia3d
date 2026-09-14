# P3D-151 — Batch Asset Processor

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 12 (Pós-GA)
- **Status Canônico**: `ROADMAP PÓS-GA (Low-Poly Game Asset Toolkit)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


## Objetivo

Executar operações seguras em conjuntos de assets pela Project Model Library.

## Operações candidatas

Rename, validate, generate thumbnails, resize textures, change material profile, generate collisions e export.

## Arquitetura

Executa services/commands existentes; não reimplementa lógica de cada feature. Deve fornecer progress, cancelamento quando seguro e relatório final.