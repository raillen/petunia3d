# P3D-106 — Frontends intercambiáveis

<aside>
🧩

Estado: **objetivo arquitetural, não trocar egui agora** · Prioridade: P0.

</aside>

## Objetivo

Tornar egui um frontend substituível sem reescrever domínio/editor.

## Prova incremental

Primeiro P3D-104 headless; depois um frontend mínimo alternativo/CLI de prova pode consumir Commands/Queries. Não implementar Qt/C#/Go só para demonstrar abstração.

## Contrato

Presentation metadata pode conter TextId/IconId/DocsTopic, mas core algorithms não. App API expõe state/query sem dar acesso irrestrito a internals.

## Dependências

P3D-102–105, P3D-107.

## Testes / DoD

Boundary documentada, egui-specific code localizado e ao menos um consumer não-egui consegue executar use cases sem fork da lógica.