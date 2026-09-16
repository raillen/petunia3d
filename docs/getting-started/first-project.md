# Seu Primeiro Projeto

Compreender como o Petunia3D organiza projetos, arquivos de cena e modelos é fundamental para manter um fluxo de trabalho eficiente e seguro.

## O Arquivo de Projeto (`.petunia`)

O formato nativo `.petunia` é um arquivo autocontido e determinístico que armazena:
- **Malhas e Geometrias**: Todas as malhas 3D com coordenadas de vértices, arestas selecionadas, faces poligonais, coordenadas UV e cores de vértices;
- **Coleções Hierárquicas**: A árvore de pastas/grupos definida no Outliner;
- **Anotações 3D**: Traços e rascunhos tridimensionais, incluindo cores, espessuras e subgrupos;
- **Medidas 3D**: Réguas métricas salvas na cena com suas posições e deltas cartesianos;
- **Configurações da Câmera**: Posição orbital, distância focal, projeção (perspectiva ou ortográfica) e posição do 3D Cursor.

## Salvar Projeto vs. Salvar Assets

Uma distinção crucial no Petunia3D é a diferença entre salvar o projeto inteiro e salvar um asset na biblioteca:

| Ação | Atalho / Local | O que faz | Onde fica salvo |
| :--- | :--- | :--- | :--- |
| **Salvar Projeto** | `Ctrl+S` (Menu Arquivo) | Salva a cena completa com todas as malhas, anotações, réguas e coleções. | Arquivo `.petunia` no seu disco rígido. |
| **Salvar como Asset** | Painel da Biblioteca de Assets | Salva a geometria da malha ativa como um modelo reutilizável dentro do projeto. | No catálogo interno do projeto, disponível na gaveta de assets para reutilização imediata. |

## Ciclo de Vida do Projeto

1. **Novo Projeto**: Ao abrir o Petunia3D ou pressionar `Ctrl+N`, um novo projeto com um cubo padrão posicionado na origem `(0, 0, 0)` é instanciado.
2. **Edição e Checkpoints**: Cada ação de modelagem cria um ponto no histórico de Undo (`Ctrl+Z`). A barra de status indica *não salvo* quando há alterações pendentes.
3. **Salvamento Automático e Seguro**: Pressione `Ctrl+S` para gravar. Ao salvar com sucesso, a barra de status passa a indicar *salvo*.
