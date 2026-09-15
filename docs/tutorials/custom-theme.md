# Criando um Tema Customizado

Temas vivem em TOML (veja `assets/themes/` e [Sistema de Temas TOML](../customization/themes.md)).

1. Copie `petunia-dark` para um novo nome (ex. `meu-tema`).
2. Ajuste cores, superfícies e tipografia no TOML.
3. Selecione em **Settings → Appearance** — aplica sem reiniciar.
4. Sem cores hardcoded na UI: use sempre os tokens (`crates/ui/src/tokens.rs`).

Teste nos dois modos (claro/escuro) e com zoom de fonte antes de publicar.
