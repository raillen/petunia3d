# P3D-079 — Layout responsivo desktop

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 3
- **Status Canônico**: `COMPLIANT (UI Infrastructure, Customization & Input)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **overlap/compressão ainda possível** · Prioridade: P0.

</aside>

## Objetivo

Interface útil de 1366×768 a telas grandes e diferentes DPI sem sobreposição de controls.

## Estratégia

1. reduzir gaps não essenciais;
2. trocar labels longas por icon+tooltip onde semântica é clara;
3. agrupar controls secundários em overflow;
4. collapse side panels;
5. nunca reduzir tipografia a ponto de ficar ilegível.

## Dependências

P3D-074, P3D-076–078.

## Testes / DoD

1366×768, 1600×900, 1920×1080 e 100/125/150/200% DPI; screenshots de regressão e zero overlap.