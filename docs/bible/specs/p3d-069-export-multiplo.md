# P3D-069 — Export múltiplo

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 8
- **Status Canônico**: `COMPLIANT (Import, Export & Delivery Pipeline)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **a detalhar sobre o mesmo pipeline de export** · Prioridade: P1.

</aside>

## Objetivo

Selecionar vários assets e exportá-los em uma operação coerente.

## Regra

Não criar segundo exporter. É uma orchestration sobre P3D-068/P3D-072 com seleção múltipla, naming e relatório por item.

## UX

Preview da lista, conflitos de nomes, profile comum ou overrides somente quando necessário, progress e falhas parciais claras.

## Dependências

P3D-068, P3D-070, P3D-072.

## Testes / DoD

Mix de assets válidos/inválidos, conflito de filename, cancelamento e relatório completo sem perder sucessos já escritos.