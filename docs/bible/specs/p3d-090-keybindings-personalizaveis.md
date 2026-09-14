# P3D-090 — Keybindings personalizáveis

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 3
- **Status Canônico**: `COMPLIANT (UI Infrastructure, Customization & Input)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **implementação precisa auditoria** · Prioridade: P1.

</aside>

## Objetivo

Toda ação configurável por `CommandId` + keymap/context, sem tools conhecendo teclas físicas.

## Modelo

InputTrigger estruturado (keyboard/mouse/wheel) é serializado em strings TOML apenas na fronteira. Suportar múltiplos bindings e unbound explícito.

## Contextos

Global, Viewport, Selection Domain, Outliner, Properties, Paint, UV, Timeline, TextInput e Modal, com prioridade definida.

## Dependências

P3D-091–099, P3D-100.

## Testes / DoD

Parsing/serialization, runtime switch, text input blocking, modal context, persistence e menus/tooltips atualizando shortcuts.