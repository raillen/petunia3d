# 04 — Invariantes de Arquitetura, Modularidade e Core Agnóstico à UI

# Direção de dependência

`Frontend/UI → Application → Core`, com renderer e infraestrutura em boundaries explícitas. Core nunca depende de egui/eframe.

# Estado

Distinguir Application/Editor state de UI state. Hover, popup aberto, scroll e tamanho de painel não pertencem ao domínio do projeto.

# Tools e Commands

Tools recebem contexto explícito e não um service locator gigante. Commands representam intenção semântica e devem poder ser disparados por UI, keymap, CLI, Lua, MCP ou futuro frontend.

# Renderer

Separar renderização 3D da composição egui. Picking, câmera e seleção devem usar tipos neutros.

# Extensibilidade

Adicionar/remover tool, command, importer ou exporter não deve exigir alterar muitos módulos não relacionados. Enum fechado é aceitável quando custo de extensão é baixo; não criar trait/dynamic plugin para tudo.

# Regras de dependência automatizáveis

Como alvo, validar regras equivalentes a: `core !→ egui/eframe`, `core !→ localized strings/icons/keycodes`, `tools !→ physical keycodes`, `application !→ file dialog/widget state`, `renderer-core !→ egui widgets`. Adapters/frontend podem depender de Application/Core, nunca o inverso.

# State ownership matrix

Classificar explicitamente: **Project/Document State** (persistente e dirty), **Editor/Application State** (selection/tool/history/session lógica), **UI State** (popup/focus/scroll/layout), **Cache/Derived State** (reconstruível), **Infrastructure State** (GPU/jobs/filesystem). Um campo não muda de categoria por conveniência de implementação.

# API/query discipline

UI não atravessa estruturas profundas arbitrariamente. Commands solicitam mutações; queries/read models expõem dados necessários; events/invalidation avisam mudanças significativas sem criar EventBus global.

# Cross-language future-ready

Primeiro desacoplar internamente, depois API estável. Futuras C ABI/IPC usam handles/DTOs, sem forçar todo o core a tipos FFI-friendly.