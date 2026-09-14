# P3D-088 — Petunia Custom Icons

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 3
- **Status Canônico**: `COMPLIANT (UI Infrastructure, Customization & Input)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Necessário para conceitos 3D especializados · Prioridade: P1.

</aside>

## Objetivo

Ícones first-party para Vertex/Edge/Face, Extrude, Inset, Bevel, Loop Cut, shading, pivot, proportional editing e outros conceitos sem candidato adequado.

## Referências

Tabler/Iconoir/Phosphor/Lucide são preferidos quando semântica é boa. `ui.blender.org/icons` pode orientar linguagem visual; qualquer asset efetivamente copiado exige verificação de licença/origem.

## Contrato

Mesmo `IconId` independente do pack. Custom icon deve respeitar tamanho óptico/stroke/hitbox do sistema.

## Testes / DoD

Icon Gallery, coverage dos controls principais, SVG válido e ausência de placeholders quadrado/círculo/emoji.