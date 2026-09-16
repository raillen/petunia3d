# Extensões, Módulos e Plugins

O Petunia3D separa três coisas que costumam ser confundidas: **módulos internos**,
**provedores oficiais** e **plugins de terceiros**. Cada uma tem contrato e fronteira
próprios.

> Autoridade: [10 — Extension API, Plugins e Módulos Oficiais](../bible/foundations/10-extension-api-plugins-modulos.md),
> [30 — I/O, Projeto .petunia, Lua Plugins e MCP](../bible/foundations/30-io-petunia-lua-mcp-rust.md) e
> [34 — Arquitetura Modular Explícita](../bible/foundations/34-arquitetura-modular-rust-safety.md).

## 1. API pública de terceiros: Lua

- A superfície pública para plugins é **Lua 5.4** (hospedada por `mlua`).
- Plugins **não** recebem acesso direto à memória, às estruturas half-edge internas,
  a `egui`, a `wgpu`, a `Painter`, a raw input ou a markup arbitrário.
- Sem ABI nativo público obrigatório na V1. Provedores nativos oficiais (como
  `manifold-rust` e `xatlas`) são **infraestrutura interna**, não precedente
  automático para plugins nativos da comunidade.
- Toda mutação passa por **transactions / Application API**, nunca por escrita direta
  no documento.

## 2. Capabilities declarativas

Um plugin declara o que precisa; o host concede por capability:

```
register_ui_panel
register_viewport_overlay
register_commands
read_selection
read_document
edit_geometry
edit_uv
edit_materials
import_files
export_files
filesystem
mcp_exposable
```

Sem a capability correspondente, a operação não existe para o plugin (sem filesystem,
sem rede). Um tema puramente declarativo **não** recebe capability de execução.

## 3. Plugin Panels (Petunia UI Extension API)

Community Plugins podem registrar **painéis completos**, mas apenas compondo
componentes públicos do Petunia:

- **Regiões V1**: `left`, `right`, `bottom`. Substituir o centro/viewport e janelas
  flutuantes arbitrárias **não** fazem parte da Community Plugin API V1.
- O descriptor do painel declara `panel_id`, `plugin_id`, `title`, `icon`,
  `preferred_region`, `allowed_regions`, `minimum_size`, `preferred_size`,
  `singleton`, `visible_by_default`, `context_requirements` e `help/manual id`.
- O painel **herda o tema atual** e não escolhe RGB/hex local: usa variants
  (`normal`, `accent`, `success`, `warning`, `danger`, `muted`).
- O painel **não varre o documento a cada frame**: assina eventos após commit,
  atualiza seu estado namespaced (`plugin_id + panel_id`) e invalida o repaint.
- Acessibilidade (focus ring, keyboard activation, semantics) é garantida pelo host e
  **não pode** ser desabilitada pelo plugin.

## 4. Isolamento e falhas

- Um **Lua State por plugin**.
- Erros de render ficam confinados ao painel; falhas repetidas podem suspender apenas
  aquele painel/plugin.
- Budgets de instrução/memória valem também durante callbacks de UI.
- **Nenhuma mutação do documento durante render** — ações produzem Commands/eventos.
- `unload` remove painéis, commands, subscriptions e estado associado, sem deixar
  referências quebradas.

## 5. Módulos internos

Os módulos do produto (`module-model`, `module-paint`, `module-uv`,
`module-assets`) são parte do core interno e seguem a cadeia funcional canônica:

```
Tool → Command → Algorithm → Data
```

Regras: `Tool` não chama `Tool`; algorithms ficam independentes de UI/Undo/Lua/MCP
sempre que possível; módulos conversam por contratos estáveis e não conhecem uns aos
outros. O `ModuleRegistry` do core distribui **eventos** (`AppEvent`) — ele não
desenha interface e não conhece `egui`.

> Nota de correção: documentações antigas descreviam um trait `Module` com
> `draw_ui(&egui::Context)`. Isso contradiz a regra absoluta de que o core não depende
> de egui e não reflete o código atual. Painéis de UI vivem na camada `ui`/adapter.

## 6. MCP

`petunia-mcp` é um **adapter sobre a Application API** (Rust, sobre `rmcp`, com Tokio
isolado no serviço):

- não controla a UI por automação de cliques como caminho principal;
- expõe *tools* semânticas com schemas estruturados;
- não oferece execução arbitrária de código como capacidade padrão;
- respeita capabilities/permissões;
- operações editáveis geram transactions/Undo;
- plugins só aparecem no MCP quando declarados exponíveis e autorizados.
