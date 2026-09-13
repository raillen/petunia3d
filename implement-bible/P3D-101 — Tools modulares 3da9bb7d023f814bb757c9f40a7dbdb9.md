# P3D-101 — Tools modulares

<aside>
🧩

Estado: **precisa auditoria de extension cost** · Prioridade: P0.

</aside>

## Objetivo

Adicionar/remover tools com baixo impacto em módulos não relacionados.

## Auditoria

Trace Select/Move/Rotate/Scale/Extrude/Inset/Bevel/Knife: registration → activation → input → execution → preview → undo → termination. Contar arquivos/matches necessários para uma tool fictícia.

## Lifecycle

Quando aplicável: activate, begin/update, confirm/cancel, deactivate. ToolContext deve ser explícito e mínimo; evitar service locator gigante.

## Regras

Tools não conhecem keycodes nem egui::Painter. Feedback visual passa por descriptors P3D-131.

## Dependências

P3D-100, P3D-103, P3D-131.

## Testes / DoD

Extension-cost report, tool fictícia testável sem UI, lifecycle consistente e regression tests das tools representativas.