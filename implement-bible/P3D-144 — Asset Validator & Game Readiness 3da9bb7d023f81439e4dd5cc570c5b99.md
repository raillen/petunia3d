# P3D-144 — Asset Validator & Game Readiness

## Objetivo

Validar um asset antes de export/entrega, usando regras configuráveis e perfis de budget.

## Escopo inicial

- manifold/topologia básica;
- pivot/origin;
- material/UV presentes quando necessários;
- contagem de triângulos/vértices;
- textura e dimensões;
- collision/LOD/socket quando exigidos pelo profile;
- naming rules.

## Perfis

Generic, PS1-like, N64-like, PS2-like, Mobile e Custom. Esses nomes representam budgets configuráveis, não emulação histórica rígida.

## UX

Relatório com `pass`, `warning`, `error`, ação de localizar problema e possibilidade de validar seleção ou biblioteca em lote.

## Arquitetura

Validator opera em dados neutros e não em widgets. Regras devem ser registráveis e reutilizáveis por exportadores, batch processor e futura engine bridge.