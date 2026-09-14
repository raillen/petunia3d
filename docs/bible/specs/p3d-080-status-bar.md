# P3D-080 — Status Bar

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 6
- **Status Canônico**: `COMPLIANT (Scene, Assets, Outliner & Inspector)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **base existe; precisa ficar útil e contextual** · Prioridade: P2.

</aside>

## Objetivo

Mostrar informações úteis sem virar painel de debug.

## Conteúdo recomendado

Esquerda: projeto/save/autosave e hints da tool ativa. Direita: tris/verts/objects quando útil, quality/performance profile e versão. Métricas de developer muito específicas podem ficar em Developer Mode.

## Integração

P3D-002 para autosave; P3D-131 para modal hints; TextId/keymap para instruções.

## Testes / DoD

Estados save/autosave/failure, tool hints, tradução, janela estreita e ausência de layout jitter.