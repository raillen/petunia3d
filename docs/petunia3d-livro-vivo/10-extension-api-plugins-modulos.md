# 10 — Extension API, Plugins e Módulos Oficiais

> A extensibilidade faz parte da arquitetura desde a V1, mas a primeira API pública será pequena e guiada por necessidades reais. O objetivo é manter o Petunia3D enxuto sem impor um teto baixo para ferramentas avançadas.

# Modelo de extensibilidade

A extensibilidade deve obedecer a [34 — Arquitetura Modular Explícita, Rust Safety e Representação em Código](34-arquitetura-modular-rust-safety.md). Em especial, **modularidade não significa trait/framework para tudo**: módulos oficiais começam como composição/registro explícito; plugins são runtime extensions; providers usam traits apenas quando substituição real existe; Tools continuam clientes de Commands.

Existem três categorias funcionais:

1. **Core** — conceitos universais, sempre disponíveis.
2. **Official Extensions** — módulos mantidos junto ao Petunia, instalados por padrão quando fizerem parte da experiência principal, mas implementados através de contratos públicos ou semi-públicos sempre que isso for prático.
3. **Community Plugins** — capacidades opcionais, especializadas ou experimentais.

Para o usuário comum, um módulo oficial pode parecer parte nativa do aplicativo. A separação é arquitetural.

# O que não será plugin

Não transformar o Petunia em um microkernel no qual até ações básicas dependam de plugins. Permanecem no Core:

- seleção;
- documento;
- transformação;
- representação mesh/topology;
- profiles/work planes;
- Extrude básico;
- Push/Pull local;
- UV representation;
- materiais/imagens essenciais;
- Undo/Redo e transactions;
- command system.

# Extension points prioritários

A API deve permitir extensões em áreas com alto valor e baixo acoplamento:

- Geometry operations.
- Geometry providers especializados.
- Primitives adicionais.
- UV unwrap/packing providers.
- Validators.
- Importers/exporters/codecs.
- Asset generators.
- Viewport overlays.
- Commands e ações de UI.
- Painéis contextuais limitados.
- Integrações com engines/pipelines.

# Estrutura conceitual da API

Namespaces sugeridos, não comprometidos com sintaxe de uma linguagem específica:

```plain text
petunia.document
petunia.scene
petunia.selection
petunia.mesh
petunia.profile
petunia.geometry
petunia.uv
petunia.texture
petunia.material
petunia.viewport
petunia.commands
petunia.history
petunia.validation
petunia.io
petunia.ui
```

Esses namespaces representam domínios. A API não deve vazar detalhes internos como layout de arrays, ponteiros de half-edge ou estruturas privadas do renderer.

# Handles e snapshots

Plugins recebem **IDs persistentes para entidades de documento** e **handles generacionais opacos para componentes topológicos**, além de estruturas de leitura controladas. Handles de Vertex/Edge/Face podem tornar-se stale após edits destrutivos; toda chamada os valida e retorna erro estruturado quando necessário. Para operações complexas, plugins podem solicitar snapshots de geometria em formato documentado, produzir um resultado e submetê-lo pela Application API.

Nenhum plugin deve reter referências cruas a memória interna entre frames ou transactions.

# Transactions obrigatórias

Toda mutação de plugin ocorre dentro de transaction. Regras:

- Iniciar com nome legível para Undo.
- Permitir preview sem commit.
- Validar antes de confirmar.
- Rollback automático em exceção/erro.
- Uma transaction bem-sucedida gera uma única ação de Undo, salvo quando explicitamente composta.

# Capabilities e permissões

Plugins devem declarar capacidades necessárias, por exemplo:

```plain text
read_document
edit_geometry
edit_uv
edit_materials
import_files
export_files
register_ui_panel
register_viewport_overlay
register_commands
```

A instalação e execução não devem conceder poderes implícitos desnecessários.

# Linguagem pública de plugins

A API pública de plugins de terceiros será **Lua 5.4**, hospedada na baseline Rust por **`mlua`**. Essa decisão preserva baixo custo de integração, distribuição simples e uma superfície de segurança/controlabilidade melhor do que expor um ABI nativo público desde o início.

Regras:

- Community Plugins usam Lua como linguagem oficial.
- A API Lua é um binding da Application API; ela não acessa estruturas internas diretamente.
- Bibliotecas nativas usadas pelo próprio Petunia ou por Official Extensions são módulos internos/providers, não fazem parte do ABI público de plugins inicialmente.
- Não prometer ABI binário estável para extensões nativas na V1.
- Se no futuro surgir necessidade real de plugins nativos de terceiros, isso será uma decisão arquitetural separada e não uma consequência automática do sistema Lua.

Essa separação mantém o ecossistema simples: **Lua para extensibilidade pública; traits/providers internos para dependências especializadas de geometry/UV/rendering.** A implementação Rust não altera a regra de que community plugins não recebem acesso cru às estruturas internas.

# Registro de módulos oficiais

Módulos oficiais não precisam de um framework abstrato próprio na primeira implementação. Preferir composition root explícito:

```rust
modeling::register(&mut application)?;
painting::register(&mut application)?;
uv::register(&mut application)?;
export::register(&mut application)?;
```

Cada módulo pode registrar commands, Tools, validators, importers/exporters e contributions de UI aprovadas. Introduzir `Module` trait/lifecycle genérico somente quando existir uma necessidade real de múltiplas implementações/dynamic loading.

Desabilitar um módulo opcional deve desregistrar suas contributions e produzir capability unavailable estruturado quando uma integração tentar chamá-lo; nunca deixar símbolo/referência quebrada.

# Lifecycle mínimo

Um plugin precisa de um ciclo de vida pequeno e previsível:

