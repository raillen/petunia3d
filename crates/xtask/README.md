# Crate `xtask` (`crates/xtask/`)

Utilitário interno de linha de comando (`cargo xtask`) para automação de tarefas do Petunia3D:
- `cargo xtask docs`: Compila o website oficial de documentação estático construído com VitePress;
- `cargo xtask docs-check`: Valida a integridade de todas as páginas essenciais de documentação e garante ausência de falhas no CI.

## Como Executar
```bash
# Ajuda
cargo run -p xtask -- help

# Checagem de integridade
cargo run -p xtask -- docs-check
```
