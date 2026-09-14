# P3D-122 — Architecture Checks

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 0 / Wave 1
- **Status Canônico**: `COMPLIANT (Architecture Checks Automatizados)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Quality gate arquitetural · Prioridade: P0.

</aside>

## Objetivo

Transformar regras como `core !→ egui`, `tools !→ keycodes` e `domain !→ localized strings` em checks automatizados.

## Implementação possível

Cargo dependency graph, scripts/ripgrep direcionados, crate boundaries e compile tests. Preferir regra robusta e simples a ferramenta pesada.

## Dependências

P3D-100–109.

## Testes / DoD

Introduzir violação de fixture/teste e confirmar falha. Mensagem explica boundary violada e como corrigir.