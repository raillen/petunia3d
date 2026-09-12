# Padrões de Codificação e Engenharia — Petunia3D

Diretrizes obrigatórias de engenharia de software e estilo em Rust para o projeto Petunia3D.

---

## 1. Formatação e Linter Estático

- **Formatador Oficial**: Todo o código Rust deve ser formatado estritamente através do `cargo fmt --all`. Não são aceitos desvios manuais de indentação ou posicionamento de chaves.
- **Clippy sem Avisos**: O pipeline de validação executa `cargo clippy --workspace --all-targets`. **Zero warnings** é a regra rígida; qualquer warning do clippy quebra o quality gate.
- **Edição da Linguagem**: Rust 2021 Edition.

---

## 2. Tratamento de Erros e Segurança de Memória

- **Proibição de `unwrap()` em I/O e Entradas Externas**: Nunca use `unwrap()` ou `expect()` em dados originados de arquivos (`.obj`, `.petunia`, `.png`), argumentos da linha de comando ou eventos do usuário. Use tipos `Result<T, CustomError>` com contextualização de erro.
- **Erros Tipados de Domínio**: Erros devem ser mapeados em enums expressivos (ex: `ExportError`, `MeshError`, `ConfigError`).
- **Política de `unsafe`**: Código inseguro (`unsafe`) é terminantemente proibido nas camadas de domínio, comandos, UI e projeto. Apenas o crate `render-gl` possui blocos `unsafe` estritamente contidos para chamadas FFI diretas da biblioteca OpenGL/glutin.

---

## 3. Coesão Modular e Arquitetura de Crates

- **Respeito aos Limites de Módulos**: Módulos funcionais (`module-*`) interagem apenas através de interfaces `dyn Module` e structs de `core`. É proibida a dependência mútua ou cruzada entre módulos plugáveis.
- **Estado Canônico Único**: O `AppState` em `petunia_core` é a autoridade central em tempo de execução. Mutações ocorrem no início do frame ou via despacho de eventos tipados.
- **Documentação de Itens Públicos**: Todas as funções públicas (`pub fn`), traits e structs exportados devem conter docstrings em Markdown (`///`) explicando objetivo, parâmetros e possíveis retornos de erro.
