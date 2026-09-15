# Criando um Pacote de Ícones

1. Leia [Pacotes de Ícones](../customization/icons.md) para entender a cadeia de fallback (pacote → Phosphor → losango vetorial).
2. Monte seu pack seguindo o layout dos existentes; todo ícone precisa resolver para um glifo real — controles nunca podem ficar invisíveis.
3. Registre via manifest para descoberta (o cache atualiza pelo file watcher).
4. Ative em **Settings → Icons** e percorra toolbar, menus e viewport bar conferindo cada glifo.

Regra de ouro: pacote muda o chrome, nunca o significado das ferramentas.
