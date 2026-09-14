# 34 — Arquitetura Modular Explícita, Rust Safety e Representação em Código

> Este capítulo é **normativo para a implementação Rust**. Ele define como representar entidades, recursos, topologia, funcionalidades e fronteiras do Petunia3D em código. A meta é maximizar simplicidade, Rust safety, modularidade real, interoperabilidade entre ferramentas e capacidade de evolução sem transformar o projeto em uma hierarquia de abstrações ou em um ECS universal.

# Filosofia arquitetural

A arquitetura oficial deste capítulo é chamada de **Explicit Modular Data Architecture**.

Ela combina:

- dados de domínio explícitos;
- ownership explícito e borrows curtos;
- **Data-Oriented Design seletivo** onde layout/fluxo de dados realmente importam;
- composição inspirada em ECS onde entidades possuem capacidades combináveis;
- **Command Architecture** para casos de uso;
- Ports/Adapters nas fronteiras reais;
- single-writer para estado mutável principal;
- snapshots imutáveis para renderer/jobs;
- tipos Rust para tornar estados inválidos difíceis de representar;
- SOLID reinterpretado para Rust, sem reproduzir padrões de OO/Java;
- Clean Code orientado a clareza, nomes de domínio e fluxo explícito.

A diretriz central é:

> **Data-oriented where data wants it. Domain-oriented where the problem wants it.**

Não adotar uma única técnica como resposta para todos os subsistemas.

# Regra de simplicidade

A ordem de preferência arquitetural é:

```plain text
1. concrete data
2. explicit ownership
3. simple functions
4. cohesive modules
5. typed commands
6. traits only at real boundaries
7. dynamic dispatch only where dynamism exists
8. concurrency only where work benefits from it
9. unsafe only behind tiny audited boundaries
10. abstraction only after repetition/variation is real
```

**Desacoplável não significa que tudo depende de interfaces.** Em Rust, tipos concretos, módulos privados, enums, ownership e functions puras frequentemente produzem menos acoplamento que camadas artificiais de traits/managers/services.

# Quatro estilos internos, não um único framework

<table>
<tr><td>Área</td><td>Representação predominante</td><td>Rationale</td></tr>
<tr><td>Document / Scene</td><td>Entidades e recursos explícitos + stores tipados</td><td>Relacionamentos legíveis, persistência simples e sem necessidade de scheduler ECS.</td></tr>
<tr><td>Geometry Core</td><td>Half-edge própria + stores/data-oriented attributes</td><td>Topologia exige estrutura especializada e operações locais eficientes.</td></tr>
<tr><td>Renderer / Painting</td><td>Buffers contíguos, snapshots e dirty regions</td><td>Hot paths beneficiam cache locality e batch processing.</td></tr>
<tr><td>Funcionalidades</td><td>Algorithm → Command → Tool/Adapter</td><td>Permite reutilização pela UI, Lua, MCP e testes sem dependência entre ferramentas interativas.</td></tr>
</table>

# Política de ECS

## Decisão

**Não usar Bevy ECS nem qualquer ECS generalista como arquitetura universal do Petunia3D.**

O Petunia pode adotar ideias úteis de ECS — composição por capacidades, stores separados, processamento em lote, IDs estáveis e data locality — sem transformar Document, Geometry, Undo, plugins, renderer e ferramentas em components/systems genéricos.

## Por que não ECS universal

Um modelador 3D possui domínios com invariantes muito diferentes:

- SceneObject possui relações de documento;
- Mesh possui topologia;
- Vertex/Edge/Face possuem lifecycle topológico;
- Texture possui storage raster;
- Brush é comportamento interativo;
- Command é caso de uso;
- Exporter/BooleanProvider são adapters/providers.

Forçar tudo para `Entity + Component + System` esconderia essas diferenças e aumentaria custo de debug, persistência e compreensão por agentes.

## Onde inspiração ECS é permitida

- stores de atributos separados;
- iteração por dados homogêneos;
- capacidades opcionais em entidades de cena;
- snapshots read-only preparados para processamento;
- processamento em lote no renderer/validation;
- IDs em vez de grafos de referências Rust de longa duração.

# Taxonomia de coisas do Petunia

