# 00 — Constituição do Projeto e Diretiva para Agentes

<aside>
📜

Esta página é normativa. Toda LLM/code agent deve lê-la antes de implementar qualquer funcionalidade P3D.

</aside>

# Missão do Petunia3D

Petunia3D é um editor/modelador 3D focado em criação de assets low-poly com baixa carga cognitiva, interação direta, workflows previsíveis e desempenho adequado a computadores modestos. O produto não deve crescer por acumulação indiscriminada de recursos nem tentar substituir Blender/Maya como DCC genérico.

# Ordem de decisão

**facilidade de uso → previsibilidade → modularidade → robustez → performance → extensibilidade → polish visual**.

# Regras absolutas

- Core e lógica de domínio não dependem de egui, eframe, widgets, cores, ícones ou atalhos físicos.
- UI é frontend substituível; egui é implementação atual.
- Ações semânticas passam por `CommandId` quando aplicável.
- Inputs físicos são resolvidos por keymaps; tools não conhecem teclas como regra de negócio.
- Strings visíveis usam `TextId`; iconografia usa `IconId`; aparência usa `ThemeToken`.
- Tools, commands, importers, exporters e módulos devem possuir fronteiras claras e baixo custo de extensão.
- Preferir composição de poucas operações previsíveis a dezenas de comandos sobrepostos.
- Não implementar remesh como parte do core principal; não transformar Petunia em compositor de cenas complexo; não exigir login.
- Performance para máquinas modestas é requisito transversal.

# Verdade da implementação

Documentação descreve intenção. **Código, testes e comportamento executável são a evidência do estado real.** Antes de corrigir ou implementar, auditar o código para evitar duplicação e preservar comportamento correto.

# Regra obrigatória de reconciliação entre caderno e código

A Implementation Bible **não deve ser aplicada cegamente como se o projeto estivesse vazio**. Antes de implementar qualquer requisito, o agente deve comparar profundamente o que o caderno exige com o que já existe no repositório e no comportamento executável.

Para cada requisito relevante, classificar a implementação atual como: `COMPLIANT`, `PARTIALLY_COMPLIANT`, `FUNCTIONAL_BUT_DIFFERENT`, `RUDIMENTARY`, `STUB`, `BROKEN`, `DUPLICATED`, `MISSING` ou `OBSOLETE`.

- `COMPLIANT`: preservar; não reimplementar.
- `PARTIALLY_COMPLIANT`: implementar apenas as lacunas comprovadas.
- `FUNCTIONAL_BUT_DIFFERENT`: comparar comportamento, arquitetura, UX, performance e manutenção; adaptar somente se a divergência violar uma decisão normativa ou houver ganho concreto.
- `RUDIMENTARY`/`STUB`/`BROKEN`: reutilizar partes válidas quando possível e corrigir incrementalmente.
- `DUPLICATED`: escolher/consolidar o caminho canônico sem manter duas implementações paralelas.
- `MISSING`: implementar conforme especificação.
- `OBSOLETE`: remover somente depois de provar ausência de consumidores e possuir cobertura/rollback adequados.

O agente deve produzir uma **Implementation-vs-Spec Gap Matrix** antes de mudanças significativas. A regra é: **preservar o que já satisfaz o contrato; modificar somente a diferença necessária para chegar ao estado exigido pelo caderno**. Rewrites completos exigem evidência de que a arquitetura atual impede a correção incremental.

# Regra de completude

Uma feature só termina quando comportamento, testes, arquitetura, UI, tokens, documentação, screenshots e changelog aplicáveis estiverem sincronizados.