# Crate `petunia_render` (`crates/render/`)

Camada intermediária agnóstica de computação gráfica:
- Definição de estruturas matemáticas compartilhadas (matrizes MVP, vetores de luz direcional, projeções).
- Geração da malha do grid infinito de piso de referência e dos quads de referências ortográficas.
- Contratos de dados de cena consumidos por ambos os backends gráficos (`render-gl` e `render-wgpu`).
