# Crate `petunia_mesh` (`crates/mesh/`)

Domínio geométrico central do Petunia3D. Responsável pela representação de malhas poligonais, estruturas de dados de vértices/arestas/faces, geração de primitivas geométricas (Cubo, Esfera, Cilindro, etc.), operações topológicas e parsers de formato Wavefront OBJ.

Totalmente desacoplado de bibliotecas gráficas ou interfaces com o usuário.

## Módulos e Recursos

- `half_edge`: Estrutura de dados Half-Edge (`HalfEdgeMesh`), queries topológicas (vertex star, face neighbors, boundary loops, manifold checks) e relatório de defeitos (`TopologyReport`, `TopologyDefect`).
- `bevel`: Chanfro transacional de uma aresta convexa manifold com extremidades trivalentes. Desloca os extremos nas arestas vizinhas e religa as faces incidentes; rejeita seleções múltiplas e topologias não suportadas sem mutação. Ainda não implementa segmentos arredondados.
- `ops`: Operações de modelagem e transformação:
  - Extrusão (`extrude_selected`), Inset (`inset_selected`), Bevel (`bevel_selected`), Subdivisão (`subdivide_selected`), Push/Pull (`push_pull`).
  - Novas operações: Inversão de normais (`flip_normals`), recálculo automático unificado de normais via BFS e volume sinalizado (`recalculate_normals`), dissolução limpa de arestas e vértices (`dissolve_selected`), fatiamento planar com fechamento de tampas (`slice_plane`), varredura ao longo de polilinha via RMF (`sweep`), combinação de malhas (`join`), ponte entre loops de faces (`connect_loops`) e fusão por proximidade (`weld`).
  - Seleção avançada: Inverter seleção (`invert_selection`), selecionar conectados/ilhas (`select_linked`), seleção por caixa (`box_select`).
- `primitives`: Geração de sólidos e malhas padrão (Cubo, Esfera, Cilindro, Cone, Toro, Cápsula, Plano).
- `triangulate`: Triangulação de polígonos côncavos via ear-clipping, cálculo de normais e testes de interseção de raios (Möller-Trumbore).
- `obj`: Importação e exportação de Wavefront OBJ com sanitização rigorosa de índices fora de limites e valores numéricos inválidos (NaN/Inf).
- `uv`: Projeção planar e manipulação UV mantendo o invariante estrito `uv.len() == verts.len()`.

## Contratos de operações interativas

`extrude_selected` substitui as faces da região selecionada por uma tampa com vértices compartilhados e cria paredes apenas no contorno. Remove vértices órfãos e mantém selecionados os vértices da tampa. O preview modal pode preparar a topologia com distância zero e deslocar a seleção; índices antigos podem ser remapeados pela limpeza de órfãos. Valores não finitos não alteram a malha.

`inset_selected` usa uma fração em direção ao centróide, limitada a 0,95. Zero, valores negativos e não finitos não alteram a malha. Não há garantia de ausência de auto-interseção em polígonos côncavos.

`slice_plane` com `fill_cap = true` mantém o semiespaço positivo da normal e fecha o contorno do corte com a normal da tampa apontando para o lado removido. Com `false`, preserva ambos os lados e subdivide a superfície, sem adicionar faces internas. Testes cobrem cubos, cortes por vértices existentes e tangências; tampas de regiões com buracos e interseções côncavas complexas ainda exigem um algoritmo específico.
