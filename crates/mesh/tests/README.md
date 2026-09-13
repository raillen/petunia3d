# Testes de Integração de `petunia_mesh`

Este diretório contém testes de conformidade e integração para operações topológicas e geométricas de `petunia_mesh`.

## Arquivos e Cobertura

- **`triangulation_tests.rs`**:
  - `test_triangulation_wireframe_cube_has_six_diagonals`: Validação das diagonais internas de triangulação fan em polígonos.
  - `test_flip_diagonal_quad`: Inversão de diagonal interna em faces quadrangulares via rotação cíclica do loop de vértices.
  - `test_flip_diagonal_shared_triangles_edge`: Delaunay Edge Flip entre dois triângulos adjacentes compartilhando uma aresta.

- **`interactive_geometry_tests.rs`**:
  - `test_extrude_individual_faces_decouples_shared_edges`: Extrusão de faces individuais desacoplando vértices e gerando paredes laterais independentes (Alt+E).
  - `test_revolve_selection_full_360_circle`: Revolução de perfis abertos/conectados em 360° em torno de eixos coordenados.
  - `test_bevel_multi_segments`: Chanfro com multi-segmentos e curvatura em arco circular de filete, mantendo manifoldness e fechamento.
  - `test_guarded_inset_extreme_factor_stability`: Inset métrico protegido contra inversão de normais e auto-interseções sob fatores extremos.
