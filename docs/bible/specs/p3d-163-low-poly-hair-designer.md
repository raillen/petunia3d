# P3D-163 — Low-Poly Hair Designer

<aside>
💇

Estado: **pós-V1 aprovado**. Sistema de guide curves → clumps geométricos, voltado a PS1/N64/anime/chibi/cartoon; não é groom físico de centenas de milhares de strands.

</aside>

# Pipeline

Guide Curve → resample → stable frame → cross-section → clump mesh → modifiers/style → optional Bake to Mesh.

# Cross-sections

Ribbon, Triangle, Diamond, Box e low-poly Tube.

# Parâmetros

Root width, tip width, thickness, segments, taper, twist, curvature, mirror, surface attachment e seed quando houver cluster.

# Ferramentas

Draw Hair, Add Clump, Duplicate; Comb, Smooth, Cut, Lengthen, Width; Clump, Separate, Noise, Randomize, Mirror.

# Cluster

Uma guide pode gerar poucas clumps vizinhas com density, spacing, width/length variation e seed. Manter budget explícito.

# Surface

Roots usam P3D-158; guides usam P3D-161.

# Hair Cards

Modo secundário para casos específicos; UI deve alertar sobre alpha/overdraw e preferir geometry clumps para o fluxo principal.

# Futuro

Curl/Wave, guide interpolation e Braids depois da V1 do módulo; sem physics hair.

# Testes / DoD

Stable frames, surface roots, mirror, deterministic seed, budget/readiness, bake to mesh e export material/alpha behavior.