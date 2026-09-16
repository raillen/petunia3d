# Guia para usuários do Blender

`Blender-like` é **um dos presets oficiais de keymap** do Petunia3D — um perfil de
*familiaridade*, não o default do produto. O preset default é **Petunia**.

> **Regra do caderno:** perfis `-like` aproximam muscle memory onde isso não conflita
> com a filosofia do Petunia. Comandos que não existem no Petunia **não são inventados**
> só para completar o perfil, e diferenças materiais precisam estar documentadas.

## Equivalências mais usadas

| Operação | Blender-like no Petunia3D | Observação |
| :--- | :--- | :--- |
| Mover (grab) | `G` | trava de eixos/planos e snap |
| Rotacionar | `R` | trava de eixos e entrada em graus |
| Escalar | `S` | trava de eixos e planos |
| Extrusão | `E` | segue a normal, com opção individual |
| Inserção (inset) | `I` | — |
| Round Edge (bevel) | `Ctrl+B` | Core V1 prioriza 1 segmento |
| Corte em anel (loop cut) | `Ctrl+R` | preview e deslizamento |
| Faca (knife) | `K` | snapping a pontos/arestas |
| Vistas ortográficas | `Numpad 1 / 3 / 7` | no preset `Blender-like Notebook`, sem numpad |
| Perspectiva ↔ ortográfica | `Numpad 5` | — |
| Desfazer / refazer | `Ctrl+Z` / `Ctrl+Shift+Z` | — |

## Diferenças conscientes

- **Não há um "Edit Mode" rígido como etapa obrigatória.** O Petunia usa **domínios de
  seleção unificados e contextuais**: `Object`, `Face`, `Edge` e `Point`. Trocar de
  domínio é uma ação (`select.cycle_domain`), não uma porta de entrada para o produto.
- **Não existe boolean como paradigma exposto**: as ações de usuário são **Fuse** e
  **Cut**. Union/Difference são termos técnicos internos.
- **Vocabulário de usuário:** pontos da malha aparecem como **Point** e o chanfro como
  **Round Edge**; `Vertex`/`Bevel` são os termos técnicos correspondentes.
- Menus de um DCC genérico (Sculpt, Geometry Nodes, Compositing, Particles, Physics,
  Constraints) **não** existem aqui — estão fora do escopo do produto-base.
- Navegação e foco por teclado (`F6`/`Shift+F6` para regiões, `Tab`/`Shift+Tab` dentro
  da região) fazem parte do contrato de acessibilidade da UI, não do keymap.

## Perfis relacionados

- **Blender-like Notebook** — mesma memória motora sem numpad, com vistas em
  combinações alternativas validadas.
- **Petunia** (default) — o fluxo recomendado para quem está aprendendo o produto.

A tabela completa de binds por perfil, sempre atualizada a partir do código, está em
[Catálogo de Perfis e Atalhos](../generated/KEYBINDS.md).
