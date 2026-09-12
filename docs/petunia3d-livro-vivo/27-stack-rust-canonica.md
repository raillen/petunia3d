# 27 — Stack Rust Canônica: Rust + egui + wgpu

> Esta página define a **technical baseline final do Petunia3D** para implementação: **Rust 2024 + egui + eframe + egui-wgpu + wgpu + Geometry Core próprio em Rust**. A stack substitui a baseline anterior centrada em Odin e adota uma UI Rust-first permissiva, diretamente alinhada ao renderer wgpu. O vertical slice técnico do capítulo 31 é agora um **teste de conformance/integração**; reabrir a stack exige bloqueador estrutural comprovado + ADR explícito.

# Objetivo da stack

A stack deve permitir que um único desenvolvedor, fortemente assistido por agentes de IA, construa e mantenha um modelador low-poly desktop com:

- interface altamente customizada e fiel ao Design System Petunia;
- viewport 3D moderno e cross-platform;
- Geometry Core previsível e seguro;
- modelagem direta e profile-based;
- UV automático e projeção de referência;
- pintura simples sobre textura;
- Undo/Redo transacional;
- formato de projeto versionado;
- plugins Lua;
- MCP nativo;
- testes automatizados extensos;
- Windows e Linux como targets obrigatórios iniciais.

A arquitetura deve minimizar glue entre linguagens. **Rust é a linguagem de primeira classe de quase todo o executável. Lua permanece a linguagem pública de plugins.** A única dependência estrutural que pode continuar usando implementação C/C++ por baixo é xatlas, isolada atrás de provider.

# Licença do projeto e política de dependências

**Petunia3D será licenciado sob MIT.** Isso é uma restrição arquitetural, não apenas uma decisão de publicação.

Sempre que houver alternativas tecnicamente equivalentes, preferir dependências sob:

```plain text
MIT
Apache-2.0
MIT OR Apache-2.0
BSD-2-Clause
BSD-3-Clause
ou licença permissiva equivalente explicitamente aprovada
```

Licenças customizadas, copyleft forte, source-available ou restrições comerciais exigem decisão arquitetural explícita antes da adoção.

`cargo-deny`/tooling equivalente deve validar a allowlist de licenças no CI.

# Stack resumida

