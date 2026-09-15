# Arquitetura de UI (egui)

UI em modo imediato com `egui`, sem retained tree própria e sem `egui_tiles`:
o shell é composto por painéis (`Panel::top/bottom/left/right/central`) com
ordem canônica — Header → StatusBar → laterais → toolbar da viewport → centro.

## Conceitos centrais

- **`UiRegions`** (`crates/ui/src/regions.rs`): fonte única dos retângulos do
  shell; cada painel registra o seu; overlays e hit-testing leem daqui.
  Invariantes testados (ex.: laterais nunca invadem a status bar).
- **Render-on-demand**: a UI redesenha sob eventos e mutações, não por frame.
- **Estado**: `AppState` (core, sem backend); UI nunca duplica domínio;
  preferências de sessão em `UiState`, persistentes só quando o app persiste.
- **Design system** (`widgets.rs` + `tokens.rs`): botões, menus e campos
  canônicos com foco visível, `widget_info` e tooltips; zero cores hardcoded.
- **Ícones**: `IconRegistry` (iconflow Lucide/Iconoir + Phosphor de fallback +
  arte vetorial própria); setas e símbolos via pintura vetorial, nunca glifo
  de fonte frágil.
- **i18n**: chaves `TextId` + TOML por idioma, paridade en/pt-BR em CI.

Mapa completo: [Mapa de Componentes UI](./ui-component-map.md).
