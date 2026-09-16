# P3D-104 — Headless readiness

<aside>
🧩

Estado: **capacidade precisa ser medida** · Prioridade: P0.

</aside>

## Objetivo

Permitir testes/automação de operações centrais sem criar janela egui.

## Cenário mínimo de prova

`new project → add cube → select/transform or edit → undo/redo → save → load → export` sem frontend gráfico.

## Estratégia

Não criar CLI completa inicialmente; primeiro disponibilizar Application API/test harness. Uma CLI mínima pode ser prova posterior.

## Dependências

P3D-100–103, P3D-068–072.

## Testes / DoD

CI executa fluxos headless determinísticos e nenhum bootstrap de GPU/UI é necessário para lógica que não renderiza.