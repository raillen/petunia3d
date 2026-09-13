# Testes de Integração de `petunia_core`

Este diretório contém os testes de integração externa para o núcleo (`petunia_core`).

## Arquivos e Cobertura

- **`command_tests.rs`**: Suíte exaustiva de testes para o `CommandDispatcher` e os comandos canônicos da aplicação:
  - `AddPrimitiveCmd` (adição de primitivas procedurais na cena e undo/redo transacional).
  - `DuplicateAssetCmd` e `DeleteAssetCmd` (gestão e ciclo de vida de assets).
  - `DeleteSelectionCmd` e `DuplicateSelectionCmd` (comportamento contextual em Edit Mode e Object Mode).
  - `SelectAllCmd`, `ClearSelectionCmd` e `InvertSelectionCmd` (operações não destrutivas de seleção).
  - `CommandDispatcher` (registro dinâmico e resolução por string de comando).
  - Execução de fluxo de modelagem completo 100% headless sem dependência de UI.
