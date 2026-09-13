---
layout: home

hero:
  name: "Petunia3D"
  text: "Modelador 3D Low-Poly Focado, Nativo e Veloz"
  tagline: "Software desktop soberano em Rust e egui. Modelagem shape-first inspirada no Blender e Wings3D, renderização híbrida WebGPU/OpenGL e ausência total de inchaço."
  image:
    src: /assets/logo.svg
    alt: Petunia3D Logo
  actions:
    - theme: brand
      text: Começar Agora (15 min)
      link: /getting-started/
    - theme: alt
      text: Manual Completo
      link: /manual/
    - theme: alt
      text: Catálogo de Ferramentas
      link: /tools/
    - theme: alt
      text: Repositório GitHub
      link: https://github.com/raillen/petunia3d

features:
  - icon: 📐
    title: Modelagem Shape-First & Modais
    details: Transformações modais atômicas (Mover, Rotacionar, Escalar), extrusão precisa, chanfro, inserção de faces e cortes topológicos com linhas-guia infinitas e feedback em tempo real.
  - icon: 🎯
    title: 3D Cursor, Medição & Anotação
    details: Posicionamento espacial com Shift+RMB, régua 3D com snapping magnético a vértices e ferramenta de grease-pencil com undo/redo total e coleções no outliner.
  - icon: 🔒
    title: Travamento de Eixos & Planos
    details: Feedback visual em 3 camadas (linhas 3D coloridas, HUD flutuante dinâmico e botões na Viewport Bar) com alternância rápida por X, Y, Z e Shift.
  - icon: 🎨
    title: Customização & Zero Hardcoded Colors
    details: 4 temas nativos (Dark, Light, Capuccino, Tokyo Nights), 5 pacotes de ícones vetoriais, 8 perfis de atalhos e internacionalização facilitada em arquivos TOML.
  - icon: ⚡
    title: Performance Nativa em Rust
    details: Inicialização instantânea (<100ms), pegada mínima de memória, renderização moderna em WebGPU com fallback automático para OpenGL ES e sem nenhum framework web pesado no desktop.
  - icon: 📦
    title: Biblioteca de Assets & Organização
    details: Gaveta integrada de modelos reutilizáveis, coleções hierárquicas no Outliner, bloqueio contra edições acidentais e isolamento de objetos com um clique.
---

## Conheça o Petunia3D

O **Petunia3D** foi projetado para artistas 3D, desenvolvedores de jogos indie e criadores de protótipos rápidos que buscam a agilidade e elegância dos atalhos clássicos do Blender sem o peso de um pacote de produção hipercomplexo.

```mermaid
graph LR
    subgraph Entrada["Fluxo de Criação"]
        A["Adicionar Primitivas / 3D Cursor"] --> B["Modo de Edição (Vértice / Aresta / Face)"]
        B --> C["Modelagem & Cortes (Extrude, Loop Cut)"]
    end
    subgraph Refinamento["Refinamento & Inspenção"]
        C --> D["Medidas 3D & Anotações"]
        D --> E["Pintura de Paleta & UV Unwrapping"]
    end
    subgraph Saida["Entrega Game-Ready"]
        E --> F["Exportação GLTF / OBJ"]
        F --> G["Godot / Unity / Unreal"]
    end
```

<div class="tip custom-block" style="padding-top: 8px">

### 💡 Começando em 3 passos simples
1. Baixe ou compile o binário único: `cargo run --release`
2. Siga o tutorial de 15 minutos em [Seu Primeiro Modelo](/getting-started/your-first-model)
3. Conheça os atalhos canônicos no [Cheatsheet de Teclado](/shortcuts/cheatsheet)
</div>
