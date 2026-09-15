# Referência da Interface de Linha de Comando (CLI) — Petunia3D

Guia de referência para execução, variáveis de ambiente, diagnósticos e códigos de saída da aplicação.

---

## 1. Command Surface (Superfície de Comandos)

O Petunia3D pode ser invocado via Cargo ou binário compilado direto:

```bash
# Execução padrão em modo release
cargo run --release

# Execução direta do binário compilado
./target/release/simple3d-modeling [OPTIONS] [PROJECT_FILE]
```

### Argumentos Posicionais
- `[PROJECT_FILE]`: Caminho opcional para um arquivo de projeto `.petunia` ou modelo Wavefront `.obj` a ser aberto na inicialização.

### Variáveis de Ambiente Suportadas

| Variável | Valores Válidos | Descrição |
|----------|-----------------|-----------|
| `PETUNIA_BACKEND` | `gl` \| `wgpu` | Força a seleção do backend gráfico. `gl` utiliza OpenGL 3.3 puro via `glow`. `wgpu` utiliza o pipeline moderno. |
| `RUST_LOG` | `error`, `warn`, `info`, `debug`, `trace` | Controla o nível de verbosidade de logs (ex: `RUST_LOG=wgpu_hal=debug` exibe motivo de backend falhar). |
| `SIMPLE3D_SPIN` | `0` \| `1` | Quando `1`, força o loop de renderização contínuo sem repouso (utilizado para benchmarks de taxa de quadros). |

---

## 2. Examples (Exemplos de Uso)

```bash
# Forçar backend OpenGL clássico (recomendado para Intel HD 3000/4000 e drivers Mesa legados):
PETUNIA_BACKEND=gl cargo run --release

# Forçar backend moderno com depuração gráfica ativada:
PETUNIA_BACKEND=wgpu RUST_LOG=wgpu_hal=debug cargo run

# Abrir um arquivo de projeto específico diretamente:
cargo run --release -- ./assets/models/character.petunia

# Executar medições de benchmark (render loop ativo):
SIMPLE3D_SPIN=1 cargo run --release
```

---

## 3. Machine Output (Saída de Máquina e Logs)

- Em execução normal, mensagens de inicialização e seleção de backend são emitidas para `stdout`/`stderr`:
  - `[INFO petunia_render_gl::bootstrap] OpenGL Context initialized: OpenGL 3.3 Core (GLSL 330)`.
  - `[DEBUG wgpu_hal] Adapter request details...`
- Em caso de falha de driver gráfico ou arquivo ilegível, o erro estruturado é impresso no fluxo padrão de erro (`stderr`).

---

## 4. Exit Codes (Códigos de Saída)

| Código | Significado |
|--------|-------------|
| `0` | Saída limpa e bem-sucedida pelo usuário (fechamento de janela ou Ctrl+Q). |
| `1` | Erro genérico na inicialização do subsistema de janela ou backend gráfico ausente. |
| `101` | Pânico recuperável de Rust (`panic!`) interceptado em camada externa. |

---

## 5. Stable Commands (Garantia de Estabilidade)

A invocação via `PETUNIA_BACKEND=gl|wgpu` e flags de logging constituem a superfície pública estável de controle da aplicação antes da inicialização do loop gráfico interativo.

---

## 6. Comandos Headless (`petunia-cli`)

Automação e pipelines sem interface gráfica:

```bash
petunia-cli new cena.petunia Cube     # novo projeto com primitiva
petunia-cli info cena.petunia         # metadados, malhas, vértices, faces
petunia-cli convert cena.petunia cena.obj
petunia-cli transform in.petunia out.petunia --select-all --subdivide
petunia-cli bench                      # bateria de desempenho headless
petunia-cli mcp                        # servidor MCP sobre stdio (--mcp-stdio)
```

Primitivas aceitas no `new`: `Cube`, `Plane`, `Sphere`, `Cylinder8`,
`Capsule`, `Cone`. Formatos: `.petunia`, `.obj`, `.glb` (ver [matriz](./formats.md)).
