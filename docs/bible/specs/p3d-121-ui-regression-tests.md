# P3D-121 — UI Regression Tests

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 9
- **Status Canônico**: `PLANEJADA (QA, Docs, Release & GA Candidate)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Quality gate contínuo, não fase final · Prioridade: P1.

</aside>

## Objetivo

Evitar regressões de comportamento/layout nos componentes e estados críticos da UI.

## Ferramentas

Usar `egui_kittest`/infra existente para interação/estado e screenshots/snapshots somente onde estáveis e úteis.

## Estados mínimos

Object/Vertex/Edge/Face, menus/submenus, toolbar/shelf, Outliner/Inspector, Model Library, Settings, Paint/UV conforme implementados.

## Dependências

P3D-073–089, P3D-118.

## Testes / DoD

Testes não dependem de timings frágeis, cobrem keyboard/focus e snapshots são revisáveis quando mudança é intencional.