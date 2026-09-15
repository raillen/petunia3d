# Compilação & Build

## Requisitos

Rust 1.98.1+ (edition 2024). Dependências de UI no Linux para o backend
nativo (o CI instala automaticamente).

## Comandos

```bash
cargo build              # debug
cargo run --release      # app otimizado
cargo test --workspace --lib
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Backends de renderização

OpenGL-first (`glow`, GL 3.3 Core) com fallback automático a partir do wgpu:

```bash
PETUNIA_BACKEND=gl cargo run        # força OpenGL puro
PETUNIA_BACKEND=wgpu cargo run      # força wgpu
RUST_LOG=wgpu_hal=debug cargo run   # motivo de backend falhar
```

## Features

- `extended-icon-packs`: Tabler + Phosphor (exige bastante RAM para compilar).
- `help-markdown`, `palette-autocomplete`, `keymap-capture`, `devtools`: opcionais.

## Portões de integridade

```bash
cargo run -p xtask -- arch-check   # fronteiras P0/P1, manifestos, FFI, CLI
cargo run -p xtask -- docs-check   # documentação essencial + drift
cargo run -p xtask -- ui-check     # mapa da UI contra o código
```
