# P3D-110 — Lua Plugin System

<aside>
🧩

Direção: **Lua-first sobre API independente de linguagem** · Prioridade: P2.

</aside>

## Objetivo

Plugins seguros e leves para estender commands/tools/painéis controlados sem expor egui/wgpu/core internals.

## Arquitetura

`Public Extension API` é a autoridade; runtime Lua é adapter inicial. Capabilities/permissions definem acesso a project, scene, mesh, assets e UI extension slots.

## Regras

Plugins declarativos/scripted não recebem raw pointers, raw GPU access ou filesystem irrestrito por padrão. Versionar API e fornecer errors claros.

## Python/JavaScript

Não incorporar runtimes pesados agora. Se houver demanda, adapters futuros consomem a mesma Extension API sem redesenhar o core.

## Dependências

P3D-111, P3D-100–109.

## Testes / DoD

Load/unload, API version mismatch, error isolation, permission denial, malformed plugin e plugin não pode crashar o host facilmente.