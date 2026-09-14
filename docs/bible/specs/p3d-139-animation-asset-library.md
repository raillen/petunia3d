# P3D-139 — Animation Asset Library

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 10 (Pós-GA)
- **Status Canônico**: `ROADMAP PÓS-GA (Animation & Rigging)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Novo item · Biblioteca reutilizável de clips · Prioridade: P3.

</aside>

## Objetivo

Salvar/importar clips uma vez e reutilizá-los sem retornar continuamente a sites externos.

## UX

Grid/list, search, tags, preview, rig compatibility e Apply/Retarget. Reutilizar padrões da Project Model Library sem duplicar backend desnecessariamente.

## Modelo

AnimationAssetId, source/format, clip metadata, duration, tags, compatibility/retarget profile e preview cache.

## Dependências

P3D-003, P3D-138.

## Testes / DoD

Import, tag/search, missing file/relink, preview, apply compatível/incompatível e metadata persistida.