<table>
<tr><td>Conceito</td><td>Categoria arquitetural</td><td>Exemplo de representação</td></tr>
<tr><td>Objeto/parte na cena</td><td>Document Entity</td><td>`SceneObject` identificado por `ObjectId`</td></tr>
<tr><td>Mesh</td><td>Resource</td><td>`PetuniaMesh` em `MeshStore`</td></tr>
<tr><td>Profile</td><td>Resource</td><td>`Profile` em `ProfileStore`</td></tr>
<tr><td>Material</td><td>Resource</td><td>`Material` em `MaterialStore`</td></tr>
<tr><td>Texture</td><td>Resource</td><td>`TextureAsset` / `TextureBitmap`</td></tr>
<tr><td>Reference Image</td><td>Resource + document placement</td><td>`ReferenceImage` • `ReferenceView`</td></tr>
<tr><td>Vertex/Edge/Face/HalfEdge</td><td>Topology Element</td><td>generational handle mesh-scoped</td></tr>
<tr><td>Extrude</td><td>Geometry Algorithm + Application Command</td><td>`extrude_faces` • `ExtrudeCommand`</td></tr>
<tr><td>Extrude com mouse</td><td>Interactive Tool</td><td>`ExtrudeTool`</td></tr>
<tr><td>Boolean</td><td>Provider boundary</td><td>`BooleanProvider`</td></tr>
<tr><td>GLB</td><td>I/O Adapter</td><td>`GltfExporter`</td></tr>
<tr><td>Lua plugin</td><td>Runtime Extension</td><td>`PluginHost` • command registrations</td></tr>
<tr><td>MCP</td><td>External Adapter</td><td>MCP ↔ Command Registry</td></tr>
</table>

Nunca adotar a regra “tudo é entidade”. Cada conceito deve receber a representação mais simples que preserve suas invariantes.

# Document Model simples

O `Document` deve ser um aggregate explícito, não um service locator nem um ECS World.

Exemplo conceitual:

```rust
pub struct Document {
    objects: ObjectStore,
    meshes: MeshStore,
    profiles: ProfileStore,
    materials: MaterialStore,
    textures: TextureStore,
    references: ReferenceStore,
    metadata: DocumentMetadata,
    revision: DocumentRevision,
}
```

Um objeto de cena guarda identidade e referências por ID:

```rust
pub struct SceneObject {
    id: ObjectId,
    name: ObjectName,
    transform: Transform,
    visibility: Visibility,
    geometry: Option<GeometryReference>,
    parent: Option<ObjectId>,
}
```

Evitar `EntityManager`, `ComponentManager`, `WorldContext`, `ServiceContainer` ou equivalentes enquanto não existir um problema concreto que justifique essas estruturas.

# Entidades persistentes versus topology handles

## Entidades persistentes

`ObjectId`, `MeshId`, `ProfileId`, `MaterialId`, `TextureId`, `ReferenceId` representam identidades do documento e devem possuir representação persistente independente do slot runtime, preferencialmente UUID/newtype.

## Handles topológicos

`VertexId`, `HalfEdgeId`, `EdgeId`, `FaceId` são **handles generacionais escopados à mesh**. Eles permanecem válidos apenas enquanto o elemento existir e a operação preservar sua identidade.

Uma reconstrução destrutiva pode invalidá-los. APIs públicas devem detectar stale handles e retornar erro estruturado, nunca reutilizar silenciosamente um índice antigo.

Long-lived relationships usam IDs. Referências Rust `&T`/`&mut T` ficam locais à operação.

# Newtypes como contrato

Usar newtypes para impedir mistura acidental de conceitos semanticamente diferentes:

```rust
pub struct ObjectId(Uuid);
pub struct MeshId(Uuid);
pub struct DocumentRevision(u64);
pub struct BevelWidth(f32);
pub struct TextureWidth(u32);
pub struct TextureHeight(u32);
```

Não passar `Uuid`, `u64`, `f32` ou `u32` crus quando a unidade/conceito altera a correção da API.

Newtypes também facilitam validation, serialization e documentação de invariantes.

# Geometry Core orientado a dados

`PetuniaMesh` continua sendo implementação própria, especializada para authoring polygonal/half-edge.

Representação conceitual:

```rust
pub struct PetuniaMesh {
    vertices: SlotMap<VertexId, VertexTopology>,
    half_edges: SlotMap<HalfEdgeId, HalfEdgeTopology>,
    edges: SlotMap<EdgeId, EdgeTopology>,
    faces: SlotMap<FaceId, FaceTopology>,
    attributes: MeshAttributes,
    revision: MeshRevision,
}
```

