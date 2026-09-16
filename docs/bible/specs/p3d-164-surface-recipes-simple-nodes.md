# P3D-164 — Surface Recipes & Simple Nodes

<aside>
🔗

Estado: **pós-V1 aprovado; Advanced**. Nodes existem para compor operações aprovadas e ficam escondidos atrás de Recipes na experiência normal.

</aside>

# Princípio

Usuário comum aplica `Edge Wear`, `Dirt`, `Stylized Wood`, `Anime Skin`; `Advanced > Edit Recipe` revela o graph.

# Graph model

DAG tipado, sem loops, cycle rejection, serialization versionada, deterministic evaluator, memoization/cache e errors explícitos.

# Surface nodes iniciais

Texture, Color, Value, UV, Vertex Color; Tint, Brightness/Contrast, Hue/Saturation, Invert; Mix/Add/Multiply; Threshold/Levels/Mask; Checker/Gradient/Noise; AO/Curvature/Normal Direction/Position quando baking existir; outputs BaseColor/Roughness/Normal/Height/Mask.

# Generator Recipes posteriores

Path Input, Profile Input, Asset Input, Sweep, Repeat Along Path, Stretch Along Path, Random Rotate/Scale, Alternate Asset, Material Assign e Bake Output.

# Proibido

Per-point fields genéricos, topology scripting arbitrário, simulation zones, arbitrary attributes, compositor, shader graph geral, gameplay/world graph.

# UI

Preset first, graph second. Exposed parameters precisam nome amigável, range, tooltip e default seguro.

# Testes / DoD

Cycle errors, graph migrations, cache invalidation, deterministic outputs, recipe preset round-trip e absence de arbitrary geometry mutation.