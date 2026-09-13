# Changelog

Todas as alterações notáveis deste projeto são documentadas neste arquivo.
O formato baseia-se no [Keep a Changelog](https://keepachangelog.com/pt-BR/1.0.0/) e adere ao [Semantic Versioning](https://semver.org/lang/pt-BR/).

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

## [0.9.0] - 2026-09-13 — Annotations & Measurements: Undo/Redo (Ctrl+Z), Dedicated Outliner Collections, Subgrouping, Strict Confinement, and Transform Properties

### Adicionado
- **Undo/Redo Transacional para Anotações e Medidas (`Ctrl+Z` / `Ctrl+Shift+Z`) (`crates/project/src/lib.rs`, `crates/core/src/state.rs`, `crates/ui/src/annotation.rs`, `crates/ui/src/measurement.rs`)**:
  - Migração de `annotations` e `measurements` de estruturas transitórias soltas para o domínio de dados persistente `Project`.
  - Checkpoint automático a cada traço finalizado, medição completada ou item excluído via `state.checkpoint()`.
  - Desfazer e refazer completos, imediatos e estáveis com `Ctrl+Z` e `Ctrl+Shift+Z` restaurando perfeitamente os traços e réguas.
- **Coleção Especializada `📝 Anotações` no Topo do Outliner (`crates/ui/src/outliner.rs`)**:
  - Posicionamento canônico no topo da árvore de cena, acima das coleções de malhas.
  - Identidade visual distinta com ícone `📝` e cor ciano característica (`#00d2d3`).
  - Controles coletivos e por item: alternância de visibilidade (`👁` / `⊘`) e alternância de bloqueio (`🔒` / `🔓`).
  - Suporte a múltiplos subgrupos internos (`📁 Subgrupo`) com menus contextuais para mover anotações entre subgrupos ou para a raiz da coleção.
  - **Confinamento Estrito**: Anotações residem exclusivamente na coleção de Anotações e não podem ser movidas ou mescladas em coleções de malhas 3D.
- **Coleção Especializada `📏 Medidas` no Outliner (`crates/ui/src/outliner.rs`)**:
  - Posicionamento canônico no topo da árvore de cena com ícone `📏` e cor amarela de destaque (`#feca57`).
  - Controles estritamente limitados a ocultar/exibir (`👁` / `⊘`) e exclusão (`🗑` / `X`), sem suporte a bloqueio ou transformações, conforme especificado.
- **Inspetor e Propriedades de Transformação de Anotações no Painel de Propriedades (`crates/ui/src/properties_panel.rs`)**:
  - Exibição automática das propriedades ao selecionar uma anotação na árvore ou após desenhá-la.
  - Campos de identificação (nome editável, visibilidade, bloqueio e atribuição de subgrupo).
  - Aparência do traço: seletor de cor RGBA e controle deslizante de espessura de traço.
  - **Seção de Transformação Completa**:
    * Posição (Location): `X`, `Y`, `Z` com badges semânticos coloridos.
    * Rotação (Rotation): `X`, `Y`, `Z` em graus de Euler.
    * Escala (Scale): `X`, `Y`, `Z` com alcance de `0.01..=100.0`.
    * Botão de redefinição de transformação (`↺ Redefinir Transformação`).
  - Ação de exclusão direta com botão `🗑 Deletar Anotação`.
- **Manipulação Direta por Gizmo no Viewport 3D (`crates/ui/src/viewport_interaction.rs`)**:
  - Gizmos tridimensionais (Mover, Rotacionar, Escalar) acoplados ao centro geométrico da anotação selecionada.
  - Suporte a arrasto de eixos e planos com cancelamento por `Escape` e gravação de checkpoint transacional ao soltar o mouse.
  - Respeito integral ao bloqueio individual da anotação e bloqueio global da coleção.

## [0.8.0] - 2026-09-12 — UI Enhancements & Interactions: Vertex Hover Demarcation, Toolbar Edit Tools, WGPU X-Ray, Reference Images, Outliner Collections/Lock/Isolate, and Vibrant Properties Tabs

### Adicionado
- **Demarcação Visual de Vértices no Modo de Edição (`crates/ui/src/viewport_interaction.rs`)**:
  - Quando em `EditMode::Edit` com `SelectMode::Vertex`, todos os vértices da malha ativa são desenhados de forma proeminente (pontos laranjas para selecionados, pontos escuros com contorno claro para não selecionados).
  - Hover dinâmico sobre vértices desenha um ponto interno dourado e um anel/halo externo ciano brilhante (`#64dcff`), garantindo feedback imediato de que o modo de vértices está ativo e indicando qual vértice será selecionado antes do clique.
- **Ferramentas de Modelagem na Barra de Ferramentas Esquerda (`crates/ui/src/toolbar.rs`)**:
  - Ao entrar em `EditMode::Edit`, a barra vertical esquerda expande automaticamente para exibir a paleta completa de 9 ferramentas de modelagem de malha (`Extrude`, `Inset`, `Bevel`, `Loop Cut`, `Knife`, `Push/Pull`, `Slice`, `Subdivide`, `Draw Profile`), em paralelo com a barra flutuante inferior.
- **Renderização e Picking em Modo Raio-X (`crates/render-wgpu/src/lib.rs`, `crates/app/src/lib.rs`, `crates/core/src/state.rs`)**:
  - Pipeline de shader WGSL `fs_xray` com translucidez (`alpha ~ 0.45`), `depth_write_enabled: false` e comparação de profundidade `LessEqual`.
  - Pipeline de arestas X-Ray sem teste de oclusão de profundidade (`CompareFunction::Always`), permitindo que as arestas sejam visíveis através de qualquer geometria.
  - O algoritmo de picking passa a considerar `state.show_xray`, permitindo selecionar vértices, arestas e faces ocultos atrás da superfície quando o modo Raio-X estiver ativado.
  - Atalho canônico `Alt+Z` para alternar modo Raio-X.
- **Reintegração Completa de Imagens de Referência (`crates/ui/src/lib.rs`, `viewport_bar.rs`, `contextual_shelf.rs`, `outliner.rs`)**:
  - Abertura assíncrona/nativa via `pick_and_add_reference_image` disponível tanto no menu `➕ Add+ ▾` da viewport bar quanto na barra contextual flutuante de baixo (`🖼 Referência`).
  - Seção dedicada `🖼 Imagens de Referência` no Outliner com controle de visibilidade (`👁` / `⊘`), alternância de Raio-X (`⚡`) e remoção.
- **Hierarquia de Pastas / Coleções no Outliner (`crates/project/src/lib.rs`, `crates/ui/src/outliner.rs`)**:
  - Capacidade de criar pastas/coleções (`📁 Coleções`) no cabeçalho do Outliner através do botão `📁+ Pasta`.
  - Suporte a agrupar modelos em coleções, renomeação inline, exclusão com retorno automático de itens à raiz, e alternância em lote de visibilidade e bloqueio.
  - Menu contextual nos objetos: `📁 Mover para Coleção ▾` (listando coleções existentes e raiz).
- **Bloqueio (`Lock`) e Isolamento (`Isolate`) de Modelos (`crates/project/src/lib.rs`, `crates/core/src/state.rs`, `crates/core/src/modal.rs`, `crates/ui/src/outliner.rs`)**:
  - Campo `asset.locked: bool` persistido no projeto.
  - Botão `🔒` / `🔓` no Outliner para fixar objetos, impedindo qualquer transformação modal (`ModalError::ActiveLocked`), manipulação por gizmo ou menus contextuais no viewport.
  - Botão e modo `⌖ Isolar` (atalho `Numpad /` ou `/`) que oculta temporariamente todos os outros modelos mantendo apenas o selecionado em visão local, com restauração perfeita do estado de visibilidade anterior ao desativar.
- **Abas de Propriedades Ampliadas e Coloridas Semanticamente (`crates/ui/src/widgets.rs`, `crates/ui/src/properties_panel.rs`)**:
  - Botões de categoria ampliados para `32x28px`, emoldurados em container estilizado (`tokens::BG_PANEL_HEADER`).
  - Cores semânticas vibrantes inspiradas no Blender:
    * `Tool`: Azul canônico (`#3169e3`)
    * `Object`: Laranja característico (`#e67e22`)
    * `Modifiers`: Azul-celeste (`#00a8ff`)
    * `Data`: Verde (`#2ecc71`)
    * `Material`: Magenta / Rosa (`#e84393`)
  - Indicador inferior de seleção ativa e realce refinado ao passar o cursor.

## [0.7.0] - 2026-09-12 — UI Reorganization & Ergonomics Refinement: Contextual Modeling Shelf, Retractable Asset Browser, Clean Two-Panel Sidebar & Viewport Bar 6 Clusters

### Adicionado
- **Barra Contextual Horizontal do Viewport (`crates/ui/src/contextual_shelf.rs`)**:
  - Cápsula flutuante na base inferior do Viewport 3D reagindo dinamicamente ao workspace e ao modo ativo (`Model + Edit`, `Model + Object`, `Paint`, `UV`, `Animate`).
  - Em `Model + Edit`: botões de seleção de malha (`⬝ Vértice`, `╱ Aresta`, `▨ Face`), comandos essenciais (`Extrude`, `Inset`, `Bevel`, `Loop Cut`, `Knife`) e operações topológicas (`Subdivide`, `Merge`).
  - Em `Model + Object`: atalhos de transformação (`Move`, `Rotate`, `Scale`) e primitivas rápidas (`Cubo`, `Esfera`, `Cilindro`, `Plano`) e duplicar objeto.
  - Em `Paint`: ferramentas de pincel, apagador, conta-gotas, ajuste de raio e chip da cor ativa.
  - Em `Animate`: timeline transport player (`◀◀`, `▶ Play / ⏸ Pausa`, `▶▶`) e seletor de frame.
  - Contenção e blindagem de eventos de ponteiro para evitar disparar raycasting de seleção 3D acidental durante cliques e ajustes na shelf.
- **Painel Lateral Retrátil de Navegação de Assets (`crates/ui/src/asset_browser.rs`)**:
  - Painel lateral dedicado à esquerda (220–340px) acionado pelo botão `[📦 Assets]` do cabeçalho superior.
  - Filtro por categorias (`Todos`, `Props`, `Personagens`, `Cenário`) e busca instantânea com `petunia_search_box`.
  - Cards detalhados com contagem de vértices e triângulos, swatch de cor e ações rápidas (`➕ Instanciar`, `🎯 Ativar`, `📋 Duplicar`, `🗑 Deletar`).
  - Botão de rodapé para salvar o modelo ativo atual diretamente na biblioteca do projeto (`state.save_active_as_asset()`).
- **Exportação Canônica nos Menus de Sistema (`crates/ui/src/main_header.rs`, `crates/ui/src/lib.rs`)**:
  - Reclassificação de `Export` de workspace para itens canônicos de menu: `Arquivo -> Exportar OBJ (.obj)...` e `Arquivo -> Exportar GLB (.glb)...`.
  - Introdução do workspace `ANIMATE` nas abas superiores: `[ MODEL ] [ PAINT ] [ UV ] [ ANIMATE ]`.

### Modificado
- **Reorganização Estrutural da Barra Superior da Viewport (`crates/ui/src/viewport_bar.rs`)**:
  - 6 clusters semânticos rigorosamente separados:
    1. Dropdown de Modo (`[ Object Mode ▾ ]` vs `[ Edit Mode ▾ ]`) com alvos contextuais (`⬝ Vértice`, `╱ Aresta`, `▨ Face`) exibidos **exclusivamente** em modo de edição.
    2. Menus rápidos com ícones (`👁 View ▾`, `▢ Select ▾`, `➕ Add+ ▾`) e menu contextual reativo (`🧊 Object ▾` ou `🕸 Mesh ▾`).
    3. Orientação de transformação (`Global`, `Local`, etc.) e Ponto de Pivô (`Median Point`, `3D Cursor`, etc.).
    4. Botões de Snapping Magnético (`🧲 Snap`) e Edição Proporcional (`◎ Prop`).
    5. Diagnóstico de cena (`⊞ Overlays`, `⧉ X-Ray`).
    6. 4 Modos de sombreamento esféricos canônicos do Blender (`○`, `●`, `◐`, `☼`).
  - Interceptação de atalhos de teclado globais (Tab para alternar Object/Edit, e 1/2/3 para alvos de vértice/aresta/face) tratada com fallback e garantia direta no egui sem conflitos de foco.
- **Descongestionamento e Limpeza da Sidebar Direita (`crates/ui/src/outliner.rs`, `crates/ui/src/properties_panel.rs`)**:
  - Redução estrita para apenas 2 componentes verticais: `Outliner` e `Properties`.
  - Remoção de galerias duplicadas, criação solta de primitivas no outliner e seção avulsa de exportação.
  - Aba `Material` no painel de propriedades agora abriga com exclusividade a cor base e a paleta interativa de swatches do projeto.
  - Aba `Object` refinada com identidade do objeto ativo e inspector `▾ Transform` com grid tri-axial e rótulos coloridos RGB (X, Y, Z).
- **Especialização da Barra de Ferramentas Vertical Esquerda (`crates/ui/src/toolbar.rs`)**:
  - Foco exclusivo nas 8 ferramentas primárias e persistentes de interação: `Select Box`, `3D Cursor`, `Move`, `Rotate`, `Scale`, `Transform`, `Measure` e `Annotate`.
  - Operações transitórias de modelagem de malha movidas para a Contextual Modeling Shelf.
- **Refatoração da Barra de Status Inferior (`crates/ui/src/status_bar.rs`)**:
  - Organizada em 3 blocos limpos:
    * Esquerda: indicador de projeto (`● Salvo` / `○ Não salvo`), nome do arquivo e atalhos de mouse/ferramenta ativa.
    * Centro: mensagens operacionais e feedbacks do sistema com truncate.
    * Direita: métricas agregadas da cena (`Tris: {} │ Verts: {} │ Objs: {} │ {:.1}ms │ v0.6.0`) e botões de Undo (`↩`) e Redo (`↪`).

## [0.6.0] - 2026-09-12 — Interactive Measurement & Annotation, Viewport Floating Bar, Asset Drawer, Theme & Icon Packs, Keymaps & TOML i18n

### Adicionado
- **Ferramenta Interativa de Régua e Medição 3D (`crates/ui/src/measurement.rs`, `viewport_interaction.rs`)**:
  - Medição espacial 3D com clique e arraste no viewport (`Tool::Measure`, atalho `M`).
  - Snapping magnético inteligente a vértices de malhas ativas com indicador circular visual.
  - Projeção de planos cartesianos e ray intersection tridimensional.
  - Régua com marcações métricas de graduação a cada 0.1 e 1.0 unidades.
  - Badge flutuante de medição exibindo distância euclidiana precisa e decomposição nos eixos cartesianos ($\Delta X, \Delta Y, \Delta Z$).
  - Cancelamento e limpeza instantânea via tecla `Delete` ou `Esc`.
- **Ferramenta Interativa de Anotação e Rascunho 3D (`crates/ui/src/annotation.rs`, `viewport_interaction.rs`)**:
  - Rascunho à mão livre em espaço tridimensional (`Tool::Annotate`, atalho `D`).
  - Projeção contínua sobre a superfície da malha ativa ou sobre o plano de referência do 3D Cursor.
  - Renderização fluida de strokes com espessura variável e suavização de pontos.
  - Limpeza e remoção de anotações via tecla `Delete` ou `Esc`.
- **Gaveta / Modal Dedicado da Biblioteca de Assets (`crates/ui/src/asset_library_drawer.rs`, `main_header.rs`)**:
  - Nova gaveta/janela flutuante dedicada (`📦 Assets` no header) para visualização e gerenciamento de assets do projeto.
  - Diferenciação conceitual e funcional explícita:
    * *Salvar Modelo Ativo como Asset*: Registra/salva a malha ativa diretamente na biblioteca interna do projeto (`state.project.assets`).
    * *Salvar Projeto*: Persiste o arquivo `.petunia` completo (cena, assets, materiais, paleta e configurações).
  - Cards visuais para cada modelo com contagem de vértices/faces, chips de cor e botões de ação: Instanciar no 3D Cursor (`➕ Instanciar`), Editar (`🎯 Editar`), Duplicar (`📋 Duplicar`) e Remover (`🗑 Remover`).
- **Novo Sistema Dinâmico de Temas Baseado em Tokens (`crates/config/src/theme.rs`, `crates/ui/src/tokens.rs`)**:
  - Suporte completo a temas declarativos em TOML com `manifest.toml` e `theme.toml`.
  - Mapeamento universal de tokens semânticos (`ThemeToken` e `ThemeColors`).
  - 4 temas nativos distribuídos: `petunia-dark`, `petunia-light`, `petunia-capuccino` e `petunia-tokyo-nights`.
  - Varredura dinâmica de diretórios (`assets/themes/`) com fallback tolerante a falhas para a paleta canônica Petunia Dark.
- **Sistema Aberto de Pacotes de Ícones (`crates/ui/src/icon_registry.rs`, `assets/icons/`)**:
  - Suporte modular a múltiplos pacotes de ícones via `manifest.toml` e `icons.toml`.
  - 5 pacotes estruturados: `Petunia`, `Phosphor`, `Tabler`, `Iconoir` e `Lucide`.
  - Cascading fallback seguro: raster/SVG do pacote -> desenho vetorial nativo Petunia -> glifo Unicode.
- **8 Perfis Canônicos de Teclado e Análise de Conflitos (`crates/config/src/keybinds.rs`, `assets/keymaps/`)**:
  - 8 perfis TOML completos: `Petunia Padrão`, `Petunia Simplificado`, `Petunia Notebook`, `Blender`, `Blender Notebook`, `Maya`, `3ds Max` e `Cinema 4D`.
  - Motor de análise e detecção automática de conflitos/colisões de atalhos em tempo de execução com alertas visuais.
- **Internacionalização (i18n) Declarativa via TOML (`crates/config/src/lib.rs`, `assets/locales/`)**:
  - Dicionários completos em TOML para `pt-BR` e `en` cobrindo todas as strings da interface.
  - Carregamento e troca dinâmica sem necessidade de reinicialização.
- **Modal Centralizado de Configurações (`crates/ui/src/settings_modal.rs`, `main_header.rs`)**:
  - Modal com 4 abas ergonômicas: Aparência (seleção de temas e chips de tokens de cor ao vivo), Ícones (seleção de pacotes e preview em grade), Idioma (pt-BR / en-US) e Teclado (troca de perfil, busca de atalhos e monitor de conflitos).

### Modificado
- **Refinamento Óptico dos Ícones da Toolbar (`crates/ui/src/icons.rs`)**:
  - Ajuste suave na espessura do traço vetorial de ~2.4–2.6px para ~1.8–1.9px, garantindo visual refinado e elegante sem perder a legibilidade ou as cores canônicas do Blender.
- **Reorganização Sem Sobreposição da Viewport Bar (`crates/ui/src/viewport_bar.rs`)**:
  - Botão de adição explicitado como `[➕ Add+ ▾]`.
  - Botões de Shading condensados em esferas compactas de 22x22px estilo Blender flutuantes (`○`, `●`, `◐`, `☼`).
  - Botões de seleção compactados (`🧊 Objeto`, `⬝ Vértice`, `╱ Aresta`, `▨ Face`) com atalhos transferidos para tooltips ricos, eliminando colisões horizontais mesmo em viewports estreitos.

## [0.5.0] - 2026-09-12 — UI Consolidation, Outliner Redesign, Dead Controls Cleanup & Blender Vector Icons

### Adicionado
- **Novo Sistema Vetorial de Ícones com Traço Reforçado e Paleta Autêntica do Blender (`crates/ui/src/{icons.rs, icon_registry.rs, app_icons.rs}`)**:
  - Traços reforçados de 1.5px para 2.0px–2.6px com renderização subpixel nítida contra fundos escuros da UI.
  - Paleta multicolorida fiel ao Blender:
    * `3D Cursor`: Anel circular vermelho vibrante com segmentos tracejados brancos e mira vazada.
    * `Transladar (Move)`: Eixos cardeais RGB (+X vermelho, +Y verde, +Z azul) com pontas de seta triangulares sólidas.
    * `Rotacionar (Rotate)`: Anéis elípticos cardeais tridimensionais em RGB com seta de rotação.
    * `Escalar (Scale)`: Hastes tridimensionais RGB terminadas em cubos sólidos preenchidos.
    * `Transformação Combinada (Transform)`: Gizmo unificado com setas de translação, cubos de escala e arco amarelo de rotação.
    * `Seleção em Caixa (Select Box)`: Retângulo de seleção azul/ciano translúcido com ponteiro branco de contorno escuro.
    * `Anotação (Annotate)`: Lápis Grease Pencil com corpo ciano, ponteira de madeira dourada e grafite escuro sobre traçado desenhado.
    * `Régua e Medição (Measure)`: Régua diagonal amarela com graduações pretas e miras de medição azul-claras.
    * `Adicionar Primitivas (Add Primitive)`: Cubo isométrico sombreado nos três tons de laranja clássicos do Blender com badge circular `+`.
    * `Ferramentas de Modelagem`: Traços reforçados de 2.0px–2.4px com destaques em laranja e amarelo vivo para `extrude`, `inset`, `bevel`, `loop_cut`, `knife`, `pushpull`, `slice`, `subdivide` e `draw_profile`.
- **Redesenho Completo e Funcional do Outliner (`crates/ui/src/outliner.rs`)**:
  - Integração direta e reativa com `state.project.assets`.
  - Botão de visibilidade funcional (`👁`) sincronizado com `asset.visible` e respeitado nos backends WebGPU e OpenGL.
  - Menu de contexto (RMB) para Duplicar (`Shift+D`) e Deletar (`Delete`).
  - Galeria rápida de primitivas integradas (`Cubo`, `Esfera`, `Cilindro`, `Plano`, `Cone`, `Cápsula`) instanciadas com precisão na coordenada do 3D Cursor (`state.cursor_3d`).
- **Atalhos e Modelo de Seleção Unificado (`crates/app/src/lib.rs`, `crates/config`)**:
  - Correção da propagação de eventos no `WgpuApp` e `GlApp`: teclas `Tab` e `0..=4` não são mais descartadas pelo egui quando nenhum campo de texto possui foco ativo (`!wants_keyboard_input()`).
  - Simplificação para 4 alvos explícitos de seleção: Objeto (`Tab` / `0`), Vértice (`1`), Aresta (`2`), Face (`3`).
- **Reorganização Semântica da Viewport Bar (`crates/ui/src/viewport_bar.rs`)**:
  - 5 clusters distintos com divisores visuais claros: Alvo de Seleção, Menus Compactos (`👁 View ▾`, `▢ Select ▾`, `+ Add ▾`), Transformação e Snapping, Toggles de Exibição (`Overlays`, `X-Ray`) e Sombreamento em 4 botões esféricos estilo Blender (`○ Wire`, `● Solid`, `◐ Material`, `☼ Render`).

### Removido
- **Limpeza de Controles e Botões Mortos**:
  - Remoção dos botões sem funcionalidade abaixo do Outliner (`Render Engine`, `Output`, `View Layer`, `Scene`, `World`, `Collection`).
  - Remoção dos pills estáticos de cena (`Scene`, `ViewLayer`) e do menu fictício `Render` do cabeçalho superior.
  - Remoção do painel inferior de Timeline, recuperando 100% da altura útil da viewport 3D.

## [0.4.0] - 2026-09-12 — Modernized egui Infrastructure, Specialized Crates & Sovereign Design System

### Adicionado
- **Soberania do Petunia Design System (`crates/ui/src/{tokens.rs, widgets.rs}`)**: Preservação estrita da identidade visual (`Blender.svg`), tokens de cores, métricas e comportamentos interativos sobrepondo qualquer estilo padrão de crates externas. Componentes atômicos: `PetuniaToolbarButton`, `PetuniaPropertyTabButton`, `PetuniaWorkspacePill` e `PetuniaSearchBox`.
- **Registro Centralizado de Ícones (`crates/ui/src/icon_registry.rs`)**:
  - `IconRegistry` e enum `PetuniaIcon` com suporte a 9 ícones de toolbar (PNGs 256x256 RGBA) e 15 ícones de abas de propriedades (PNGs 22x22 RGBA) extraídos diretamente do Figma (`assets/ui/icons/properties/`).
  - Decodificação de PNG embutido com preservação do canal alfa e tingimento dinâmico conforme estado de interação (repouso `#BCBCBC`, hover `#FFFFFF`, ativo `#3169E3`).
  - Fallback automático para ícones vetoriais de grade 24x24 e glifos tipográficos (Phosphor).
- **Integração de Gizmos de Transformação 3D (`transform-gizmo-egui`)**:
  - Módulo `crates/ui/src/transform_gizmo_integration.rs` com conversão bidirecional entre `petunia_core::camera::Camera` e `transform_gizmo::math::Transform`.
  - Mapeamento de modos de manipulação (`Translate`, `Rotate`, `Scale`) e orientação (`Global`, `Local`).
- **Navegação Hierárquica da Cena (`egui_ltreeview`)**:
  - Refatoração do Outliner (`crates/ui/src/outliner.rs`) adotando `egui_ltreeview::TreeView` estilizado com tokens Petunia.
  - Suporte a seleção de nós (`OutlinerNodeId`), expansão persistente, filtragem instantânea de nós via `petunia_search_box` e toggles de visibilidade e renderização.
- **Sistema de Layout e Docking Multi-Painel (`egui_tiles`)**:
  - Módulo `crates/ui/src/tiles_workspace.rs` gerenciando `egui_tiles::Tree<PetuniaPane>` estruturado na árvore canônica de visualização (`Blender.svg`): Toolbar à esquerda, Viewport 3D ao centro, Outliner superior direito, Propriedades inferior direito e Timeline inferior.
  - Implementação de `PetuniaTilesBehavior` customizando barras de abas, fundos, abas ativas, hover e strokes de redimensionamento em estrita conformidade com os tokens Petunia.
- **Serviço de Diálogos de Arquivos Multiplataforma (`egui-file-dialog`)**:
  - Módulo `crates/ui/src/file_dialog_service.rs` desacoplado, suportando Open Project (`.petunia`), Save Project, Save Project As, Import OBJ, Export OBJ e Export GLB.
- **Suíte de Testes de Fluxo UI Headless (`egui_kittest`)**:
  - Configuração de `dev-dependencies` e ativação da feature `accesskit` para interoperabilidade completa com `egui-winit`.
  - Bateria de testes de integração em `crates/ui/tests/kittest_ui_flows.rs` cobrindo fluxos de cabeçalho principal, barra de contexto, toolbar com contextualização de modos, árvore de outliner e layout de tiles.
  - Suíte completa de 64 testes automatizados em `petunia_ui` (59 unitários + 5 de fluxo kittest), 100% de aprovação e zero warnings em `cargo clippy --workspace --all-targets -- -D warnings`.

## [0.3.0] - 2026-09-12 — Canonical Desktop UI Architecture (`Blender.svg` Golden Reference)

### Adicionado
- **Design Tokens Canônicos (`crates/ui/src/tokens.rs`)**: Centralização da paleta Dark Theme profissional do Blender (`#121212`, `#1a1a1a`, `#202020`, `#2d2d2d`, `#3169e3`), raios de curvatura de controles/pílulas e métricas de layout.
- **Gerenciador de Ícones Nativos com Tingimento Dinâmico (`crates/ui/src/app_icons.rs`)**: Embutimento em tempo de compilação dos 9 PNGs transparentes extraídos do Figma (`assets/ui/icons/toolbar/*.png`), tingimento dinâmico com base no estado do botão (repouso `#BCBCBC`, hover `#FFFFFF`, ativo `#3169E3` com ícone branco) e fallback vetorial seamlessly integrado.
- **Cabeçalho Superior Principal (`crates/ui/src/main_header.rs`)**: Menus do sistema (File, Edit, Render, Window, Help), branding Petunia3D e abas de workspaces em pílulas arredondadas.
- **Barra de Contexto do Viewport 3D (`crates/ui/src/viewport_bar.rs`)**: Seletor de modo com badges coloridos (Object/Edit/Paint), botões de seleção de componentes (Vértice [1], Aresta [2], Face [3]), orientação de transformação, ponto de pivô, snapping magnético, edição proporcional e os 4 modos canônicos de sombreamento (Wireframe, Solid, Material, Render).
- **Barra Lateral de Ferramentas (`crates/ui/src/toolbar.rs`)**: Toolbar redimensionável com suporte a largura dinâmica (modo compacto de 48px ou expandido até 240px com ícones e rótulos de ferramentas); contextualização onde ferramentas de modelagem de malha (`extrude`, `inset`, `bevel`, `loop_cut`, etc.) aparecem exclusivamente no modo de edição (`EditMode::Edit`), com normalização defensiva para `select` no modo de objeto (`EditMode::Object`).
- **Cabeçalhos Redimensionáveis (`crates/ui/src/{main_header.rs, lib.rs, viewport_bar.rs}`)**: Cabeçalho principal do sistema e barra de contexto do viewport agora possuem altura redimensionável com limites definidos em `tokens.rs` e alinhamento vertical centralizado de todos os itens.
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