## Separar conectividade de atributos

Sempre que isso simplificar lifecycle e locality, atributos ficam em stores separados:

```plain text
Topology
Vertex → outgoing HalfEdge
HalfEdge → origin/twin/next/previous/face/edge
Face → boundary HalfEdge

Attributes
Vertex → Position
Corner/HalfEdge → UV
Edge → Sharp flag
Face → MaterialId
```

Não transformar `Vertex` em um struct enorme contendo todas as propriedades possíveis apenas por conveniência inicial.

## Regra de DOD

Aplicar Data-Oriented Design quando houver processamento em lote ou hot path comprovado. Não converter estruturas de domínio para SoA apenas por princípio. Measure first quando a escolha for principalmente de performance.

# Representação derivada para rendering

Renderer não deve navegar a half-edge mesh viva.

```plain text
PetuniaMesh
    ↓ triangulation/cache extraction
RenderSnapshot
    ↓
contiguous vertices / indices / materials
    ↓
wgpu buffers
```

O renderer trabalha sobre estruturas preparadas para leitura/batch. IDs de origem necessários para picking/diagnostics são armazenados no mapping derivado, por exemplo `triangle → FaceId`.

# Arquitetura fundamental de funcionalidades

A regra normativa é:

```plain text
INPUT
  ↓
TOOL
interactive state only
  ↓
COMMAND
application use-case
  ↓
ALGORITHM
pure/domain operation whenever possible
  ↓
DATA
Document / PetuniaMesh / Texture
```

## Algorithm

Uma operação de domínio não conhece egui, eframe, MCP, Lua, Undo, menus ou input devices.

Exemplo:

```rust
pub fn extrude_faces(
    mesh: &mut PetuniaMesh,
    face_ids: &[FaceId],
    parameters: &ExtrudeParameters,
) -> Result<ExtrudeResult, ExtrudeError>;
```

## Command

O command representa um caso de uso da aplicação e orquestra:

- resolução/validação de IDs;
- transaction;
- chamada ao algorithm/provider;
- invariants;
- cache invalidation;
- commit/rollback;
- Undo/Redo;
- resultado estruturado.

Exemplo:

```rust
pub struct ExtrudeCommand {
    pub mesh_id: MeshId,
    pub face_ids: Vec<FaceId>,
    pub distance: ExtrudeDistance,
}
```

## Tool

A Tool contém somente estado de interação e preview necessário para converter input humano em command:

```plain text
pointer down
→ picking
→ drag
→ preview distance
→ pointer release
→ ExtrudeCommand
```

Tool não implementa novamente a geometria.

# Regra absoluta: Tool não chama Tool

Proibido formar dependências como:

```plain text
BevelTool → ExtrudeTool → SelectionTool
```

Quando duas tools precisam da mesma capacidade, ambas dependem do mesmo command/algorithm/domain service:

```plain text
Tool A ─┐
        ├→ Common Command / Domain Operation
Tool B ─┘
```

Remover uma tool da UI não pode quebrar outra funcionalidade de domínio.

# Uma única linguagem de operações

UI, atalhos, menus, Lua, MCP e testes usam os mesmos Commands:

```plain text
egui/Petunia UI ─────┐
Keyboard shortcut ───┤
Lua plugin ──────────┤
MCP ─────────────────┤
Automated tests ─────┤
                     ↓
              Command Dispatcher
                     ↓
              Typed Command
                     ↓
             Domain / Geometry
```

Não criar `McpExtrude`, `LuaExtrude` e `UiExtrude` com semânticas diferentes.

# Commands permanecem tipados

Não usar `HashMap<String, Value>` como domínio universal.

Preferir commands concretos:

- `ExtrudeCommand`;
- `BevelCommand`;
- `SliceCommand`;
- `MoveObjectsCommand`;
- `FuseObjectsCommand`;
- `ConnectSurfacesCommand`;
- `ProjectReferenceCommand`;
- `CreateTextureCommand`.

Argumentos JSON/Serde/Schemars só existem nas fronteiras dinâmicas.

# Command metadata separado do comportamento

O Command Registry descreve capabilities externas sem transformar o core em JSON:

