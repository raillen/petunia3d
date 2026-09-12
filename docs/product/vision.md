# Visão de Produto — Petunia3D

Modelador 3D nativo, leve e focado em criação ultra-rápida de assets low-poly com estética retrô (PS1, N64, Nintendo DS e jogos indie nostálgicos).

## 1. Problem and Users (Problema e Usuários)

- **O Problema**: Ferramentas DCC tradicionais (como Blender, Maya ou 3ds Max) são hipercomplexas para desenvolvedores indie e artistas solo que desejam apenas produzir assets 3D low-poly estilizados. A curva de aprendizado íngreme, interface densa com centenas de atalhos e sobrecarga de menus atrasam a iteração de arte e prototipagem de jogos.
- **Os Usuários**: Desenvolvedores de jogos independentes, artistas 3D focados em estética retrô/PSX, participantes de game jams e estudantes que necessitam de um fluxo direto: desenhar silhueta sobre imagem de referência, gerar malha tridimensional, extrudar, mapear UVs em pixels e aplicar vertex paint em minutos.

## 2. Product Outcome (Resultado e Proposta de Valor)

- **Shape-First Workflow**: O usuário desenha o contorno da silhueta 2D sobre referências ortográficas e obtém imediatamente a malha gerada (extrusão ou revolução radial), permitindo modelagem estrutural sem manipulação tediosa de vértices individuais soltos.
- **Suíte Integrada de 4 Workspaces**:
  1. **MODEL**: Primitivas, Draw Profile, Extrude, Push/Pull, Inset, Bevel, Subdivide, Mirror e Merge.
  2. **PAINT**: Vertex painting direto e pintura albedo 2D com visualização texturizada em tempo real.
  3. **UV**: Editor sincronizado com seleção 3D, projeção planar e empacotamento de ilhas.
  4. **EXPORT**: Validação game-ready e exportação em lote OBJ ou GLB limpo e autocontido.
- **Desempenho Nativo em Hardware Antigo**: Renderizador OpenGL 3.3 Core via `glow` com fallback transparente a partir do `wgpu`, funcionando com menos de 80 MB de memória RAM idle e 0 frames renderizados quando ocioso (render-on-demand).

## 3. Success Boundaries (Critérios de Sucesso e Limites)

- **Limites de Sucesso**:
  - Tempo de inicialização inferior a 500 ms (baseline medido: 234 ms).
  - Consumo de memória idle abaixo de 150 MB (baseline medido: ~78 MB).
  - Malhas geradas com topologia limpa, sem vértices não referenciados e com coordenadas UV válidas no espaço [0, 1].
  - Arquivo de projeto `.petunia` binário e determinístico, com UUIDs persistentes por asset.
