# P3D-109 — Events / State synchronization

<aside>
🧩

Estado: **precisa auditoria; evitar EventBus global caótico** · Prioridade: P0.

</aside>

## Objetivo

Sincronizar Scene, Selection, Tools, Project, Undo e frontends com direção e ownership explícitos.

## Auditoria

Mapear callbacks, observers, EventBus, Arc/Mutex/RefCell, deep state access e pontos onde UI precisa pollar estruturas internas.

## Direção

Commands mutam; Queries expõem state/views; eventos/invalidation notificam mudanças significativas (`SelectionChanged`, `ProjectSaved`, `AssetUpdated` etc.) sem broadcasting de tudo para todos.

## Dependências

P3D-100–108.

## Testes / DoD

Sem loops/event storms, ordem determinística quando necessária, unsubscribe/lifecycle corretos e frontend recebe mudanças sem possuir estado duplicado.