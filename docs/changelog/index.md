# Histórico de Versões (Changelog)

Todas as alterações notáveis do **Petunia3D** são documentadas nesta página em conformidade com o formato [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/) e [Semantic Versioning](https://semver.org/lang/pt-BR/).

---

## [0.10.0] - 2026-09-13 — Viewport Axis Locking: 3D Guide Lines, Real-Time HUD, and Viewport Bar Controls

### Adicionado
- **Linhas-Guia 3D Infinitas no Viewport (`crates/ui/src/modal_viewport.rs`, `crates/ui/src/viewport_interaction.rs`)**:
  - Renderização de linhas-guia 3D brilhantes atravessando o pivô da seleção de ponta a ponta da tela quando um eixo cartesiano é travado (`X`, `Y`, `Z`).
  - Cores canônicas de alta visibilidade do Blender (`AXIS_X` vermelho `#e03c42`, `AXIS_Y` verde `#62c934`, `AXIS_Z` azul `#3182f6`).
  - Efeito halo/glow (`Stroke(6.0px)`) com núcleo sólido (`Stroke(2.0px)`) garantindo legibilidade perfeita sobre qualquer geometria ou grid de fundo.
  - Suporte completo a planos coordenados (`Shift+X` para YZ, `Shift+Y` para XZ, `Shift+Z` para XY): traçado simultâneo dos dois eixos do plano e polígono translúcido estilizado preenchendo a região de transformação.
  - Ativação imediata também ao arrastar eixos em gizmos de malha e anotações.
- **HUD Flutuante de Alta Visibilidade no Viewport (`crates/ui/src/modal_viewport.rs`)**:
  - Cápsula/pill estilizada acompanhando o cursor de edição com fundo translúcido escuro e borda na cor do eixo travado.
  - Badge semântico de status: `[ 🔒 EIXO X ]`, `[ 🔒 EIXO Y ]`, `[ 🔒 EIXO Z ]`, `[ 🔒 PLANO YZ (Shift+X) ]`, `[ 🔒 PLANO XZ (Shift+Y) ]`, `[ 🔒 PLANO XY (Shift+Z) ]`, ou `[ 🔓 LIVRE ]`.
  - Exibição de valores numéricos digitados diretamente e guia de atalhos (`X/Y/Z: travar eixo · Shift: plano · Ctrl: snap`).
- **Controles e Indicadores de Eixo na Barra da Viewport (`crates/ui/src/viewport_bar.rs`)**:
  - Grupo dedicado no Cluster 3 de Transformação: `🔒 [ X ] [ Y ] [ Z ]`.
  - Botões interativos de alternância rápida com preenchimento sólido colorido quando ativos e estado neutro quando livres.
  - Badge dinâmico estilizado (`[ 🔒 Eixo X ]`, etc.) indicando o travamento ativo para feedback inequívoco com um único relance.
  - Capacidade bidirecional: alternar eixos durante a edição ou pré-configurar eixos antes de iniciar uma transformação modal.
- **Sincronização de Estado e Arquitetura no Núcleo (`crates/core/src/state.rs`, `crates/core/src/modal.rs`)**:
  - Campo `locked_axes: [bool; 3]` integrado no `AppState` com métodos `is_axis_locked`, `active_axis_constraint_label` e `toggle_axis_lock`.
  - Herança automática de restrições em `begin_modal` e sincronização bidirecional em tempo de execução.
  - Limpeza e reset limpo ao finalizar ou cancelar operações modais (`commit_modal` / `cancel_modal`).

---

## [0.9.0] - 2026-09-13 — Annotations & Measurements: Undo/Redo (Ctrl+Z), Dedicated Outliner Collections, Subgrouping, Strict Confinement, and Transform Properties

### Adicionado
- **Undo/Redo Transacional para Anotações e Medidas (`Ctrl+Z` / `Ctrl+Shift+Z`)**:
  - Migração de `annotations` e `measurements` para o domínio de dados persistente `Project`.
  - Checkpoint automático a cada traço finalizado, medição completada ou item excluído via `state.checkpoint()`.
- **Coleção Especializada `📝 Anotações` no Topo do Outliner**:
  - Identidade visual com ícone `📝` e cor ciano (`#00d2d3`).
  - Suporte a múltiplos subgrupos internos (`📁 Subgrupo`) e confinamento estrito.
- **Coleção Especializada `📏 Medidas` no Outliner**:
  - Posicionamento canônico com ícone `📏` e cor amarela (`#feca57`).
- **Inspetor e Propriedades de Transformação de Anotações**:
  - Seção de Transformação Completa: Location X/Y/Z, Rotation X/Y/Z e Scale X/Y/Z.
- **Manipulação Direta por Gizmo no Viewport 3D**:
  - Gizmos 3D interativos acoplados ao centro geométrico da anotação selecionada.

---

## [0.8.0] - 2026-09-12 — UI Enhancements & Interactions

### Adicionado
- Demarcação visual de vértices com hover dinâmico ciano/dourado em modo de edição;
- Expansão automática da toolbar esquerda com as 9 ferramentas de modelagem de malha;
- Renderização e picking em modo Raio-X (`Alt+Z`) com pipeline WGSL;
- Reintegração completa de imagens de referência ortogonais no viewport e no outliner;
- Criação de pastas/coleções (`📁 Coleções`), bloqueio (`🔒 Lock`) e modo de isolamento (`⌖ Isolar`).
