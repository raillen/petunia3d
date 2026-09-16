# 16 — Documento, Formato de Projeto, Undo, Autosave e Recovery

<aside>
💾

O formato do projeto e o sistema de histórico priorizam robustez e simplicidade. Como o foco é low-poly, preferimos snapshots claros a sistemas complexos de event sourcing ou bancos de dados internos.

</aside>

# Documento em memória

Um `Document` possui IDs estáveis e domínios independentes:

```
Document
├ Objects
├ Meshes
├ Profiles / Generators
├ References
├ Materials
├ Textures
├ UV data
├ Project Settings
└ Derived caches (não persistidos)
```

Transforms, seleção e caches de viewport não devem contaminar a serialização quando forem apenas estado efêmero.

# Identidade

- `ObjectId`, `MeshId`, `ProfileId`, `MaterialId`, `TextureId` e `ReferenceId`: **UUID/identidade persistente**, preservada por save/load e migrations.
- `VertexId`, `HalfEdgeId`, `EdgeId` e `FaceId`: **handles generacionais tipados**, escopados a uma `MeshId` e ao lifecycle da topologia. Na implementação Rust, são baseados em `slotmap` ou abstração equivalente.
- Handles topológicos permanecem válidos enquanto o elemento existir e a operação preservar sua identidade; operações destrutivas/reconstrutivas, como Boolean, import/rebuild ou certos edits de topology, podem invalidá-los e retornar remaps quando fizer sentido.
- O formato `.petunia` serializa a **topologia e seus atributos por um schema próprio**, não promete que o valor binário/interno de uma key `slotmap` seja idêntico após reload. O loader reconstrói handles runtime e preserva a semântica do documento.
- APIs públicas nunca aceitam índices crus de arrays. Handles stale retornam erro estruturado e nunca resolvem silenciosamente para outro elemento.

# Coordenadas e precisão

- espaço interno: **right-handed, Y-up, metros**;
- positions/transforms e cálculos geométricos do authoring: `f64` quando custo for irrelevante;
- buffers GPU e export comum: `f32`;
- exporters fazem conversão explícita de eixo/unidade quando o formato/pipeline exigir.

# Formato `.petunia`

V1 usa um **container ZIP versionado** para portabilidade.

Estrutura conceitual:

```
project.petunia
├ manifest.json
├ document.json
├ textures/
├ references/
└ thumbnails/ (opcional/derivado)
```

Decisão importante: **authoring geometry permanece em JSON na V1**. Para assets low-poly, simplicidade, debuggability e migração valem mais que otimizar alguns megabytes. Um chunk binário pode ser introduzido futuramente sem quebrar o formato através de versionamento.

# Manifest

`manifest.json` contém pelo menos:

- format_version;
- app_version;
- project_id;
- created/modified metadata não sensível;
- lista de assets embutidos e hashes opcionais;
- feature flags necessárias para carregar o documento.

# Assets

V1 embute referências e texturas por padrão para o arquivo ser portátil. Links externos são evolução futura. Pinturas/bakes usam PNG lossless; JPEG pode ser preservado para reference photos quando apropriado.

# Versionamento e migração

`format_version` é independente da versão do aplicativo.

Loader:

```
read manifest
→ validate version
→ migrate older schema step-by-step
→ validate document
→ open
```

Nunca sobrescrever automaticamente o original durante uma migração que possa falhar; save posterior escreve formato atual.

# Undo/Redo

Todo command mutável abre uma `Transaction`.

Para V1, usar **snapshots dos objetos/regiões afetados** em vez de implementar inverses especializados para cada operação.

Vantagens:

- menos bugs em Undo;
- plugins seguem a mesma regra;
- operações complexas continuam atômicas;
- baixo custo aceitável em low-poly.

## Granularidade

- transform drag completo → uma entrada;
- bevel/connect/fuse/cut → uma entrada;
- paint stroke completo → uma entrada;
- batch MCP explícito → uma entrada quando atomicidade for declarada.

# Undo de textura

Não copiar uma textura inteira a cada brush sample. Dividir raster em tiles fixos (por exemplo 64×64) e guardar before/after apenas dos tiles modificados durante o stroke.

# Limite de memória

History possui orçamento de memória configurável e remove entradas mais antigas quando necessário. **Default V1: 256 MiB por documento aberto**, com configuração pelo usuário; instalações futuras podem oferecer presets maiores sem alterar o formato do projeto. Ao atingir o budget, remover entradas mais antigas por ordem temporal, preservando sempre a integridade da entrada atual. Undo command history **não é persistido no `.petunia` V1**. O estado procedural de generators, por outro lado, faz parte do documento e é persistido.

# Save normal

Salvar de forma atômica:

```
serialize
→ write temporary file
→ flush/close
→ replace/rename target atomically quando suportado
```

Nunca escrever diretamente sobre o único arquivo válido enquanto ele está sendo construído.

# Autosave / Crash Recovery

Recovery não usa event sourcing. Ele grava **snapshots completos do projeto** em diretório de cache separado.

**Defaults V1:**

- recovery automático a cada **2 minutos** enquanto o documento estiver dirty;
- manter **5 gerações recentes** por documento/projeto;
- intervalo e quantidade podem ser configurados ou desabilitados pelo usuário;
- nunca executar mais de um recovery write concorrente para o mesmo documento; se um ciclo ainda estiver ativo, coalescer/adiar o próximo;
- serialização pode ocorrer em worker a partir de snapshot imutável/revisionado, sem bloquear a thread de edição além da captura necessária;
- recovery nunca substitui o arquivo do usuário silenciosamente;
- excluir recovery correspondente após save/close limpo conforme política;
- detectar snapshot mais recente que o save normal na próxima abertura.

# Thread ownership

O Document mutável pertence a **uma thread principal de edição**. Workers nunca modificam topology viva diretamente.

Jobs pesados recebem snapshot imutável + `document_revision`; ao terminar, o resultado só pode ser aplicado se a revisão/origem ainda for compatível.

Isso vale para Boolean, Auto UV, validation pesada, thumbnail generation e futuros providers.

# Derived caches

Não persistir caches que podem ser reconstruídos:

- triangulation buffers;
- normals/tangents;
- picking acceleration/BVH **quando habilitado por profiling**;
- GPU buffers;
- UV stretch analysis;
- thumbnails quando regeneráveis.

Persistir apenas dados de authoring e decisões explícitas como locked triangulation, seams e sharp flags.

# Regra final

O formato deve ser fácil de inspecionar, migrar e recuperar. Complexidade de armazenamento só entra depois de uma medição real mostrar que JSON/ZIP deixou de ser adequado ao low-poly.