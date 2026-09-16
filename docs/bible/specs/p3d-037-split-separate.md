# P3D-037 — Split / Separate

<aside>
🧩

Estado: **precisa auditoria** · Prioridade: P1.

</aside>

## Objetivo

Separar elementos de mesh com semântica clara: split dentro da mesh e/ou separate para novo objeto somente se ambos forem suportados.

## Auditoria

Identificar comportamento atual e nomenclatura; eliminar mistura entre “split topology” e “create new object”.

## Arquitetura

Operação atualiza ObjectId/MeshId, seleção, materials/references e undo de forma consistente.

## Dependências

P3D-020, P3D-041, P3D-108.

## Testes / DoD

Selection parcial, connected islands, novo objeto quando aplicável, undo e IDs estáveis.