```plain text
command_id
label_key
description_key
argument_schema
result_schema
undoable
previewable
required_capabilities
lua_exposable
mcp_exposable
help_manual_id
```

A implementação Rust tipada é a fonte semântica; adapters produzem schemas/bindings a partir dela.

# SOLID adaptado a Rust

<table>
<tr><td>Princípio</td><td>Interpretação Petunia/Rust</td></tr>
<tr><td>Single Responsibility</td><td>Type/module/function possui responsabilidade de domínio clara e uma razão coerente para mudar.</td></tr>
<tr><td>Open/Closed</td><td>Extension points apenas onde substituição real existe; não criar trait antecipadamente para todo tipo.</td></tr>
<tr><td>Liskov</td><td>Providers substituíveis precisam passar a mesma conformance suite e manter invariantes/resultados contratuais.</td></tr>
<tr><td>Interface Segregation</td><td>Traits pequenas e focadas em capabilities concretas; consumidores não dependem de métodos irrelevantes.</td></tr>
<tr><td>Dependency Inversion</td><td>Domínio/application define os contratos; egui/eframe, Manifold, xatlas, Lua, MCP e formatos externos adaptam-se a eles.</td></tr>
</table>

SOLID não autoriza criar `Manager → Service → Repository → Factory → Controller` para cada conceito.

# Quando usar struct, enum, trait e dyn Trait

<table>
<tr><td>Situação</td><td>Escolha padrão</td></tr>
<tr><td>dados concretos</td><td>`struct`</td></tr>
<tr><td>conjunto fechado de estados/alternativas</td><td>`enum`</td></tr>
<tr><td>implementação realmente substituível</td><td>`trait`</td></tr>
<tr><td>runtime plugin/provider heterogêneo</td><td>`dyn Trait` na fronteira</td></tr>
<tr><td>hot path polimórfico conhecido em compile time</td><td>generic somente se benefício concreto justificar</td></tr>
<tr><td>encapsular implementação</td><td>module + private fields antes de inventar trait</td></tr>
<tr><td>identidade</td><td>newtype ID</td></tr>
</table>

Exemplo de conjunto fechado:

```rust
pub enum SelectionMode {
    Object,
    Face,
    Edge,
    Point,
}
```

Não criar `SelectionMode` como trait apenas para imitar polymorphism OO.

# Traits: política rigorosa

Traits são permitidas quando há pelo menos uma destas necessidades:

1. múltiplas implementações reais;
2. implementação externa/plugin precisa cumprir contrato;
3. provider/vendor precisa ser substituível;
4. testes precisam injetar implementação alternativa e o boundary é semanticamente real.

Exemplos válidos:

```plain text
BooleanProvider
UvUnwrapProvider
Exporter
Importer
ProjectStorage
```

Evitar traits especulativas criadas apenas porque “talvez um dia existam duas implementações”.

Quando consumidores externos **não** devem implementar uma trait pública, considerar sealing. Em APIs evolutivas, considerar `#[non_exhaustive]` onde adicionar variantes/campos deve continuar possível.

# Evitar generic hell

Não estruturar a aplicação como:

```plain text
Application<Geometry, Renderer, Document, Plugins, Materials, ...>
```

se isso fizer generics e lifetimes atravessarem o programa inteiro.

Preferir tipos concretos no interior e traits/dynamic dispatch nas bordas onde dinamismo é um requisito real.

Uma virtual call em exporter/plugin/provider não é relevante para performance do hot path do renderer e vale a clareza obtida.

# Ownership como parte da arquitetura

## Regra principal

**Long-lived relationships use IDs, not Rust references.**

Não criar grafos de structs com referências lifetime-parametrizadas de longa duração.

Errado como arquitetura base:

```rust
pub struct Face<'mesh> {
    mesh: &'mesh PetuniaMesh,
}
```

Preferido:

```rust
pub struct SceneObject {
    material_id: Option<MaterialId>,
}
```

Resolver IDs em borrows curtos no ponto de uso.

## Clone não é ferramenta de fuga do borrow checker

`.clone()` só é usado quando duplicação/CoW/snapshot faz parte da semântica. Nunca adicionar clone apenas para silenciar um conflito de borrowing sem entender o ownership esperado.

# Single-writer Document

O documento mutável principal possui um único escritor lógico, normalmente Application/Main thread.

