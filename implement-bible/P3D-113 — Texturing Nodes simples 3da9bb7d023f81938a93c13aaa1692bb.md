# P3D-113 — Texturing Nodes simples

<aside>
🧩

Estado: **futuro; bloqueado pela fundação Surface & Texture** · Prioridade: P3.

</aside>

## Objetivo

Graph simples para texturização/filtros, não compositor de cena nem scripting visual genérico.

## Ordem

Só iniciar após P3D-050/055/063 estarem estáveis. Nodes devem operar sobre os mesmos TextureResources, channels, layers e profiles existentes.

## Escopo inicial

Inputs de textura/cor, operações/filtros simples e output para channel/layer quando houver benefício real. O graph data é serializável e independente do widget de node editor.

## Não objetivos

Render compositor, gameplay graphs, geometry nodes completos.

## Testes / DoD

Graph deterministic, cycle/error handling, serialization e avaliação cacheável.