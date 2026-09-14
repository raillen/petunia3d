# P3D-136 — Rig Presets — Humanoid, Quadruped e Multi-Leg

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 10 (Pós-GA)
- **Status Canônico**: `ROADMAP PÓS-GA (Animation & Rigging)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Novo item · Prioridade: P3.

</aside>

## Objetivo

Templates de rig reutilizáveis adequados a humanoides, quadrúpedes e criaturas multi-leg como aranhas/escorpiões.

## Princípio

São **templates de dados**, não engines separadas por espécie. Devem definir naming/orientation/hierarchy recomendados e permanecer totalmente editáveis.

## Escopo incremental

1. Humanoid simples.
2. Quadruped simples.
3. Multi-leg genérico com preset demonstrativo para aranha/escorpião.

## Dependências

P3D-135.

## Testes / DoD

Instantiate preset, editar proportions/bones, serialize, aplicar clip compatível quando existir e nenhuma lógica humanoide hardcoded no core.