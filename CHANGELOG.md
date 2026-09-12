# Changelog

Todas as alterações notáveis deste projeto são documentadas neste arquivo.
O formato baseia-se no [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/) e adere ao [Semantic Versioning](https://semver.org/lang/pt-BR/).

## [0.3.0] - 2026-09-12 — Canonical Desktop UI Architecture (`Blender.svg` Golden Reference)

### Adicionado
- **Design Tokens Canônicos (`crates/ui/src/tokens.rs`)**: Centralização da paleta Dark Theme profissional do Blender (`#121212`, `#1a1a1a`, `#202020`, `#2d2d2d`, `#3169e3`), raios de curvatura de controles/pílulas e métricas de layout.
- **Gerenciador de Ícones Nativos com Tingimento Dinâmico (`crates/ui/src/app_icons.rs`)**: Embutimento em tempo de compilação dos 9 PNGs transparentes extraídos do Figma (`assets/ui/icons/toolbar/*.png`), tingimento dinâmico com base no estado do botão (repouso `#BCBCBC`, hover `#FFFFFF`, ativo `#3169E3` com ícone branco) e fallback vetorial seamlessly integrado.
- **Cabeçalho Superior Principal (`crates/ui/src/main_header.rs`)**: Menus do sistema (File, Edit, Render, Window, Help), branding Petunia3D e abas de workspaces em pílulas arredondadas.
- **Barra de Contexto do Viewport 3D (`crates/ui/src/viewport_bar.rs`)**: Seletor de modo com badges coloridos (Object/Edit/Paint), botões de seleção de componentes (Vértice [1], Aresta [2], Face [3]), orientação de transformação, ponto de pivô, snapping magnético, edição proporcional e os 4 modos canônicos de sombreamento (Wireframe, Solid, Material, Render).
- **Barra Lateral de Ferramentas (`crates/ui/src/toolbar.rs`)**: Toolbar compacta de 40px consumindo os ícones rasterizados reais para ferramentas de interação/transformação e ferramentas de modelagem poligonal com dicas de atalhos.
- **Painel Outliner Hierárquico (`crates/ui/src/outliner.rs`)**: Cabeçalho com modo de visualização, filtro de busca instantânea, botão de nova coleção e árvore hierárquica da cena com toggles de visibilidade e renderização.
- **Painel de Propriedades Modular (`crates/ui/src/properties_panel.rs`)**: Barra de navegação por abas (`Tool`, `Render`, `Output`, `Scene`, `World`, `Object`, `Modifiers`, `Data`, `Material`) com seções sanfonadas, campos de transformação coloridos por eixo (X vermelho, Y verde, Z azul) e integração com ferramentas ativas.
- **Painel de Timeline de Animação (`crates/ui/src/timeline.rs`)**: Controles de transporte completo (`|<<`, `<|`, `Play/Pause`, `|>`, `>>|`), contador numérico de frames, intervalo (Start/End) e régua de scrubbing temporal com cursor interativo.
- **Barra de Status Inferior (`crates/ui/src/status_bar.rs`)**: Dicas contextuais dos botões do mouse, mensagens de status do sistema e telemetria de malha e desempenho em tempo real.
- **Documentação de Diretórios**: READMEs estruturados em `assets/ui/icons/` e `assets/ui/icons/toolbar/` e atualização da arquitetura em `crates/ui/src/README.md`.

## [0.2.0] - 2026-09-12

### Adicionado
- Estrutura topológica Half-Edge (`HalfEdgeMesh`) com detecção de 2-variedades, anomalias de 1-anel via BFS e relatório de defeitos (`TopologyReport`).
- Operações geométricas avançadas: Método de Newell para normais poligonais, triangulação em leque para N-gons arbitrários ($N \ge 3$), fatiamento planar com fechamento de tampas e soldagem de costuras (`cut_edge_cache`), varredura ao longo de polilinhas com RMF (`sweep`), ponte entre loops de faces com minimização cíclica de distância (`connect_loops`), dissolução de arestas/vértices (`dissolve_selected`), inversão e recálculo unificado de normais.
- Modos de sombreamento no pipeline de renderização: Flat, Smooth (normais interpoladas por vértice) e Unlit em OpenGL 3.3 Core e WebGPU (WGSL).
- Suporte a 6 planos ortogonais de imagens de referência (Front, Back, Left, Right, Top, Bottom) com ângulo de rotação arbitrário e modo X-Ray em split-pass (renderizado após a geometria sólida com bypass de profundidade).
- Otimização de barramento PCIe no WebGPU com cache de hash FNV-1a para uploads de textura sob demanda.
- Ferramentas de interface e modelagem: Slice, Connect e Dissolve na barra de ferramentas esquerda com área de rolagem vertical responsiva e ativação não-destrutiva; seleção por caixa (`box_select`) com descarte de vértices atrás da câmera; inversão completa de seleção sincronizada (`invert_selection`).
- Preservação do índice do asset ativo na remoção de assets precedentes em `Project::remove`.
- Sincronização automática de paleta ativa com a paleta do projeto em operações de `undo()` e `redo()`.
- Cobertura expandida para 48 testes automatizados sem falhas e 0 warnings no Clippy com `-D warnings`.
- Validação contínua do ciclo de vida da aplicação com teste de fumaça headless (`petunia3d --smoke-test`).

## [0.1.0] - 2026-09-12

### Adicionado
- Inicialização da estrutura canônica do Prumo v0.5.
- Configuração do manifesto `prumo.json` e orquestração `.ai/`.
- Definição da hierarquia de documentação canônica em `docs/`.
- Contrato estrito de arquitetura em `docs/architecture/clean-code-contract.md`.
- Estratégia de testes exaustivos em `docs/development/testing-strategy.md` (unitários, integração, conformidade, segurança SAST/secrets, performance/stress, UI).
- Política de documentação mandatória com `README.md` explicativo em cada diretório do projeto.

## 2026-09-12 — Premium viewport, second implementation round (UI, Ortho Camera & Vector Icons)

- Implementada câmera ortográfica explícita (`Projection::Ortho`) com 6 vistas predefinidas (`Front`, `Back`, `Right`, `Left`, `Top`, `Bottom`), atalhos de Numpad e controle contínuo de altura de enquadramento.
- Criado motor de ícones vetoriais nativos em grade 24x24 (`crates/ui/src/icons.rs`) e componente acessível `tool_button` com variantes compacta (40x40) e expandida.
- Implementados campos de preenchimento numérico direto para ferramentas de transformação e modelagem (`crates/ui/src/tool_fields.rs`) sincronizados com a máquina de estados modal.
- Interface responsiva com adaptação a janelas estreitas, status bar aprimorada e tokens de tema com conformidade de contraste WCAG 2.2 AA.
- Suíte de testes expandida para 130 testes automatizados com 100% de aprovação, Clippy com 0 warnings e formatação canônica.

## 2026-09-12 — Premium viewport, first implementation round

- Added transactional modal previews, viewport HUD, exact input, axis/plane
  constraints, snapping, transform gizmos and visible-component hover/picking.
- Added quad loop preview/slide, welded knife segments, planar slice gestures,
  atomic vertex-paint strokes and contextual brush radius/eyedropper input.
- Fixed region extrusion topology, closed single-edge bevel, capped slice
  half-space semantics, Top/Bottom camera math and viewport input/redraw ordering.
- Standardized 1/2/3/4 selection, G/R/S, P, Ctrl+R, K/Shift+K and camera shortcuts.
- Added independent review and domain/egui regression tests. Historical premium
  convergence claims are superseded; remaining criteria are explicit in the plan.
