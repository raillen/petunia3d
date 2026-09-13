# 12 — Contrato de Materiais, Texturas, UV e Color Pipeline

<aside>
🎨

Página normativa para P3D-050–065, P3D-113, P3D-132–134 e P3D-140. Antes do Paint/UV ser considerado implementável, esta pipeline precisa estar congelada sobre a realidade do renderer.

</aside>

# Pipeline única

`Material → Texture Channels → UV/Projection → Paint Surface/Layers → Renderer → Import/Export`. Não criar representação paralela para Paint, preview ou nodes.

# Canais base

Albedo/Diffuse, Normal/Bump, Roughness/Glossiness e Height/Displacement. Perfis adicionais de P3D-140 podem adicionar Emission/Opacity/Transmission/Metallic/Specular somente quando a pipeline suportar de forma coerente.

# Color management

Congelar quais dados são sRGB versus linear, conversão na carga/upload/render/save e comportamento de alpha. Não aplicar gamma em dados como Normal/Roughness/Height.

# Normal maps

Definir tangent-space convention, orientação de Y quando aplicável, geração/necessidade de tangents e fallback quando UV/tangent não existir.

# Texture resources

Definir identidade, resolução, pixel format/bit depth suportado, mip policy, cache, dirty state, save/export e missing-resource behavior. Alterar resolução exige operação explícita.

# Paint layers

Layer stack pertence aos dados de pintura, não à UI. Definir blend/opacity/visibility, reorder, masks, undo por stroke/operation e composição determinística. Effect/Decal layers usam o mesmo stack.

# Paint engine

Antes da implementação final, fechar brush sampling/interpolation, hardness/strength, coordinate mapping, seam behavior, channel target, GPU/CPU ownership e transaction granularity. Evitar divergência visual entre preview e resultado salvo.

# UV

Definir V1 mínimo explicitamente: seleção/transform de islands/elements, seam workflow quando existir, unwrap escolhido, pack, pin, texel-density apenas se aprovado. Recursos como UDIM não entram silenciosamente.

# Import/Export

Matriz por formato documenta canais suportados, alpha, texture embedding/copy, color-space e perdas. Glossiness↔Roughness conversion deve ser explícita.

# Tests

Color-space goldens, channel fallback, save/load, layer composition, seam cases, UV operations, texture resize, missing textures, import/export round-trip e visual reference images quando necessário.

# DoD

Paint, UV, Material e Renderer compartilham os mesmos resources/IDs e não exigem conversões ad-hoc espalhadas em widgets.