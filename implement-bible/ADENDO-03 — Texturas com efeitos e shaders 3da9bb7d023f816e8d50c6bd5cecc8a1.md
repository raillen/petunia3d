# ADENDO-03 — Texturas com efeitos e shaders

<aside>
🧩

Este adendo foi formalizado como **P3D-140 — Material Shader Profiles & Effects**.

</aside>

## Direção

Suportar efeitos úteis a assets low-poly como vidro/transparência, emissive/lava, obsidian, unlit e toon por perfis simples de material antes de adotar node editor completo.

## Separação importante

- **Paint Effect Stack (P3D-134):** efeitos que alteram/compoem textura/layers.
- **Material Shader Profiles (P3D-140):** efeitos de shading em tempo real/material.
- **Texturing Nodes (P3D-113):** camada avançada posterior sobre o mesmo sistema.

## Regra

Nenhum efeito cria um segundo Material System paralelo; capabilities do renderer/export format são validadas e documentadas.