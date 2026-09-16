# P3D-007 — Navigation Gizmo

<aside>
🧩

Estado inicial: **implementado e funcional; refino visual** · Prioridade: P2.

</aside>

## Objetivo

Manter o gizmo XYZ como acesso rápido à orientação e vistas, refinando aparência, hitboxes e feedback.

## Auditoria

Não reimplementar a matemática já correta. Verificar click targets, DPI, hover, tooltips, alinhamento dos botões auxiliares e sincronização com camera state.

## Contrato visual

Usar componentes/tokens Petunia e IconId; controles auxiliares devem formar um grupo coerente, não círculos desconectados. Integrar com o Navigation HUD de P3D-005 sem poluir o viewport.

## Dependências

P3D-005, P3D-006, P3D-084, P3D-088.

## Testes / DoD

Todas as faces/eixos clicáveis continuam funcionais, estados são claros em diferentes DPI/resoluções e screenshots/documentação refletem o acabamento final.