```plain text
Application/Main
      ↓
   Document
 single writer
```

Trabalhos pesados recebem snapshots imutáveis:

```plain text
Document revision 42
      ↓ snapshot
Worker / Rayon
      ↓
ProposedResult(base_revision = 42)
      ↓
Application
      ↓
current revision == 42 ? commit : stale/recompute
```

Não compartilhar `Arc<Mutex<Document>>` entre UI, renderer, jobs, plugins e MCP.

# Ordem de preferência para compartilhamento

```plain text
explicit ownership
    > message passing
    > immutable snapshot
    > small localized lock
    > Arc<Mutex<BigApplicationState>>
```

`Arc`, `Mutex`, `RwLock`, `RefCell` e interior mutability são ferramentas locais; não constituem a arquitetura do aplicativo.

# Async é boundary, não domínio

Tokio/async permanece restrito a MCP, network ou I/O que realmente necessite `await`.

Não tornar Geometry/Application APIs assíncronas por contaminação:

```plain text
MCP Tokio runtime
      ↓ channel/request
synchronous Application Core
      ↓
Geometry
```

Não existirão `async fn extrude`, `async fn bevel` ou `async fn move_vertex` sem uma razão concreta futura.

# Política de unsafe

Por padrão, crates do domínio devem usar:

```rust
#![forbid(unsafe_code)]
```

Aplicar pelo menos a:

- domain/core;
- application;
- geometry própria;
- I/O seguro escrito por nós;
- UI business bindings;
- plugin/MCP adapters quando possível.

FFI/vendor boundaries inevitáveis ficam em módulos/crates pequenos e auditáveis. Cada bloco `unsafe` deve documentar:

- invariantes exigidas antes da chamada;
- o que o código garante após a chamada;
- lifetime/aliasing assumptions;
- ownership dos ponteiros/buffers;
- comportamento em erro/panic.

Usar Miri em código aplicável e fuzz/contract tests nas fronteiras FFI.

# Error model orientado ao domínio

Nunca usar `Result<T, String>` como contrato interno estável.

Exemplo:

```rust
pub enum ConnectSurfacesError {
    MissingBoundary,
    StaleFaceId(FaceId),
    AmbiguousBoundary,
    DegenerateResult,
    SelfIntersection,
}
```

A Application traduz isso para error code/context estruturado. UI escolhe texto localizado. MCP recebe code + structured details. Logs preservam informação técnica.

# Clean Code adaptado ao Rust

A interpretação oficial de Clean Code é:

```plain text
explicit > magical
domain name > obscure abbreviation
data flow > service locator
composition > inheritance emulada
small cohesive modules > giant generic abstractions
concrete type > premature trait
short borrows > lifetime propagation
owned/immutable snapshot > shared mutable graph
```

## Nomes

Evitar abreviações internas obscuras como:

```plain text
ctx mgr svc cfg doc rev obj geom tex sel cmd
```

quando um nome completo cabe naturalmente:

```plain text
context configuration document revision object geometry texture selection command
```

Abreviações de domínio universalmente reconhecidas são aceitas: `UV`, `GPU`, `CPU`, `UUID`, `MCP`, `PBR`, `RGB`.

Variáveis matemáticas `x`, `y`, `u`, `v` são aceitáveis em escopo matemático curto.

## Funções e módulos

Funções devem ser pequenas o bastante para terem um objetivo claro, mas não quebradas artificialmente apenas para atingir uma contagem de linhas. Módulos agrupam conceitos coesos. Comentários explicam invariantes, trade-offs e motivos; não narram linha por linha o que o código já expressa.

# Crate não é sinônimo de módulo

Cargo workspace é fronteira de dependência/compilação. Não criar crate por command, type ou feature pequena.

**Domain e Application são camadas lógicas, não exigência de crates separadas.** Na estrutura física vigente do capítulo 28, ambas podem começar dentro de `petunia-core` em módulos claramente separados, por exemplo `domain/`, `application/`, `commands/` e `history/`. Só criar uma futura `petunia-application` ou `petunia-domain` crate se dependências, compilação, testes ou API demonstrarem uma fronteira física concreta. O nome conceitual `Application` neste capítulo não altera sozinho o Cargo workspace.

Uma nova crate precisa justificar pelo menos um:

