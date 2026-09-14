> [← Voltar ao Índice de Modernização](/modernization/) | [Documento Integral Monolítico](/modernization/full-gauntlet)

# 10. Phase E — P1 integration wave

Split P1 into product infrastructure and development tooling.

## E1. Product infrastructure

Integrate in this order:

```text
zip
  ↓
mlua
  ↓
rmcp + tokio
  ↓
xatlas-rs
  ↓
notify
  ↓
egui_inbox
  ↓
egui_taffy
  ↓
twill
  ↓
egui_commonmark
  ↓
egui_autocomplete
  ↓
egui_hotkey
```

### Package container (`zip`)

Do not simply zip arbitrary project directories.

Define:

- package manifest;
- format version;
- contained paths;
- size limits;
- compression policy;
- atomic write;
- migration;
- extraction containment;
- cleanup on cancel/failure;
- test fixtures.

### Lua (`mlua`)

Define capabilities before exposing APIs.

At minimum:

```text
plugin identity
plugin API version
declared capabilities
commands allowed
queries allowed
filesystem roots if any
network policy
destructive-operation policy
diagnostics
timeout/resource strategy where possible
```

Plugin APIs operate through Petunia Commands/Queries/DTOs.

### MCP (`rmcp` + `tokio`)

MCP must not become a second application architecture.

Flow:

```text
MCP transport
    ↓
Petunia MCP adapter
    ↓
capability/validation
    ↓
Command / Query
    ↓
Application
    ↓
Core
```

### xatlas

Wrap as a provider.

Never make xatlas-specific UV structures the canonical project representation.

### notify

Route file events into controlled service messages. Re-read and validate the file after events; filesystem events are hints, not trusted content.

### egui_inbox

Use as a UI bridge, not as domain ownership.

### egui_taffy

Use for selected complex sublayouts. Compare complexity and performance against native egui layout before broad adoption.

### Twill and theme infrastructure

Use Twill only behind the Petunia theme/foundation adapter.

The objective is not “use Twill everywhere.” The objective is:

```text
semantic Petunia tokens
        ↓
validated theme data
        ↓
type-safe style/variant construction
        ↓
egui Style + custom Petunia Components
```

Run a before/after scan of hardcoded UI colors, spacing, radii, dimensions, font sizes, and state visuals. The count must trend toward zero outside approved foundation/theme files.

### Markdown/autocomplete/hotkey

Keep all semantics owned by Petunia:

- Markdown is presentation;
- autocomplete returns CommandIds/search items;
- hotkey capture edits the Petunia keymap.

## E2. Development tooling

Integrate:

- `criterion`;
- `egui_inspection`;
- `egui_mcp`;
- `egui_probe`;
- `puffin`;
- `puffin_egui`;
- `assert_fs`;
- `insta`.

Create a unified `petunia-ui/devtools` entry rather than scattered debug windows.

Development build behavior should allow:

```text
Inspect UI tree
Capture screenshot
Inject input
Run UI scenarios
Open profiler
Inspect selected debug state
Review structured logs
```

Distribution behavior:

```text
Remote UI inspection OFF
UI MCP OFF
Probe panels OFF
Profiler UI OFF unless explicitly enabled
No listening debug server
```

## E3. P1 Gauntlet

In addition to normal gates:

- run a real agent-driven UI smoke flow through the inspection/MCP stack if technically possible;
- verify it binds only to intended development interfaces;
- verify production/default release does not expose it;
- benchmark representative paths before/after taffy/inbox/profiling changes.

Record:
`docs/audits/stack-modernization/40-p1-integration-gauntlet.md`.

---
