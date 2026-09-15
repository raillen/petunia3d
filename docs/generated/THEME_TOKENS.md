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

| Token Semântico | Função no Design | Petunia Dark | Petunia Light | Capuccino | Tokyo Nights |
| :--- | :--- | :---: | :---: | :---: | :---: |
| `ThemeToken::BgCanvas` | Fundo geral da área de visualização 3D (Canvas) | `#17181C` | `#EAECEF` | `#231E1C` | `#13141F` |
| `ThemeToken::BgHeader` | Barra de menu principal superior e cabeçalho da aplicação | `#202126` | `#F3F4F6` | `#2B2522` | `#1A1B26` |
| `ThemeToken::BgPanel` | Fundo das barras laterais (Toolbar, Outliner, Properties) | `#24252A` | `#F8F9FA` | `#302926` | `#1F2335` |
| `ThemeToken::BgPanelHeader` | Cabeçalho e divisores de seções dos painéis laterais | `#292A30` | `#E9EBED` | `#362E2A` | `#24283B` |
| `ThemeToken::BgSurface` | Superfície de widgets, caixas de entrada e botões em repouso | `#30323A` | `#FFFFFF` | `#3F3631` | `#292E42` |
| `ThemeToken::BgSurfaceHover` | Superfície de widgets com realce de cursor (hover) | `#3B3D47` | `#E2E5E9` | `#4D423C` | `#3B4261` |
| `ThemeToken::BgSurfaceActive` | Superfície de widgets em estado ativo/pressionado | `#474A56` | `#D1D5DB` | `#5C4F48` | `#414868` |
| `ThemeToken::TextPrimary` | Texto de máxima ênfase (títulos, etiquetas principais) | `#F0F2F5` | `#1F2937` | `#F5EDE8` | `#C0CAF5` |
| `ThemeToken::TextSecondary` | Texto de média ênfase (descrições, valores numéricos) | `#A3A7B5` | `#4B5563` | `#C4B5AC` | `#9AA5CE` |
| `ThemeToken::TextMuted` | Texto atenuado e atalhos secundários | `#6E7280` | `#9CA3AF` | `#8C7E75` | `#565F89` |
| `ThemeToken::TextActive` | Texto sobre fundo de destaque (seleção ativa) | `#FFFFFF` | `#111827` | `#FFFFFF` | `#FFFFFF` |
| `ThemeToken::AccentBlue` | Cor de destaque principal (seleção de objetos e foco) | `#3B82F6` | `#2563EB` | `#D97736` | `#7AA2F7` |
| `ThemeToken::AccentOrange` | Cor de destaque secundária (transformações e alertas) | `#F97316` | `#EA580C` | `#F59E0B` | `#BB9AF7` |
| `ThemeToken::AccentHover` | Realce sobre botões ou elementos de destaque | `#60A5FA` | `#3B82F6` | `#EA8C4A` | `#89B4FA` |
| `ThemeToken::AccentBorder` | Bordas de destaque e anéis de foco ativo | `#2563EB` | `#1D4ED8` | `#B45309` | `#7DCFFF` |
| `ThemeToken::BorderSubtle` | Divisores sutis entre seções e painéis | `#32343C` | `#E5E7EB` | `#3D342F` | `#292E42` |
| `ThemeToken::BorderStrong` | Bordas pronunciadas de caixas de diálogo e popups | `#434652` | `#CBD5E1` | `#52463F` | `#3B4261` |
| `ThemeToken::BorderFocus` | Anel de foco acessível de teclado e widgets | `#3B82F6` | `#2563EB` | `#D97736` | `#7AA2F7` |
| `ThemeToken::StatusInfo` | Mensagens informativas e telemetria | `#38BDF8` | `#0284C7` | `#67E8F9` | `#7DCFFF` |
| `ThemeToken::StatusWarning` | Alertas de geometria não conforme ou limites | `#FBBF24` | `#D97706` | `#FCD34D` | `#E0AF68` |
| `ThemeToken::StatusError` | Erros críticos de I/O, formato corrompido ou colisão | `#F87171` | `#DC2626` | `#F87171` | `#F7768E` |
| `ThemeToken::StatusSuccess` | Confirmação de salvamento, exportação e snapshots | `#4ADE80` | `#16A34A` | `#86EFAC` | `#9ECE6A` |

