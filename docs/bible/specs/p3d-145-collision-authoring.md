# P3D-145 — Collision Authoring

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 12 (Pós-GA)
- **Status Canônico**: `ROADMAP PÓS-GA (Low-Poly Game Asset Toolkit)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


## Objetivo

Permitir autoria simples de colliders de game sem transformar Petunia em physics editor.

## Tipos iniciais

Box, Sphere, Capsule, Convex Hull e Mesh quando o formato alvo suportar.

## Operações

Generate From Bounds, transform, duplicate, visibility/preview e association com AssetId/ObjectId.

## Arquitetura

Collision data é metadata neutra exportável. Physics runtime pertence à engine, não ao Petunia.