- dependency boundary real;
- runtime/platform boundary;
- compilação/teste independente relevante;
- ownership arquitetural distinto;
- dependências pesadas que não devem vazar;
- API pública isolada.

Dentro de `petunia-geometry`, por exemplo, usar modules:

```plain text
mesh/
profile/
extrude/
bevel/
connect/
slice/
revolve/
boolean/
```

sem transformar cada pasta em crate.

# Dependency direction como invariant

Camadas internas não podem depender de adapters externos. As setas abaixo são **lógicas**; `domain/core` e `application` podem residir fisicamente na mesma crate `petunia-core` enquanto preservarem módulos e dependências internas claras:

```plain text
domain/core ← application ← UI/Lua/MCP
petunia-geometry ← application orchestration
petunia-render ← RenderSnapshot adapter
```

Proibições exemplares:

```plain text
domain → egui/eframe
geometry → egui/eframe
geometry → MCP
geometry → Lua
renderer → egui/eframe
Document → wgpu
```

Criar checks/scripts no `xtask` sempre que uma regra puder ser verificada automaticamente.

# Cargo features não são module system

Cargo features devem representar capabilities opcionais **aditivas**. Não usar features como mecanismo principal para trocar implementações mutuamente exclusivas ou ligar/desligar metade da arquitetura.

Official Modules são compostos explicitamente no composition root. Community Plugins são runtime extensions.

# Official Modules sem framework prematuro

Começar com funções de registro simples:

```rust
modeling::register(&mut application)?;
painting::register(&mut application)?;
uv::register(&mut application)?;
export::register(&mut application)?;
```

Cada módulo pode registrar:

- commands;
- interactive tools;
- validators;
- importers/exporters;
- UI contributions aprovadas.

Não criar inicialmente `AbstractModuleFactoryProviderRegistry` ou equivalente. Introduzir trait/module protocol apenas quando o lifecycle real exigir.

# Módulos acopláveis/desacopláveis

Para uma feature ser considerada modular:

1. seu algoritmo funciona independentemente da UI;
2. seu command funciona por Application API;
3. sua Tool pode ser removida sem remover o command;
4. Lua/MCP podem chamar o command sem reproduzir a lógica;
5. dependencies específicas ficam em seu módulo/provider;
6. removal desregistra contributions sem deixar referências quebradas;
7. tests cobrem comportamento isolado e composição com outras features;
8. persistência não depende de estado efêmero da Tool;
9. renderer recebe somente dados derivados necessários;
10. desabilitar uma feature opcional produz capability unavailable explícito, nunca panic/símbolo faltando.

# Internal API versus Extension API versus MCP

Nunca tratá-las como o mesmo nível de estabilidade:

```plain text
Internal Rust API
    → evolui com refactors controlados

Extension API
    → versionada e projetada para plugins

MCP API
    → contrato externo versionado e semanticamente estável
```

Não expor `slotmap` keys, raw pointers, vendor structs ou `PetuniaMesh` mutável diretamente a plugins/MCP.

Usar DTOs/views estáveis e commands semânticos.

# Compatibilidade evolutiva

Para APIs externas:

- private fields por padrão;
- newtypes para representação opaca;
- constructors/validation explícitos;
- error codes estáveis quando possível;
- `#[non_exhaustive]` quando crescimento de enum/struct público deve ser permitido;
- sealed traits quando consumers não devem implementar extensões arbitrárias;
- versionamento explícito para Extension API/MCP;
- capability discovery em vez de presumir presença de feature.

# Testabilidade como requisito de arquitetura

Cada feature precisa ser testável sem egui/eframe.

<table>
<tr><td>Camada</td><td>Estratégia obrigatória</td></tr>
<tr><td>Tipos/Math</td><td>unit tests</td></tr>
<tr><td>Geometry algorithms</td><td>unit + fixtures + property tests</td></tr>
<tr><td>Half-edge</td><td>invariant/property tests</td></tr>
<tr><td>Commands</td><td>transaction + errors + Undo/Redo</td></tr>
<tr><td>Providers</td><td>conformance suites independentes do vendor</td></tr>
<tr><td>I/O</td><td>round-trip + migration + golden</td></tr>
<tr><td>Plugins</td><td>capability/sandbox/lifecycle</td></tr>
<tr><td>MCP</td><td>schema + command conformance</td></tr>
<tr><td>Renderer</td><td>render snapshots/fixtures + visual tests quando viável</td></tr>
<tr><td>UI</td><td>interaction + accessibility + screenshots</td></tr>
<tr><td>Workflows</td><td>integration/end-to-end</td></tr>
</table>

