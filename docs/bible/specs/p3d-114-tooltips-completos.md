# P3D-114 — Tooltips completos

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 3
- **Status Canônico**: `COMPLIANT (UI Infrastructure, Customization & Input)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **cobertura precisa auditoria** · Prioridade: P1.

</aside>

## Objetivo

Todo control icon-only/ambíguo possui tooltip consistente com nome, descrição curta e keybind atual quando aplicável.

## Arquitetura

Tooltip deriva de TextId + CommandMetadata + KeymapResolver. Nenhum `Move (G)` hardcoded.

## UX

Delay razoável, não cobrir alvo desnecessariamente, disabled control explica pré-condição quando útil.

## Dependências

P3D-089–090, P3D-100, P3D-115.

## Testes / DoD

Icon packs/idiomas/keymaps diferentes, disabled states, keyboard focus e cobertura dos controls principais.