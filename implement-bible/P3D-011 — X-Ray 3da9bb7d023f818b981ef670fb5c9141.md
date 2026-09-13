# P3D-011 — X-Ray

<aside>
🧩

Estado inicial: **stub visual / botão sem efeito** · Prioridade: P0.

</aside>

## Objetivo

Implementar X-Ray de verdade: feedback visual de transparência/oclusão e seleção coerente de componentes encobertos.

## Requisitos

- o botão atual precisa alterar comportamento real;
- reduzir opacidade/oclusão de forma legível;
- permitir seleção de Vertex/Edge/Face através da geometria conforme contexto;
- definir interação com wireframe, overlays e selection highlight.

## Arquitetura

Selection/picking recebe política X-Ray neutra; renderer recebe estado de visualização. A UI apenas alterna o estado via Command/setting.

## Dependências

P3D-017–019, P3D-105.

## Testes / DoD

Seleção frontal/oculta, objetos sobrepostos, ortho/perspective e desligar/ligar repetidamente; nenhum botão enganoso permanece.