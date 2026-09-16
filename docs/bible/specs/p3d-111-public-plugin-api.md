# P3D-111 — Public Plugin API

<aside>
🧩

Estado: **boundary deve ser desenhada antes de ampliar runtimes** · Prioridade: P1.

</aside>

## Objetivo

API pública estável o suficiente para Lua, futuros adapters e módulos oficiais sem expor internals frágeis.

## Superfícies candidatas

Commands/queries, scene/object handles, geometry operations aprovadas, asset/project operations, registration metadata, controlled UI panel descriptors e events.

## Regras

- handles/DTOs em vez de refs/lifetimes;
- capability/permission model;
- API versioning/feature discovery;
- plugins não recebem egui context nem wgpu device;
- operações destrutivas passam por Application/Undo quando aplicável.

## Dependências

P3D-100–110.

## Testes / DoD

Mock plugin usa API sem imports internos, breaking changes detectáveis e documentação de cada capability.