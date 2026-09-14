# P3D-015 — Object Mode / Modelo Unificado de Seleção

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Selection Domain, Transform & Modeling Core)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Decisão consolidada: **não expor Object Mode e Edit Mode como dois mundos separados** · Prioridade: P0.

</aside>

## Objetivo

Unificar a interação em quatro domínios: **Object / Vertex / Edge / Face**. Object permite editar o objeto completo; os demais operam componentes da mesh.

## UX

`Tab` alterna Object ↔ último domínio de componente utilizado. A UI mostra claramente o domínio ativo e não exige ritual de entrar/sair de Edit Mode.

## Auditoria

Mapear o atual `Object Mode`, `Edit Mode`, seleção, tools, keymaps, header, shelf e commands. Preservar operações funcionais enquanto o estado público é simplificado.

## Arquitetura

Pode existir estado interno de edição para implementação, mas ele não deve dominar a UX nem duplicar comportamento. O estado semântico deve ser algo equivalente a `SelectionDomain::{Object, Vertex, Edge, Face}`.

## Dependências

P3D-016–020, P3D-074, P3D-076, P3D-090.

## Testes / DoD

Troca por UI/Tab, manutenção do último domínio, tools corretas por domínio, seleção sincronizada, undo e ausência de duplicação de Vertex/Edge/Face em regiões diferentes.