# Ferramenta: Régua 3D (Measure)

A ferramenta de **Medição 3D** permite aferir distâncias euclidianas precisas e deltas cartesianos entre quaisquer pontos no espaço tridimensional.

- **Atalho de Ativação**: `M` (ou clique no ícone da régua na Toolbar).
- **Modos Suportados**: Modo Objeto e Modo de Edição.

## Recursos e Funcionamento
- **Snapping Magnético a Vértices**: Ao aproximar o início ou o fim da régua de um vértice da malha, o ponto se prende magneticamente garantindo precisão milimétrica.
- **Badge Flutuante Informativo**: Exibe a distância linear total (ex: `1.414 m`) e a decomposição em deltas nos eixos: `ΔX`, `ΔY` e `ΔZ`.
- **Coleção Dedicada no Outliner**: Todas as réguas criadas são agrupadas automaticamente na coleção especializada `📏 Medidas` no topo do Outliner, com opções para ocultar (`👁`) ou excluir (`🗑`).
- **Suporte a Undo / Redo (`Ctrl+Z`)**: Criar ou apagar medidas gera checkpoints automáticos no histórico transacional.
