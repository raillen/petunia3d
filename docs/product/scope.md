# Escopo do Projeto — Petunia3D

Delimitação explícita de capacidades suportadas, fronteiras de desenvolvimento, restrições e não-objetivos.

## 1. In-Scope Capabilities (Capacidades em Escopo)

- **Modelagem Shape-First e Operações de Malha**:
  - Geração de perfis 2D (Draw Profile) sobre referências ortográficas com extrusão linear ou revolução polar 360°.
  - 8 ferramentas essenciais de edição de topologia: Extrude, Push/Pull, Inset, Bevel (manifold), Subdivide, Mirror (com merge no eixo central), Merge de vértices próximos, e transformações (Move/Rotate/Scale).
  - Modos de seleção ortogonais: Vértice (1), Aresta (2) e Face (3).
- **Vertex Painting & Texturização UV**:
  - Vertex paint com seleção de cor por paleta, pincel contínuo, balde de preenchimento e conta-gotas (Eyedropper com atalho Alt).
  - Editor UV integrado bidirecional com projeção planar e manipulação de ilhas de UV sobre mapas de pixels.
- **Renderização e Backends Gráficos**:
  - Renderizador OpenGL puro via `glow` (OpenGL 3.3 Core Profile / GLSL 330) com suporte comprovado a GPUs antigas (Intel HD Graphics 3000/4000).
  - Backend secundário baseado em `wgpu` com detecção automática e fallback resiliente para GL puro caso Vulkan/EGL falhe.
  - Arquitetura render-on-demand: redesenho condicionado a eventos de entrada e mutação de malha (`0 frames` em repouso).
- **Importação, Exportação e Persistência**:
  - Formato proprietário de projeto `.petunia`: binário versionado, transacional e com validação contra bounds out-of-bounds e corrupção.
  - Exportação e importação de modelos em formato Wavefront OBJ e glTF 2.0 binário (`.glb`).
- **Arquitetura Modular**:
  - 13 crates acíclicos em Rust com desacoplamento estrito através do `ModuleRegistry` e contratos do crate `core`.

## 2. Non-Goals (O que NÃO é Objetivo do Petunia3D)

- **Escultura Digital com Milhões de Polígonos**: Petunia3D é um modelador estritamente **low-poly** (até ~50.000 triângulos por cena); não substituirá o ZBrush nem ferramentas de dyntopo/multires.
- **Renderização Fotorrealista PBR e Ray Tracing**: A proposta visual foca em estética retrô (flat shading, Gouraud shading, unlit, pixel art textures, PSX look). Shaders complexos de materiais dielétricos, subsurface scattering e path tracing estão fora de escopo.
- **Rigging Complexo e Animação Baseada em Curvas/Timeline**: Animações de esqueleto e curvas de animação não fazem parte do V1. O escopo foca na geometria, UV e cores do asset.
- **Motor de Física Integrado**: Sem simulação de corpos rígidos ou tecidos no editor.
- **Nós Shaders e Grafos Procedurais**: Sem editores de nós no estilo Blender Shader Nodes; texturização direta em albedo/vertex colors.

## 3. Compatibility Constraints (Restrições de Compatibilidade)

- **Requisitos de GPU e Drivers**:
  - Mínimo de OpenGL 3.3 Core Profile no Linux, Windows ou macOS.
  - Suporte total a drivers Mesa legados no Linux (onde implementações de EGL para wgpu frequentemente sofrem falhas com drivers i965/crocus).
- **Recursos de CPU e Memória**:
  - Executável estático de início rápido, utilizável confortavelmente em laptops modestos (4 GB RAM, dual-core).
- **Integridade de Formato**:
  - Compatibilidade garantida entre versões menores do formato `.petunia` através de cabeçalhos com magic bytes e versões semânticas registradas.
