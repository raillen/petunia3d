# P3D-050 — Material System

<aside>
🧩

Estado: **rudimentar / integração com Paint quebrada** · Prioridade: P0.

</aside>

## Objetivo

Definir um único modelo de material/textura compartilhado por viewport, Paint, UV, import/export e futuro nodes.

## Arquitetura canônica

`Material → Texture Channels → UV/Projection → Paint Surface/Layers → Renderer`.

Canais base: Albedo, Normal, Roughness e Height/Displacement. Perfis adicionais entram em P3D-140. P3D-113 só compõe este modelo depois.

## Auditoria

Mapear material atual, color-only paths, texture resources, shader bindings, serialization e por que Paint não conversa com eles.

## Regras

Nenhum segundo sistema paralelo de “material pintado”. Texturas e canais usam IDs/resources explícitos. Renderer interpreta descriptor por boundary clara.

## Dependências

P3D-051–055, P3D-062–065, P3D-113, P3D-140.

## Testes / DoD

Create/assign/save/load material, channel missing/fallback, preview, asset duplication e integração mínima com Paint/UV.