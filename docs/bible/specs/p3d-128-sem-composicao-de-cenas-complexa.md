# P3D-128 — Sem composição de cenas complexa

<aside>
🧩

**Invariante de escopo** · Prioridade: RULE.

</aside>

## Decisão

Petunia3D é editor de assets, não substituto do Blender como scene compositor/DCC completo.

## Permitido

Múltiplos objetos necessários à criação do asset, referências, materiais, rigs/animações simples e organização mínima do projeto.

## Evitar

World/lighting/render pipeline cinematográfico complexo, scene sequencing, compositor, simulações extensas e sistemas que desviam o fluxo principal.

## Critério

Se uma feature é melhor tratada por game engine, DCC completo ou módulo separado, preferir integração/export/plugin em vez de bloating do core.