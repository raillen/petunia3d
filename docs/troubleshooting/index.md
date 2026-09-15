# Solução de Problemas

## O app não abre / trava na inicialização

1. Force OpenGL: `PETUNIA_BACKEND=gl cargo run`.
2. Veja o motivo: `RUST_LOG=wgpu_hal=debug cargo run`.
3. GPUs sem Vulkan funcional caem para GL sozinho; se insistir, o item 1 resolve.

## Viewport preta, UI normal

Superfície GPU fora de posição. Confira backend ativo e `viewport_rect` nos logs; redimensione a janela (força recomposição).

## Atalhos pararam de funcionar

Confira **Settings → Keymap**: outro perfil pode estar ativo (Blender, Maya, 3ds Max...). Volte a `petunia-default`. Arquivo editável em `assets/keymaps/`.

## Layout bagunçado

**Settings → Interface → Reset Current Workspace Layout** (ou Reset All UI Layouts). Divisor Scene/Inspector volta ao AUTO com duplo-clique.

## Projeto não abre / parece corrompido

O formato `.petunia` é versionado e validado (`Project::validate` normaliza legados). Tente abrir via **File → Open** e veja a status bar.
Recuperação automática (autosave) oferece restaurar na próxima abertura.

## Idioma errado

**Settings → Language**: English ou Português. Ambos têm paridade total.

Nada resolveu? Abra uma issue com versão, SO, backend e passos mínimos ([guia](../developers/contributing.md)).