<table>
<tr><td>Área</td><td>Baseline</td><td>Papel</td></tr>
<tr><td>Linguagem</td><td>Rust 2024 Edition</td><td>Aplicação, core, geometry, renderer, I/O, plugins host, MCP.</td></tr>
<tr><td>Build</td><td>Cargo workspace</td><td>Build, testes, dependências, tooling.</td></tr>
<tr><td>UI</td><td>egui 0.36.x</td><td>Immediate-mode UI, input, layout, painting, accessibility tree e shell desktop.</td></tr>
<tr><td>Desktop host</td><td>eframe 0.36.x</td><td>Window/app integration e execução desktop.</td></tr>
<tr><td>UI ↔ GPU</td><td>egui-wgpu 0.36.x</td><td>Integração oficial egui/wgpu e custom rendering dentro de regiões da UI.</td></tr>
<tr><td>GPU</td><td>wgpu 30.x</td><td>Viewport, overlays, gizmos e uploads de textura.</td></tr>
<tr><td>Shaders</td><td>WGSL</td><td>Shaders únicos cross-backend.</td></tr>
<tr><td>Math</td><td>glam</td><td>Vec2/Vec3/Quat/Mat4 e matemática 2D/3D.</td></tr>
<tr><td>IDs</td><td>slotmap</td><td>Handles generacionais tipados.</td></tr>
<tr><td>2D Geometry</td><td>geo</td><td>Profile operations e polygon predicates/booleans quando apropriado.</td></tr>
<tr><td>3D Boolean</td><td>manifold-rust</td><td>Fuse/Cut complexo.</td></tr>
<tr><td>Auto UV</td><td>Petunia native + xatlas fallback</td><td>UV determinística/projeção + unwrap genérico.</td></tr>
<tr><td>Picking</td><td>ray/triangle próprio inicialmente</td><td>Seleção e Paint on Model; BVH somente se profiling justificar.</td></tr>
<tr><td>Images</td><td>image com codecs explícitos</td><td>Decode/encode de texturas/referências.</td></tr>
<tr><td>Painting</td><td>TextureBitmap próprio</td><td>Buffer CPU, Undo e dirty regions.</td></tr>
<tr><td>glTF/GLB</td><td>gltf + gltf-json</td><td>Import/export principal.</td></tr>
<tr><td>OBJ</td><td>writer pequeno próprio + tobj para import</td><td>Formato secundário; export controlado pelo Petunia e import atrás de Importer.</td></tr>
<tr><td>Serialize</td><td>Serde + serde_json</td><td>Document/project data.</td></tr>
<tr><td>Container</td><td>zip</td><td>Arquivo de projeto versionado.</td></tr>
<tr><td>Plugins</td><td>Lua 5.4 + mlua</td><td>Extension API pública.</td></tr>
<tr><td>MCP</td><td>rmcp + Tokio isolado</td><td>Servidor MCP Rust nativo.</td></tr>
<tr><td>CPU jobs</td><td>Rayon</td><td>Boolean/UV/validation/export work.</td></tr>
<tr><td>Channels</td><td>flume</td><td>Comunicação service/worker → single writer.</td></tr>
<tr><td>Errors</td><td>thiserror; anyhow nas bordas</td><td>Erros de domínio tipados + contexto de app.</td></tr>
<tr><td>Logging</td><td>tracing</td><td>Observabilidade estruturada.</td></tr>
<tr><td>Schemas</td><td>schemars + Serde</td><td>Command/MCP/plugin schemas.</td></tr>
<tr><td>Property tests</td><td>proptest</td><td>Invariantes topológicas e commands.</td></tr>
<tr><td>Benchmarks</td><td>Criterion</td><td>Performance baseada em fixtures reais.</td></tr>
<tr><td>Fuzzing</td><td>cargo-fuzz</td><td>Loaders, geometry boundaries e parsers.</td></tr>
</table>

# Por que egui

A UI do Petunia é uma ferramenta técnica/dcc-like com grande quantidade de controls contextuais, viewport próprio e forte integração com input. egui se encaixa nesse perfil porque:

- é Rust-first e permissivo (`MIT OR Apache-2.0`);
- integra diretamente com wgpu pelo `egui-wgpu` oficial;
- permite custom rendering em uma região da UI, reduzindo glue de viewport;
- possui AccessKit como base de accessibility/inspection;
- possui ecossistema ativo de widgets para tree, tiling, tables, dialogs e testing;
- permite construir o Design System Petunia em uma camada própria sem vazar o toolkit para o core;
- é usado em aplicações profissionais/visualizers Rust com renderer próprio, oferecendo referência arquitetural real.

Immediate mode não autoriza misturar estado de domínio com estado de apresentação. O Document continua single-writer e a UI somente observa view models/snapshots e dispara Commands.

# UI como camada própria

A regra de dependência é:

```plain text
Petunia workspaces/screens
        ↓
Petunia Components
        ↓
Petunia UI adapters
        ↓
egui / eframe / selected egui crates
```

Não espalhar chamadas cruas de egui por todo o projeto. **A UI de produto deve passar por Petunia Components/adapters como regra arquitetural**; egui cru fica restrito à implementação desses boundaries e a devtools explicitamente delimitadas. Componentes próprios incluem:

```plain text
PetuniaButton
PetuniaIconButton
PetuniaSplitButton
PetuniaSegmentedControl
PetuniaNumberField
PetuniaPanel
PetuniaPanelHeader
PetuniaContextSection
PetuniaTree
PetuniaAssetCard
PetuniaWorkspacePill
PetuniaTooltip
PetuniaToolbar
PetuniaModal
```

Widgets externos complexos entram atrás de adapters próprios, preservando substituibilidade.

# Design System

O Design System Petunia é autoridade visual. egui fornece interação/layout/semântica; não define a identidade do produto.

A camada `foundation` deve centralizar:

