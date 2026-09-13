# P3D-035 — Subdivide

<aside>
🧩

Estado reportado: **implementado e funcional** · Prioridade: P1.

</aside>

## Objetivo

Subdividir geometria selecionada de forma simples e controlada, mantendo foco low-poly.

## Auditoria

Verificar quantidade de cuts/segments realmente suportada, efeitos em UV/material/selection e undo.

## Não objetivo

Não introduzir subdivision surface/smoothing high-poly como consequência automática.

## Dependências

P3D-017–019, P3D-041.

## Testes / DoD

Vertex/edge/face contexts suportados, UV/attributes preservados quando possível e topologia válida.