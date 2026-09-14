# P3D-146 — Sockets & Attachment Points

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 12 (Pós-GA)
- **Status Canônico**: `ROADMAP PÓS-GA (Low-Poly Game Asset Toolkit)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


## Objetivo

Criar transforms nomeados usados como attachment points por engines e workflows modulares.

## Exemplos

Hand_R, WeaponSocket, HeadAccessory, Wheel_FL, Exhaust.

## Requisitos

IDs estáveis, nome editável, transform, visualização no viewport, hide/show, export/import quando suportado.

## Arquitetura

Socket é dado de asset, não objeto de UI. Deve integrar Bridge/API e permanecer independente da engine.