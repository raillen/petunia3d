# P3D-132 — Paint Masks / Face & Selection Isolation

<aside>
🧩

Novo item · Prioridade: P1.

</aside>

## Objetivo

Permitir pintar uma face/seleção sem vazar para regiões não desejadas.

## Modos

- isolamento temporário pela seleção atual;
- mask persistente/layer mask somente se a arquitetura de layers justificar.

## Regras

A máscara deve funcionar de forma determinística em UV seams e múltiplas ilhas. Não duplicar selection state dentro do Paint.

## Arquitetura

Stroke engine consulta mask/allowed texels/surface coverage antes de aplicar pixels. UI apenas escolhe modo/visualiza máscara.

## Dependências

P3D-055, P3D-061, P3D-019.

## Testes / DoD

Uma face, múltiplas faces, borders/seams, undo, trocar seleção e nenhuma pintura fora da área permitida.