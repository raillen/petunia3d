# P3D-143 — Game Engine Integration / Bridge

<aside>
🧩

Novo item · Pesquisa/futuro · Prioridade: RESEARCH.

</aside>

## Objetivo

Preparar integração futura entre Petunia3D e uma game engine retro/low-poly simples, sem transformar os dois produtos em um único executável bloated.

## Princípio

Petunia permanece editor independente e exporta para qualquer pipeline suportado. Uma futura engine pode consumir um **Petunia Asset/Project Bridge** compartilhando formatos, metadata e operações aprovadas.

## Possibilidades a estudar

- export profile/manifest para engine;
- live-reload de assets por file watching/IPC;
- collision helpers/LOD/atlas metadata;
- material profile mapping;
- animation clip/rig metadata;
- scene/map editor da engine consumindo assets Petunia.

## Limites

Não colocar gameplay ECS, runtime de jogo, physics world ou editor de mapas dentro do core Petunia por conveniência.

## Arquitetura

Bridge é adapter sobre P3D-068–072/111–112; formatos e IDs têm versionamento. Integração live precisa tolerar engine ausente e funcionar offline.

## DoD da fase de pesquisa

Documento de contrato de asset bridge, protótipo mínimo export/reload e decisão clara do que pertence ao Petunia versus à futura engine.

## Decisão complementar aprovada — reutilização da base 3D

A futura Game Engine e o Map Editor podem reutilizar componentes consolidados do Petunia3D **desde que esses componentes sejam extraídos como bibliotecas neutras**, sem dependência de egui, menus, editor state ou regras específicas de modelagem.

Candidatos: math/geometry, asset schemas, mesh data, material schemas, render core, camera/picking, serialization, cache, import/export contracts, collision/socket/LOD metadata, jobs/diagnostics e bridge protocol.

Não extrair prematuramente. Primeiro consolidar no Petunia; depois, quando existir um segundo consumidor real, separar a parte genérica e criar testes de arquitetura para impedir dependências reversas.

Ver [09 — Shared 3D Foundation, Map Editor e Game Engine](../constitution/09-shared-3d-foundation-map-editor-e-game-engin.md).