```plain text
manifest
→ load
→ register capabilities / commands / providers
→ enable
→ disable
→ unload
```

Evitar callbacks globais excessivos. Preferir eventos de domínio e subscriptions explícitas.

# Manifest conceitual

```plain text
id
name
version
petunia_api_version
entrypoint
capabilities
optional_dependencies
mcp_exposable_commands
```

O formato concreto **já está definido na baseline Rust**: `plugin.toml` dentro do container ZIP versionado `.petunia-plugin`. Os campos acima formam o contrato mínimo do manifest; extensões futuras devem evoluir por versionamento/capability detection, não por formatos paralelos.

# Versionamento

Durante Petunia 0.x, a Extension API pode evoluir com breaking changes documentadas. A meta é chegar a `Extension API 1.0` apenas após os módulos oficiais exercitarem seus principais contratos.

Preferir versionamento explícito e feature/capability detection em vez de assumir que toda instalação possui todas as extensões.

# Módulos oficiais candidatos

Capacidades que podem ser fornecidas como módulos oficiais sem poluir o core:

- Boolean provider robusto usado por Fuse/Cut quando necessário.
- Simple Sweep.
- Advanced UV / unwrap providers.
- Game Ready Validator.
- Exportadores adicionais.
- Photo Projection avançada.

Um módulo oficial crítico para a experiência padrão deve vir habilitado por padrão; o usuário não precisa instalar manualmente algo para fazer `Fuse` ou `Cut` se essas ações aparecem na interface principal.

# Plugins de comunidade esperados

Exemplos de extensões que não justificam entrar no núcleo:

- geradores de árvore/rocha/terreno;
- Auto LOD;
- kits PS1 específicos;
- ferramentas de arquitetura especializadas;
- exporters de engines específicas;
- operações geométricas experimentais;
- retopology/remesh opcionais;
- loft ou sweep avançados.

# Integração com UI

Plugins registram commands e podem, quando autorizados, registrar **Plugin Panels** completos através da `Petunia UI Extension API`. A UI continua responsável por consistência visual, accessibility, keyboard/focus, tema e layout estrutural.

Preferir para operações simples:

```plain text
Command + metadata
→ Petunia escolhe apresentação coerente
```

Quando um workflow realmente exige superfície própria:

```plain text
Plugin Panel descriptor
→ allowed extension slot
→ Petunia UI Builder
→ Petunia Components
→ current theme + AccessKit semantics
```

Community Plugins **não recebem `egui::Ui`, wgpu, raw input, custom Painter ou markup livre**. A API pública de painel expõe componentes estáveis do Petunia, state namespaced, regiões controladas (`left`, `right`, `bottom`) e actions que retornam ao Command Registry/Application API. O contrato completo, incluindo themes e panel lifecycle, está no capítulo 36.

Temas são uma superfície separada e **puramente declarativa** (`.petunia-theme`) na V1. Um plugin funcional pode bundlar um theme package, mas resolução de tokens nunca executa Lua nem qualquer outro código.

# Packaging e sandbox de plugins

Community Plugins são distribuídos como **`.petunia-plugin`**, um ZIP com `plugin.toml`, arquivos Lua e assets opcionais.

Na baseline Rust, `plugin.toml` é preferido por legibilidade e integração simples com tooling Rust, mas continua sendo um formato versionado/validado do Petunia — não uma estrutura arbitrária que plugins podem interpretar livremente.

Manifest mínimo:

```plain text
id
name
version
petunia_api_version
entrypoint
capabilities
optional_dependencies
mcp_exposable_commands
```

Cada plugin roda em **um Lua State próprio**, permitindo unload, budget e isolamento melhores que um estado global compartilhado.

## Bibliotecas Lua

Por padrão, o host não expõe `os`, `io`, `debug` nem carregamento de bibliotecas nativas (`package.loadlib`). A API de filesystem, quando necessária, é fornecida pelo Petunia e limitada por capability/root autorizada. Networking não faz parte da API pública V1.

## Budgets

O host pode impor:

- limite de memória por Lua State através do allocator;
- cancelamento/cooperative instruction hook para scripts travados;
- limite de profundidade/recursão quando aplicável;
- diagnóstico de plugin sem encerrar o processo principal.

Um erro Lua durante mutação causa rollback da transaction.

# Command Registry como fonte da API

Evitar manter três APIs manuais divergentes. Commands públicos registram metadata estruturada:

```plain text
command_id
version
parameters
result
required_capabilities
undoable
previewable
mcp_exposable
lua_exposable
```

A Lua API e o catálogo MCP devem ser derivados dessa metadata sempre que possível. Bindings especializados só existem quando o domínio exige uma representação mais eficiente.

# Eventos

Plugins assinam eventos de domínio **pós-commit** por padrão (`document_changed`, `selection_changed`, `asset_exported` etc.). Callbacks antes do commit só existem para extension points explicitamente projetados, evitando reentrância imprevisível.

# Segurança de providers

Community Plugins Lua podem registrar commands e generators, mas **não substituem diretamente providers nativos críticos** como Boolean/UV backend na V1. Providers nativos são Official Extensions compiladas e versionadas junto ao aplicativo. Isso evita ABI/plugin nativo instável e reduz superfície de crash.

# Regra contra bloat

Uma funcionalidade avançada pode existir no ecossistema sem precisar virar parte permanente da interface principal. A Extension API é a válvula de expansão do Petunia3D.

A implementação deve evitar `Manager/Service/Repository/Factory` genéricos, module frameworks prematuros e Cargo features como sistema principal de módulos. Cargo features representam capabilities aditivas de build; composição funcional pertence ao composition root/Extension Host.
