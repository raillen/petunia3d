# P3D-010 — Overlays

<aside>
🧩

Estado inicial: **precisa auditoria** · Prioridade: P1.

</aside>

## Objetivo

Controlar informações auxiliares como grid, axes, selection highlights e overlays aprovados.

## Auditoria

Inventariar overlays existentes e verificar quais são reais, stubs ou duplicados. Medir custo de renderização e identificar dependências de egui no renderer.

## UX

Um toggle principal pode habilitar/desabilitar overlays e um popover/dropdown concentra opções. Não espalhar vários toggles equivalentes pelo header.

## Arquitetura

Cada overlay deve ser descriptor/setting semântico consumido pelo render adapter; não ler widgets diretamente.

## Dependências

P3D-009, P3D-011, P3D-074, P3D-105.

## Testes / DoD

Estados persistem corretamente quando aplicável, não quebram selection/shading e não introduzem custo relevante quando desligados.