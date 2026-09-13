# Ferramenta: Chanfro (Bevel)

O **Chanfro** suaviza arestas e quinas vivas substituindo-as por superfícies inclinadas facetadas.

- **Atalho de Ativação**: `Ctrl+B`
- **Modos Suportados**: Modo de Edição (`EditMode::Edit`).

## Como Usar
1. Selecione as arestas que deseja chanfrar (modo de seleção `2`);
2. Pressione `Ctrl+B` ou selecione a ferramenta na Toolbar lateral;
3. Mova o mouse para ajustar a largura do chanfro;
4. Digite uma largura exata no teclado (ex: `0.05m`);
5. Clique com `LMB` para confirmar ou `Escape` para cancelar e restaurar o modelo.

## Chanfro com Filetagem Arredondada (Multi-Segmentos)
O algoritmo de chanfro do Petunia3D suporta subdivisão em multi-segmentos ($N \ge 1$) com curvatura cilíndrica de arco circular:
- **Perfil de Filete**: Deslocamento suave calculado através de interpolação não-linear em arco parabólico ($\text{bulge}(t) = (1 - (2t - 1)^2) \times 0.4142$);
- **Topologia Manifold Fechada**: Os anéis intermediários são costurados diretamente nas faces de canto perpendiculares sem criar T-junctions ou buracos na malha;
- **Preservação de Planaridade**: Faces adjacentes conservam estrita orientação e coordenadas UV mapeadas.

