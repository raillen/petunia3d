# P3D-102 — Core agnóstico à UI

<aside>
🧩

Estado: **auditoria obrigatória antes de refatorar** · Prioridade: P0.

</aside>

## Objetivo

Eliminar dependências de toolkit gráfico da lógica real do Petunia.

## Prova principal

Responder por código: se removêssemos `src/ui`/egui, quais partes de project, mesh, tools, commands, save/export ainda compilariam?

## Auditoria

Pesquisar `egui::`, `eframe::`, tipos encapsulados de UI, Color32/Rect/Pos2/Painter/TextureHandle vazando para core/application/tools/renderer.

## Contrato

Tipos geométricos do domínio usam tipos neutros. File dialogs, clipboard, toasts e popups pertencem a adapters/frontend.

## Dependências

P3D-103–107, P3D-122.

## Testes / DoD

Dependency graph melhora, architecture test proíbe Core→egui/eframe e operações essenciais rodam em testes headless.