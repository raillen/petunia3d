# Workspace: MODEL

O workspace **MODEL** é o ambiente padrão do Petunia3D e maximiza o viewport 3D,
mantendo o contexto de seleção e a biblioteca de assets sempre acessíveis.

## Regiões do shell

| Região | Elemento | Baseline |
| :--- | :--- | :--- |
| Esquerda | **Parts** (hierarquia da cena) | `248 px` default · `200–400` · recolhível |
| Centro | **Viewport** + toolbar contextual | preserva ~`480 × 360` logical px |
| Direita | **Context** (selection/tool-centric) | `288 px` default · `240–440` |
| Embaixo | **Asset Library** | `176 px` default · `120–360` · collapse |
| Topo | **Top bar** | `40 px` (workspace pills ao centro) |

A toolbar contextual vive **dentro** do viewport e muda conforme o domínio de seleção
e a ferramenta ativa. Não existe uma toolbar vertical permanente de 20+ ícones.

## Seleção e contexto

- Domínios: `Object` (malhas inteiras) e componentes `Face` / `Edge` / `Point`.
- `Tab` alterna entre seleção de objeto e o último domínio de componente
  (`select.cycle_domain`) no preset Petunia.
- As ferramentas de malha aparecem quando o domínio de componente está ativo:
  extrude, inset, round edge (bevel), loop cut, knife, push/pull, slice, subdivide e
  draw profile.
- O painel **Context** mostra os parâmetros da ferramenta/operacão em foco; campos
  numéricos aceitam arrasto horizontal.

## Modos de visualização

Modos-base: `Wireframe` · `Solid` · `Textured` · `Silhouette`. Overlays (grid, eixos,
wireframe-on-shaded, contagem de geometria) são **composição sobre** o modo-base, não
modos adicionais. Flat shading é o default; Smooth é opção secundária.

## Operações de contexto

- **Transformações**: move, rotate, scale — modais, com trava de eixos/planos e snap.
- **Modelagem**: extrude, push/pull, inset, round edge (1 segmento no Core V1),
  loop cut, knife, slice, subdivide, mirror, weld/merge.
- **Combine**: `Keep Parts`, `Join`, `Fuse`, `Connect`. Booleans não são paradigma
  exposto: o usuário vê **Fuse** e **Cut**.
- Toda operação é **transacional**: preview, validação e Undo fazem parte do contrato.

## Undo, persistência e performance

- Undo/Redo usa snapshots das regiões/objetos afetados; um stroke de pintura é uma
  transaction.
- O viewport é **event-driven** em idle: sem interação, não há renderização contínua.
- A performance alvo é de máquinas modestas (requisito transversal do projeto).
