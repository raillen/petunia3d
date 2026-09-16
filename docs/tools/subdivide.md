# Ferramenta: Subdivisão (Subdivide)

A **Subdivisão** divide as faces e arestas selecionadas adicionando novos vértices intermediários para aumentar a resolução poligonal local.

- **Atalho de Ativação**: `Ctrl+D`
- **Disponível em**: componentes da malha (`Face` / `Edge` / `Point`); detalhe técnico: `EditMode::Edit`.

## Como Funciona
- Cada quadrilátero selecionado é dividido em 4 novos quadriláteros uniformes;
- Cada aresta selecionada é dividida pelo ponto médio;
- Ideal para preparar áreas que necessitam de maior densidade de vértices para detalhes.
