# P3D-051 — Albedo / Diffuse

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