# P3D-057 — Soft Brush

::: warning STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 7
- **Status Canônico**: `ACTIVE / PRÓXIMA (Materials, Texture, UV & Paint)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **implementação incerta; auditar** · Prioridade: P1.

</aside>

## Objetivo

Pincel suave convencional com size, opacity/strength e falloff simples.

## Arquitetura

Compartilhar stroke engine com Pixel Brush/Eraser; brush parameters são descriptors, não lógica de widget.

## Dependências

P3D-055, P3D-061, P3D-132.

## Testes / DoD

Pressure somente se suportado, strokes rápidos/lentos, opacity acumulada, mask e undo agrupado por stroke.