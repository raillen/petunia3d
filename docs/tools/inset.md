# Ferramenta: Inserção de Faces (Inset)

A ferramenta **Inserção** subdivide a face selecionada gerando um anel de novas faces recuadas paralelamente para dentro do polígono original.

- **Atalho de Ativação**: `I`
- **Disponível em**: componentes da malha (`Face` / `Edge` / `Point`); detalhe técnico: `EditMode::Edit`.

## Aplicações Práticas
- Criação de molduras de portas, janelas e painéis metálicos;
- Preparação de superfícies para posterior extrusão interior (rebaixos) ou exterior (botões/torres);
- Controle de fluxo topológico em cantos.

## Como Usar
1. Selecione uma ou mais faces;
2. Pressione `I` ou ative o botão na Toolbar;
3. Mova o cursor do mouse em direção ao centro da face para definir a espessura do recuo;
4. Digite um valor numérico exato no teclado (ex: `0.15m`);
5. Clique com `LMB` para confirmar ou `Escape` para cancelar.

## Proteção Contra Auto-Interseção (Guarded Inset)
Diferente de implementações ingênuas que invertem polígonos sob fatores altos, o Petunia3D implementa **Guarded Metric Inset**:
- **Verificação de Inversão de Normais**: Monitora o produto escalar entre o polígono interno recuado e a normal de referência da face original;
- **Amortecimento Step-Down**: Caso um deslocamento extremo tente colapsar ou inverter a face, a ferramenta amortece iterativamente o fator de recuo para preservar a 2-variedade e a integridade da malha;
- **Interpolação UV Automática**: Vértices inseridos calculam coordenadas UV contínuas preservando o mapeamento de texturas.

