# P3D-134 — Paint Effect Stack

<aside>
🧩

Novo item · V1.x · Prioridade: P2.

</aside>

## Objetivo

Efeitos não destrutivos simples sobre layers/texturas.

## Ordem recomendada

1. Pixelate.
2. Posterize/quantize.
3. Toon-like texture filter se realmente útil.

Aquarela/lápis/caneta só entram após protótipos de qualidade aceitável e custo conhecido.

## Regra conceitual

Diferenciar **efeito sobre textura** de **shader toon em tempo real** (P3D-140).

## Arquitetura

Effect descriptors serializáveis, evaluation cacheable e sem filtro pesado refeito por frame sem necessidade.

## Dependências

P3D-055, P3D-061.

## Testes / DoD

Reorder effects, parameters, disable/enable, serialization, invalid values e performance.