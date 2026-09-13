# P3D-062 — Paint de mapas via UV

<aside>
🧩

Estado: **bloqueado pela fundação Material/Paint/UV** · Prioridade: P1.

</aside>

## Objetivo

Pintar Normal, Roughness e Height usando o mesmo sistema de layers/UV/canais do material.

## Contrato

O workspace seleciona o channel alvo; strokes escrevem no resource correspondente. Não criar brushes independentes por canal se a diferença puder ser parameterized.

## Dependências

P3D-050, P3D-052–054, P3D-055, P3D-063–065.

## Testes / DoD

Troca de channel sem perda de estado, undo, save/load, Material Preview e export dos mapas corretos.