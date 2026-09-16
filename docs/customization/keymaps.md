# Perfis de Atalhos (Keymaps)

O Petunia3D inclui **8 presets oficiais** de teclado. O default do produto é
**Petunia** — os perfis `-like` existem para familiaridade, não para emular outro
software por completo (comandos que não existem no Petunia **não** são inventados para
fechar a semelhança).

| # | Preset | Para quem |
| :--- | :--- | :--- |
| 1 | **Petunia** (default) | fluxo canônico: acesso direto a ferramentas e navegação ágil, sem depender de numpad |
| 2 | **Petunia Simple** | iniciantes: poucos atalhos essenciais, o resto por menus/command palette |
| 3 | **Petunia Notebook** | laptops e teclados compactos: zero numpad e nenhuma F-key essencial |
| 4 | **Blender-like** | memória motora do Blender onde não conflita com a filosofia do Petunia |
| 5 | **Blender-like Notebook** | o mesmo, com vistas acessíveis sem numpad |
| 6 | **Maya-like** | `Q/W/E/R` + navegação `Alt+mouse` |
| 7 | **3ds Max-like** | `Q/W/E/R` + padrões de edição do 3ds Max |
| 8 | **Cinema 4D-like** | `E/R/T` para mover/rotacionar/escalar |

## Como o remapping funciona

- Todo bind aponta para um **`CommandId`** (ex.: `model.bevel`, `select.cycle_domain`) —
  nenhuma ferramenta conhece tecla física como regra de negócio.
- **Contextos**: Global, Viewport, Selection Domain, Outliner, Properties, Paint, UV,
  Timeline, TextInput e Modal, com prioridade definida.
- **Recursos**: busca de comando, captura de teclas, detecção de conflito, reset por
  command/categoria/preset, múltiplos bindings por ação e *unbound* explícito.
- **Import/export** em JSON versionado.

## Onde os perfis vivem

Os perfis canônicos ficam em `assets/keymaps/<perfil>.toml`; o default é
`assets/keymaps/petunia-default.toml`. O catálogo completo, sempre derivado do código,
está em [Catálogo de Perfis e Atalhos](../generated/KEYBINDS.md), e o resumo do perfil
default em [Cheatsheet](../shortcuts/cheatsheet.md).

> ⚠️ Não use `assets/keybinds/petunia.toml` como referência de default: ele é um
> template legado. A fonte é `assets/keymaps/`.
