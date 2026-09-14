# P3D-126 — Performance para PCs modestos

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 0
- **Status Canônico**: `COMPLIANT (Performance Baseline & PC Modesto)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

**Invariante permanente**, não feature isolada · Prioridade: RULE.

</aside>

## Princípio

Petunia3D deve permanecer responsivo em hardware modesto; polish visual ou abstração arquitetural não justificam regressão grande sem benefício comprovado.

## Budget mental

Priorizar viewport/frame time, memória, startup, asset loading, operações de mesh e caches. Não executar parse SVG, image decode, thumbnail generation, filesystem scans ou serialization pesada por frame.

## Processo

Toda feature nova avalia custo, cria baseline quando relevante e mede antes/depois quando otimização é proposta. Preferir simplicidade algorítmica e lazy/background work seguro.

## Settings

Quality presets podem existir quando há trade-off real; defaults devem funcionar bem em máquinas comuns.

## DoD

Regressões relevantes são detectadas por benchmarks/profiling e documentadas; nenhuma feature crítica depende de GPU/CPU topo de linha.