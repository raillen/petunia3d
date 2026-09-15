# Petunia3D fuzzing (P0-13)

Bounded fuzz targets for trust boundaries. Requires nightly + `cargo-fuzz`:

```bash
cargo install cargo-fuzz
cargo +nightly fuzz run obj_parse -- -max_total_time=300
cargo +nightly fuzz run project_parse -- -max_total_time=300
```

## Targets

| Target | Boundary | Bound |
|---|---|---|
| `obj_parse` | OBJ loader (`tobj` via `import_obj_bytes`) | 64 KiB |
| `project_parse` | `.petunia` parser (`format::load_bytes`) | 256 KiB |

Both harnesses assert no-panic: only typed errors may escape.
The same logic runs on stable in
`crates/project/tests/fuzz_corpus.rs` (bounded hostile corpus).
