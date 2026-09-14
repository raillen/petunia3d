# 08 — Roadmap Pós-GA Aprovado

<aside>
🚀

Roadmap aprovado para o período posterior à consolidação do backlog principal. Nenhum item desta página deve atropelar o GA Hardening nem descaracterizar o Petunia3D como ferramenta focada em criação de assets low-poly para jogos.

</aside>

# Regra de entrada pós-GA

Uma funcionalidade futura deve reforçar pelo menos uma destas metas: facilitar criação de assets low-poly, reduzir trabalho entre Petunia e game engines, reutilizar o Core/Commands/API existentes, preservar modularidade e manter boa performance em PCs modestos. Recursos que empurrem o produto para DCC universal devem virar outro produto ou módulo externo.

# Era 0 — GA Hardening

Antes de novas features grandes, executar uma fase sem expansão de escopo focada em confiabilidade, compatibilidade de projetos, migrations, performance, UX, testes, documentação e encerramento dos débitos arquiteturais críticos. A cadeia `abrir → modelar → pintar → organizar → salvar → fechar → reabrir → exportar` deve ser confiável antes do pós-GA.

# Era 1 — The Low-Poly Asset Tool

Prioridade em recursos diretamente ligados ao pipeline de assets de games: Asset Validator, collision authoring, sockets, LODs, palettes, texture atlas, vertex colors, batch processing, portable project packages, tool presets e command recipes/macros.

# Era 2 — Surface & Lookdev

Evoluir Material/Paint/UV com shader profiles, decals, masks, effect stack e baking simples quando custo/benefício justificar. Começar por perfis e efeitos previsíveis; nodes completos continuam posteriores.

# Era 3 — Characters & Animation

Evoluir Skeleton/Rig Core, skinning, keyframe animation, rig presets, retargeting, auto-rig e Animation Asset Library nessa ordem. Auto-rig só entra após a base manual estar sólida.

# Era 4 — Programmable & Intelligent Petunia

Public Extension API independente do runtime, Lua como primeiro adapter, Command Recipes, MCP, Internal Agent Panel e AI-Assisted Modeling. IA deve operar sobre Commands/Tools/API sem simular cliques de UI.

# Era 5 — Ecosystem

Petunia3D continua independente. Map Editor e Game Engine devem ser produtos separados que compartilham formatos, bibliotecas e módulos neutros. Integrações entram por bridge, export profiles e futuramente Live Asset Link.

# Recursos aprovados

- P3D-144 — Asset Validator & Game Readiness.
- P3D-145 — Collision Authoring.
- P3D-146 — Sockets & Attachment Points.
- P3D-147 — LOD Manager.
- P3D-148 — Palette Manager.
- P3D-149 — Texture Atlas Builder.
- P3D-150 — Vertex Color Painting.
- P3D-151 — Batch Asset Processor.
- P3D-152 — Portable Project Package.
- P3D-153 — Tool Presets.
- P3D-154 — Command Recipes / Macros.
- P3D-155 — Live Asset Link.

# Recursos complementares aprovados como candidatos

- Origin & Pivot Presets.
- Bounding Box Tools.
- Naming Rules para engine/export.
- Asset Variants.
- Pose Library.
- Material Presets.
- Brush Library.
- Procedural Primitive Presets.
- Quick Symmetry.
- Dithering Preview.
- Retro Renderer Preview Profiles.
- Baking simples, inicialmente estudando AO/Normal/Curvature/Vertex AO.
- Package format local para Theme/Icon Pack/Keymap/Plugin/Material Preset/Brush Preset/Rig Preset sem marketplace obrigatório.

# Recursos deliberadamente fora do centro

Sculpting avançado, remesh, fluid simulation, cloth completa, particles complexo, compositor, render farm, video editor, CAD paramétrico completo, Geometry Nodes equivalente ao Blender, scene editor completo, terrain/world editor embutido, game engine dentro do Petunia, marketplace/login/cloud obrigatório.

# Regra para agentes

Quando detalhar ou implementar qualquer item pós-GA, primeiro confirmar que ele continua respeitando a Constituição, que não cria um sistema paralelo aos Commands/Core/API e que pode permanecer isolado do workflow principal quando o usuário não precisar dele.

[P3D-144 — Asset Validator & Game Readiness](../specs/p3d-144-asset-validator-game-readiness.md)

[P3D-145 — Collision Authoring](../specs/p3d-145-collision-authoring.md)

[P3D-146 — Sockets & Attachment Points](../specs/p3d-146-sockets-attachment-points.md)

[P3D-147 — LOD Manager](../specs/p3d-147-lod-manager.md)

[P3D-148 — Palette Manager](../specs/p3d-148-palette-manager.md)

[P3D-149 — Texture Atlas Builder](../specs/p3d-149-texture-atlas-builder.md)

[P3D-150 — Vertex Color Painting](../specs/p3d-150-vertex-color-painting.md)

[P3D-151 — Batch Asset Processor](../specs/p3d-151-batch-asset-processor.md)

[P3D-152 — Portable Project Package](../specs/p3d-152-portable-project-package.md)

[P3D-153 — Tool Presets](../specs/p3d-153-tool-presets.md)

[P3D-154 — Command Recipes / Macros](../specs/p3d-154-command-recipes-macros.md)

[P3D-155 — Live Asset Link](../specs/p3d-155-live-asset-link.md)