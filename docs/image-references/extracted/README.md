# Catálogo de Elementos e Assets da Interface (Figma References)

Este diretório contém os elementos atômicos da interface do Petunia3D extraídos
e separados a partir dos SVGs exportados do Figma em `docs/image-references/`.

> [!TIP]
> **Galeria Visual Interativa**: Abra o arquivo [`index.html`](index.html) em qualquer navegador
> para inspecionar visualmente todos os ícones, botões e componentes com alternância de tema (Claro/Escuro)!

## Resumo Geral dos Elementos Extraídos

- **Total de SVGs analisados**: 15
- **Ícones vetoriais isolados**: 172
- **Imagens raster embutidas (PNG)**: 23
- **Conjuntos de componentes/variantes**: 15
- **Botões e controles isolados**: 58

## Pastas por Componente / Região da Interface

| Arquivo Fonte | Ícones | Imagens PNG | Componentes | Botões | Catálogo Detalhado |
| :--- | :---: | :---: | :---: | :---: | :--- |
| `3d-view_bar.svg` | 3 | 0 | 0 | 5 | [`3d-view_bar/README.md`](3d-view_bar/README.md) |
| `Blender.svg` | 36 | 11 | 0 | 4 | [`Blender/README.md`](Blender/README.md) |
| `UI Buttons.svg` | 0 | 0 | 11 | 23 | [`UI_Buttons/README.md`](UI_Buttons/README.md) |
| `UI Elements.svg` | 0 | 1 | 2 | 0 | [`UI_Elements/README.md`](UI_Elements/README.md) |
| `data_panel_vertical_menu.svg` | 13 | 0 | 0 | 0 | [`data_panel_vertical_menu/README.md`](data_panel_vertical_menu/README.md) |
| `header.svg` | 9 | 1 | 0 | 2 | [`header/README.md`](header/README.md) |
| `outliner_panel_header.svg` | 0 | 0 | 0 | 2 | [`outliner_panel_header/README.md`](outliner_panel_header/README.md) |
| `properties.svg` | 110 | 1 | 1 | 2 | [`properties/README.md`](properties/README.md) |
| `properties_panel_header.svg` | 0 | 0 | 0 | 2 | [`properties_panel_header/README.md`](properties_panel_header/README.md) |
| `status_bar.svg` | 0 | 0 | 0 | 0 | [`status_bar/README.md`](status_bar/README.md) |
| `timeline_cursor.svg` | 0 | 0 | 0 | 0 | [`timeline_cursor/README.md`](timeline_cursor/README.md) |
| `timeline_list_panel.svg` | 0 | 0 | 0 | 2 | [`timeline_list_panel/README.md`](timeline_list_panel/README.md) |
| `timeline_panel_header.svg` | 0 | 0 | 0 | 7 | [`timeline_panel_header/README.md`](timeline_panel_header/README.md) |
| `toolbar.svg` | 0 | 9 | 1 | 9 | [`toolbar/README.md`](toolbar/README.md) |
| `windows_bar.svg` | 1 | 0 | 0 | 0 | [`windows_bar/README.md`](windows_bar/README.md) |

## Como Executar Novamente a Extração

```bash
python3 scripts/extract_svg_elements.py
```

Opções disponíveis:
- `--input-dir <caminho>`: Diretório com os SVGs de referência (padrão: `docs/image-references`)
- `--output-dir <caminho>`: Diretório de destino (padrão: `docs/image-references/extracted`)
- `--file <nome.svg>`: Processa apenas um arquivo específico