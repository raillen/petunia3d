# Pintura & Cores

O workspace **PAINT** do Petunia3D foi desenhado para pintura rápida de texturas estilizadas e atribuição direta de paletas de cores a vértices e faces.

---

## 1. Modos de Coloração

1. **Pintura por Vértice (Vertex Color)**: As cores são interpoladas diretamente pela malha entre os vértices, sem necessidade de mapas de textura adicionais — perfeito para estilos low-poly minimalistas e otimização máxima de draw-calls.
2. **Pintura de Textura**: Pintura sobre mapas de pixel mapeados nas coordenadas UV da malha.

---

## 2. Controles do Pincel

- **Tamanho do Raio do Pincel**: Pressione a tecla `F` e arraste o mouse para ajustar interativamente o raio do traço.
- **Seletor de Cor**: Disponível no painel lateral esquerdo em formato de roda de cores HSV e amostras de paleta salva.
- **Ferramentas**: Pixel, Soft, Borracha, Preencher, Conta-gotas, Linha e Retângulo (formas confirmam ao soltar o botão).
- **Atalhos Rápidos**:
  - `P`: Ativar ferramenta Pincel;
  - `E`: Alternar para Borracha;
  - `Ctrl + LMB`: Selecionar cor sob o cursor (conta-gotas / eyedropper).

## 3. Camadas (Layers)

Cada asset tem uma pilha de camadas Raster: crie, renomeie, reordene (↑/↓), ajuste visibilidade e opacidade e escolha a ativa. Pinceladas vão sempre para a camada ativa; o composto aparece na textura e no viewport. Pintura atua no canal **Albedo**; outros canais PBR chegam na V1.x.

## 4. Tamanho da Textura

Resoluções de 64 a 1024 (padrão 256). Trocar redimensiona base e camadas preservando o conteúdo — é uma operação com undo próprio.
