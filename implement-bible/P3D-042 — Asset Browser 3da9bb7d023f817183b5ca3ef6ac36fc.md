# P3D-042 — Asset Browser

<aside>
🧩

Estado: **precisa auditoria / provável parcial** · Prioridade: P1.

</aside>

## Objetivo

Asset Browser compacto para uso rápido dentro do workspace, compartilhando backend com a Project Model Library.

## Auditoria

Mapear lista atual, source of truth, thumbnails, selection, drag/drop e qualquer duplicação de metadata.

## Contrato

Browser não é o gerenciador completo. Ele oferece busca rápida, categorias essenciais, thumbnails e inserção/uso do asset. Backend usa AssetId e index/cache comuns ao P3D-003.

## Arquitetura

UI não revarre filesystem por frame. Assets, metadata, tags/favorites e thumbnails pertencem ao serviço de assets.

## Dependências

P3D-003, P3D-043–045, P3D-125.

## Testes / DoD

Projeto vazio, assets ausentes, centenas de assets, search/filter, seleção e consistência com a Model Library.