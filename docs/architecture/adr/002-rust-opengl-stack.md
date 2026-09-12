# ADR 002: Escolha de Rust, OpenGL 3.3 Core via Glow e WGPU Fallback

## Status
Aceito

## Contexto
O Petunia3D visa atender artistas e desenvolvedores em computadores modernos e equipamentos legados (como laptops modestos com GPUs integradas Intel Ivy Bridge HD Graphics 4000). A escolha da stack técnica precisa conciliar segurança de memória em manipulações de geometria, desempenho nativo, portabilidade entre plataformas e compatibilidade com drivers antigos sem suporte a Vulkan.

## Decisão
1. **Linguagem**: Rust (substituindo proposta exploratória preliminar em Odin — ver `docs/petunia3d-livro-vivo/32-adr-odin-para-rust.md`).
2. **Backend Primário (Compatibilidade Universal)**: OpenGL 3.3 Core Profile via crate `glow` com shaders GLSL 330.
3. **Backend Moderno**: `wgpu` como alternativa com detecção automática e fallback seguro para OpenGL clássico em caso de falha de inicialização.
4. **Interface Imediata**: `egui` pelo baixo overhead de memória, facilidade de layout e ergonomia para ferramentas interativas de criação de conteúdo.

## Consequências
- **Positivas**: Memória previsível, zero vazamento em malhas, interoperabilidade garantida em Linux (Mesa i965/crocus), Windows e macOS; render-on-demand consumindo 0 frames quando ocioso.
- **Negativas**: Dois caminhos de renderização (`render-gl` e `render-wgpu`) requerem isolamento cuidadoso de tipos em `render`.
