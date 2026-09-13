# 05 — Política de Documentação, Screenshots e Changelog

# Documentação é parte da feature

Toda mudança user-facing deve verificar manual, referência, screenshots, changelog e website.

# Website vivo

A documentação pública deve ser navegável, pesquisável, versionada com o código e publicada automaticamente quando infraestrutura existir.

# Screenshots

Usar screenshots reais da implementação, nunca mockups como prova de funcionalidade. Atualizar quando layout/controle/workspace muda.

# Referências geradas

Commands, keybinds, IconIds, TextIds e ThemeTokens devem ser gerados automaticamente quando possível; CI deve detectar drift.

# Changelog

Registrar mudanças relevantes ao usuário, não despejar commits técnicos. Breaking changes de schema, project format, plugin API ou keymaps exigem migration notes.

# Regra para agentes

Nenhuma tarefa é concluída com documentação conhecida como obsoleta.