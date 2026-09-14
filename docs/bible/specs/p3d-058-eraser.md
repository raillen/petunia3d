# P3D-058 — Eraser

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

Apagar conteúdo da layer ativa de forma previsível sem destruir outras layers/canais.

## Contrato

Eraser é brush mode/operation sobre layer, respeita mask/selection isolation e alpha semantics definidas.

## Dependências

P3D-055, P3D-061, P3D-132.

## Testes / DoD

Alpha/opaque layer behavior, masks, edges, undo e nenhuma alteração em layers inativas.