# Interoperabilidade precisa de testes próprios

Uma feature não está pronta apenas porque seu unit test passa.

Criar workflows de composição como:

```plain text
Primitive
→ Extrude
→ Bevel
→ Mirror
→ Cut
→ Auto UV
→ Paint
→ Save
→ Reload
→ Export
```

e:

```plain text
Reference
→ Profile
→ Extrude
→ Project From Reference
→ Paint
→ Export
```

Testes de interoperabilidade protegem contratos que unit tests isolados não enxergam.

# Safety não termina no compilador

`cargo check` prova propriedades de tipos/borrowing, não correção funcional. Bugs lógicos, panics, inconsistências geométricas, corrupção semântica de save e problemas de UX continuam possíveis.

Por isso Rust safety é combinada com:

```plain text
compiler
+ clippy
+ invariants
+ property tests
+ fuzzing
+ conformance
+ integration tests
+ Miri onde aplicável
+ visual/accessibility tests
```

# Arquitetura amigável para agentes de IA

O repositório deve ser legível por agentes sem depender de contexto implícito do autor.

## Regras

- nomes de módulos e types refletem domínio;
- architecture invariants documentadas perto da fronteira;
- feature tests localizados próximos à implementação;
- comandos canônicos centralizados em `xtask`;
- TODOs explicam contexto e condição de conclusão;
- nenhum arquivo “god object” com múltiplos domínios;
- evitar metaprogramming/macros complexas quando uma função/derive convencional resolve;
- erros estruturados e logs permitem feedback automático;
- cada crate possui documentação curta de responsabilidade e dependências permitidas.

# Knowledge map para agentes

Não criar um `AGENTS.md` gigantesco contendo toda a especificação do Petunia.

Usar um mapa hierárquico:

```plain text
AGENTS.md
    → architecture map
    → commands para build/test
    → links para docs canônicas

docs/architecture/
    domain.md
    geometry.md
    commands.md
    rendering.md
    plugins.md
    testing.md

crates/petunia-geometry/AGENTS.md
crates/petunia-render/AGENTS.md
...
```

O arquivo raiz orienta; documentos locais explicam invariantes específicas do diretório. O Prumo deve gerar/manter esse mapa a partir da documentação canônica, sem duplicar integralmente o Livro Vivo em cada arquivo.

# Definition of Done por feature

Uma feature só está concluída quando:

1. algorithm/domain behavior implementado;
2. command tipado implementado quando aplicável;
3. transaction/Undo definidos;
4. errors estruturados;
5. invariants/testes da camada passam;
6. interoperability test relevante passa;
7. UI Tool/adapter usa o command sem duplicar lógica;
8. Lua/MCP exposure, quando declarada, deriva do mesmo contrato;
9. documentação e help identifier existem quando a feature é pública;
10. `cargo xtask verify` passa.

# Architecture invariants obrigatórias

O Prumo e CI devem tratar estas regras como invariants:

- Geometry não depende de egui, eframe, MCP ou Lua.
- Core/Domain não depende de wgpu.
- Renderer não possui `&mut Document`.
- UI não muta half-edge diretamente.
- Tool não chama Tool.
- Lua/MCP não possuem implementação paralela de commands.
- Providers não vazam vendor structs para o domínio.
- long-lived relations não usam referências Rust.
- estado mutável principal tem single writer.
- jobs trabalham sobre snapshots/base revision.
- stale IDs geram erro, não alias silencioso.
- `unsafe` nosso fica isolado e documentado.
- cargo features não substituem composition root.
- optional feature ausente gera capability error explícito.
- tests acompanham feature, não são backlog posterior.

# Anti-patterns proibidos

Evitar deliberadamente:

```plain text
Arc<Mutex<ApplicationState>> como estado global
Rc<RefCell<Everything>> como escape de ownership
giant generic Application<T...>
trait para cada struct
Manager/Service/Repository/Factory sem necessidade real
HashMap<String, Value> como linguagem interna universal
Tool chamando Tool
UI chamando half-edge diretamente
MCP/Lua duplicando lógica de domínio
.clone() apenas para calar borrow checker
unsafe espalhado
vendor types dentro do Document
ECS World universal
crate por classe/command
feature flags mutuamente exclusivas como arquitetura
macros/metaprogramming para esconder fluxo essencial
```

