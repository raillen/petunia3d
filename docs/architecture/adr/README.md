# Architectural Decision Records (`docs/architecture/adr/`)

## O que é este diretório?
Contém os registros formais e imutáveis de decisões arquiteturais significativas tomadas ao longo do projeto Petunia3D.

## Para que serve?
Preserva o contexto histórico, as opções avaliadas, as consequências aceitas e os critérios que justificaram cada decisão estrutural.

## Inventário
- [`001-architecture-baseline.md`](001-architecture-baseline.md): Decisão que estabelece a linha de base de Clean Architecture e separação de domínio.
- [`002-rust-opengl-stack.md`](002-rust-opengl-stack.md): Escolha técnica da stack Rust, OpenGL 3.3 Core Profile (`glow`), fallback `wgpu` e interface `egui`.
- [`../../petunia3d-livro-vivo/32-adr-odin-para-rust.md`](../../petunia3d-livro-vivo/32-adr-odin-para-rust.md): ADR histórica da transição de prototipagem em Odin para Rust.
