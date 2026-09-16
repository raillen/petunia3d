# P3D-151 — Batch Asset Processor

## Objetivo

Executar operações seguras em conjuntos de assets pela Project Model Library.

## Operações candidatas

Rename, validate, generate thumbnails, resize textures, change material profile, generate collisions e export.

## Arquitetura

Executa services/commands existentes; não reimplementa lógica de cada feature. Deve fornecer progress, cancelamento quando seguro e relatório final.