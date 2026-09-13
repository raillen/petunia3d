# Código Fonte — Petunia UI (`crates/ui/src`)

Módulos Rust e implementações dos componentes de interface gráfica desktop do Petunia3D baseados em `egui`.

## Arquitetura de Componentes da Interface (Fidelidade Canônica `Blender.svg`)

- `tokens.rs`: Sistema centralizado de design tokens (paleta de cores do Blender Dark Theme, raios, margens e métricas).
- `icon_registry.rs`: Registro centralizado de ícones (`IconRegistry` e enum `PetuniaIcon`), gerenciando carregamento de PNGs embutidos (toolbar e properties), tingimento dinâmico e fallback vetorial.
- `widgets.rs`: Biblioteca de widgets canônicos sovereign (`PetuniaToolbarButton`, `PetuniaPropertyTabButton`, `PetuniaWorkspacePill`, `PetuniaSearchBox`) padronizados segundo o Design System.
- `app_icons.rs`: Gerenciador de texturas rasterizadas embutidas legadas (`assets/ui/icons/toolbar/*.png`) com tingimento dinâmico e fallback vetorial.
- `main_header.rs`: Barra superior redimensionável de menus do sistema (File com exportação OBJ/GLB, Edit, Render, Window, Help), branding Petunia3D, seletor de workspaces em pílulas (`[ MODEL ] [ PAINT ] [ UV ] [ ANIMATE ]`), botão `[📦 Assets]` e `[⚙ Config]`.
- `viewport_bar.rs`: Barra de contexto horizontal do Viewport 3D estruturada em 6 clusters limpos: Dropdown de Modo (`[ Object Mode ▾ ]` vs `[ Edit Mode ▾ ]`), alvos de seleção (`⬝ Vértice`, `╱ Aresta`, `▨ Face`) exibidos exclusivamente em Edit Mode, menus `👁 View ▾`, `▢ Select ▾`, `➕ Add+ ▾` e menu contextual de Objeto/Malha, orientação e pivô, snapping magnético e edição proporcional, toggles de overlays/x-ray, e 4 esferas de sombreamento canônicas (Wireframe, Solid, Material, Rendered).
- `contextual_shelf.rs`: Barra contextual horizontal flutuante na base inferior do Viewport 3D (`Contextual Modeling Shelf`). Reage dinamicamente ao workspace e ao modo ativo, oferecendo comandos de modelagem rápida (Extrude, Inset, Bevel, Loop Cut, Knife, Subdivide, Merge) em Model/Edit, primitivas e transformações em Model/Object, pincéis em Paint, desdobramento em UV e timeline transport player em Animate, com contenção de eventos para evitar interferência na cena 3D.
- `asset_browser.rs`: Painel lateral retrátil à esquerda (220–340px) com busca instantânea via `petunia_search_box`, categorias (Todos, Props, Personagens, Cenário), cartões visuais com contagem de vértices/triângulos, chips de cor, ações rápidas (instanciar no cursor 3D, ativar, duplicar, deletar) e botão de salvar modelo ativo como asset permanente.
- `toolbar.rs`: Barra vertical retrátil à esquerda estritamente restrita às 8 ferramentas primárias e persistentes de manipulação: `Select Box`, `3D Cursor`, `Move`, `Rotate`, `Scale`, `Transform`, `Measure` e `Annotate`.
- `outliner.rs`: Painel superior direito com árvore hierárquica de cena estritamente dedicada a objetos, coleções, busca e toggles de visibilidade, sem poluição de primitivas ou galerias duplicadas.
- `properties_panel.rs`: Painel de propriedades modular com abas icônicas (`Tool`, `Render`, `Output`, `Scene`, `World`, `Object`, `Modifiers`, `Data`, `Material`, `Animate`). Contém inspector de transform com grid tri-axial e cores RGB, seletor de material com paleta interativa de swatches do projeto e controles de animação.
- `transform_gizmo_integration.rs`: Integração de gizmos de transformação 3D com `transform-gizmo-egui`, convertendo câmera, matrizes e modos de transformação com renderização direta no viewport.
- `tiles_workspace.rs`: Infraestrutura de layout e docking de múltiplos painéis com `egui_tiles::Tree<PetuniaPane>` e estilização estrita sob o `PetuniaTilesBehavior`.
- `file_dialog_service.rs`: Serviço de diálogos de arquivos com `egui-file-dialog` para carregamento/salvamento de projetos e importação/exportação de OBJ e GLB.
- `timeline.rs`: Painel inferior de reprodução de animação, botões de transporte, contador de frames e régua de scrubbing temporal, ativo no workspace `Animate`.
- `status_bar.rs`: Barra inferior de status dividida em 3 blocos: Bloco 1 (identidade do projeto com badge `● Salvo` / `○ Não salvo`, nome do arquivo e atalhos contextuais do mouse/ferramenta ativa), Bloco 2 (mensagens e feedbacks centrais) e Bloco 3 (telemetria agregada com triângulos, vértices, objetos, frame time e botões de histórico Undo/Redo).
- `nav_gizmo.rs`: Gizmo de orientação tridimensional interativo no viewport com 6 eixos ordenados por profundidade e botões rápidos de navegação.
- `viewport_interaction.rs`: Interações com o viewport (menu contextual RMB, posicionamento de 3D Cursor, seleção por caixa, órbita e pan).
- `gizmo.rs`: Gizmo de transformação 3D no viewport (translação, rotação e escala).
- `camera_controls.rs`: Controles de câmera e transições ortogonais/perspectiva.
- `tool_fields.rs`: Campos numéricos de transformação interativa de ferramentas modais.
- `cutting.rs`: Lógica de renderização de linhas de corte no viewport para Loop Cut, Knife e Bisect.
- `measurement.rs`: Ferramenta interativa de régua e medição 3D (`measure`) no viewport com snapping magnético a vértices, marcas métricas de graduação e badge com distância euclidiana e deltas cartesianos ($\Delta X, \Delta Y, \Delta Z$).
- `annotation.rs`: Ferramenta de anotação e rascunho 3D (`annotate`) no viewport com projeção em superfícies e plano do 3D cursor para grease-pencil e anotações técnicas.
- `asset_library_drawer.rs`: Gaveta/modal dedicado para a biblioteca de assets do projeto com pré-visualizações, métricas de polígonos, instanciação no 3D Cursor e separação conceitual entre salvar asset e salvar projeto.
- `settings_modal.rs`: Modal centralizado de configurações (`⚙ Config`) com abas para Aparência (temas e tokens ao vivo), Ícones (pacotes e pré-visualização), Idioma (i18n TOML) e Teclado (8 perfis canônicos e detecção de conflitos).
