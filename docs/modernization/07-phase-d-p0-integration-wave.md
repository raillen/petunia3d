> [← Voltar ao Índice de Modernização](/modernization/) | [Documento Integral Monolítico](/modernization/full-gauntlet)

# 9. Phase D — P0 integration wave

Integrate P0 in dependency-order, not arbitrary alphabetical order.

## D1. Foundation first

Recommended sequence:

```text
tracing + tracing-subscriber
        ↓
slotmap
        ↓
rayon + flume
        ↓
tempfile
        ↓
schemars
        ↓
cargo-deny
        ↓
proptest + cargo-fuzz
```

Why: observability, IDs, job ownership, safe IO, schemas, supply-chain policy, and test infrastructure should exist before heavy feature providers.

## D2. UI baseline, vector icons, and design-system foundation

Integrate:

- `eframe`;
- `egui_extras` with the required image features, including **`svg`**;
- **`iconflow`** with the selected built-in Petunia generic packs.

Create/finish adapters so workspaces do not import ecosystem crates directly.

### D2.1 Icon migration

Audit every icon source in:

```text
assets/ui/icons/**
crates/ui/src/app_icons.rs
crates/ui/src/icons.rs
crates/ui/src/icon_registry.rs
toolbar/menu/properties/outliner/settings widgets
```

Classify each icon as:

```text
GENERIC
DOMAIN_SPECIFIC
CONTENT_RASTER
TEMPORARY_MIGRATION
DEAD/UNUSED
```

Then migrate:

```text
GENERIC
→ semantic IconId
→ iconflow

DOMAIN_SPECIFIC
→ canonical Petunia SVG
→ SVG loader/cache
→ semantic IconId

CONTENT_RASTER
→ PNG/JPEG/WebP allowed when raster is appropriate
```

The current PNG toolbar/property UI-icon pipeline must not survive as the canonical production path merely to avoid migration work.

### D2.2 Icon visual and accessibility gates

For every built-in selectable pack:

- verify all required `IconId` mappings;
- verify fallback;
- verify no missing glyph/tofu;
- verify 16/18/20/24/32 px-equivalent readability where used;
- verify active/hover/disabled tinting;
- verify high-DPI rendering;
- verify light/dark theme contrast;
- verify icon-only accessible names/tooltips;
- verify command meaning does not change between packs.

### D2.3 Theme-token foundation

Before broad visual refactoring, generate a machine-readable **UI visual inventory** from the current code and map every themeable value to a semantic Petunia token.

Create a token schema/version and a completeness validator before declaring any theme fixed.


## D3. Geometry baseline

Integrate:

- `geo`;
- `manifold-rust`.

Add conversion validation around external provider boundaries:

- finite coordinates;
- valid indices;
- winding policy;
- manifold expectations;
- material/attribute preservation policy;
- failure diagnostics;
- cancellation if operation is jobified.

Add fixtures for:

- simple union;
- disjoint union;
- subtraction;
- intersecting faces;
- degenerate input;
- empty input;
- large low-poly fixture;
- deterministic/repeatability expectations where meaningful.

## D4. IO baseline

Integrate:

- `gltf-json`;
- `tobj`.

The Importer/Exporter contract remains headless.

Required tests:

- valid fixture;
- malformed file;
- unsupported feature;
- missing external resource;
- path traversal attempt where relevant;
- unit/axis conversion;
- normals/UV/material mapping;
- deterministic output where expected.

## D5. P0 Gauntlet

Run the full loop for each group, then a combined P0 loop.

No P0 dependency is complete merely because it appears in `Cargo.toml`.

Record:
`docs/audits/stack-modernization/30-p0-integration-gauntlet.md`.

---