```plain text
colors
typography
spacing
radii
strokes
metrics
motion
icons
interaction states
focus visuals
```

`egui::Style`, `Visuals`, `Spacing`, `WidgetVisuals` e custom painting são configurados a partir desses tokens.

# Viewport: integração direta egui-wgpu

A arquitetura preferida do viewport é:

```plain text
egui layout
    ↓ allocate viewport Rect
Petunia viewport widget/adapter
    ↓ egui-wgpu custom callback
PetuniaRenderer
    ↓ wgpu RenderPass / resources
GPU
```

O renderer continua em `petunia-render`; `petunia-ui` apenas fornece o retângulo, input/view state e adapter de integração.

Não fazer o Geometry Core depender de egui. Não fazer `PetuniaRenderer` depender de objetos de estado mutável da UI além de um adapter estreito na fronteira de renderização.

# Accessibility

Accessibility usa a árvore AccessKit exposta pelo egui/host.

Todo componente Petunia customizado deve publicar semântica apropriada quando aplicável:

```plain text
role
accessible name
description
enabled/disabled
selected/checked/expanded
keyboard activation
focus state
```

Uma customização visual não pode eliminar a semântica do controle.

# Testabilidade da UI

UI é código testável. A baseline de desenvolvimento deve usar:

- `egui_kittest` para semantic queries, interaction tests e render/snapshot tests;
- `egui_inspection` em builds de desenvolvimento para inspeção externa da árvore/interface;
- `egui_mcp` em ambiente de desenvolvimento para permitir agentes inspecionarem, interagirem e capturarem screenshots;
- visual regression com tolerância explicitamente controlada.

Serviços de inspection/MCP de UI ficam **desabilitados em builds de distribuição** e, quando ativos, restritos a loopback/configuração de desenvolvimento.

# Ecossistema UI inicial

A lista abaixo **não é uma lista de dependências automáticas do Cargo**. O capítulo 35 é a autoridade para classificação detalhada e adapters.

## BASELINE

- `egui`;
- `eframe`;
- `egui-wgpu`;
- `egui_extras`.

## BASELINE

- `egui_ltreeview` para Parts tree, atrás de `PetuniaTreeAdapter`;
- `egui_tiles` para engine de layout/painéis controlados, atrás de `PetuniaLayoutAdapter`;
- `egui-lucide` como pack genérico principal de ícones;

## OPTIONAL / REFERENCE

- `egui-file-dialog` permanece optional; fluxo padrão usa dialogs nativos do SO;
- `egui-phosphor` permanece REFERENCE/MONITOR.

## DEV TOOL

- `egui_kittest` — obrigatório em dev/tests para semântica, interação e snapshots;
- `egui_inspection` — somente development build;
- `egui_mcp` — somente development build e acesso local/loopback;
- `egui_json_tree`, `egui_probe`, `puffin_egui` e tooling semelhante conforme necessidade de diagnóstico.

## OPTIONAL / FUTURE

- `egui_animation`;
- `egui_form`;
- `egui_dnd`;
- `egui_commonmark`;
- `egui_table`;
- `egui-data-table`;
- `egui-snarl`.

Itens `REFERENCE/MONITOR` ou `REJECTED/NOT RELEVANT` não entram no baseline e são catalogados no capítulo 35.

Detalhes, critérios, wrappers e promoção entre categorias são normatizados no capítulo específico do ecossistema egui.

# Política para crates auxiliares de UI

Uma crate de UI só entra se resolver interação/infraestrutura não trivial que seria custo real reproduzir.

Antes de adicionar:

1. licença permissiva compatível com a política MIT do projeto;
2. compatibilidade com a linha egui pinada;
3. atividade/manutenção;
4. targets Windows/Linux;
5. accessibility behavior quando relevante;
6. impacto de build/dependências;
7. possibilidade de isolamento atrás de adapter;
8. snapshot/interaction tests próprios do Petunia.

Uma biblioteca de componentes nunca se torna autoridade visual do Petunia.

# Rust como linguagem central

Rust deverá ser usado para:

