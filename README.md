# Petunia3D — editor low-poly shape-first (Rust + OpenGL)

Modelador 3D nativo focado em criação rápida de assets low-poly
(estética PS1/N64/DS/indie): desenhe a silhueta sobre a referência,
gere a malha, extrude/ajuste, faça UV e pinte — sem dominar um DCC.

## Rodar

```bash
cargo run --release
# diagnóstico:
PETUNIA_BACKEND=gl cargo run        # força OpenGL puro
PETUNIA_BACKEND=wgpu cargo run      # força wgpu
RUST_LOG=wgpu_hal=debug cargo run   # motivo de backend falhar
```

Controles: **MMB** orbita • **Shift+MMB** pan • **scroll** zoom •
**Tab** modo • **Del** apaga • **1/2/3** vértice/aresta/face •
**G/E/I/W/M/A/P/B** ferramentas • **Ctrl+Z/Y** undo •
**Home** reseta câmera • **H** ajuda. Tudo remapeável em
`assets/keybinds/petunia.toml`.

## Workspaces (pílulas no header)

- **MODEL** — primitivas (10 espécies), Draw Profile (extrude/revolve),
  Extrude, Push/Pull, Inset, Bevel, Subdivide, Mirror, Merge, Symmetrize +
  referências ortográficas.
- **PAINT** — vertex paint (brush/fill/eyedropper+Alt/palette) e
  canvas albedo 2D com preview texturizado no viewport.
- **UV** — editor sincronizado com a seleção, projeção planar,
  mover/escalar ilhas.
- **ANIMATE** — esqueletos, rigs e timeline com keyframes.

Exportação (lote OBJ em pasta ou GLB em arquivo) via diálogo de exportação.

Arquivo de projeto `.petunia` (binário versionado, UUIDs persistentes).

## Backends

OpenGL-first: renderer puro `glow` (GL 3.3 Core, GLSL 330) com fallback
automático a partir do wgpu. Em GPU antiga sem Vulkan funcional
(ex. Intel Ivy Bridge no Mesa, onde o EGL do wgpu falha), o app cai
sozinho para OpenGL desktop. Detalhes em `docs/ARCHITECTURE.md`.

## Layout

```
petunia3d/
├── src/main.rs            # binário fino (chama petunia_app::run)
├── crates/
│   ├── core/              # estado, câmera, eventos, contrato Module
│   ├── mesh/              # malha: prims/ops/uv/obj/triangulate
│   ├── commands/          # undo/redo (snapshots)
│   ├── config/            # i18n, keybinds, tema, tools.toml
│   ├── project/           # Asset(UUID)/Project, .petunia, export OBJ/GLB
│   ├── render/            # tipos + matemática compartilhada (grid, luz)
│   ├── render-gl/         # OpenGL puro (único lugar com GL)
│   ├── render-wgpu/       # wgpu (quando há GPU compatível)
│   ├── module-model/      # 16 ferramentas plugáveis
│   ├── module-paint/      # vertex paint + canvas
│   ├── module-uv/         # editor UV
│   ├── module-assets/     # asset library
│   ├── ui/                # layout egui (pílulas, painéis, viewport, gizmos)
│   ├── app/               # Core, backends, loop render-on-demand
│   ├── cli/               # CLI headless puro para automação e pipelines
│   ├── ffi/               # Camada C-ABI e include/petunia.h
│   ├── mcp/               # Fronteira MCP (ferramentas allowlisted p/ agentes)
│   ├── plugins/           # Sistema de plugins
│   └── xtask/             # Automação de CI/CD e prevenção de drift documental
├── assets/                # locales, keybinds, themes, ícones vetoriais/raster
└── docs/                  # Documentação VitePress, Manuais e Bíblia de Implementação (SSOT)
```

## Documentação & Fonte Única da Verdade

- **Site Oficial de Documentação**: Navegue em `docs/` ou execute `cargo xtask docs`.
- **Bíblia de Implementação (SSOT)**: [`docs/bible/index.md`](docs/bible/index.md) reúne 155 especificações P3D, 17 capítulos constitucionais, 15 seções temáticas, 3 adendos e 36 capítulos de fundação.

## Qualidade & Status Atual

- **Testes Automatizados**: **400+ testes** de unidade passando
  (`cargo test --workspace --lib`), mais suites de integração
  (kittest de fluxos de UI, CLI headless, malha, config com paridade i18n).
- **Linter & Formatação**: `cargo clippy --workspace --all-targets -- -D warnings`
  e `cargo fmt --all -- --check` 100% limpos.
- **Portões de Integridade**: `cargo xtask arch-check`, `docs-check` e
  `ui-check` (mapa de 193 componentes) aprovados.
- **Progresso**: Waves 0–10 concluídas (modelagem, materiais, entrega, QA,
  animação & rigging) + sistema V1 de primitivas (10 espécies) + sidebar
  direita redesenhada (Scene + Inspector contextual).

## Contribuindo

Leia [`CONTRIBUTING.md`](CONTRIBUTING.md) e o guia completo em
[`docs/developers/contributing.md`](docs/developers/contributing.md).
Resumo: TDD, `clippy`/`fmt` limpos, CHANGELOG + docs no mesmo PR,
i18n en/pt-BR em paridade.

## Licença

MIT — veja [`LICENSE`](LICENSE). Se o projeto te ajuda, considere apoiar em
[ko-fi.com/raillen](https://ko-fi.com/raillen).
