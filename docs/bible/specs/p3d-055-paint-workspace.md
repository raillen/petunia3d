# P3D-055 — Paint Workspace

::: warning STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 7
- **Status Canônico**: `ACTIVE / PRÓXIMA (Materials, Texture, UV & Paint)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **workspace rudimentar/não funcional** · Prioridade: P0.

</aside>

## Objetivo

Reconstruir Paint como workspace contextual simples e poderoso para low-poly, em espírito de “mini Photoshop” sem virar editor 2D generalista.

## UI desejada

- Layers com reorder drag-and-drop, visibility e opacity.
- Brush/Pencil/Eraser/Fill/Picker.
- Painel de efeitos simples.
- Isolamento/máscaras via P3D-132.
- Decals via P3D-133.
- Effect Stack via P3D-134.
- Tool Properties adaptadas à tool ativa.

## Arquitetura

Paint opera sobre texture/layer resources do P3D-050; strokes são transacionais e headless-testable quando possível. Workspace não cria cópia própria do material.

## Dependências

P3D-050, P3D-056–062, P3D-132–134.

## Testes / DoD

Pintura funcional no modelo, undo, save/load, layers, resize/DPI, seleção de canal e ausência de vazamento de estado entre workspaces.