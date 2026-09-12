# Retomada — Gauntlet Petunia3D

Atualizado: 2026-09-12. Rodada 2 concluída com sucesso.

## Pedido e continuidade

Continuidade do Gauntlet com foco em câmera ortográfica, ícones vetoriais, UI responsiva
e legível em janelas estreitas, e campos numéricos preenchíveis para ferramentas.
Preservação estrita de prévia transacional, cancelamento exato e histórico atômico de undo/redo.

## Estado comprovado nesta rodada (Rodada 2)

- **130 testes automatizados** passando em todo o workspace (`cargo test --workspace`).
- **Clippy estrito aprovado**: 0 erros e 0 warnings com `-D warnings`.
- **Formatação canônica**: `cargo fmt --all -- --check` 100% aderente.
- **Teste de fumaça aprovado**: `petunia3d --smoke-test` passou sem interface gráfica.
- **Commit de baseline para rollback criado**:
  - Hash: `594dcd6`
  - Tags: `rollback-point`, `checkpoint-v0.2.0`

## Entregas implementadas na rodada

1. **Câmera Ortográfica & Controles de Câmera (`core/camera`, `ui/camera_controls`)**:
   - Suporte explícito a projeções `Perspective` e `Ortho`.
   - Seis vistas canônicas (`Front`, `Back`, `Right`, `Left`, `Top`, `Bottom`) com atalhos numéricos (`Numpad 1/3/7` e variantes com `Ctrl`).
   - Controle de enquadramento e escala com ajuste contínuo de altura visível (`DragValue`).
   - Transição suave entre projeções mantendo enquadramento.

2. **Ícones Vetoriais Nativos (`ui/icons`)**:
   - Motor de renderização vetorial em grade lógica de 24 pontos para todas as ferramentas (Select, Transform, Rotate, Scale, Primitives, Extrude, Inset, Bevel, PushPull, LoopCut, Knife, Slice, Subdivide, DrawProfile, Connect, Dissolve, Mirror, Merge, Paint, Camera, Undo, Redo).
   - Componente `tool_button` com variante compacta (40×40) e expandida (ícone + rótulo).
   - Marcador lateral de seleção, feedback de hover e borda acessível de foco de teclado.

3. **Campos de Propriedades Numéricas de Ferramentas (`ui/tool_fields`, `core/modal`)**:
   - Edição de valores exatos para operações ativas (translação, rotação, escala, extrusão, inset, bisel e push/pull).
   - Sincronização direta com a sessão modal (`ModalOp`) no viewport, permitindo prévia visual e confirmação atômica.

4. **Interface e Painéis Responsivos (`ui/lib`, `config/theme`)**:
   - Detecção dinâmica de largura de tela via `ctx.screen_rect()`.
   - Toolbar compacta automática em telas menores (< 1000px) e painel lateral com largura dinâmica.
   - Status bar inferior com contadores de vértices/faces, métricas de frame e botões acessíveis de Undo/Redo.
   - Tokens de tema dark validados com contraste acessível $\ge 4.5:1$.

## Próximos passos e pendências premium

Consultar [`docs/development/premium-interaction-plan.md`](development/premium-interaction-plan.md):
- Bevel multiaresta / arredondado com perfis variáveis.
- Inset métrico para polígonos côncavos complexos.
- Oclusão entre múltiplos assets da cena.
- Pintura de textura UV no viewport 3D integrado.
- Validação de golden path contínua com captura visual em hardware.
