# Atalhos de Teclado (Shortcuts)

O Petunia3D resolve input por **keymap + `CommandId`**: nenhuma ferramenta conhece
tecla física como regra de negócio. Você pode trocar de perfil, remapear comandos,
detectar conflitos e importar/exportar seu keymap.

## Perfil default

**Petunia** é o preset canônico do produto — equilíbrio entre memória motora simples
e acesso direto às ferramentas, sem dependência de numpad.

- **[Cheatsheet do perfil Petunia](./cheatsheet)** — gerado a partir do keymap canônico.
- **[Catálogo completo de perfis e atalhos](../generated/KEYBINDS.md)** — todos os
  perfis, todos os binds (gerado automaticamente).

## Presets oficiais

| Preset | Para quem |
| :--- | :--- |
| **Petunia** (default) | fluxo canônico do produto |
| **Petunia Simple** | iniciantes; poucos atalhos essenciais, o resto por menus/command palette |
| **Petunia Notebook** | laptops e teclados compactos; zero dependência de numpad e de F-keys essenciais |
| **Blender-like** | familiaridade com Blender, sem copiar o que não existe no Petunia |
| **Blender-like Notebook** | muscle memory do Blender adaptada a teclado sem numpad |
| **Maya-like** | Q/W/E/R + navegação estilo Maya |
| **3ds Max-like** | Q/W/E/R + padrões de edição do 3ds Max |
| **Cinema 4D-like** | E/R/T para mover/rotacionar/escalar |

Guias de familiaridade:

- **[Guia para usuários do Blender](./blender)** — equivalências e diferenças conscientes.

## Como funciona o remapping

- Binds apontam para `CommandId` (ex.: `model.bevel`, `select.cycle_domain`).
- Contextos: Global, Viewport, Selection Domain, Outliner, Properties, Paint, UV,
  Timeline, TextInput e Modal, com prioridade definida.
- Recursos: busca, captura de teclas, detecção de conflito, reset por
  command/categoria/preset, múltiplos bindings por ação e *unbound* explícito.
- Import/export em JSON versionado.

> ⚠️ **Nunca edite `docs/generated/KEYBINDS.md` nem `docs/shortcuts/cheatsheet.md` à mão.**
> Ambos são gerados do código: `cargo run -p xtask -- docs-generate`.
