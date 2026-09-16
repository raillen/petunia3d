# Manual do Usuário

O **Manual do Usuário do Petunia3D** é a documentação canônica de referência técnica para todas as funções operacionais, mecânicas de interação, atalhos de produtividade e convenções de fluxo de trabalho do software.

## Sumário do Manual

- **[Interface & Docking](./interface)**: Layout de tela, manipulação de painéis e personalização visual.
- **[Viewport 3D & Câmera](./viewport)**: Navegação espacial, projeção ortográfica, esferas de sombreamento, modo Raio-X e o sistema de **Linhas-Guia de Travamento de Eixos**.
- **[Seleção](./selection)**: domínios unificados `Object`, `Face`, `Edge` e `Point`, com realce de hover, seleção por caixa e comandos de seleção.
- **[Fluxo de Modelagem](./modeling)**: Filosofia shape-first, transformações modais atômicas (`G`, `R`, `S`) e histórico transacional de checkpoints com Undo/Redo (`Ctrl+Z` / `Ctrl+Shift+Z`).
- **[Pintura & Cores](./paint)**: Paint on Model sobre Albedo, pincéis com raio dinâmico, paletas com import/export e camadas raster.
- **[Mapeamento UV](./uv)**: Auto UV, `Project From Reference/View`, packing, texel density e avisos de stretch.
- **[Biblioteca de Assets](./asset-library)**: A gaveta de modelos reutilizáveis, salvamento isolado de assets e instanciação rápida.
- **[Projetos (.petunia)](./projects)**: Anatomia interna do formato de projeto e integridade de dados.
- **[Exportação & Formatos](./export)**: `.glb` (principal) e `.obj` (secundário) para pipelines de jogos; FBX fora da V1.

---

## Convenções Visuais de Teclado e Mouse

Ao longo deste manual, utilizamos as convenções universais do setor:
- **`LMB`**: Botão esquerdo do mouse (*Left Mouse Button*) — seleção e confirmação.
- **`RMB`**: Botão direito do mouse (*Right Mouse Button*) — menus de contexto e cancelamento imediato de ações modais.
- **`MMB`**: Botão do meio do mouse (*Middle Mouse Button* / Scroll) — rotação orbital e pan segurando `Shift`.
- **`[Tecla]`**: Atalhos de teclado (ex: `G`, `R`, `S`, `Tab`, `Ctrl+Z`).
