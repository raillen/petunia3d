# Ferramenta: Push / Pull

A ferramenta **Push / Pull** permite empurrar ou puxar faces planas na direção normal sem criar nova topologia, reposicionando as faces adjacentes de forma coerente.

- **Atalho de Ativação**: `Shift+P`
- **Modos Suportados**: Modo de Edição (`EditMode::Edit`).

## Diferença em Relação ao Extrude
- Enquanto o **Extrude (`E`)** cria novas faces laterais e nova geometria, o **Push/Pull** apenas desloca a superfície existente e estica as arestas conectadas, preservando a contagem de polígonos intacta.
