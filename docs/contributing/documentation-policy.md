# Política de Documentação (Living Docs)

A documentação acompanha o código no mesmo PR — nunca depois.

## Regras

1. **CHANGELOG.md**: toda mudança funcional entra em `[Unreleased]`.
2. **Novos atalhos/parâmetros/ferramentas**: atualize `docs/tools/`, `docs/shortcuts/` e os locales.
3. **Nova superfície de UI**: adicione o nó em `docs/public/ui-map.json`
   (`cargo xtask ui-check` valida ids, arquivos, símbolos e aciclicidade).
4. **Nunca edite `docs/generated/` à mão**: regenere com `cargo run -p xtask -- docs`.
5. **i18n**: `en.toml` e `pt-BR.toml` sempre em paridade exata.
6. **Bíblia (SSOT)**: mudanças de comportamento de specs P3D atualizam `docs/bible/specs/`.

## Verificação

```bash
cargo run -p xtask -- docs-check   # arquivos essenciais + drift de gerados
cargo run -p xtask -- ui-check     # mapa da UI contra o código
```
