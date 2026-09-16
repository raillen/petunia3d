# P3D-156 — Decals & Surface Details

<aside>
🎭

**Prioridade alta pós-GA.** Entregar uma superfície simples para detalhes visuais reutilizáveis em personagens, roupas, props e environments, integrada a UV, atlas, animation variables e export.

</aside>

# Fonte especializada

[39 — Pós-V1: Decals, Surface Details e Facial Atlases](../foundations/39-pos-v1-decals-surface-details-facial-atlas.md)

# Escopo inicial

- Decal Library;
- static BaseColor + alpha decals;
- visual placement;
- transform/tint/opacity;
- surface or UV attachment;
- Mirror;
- Bake to BaseColor;
- Undo/Redo;
- integration with Texture Atlas Builder.

# Evolução

- Decal Sets e semantic variables;
- eyes/mouth/face atlas presets;
- simple step animation de variants;
- engine adapters;
- normal/roughness decals;
- multi-channel baking.

# Definition of Done inicial

- usuário consegue importar PNG com alpha, salvá-lo na biblioteca, aplicar a qualquer mesh, reposicionar sem editar UV manualmente e exportar um GLB estático com o detalhe corretamente baked;
- comportamento determinístico após save/reload;
- decals participam do Asset Validator;
- budget/overdraw warnings disponíveis quando aplicável;
- nenhuma dependência de node editor.