# P3D-141 — Internal AI Agent Panel

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 11 (Pós-GA)
- **Status Canônico**: `ROADMAP PÓS-GA (Plugins, Automation & AI)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Novo item · Futuro · Prioridade: P3.

</aside>

## Objetivo

Painel interno, modal/dockável conforme UX, para conversar e trabalhar com agentes/LLMs sem acoplar provider ao core.

## Arquitetura

UI do chat é frontend; `AgentService`/provider adapter gerencia modelos, streaming, tool calls e conversation state. As tools expostas ao agente são as mesmas semantic APIs de Commands/MCP quando possível.

## UX

History, stop/cancel, tool-call visibility, confirmations para ações destrutivas, provider/model settings e contexto claro do projeto atual. Não ocupar viewport permanentemente por padrão.

## Segurança

Nunca executar texto do modelo como shell/script arbitrário sem capability explícita. Ações mutáveis passam por commands e undo quando aplicável.

## Dependências

P3D-112, P3D-142, P3D-078.

## Testes / DoD

Provider mock, streaming, cancel, malformed tool call, confirmation flow e painel destacável sem contaminar application state.