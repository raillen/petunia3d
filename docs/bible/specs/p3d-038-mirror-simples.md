# P3D-038 — Mirror simples

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Selection Domain, Transform & Modeling Core)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **precisa auditoria** · Prioridade: P1.

</aside>

## Objetivo

Espelhamento simples adequado a low-poly, sem criar modifier stack complexa se não houver necessidade.

## Auditoria

Descobrir se mirror atual é operação destrutiva, procedural ou apenas transform. Definir eixo/plano/pivot de referência e seam/weld behavior suportado.

## Arquitetura

Mirror core recebe mesh + plane/pivot descriptor; UI não deve manter cópia paralela.

## Dependências

P3D-025–027, P3D-041.

## Testes / DoD

X/Y/Z, objeto deslocado/rotacionado, seam, normals/UV e undo.