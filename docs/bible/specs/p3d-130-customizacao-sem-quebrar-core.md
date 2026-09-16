# P3D-130 — Customização sem quebrar Core

<aside>
🧩

**Invariante arquitetural** · Prioridade: RULE.

</aside>

## Decisão

Theme, icons, translations, keymaps e layouts são camadas substituíveis; não fazem parte da lógica de modelagem.

## Regras

Core não conhece ThemeToken/IconId/TextId/keycodes salvo metadata de presentation em boundary apropriada. Packs customizados são dados declarativos e podem falhar sem derrubar a aplicação.

## Teste conceitual

Trocar theme/icon/language/keymap não altera project data nem comportamento semântico dos Commands.

## Dependências

P3D-084–109, P3D-122.