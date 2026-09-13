# Ferramenta: Extrusão (Extrude)

A **Extrusão** é a principal operação para criar novos volumes tridimensionais a partir de faces ou arestas existentes.

- **Atalho de Ativação**: `E`
- **Modos Suportados**: Modo de Edição (`EditMode::Edit`).

## Como Funciona
1. Selecione uma ou mais faces (ou arestas);
2. Pressione `E`;
3. Uma nova camada de polígonos conectores é gerada entre a posição original e a nova tampa;
4. Por padrão, a extrusão segue a **normal média da face** selecionada;
5. Para forçar a extrusão em um eixo global, pressione `X`, `Y` ou `Z`;
6. Digite uma distância no teclado (ex: `0.5m`) ou arraste com o mouse;
7. Clique com `LMB` para confirmar ou `Escape` para cancelar e reverter a malha.
