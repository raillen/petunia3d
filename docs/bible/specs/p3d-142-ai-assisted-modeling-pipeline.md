# P3D-142 — AI-Assisted Modeling Pipeline

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 11 (Pós-GA)
- **Status Canônico**: `ROADMAP PÓS-GA (Plugins, Automation & AI)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Novo item · **Pesquisa profunda antes de implementação** · Prioridade: RESEARCH.

</aside>

## Objetivo

Investigar e construir modelagem assistida por LLM usando operações semânticas Petunia, reduzindo a necessidade de o modelo gerar geometria bruta diretamente.

## Hipótese central

LLM planeja sequências verificáveis como `add_primitive → transform → select_faces → extrude → bevel → assign_material`, recebe state/diagnostics e itera. Isso deve permitir modelos menores/baratos funcionarem melhor que controle por pixels/coordenadas.

## Pesquisa obrigatória

Comparar tool-use/planning, scene representations compactas, constrained DSL/IR, program synthesis para CAD/3D, self-correction/verifiers, multimodal reference understanding e agentes hierárquicos. Separar evidência acadêmica/protótipos de inferência.

## Arquitetura proposta a validar

Planner → semantic operation IR → validator/simulator → Command/Application API → result/state summary → correction loop. Undo transaction por plano/etapa e limites de custo/token.

## Não objetivos iniciais

Gerar arbitrary mesh blobs, clicar UI, treinar modelo próprio como pré-requisito.

## Dependências

P3D-100–112, P3D-141.

## DoD da fase de pesquisa

Relatório comparativo, benchmark tasks Petunia, protótipo headless com operações restritas, métricas de sucesso/custo e decisão go/no-go.