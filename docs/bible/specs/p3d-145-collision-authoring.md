# P3D-145 — Collision Authoring

## Objetivo

Permitir autoria simples de colliders de game sem transformar Petunia em physics editor.

## Tipos iniciais

Box, Sphere, Capsule, Convex Hull e Mesh quando o formato alvo suportar.

## Operações

Generate From Bounds, transform, duplicate, visibility/preview e association com AssetId/ObjectId.

## Arquitetura

Collision data é metadata neutra exportável. Physics runtime pertence à engine, não ao Petunia.