# 01 — Protocolo de Especificação P3D e Prompt para LLM

# Como usar cada página P3D

A página da funcionalidade é o prompt canônico. O agente deve ler esta página, a Constituição, as dependências listadas e as páginas normativas relevantes do Livro Vivo.

# Ordem obrigatória antes do código

1. Auditar implementação atual no repositório e executar o comportamento relevante sempre que possível.
2. Decompor a especificação da P3D em requisitos verificáveis.
3. Para cada requisito, comparar profundamente a implementação atual com o contrato do caderno e produzir uma **Implementation-vs-Spec Gap Matrix**.
4. Classificar cada requisito como `COMPLIANT`, `PARTIALLY_COMPLIANT`, `FUNCTIONAL_BUT_DIFFERENT`, `RUDIMENTARY`, `STUB`, `BROKEN`, `DUPLICATED`, `MISSING` ou `OBSOLETE`.
5. Preservar tudo que já estiver `COMPLIANT`; não reimplementar por preferência arquitetural ou estética.
6. Para `PARTIALLY_COMPLIANT`, corrigir somente as lacunas comprovadas. Para `FUNCTIONAL_BUT_DIFFERENT`, justificar por evidência qualquer mudança antes de alterar código.
7. Mapear dependências, estado, commands, tools, undo, renderer, persistência, UI, testes e consumidores reais dos módulos tocados.
8. Registrar riscos, migração, rollback e comportamento que deve permanecer intacto antes de mudanças destrutivas.
9. Implementar incrementalmente, escolhendo o menor delta que leve o código ao contrato desejado.

# Template mínimo de cada especificação

- Objetivo e problema do usuário.
- Escopo e não objetivos.
- Estado atual esperado e pontos a auditar.
- Comportamento funcional e UX.
- Modelo de dados/estado e boundaries Core/Application/UI.
- Commands, TextIds, IconIds, ThemeTokens e keymaps quando aplicáveis.
- Undo/Redo, persistência, erros, performance e segurança.
- Testes unitários, integração, UI, regressão e performance.
- Documentação, screenshots e changelog.
- Plano de implementação, Gauntlet Loop, critérios de aceitação e Definition of Done.

# Gate `SPEC READY`

Antes de alterar código de uma P3D P0/P1, confirmar que a especificação possui, conforme aplicável: objetivo; escopo/non-goals; auditoria do código; dependências; modelo de estado/dados; boundary Core/Application/UI/Renderer; Commands/TextIds/IconIds/ThemeTokens/keymap; undo/persistência; invalid states/erros; performance; segurança; testes; docs; acceptance criteria; riscos de migration/rollback. Se faltar informação, aprofundar a página primeiro.

# Gate `IMPLEMENTATION READY`

Além de SPEC READY: baseline tests executados, bloqueadores da wave conhecidos, arquivos/módulos reais mapeados, architecture checks relevantes definidos e nenhuma decisão estrutural crítica deixada implícita.

# Regra de prompt

Quando solicitado a implementar `P3D-XXX`, não inventar requisitos fora da página sem justificar. Decisões recentes prevalecem. **A primeira tarefa não é implementar: é reconciliar especificação e código.** Se o código já satisfaz um requisito, marque-o como compliant e preserve-o. Se divergir, documente a divergência, o impacto e a menor correção necessária. Reimplementação completa só é aceitável quando a auditoria demonstrar que o desenho atual impede satisfazer o contrato incrementalmente ou possui dívida estrutural maior que o custo/risco da substituição. Usar `16 — Master Prompt de Implementação por Gauntlet Waves` para tarefas multi-feature.