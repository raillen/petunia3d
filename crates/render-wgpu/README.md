# Crate `petunia_render_wgpu` (`crates/render-wgpu/`)

Backend gráfico moderno acelerado baseado na API `wgpu` (Vulkan / Metal / DirectX 12):
- Utilizado quando há GPU moderna e compatível com as APIs modernas de baixo overhead.
- Shaders compilados com WGSL.
- Suporta mecanismo de fallback transparente para o backend `render-gl` caso o adaptador não consiga inicializar o pipeline de renderização.
