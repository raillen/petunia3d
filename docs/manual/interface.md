# Interface & Docking

A interface do Petunia3D adota uma hierarquia estável e determinística baseada na biblioteca `egui_tiles`, permitindo dividir e reorganizar painéis sem quebrar a coerência espacial da área de trabalho.

## Anatomia dos Painéis

```
+-----------------------------------------------------------------------------------+
| 1. Top Header: Menus [Arquivo] [Editar] [Ajuda]       Workspaces: [MODEL] [PAINT] |
+-----------------------------------------------------------------------------------+
| 2. Viewport Bar: Modo [Edit]  Seleção [V/E/F]  Add+  Travar: [X][Y][Z]  Shading ○● |
+--------+-------------------------------------------------------------+------------+
| 3.     | 4. Viewport 3D                                              | 5.         |
| Tool-  |                                                             | Outliner   |
| bar    | [Cena 3D interativa, Gizmos, 3D Cursor]                    | (Coleções) |
| (40px) |                                                             +------------+
|        |                                                             | 6.         |
|        | [Contextual Shelf inferior flutuante]                      | Properties |
+--------+-------------------------------------------------------------+ (Abas)     |
| 7. Status Bar: ● Salvo | Dicas de atalho contextuais | Tris: 12  Verts: 8        |
+-----------------------------------------------------------------------------------+
```

## Manipulação e Redimensionamento

- **Divisores de Painel (Splitters)**: Ao posicionar o cursor sobre as bordas entre o Viewport e a barra lateral direita, o cursor se transforma em uma seta bidirecional. Clique e arraste para redimensionar a largura do Outliner e Properties (faixa padrão entre 240px e 400px).
- **Controle de Escala de Interface (HiDPI)**: O Petunia3D escala perfeitamente em telas 4K e monitores de alta densidade de pixels. O fator de zoom pode ser customizado no modal de configurações (`Ctrl+,`).
- **Navegação Acessível por Teclado**: Todos os botões e campos de entrada podem ser focados via tecla `Tab` com anel de foco visível de alto contraste (`#5b8eff`).
