# 00 — Baseline congelada (Wave 0)

<aside>
🧊

**Wave 0 da Egui Ecosystem Final Push.** Este documento congela o estado da camada
de UI **antes** de qualquer refatoração de layout, para que cada wave seguinte
possa ser comparada contra números reais em vez de impressões.

Data: 2026-09-16 · Gerado a partir do código com `cargo xtask ui-guard --baseline`.

</aside>

# Como reproduzir

```bash
cargo xtask ui-guard              # relatório
cargo xtask ui-guard --baseline   # tabela desta página
cargo xtask ui-guard --strict     # falha se um tipo de adapter escapar
```

O guard varre `crates/**/*.rs` (exceto `crates/xtask`, que contém os nomes das
bibliotecas como dado, não como uso) e classifica cada ocorrência em
**foundation/adapter** (permitido) ou **produto** (candidato a migração).

# Contagens congeladas

<!-- Gerado por `cargo xtask ui-guard --baseline`. Não editar à mão. -->

| Regra | Tipo | Total | Foundation/adapter | Produto |
| :--- | :--- | ---: | ---: | ---: |
| `egui_taffy` | confinamento | 10 | 10 | 0 |
| `egui_tiles` | confinamento | 11 | 11 | 0 |
| `egui_dnd` | confinamento | 0 | 0 | 0 |
| `twill` | confinamento | 7 | 7 | 0 |
| `egui_ltreeview` | confinamento | 1 | 1 | 0 |
| `iconflow` | confinamento | 14 | 14 | 0 |
| `egui_inbox` | confinamento | 3 | 3 | 0 |
| `egui_file_dialog` | confinamento | 1 | 1 | 0 |
| `transform_gizmo` | confinamento | 1 | 1 | 0 |
| `available_width` | cheiro de produto | 45 | 31 | 14 |
| `spacing_mut` | cheiro de produto | 45 | 8 | 37 |
| `allocate_exact_size` | cheiro de produto | 54 | 23 | 31 |
| `painter_rect_filled` | cheiro de produto | 15 | 1 | 14 |
| `egui_window` | cheiro de produto | 6 | 5 | 1 |
| `egui_popup` | cheiro de produto | 6 | 6 | 0 |
| `ui_horizontal` | informativo | 153 | 34 | 119 |
| `color32_from` | informativo | 171 | 80 | 91 |

Arquivos `.rs` varridos: **194** · regras: **17**

### Arquivos de produto mais afetados

| # | Arquivo | Ocorrências |
| ---: | :--- | ---: |
| 1 | `crates/ui/src/viewport_bar.rs` | 24 |
| 2 | `crates/ui/src/outliner.rs` | 19 |
| 3 | `crates/ui/src/properties_panel.rs` | 9 |
| 4 | `crates/ui/src/asset_browser.rs` | 7 |
| 5 | `crates/ui/src/command_palette.rs` | 5 |
| 6 | `crates/ui/src/primitive_card.rs` | 5 |
| 7 | `crates/ui/src/status_bar.rs` | 5 |
| 8 | `crates/ui/src/reference_manager.rs` | 4 |
| 9 | `crates/ui/src/settings_modal.rs` | 4 |
| 10 | `crates/ui/src/toolbar.rs` | 4 |
| 11 | `crates/ui/src/asset_library_drawer.rs` | 3 |
| 12 | `crates/ui/src/contextual_shelf.rs` | 3 |
| 13 | `crates/ui/src/shell.rs` | 3 |
| 14 | `crates/ui/src/camera_controls.rs` | 1 |
| 15 | `crates/ui/src/tool_properties_popover.rs` | 1 |

<aside>
📸

Esta tabela é **regenerada** a cada wave (o comando está no cabeçalho): o que se
lê aqui é o estado atual, não o da Wave 0. Última regeneração: **Wave 6**.

O que importa não é o total (o foundation **cresce** quando adapter nasce), e sim
a coluna de produto: `available_width` de produto em **14** (era 18 ao fim da
Wave 5) e `spacing_mut` em **37** (era 48 na Wave 5); as contagens por arquivo de
`toolbar.rs`, `settings_modal.rs` e `reference_manager.rs` caíram todas, e
`lib.rs` saiu da lista (a geometria do shell vive no adapter desde a Wave 5b).

</aside>
