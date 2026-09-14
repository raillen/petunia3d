# P3D-131 — Modal Tool Feedback System

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Modal Tool Feedback & Snapping)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Novo item · Infraestrutura compartilhada · Prioridade: P1.

</aside>

## Objetivo

Criar feedback visual reutilizável para ferramentas modais como Extrude, Inset, Bevel e transforms.

## Descriptor proposto

Um modelo neutro como `ToolFeedback`/`OverlayPrimitive` pode conter origem, direção/guia, ponto atual, delta/valor, labels/hints e estado de axis/confirm/cancel. O frontend/render adapter decide como desenhar.

## UX

Linha pontilhada/guia discreta, valor quando útil, sem cobrir a geometria. Status bar pode mostrar LMB Confirm, RMB/Esc Cancel e modifiers atuais.

## Arquitetura

Tools produzem semântica; não chamam `egui::Painter`. O sistema não deve existir apenas para Extrude.

## Dependências

P3D-029, P3D-031, P3D-032, P3D-083.

## Testes / DoD

Reutilização por múltiplas tools, cancel/confirm, DPI, ausência de alocações pesadas e feedback sincronizado com o estado real da operação.