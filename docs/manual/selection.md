# Modos de Seleção

O Petunia3D oferece controle rigoroso sobre os modos de seleção, garantindo que você trabalhe com precisão milimétrica em nível de objeto inteiro ou em subcomponentes geométricos.

---

## 1. Modo Objeto vs. Modo de Edição

- **Modo Objeto (`Tab` ou tecla `0`)**: Permite selecionar, mover, rotacionar e escalar malhas inteiras na cena como unidades atômicas.
  - **Seleção Direta no Viewport**: Clicar com o botão esquerdo (`LMB`) sobre qualquer objeto visível e desbloqueado na cena calcula o raio tridimensional contra a câmera e ativa o objeto mais próximo imediatamente.
  - **Alternância com Shift**: Segure `Shift` enquanto clica para alternar a seleção de objetos adicionais sem desselecionar os atuais.
  - **Sincronização com o Outliner**: A seleção na viewport é bidirecional e sincronizada em tempo real com a árvore do Outliner.
- **Modo de Edição (`Tab` com malha selecionada)**: Entra na malha ativa para editar sua topologia de vértices, arestas e faces.

---

## 2. Alvos de Seleção de Malha (Em Modo de Edição)

Ao entrar no Modo de Edição, três seletores dedicados surgem no Cluster 1 da Viewport Bar:

| Alvo de Seleção | Ícone | Atalho | Descrição |
| :--- | :--- | :--- | :--- |
| **Vértice** | `⬝ Vértice` | `1` | Seleciona pontos tridimensionais individuais. |
| **Aresta** | `╱ Aresta` | `2` | Seleciona os segmentos de linha que conectam dois vértices. |
| **Face** | `▨ Face` | `3` | Seleciona os polígonos planos (triângulos ou quads) formados por arestas. |

### Demarcação Visual de Vértices
Quando o modo de seleção de vértices (`1`) está ativo:
- Todos os vértices são demarcados com pequenos pontos;
- **Hover Dinâmico**: Ao passar o cursor do mouse próximo a um vértice, ele acende com um anel dourado e ciano brilhante (`#64dcff`), garantindo que você tenha certeza visual de qual ponto será capturado antes de clicar.

---

## 3. Comandos de Seleção

- **Selecionar Tudo**: Tecla `A`.
- **Desmarcar Tudo**: `Alt+A` ou clique no espaço vazio do viewport.
- **Seleção Múltipla**: Segure `Shift` enquanto clica com `LMB` nos elementos desejados.
- **Seleção por Caixa (`Box Select`)**: Tecla `B` ou clique e arraste com `LMB` na área vazia da tela.
- **Inverter Seleção**: `Ctrl+I`.
