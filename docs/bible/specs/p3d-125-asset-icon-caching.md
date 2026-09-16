# P3D-125 — Asset/Icon Caching

<aside>
🧩

Infraestrutura de performance transversal · Prioridade: P1.

</aside>

## Objetivo

Cache central para SVG/PNG, thumbnails, textures e demais recursos sem parse/decode/rasterização pesada por frame.

## Contrato

Cache keys estáveis, invalidation explícita, memory budget/policy simples, lazy loading para thumbnails e placeholder/failure state.

## Arquitetura

UI pede resource por ID; não abre arquivo em `show()`/frame. Background workers só quando ownership/synchronization estiver seguro.

## Dependências

P3D-003, P3D-042–045, P3D-086–088, P3D-133.

## Testes / DoD

Cache hit/miss/invalidate, missing/corrupt asset, memory bound razoável e profiling demonstra ausência de decode repetitivo por frame.