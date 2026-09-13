# P3D-003 — Biblioteca de Modelos por Projeto

<aside>
🗂️

**Specification Status:** consolidada para implementação incremental. **Implementation Status:** auditar no código. A Project Model Library é um gerenciador completo do projeto, distinto do Asset Browser compacto integrado ao workspace.

</aside>

# Objetivo

Criar uma biblioteca de modelos/assets por projeto capaz de organizar, pesquisar, filtrar, classificar, visualizar e reutilizar conteúdo sem transformar a sidebar principal em um gerenciador pesado.

# Referências canônicas relacionadas

- [16 — Documento, Formato de Projeto, Undo, Autosave e Recovery](https://app.notion.com/p/16-Documento-Formato-de-Projeto-Undo-Autosave-e-Recovery-3d79bb7d023f812e80d2ebd9fe12f7a5?pvs=21)
- [23 — Macroarquitetura da Interface Petunia3D](https://app.notion.com/p/23-Macroarquitetura-da-Interface-Petunia3D-3d79bb7d023f8148ad6cff403996f066?pvs=21)
- [24 — Design System Visual: Tokens, Hierarquia e Estados](https://app.notion.com/p/24-Design-System-Visual-Tokens-Hierarquia-e-Estados-3d79bb7d023f81558f75c53d1092b9e2?pvs=21)
- [25 — Biblioteca de Componentes e Contratos de Interação](https://app.notion.com/p/25-Biblioteca-de-Componentes-e-Contratos-de-Intera-o-3d79bb7d023f815395c0d4da4adea534?pvs=21)

# Distinção fundamental

- **Project Model Library:** janela/utility view completa para gerenciamento, metadata e organização.
- **Asset Browser:** visão compacta no workspace para descoberta/uso rápido.

Não duplicar regras de dados; ambos consomem o mesmo ModelLibraryService/index.

# Auditoria obrigatória

Localizar Asset Browser, listas de assets, thumbnails, metadata, import, IDs, cache e qualquer janela existente. Identificar duplicações e preservar o que já funciona.

# Janela principal

Abrir por `File → Project Model Library`, `Window → Project Model Library` e/ou botão/CommandId equivalente. Deve ser uma utility window não bloqueante, não um modal que impeça modelagem. Subdialogs destrutivos internos podem ser modais.

# Comportamento da janela

Suportar:

- move/drag;
- resize;
- close;
- maximize dentro da área útil da aplicação;
- restore para a geometria anterior;
- persistência adequada de tamanho/posição/estado maximizado como preferência/sessão.

Fechar a janela não descarrega a biblioteca nem fecha o projeto.

# Layout alvo

```
┌───────────────────────────────────────────────────────────────┐
│ Project Model Library                           [□] [×]       │
├───────────────────────────────────────────────────────────────┤
│ Search...                     Filter ▾ Sort ▾ [Grid][List]    │
├───────────────┬───────────────────────────────────────────────┤
│ Categories    │                                               │
│ All Models    │                  CONTENT                      │
│ Favorites     │                                               │
│ Tags          │                                               │
│  Environment  │                                               │
│  Props        │                                               │
│  Character    │                                               │
├───────────────┴───────────────────────────────────────────────┤
│ 24 models                                     Thumbnail ━●━━  │
└───────────────────────────────────────────────────────────────┘
```

A aparência final usa o Petunia Design System e deve respeitar limitações/forças do egui, evitando coordenadas mágicas.

# Search

Busca incremental por nome, tags, tipo e metadata indexada relevante. Não recarregar/rasterizar thumbnails ao digitar. Empty state deve explicar ausência de resultados e oferecer limpar filtros.

# Filtros

Suportar filtros realmente disponíveis, como tipo, tag, favorite, material/textura e datas quando houver metadata. Filtros ativos devem aparecer como chips ou representação clara, com ação de limpar.

# Sort

Opções úteis: Name A–Z/Z–A, Newest, Oldest, Recently Modified e Recently Used quando houver dado confiável.

# Grid / Thumbnail View

Modo principal recomendado. Cada card exibe thumbnail, nome curto e indicador de tipo quando útil.

## Thumbnail size

Slider/controle para aumentar e diminuir. Intervalo pode cobrir aproximadamente 64–256 px ou equivalente responsivo. A quantidade de colunas é derivada da largura disponível e do tamanho escolhido.

# List View

Tabela/lista compacta com colunas apenas quando os dados existirem, por exemplo Name, Type, Tags, Modified, Triangles. Permitir sort por header e resize de coluna quando viável sem overengineering.

# Seleção

Suportar single selection e multi-selection quando as operações fazem sentido. Planejar Ctrl/Shift/keyboard navigation respeitando o keymap/contexto da janela.

# Detail Panel

Painel lateral opcional/collapsible para asset selecionado, mostrando thumbnail, tags, stats, materiais e metadata confiável. Deve poder ser ocultado para maximizar a grade.

# Tags

Tags são metadata flexível por asset e devem suportar:

- adicionar/remover;
- criar nova tag;
- autocomplete de tags existentes;
- filtro por clique/sidebar;
- múltiplas tags;
- normalização para evitar duplicatas acidentais por case (`Props`, `props`, `PROP`).

Definir política de key normalizada versus display name se necessário.

# Semântica de multi-tag

Default recomendado: múltiplas tags ativas funcionam como AND. Se houver OR futuramente, deve ser explícito no UI.

# Favorites

Permitir marcar/desmarcar `Favorite` e filtrar por Favorites.

# Collections versus Tags

Não confundir tags com pastas/collections. Para a primeira versão, `All`, `Favorites` e `Tags` podem ser suficientes; não criar sistema de collections complexo sem necessidade.

# Adicionar conteúdo

Conforme arquitetura existente, suportar ações como:

- Add Current Model;
- Import Model / Add Existing Model;
- outras fontes somente se houver comportamento real.

Nova entrada recebe `AssetId` estável, nome, source/path, metadata e thumbnail/cache.

# Rename

Rename altera display/path conforme política, mas não muda AssetId.

# Duplicate

Duplicar asset/entrada quando suportado, gerando novo AssetId.

# Remove vs Delete

`Remove from Library` e `Delete From Disk` são conceitos diferentes. Se ambos existirem, comunicar claramente. Delete From Disk é destrutivo e exige confirmação.

# Context Menu

Quando ações existirem:

```
Open / Edit
Add to Scene / Workspace
Duplicate
Rename
─────────
Favorite
Tags ›
─────────
Reveal in File Manager
─────────
Remove from Library
Delete From Disk
```

Não mostrar comandos ainda não implementados.

# Drag & Drop

Preparar/implementar conforme infraestrutura `Library → Viewport`, com ghost/preview e indicação de target válido/inválido. Não acoplar ModelLibraryService a egui drag state.

# Thumbnail pipeline

Thumbnails devem ser cacheadas. Fluxo conceitual:

```
asset added/changed
→ thumbnail invalidated/requested
→ generation
→ cache
→ reuse
```

Nunca renderizar modelo do zero a cada frame. Falha de thumbnail usa placeholder Petunia e não invalida o asset.

# Lazy loading e memória

Para bibliotecas maiores, carregar thumbnails visíveis/próximas primeiro e usar política de cache limitada. Projetar para centenas/milhares de assets sem O(N²) óbvio, mantendo pragmatismo para V1.

# Library index

Representação conceitual:

```
AssetId
name
path/source
type
tags
favorite
created/modified quando confiável
thumbnail reference/state
```

Não revarrer filesystem complexo a cada frame.

# Missing file

Asset cujo arquivo desapareceu deve permanecer detectável como `Missing`, com ações `Relink` e `Remove`. Relink preserva AssetId, tags, favorite e metadata compatível.

# Status bar da Library

Exibir informações como número de modelos, quantidade selecionada e controle de tamanho de thumbnail. Evitar informações técnicas irrelevantes.

# Empty state

Biblioteca vazia deve apresentar orientação clara e ações reais como `Add Current Model` e `Import Model`. Busca sem resultados deve oferecer Clear Filters.

# Header e controles da janela

Header exibe título/contexto do projeto, maximize/restore e close. Não criar minimize interno inicialmente sem necessidade. Ícones via `IconId`, tooltips via `TextId` e shortcuts via KeymapResolver quando aplicável.

# Persistência de UI versus projeto

## Preferência/sessão do usuário

- window size/position;
- maximized;
- Grid/List;
- thumbnail size;
- detail panel visibility;
- possivelmente filtros temporários conforme política.

## Dados do projeto

- AssetId;
- tags;
- favorite;
- metadata persistente;
- links/paths de assets.

Search text não precisa persistir por padrão.

# Arquitetura

Possível separação adaptável ao código real:

```
Core
→ AssetId + metadata persistente
Application
→ ModelLibraryService, search/filter/sort/index operations
Infrastructure
→ filesystem/import/thumbnail generation/cache
UI
→ Library Window, Grid/List, filters, dialogs, drag visuals
```

`ModelLibraryService` não conhece `egui::Rect`, slider de thumbnail ou window state.

# Commands e events

Avaliar CommandIds como `project.library.open`, `library.add_current`, `library.import`, `library.rename`, `library.remove`, `library.delete`, `library.favorite`. Não criar CommandId para simples toggle visual como mudar Grid/List, salvo se houver motivo de automação. Events podem sinalizar AssetAdded/Removed/Updated sem virar EventBus global.

# Tokens, i18n e iconografia

Toda UI usa `TextId`, `IconId` e ThemeTokens. Ícones recomendados incluem Search, Filter, Sort, GridView, ListView, Favorite, Tag, Add, Rename, Delete, Maximize e Close. Packs Tabler/Iconoir/Phosphor/Lucide/custom continuam funcionando.

# Testes funcionais

- add/import model;
- rename;
- duplicate;
- remove/delete distinction;
- tags/create/autocomplete/normalization;
- favorites;
- search;
- filtros combinados;
- sort;
- Grid/List;
- thumbnail size;
- missing file e relink;
- metadata persistence;
- multi-select quando suportado;
- state persistence da janela.

# Testes UI

Com egui_kittest/snapshot quando apropriado:

- abrir janela;
- resize;
- maximize/restore/close;
- Grid/List;
- search/filter;
- context menu;
- empty states;
- responsive layout.

# Performance

Testar aproximadamente 100, 500 e 1000 assets com metadata/thumbnails sintéticos quando viável. Observar frame time, memória e geração de thumbnail; não exigir otimização prematura, mas corrigir regressões óbvias.

# Documentação e screenshots

Criar/atualizar manual `Project Model Library` com Grid, List, tags, filtros, search, thumbnail sizing, missing/relink e ações destrutivas. Capturar Grid View, List View, maximized, tag filtering e empty state.

# Critérios de aceitação

- [ ]  Janela dedicada existe e é não bloqueante.
- [ ]  Move/resize/maximize/restore/close funcionam.
- [ ]  Grid e List funcionam.
- [ ]  Thumbnail size é ajustável e responsivo.
- [ ]  Search, sort e filtros funcionam.
- [ ]  Tags e Favorites persistem no projeto.
- [ ]  Missing/Relink é tratado sem perder metadata.
- [ ]  Remove from Library não é confundido com Delete From Disk.
- [ ]  Thumbnails usam cache e não são regeneradas por frame.
- [ ]  Biblioteca mantém core/application agnósticos ao egui.
- [ ]  UI usa tokens, i18n e IconRegistry.
- [ ]  Testes funcionais/UI/performance relevantes passam.
- [ ]  Documentação e screenshots foram atualizados.

# Ordem de implementação recomendada

1. Auditar AssetId/project persistence.
2. Consolidar ModelLibraryService/index e metadata.
3. Implementar operações headless e testes.
4. Implementar thumbnail cache/generation boundary.
5. Implementar Library Window básica.
6. Grid/List, search/sort/filter.
7. Tags/Favorites/details/context menu.
8. Maximize/restore/persistência de UI.
9. Drag/drop e refinamentos quando couberem.
10. Regression, performance, docs e Gauntlet.

# Gauntlet Loop específico

Audit → backend/headless tests → UI slice → visual review → data safety → performance → responsive/DPI → token/i18n/icon/keymap audit → documentation → score → fix → repeat. Não encerrar enquanto gaps P0/P1 do escopo permanecerem.