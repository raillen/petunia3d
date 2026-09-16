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

## Fluxo recomendado

```
REFERENCE → DRAW/CREATE → SHAPE → PAINT/PROJECT → CHECK → EXPORT
```

Topologia, triangulação e UV continuam acessíveis, mas **não** são pré-requisito para
produzir o primeiro asset.

## Inspirações

O Petunia3D é um **modelador low-poly shape-first e direct-mesh**, inspirado em:

- **MoI 3D**: simplicidade conceitual e clareza das operações;
- **Plasticity**: interação direta com o modelo;
- **Blockbench**: acessibilidade low-poly e prototipagem de assets para jogos;
- **Blender**: edição de mesh e ergonomia de transformações modais.

A referência é a **ergonomia** desses workflows sobre um núcleo poligonal — não a
adoção de um kernel CAD/NURBS (B-Rep) como arquitetura central.

> ⚠️ O Petunia3D **não é um editor 2D que adivinha 3D**. Um profile é uma forma plana
> localizada em um **Work Plane conhecido no espaço 3D**. Existem dois caminhos
> igualmente válidos: `Profile → Extrude/Shape` e `Primitive → Direct Edit` — desenhar
> não é obrigatório quando uma primitiva resolve mais rápido.
