# P3D-061 — Simple Paint Layers

::: warning STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 7
- **Status Canônico**: `ACTIVE / PRÓXIMA (Materials, Texture, UV & Paint)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Decisão revisada: **promovida de “V1 se viável” para fundação estrutural do Paint** · Prioridade: P0.

</aside>

## Objetivo

Layers simples, previsíveis e suficientes para pintura low-poly, decals, masks e efeitos sem construir um Photoshop.

## Mínimo obrigatório

Create/delete/rename, reorder por drag-and-drop, visibility, opacity, active layer, serialization e undo.

## Arquitetura

Layer data pertence ao recurso de pintura/material; UI mantém apenas seleção/scroll/expanded state. Composite deve ser cacheável e evitar recomposição integral desnecessária por frame.

## Integrações

P3D-055, P3D-132, P3D-133, P3D-134.

## Testes / DoD

Reorder persistido, opacity/visibility, undo, save/load, layer vazia, many layers e performance aceitável.