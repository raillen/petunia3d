# 30 — I/O, Projeto .petunia, Lua Plugins e MCP na Stack Rust

<aside>
🔌

Esta página define as fronteiras de persistência e extensibilidade na stack Rust. Save, export, Lua e MCP são adapters em torno da mesma Application API; nenhum deles contorna transactions ou Geometry invariants.

</aside>

# Formato `.petunia`

Container ZIP versionado com authoring data em JSON e assets binários separados.

Estrutura V1 conceitual:

```
character.petunia
├ manifest.json
├ document.json
├ textures/
│  ├ body.png
│  └ face.png
├ references/
│  ├ front.png
│  └ side.png
└ thumbnails/
   └ preview.png
```

Não usar SQLite ou formato binário proprietário na V1.

# Serde

Structs de persistência Rust usam `Serialize`/`Deserialize`, mas devem ser separados quando necessário das estruturas runtime para preservar migrations e evitar que refactors internos quebrem automaticamente o file format.

`slotmap`/IDs serializados precisam de round-trip tests garantindo que o documento reconstruído preserva semanticamente as mesmas entidades persistentes e relações após save/load; handles topológicos runtime podem receber novos valores internos e não são promessa de identidade binária persistente.

# Versionamento do formato

Manifest contém ao menos:

```
format = petunia
version = integer
application_version
```

Loader:

```
same version → read
older version → migration chain
newer unsupported version → explicit diagnostic
```

Migrations são sequenciais e testadas (`v1→v2`, `v2→v3`), nunca uma coleção de condicionais espalhadas pelo loader.

# Atomic save

Pipeline:

```
serialize snapshot
→ write temporary archive
→ flush/validate when appropriate
→ atomic replace/rename
→ keep previous valid save on failure
```

Recovery snapshots nunca sobrescrevem silenciosamente o arquivo principal.

# Image codecs

A crate `image` é adapter de decode/encode, com `default-features = false` e apenas codecs realmente suportados habilitados. Input externo é não confiável; decode errors/panics potenciais devem ser tratados/testados e fuzzing aplicado às fronteiras.

O documento persiste conteúdo em representação própria, não em tipos da crate `image`.

# glTF/GLB

Usar `gltf`/`gltf-json` para o formato principal. Export recebe `ExportModel` já triangulado/validado:

```
Document
→ prepare ExportModel
→ validate
→ glTF/GLB writer
```

Não espalhar tipos glTF pelo Geometry Core.

# OBJ

A V1 usa **writer próprio pequeno para export OBJ** e **`tobj` como parser baseline de import OBJ**. `tobj` permanece atrás do `Importer` trait, com fixtures/golden/round-trip tests próprios; seus tipos nunca vazam para `Document`, `PetuniaMesh`, Application API ou formato `.petunia`. Se a crate se tornar inadequada, a substituição não altera o contrato público.

# FBX futuro

FBX não entra na baseline. Se houver demanda, avaliar ufbx/binding dedicado antes de adotar importer universal enorme.

# Lua 5.4 + mlua

Community Plugins continuam em Lua. `mlua` é o host Rust.

Baseline:

```
Lua 5.4 vendored
one Lua State per plugin
Serde conversion quando útil
no direct mutable mesh exposure
```

# Sandbox Lua

Bibliotecas permitidas por padrão podem incluir math, table, string, utf8 e coroutine. Bloquear/omitir por default:

```
os
io
debug
package.loadlib / native arbitrary library loading
```

Filesystem só via Petunia capability APIs e roots autorizados. Networking não faz parte da Plugin API V1.

# Plugin manifest

Preferir `plugin.toml` na stack Rust por legibilidade e validação. Campos conceituais:

```toml
id = "example.low_poly_roof"
name = "Low Poly Roof"
version = "1.0.0"
petunia_api_version = "1"
entrypoint = "main.lua"

[permissions]
read_document = true
modify_geometry = true
filesystem = false
mcp_exposable = false
```

O formato final deve ser versionado e validado por schema.

# Plugin lifecycle

