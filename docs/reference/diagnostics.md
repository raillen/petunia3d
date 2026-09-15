# Diagnóstico & GPU

## Descobrir por que um backend falha

```bash
RUST_LOG=wgpu_hal=debug cargo run
```

## Forçar backend

```bash
PETUNIA_BACKEND=gl cargo run      # OpenGL puro (GL 3.3 Core)
PETUNIA_BACKEND=wgpu cargo run    # wgpu
```

## Casos conhecidos

- **GPU antiga sem Vulkan funcional** (ex. Intel Ivy Bridge no Mesa): o app
  cai sozinho para OpenGL desktop. Se travar, force `PETUNIA_BACKEND=gl`.
- **Wayland**: diálogos de arquivo são in-canvas (sem portal nativo); clipboard
  usa caminho seguro. Problemas de janela geralmente são do compositor — teste
  com `WAYLAND_DISPLAY` unset (XWayland) para isolar.
- **Tela preta com UI visível**: superfície GPU não posicionada — confira o
  retângulo da viewport (logs de `viewport_rect`) e o backend ativo.
- **Lentidão ao mover o mouse**: resolvido — movimentos de cursor não forçam
  mais frames cheios (o egui decide o repaint).

Mais sintomas e soluções: [Solução de Problemas](../troubleshooting/).
