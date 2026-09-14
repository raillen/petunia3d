# P3D-039 — Proportional Editing

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Selection Domain, Transform & Modeling Core)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **precisa auditoria** · Prioridade: P1.

</aside>

## Objetivo

Transformar seleção influenciando componentes próximos por radius/falloff, mantendo comportamento previsível.

## Auditoria

Verificar se botão/estado existente tem efeito real e quais falloffs estão implementados. Não expor uma galeria de falloff sem suporte.

## Arquitetura

Influence weights são calculados no core/editor a partir da topologia/distância; UI ajusta radius/falloff via input normalizado.

## Dependências

P3D-017–019, P3D-021–023, P3D-090.

## Testes / DoD

Radius mínimo/máximo, boundaries, falloff suportado, cancel/undo e performance em meshes maiores.