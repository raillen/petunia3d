# Manual do Usuário

O **Manual do Usuário do Petunia3D** é a documentação canônica de referência técnica para todas as funções operacionais, mecânicas de interação, atalhos de produtividade e convenções de fluxo de trabalho do software.

## Sumário do Manual

- **[Interface & Docking](./interface)**: Layout de tela, manipulação de painéis e personalização visual.
- **[Viewport 3D & Câmera](./viewport)**: Navegação espacial, projeção ortográfica, esferas de sombreamento, modo Raio-X e o sistema de **Linhas-Guia de Travamento de Eixos**.
- **[Modos de Seleção](./selection)**: Modos Objeto, Vértice (com demarcação visual de hover), Aresta e Face, além de seleção por caixa (`Box Select`).
- **[Fluxo de Modelagem](./modeling)**: Filosofia shape-first, transformações modais atômicas (`G`, `R`, `S`) e histórico transacional de checkpoints com Undo/Redo (`Ctrl+Z` / `Ctrl+Shift+Z`).
- **[Pintura & Cores](./paint)**: Pintura de vértices e texturas, pincéis com ajuste dinâmico de raio e paletas de cores.
- **[Mapeamento UV](./uv)**: Projeções ortogonais e unwrapping UV para exportação game-ready.
- **[Linha do Tempo](./animation)**: Linha do tempo de quadros-chave e controles de transporte.
- **[Biblioteca de Assets](./asset-library)**: A gaveta de modelos reutilizáveis, salvamento isolado de assets e instanciação rápida.
- **[Projetos (.petunia)](./projects)**: Anatomia interna do formato de projeto e integridade de dados.
- **[Exportação & Formatos](./export)**: Exportação otimizada para os motores Godot, Unity, Unreal Engine e Blender.

---

## Convenções Visuais de Teclado e Mouse

Ao longo deste manual, utilizamos as convenções universais do setor:
- **`LMB`**: Botão esquerdo do mouse (*Left Mouse Button*) — seleção e confirmação.
- **`RMB`**: Botão direito do mouse (*Right Mouse Button*) — menus de contexto e cancelamento imediato de ações modais.
- **`MMB`**: Botão do meio do mouse (*Middle Mouse Button* / Scroll) — rotação orbital e pan segurando `Shift`.
- **`[Tecla]`**: Atalhos de teclado (ex: `G`, `R`, `S`, `Tab`, `Ctrl+Z`).
