# P3D-056 — Pixel Brush

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

Brush pixel-oriented com bordas/densidade adequadas a texturas retro/low-poly.

## Arquitetura

Reutilizar stroke engine comum; diferença do Pixel Brush deve ser sampling/filter/shape, não um pipeline separado. Integrar active layer, mask e channel.

## UX

Nearest/pixel behavior previsível, size em pixels/texels quando fizer sentido, preview do cursor e sem suavização inesperada.

## Dependências

P3D-055, P3D-061, P3D-132.

## Testes / DoD

Diferentes zooms/resoluções, UV seams, masks, undo e pixel-perfect quando prometido.