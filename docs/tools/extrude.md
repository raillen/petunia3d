# Ferramenta: Extrusão (Extrude)

A **Extrusão** é a principal operação para criar novos volumes tridimensionais a partir de faces ou arestas existentes.

- **Atalhos de Ativação**:
  - `E`: Extrusão de Região (Extrude Region)
  - `Alt+E`: Extrusão de Faces Individuais (Extrude Individual Faces)
- **Disponível em**: componentes da malha (`Face` / `Edge` / `Point`); detalhe técnico: `EditMode::Edit`.

## Modos de Extrusão

### 1. Extrusão de Região (`E`)
Na extrusão de região, faces adjacentes selecionadas compartilham as novas paredes laterais e formam uma tampa contínua comum:
1. Selecione uma ou mais faces (ou arestas);
2. Pressione `E`;
3. A nova camada de polígonos segue a **normal média da seleção**;
4. Digite uma distância no teclado (ex: `0.5m`) ou arraste com o mouse;
5. Pressione `X`, `Y` ou `Z` para forçar a extrusão em um eixo global se desejado;
6. Clique com `LMB` para confirmar ou `Escape` para cancelar e restaurar a geometria original.

### 2. Extrusão de Faces Individuais (`Alt+E`)
Na extrusão individual, cada face selecionada é projetada isoladamente ao longo de sua própria normal local, gerando anéis de parede independentes e topos disjuntos:
1. Selecione múltiplas faces (mesmo que sejam adjacentes);
2. Pressione `Alt+E` ou acione **Extrude Individual** no menu contextual de malha;
3. Cada face gera seus próprios vértices de topo exclusivos, prevenindo o colapso e a fusão de paredes entre polígonos vizinhos;
4. Ideal para modelagem de tijolos, escamas, painéis técnicos e padrões geométricos complexos;
5. Totalmente integrado ao sistema de **Undo/Redo** (`Ctrl+Z` / `Ctrl+Shift+Z`).

