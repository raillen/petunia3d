# Testes de Integração e UI Flow — Petunia UI (`crates/ui/tests`)

Este diretório contém testes de integração e testes de fluxo de interface de usuário (UI flows) automatizados para o crate `petunia_ui`, utilizando `egui_kittest` para renderização determinística headless e simulação de eventos do usuário sem necessidade de display físico.

## Arquivos de Teste

- `kittest_ui_flows.rs`:
  - `test_kittest_main_header_flow`: Validação da renderização do cabeçalho principal, branding, menus e navegação de abas com alternância de layout.
  - `test_kittest_viewport_bar_flow`: Validação da barra superior de contexto do viewport, incluindo modo de edição, seleção de componentes (vértice/aresta/face) e modos de shading.
  - `test_kittest_toolbar_flow`: Validação da toolbar lateral responsiva, expansão de largura dinâmica e contextualização de ferramentas entre modo de objeto e modo de edição.
  - `test_kittest_outliner_tree_flow`: Validação do painel de hierarquia de cena baseado em `egui_ltreeview`, filtragem de busca interativa e seleção de nós.
  - `test_kittest_right_dock_regions_are_disjoint_and_safe`, `test_kittest_dock_split_fraction_resizes_sections`, `test_kittest_dock_collapse_gives_space_to_sibling`, `test_kittest_workspace_switch_preserves_dock_layout`: Invariantes estruturais do dock direito e composição por workspace via `UiRegions`.

## Execução

```bash
cargo test -p petunia_ui --test kittest_ui_flows
```
