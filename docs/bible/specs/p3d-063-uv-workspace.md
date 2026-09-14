# P3D-063 — UV Workspace

::: warning STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 7
- **Status Canônico**: `ACTIVE / PRÓXIMA (Materials, Texture, UV & Paint)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **precisa design conjunto com Materials/Paint** · Prioridade: P1.

</aside>

## Objetivo

Workspace dedicado a visualizar/editar UV sem duplicar dados do Paint/Material.

## UI

Viewport UV 2D, seleção sincronizável quando útil, toolbar/shelf contextual e acesso claro à texture/channel ativo. Não manter timeline ou painéis irrelevantes por padrão.

## Arquitetura

UV data pertence à mesh; texture/material state é compartilhado com P3D-050/055. Workspace é apenas composição de views/tools.

## Dependências

P3D-050, P3D-055, P3D-064, P3D-065.

## Testes / DoD

Abrir/fechar workspace preserva seleção/material, DPI/responsive e nenhum dado é copiado para estado exclusivo da UI.