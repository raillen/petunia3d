# Espaços de Trabalho (Workspaces)

Os **Workspaces** são layouts pré-configurados para tarefas específicas. Eles são
trocados pelas *pills* no centro da top bar, a qualquer momento.

A **UI Baseline Final V1** congela **três** workspaces
([capítulo 36](../bible/foundations/36-ui-baseline-temas-plugin-panels.md)). Workspace
não implementado **não aparece** como pill desabilitada.

| Workspace | Propósito principal | Regra de layout |
| :--- | :--- | :--- |
| **[MODEL](./modeling)** | Modelagem: viewport dominante + Parts + Context + Asset Library | default da aplicação |
| **[PAINT](./paint)** | Pintura e material: viewport 3D dominante, Context com brush/material/texture | editor 2D de textura é painel opcional, fechado por padrão |
| **[UV](./uv)** | Split UV 2D + viewport 3D com seleção sincronizada | default aproximado `55/45`, redimensionável |

## O que não é workspace V1

- **EXPORT** não é um workspace: exportação e checagem de game-readiness acontecem
  por comandos e painéis, não por uma pílula dedicada.
- **Animation / Animate** pertence a pós-V1 (P3D-066 / P3D-135–139), não à baseline V1.
  Veja [Animation (pós-V1)](./animation).
- **Sculpt**, **Shading**, **Compositing** e **Geometry Nodes** estão explicitamente
  fora do produto-base (`Out of Scope`).

## Regras estruturais

- O shell é `Parts` à esquerda, `Context` à direita e `Asset Library` embaixo, em
  todas as combinações V1; o centro é o viewport/editor do workspace.
- Estado de layout é persistido **por workspace**.
- Não há docking irrestrito na V1: resize só existe em divisores autorizados e
  double-click restaura a medida default.
- Plugin panels entram apenas em extension slots controlados (`left`, `right`, `bottom`).
