# P3D-064 — UV Editing básico

<aside>
🧩

Estado: **a especificar sobre o UV core real** · Prioridade: P1.

</aside>

## Objetivo

Operações essenciais de UV compatíveis com o workflow low-poly.

## Escopo inicial sugerido

Visualizar ilhas, selecionar, Move/Rotate/Scale UV, mark/unmark seam quando existir suporte, unwrap básico e packing apenas se a implementação permanecer simples/previsível.

## Regras

Não prometer smart unwrap avançado. Preservar correspondência vertex/loop/UV com alterações topológicas suportadas.

## Dependências

P3D-063, P3D-065, P3D-050.

## Testes / DoD

Meshes simples, seams, múltiplas ilhas, overlap permitido/detectado conforme política, undo e serialization.