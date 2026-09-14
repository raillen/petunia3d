# P3D-076 — Contextual Tool Shelf

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 6
- **Status Canônico**: `COMPLIANT (Scene, Assets, Outliner & Inspector)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **parcial e duplicando conceitos** · Prioridade: P0.

</aside>

## Objetivo

Shelf flutuante inferior dedicada a ações contextuais, não a repetir Selection Domain.

## Decisão

Remover Vertex/Edge/Face da shelf. Em Model pode mostrar Extrude/Inset/Bevel/Loop Cut/Knife/Subdivide/Merge etc. Paint/UV/Animation exibem conjuntos próprios.

## UX

Grupos com separadores, icon+label em largura ampla, icon-only + tooltip em largura curta e overflow menu quando necessário. Nunca overlap.

## Dependências

P3D-015, P3D-077, P3D-079, P3D-083.

## Testes / DoD

Todos os domínios/workspaces relevantes, resize, overflow, commands corretos e sem duplicação de estado.