# P3D-043 — Busca e filtros de Assets

<aside>
🧩

Estado: **precisa auditoria** · Prioridade: P1.

</aside>

## Objetivo

Pesquisar e filtrar modelos/materiais/texturas por metadata sem bloquear a UI.

## Contrato

Busca por nome/tags/tipo e filtros aprovados. A Project Model Library oferece filtros completos; Asset Browser pode expor subset compacto.

## Arquitetura

Query opera sobre index em memória/cache apropriado, não filesystem traversal por keystroke. Tags têm normalização consistente.

## Dependências

P3D-003, P3D-042, P3D-125.

## Testes / DoD

Search incremental, combinação de tags, zero results, clear filters e resultado determinístico.