# P3D-051 — Albedo / Diffuse

::: warning STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 7
- **Status Canônico**: `ACTIVE / PRÓXIMA (Materials, Texture, UV & Paint)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **precisa auditoria; depende do Material System** · Prioridade: P1.

</aside>

## Objetivo

Canal principal de cor/textura do material, suportando cor constante, textura importada e resultado de Paint.

## Contrato

Uma única representação de Albedo é consumida pelo renderer/exporter; UI apenas escolhe source/parâmetros. Color-only atual pode ser mantido como fallback simples.

## Dependências

P3D-050, P3D-055, P3D-062.

## Testes / DoD

Cor sem textura, textura, paint result, missing resource, save/load e Material Preview.