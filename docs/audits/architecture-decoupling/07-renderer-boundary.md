# 07 — Fronteira de Renderização (Renderer Boundary Report)

> **Auditoria dos backends gráficos WebGPU e OpenGL, avaliação da capacidade de renderização offscreen e desacoplamento do pipeline 3D em relação à UI.**

---

## 1. Avaliação dos Crates de Renderização

O Petunia3D possui três crates dedicados à renderização:
1. `petunia_render`: Tipos neutros compartilhados (`Shading`, `FrameStats`, `GpuCaps`).
2. `petunia_render_wgpu`: Pipeline moderno baseado em WebGPU (`wgpu` v25).
3. `petunia_render_gl`: Pipeline de compatibilidade baseado em OpenGL (`glow` / `glutin`).

### Diagnóstico de Acoplamento:
* **`petunia_render_wgpu` é um dos pontos arquiteturais mais fortes do projeto:**
  * **Zero dependências de egui:** O manifesto `crates/render-wgpu/Cargo.toml` não referencia `egui`, `eframe` ou `epaint`.
  * As assinaturas públicas de `Renderer` operam exclusivamente com tipos padrão do ecossistema wgpu e glam:
    * `Renderer::new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self`
    * `Renderer::resize(&mut self, device: &wgpu::Device, width: u32, height: u32)`
    * `Renderer::update(&mut self, device, queue, project, camera, refs, shading, xray, select_mode)`
    * `Renderer::render(&self, pass: &mut wgpu::RenderPass<'_>, refs: &[ReferenceImage])`
* **`petunia_render_gl`:**
  * Utiliza `egui_glow` unicamente para obter acesso compartilhado ao contexto GL carregado pelo winit, mas seu código de desenho é puramente chamadas `gl.*`.

---

## 2. Resposta à Pergunta Fundamental da Auditoria

> **"É possível renderizar uma cena do Petunia offscreen sem abrir egui?"**

### Diagnóstico: **SIM, PLENAMENTE POSSÍVEL**.

Como o `petunia_render_wgpu::Renderer` desenha recebendo um `wgpu::RenderPass`, é trivial instanciá-lo em uma rotina de teste, passar um `wgpu::Texture` de destino offscreen, ler os pixels da GPU e salvá-los como PNG, sem carregar nenhuma janela winit ou contexto egui.

---

## 3. Onde Reside o Acoplamento do Renderer Hoje

O acoplamento existente não está no código do renderer 3D em si, mas sim na sua **orquestração dentro do `petunia_app`**:

1. **Ausência de Trait Comum (`SceneRenderer`)**:
   * Não existe uma interface comum que unifique `petunia_render_wgpu` e `petunia_render_gl`. O `petunia_app` gerencia instâncias separadas dentro de um enum `GfxBackend`.
2. **Entrelaçamento no Loop de Janela (`crates/app/src/lib.rs`)**:
   * O loop de redraw winit mistura no mesmo método:
     * Obtenção da superfície da janela winit (`surface.get_current_texture()`);
     * Execução do passe 3D via `Renderer::render`;
     * Execução do passe de UI via `egui-wgpu::Renderer::render`.
3. **Contrato de Retângulo do Viewport (`crates/core/src/viewport.rs`)**:
   * O cálculo das dimensões físicas do viewport para o renderer (`PhysicalViewport::from_logical`) recebe um `Option<egui::Rect>`.
   * Essa dependência de um tipo de dados do egui contamina desnecessariamente o `petunia_core`.

---

## 4. Opções de Integração de Viewport para Frontends Futuros

Ao substituir o egui por outro frontend (ex: Qt, Slint, C#, Web), o viewport 3D pode ser integrado através de três padrões consagrados:

| Padrão de Integração | Como Funciona | Latência / Desempenho | Complexidade |
| :--- | :--- | :---: | :---: |
| **Opção A: Native Window Handle (`RawWindowHandle`)** | O frontend cria uma sub-janela/canvas nativo do SO (`HWND`, `NSView`, `X11 Window`) e passa o handle ao core. O `petunia_render_wgpu` cria uma superfície wgpu diretamente nessa área e renderiza nativamente a 60+ FPS. | **Zero overhead** (Nativo) | Baixa a Média |
| **Opção B: Textura Compartilhada (Shared Texture / Offscreen)** | O renderer wgpu renderiza a cena em uma textura offscreen da GPU e compartilha a memória gráfica com o toolkit da UI (via DirectX shared handle, Vulkan external memory, ou leitura FBO). | **Extremamente alta** (Zero cópias de CPU) | Média a Alta |
| **Opção C: Streaming de Pixels (Frame Streaming)** | O core renderiza offscreen, lê os pixels para a RAM e os transmite por memória compartilhada ou IPC para o processo da UI. | **Baixa** (Sobrecarga de cópia de CPU / compressão) | Alta |

> [!TIP]
> Para aplicações desktop locais de criação 3D em tempo real, a **Opção A (Native Window Handle)** é a recomendação padrão de engenharia para frontends em Rust, C++, C# e Go, evitando qualquer sobrecarga de cópia de pixels.
