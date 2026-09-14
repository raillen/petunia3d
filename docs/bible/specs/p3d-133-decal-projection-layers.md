# P3D-133 — Decal & Projection Layers

::: warning STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 7
- **Status Canônico**: `ACTIVE / PRÓXIMA (Decal & Projection Layers)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Novo item · V1.x · Prioridade: P2.

</aside>

## Objetivo

Adicionar PNG/JPEG/SVG como layer projetada e reposicionável sobre o modelo.

## Requisitos

Import, move, rotate, scale, opacity, mask/clip, visibility e persistência. Decal permanece não destrutivo enquanto for uma layer.

## Arquitetura

Asset original + projection descriptor + raster/cache. SVG pode ser rasterizado/cacheado para composição mantendo source/metadata. Compartilhar matemática de projection/reference quando útil sem misturar UX.

## Dependências

P3D-055, P3D-061, P3D-132, P3D-125.

## Testes / DoD

Formatos válidos/inválidos, transform, seams, save/load, missing source e bake/export quando aplicável.