```
manifest
→ validate
→ create Lua State
→ register commands/events
→ enable
→ disable
→ unload/destroy state
```

Um plugin com erro não deve destruir states de outros plugins.

# Plugin mutations

Nunca expor:

```
mesh.vertices[...].x = ...
```

Preferir:

```
petunia.command("model.extrude", args)
```

ou APIs de alto nível que internamente produzam commands/transactions.

Erro Lua durante mutação causa rollback.

# Plugin UI e Plugin Panels

V1 **não permite acesso cru a egui/wgpu, Painter, raw input ou markup arbitrário**, mas Community Plugins podem registrar **painéis completos** através da `Petunia UI Extension API` definida no capítulo 36.

Fluxo preferido para operações simples continua sendo:

```
Command + metadata/argument schema
→ Petunia escolhe apresentação coerente
```

Quando a extensão realmente precisa de uma superfície persistente, o plugin pode registrar `Plugin Panel` com descriptor, região permitida e callback/builder Lua baseado exclusivamente em Petunia Components públicos.

Regras:

- regiões V1: `left`, `right`, `bottom`;
- sem substituição do viewport/center por Community Plugin V1;
- sem floating windows arbitrárias;
- panel state namespaced por `plugin_id + panel_id`;
- render não muta Document; ações disparam Commands/events;
- unload remove panel, subscriptions e state associado;
- o host preserva keyboard/focus/AccessKit e aplica automaticamente o tema atual;
- erro de render fica isolado ao panel/plugin.

Capabilities específicas incluem `register_ui_panel` e `register_viewport_overlay`. O contrato completo é normativo no capítulo 36.

# MCP Rust nativo

A mudança para Rust elimina o sidecar Go como necessidade arquitetural. `petunia-mcp` usa `rmcp` e a versão de MCP suportada pelo SDK pinado.

MCP continua sendo adapter, não dono de estado.

```
MCP runtime / Tokio
→ validated request
→ flume channel
→ Application/Main single-writer
→ Command Registry
→ result
→ MCP response
```

# Tokio isolado

Tokio existe em `petunia-mcp`, não no Geometry Core. Core/domain continuam síncronos e determinísticos.

# MCP Tools

Derivar do Command Registry sempre que possível. Baseline semântica já aprovada inclui:

```
get_document_summary
list_objects
get_object
get_selection
select
create_primitive
create_profile
extrude_profile
push_pull
move
rotate
scale
mirror
connect
weld_points
fuse
cut
slice
bevel
revolve
project_from_reference
auto_uv
pack_uv
validate_asset
export_asset
undo
redo
```

Não expor cada half-edge primitive como tool.

# MCP Resources

```
project://summary
document://objects
selection://current
object://{id}
mesh://{id}/stats
reference://{id}
validation://current
```

Resources descrevem; tools alteram.

# Schemas

Command argument/result structs tipados em Rust podem derivar JSON Schema com `schemars`. Isso permite reutilizar contratos em MCP e na validação de entrada de plugins sem transformar o core em JSON dinâmico.

# Capabilities

MCP e plugins têm autorização granular. Exemplos:

```
read_document
edit_geometry
edit_uv
edit_materials
import_files
export_files
delete_objects
invoke_plugins
register_ui_panel
register_viewport_overlay
```

Permissão é verificada pelo core/Application API também, não apenas pelo adapter.

# Stale handles

Tools e plugins validam IDs generacionais. Um handle destruído retorna erro explícito; nunca resolver silenciosamente para o conteúdo atualmente presente no mesmo índice.

# Batch atômico

MCP pode solicitar lote de commands autorizados:

```
begin logical batch
→ A
→ B
→ C
→ validate
→ commit once
```

Qualquer erro causa rollback. Não oferecer `execute arbitrary script` como alternativa.

# Events pós-commit

Plugins observam eventos de domínio depois do commit por padrão. Isso reduz reentrância e garante que subscribers sempre enxerguem um documento válido.

# Regra final

**Persistência e extensibilidade passam pelas mesmas invariantes que a UI: IDs válidos, commands, transactions, validation e Undo.**