# Documentação de Interface e Design System — Petunia3D (`docs/ui/`)

Especificação de fluxos de usuário, telas, estados de interação, tokens de design, acessibilidade e validação visual do Petunia3D.

---

## 1. User Flows (Fluxos de Usuário)

O fluxo primordial do Petunia3D é o **Shape-First 3D Asset Creation Flow**:

```mermaid
flowchart LR
    A[1. Carregar Referência 2D] --> B[2. Desenhar Silhueta (Draw Profile)]
    B --> C[3. Gerar Malha (Extrude/Revolve)]
    C --> D[4. Refinar Topologia (Inset/Bevel)]
    D --> E[5. Mapear UV e Ilhas]
    E --> F[6. Vertex Paint / Textura]
    F --> G[7. Validação e Exportação (OBJ/GLB)]
```

1. **Importação de Referências**: Imagens ortográficas (frente, lado, topo) são carregadas no viewport como guias de modelagem com transparência regulável.
2. **Desenho de Perfil**: O usuário traça vértices sobre a imagem e aciona revolução radial (ex: garrafas, copos, rodas) ou extrusão ortogonal (ex: lâminas, prédios, móveis).
3. **Refinamento**: Aplicação de operações fundamentais (Bevel em quinas, Inset para cavidades, Subdivisão para detalhe estruturado).
4. **Pintura e Cores**: Aplicação de cores por vértice diretamente sobre as faces com paleta estilizada.
5. **Exportação**: Inspeção de conformidade topológica e gravação do asset final para motores de jogo (Godot, Unity).

---

## 2. Screens and States (Telas e Estados de Usuário Suportados)

### Workspaces (Pílulas no Cabeçalho Superior)
A aplicação apresenta uma barra de abas estilo "pílula" no topo que transiciona entre os quatro contextos operacionais:
- **MODEL**: Focado no viewport 3D, ferramentas de topologia e árvore de assets da cena.
- **PAINT**: Viewport configurado com shading de cores de vértice/albedo, seletor de paleta de cores, pincel e ferramentas de pintura 2D/3D.
- **UV**: Layout bipartido (viewport 3D à esquerda e tela plana de projeção UV [0, 1] à direita sincronizada com a seleção).
- **EXPORT**: Painel de conferência com contagem de polígonos, verificação de erros de malha e opções de exportação em lote.

### Estados de Seleção e Ferramentas
- **Modos de Sub-elementos**: `Vertex` (1), `Edge` (2) e `Face` (3).
- **Estados de Interação**:
  - `Idle`: Nenhum evento pendente; zero renderizações na GPU (consumo mínimo de energia e bateria).
  - `Hovering`: Destaque visual em tempo real sob o cursor do mouse.
  - `Active Transform`: Manipulação interativa de translação, rotação ou escala via atalhos e gizmos.
  - `Camera Orbit / Pan / Zoom`: Transformações matriciais suaves sem stuttering.

---

## 3. Accessibility (Acessibilidade e Ergonomia)

- **Contraste e Legibilidade**: Tema escuro com alto contraste nativo (dark mode contrastante) para prevenir fadiga visual prolongada.
- **Navegação 100% por Teclado**: Todas as ferramentas e trocas de modo possuem atalhos dedicados mapeados e reconfiguráveis via `assets/keybinds/petunia.toml`.
- **Dicas Visuais e Tooltips**: Cada botão da interface exibe seu nome, descrição breve e respectivo atalho de teclado entre parênteses ao passar o cursor.

---

## 4. Visual Validation (Validação Visual)

- O projeto adota validações visuais contínuas registradas nos relatórios do Gauntlet ([`docs/GAUNTLET.md`](../GAUNTLET.md)):
  - Inspeção visual de anti-aliasing e desenho de wireframe.
  - Integridade visual de renderização de quads e triângulos sob iluminação Gouraud/flat.
  - Sincronização em tempo real entre alterações na malha e na visualização da projeção UV.

## 5. Iconografia e tokens de interface

Os ícones de ferramentas são desenhos vetoriais de `crates/ui/src/icons.rs`, construídos em uma grade lógica de 24 pontos. Não dependem de símbolos Unicode ou fontes de ícones. A variante compacta usa alvo de 40 × 40 pontos; a variante ampla mantém o ícone e o nome na mesma linha. O nome completo permanece na informação acessível do botão em ambas as variantes; o chamador fornece tooltip e atalho.

O componente `tool_button(ui, id, label, selected, compact)` respeita estados habilitado, hover, seleção e foco de teclado. A seleção combina fundo e marcador lateral; o foco acrescenta contorno visível. A pintura isolada `icons::paint` permite reutilizar câmera, desfazer e refazer nos controles do editor.

`assets/themes/dark.toml` é a fonte de configuração do tema, com defaults tipados em `crates/config/src/theme.rs`. As cores são semânticas: fundo, painel, superfície de controle, hover, seleção, texto, texto secundário, destaque e borda. A tipografia padrão é proporcional, com corpo e botões de 14 pontos, texto secundário de 12 e títulos de 18. A escala é em pontos egui e acompanha o DPI.

Os testes verificam contraste mínimo de 4,5:1 nos pares de texto efetivamente usados, estado/foco e ativação de botão por teclado. A validação visual com captura OpenGL continua necessária; esses testes não comprovam suporte a leitor de tela nem equivalência visual entre drivers.
