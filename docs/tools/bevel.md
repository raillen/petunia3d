# Ferramenta: Round Edge (Bevel)

O **Round Edge** suaviza arestas e quinas vivas substituindo-as por superfícies inclinadas facetadas — o acabamento característico de assets low-poly.

> **Vocabulário:** a linguagem amigável é **Round Edge**. `Bevel` / `Chamfer` são os termos técnicos usados em documentação avançada, no código e no catálogo de comandos (`model.bevel`).

- **Atalho de Ativação**: `Ctrl+B` (preset Petunia)
- **Comando**: `model.bevel`
- **Disponível em**: componentes da malha, com arestas selecionadas (`Face` / `Edge` / `Point`); detalhe técnico: `EditMode::Edit`.

## Como Usar
1. Ative o domínio de seleção **Edge** (`2`) e selecione as arestas que deseja arredondar;
2. Pressione `Ctrl+B` ou selecione a ferramenta na Toolbar lateral;
3. Mova o mouse para ajustar a largura do arredondamento;
4. Digite uma largura exata no teclado (ex: `0.05m`);
5. Clique com `LMB` para confirmar ou `Escape` para cancelar e restaurar o modelo.

A operação é **transacional**: preview, validação de topologia e Undo fazem parte do contrato. Cancelar restaura a geometria original.

## Escopo no Core V1

O Core V1 entrega **1 segmento** — um único chanfro facetado por aresta, suficiente para o acabamento low-poly e alinhado ao princípio "composição de poucas operações previsíveis".

## Múltiplos segmentos (avançado)

A implementação atual suporta subdivisão multi-segmento ($N \ge 1$) com curvatura de arco circular:

- **Perfil de filete**: deslocamento calculado por interpolação não-linear em arco ($\text{bulge}(t) = (1 - (2t - 1)^2) \times 0.4142$);
- **Topologia manifold fechada**: os anéis intermediários são costurados diretamente nas faces de canto perpendiculares, sem criar T-junctions ou buracos na malha;
- **Preservação de planaridade**: faces adjacentes conservam orientação e coordenadas UV mapeadas.

> **Fronteira de escopo:** múltiplos segmentos são **evolução posterior / recurso avançado**, não requisito do Core V1. A interface principal não deve apresentá-los como caminho padrão.
