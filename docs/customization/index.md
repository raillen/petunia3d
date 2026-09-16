# Personalização

O Petunia3D adota a política estrita de **zero hardcode**: aparência vem de tokens
semânticos (`ThemeToken`), ícones de `IconId`, texto de `TextId` e ações de `CommandId`.
Você personaliza profundamente — sem que o aplicativo se fragmente.

## O que você pode personalizar

- **[Temas visuais](./themes)** — tema oficial da V1 é **Dark**, com **High Contrast**
  oficial de acessibilidade. Temas adicionais são pacotes declarativos
  `.petunia-theme` (criar, editar, importar, exportar e compartilhar, **sem código**).
- **[Pacotes de ícones](./icon-packs)** — pack ativo + Petunia Custom Icons para
  conceitos 3D sem representação genérica adequada.
- **[Tradução & idiomas (i18n)](./translations)** — arquivos TOML, com paridade exigida
  entre locales.
- **[Perfis de atalhos (keymaps)](./keymaps)** — **8 presets oficiais**, default
  **Petunia**, com remapping por `CommandId`.

## Regras que a personalização não pode quebrar

Um tema ou pacote **não** pode reduzir silenciosamente: minimum hit areas, semântica de
foco, accessible names/roles, comportamento de teclado, topologia do shell, tamanho
mínimo do viewport, contraste mínimo do modo High Contrast, indicadores de segurança e
semântica de ação destrutiva.

Plugin panels também **herdam** o tema atual e usam variants semânticos
(`normal`, `accent`, `success`, `warning`, `danger`, `muted`) — nunca RGB/hex local.

> **Petunia deve ser personalizável sem ser fragmentável.** Ajustes finos são *tuning*;
> mudanças de toolkit, grafo do shell, docking irrestrito, acesso cru de plugin a UI/GPU
> ou invariantes de acessibilidade exigem decisão arquitetural explícita.
