# P3D-005 — Navegação 3D

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 4
- **Status Canônico**: `COMPLIANT (Viewport, Navigation & Reference Workflow)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado inicial: **funcional, precisa refinamento** · Prioridade: P1.

</aside>

## Objetivo

Orbit, pan e zoom previsíveis para mouse e notebook/touchpad, com melhor orientação espacial.

## Decisões de produto

- Criar perfil de navegação/keymap para notebook/touchpad sem depender de mouse com botão central.
- Adicionar Navigation HUD opcional junto ao gizmo. Preferir **vista nominal + yaw/pitch** ou representação igualmente legível a dois números 0–360 sem contexto.
- HUD pode ser desativado.

## Auditoria

Verificar bindings atuais, sensitivity, focus/capture, delta do mouse/trackpad e relação com camera state.

## Contrato

Input físico é normalizado antes de chegar ao controller de câmera; keymaps/gestos não pertencem ao core. Navegação não suja o projeto nem cria undo.

## Dependências

P3D-007, P3D-090, P3D-094.

## Testes / DoD

Mouse e touchpad, perspective/ortho, focus loss, DPI e diferentes velocidades; UI sem overlap e documentação dos controles atualizada.