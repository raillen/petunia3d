# P3D-161 — Spline Core

<aside>
〰️

Estado: **pós-V1 aprovado**. Spline é infraestrutura do Core; Hair, Path Paint e generators reutilizam a mesma matemática.

</aside>

# Escopo

Polyline + Bézier, open/closed, control points, handles simples, arc-length sampling, tangent evaluation, parallel-transport frame, resampling por distância, snapping, optional surface attachment, serialization, Commands, Undo e Bake/Convert.

# Interação

Create Path, Add Point, Move Point, Break/Align handles, Close/Open, Reverse, resample preview. Não expor terminologia CAD avançada por padrão.

# Operadores compartilhados

Sweep Profile Along Path, Repeat Asset Along Path, Stretch Segment Along Path e End/Corner Rules como primitives de application layer.

# Performance

Cache de samples/frames por revision; edição de ponto invalida apenas path dependents.

# Testes / DoD

Arc length, tangent/frame stability, loops, sharp corners, reverse, serialization, surface-attached points e deterministic sweep input.