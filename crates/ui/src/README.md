# Código Fonte — Petunia UI (`crates/ui/src`)

Módulos Rust e implementações dos componentes de interface gráfica desktop do Petunia3D baseados em `egui`.

## Arquitetura de Componentes da Interface (Fidelidade Canônica `Blender.svg`)

- `tokens.rs`: Sistema centralizado de design tokens (paleta de cores do Blender Dark Theme, raios, margens e métricas).
- `icon_registry.rs`: Registro centralizado de ícones (`IconRegistry` e enum `PetuniaIcon`), gerenciando carregamento de PNGs embutidos (toolbar e properties), tingimento dinâmico e fallback vetorial.
- `widgets.rs`: Biblioteca de widgets canônicos sovereign (`PetuniaToolbarButton`, `PetuniaPropertyTabButton`, `PetuniaWorkspacePill`, `PetuniaSearchBox`) padronizados segundo o Design System.
- `app_icons.rs`: Gerenciador de texturas rasterizadas embutidas legadas (`assets/ui/icons/toolbar/*.png`) com tingimento dinâmico e fallback vetorial.
- `main_header.rs`: Barra superior redimensionável de menus do sistema (File, Edit, Render, Window, Help), branding Petunia3D e abas de workspaces em pílulas arredondadas.
- `viewport_bar.rs`: Barra de contexto redimensionável do Viewport 3D (seletor de modo Object/Edit com badges coloridos, seleção 1/2/3, orientação de transformação, snapping magnético, edição proporcional e os 4 modos de shading).
- `toolbar.rs`: Barra vertical redimensionável com largura dinâmica (modo compacto com ícones de 40px ou expandido com ícones + nomes), consumindo os ícones reais extraídos do Figma; ferramentas de modelagem de malha (`extrude`, `inset`, `bevel`, `loop_cut`, etc.) são contextualizadas e exibidas exclusivamente no modo de edição (`EditMode::Edit`).
- `outliner.rs`: Painel superior direito com árvore hierárquica de cena baseada em `egui_ltreeview::TreeView`, com filtragem por busca instantânea e toggles de visibilidade/render.
- `properties_panel.rs`: Painel de propriedades modular com abas verticais/horizontais icônicas (`Tool`, `Render`, `Output`, `Scene`, `World`, `Object`, `Modifiers`, `Data`, `Material`) baseadas em `PetuniaPropertyTabButton` e formulários sanfonados.
- `transform_gizmo_integration.rs`: Integração de gizmos de transformação 3D com `transform-gizmo-egui`, convertendo câmera, matrizes e modos de transformação com renderização direta no viewport.
- `tiles_workspace.rs`: Infraestrutura de layout e docking de múltiplos painéis com `egui_tiles::Tree<PetuniaPane>` e estilização estrita sob o `PetuniaTilesBehavior`.
- `file_dialog_service.rs`: Serviço de diálogos de arquivos com `egui-file-dialog` para carregamento/salvamento de projetos e importação/exportação de OBJ e GLB.
- `timeline.rs`: Painel inferior de reprodução de animação, botões de transporte, contador de frames e régua de scrubbing temporal.
- `status_bar.rs`: Barra inferior de status com atalhos e dicas de mouse à esquerda, feedback operacional ao centro e telemetria de malha/performance à direita.
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
