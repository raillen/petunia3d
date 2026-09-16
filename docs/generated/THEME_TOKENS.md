---
title: Tokens de Temas Visuais
description: Catálogo canônico de tokens visuais ThemeToken e paletas do Design System (P3D-119)
---

<!--
  ARQUIVO GERADO AUTOMATICAMENTE — NÃO EDITE MANUALMENTE!
  Gerado deterministicamente por `cargo xtask docs` (P3D-119).
  Para atualizar execute: cargo run -p xtask -- docs
-->

# Catálogo Canônico de Tokens de Temas (`ThemeToken`)

> **Single Source of Truth (P3D-085, P3D-119)**
> O Petunia Design System define tokens semânticos universais para eliminar cores hardcoded e garantir contraste, acessibilidade e flexibilidade estética.

## Matriz Comparativa de Cores por Tema

> Temas oficiais de V1 (capítulo 36): **Petunia Dark** (completo, default) e **Petunia High Contrast** (variação oficial de acessibilidade). Temas adicionais são packs declarativos do usuário (`themes/<id>/` ou `.petunia-theme`).

| Token Semântico | Função no Design | Petunia Dark | Petunia High Contrast |
| :--- | :--- | :---: | :---: |
| `ThemeToken::BgCanvas` | Fundo geral da área de visualização 3D (Canvas) | `#17181C` | `#000000` |
| `ThemeToken::BgHeader` | Barra de menu principal superior e cabeçalho da aplicação | `#202126` | `#0A0A0A` |
| `ThemeToken::BgPanel` | Fundo das barras laterais (Toolbar, Outliner, Properties) | `#24252A` | `#121212` |
| `ThemeToken::BgPanelHeader` | Cabeçalho e divisores de seções dos painéis laterais | `#292A30` | `#1A1A1A` |
| `ThemeToken::BgSurface` | Superfície de widgets, caixas de entrada e botões em repouso | `#30323A` | `#1F1F1F` |
| `ThemeToken::BgSurfaceHover` | Superfície de widgets com realce de cursor (hover) | `#3B3D47` | `#2E2E2E` |
| `ThemeToken::BgSurfaceActive` | Superfície de widgets em estado ativo/pressionado | `#474A56` | `#3D3D3D` |
| `ThemeToken::TextPrimary` | Texto de máxima ênfase (títulos, etiquetas principais) | `#F0F2F5` | `#FFFFFF` |
| `ThemeToken::TextSecondary` | Texto de média ênfase (descrições, valores numéricos) | `#A3A7B5` | `#E6E6E6` |
| `ThemeToken::TextMuted` | Texto atenuado e atalhos secundários | `#6E7280` | `#B8B8B8` |
| `ThemeToken::TextActive` | Texto sobre fundo de destaque (seleção ativa) | `#FFFFFF` | `#FFFFFF` |
| `ThemeToken::AccentBlue` | Cor de destaque principal (seleção de objetos e foco) | `#3B82F6` | `#00B0FF` |
| `ThemeToken::AccentOrange` | Cor de destaque secundária (transformações e alertas) | `#F97316` | `#FFB000` |
| `ThemeToken::AccentHover` | Realce sobre botões ou elementos de destaque | `#60A5FA` | `#4DD0FF` |
| `ThemeToken::AccentBorder` | Bordas de destaque e anéis de foco ativo | `#2563EB` | `#FFFFFF` |
| `ThemeToken::BorderSubtle` | Divisores sutis entre seções e painéis | `#32343C` | `#6B6B6B` |
| `ThemeToken::BorderStrong` | Bordas pronunciadas de caixas de diálogo e popups | `#434652` | `#A0A0A0` |
| `ThemeToken::BorderFocus` | Anel de foco acessível de teclado e widgets | `#3B82F6` | `#FFD400` |
| `ThemeToken::StatusInfo` | Mensagens informativas e telemetria | `#38BDF8` | `#57C7FF` |
| `ThemeToken::StatusWarning` | Alertas de geometria não conforme ou limites | `#FBBF24` | `#FFD400` |
| `ThemeToken::StatusError` | Erros críticos de I/O, formato corrompido ou colisão | `#F87171` | `#FF6B6B` |
| `ThemeToken::StatusSuccess` | Confirmação de salvamento, exportação e snapshots | `#4ADE80` | `#6EE7A8` |

