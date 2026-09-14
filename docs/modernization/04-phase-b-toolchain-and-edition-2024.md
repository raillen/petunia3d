> [← Voltar ao Índice de Modernização](/modernization/) | [Documento Integral Monolítico](/modernization/full-gauntlet)

# 5. Phase B — Rust toolchain and Edition 2024 migration

## B1. Add/pin the toolchain

Create or update `rust-toolchain.toml`:

```toml
[toolchain]
channel = "1.98.1"
profile = "default"
components = ["rustfmt", "clippy"]
```

Add additional compilation targets only if the project/CI actually uses them.

## B2. Centralize workspace package metadata where sensible

Prefer a root pattern such as:

```toml
[workspace.package]
edition = "2024"
license = "MIT"
```

Then member crates can use:

```toml
edition.workspace = true
license.workspace = true
```

Do not perform this mechanical cleanup if it obscures meaningful per-crate metadata. Keep descriptions/versioning where appropriate.

## B3. `rust-version`

Do not set a false MSRV.

- The active migration toolchain is Rust 1.98.1.
- egui 0.36.x requires Rust >=1.95.
- If Petunia wants MSRV 1.95, prove it in CI.
- Otherwise set a truthful project MSRV or omit it until verified.

## B4. Edition migration process

For every crate:

1. migrate syntax/lints with compiler-supported tooling;
2. inspect edition-related changes;
3. resolve unsafe/trait/import behavior explicitly;
4. run crate-local tests;
5. run workspace tests;
6. do not bundle unrelated refactors into the same commit.

## B5. Exit gate

- all workspace members compile as Edition 2024;
- Rust 1.98.1 builds the entire workspace;
- rustfmt/clippy pass;
- baseline tests still pass;
- no architecture regression.

Execute the full Gauntlet and record `docs/audits/stack-modernization/10-rust-2024-gauntlet.md`.

---
