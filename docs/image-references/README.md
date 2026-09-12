# Referências Visuais da Interface — Petunia3D (`docs/image-references/`)

Este diretório armazena imagens de referência, telas conceituais e exportações SVG/PNG derivadas do redesign de interface do Figma para o Petunia3D.

---

## 1. Arquivos de Referência do Figma

Os arquivos na raiz deste diretório representam capturas e exportações diretas das páginas e componentes do Figma:

| Arquivo SVG | Preview PNG | Descrição |
| :--- | :--- | :--- |
| `Blender.svg` | `Blender.png` | Layout geral da aplicação (1920 × 1080) com distribuição dos quatro quadrantes |
| `3d-view_bar.svg` | `3d-view_bar.png` | Barra de controles de visualização e cabeçalho do viewport 3D |
| `toolbar.svg` | `toolbar.png` | Barra lateral esquerda de ferramentas de modelagem e transformação |
| `UI Buttons.svg` | `UI Buttons.png` | Folha de estilos de botões, variantes de estado (normal, hover, ativo, desabilitado) |
| `UI Elements.svg` | `UI Elements.png` | Elementos atômicos, seletores, toggles e caixas de checagem |
| `header.svg` | `header.png` | Cabeçalho principal com abas de workspace em formato de pílula |
| `properties.svg` | `properties.png` | Painel de propriedades à direita com campos de ajuste |
| `properties_panel_header.svg` | `properties_panel_header.png` | Cabeçalho do painel de propriedades com botões de recolhimento |
| `outliner_panel_header.svg` | `outliner_panel_header.png` | Cabeçalho do outliner/árvore de objetos da cena |
| `data_panel_vertical_menu.svg` | `data_panel_vertical_menu.png` | Menu vertical de seleção de abas de dados e modificadores |
| `timeline_panel_header.svg` | `timeline_panel_header.png` | Controles de reprodução, transporte e cabeçalho da timeline |
| `timeline_list_panel.svg` | `timeline_list_panel.png` | Painel lateral de canais e lista de faixas da timeline |
| `timeline_cursor.svg` | `timeline_cursor.png` | Cursor indicador de tempo e agulha de reprodução |
| `status_bar.svg` | `status_bar.png` | Barra de status inferior com contadores de geometria e estatísticas |
| `windows_bar.svg` | `windows_bar.png` | Barra de título da janela da aplicação e menus superiores |

---

## 2. Elementos Atômicos Extraídos

Todos os elementos internos (ícones vetoriais, imagens raster embutidas, conjuntos de componentes e botões individuais) foram extraídos e organizados em subpastas individuais em:

📂 [`extracted/`](extracted/README.md)

Para re-executar a extração automática ou atualizar elementos:

```bash
python3 scripts/extract_svg_elements.py
```
