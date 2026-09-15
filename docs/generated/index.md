---
title: Referências Geradas do Código
description: Catálogos e tabelas geradas deterministicamente pelo xtask (P3D-119)
---

<!--
  ARQUIVO GERADO AUTOMATICAMENTE — NÃO EDITE MANUALMENTE!
  Gerado deterministicamente por `cargo xtask docs` (P3D-119).
  Para atualizar execute: cargo run -p xtask -- docs
-->

# Dados e Referências Geradas do Código (`docs/generated/`)

Esta seção contém tabelas e catálogos técnicos extraídos diretamente das fontes canônicas do Petunia3D para garantir que a documentação permaneça 100% sincronizada com a implementação.

## Seções Disponíveis

- [Comandos & Ferramentas (`CommandId`)](/generated/COMMANDS): Todos os comandos registrados e suas propriedades.
- [Perfis e Atalhos de Teclado (`Keybinds`)](/generated/KEYBINDS): 8 perfis canônicos e atalhos mapeados.
- [Tokens de Ícones Semânticos (`IconId`)](/generated/ICON_TOKENS): Identificadores de iconografia.
- [Tokens de Tradução e i18n (`TextId`)](/generated/TEXT_TOKENS): Strings e chaves de internacionalização.
- [Tokens de Temas Visuais (`ThemeToken`)](/generated/THEME_TOKENS): Design system e matriz de cores.
- [Formatos 3D Suportados (`DeliveryPipeline`)](/generated/SUPPORTED_FORMATS): Matriz de capacidades de import/export.

Para regenerar ou validar estes catálogos:
```bash
cargo run -p xtask -- docs        # Regenera catálogos e compila o site
cargo run -p xtask -- docs-check  # Valida integridade e detecta drift
```
