# Camada de Renderização Híbrida

O Petunia3D foi desenhado com um pipeline de renderização híbrido e resiliente que suporta duas implementações de hardware:

---

## 1. Backend WebGPU (`petunia_render_wgpu`)
O backend prioritário utiliza a especificação moderna **WebGPU** através da biblioteca `wgpu`:
- Shaders em formato padronizado **WGSL** com validação rigorosa via Naga;
- Suporte a iluminação difusa, sombreamento sólido, renderização de arame (wireframe) e modo Raio-X (`fs_xray`) com mistura alfa e oclusão de profundidade desabilitada;
- Renderização limpa de grid espacial e linhas infinitas de eixos cartesianos;
- Compatível nativamente com Vulkan (Linux/Android), Metal (macOS/iOS) e DirectX 12 (Windows).

---

## 2. Backend OpenGL (`petunia_render_gl`)
Para máquinas legadas, máquinas virtuais ou sistemas sem suporte a Vulkan:
- Implementado sobre o crate `glow`;
- Utiliza subconjunto estrito OpenGL ES 3.0 / OpenGL 3.3 Core;
- Mantém rigorosa equivalência visual pixel a pixel com a saída do pipeline WebGPU.

O Petunia3D tenta inicializar WebGPU primeiro. Se ocorrer erro de inicialização de adapter, o runtime realiza transição suave e transparente para o pipeline OpenGL sem falhar o início do aplicativo.
