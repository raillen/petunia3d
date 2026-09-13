# P3D-073 — Workspace System

<aside>
🧩

Estado: **parcial; precisa adaptação contextual** · Prioridade: P1.

</aside>

## Objetivo

Model, Paint, UV e Animation como composições contextuais de painéis/tools, sem duplicar estado do projeto.

## Regras

- workspace troca layout/contexto, não cria cópia de scene/material/selection;
- painéis podem ser personalizados, redimensionados e ocultados;
- layout inicial continua simples e previsível;
- Export não vira workspace permanente sem uma necessidade real.

## Dependências

P3D-078, P3D-079, P3D-083.

## Testes / DoD

Troca repetida sem perder seleção/estado, layout persistido como UI settings e funcionamento em 1366×768.