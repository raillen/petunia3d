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

- **MODEL** — primitivas, Draw Profile (extrude/revolve), Extrude,
  Push/Pull, Inset, Bevel, Subdivide, Mirror, Merge + referências.
- **PAINT** — vertex paint (brush/fill/eyedropper+Alt/palette) e
  canvas albedo 2D com preview texturizado no viewport.
- **UV** — editor sincronizado com a seleção, projeção planar,
  mover/escalar ilhas.
- **EXPORT** — relatório + lote OBJ (pasta) ou GLB (arquivo).

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
│   ├── module-model/      # 12 ferramentas plugáveis
│   ├── module-paint/      # vertex paint + canvas
│   ├── module-uv/         # editor UV
│   ├── module-assets/     # asset library
│   ├── ui/                # layout egui (pílulas, painéis, viewport, gizmos)
│   ├── app/               # Core, backends, loop render-on-demand
│   ├── cli/               # CLI headless puro para automação e pipelines
│   ├── ffi/               # Camada C-ABI e include/petunia.h
│   └── xtask/             # Automação de CI/CD e prevenção de drift documental
├── assets/                # locales, keybinds, themes, ícones vetoriais/raster
└── docs/                  # Documentação VitePress, Manuais e Bíblia de Implementação (SSOT)
```

## Documentação & Fonte Única da Verdade

- **Site Oficial de Documentação**: Navegue em `docs/` ou execute `cargo xtask docs`.
- **Bíblia de Implementação (SSOT)**: [`docs/bible/index.md`](docs/bible/index.md) reúne 155 especificações P3D, 17 capítulos constitucionais, 15 seções temáticas, 3 adendos e 36 capítulos de fundação.

## Qualidade & Status Atual

- **Testes Automatizados**: **277+ testes** passando em todo o workspace (`cargo test --workspace`).
- **Linter & Formatação**: `cargo clippy --workspace -- -D warnings` e `cargo fmt --check` 100% limpos.
- **Portões de Integridade**: `cargo xtask arch-check` e `cargo xtask docs-check` aprovados.
- **Progresso de Waves**: Waves 0 a 6 concluídas; Wave 7 (Materials, Texture, UV & Paint) em planejamento ativo.
