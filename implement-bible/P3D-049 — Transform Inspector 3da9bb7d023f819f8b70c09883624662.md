# P3D-049 — Transform Inspector

<aside>
🧩

Estado: **parcial; segue redesign do Inspector** · Prioridade: P0.

</aside>

## Objetivo

Editar Position, Rotation e Scale numericamente com layout adaptativo e integração total ao mesmo Transform System do viewport.

## UX

Rows/fields consistentes, axis labels claros, reset contextual e uniform scale quando suportado. Em painel estreito, quebrar layout de forma legível; em janela ampla, aproveitar espaço sem excesso.

## Arquitetura

Fields disparam a mesma operação/transaction de transform que gizmo/modal tools. Drag contínuo gera uma entrada de undo. Nenhum valor fica mantido só na UI.

## Dependências

P3D-021–027, P3D-041, P3D-048.

## Testes / DoD

Input numérico, drag, invalid values, local/global quando aplicável, undo e sincronização imediata viewport↔Inspector.