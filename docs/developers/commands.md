# Sistema de Comandos & Transações

Para evitar estados corrompidos ou falhas silenciosas de desfecho, toda mutação de geometria no Petunia3D adota o **Padrão de Comandos Transacionais**.

---

## O Ciclo de Vida Modal

```mermaid
sequenceDiagram
    participant User as Usuário
    participant UI as modal_viewport
    participant Core as AppState (ModalOp)
    participant Undo as UndoStack

    User->>UI: Pressiona 'G' (Move)
    UI->>Core: begin_modal(ModalKind::Move)
    Core->>Core: Clona snapshot original da malha
    loop Prévia Interativa em Tempo Real
        User->>UI: Move o mouse ou digita números
        UI->>Core: update_modal(translation, value)
        Core->>UI: Atualiza vértices de prévia no viewport
    end
    alt Confirmar (Enter ou LMB)
        User->>UI: Confirmação
        UI->>Core: commit_modal()
        Core->>Undo: checkpoint("Mover", original)
        Core->>Core: locked_axes resetam para [false; 3]
    else Cancelar (Escape ou RMB)
        User->>UI: Cancelamento
        UI->>Core: cancel_modal()
        Core->>Core: Restaura snapshot original instantaneamente
        Note over Undo: Nenhuma entrada adicionada ao histórico
    end
```

## Benefícios
- **Cancelamento com Custo Zero**: Se o usuário cancelar no meio de uma transformação complexa, nenhum checkpoint inútil é registrado no histórico;
- **Determinismo Absoluto**: O histórico de `Ctrl+Z` contém apenas operações finais intencionais confirmadas pelo usuário.
