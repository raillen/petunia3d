# P3D-078 — Painéis retráteis e redimensionáveis

<aside>
🧩

Estado: **parcial** · Prioridade: P1.

</aside>

## Objetivo

Assets, Outliner, Properties e outros painéis com resize/collapse/detach onde útil, preservando viewport.

## Decisão

Começar com layout default previsível + resize + collapse. Docking controlado é permitido para casos claros (ex. Inspector), mas não expor docking irrestrito em toda a aplicação sem necessidade.

## Arquitetura

Geometria/visibilidade de painéis são UI settings; nenhum painel possui estado de domínio próprio.

## Dependências

P3D-048, P3D-073, P3D-079.

## Testes / DoD

Min/max size, restore, DPI, persistence, painéis ausentes e viewport nunca reduzido a tamanho inutilizável.