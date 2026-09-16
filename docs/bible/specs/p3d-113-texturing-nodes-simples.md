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

## Adendo pós-V1 — Surface Recipes

A direção foi aprofundada no capítulo `42 — Pós-V1: Nodes Simples, Recipes e Procedural Surface`.

Regras adicionais:

- o graph inicial passa a ser chamado conceitualmente de **Surface Recipe**;
- presets/effects são a UX principal; edição de graph é Advanced;
- nodes iniciais: Texture/Color/Value/UV, Tint, Brightness/Contrast, Hue/Saturation, Invert, Mix, Multiply, Threshold/Levels, Noise/Checker/Gradient, Mask e outputs de channels;
- generators baseados em AO/Curvature/Position/Normal só entram quando a infraestrutura de baking correspondente existir;
- graph é DAG; ciclos são rejeitados;
- parâmetros podem ser explicitamente expostos pelo recipe;
- recipes possuem schema versionado e cache/invalidation determinísticos;
- não introduzir fields, loops, simulation zones, arbitrary attributes ou topology mutation genérica;
- Generator Recipes geométricos são um sistema posterior separado e só podem compor operadores high-level aprovados do Spline/Modifier/Core.