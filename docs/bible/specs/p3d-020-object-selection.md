# P3D-020 — Object Selection

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Selection Domain, Transform & Modeling Core)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **parcial / bugs de sincronização e delete** · Prioridade: P0.

</aside>

## Objetivo

Uma única seleção de objetos compartilhada entre viewport, Outliner, Inspector, commands e futuras APIs.

## Correções obrigatórias

- clique no viewport seleciona o mesmo ObjectId refletido no Outliner;
- seleção no Outliner atualiza viewport/Inspector;
- `Delete` e ação de delete do Outliner disparam exatamente o mesmo Command/undo;
- nenhum widget possui seleção paralela.

## Arquitetura

SelectionService/EditorState é autoridade; UI é projection. Delete deve limpar dependências/seleção de forma segura.

## Dependências

P3D-015, P3D-041, P3D-046, P3D-048.

## Testes / DoD

Single/multi-select se suportado, delete de item selecionado, objeto já removido, undo/redo e seleção por API/headless.