# O que é o Petunia3D?

O **Petunia3D** é um ambiente desktop de modelagem tridimensional nativo, leve e de alta precisão projetado com foco em **geometria low-poly**, **estética estilizada** e **criação de assets para jogos**.

## O que o Petunia3D é

- **Um modelador focado em formas**: Criado para construir modelos limpos com topologia quad/tri equilibrada, sem distrações com milhões de polígonos esculturais.
- **Rápido e nativo**: Desenvolvido integralmente em Rust com egui e renderização híbrida em WebGPU (com fallback para OpenGL ES). O executável é um binário único e abre em menos de 100 milissegundos.
- **Transacional e Confiável**: Todas as operações de edição respeitam um modelo de checkpoints atômicos. Você pode cancelar qualquer ação pressionando `Esc` e o estado da malha é preservado integralmente.
- **Totalmente customizável**: Suporta temas em TOML, icon packs com SVG soberano, tradução simples de interface e perfis de atalhos para quem vem do Blender, Maya ou 3ds Max.

## O que o Petunia3D NÃO é

Para manter a clareza cognitiva e o foco técnico, o Petunia3D deliberadamente não tenta ser:
- **Um substituto completo para o Blender em VFX**: Não implementamos simulação de fluidos, fumaça ou renderizadores de ray tracing com volumetria pesada;
- **Um software de escultura hiperdensa**: O foco é low-poly e mid-poly com contagem de polígonos controlada;
- **Um aplicativo baseado em Electron**: Não há navegador embutido nem consumo desnecessário de gigabytes de RAM.

## Inspirações

O design e os fluxos de trabalho do Petunia3D foram inspirados no melhor de clássicos renomados:
- **Blender**: Teclas modais fluidas (`G`, `R`, `S`, `E`, `I`), convenções de cores para eixos cartesianos (`X` vermelho, `Y` verde, `Z` azul) e 3D Cursor para posicionamento rápido;
- **Wings3D**: Modelagem direta de superfícies por seleção contextual de vértices, arestas e faces;
- **Blockbench / Sprytile**: Agilidade para prototipagem de assets prontos para motores como Godot, Unity e Unreal Engine.
