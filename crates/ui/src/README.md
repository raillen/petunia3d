# Código Fonte — Petunia UI (`crates/ui/src`)

Módulos Rust e implementações dos componentes de interface gráfica desktop do Petunia3D baseados em `egui`.

## Arquitetura de Componentes da Interface (Fidelidade Canônica `Blender.svg`)

- `tokens.rs`: Sistema centralizado de design tokens (paleta de cores do Blender Dark Theme, raios, margens e métricas).
- `app_icons.rs`: Gerenciador de texturas rasterizadas embutidas (`assets/ui/icons/toolbar/*.png`) com tingimento dinâmico e fallback vetorial.
- `main_header.rs`: Barra superior de menus do sistema (File, Edit, Render, Window, Help), branding Petunia3D e abas de workspaces em pílulas arredondadas.
- `viewport_bar.rs`: Barra de contexto do Viewport 3D (seletor de modo Object/Edit com badges coloridos, seleção 1/2/3, orientação de transformação, snapping magnético, edição proporcional e os 4 modos de shading).
- `toolbar.rs`: Barra vertical de 40px com os ícones reais extraídos do Figma (`select_box`, `cursor_3d`, `move`, `rotate`, `scale`, `transform`, `annotate`, `measure`, `add_primitive`) e ferramentas poligonais.
- `outliner.rs`: Painel superior direito com árvore de coleções (Scene Collection -> Collection -> Câmera, Malhas ativas, Luz) e toggles de visibilidade/render.
- `properties_panel.rs`: Painel de propriedades com abas horizontais icônicas (`Tool`, `Render`, `Output`, `Scene`, `World`, `Object`, `Modifiers`, `Data`, `Material`) e formulários sanfonados.
- `timeline.rs`: Painel inferior de reprodução de animação, botões de transporte, contador de frames e régua de scrubbing temporal.
- `status_bar.rs`: Barra inferior de status com atalhos e dicas de mouse à esquerda, feedback operacional ao centro e telemetria de malha/performance à direita.
- `nav_gizmo.rs`: Gizmo de orientação tridimensional interativo no viewport com 6 eixos ordenados por profundidade e botões rápidos de navegação.
- `viewport_interaction.rs`: Interações com o viewport (menu contextual RMB, posicionamento de 3D Cursor, seleção por caixa, órbita e pan).
- `gizmo.rs`: Gizmo de transformação 3D no viewport (translação, rotação e escala).
- `camera_controls.rs`: Controles de câmera e transições ortogonais/perspectiva.
- `tool_fields.rs`: Campos numéricos de transformação interativa de ferramentas modais.
- `cutting.rs`: Lógica de renderização de linhas de corte no viewport para Loop Cut, Knife e Bisect.
