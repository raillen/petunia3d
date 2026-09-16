# P3D-038 — Mirror simples

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