# Fluxo de referência: Extrude

```plain text
Petunia ExtrudeTool (egui presentation)
    ↓
preview state
    ↓
ExtrudeCommand
    ↓
Application transaction
    ↓
geometry::extrude_faces
    ↓
PetuniaMesh revision
    ↓
validation
    ↓
commit + Undo
    ↓
RenderSnapshot rebuild/invalidation
```

O mesmo `ExtrudeCommand` pode vir de Lua, MCP, shortcut ou teste.

# Fluxo de referência: job pesado

```plain text
FuseCommand
    ↓
Document snapshot + base revision
    ↓
Boolean job/provider
    ↓
ProposedGeometryResult
    ↓
return to Application writer
    ↓
revision check
    ↓
validate
    ↓
commit or stale result
```

Provider nunca escreve o Document diretamente.

# Relação com a stack atual

Este capítulo complementa os capítulos 27–31 e 35. Ele **não troca Rust + egui + eframe + egui-wgpu + wgpu nem a divisão funcional já escolhida**; define como escrever o código dentro dessa stack.

Em conflito de estilo/representação com exemplos antigos, este capítulo prevalece para implementação Rust, salvo ADR posterior explícito.

# Fronteiras que este capítulo não congela

Este capítulo **não decide sozinho**:

- schema concreto/versionamento do arquivo `.petunia`;
- política de migrations/forward-backward compatibility em detalhe;
- versionamento do aplicativo e da Extension API;
- versionamento de plugin manifests/MCP schemas;
- layout final de diretórios dentro do container de projeto;
- decisões UI BASELINE FINAL V1 já registradas.

As decisões existentes dos capítulos 16 e 30 continuam como baseline corrente até a revisão específica de formatos/versionamento. Essa revisão deve preservar os princípios daqui: IDs persistentes separados de handles runtime, vendor types fora do formato, migrations explícitas, erros estruturados e módulos opcionais incapazes de corromper projetos que não os utilizam.

# Referências técnicas e arquiteturais

Estas fontes orientaram a política, mas não substituem os contratos Petunia:

- [Rust API Guidelines — Checklist](https://rust-lang.github.io/api-guidelines/checklist.html)
- [Rust API Guidelines — Naming](https://rust-lang.github.io/api-guidelines/naming.html)
- [Rust API Guidelines — Future Proofing](https://rust-lang.github.io/api-guidelines/future-proofing.html)
- [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Cargo Resolver / Features](https://doc.rust-lang.org/cargo/reference/resolver.html)
- [std::marker::Send](https://doc.rust-lang.org/std/marker/trait.Send.html)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/patterns/)
- [Borrow clone anti-pattern](https://rust-unofficial.github.io/patterns/anti_patterns/borrow_clone.html)
- [rust-analyzer architecture](https://rust-analyzer.github.io/book/contributing/architecture.html)
- [RustBelt](https://plv.mpi-sws.org/rustbelt/popl18/)
- [Miri](https://github.com/rust-lang/miri/)
- [OpenAI — How OpenAI uses Codex](https://openai.com/business/guides-and-resources/how-openai-uses-codex/)
- [OpenAI — Harness engineering](https://openai.com/index/harness-engineering/)
- [GitHub — Repository custom instructions](https://docs.github.com/en/copilot/how-tos/configure-custom-instructions-in-your-ide/add-repository-instructions-in-your-ide)
- [egui](https://github.com/emilk/egui) — UI/accessibility/testing/inspection ecosystem
- [egui_kittest](https://docs.rs/egui_kittest/latest/egui_kittest/) — semantic and snapshot UI testing
- Rerun, Bevy, Zed e Graphite permanecem referências de estudo para DOD/ECS composition, large Rust workspaces, rendering e message/command-oriented editor architecture, sem serem dependências do Petunia.

# Regra final

> **O Petunia deve ser composto por dados explícitos e operações explícitas. Ferramentas interativas são clientes do domínio, não o domínio. Abstrações existem para preservar uma fronteira real, não para demonstrar sofisticação.**
