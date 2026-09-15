# Diretrizes de Contribuição

Este é o guia prático. As regras técnicas completas estão em
[Guia de Contribuição](../developers/contributing.md).

## Ambiente

```bash
cargo build          # debug
cargo run --release  # app otimizado
cargo test --workspace --lib
```

## Antes de abrir o PR

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p xtask -- arch-check
cargo run -p xtask -- docs-check
cargo run -p xtask -- ui-check
```

## Convenções

- Commits no imperativo com prefixo e escopo: `feat(modeling): ...`,
  `fix(ui/viewport): ...`, `docs(ui-map): ...`, `perf(app): ...`.
- Todo comportamento novo ou bugfix vem com teste (TDD).
- `CHANGELOG.md` atualizado na seção `[Unreleased]` no mesmo PR.
- Textos novos de UI em `en.toml` **e** `pt-BR.toml` (paridade cobrada em CI).
- Sem cores hardcoded na UI (tokens em `crates/ui/src/tokens.rs`).
- Sem dependências novas sem justificativa (fronteiras P0/P1).
