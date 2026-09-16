# Visão de Produto — Petunia3D

Modelador 3D nativo, leve e focado em **transformar referências e ideias visuais em modelos low-poly** com o menor atrito possível, mantendo um core pequeno e uma superfície avançada extensível por módulos/plugins.

> O Petunia3D **não** é um DCC genérico e não tenta substituir Blender/Maya. A meta é a ergonomia de workflows shape-first sobre um núcleo poligonal adequado a jogos.

## 1. Problem and Users (Problema e Usuários)

- **O Problema**: Ferramentas DCC tradicionais (como Blender, Maya ou 3ds Max) são hipercomplexas para desenvolvedores indie e artistas solo que desejam apenas produzir assets 3D low-poly estilizados. A curva de aprendizado íngreme, interface densa com centenas de atalhos e sobrecarga de menus atrasam a iteração de arte e prototipagem de jogos.
- **Os Usuários**: Desenvolvedores de jogos independentes, artistas 3D focados em estética retrô/PSX, participantes de game jams e estudantes que necessitam de um fluxo direto: desenhar silhueta sobre imagem de referência, gerar malha tridimensional, extrudar, mapear UVs em pixels e aplicar vertex paint em minutos.

## 2. Product Outcome (Resultado e Proposta de Valor)

- **Fluxo canônico**: `Reference → Draw/Create → Shape → Paint/Project → Check → Export`. Topologia, triangulação e UV continuam acessíveis, mas não são pré-requisito para o primeiro asset.
- **Dois caminhos igualmente válidos**: `Profile → Extrude/Shape` (desenhar sobre referência) e `Primitive → Direct Edit` (começar por uma primitiva). Desenhar nunca é obrigatório quando a primitiva resolve mais rápido.
- **Três workspaces V1** (UI Baseline Final, capítulo 36):
  1. **MODEL**: primitivas, Draw Profile, Extrude, Push/Pull, Inset, Round Edge (1 segmento), Subdivide, Mirror, Merge e Combine (`Keep Parts / Join / Fuse / Connect`).
  2. **PAINT**: Paint on Model sobre Albedo, com paleta simples, Pixel Grid contextual e editor 2D opcional (fechado por padrão).
  3. **UV**: split 2D + viewport 3D com seleção sincronizada, `55/45` por padrão.
  
  A checagem game-readiness e a exportação acontecem por comandos e painéis — **não** existe um workspace `EXPORT` na V1, e `Animation` pertence a pós-V1.
- **Desempenho nativo em hardware modesto**: viewport integrado por **egui-wgpu/wgpu** (stack final do capítulo 27), render **event-driven** — em idle não há renderização contínua — e orçamento de memória baixo por design.

## 3. Success Boundaries (Critérios de Sucesso e Limites)

- **Limites de Sucesso**:
  - Tempo de inicialização inferior a 500 ms (baseline medido: 234 ms).
  - Consumo de memória idle abaixo de 150 MB (baseline medido: ~78 MB).
  - Malhas geradas com topologia limpa, sem vértices não referenciados e com coordenadas UV válidas no espaço [0, 1].
  - Arquivo de projeto `.petunia` binário e determinístico, com UUIDs persistentes por asset.
