# P3D-008 — Frame Selected / Frame All

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 4
- **Status Canônico**: `COMPLIANT (Viewport, Navigation & Reference Workflow)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado inicial: **integração pendente** · Prioridade: P1.

</aside>

## Objetivo

`Frame Selected` centraliza/enquadra a seleção; `Frame All` enquadra todo conteúdo relevante da cena/projeto.

## Contrato

Implementar como Commands semânticos, independentes de atalho ou menu. A câmera calcula bounds a partir de queries do editor/scene; Outliner e viewport compartilham a mesma seleção.

## UX

Atalhos devem vir do keymap ativo, inclusive notebook/no-numpad. Menus/tooltips exibem o binding resolvido dinamicamente.

## Dependências

P3D-020, P3D-090, P3D-100.

## Testes / DoD

Sem seleção, seleção única/múltipla, objetos muito pequenos/grandes, referências e scene bounds; operação não altera project dirty state.