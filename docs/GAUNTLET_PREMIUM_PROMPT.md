# Prompt Mestre — Gauntlet Loop para Excelência de Software 3D Premium (Petunia3D)

> **Instruções de Uso**: Este prompt foi desenhado para ser injetado diretamente em um assistente autônomo ou em um enxame de subagentes especializados no ciclo **Gauntlet Loop**. Ele contém a especificação completa, referências da indústria, padrões de interação direta no viewport e critérios rigorosos para elevar o **Petunia3D** ao nível de acabamento e fluidez de softwares consolidados como **Blender (Edit Mode)**, **Blockbench**, **Wings 3D**, **Plasticity**, **Shapr3D**, **Crocotile 3D** e **picoCAD**.

---

```markdown
# MISSÃO: ELEVAÇÃO DO PETUNIA3D AO PADRÃO PREMIUM DE INTERAÇÃO E MODELAGEM DIRETA

Você está assumindo a evolução definitiva do **Petunia3D**, um modelador tridimensional low-poly focado em arte para jogos, desenvolvido em Rust (OpenGL 3.3 Core via `glow` com fallback `wgpu`, e interface em modo imediato `egui`).

A base técnica, matemática e arquitetural do projeto já possui um núcleo sólido (grafo de crates desacoplado, estrutura Half-Edge, algoritmos de Newell, testes unitários e conformidade Prumo v0.5). No entanto, **o software ainda falha no aspecto mais crítico para um artista 3D: a ergonomia de manipulação direta no viewport**. 

Atualmente, operações cruciais comportam-se como botões estáticos ou caixas de diálogo numéricas em painéis secundários, em vez de **interações fluidas, visuais e manipuláveis diretamente na tela através do mouse e atalhos contextuais**.

Sua missão NÃO é criar protótipos nem adicionar botões secundários em abas laterais.
Sua missão é **revolucionar a experiência de uso (UX), o pipeline de interação no viewport e a robustez algorítmica** através de ciclos repetidos do **Gauntlet Loop**, implementando uma engine de manipulação direta no mesmo patamar de excelência de programas de referência do mercado.

---

## 1. BENCHMARKING DE PROGRAMAS DE REFERÊNCIA

O Petunia3D deve combinar o melhor de cada ferramenta de referência consagrada, adaptado à filosofia de modelagem low-poly limpa e sem sobrecarga cognitiva:

### 1.1. Blender (Edit Mode) — O Padrão Ouro de Modalidade & Atalhos
- **Operações Modais no Viewport**: Pressionar `G` (Move), `R` (Rotate), `S` (Scale), `E` (Extrude), `I` (Inset), `Ctrl+B` (Bevel) ou `K` (Knife) entra instantaneamente em um estado modal onde **o movimento do mouse no viewport controla o valor em tempo real com taxa de atualização contínua e fluida**.
- **Cancelamento Limpo (Escape / Botão Direito)**: Cancelar uma operação a qualquer momento com `Esc` ou `RMB` reverte a malha com precisão cirúrgica para o estado imediatamente anterior ao início do drag, sem poluir o histórico de Desfazer (`Undo`).
- **Confirmação Clara (Enter / Botão Esquerdo)**: Clicar com `LMB` ou pressionar `Enter` comita a alteração e cria um único checkpoint indivisível no histórico de Undo.
- **Restrição Numérica e por Eixos**: Durante qualquer transformação ou extrusão, pressionar `X`, `Y` ou `Z` trava a operação no respectivo eixo cartesiano; pressionar `Shift+X/Y/Z` trava no plano perpendicular. Digitar um número (ex: `0.5` ou `-1.2`) define a distância exata.
- **Navegação Consistente**: `MMB` orbita ao redor do ponto focal, `Shift+MMB` faz pan horizontal/vertical, e a tecla `F` (ou Numpad `.`) centraliza e enquadra suavemente a câmera na seleção atual (*Frame Selection*).

### 1.2. Blockbench — Fricção Zero & Sincronização Pixel-Art
- **Seleção Sem Ambiguidade**: Feedback visual instantâneo do que está sob o cursor; seleção de faces, arestas e vértices com outline destacado e nítido.
- **Pivô Inteligente & Manipulável**: O centróide da seleção é recalculado dinamicamente; o usuário pode alterar o ponto de pivô com facilidade.
- **Pintura 3D Direta na Malha**: Clicar e arrastar com a ferramenta de pintura pinta diretamente sobre o canvas de textura UV ou vértice, com conta-gotas imediato (`Alt` ou `I`) e ferramenta de balde de tinta que preenche a face inteira com um clique.
- **Latência de Entrada Mínima**: Resposta tátil imediata, sem atrasos de frame ou dessincronizações entre o cursor e o elemento 3D.

### 1.3. Wings 3D — Ergonomia Contextual & Pureza Topológica
- **Menu Contextual Inteligente (RMB)**: Clicar com o botão direito abre um menu contextual contendo apenas as operações matematicamente válidas para o elemento selecionado:
  - *Selecionei Vértice*: Mover, Deslocar, Conectar a outro vértice, Colapsar, Dissolver.
  - *Selecionei Aresta*: Bevel/Chanfro, Loop Cut, Dissolver, Conectar, Colapsar.
  - *Selecionei Face*: Extrude Normal, Inset, Bevel, Bridge/Ponte, Duplicar, Deletar.
- **Garantia de 2-Variedade**: Nenhuma operação aceita produzir geometria corrompida, não-planar ou com buracos acidentais sem avisar e proteger o usuário.

### 1.4. Plasticity & Shapr3D — Manipulação Direta Elegante & Micro-Interações
- **Gizmos 3D Visuais de Alta Resolução**: Setas tridimensionais (Vermelho=X, Verde=Y, Azul=Z) projetadas sobre a seleção, com planos intermediários para translação livre em 2D.
- **Hover Pre-Selection Highlight**: Antes de clicar, passar o mouse sobre qualquer face, aresta ou vértice ilumina sutilmente o elemento (*pre-highlight*), garantindo que o usuário nunca erre o clique.
- **Direct Push/Pull Handle**: Selecionar uma face revela instantaneamente uma alça/seta perpendicular no seu centróide; arrastar essa alça empurra ou puxa o volume da geometria diretamente.

### 1.5. Crocotile 3D & picoCAD — Disciplina Low-Poly & Mapeamento Imediato
- **Snapping Magnético a Grade (Grid Snap)**: Snapping configurável em tempo real (0.125m, 0.25m, 0.5m, 1.0m) ativado ao segurar `Ctrl`.
- **Mapeamento UV de 1 Clique**: Atribuição de texturas e projeção de faces sem exigir abertura de editores UV complexos para tarefas triviais; atalhos rápidos para girar UVs em 90° e inverter horizontalmente/verticalmente.

---

## 2. DIAGNÓSTICO PROFUNDO: O QUE DEVE SER CORRIGIDO NO PETUNIA3D

Para que qualquer operação funcione de forma 100% correta e intuitiva, você deve erradicar os seguintes problemas fundamentais:

1. **Eliminar Ações Instantâneas e Não-Interativas em Ferramentas**:
   - Nenhuma ferramenta deve realizar mutações destrutivas ao ser apenas clicada na barra de ferramentas.
   - Ferramentas como Extrusão, Inset, Bevel, Transformação e Fatiamento não podem depender de o usuário digitar números em uma barra lateral antes de ver o resultado. Elas devem ser **iniciadas no viewport e controladas pelo mouse**.
2. **Implementar a Máquina de Estados Modal (`ModalOp`) no Viewport**:
   - Ao iniciar uma operação (por atalho ou clique no botão da ferramenta), o editor entra em estado interativo:
     - O cursor do mouse é ocultado ou alterado para cursor contextual.
     - Uma linha guia visual é traçada entre o ponto inicial do drag e o cursor atual.
     - Um mini-HUD flutuante próximo ao cursor exibe a métrica em tempo real (ex: `Distância: +0.45m`, `Ângulo: 45.0°`, `Segmentos: 2`).
     - Mover o mouse altera a geometria em tempo real a 60 FPS contínuos.
     - Segurar `Ctrl` ativa snapping incremental (ex: passos de 0.1m ou 15°).
     - Pressionar `X`, `Y` ou `Z` trava a transformação no respectivo eixo global ou local.
     - `LMB` ou `Enter` confirma e grava no histórico de Desfazer (`undo checkpoint`).
     - `RMB` ou `Esc` cancela e reverte a malha para o estado exato inicial.
3. **Implementar Gizmo 3D Completo e Interativo no Viewport**:
   - Um gizmo 3D com teste de interseção de raio (*ray-gizmo intersection*) desenhado no centróide da seleção atual:
     - Setas direcionais X (Vermelho), Y (Verde), Z (Azul) para Translação.
     - Arcos/círculos para Rotação com indicação visual de ângulo.
     - Quadrados/cubos nas pontas para Escala.
     - Planos nos cantos dos eixos para movimentação livre no plano (XY, XZ, YZ).
   - O usuário pode clicar diretamente em qualquer eixo do gizmo no viewport 3D e arrastar para transformar a geometria selecionada com restrição automática ao eixo.
4. **Precisão de Raycasting e Sistema de Picking com Pre-Highlight**:
   - O raycasting deve priorizar a profundidade do buffer (*depth sorting*), selecionando apenas elementos visíveis (não obstruídos por faces frontais), a menos que o modo Raio-X / Wireframe esteja ativo.
   - Distância de tolerância de clique em pixels na tela:
     - Vértices: ~8 pixels de raio de tolerância.
     - Arestas: ~5 pixels de proximidade linear.
     - Faces: teste preciso de raio-triângulo com seleção do mais próximo da câmera.
   - Pré-visualização de seleção (*hover highlight*): ao passar o mouse sobre o modelo sem clicar, o vértice/aresta/face sob o cursor deve ser realçado visualmente.
5. **Comutação Fluida de Nível de Seleção**:
   - Teclas padrão universais: `1` para Vértices, `2` para Arestas, `3` para Faces, `4` para Objeto/Corpo inteiro.
   - A barra de ferramentas e os atalhos devem adaptar os comandos contextualmente ao nível ativo.
6. **Navegação de Câmera Suave e Precisa**:
   - Tecla `F` deve calcular a AABB (Axis-Aligned Bounding Box) ou esfera delimitadora da seleção atual e interpolar suavemente a posição e o ponto de foco da câmera para enquadrar a seleção.
   - Teclas numéricas `Numpad 1` (Front), `Numpad 3` (Right), `Numpad 7` (Top) e `Numpad 9` (Inverter) devem alternar instantaneamente para visão ortográfica travada no plano de trabalho sem desorientar o artista.

---

## 3. ESPECIFICAÇÃO TÉCNICA DAS OPERAÇÕES INTERATIVAS

Cada operação abaixo deve ser verificada e implementada com rigor absoluto:

### 3.1. Extrusão Interativa (`ExtrudeTool`)
- **Gatilho**: Tecla `E` ou botão na barra de ferramentas com face(s) selecionada(s).
- **Comportamento**:
  1. Cria novas faces conectando a base ao novo conjunto de faces extrudadas.
  2. Entra no modo modal: o vetor normal médio da seleção determina o eixo de deslocamento padrão.
  3. O deslocamento do mouse ao longo do vetor projetado em espaço de tela calcula a distância $d$.
  4. HUD no viewport exibe: `Extrude: <d>m  [Ctrl: Snap 0.1m | X/Y/Z: Travar Eixo | LMB: Aplicar | RMB: Cancelar]`.
  5. `Alt+E` abre menu de extrusão: *Extrude along Normals* (cada face na sua própria normal) vs *Extrude Individual Faces*.

### 3.2. Inset Interativo (`InsetTool`)
- **Gatilho**: Tecla `I` com face(s) selecionada(s).
- **Comportamento**:
  1. Insere um novo anel de faces conectadas ao contorno da face original.
  2. Mover o mouse em direção ao centróide reduz a área interna; mover para fora expande.
  3. HUD exibe: `Inset: <distancia>m  [Ctrl: Snap | LMB: Aplicar | RMB: Cancelar]`.
  4. Detecção automática para impedir auto-interseção de vértices quando a distância excede o raio máximo da face.

### 3.3. Chanfro / Bevel Interativo (`BevelTool`)
- **Gatilho**: Tecla `Ctrl+B` com aresta(s) ou face(s) selecionada(s).
- **Comportamento**:
  1. O deslocamento do mouse a partir do centro da seleção define a largura do chanfro.
  2. Girar a roda do mouse (`Scroll Up / Down`) aumenta ou diminui dinamicamente a quantidade de segmentos (de 1 a 8 cortes arredondados).
  3. HUD exibe: `Bevel: <largura>m | Segmentos: <N>  [Scroll: Ajustar cortes | LMB: Aplicar | RMB: Cancelar]`.
  4. Tratamento matemático estrito de cantos para manter malhas fechadas e normais suaves calculadas por Newell.

### 3.4. Corte em Loop Interativo (`LoopCutTool`)
- **Gatilho**: Tecla `Ctrl+R`.
- **Comportamento**:
  1. Ao passar o cursor sobre qualquer aresta, o sistema encontra o loop de quads adjacentes e desenha uma linha amarela brilhante de pré-visualização contornando todo o anel.
  2. Rolar a roda do mouse adiciona múltiplos cortes paralelos (1, 2, 3, 4...).
  3. Primeiro clique com `LMB` comita o corte na posição inicial (50%) e entra no modo de deslizamento (*Edge Slide*).
  4. Mover o mouse desliza os cortes ao longo das arestas (de -100% a +100%).
  5. Segundo clique com `LMB` confirma a posição; `RMB` posiciona exatamente no centro (0%).

### 3.5. Fatiamento Planar e Faca Interativa (`SliceTool` / `KnifeTool`)
- **Gatilho**: Tecla `K` (Knife interativa) ou `Shift+K` (Fatiador Planar).
- **Comportamento**:
  1. *Knife*: O cursor exibe um ícone de bisturi/faca. Clicar em uma aresta fixa o ponto inicial; mover o mouse exibe a linha de corte; clicar na aresta oposta insere a divisão nas faces atravessadas. Pressionar `Enter` executa os cortes.
  2. *Slice Plane*: Clicar e arrastar desenha um plano infinito de corte perpendicular à tela; o volume é bisseccionado e as tampas poligonais são fechadas e soldadas automaticamente via `cut_edge_cache`.

### 3.6. Empurrar/Puxar Direto (`PushPullTool`)
- **Gatilho**: Clicar e arrastar diretamente sobre qualquer face sem necessidade de ferramenta pré-selecionada (ou tecla `P`).
- **Comportamento**:
  1. Desloca a geometria da face ao longo de sua normal, criando geometria volumétrica quando conectada a arestas adjacentes ou transladando superfícies planares com preservação de continuidade.

### 3.7. Pintura e Projeção UV Direta no Viewport (`PaintModule`)
- **Gatilho**: Workspace Paint ou tecla `B` (Brush).
- **Comportamento**:
  1. Clicar e arrastar sobre o modelo 3D no viewport projeta o pincel sobre a textura ou atribui cores da paleta aos vértices mais próximos do raio do pincel.
  2. Círculo do pincel renderizado na superfície da malha (*decal/projected circle*) indicando o raio de ação em tempo real.
  3. Tecla `G` ou `Alt` captura a cor sob o mouse (Eyedropper).
  4. Tecla `F` (quando no Paint) ajusta o raio do pincel interativamente arrastando o mouse.

---

## 4. O LOOP DO GAUNTLET: ENXAME DE AGENTES AUDITORES

Para alcançar a máxima qualidade prática, o trabalho deve ser executado através de subagentes especializados operando em rodadas contínuas. A cada iteração, cada perspectiva deve emitir notas honestas (0 a 10) e uma lista detalhada de defeitos que impeçam a experiência de um software premium:

### Subagentes Auditores Necessários:
1. **Auditor de Interação 3D & Gizmos**:
   - Avalia a precisão matemática do picking de raios, estabilidade de arrasto dos gizmos, resposta a eventos do teclado e ausência de travamentos ou atrasos perceptíveis.
2. **Auditor de Ergonomia de Modelagem & Ferramentas**:
   - Testa interativamente as ferramentas (Extrude, Inset, Bevel, Loop Cut, Slice, Push/Pull) contra o fluxo real de trabalho de um artista 3D.
   - Verifica se o cancelamento com `Esc` restaura perfeitamente o estado anterior e se a confirmação com `Enter/LMB` gera um checkpoint atômico.
3. **Auditor de Renderização, Overlays & Shading**:
   - Inspeciona o antialiasing de arestas, renderização de gizmos em primeiro plano (sempre legíveis), iluminação Flat/Smooth e visualização de referências sem oclusão indevida.
   - Garante que a aplicação permaneça a 0 FPS em repouso absoluto (*render-on-demand*), mas atinja 60 FPS lisos durante qualquer manipulação ativa com o mouse.
4. **Auditor de Robustez, Topologia & QA Hostil**:
   - Submete malhas a operações extremas (arrasto com valores negativos gigantes, chanfro em geometrias não-convexas, fatiamento rente a vértices existentes, polígonos degenerados com área quase nula).
   - Exige política de **ZERO PANIC**: nenhum `.unwrap()` desprotegido em caminhos de execução da UI, entrada do usuário ou deserialização.
5. **Auditor de Arquitetura, Segurança & Governança Prumo**:
   - Garante conformidade total com o contrato Clean Architecture de 13 crates acíclicos, suites de testes unitários/integrados automatizados em `cargo test --workspace` e `cargo clippy --all-targets -- -D warnings`.

---

## 5. CRITÉRIOS DE ACEITE E DECLARAÇÃO DE CONVERGÊNCIA PREMIUM

O Gauntlet Loop só pode ser considerado concluído quando **todas** as seguintes condições forem plenamente comprovadas com evidências reproduzíveis:

1. **Teste do "Caminho Dourado" (The Golden Path Asset Test)**:
   - Um usuário deve ser capaz de criar um modelo low-poly clássico (ex: uma espada estilizada, um baú com tampa chanfrada ou uma cadeira) partindo de um cubo inicial, utilizando **exclusivamente atalhos de teclado e manipulação direta com o mouse no viewport**, em menos de 2 minutos, sem abrir nenhum menu numérico de texto lateral.
2. **Manipulação por Gizmo 100% Funcional**:
   - Clicar nas setas X, Y, Z ou nos planos do gizmo arrasta a seleção perfeitamente restrita ao eixo escolhido, com feedback numérico em HUD flutuante no viewport.
3. **Comportamento Modal Impecável**:
   - Cancelar qualquer operação em andamento com `Esc` ou `RMB` reverte instantaneamente 100% das mutações geométricas sem deixar vértices órfãos ou duplicados.
4. **Pré-seleção Visual (Hover Highlight)**:
   - Passar o mouse sobre qualquer parte da malha destaca o componente selecionável antes do clique.
5. **Automação de Testes e Sanidade Estática**:
   - `cargo test --workspace` passa com 100% de sucesso.
   - `cargo clippy --workspace --all-targets -- -D warnings` emite zero warnings e zero erros.
   - `cargo run -- --smoke-test` valida todo o ciclo de vida sem interface gráfica.
6. **Média Ponderada do Gauntlet $\ge 9.8 / 10$**:
   - Nenhuma categoria individual pode receber nota inferior a 9.5/10.

---

Execute agora a auditoria inicial dessa nova fase, trace o plano cirúrgico de implementação das operações interativas e proceda sem hesitar até atingir o patamar definitivo de um software 3D premium.
```
