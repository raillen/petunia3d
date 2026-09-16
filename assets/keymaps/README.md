# Perfis de Mapeamento de Teclado (`assets/keymaps/`)

Este diretório é a **fonte única** dos perfis de keymap do Petunia3D (P3D-090).
Não existe um segundo diretório de keybinds: cada perfil é um único `.toml` aqui.
Perfis editados pelo usuário vivem no diretório de configuração do sistema e têm
precedência sobre os perfis embarcados.

## Perfis nativos

1. **`petunia-default.toml`** — perfil canônico do Petunia3D (default).
2. **`petunia-simple.toml`** — conjunto reduzido para quem está começando.
3. **`petunia-notebook.toml`** — adaptado para notebooks sem teclado numérico.
4. **`blender.toml`** — paridade com o padrão clássico do Blender (G/R/S, E, I, Ctrl+B, Shift+A, Tab).
5. **`blender-notebook.toml`** — padrão Blender adaptado a teclados compactos.
6. **`maya.toml`** — convenções do Autodesk Maya (Q/W/E/R).
7. **`3ds-max.toml`** — convenções do Autodesk 3ds Max.
8. **`cinema-4d.toml`** — convenções do Maxon Cinema 4D.

## Estrutura do arquivo

O arquivo tem uma seção `[profile]` com metadados e, em seguida, **uma seção por
namespace de ação**. A chave completa de uma ação é `<namespace>.<ação>` — é
essa string que aparece no Command Registry, na Cheat Sheet e no editor de
atalhos.

```toml
[profile]
id = "petunia-default"
name = "Petunia Padrão"
description = "Mapa de teclas canônico do Petunia3D."

[global]
undo = "Ctrl+Z"
redo = "Ctrl+Shift+Z"
save_project = "Ctrl+S"
cycle_mode = "Tab"

[model]
select_vertex = "1"
select_edge = "2"
select_face = "3"
select_object = "0"
move = "G"
rotate = "R"
scale = "S"
extrude = "E"

[paint]
paint = "B"
```

Ações desconhecidas são ignoradas; ações ausentes caem no default interno. Isso
mantém um perfil parcial válido.

## Resolução e conflitos

`petunia_config::keybinds` resolve a precedência na ordem: diretório do usuário →
`assets/keymaps/` → `keymaps/` → defaults internos. A detecção de conflitos
(P3D-091) distingue:

- **Exato** — mesmo namespace usando o mesmo atalho;
- **Sobreposição global** — atalho global sombreando um atalho contextual;
- **Tecla reservada** — uso de tecla protegida do sistema.

Os resultados aparecem no editor de atalhos de **Configurações → Teclado**, que
também exporta o mapa atual de volta para TOML.
