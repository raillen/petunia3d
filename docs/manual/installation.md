# Guia de Instalação e Ciclo de Vida — Petunia3D

Instruções para instalação, compilação de código-fonte, caminhos de sistema e desinstalação segura.

---

## 1. Installation Paths (Caminhos de Instalação e Requisitos)

### Requisitos Prévios
- **Linguagem / Toolchain**: Rust estável ($\ge 1.75$) com Cargo.
- **Bibliotecas de Sistema (Linux)**:
  - Servidor de display X11 ou Wayland.
  - Driver OpenGL 3.3 funcional (Mesa ou proprietário).
  - Pacotes de desenvolvimento comuns: `libx11-dev`, `libasound2-dev`, `libudev-dev`.

### Compilação e Instalação Local

```bash
# Clone do repositório
git clone https://github.com/raillen/simple3d-modeling.git
cd simple3d-modeling

# Compilação em modo release otimizado
cargo build --release

# Instalação opcional no PATH do usuário (~/.cargo/bin)
cargo install --path .
```

### Installation Paths (Estrutura de Diretórios de Runtime)
- **Binário Executável**: `~/.cargo/bin/simple3d-modeling` ou `./target/release/simple3d-modeling`.
- **Assets e Recursos**: Pasta `assets/` relativa ao binário ou no diretório de trabalho (contendo `locales/`, `keybinds/`, `themes/`, `tools.toml`).
- **Configurações do Usuário**: `~/.config/petunia3d/` (ou diretório correspondente da plataforma).

---

## 2. Ownership (Permissões e Propriedade de Arquivos)

- Todos os arquivos instalados pertencem exclusivamente ao usuário local (`user ownership`), dispensando privilégios de superusuário (`root`/`sudo`).
- Arquivos de projeto `.petunia` criados herdam permissões de leitura/escrita do usuário ativo (0644).

---

## 3. Rollback Strategy (Estratégia de Reversão de Atualização)

- A ferramenta `cargo install` substitui o binário anterior somente após compilação completa bem-sucedida.
- Caso uma nova versão apresente regressão visual ou gráfica em GPU específica, é possível reverter para a versão anterior executando:
  ```bash
  git checkout <tag-versao-estavel>
  cargo install --path . --force
  ```
- **Interrupted Update**: Em caso de cancelamento abrupto durante a compilação, o binário ativo anterior permanece intocado e operacional.

---

## 4. Uninstall Safety (Desinstalação Segura)

- Para remover o binário instalado sem afetar projetos criados:
  ```bash
  cargo uninstall simple3d-modeling
  ```
- A remoção do binário é segura e não apaga nenhum arquivo de projeto `.petunia`, modelos exportados ou arquivos em pastas de documentos.
