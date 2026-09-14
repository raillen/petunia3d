# P3D-152 — Portable Project Package

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 12 (Pós-GA)
- **Status Canônico**: `ROADMAP PÓS-GA (Low-Poly Game Asset Toolkit)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


## Objetivo

Empacotar um projeto de forma portável para backup, compartilhamento e arquivamento offline.

## Fluxo

Pack Project → validar dependências → incluir assets necessários → manifest/version → package. Unpack deve validar e restaurar sem paths absolutos quebrados.

## Filosofia

Local-first, sem cloud/login obrigatório.