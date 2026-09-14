# 19 — Concorrência, Memória, Caches e Performance

> O escopo low-poly permite uma arquitetura de concorrência muito mais simples que um DCC generalista. A prioridade é evitar races e invalid states, não maximizar paralelismo teórico.

# Single-writer document

A authoring state tem **um único writer**: a thread principal de edição/Application API.

UI, plugin e MCP nunca alteram arrays de mesh diretamente. Todos enviam commands que são serializados no mesmo fluxo transacional.

Isso elimina a necessidade de locks finos na topology viva.

# Workers operam em snapshots

Operações pesadas:

- Boolean;
- generic Auto UV;
- validação global;
- texture bake;
- thumbnails;
- future LOD/providers;

recebem snapshots imutáveis.

Cada job carrega:

```plain text
document_revision
source_object_ids
source_revisions
cancel_token
```

No retorno, o resultado só é aplicado se a origem ainda for compatível. Caso contrário, é descartado/recalculado sem alterar o documento.

# Thread pool

Na baseline Rust, usar **Rayon** para CPU jobs paralelizáveis. Não criar uma thread por ferramenta. Providers externos podem usar paralelismo próprio somente quando medição demonstrar benefício; evitar nested oversubscription.

Comunicação serviço/worker → Application/Main thread usa channels tipados, com **flume** como baseline. Tokio não governa o core e fica restrito a serviços que realmente precisam de async, principalmente MCP.

# Memória

Rust ownership é a política base. Não reproduzir manualmente um sistema de arenas como requisito universal.

**Long-lived relationships usam IDs, não referências Rust de longa duração.** Borrows devem permanecer curtos e locais à operação; não propagar lifetimes por grandes partes do modelo apenas para representar relações persistentes.

- dados persistentes pertencem ao Document e aos seus owners explícitos;
- snapshots/Undo podem compartilhar blobs grandes imutáveis por `Arc`;
- texturas/referências grandes podem usar copy-on-write com `Arc::make_mut` quando isso reduzir cópias sem tornar ownership obscuro;
- operações/jobs usam valores temporários locais e buffers reutilizáveis quando profiling justificar;
- buffers GPU pertencem exclusivamente a `petunia-render`/wgpu;
- caches são descartáveis/reconstruíveis e não são fonte de verdade.

Não introduzir GC no core. Não usar `Arc<Mutex<...>>` como substituto automático de modelagem de ownership; o Document continua single-writer.

# Mesh storage

Authoring mesh usa half-edge topology própria com **IDs generacionais tipados via `slotmap`** e atributos separados. Estruturas auxiliares são reconstruíveis.

Manter posições de authoring em `f64` quando necessário à robustez das operações; caches/render data podem usar `f32` quando apropriado. A conversão deve ocorrer em fronteira explícita.

# Dirty flags e revision counters

Cada domínio mantém revisões/dirty bits suficientemente granulares:

```plain text
TopologyDirty
PositionsDirty
UVDirty
MaterialDirty
TextureDirty
TransformDirty
ReferenceDirty
```

Uma operação invalida apenas caches dependentes.

Exemplos:

- mover objeto → transform/render bounds, não triangulation;
- mover vertex → normals, BVH, GPU vertices; topology pode permanecer;
- alterar edge connectivity → triangulation, normals, BVH, UV analysis;
- paint stroke → texture upload apenas nos tiles modificados.

# Triangulation cache

Cache por mesh/revision. A triangulação é gerada deterministically e reutilizada por viewport, picking, validation e export quando a revisão coincide.

# Spatial acceleration

**Não tornar BVH obrigatória na primeira implementação.** O escopo low-poly permite começar com ray/triangle sobre a triangulação derivada, medindo custo real.

Se profiling mostrar necessidade, introduzir um `PickingBackend`/BVH derivado dos triangles de render. Rebuild após mudança topológica e refit após alteração apenas de posições podem entrar depois. A estrutura espacial nunca faz parte da authoring mesh nem do formato persistente.

# Texture dirty regions

Raster painting acompanha retângulos/tiles alterados para upload parcial de texture e Undo por tiles. Não reuploadar atlas inteiro por sample se o backend final permitir subresource update.

# Previews

Preview geométrico deve trabalhar em estado temporário/overlay quando possível. Não criar centenas de Undo entries durante drag.

Commit final troca o resultado validado em uma única transaction.

# Cancelamento

Jobs externos precisam aceitar cancelamento cooperativo quando a biblioteca permitir. Se um provider não puder cancelar, resultado antigo é simplesmente ignorado por revision mismatch.

# Performance targets

Não otimizar para high-poly. Critérios iniciais:

- interação de transform/picking deve permanecer responsiva em assets low-poly normais;
- operações locais devem ocorrer síncronas quando são perceptualmente imediatas;
- operações que podem ultrapassar um frame usam preview/job;
- nenhuma otimização pode quebrar determinismo ou Undo apenas para ganhar performance marginal.

Benchmarks concretos serão definidos com fixtures reais, não números arbitrários antes da implementação.

# Profiling

Adicionar instrumentation leve por domínio e permitir build de profiling. Só introduzir otimizações complexas depois de perfil mostrar hotspot real.

# Relação com a arquitetura Rust-safe

Este capítulo implementa as regras de ownership/concurrency do capítulo 34. `Arc`, `Mutex`, `RwLock`, `RefCell` e interior mutability são ferramentas locais, não o modelo arquitetural. A preferência é ownership explícito → message passing → immutable snapshot → lock pequeno/localizado.

# Regra final

**Single writer + immutable jobs + revision check** é a política de concorrência oficial. É simples de testar, simples de explicar e suficiente para o escopo.
