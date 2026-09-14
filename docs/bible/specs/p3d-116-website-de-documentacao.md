# P3D-116 — Website de documentação

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 9
- **Status Canônico**: `PLANEJADA (QA, Docs, Release & GA Candidate)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **política definida; implementação real precisa ser verificada** · Prioridade: P1.

</aside>

## Objetivo

Site estático, navegável, pesquisável e fácil de entender com Getting Started, User Manual, Tools, Workspaces, Customization, Reference, Troubleshooting e Developer Docs.

## Stack

Avaliar VitePress/Starlight ou equivalente Markdown-first; não construir framework próprio. GitHub Pages e build local simples.

## Regra

Docs descrevem comportamento real, não roadmap como se estivesse implementado. Pages podem consumir referências geradas.

## Dependências

P3D-117–120.

## Testes / DoD

Build local/CI, search, navigation, mobile, links/assets, Edit this page e publicação automática quando configurada.