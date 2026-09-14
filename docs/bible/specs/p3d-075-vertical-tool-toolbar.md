# P3D-075 — Vertical Tool Toolbar

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 4
- **Status Canônico**: `COMPLIANT (Viewport, Navigation & Reference Workflow)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **parcial; iconografia/semântica precisa refino** · Prioridade: P1.

</aside>

## Objetivo

Toolbar vertical para tools persistentes de interação, não catálogo de todos os commands.

## Conteúdo típico

Select/Cursor, Move, Rotate, Scale, Universal Transform, Annotate/Measure e outras tools realmente persistentes.

## Regras

Commands como Delete/Add Cube não ocupam a toolbar apenas por disponibilidade de espaço. Tool ativa tem estado inequívoco, hitbox adequada e tooltip com keybind atual.

## Dependências

P3D-088, P3D-101, P3D-114.

## Testes / DoD

Icon packs diferentes, DPI, seleção de tool, disabled state e ausência de handlers duplicados.