# Operações Avançadas (Mesh)

Ferramentas de malha que não têm painel próprio no menu principal. Todas operam sobre a seleção atual no Modo de Edição e participam do **Undo/Redo** (`Ctrl+Z` / `Ctrl+Shift+Z`).

## Espelhar (Mirror) — `Ctrl+M`

Duplica a seleção espelhada no eixo escolhido (X/Y/Z) com solda opcional no plano (`Weld`). Previsível: eixo + tolerância, sem surpresas.

## Fundir (Merge) — `M`

- **Merge center**: funde os vértices selecionados no ponto central.
- **Merge by distance**: solda vértices duplicados dentro de uma tolerância (limpeza após Mirror e Slice).

## Simetrizar (Symmetrize) — `Alt+M`

Copia um lado para o outro através do eixo (+ para − ou o inverso) e solda a costura. Sem lado-fonte, não faz nada (nunca apaga a malha).

## Revolução (Revolve)

Gira arestas conectadas ao redor de um eixo (X/Y/Z) com segmentos (3–64) e ângulo (5–360°). Ângulos abaixo de 360° deixam o perfil aberto. Para perfis desenhados à mão, veja [Draw Profile](../manual/modeling.md).

## Subdividir e Triangular

Subdivide a seleção (loop cut) ou triangulariza quads/n-gons. Use antes de exportar quando o destino exigir só triângulos.
