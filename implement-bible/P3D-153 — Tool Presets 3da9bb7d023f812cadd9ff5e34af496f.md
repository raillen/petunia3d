# P3D-153 — Tool Presets

## Objetivo

Salvar e reaplicar parâmetros de ferramentas sem salvar geometria.

## Exemplos

Bevel Tiny PS1, Brush Pixel Hard, Primitive Low-Sides.

## Arquitetura

Presets referenciam ToolId/Command metadata e parâmetros serializáveis; UI apenas edita/seleciona presets.