# P3D-112 — MCP API / Automação

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 11 (Pós-GA)
- **Status Canônico**: `ROADMAP PÓS-GA (Plugins, Automation & AI)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Escopo revisado: **MCP/Automation API separada da UI de agente e da modelagem por IA** · Prioridade: P2.

</aside>

## Objetivo

Permitir que agentes/ferramentas externas consultem e executem operações Petunia por interface estruturada.

## Superfície

Commands, queries, project/assets, scene hierarchy, selection, modeling operations permitidas, import/export e diagnostics. Ferramentas devem ser semanticamente estáveis e não simular cliques.

## Pesquisa

Comparar MCP com outras formas de automação/IPC somente quando a Application API estiver clara; MCP pode ser um adapter sobre a mesma superfície.

## Segurança

Permissions, explicit destructive actions, path sandbox/validation, rate/size limits e logs auditáveis quando apropriado.

## Dependências

P3D-100–109, P3D-111.

## Testes / DoD

Headless calls, invalid arguments, cancellation/error, no widget coordinates e documentação/schema gerados.