- Application Core;
- Document Model;
- Command Registry;
- Geometry Core;
- Renderer;
- caches;
- UI/application presentation;
- I/O;
- import/export orchestration;
- plugin host;
- MCP adapter/server;
- build tooling/xtask.

Não criar um core Rust e depois espalhar C++, Go ou outra linguagem pela aplicação sem uma necessidade comprovada.

# Política de unsafe

Regra default para crates de domínio:

```rust
#![forbid(unsafe_code)]
```

Aplicar onde viável em `petunia-core`, `petunia-geometry`, `petunia-io`, lógica de plugins e MCP. Dependências externas podem internamente utilizar unsafe, mas isso não autoriza unsafe arbitrário no nosso código.

Se uma integração nativa realmente exigir unsafe, concentrá-la em módulo/crate de fronteira pequeno, documentado e testado.

# Dependências deliberadamente fora da baseline

- Bevy/engine completa;
- ECS genérico;
- Skia para painting V1;
- CAD/B-Rep/OpenCascade;
- Assimp como importer universal;
- banco de dados para o documento;
- engine de render offline;
- runtime web/CEF;
- Tokio dentro do Geometry/Application Core;
- remesh/retopology automática;
- segundo toolkit UI concorrente no build normal.

# Targets de plataforma

Baseline obrigatória:

- Windows x86_64;
- Linux x86_64, com Wayland como cenário de teste importante.

macOS deve permanecer possível arquiteturalmente, mas não precisa bloquear o primeiro milestone se não houver infraestrutura de CI/teste suficiente.

# Cargo workspace — baseline orientativa

O workspace real deve pinar versões compatíveis e versionar `Cargo.lock`.

```toml
[workspace]
resolver = "2"

[workspace.package]
edition = "2024"

[workspace.dependencies]
# UI / GPU
egui = "0.36"
eframe = { version = "0.36", default-features = false, features = ["wgpu"] }
egui-wgpu = "0.36"
egui_extras = "0.36"
wgpu = "30"
bytemuck = { version = "1", features = ["derive"] }

# Domain / math / IDs
glam = { version = "0", features = ["serde"] }
slotmap = { version = "1", features = ["serde"] }
smallvec = { version = "1", features = ["serde"] }
indexmap = { version = "2", features = ["serde"] }
uuid = { version = "1", features = ["serde", "v4"] }

# Geometry
geo = "0.33"
manifold-rust = "0.13"
xatlas_rs_v2 = "0.1"

# Data / I/O
serde = { version = "1", features = ["derive"] }
serde_json = "1"
schemars = "1"
zip = "8"
image = { version = "0.25", default-features = false, features = ["png", "jpeg"] }

# Plugins
mlua = { version = "0.12", features = ["lua54", "vendored", "serde"] }

# Runtime services
rayon = "1"
flume = "0"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
rmcp = "<pin compatible>"

# Diagnostics
thiserror = "2"
anyhow = "1"
tracing = "0.1"
```

Crates auxiliares egui devem ser adicionadas somente quando a feature correspondente entrar no vertical slice/produto e sempre pinadas na linha compatível com egui 0.36.x.

# Política de version/update

Updates de dependência exigem:

```plain text
read release notes
→ verify egui/eframe/egui-wgpu/wgpu compatibility
→ cargo check/test/clippy
→ provider contract tests
→ UI semantic/snapshot tests
→ viewport GPU tests
→ license/security check
→ update Cargo.lock
→ record material delta
```

Não fazer bulk update cego em dependencies centrais.

# Toolchain Rust

Versionar `rust-toolchain.toml`. Preferir stable Rust compatível com as crates pinadas. Nightly só pode ser introduzido para ferramenta isolada e nunca como requisito silencioso do aplicativo.

# Build profiles

Manter pelo menos:

```plain text
dev       → debug rápido e assertions/invariants
release   → distribuição
profiling → símbolos/instrumentation suficiente para medir hotspots
```

# Princípio final

**Uma linguagem principal, UI Rust-first permissiva, renderer wgpu próprio, fronteiras estreitas e bibliotecas especializadas apenas para problemas que não criam identidade ao Petunia.**
