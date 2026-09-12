# Perfis de Mapeamento de Teclado — Petunia3D (`assets/keymaps`)

Este diretório contém os perfis de keymap e atalhos de teclado do Petunia3D, estruturados em formato TOML.

## Perfis Nativos

1. **`petunia-default.toml`**: Perfil padrão balanceado e ergonômico do Petunia3D.
2. **`petunia-simple.toml`**: Perfil simplificado para iniciantes com atalhos intuitivos (W/E/R para ferramentas de transformação).
3. **`petunia-notebook.toml`**: Perfil adaptado para notebooks sem teclado numérico dedicado.
4. **`blender.toml`**: Perfil com paridade completa com os atalhos clássicos do Blender (G=Grab, R=Rotate, S=Scale, Tab=Alternar Seleção, Shift+A=Adicionar).
5. **`blender-notebook.toml`**: Perfil com convenções do Blender adaptado para teclados compactos.
6. **`maya.toml`**: Perfil com convenções da suíte Autodesk Maya (Q/W/E/R).
7. **`3ds-max.toml`**: Perfil com convenções da suíte Autodesk 3ds Max.
8. **`cinema-4d.toml`**: Perfil com convenções do Maxon Cinema 4D.

## Estrutura do Arquivo TOML

```toml
[profile]
id = "petunia-default"
name = "Petunia Padrão"
description = "Perfil balanceado padrão do Petunia3D."
author = "Petunia3D Team"

[bindings]
select = "B"
translate = "G"
rotate = "R"
scale = "S"
delete = "Delete"
undo = "Ctrl+Z"
redo = "Ctrl+Y"
toggle_mode = "Tab"
measure = "M"
annotate = "D"
```

## Resolução e Detecção de Conflitos

O subsistema `petunia_config::keybinds` analisa os perfis e sinaliza colisões ou conflitos de atalhos em tempo de execução através do painel de Configurações (`⚙ Config -> Teclado`).
