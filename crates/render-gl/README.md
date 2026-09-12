# Crate `petunia_render_gl` (`crates/render-gl/`)

Backend nativo e prioritário em OpenGL clássico via crate `glow`:
- Compatibilidade estrita com OpenGL 3.3 Core Profile e shaders GLSL 330.
- Bootstrap do contexto de janela gráfica via `glutin` isolado neste crate.
- Otimizações de renderização: upload de texturas condicionado por hash FNV-1a e controle de VAO/VBOs.
- Suporte robusto garantido para GPUs antigas e drivers Mesa legados (ex: Intel Ivy Bridge HD 4000).
