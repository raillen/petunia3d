# P3D-030 — Multi-Extrude

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Selection Domain, Transform & Modeling Core)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **não implementado** · Prioridade: P1.

</aside>

## Objetivo

Extrusão múltipla em dois modos quando a seleção permitir: **como conjunto/centro** e **individual**.

## Decisões

A UI deve explicar o modo ativo; não aplicar semântica ambígua em seleções incompatíveis. Direção pode usar normal individual ou eixo/offset comum conforme modo.

## Arquitetura

Reutilizar o máximo possível do kernel de Extrude e do Modal Tool Feedback; evitar segunda implementação topológica paralela.

## Dependências

P3D-029, P3D-131, P3D-041.

## Testes / DoD

Faces desconectadas, adjacentes, individual vs conjunto, cancel, self-intersection/casos inválidos e undo único.