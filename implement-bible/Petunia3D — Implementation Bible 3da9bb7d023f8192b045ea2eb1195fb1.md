# Petunia3D — Implementation Bible

<aside>
📘

Este caderno é a especificação canônica de implementação das funcionalidades P3D. Ele complementa o Livro Vivo: o Livro Vivo registra visão, decisões e arquitetura; a Implementation Bible detalha, por funcionalidade, o comportamento esperado, UX, arquitetura, testes, documentação e critérios de aceitação.

</aside>

# Constituição de implementação

Toda implementação deve preservar a filosofia do Petunia3D.

- O Petunia3D é um editor/modelador focado em criação simples e eficiente de assets 3D low-poly; não deve crescer indiscriminadamente em complexidade.
- Prioridade permanente: **facilidade de uso → previsibilidade → arquitetura modular → robustez → performance suficiente para PCs modestos → extensibilidade → polish visual**.
- O core deve permanecer independente da interface gráfica; egui é um frontend atual, não a identidade do núcleo.
- Ferramentas e funções devem ser adicionáveis, removíveis e substituíveis com baixo impacto em módulos não relacionados.
- `CommandId` representa ações semânticas; keybinds não podem ser codificados diretamente dentro de tools.
- Strings visíveis usam `TextId`; ícones usam `IconId`; cores e métricas visuais usam `ThemeToken`.
- Toda feature nova deve avaliar `CommandId → TextId → IconId → ThemeToken → Keymap → Tests → Docs → Changelog` quando aplicável.
- Documentação é parte da implementação; screenshots e changelog devem acompanhar mudanças visuais/funcionais.
- A documentação descreve intenção; **código, testes e comportamento real são a evidência da implementação atual**.
- Antes de implementar uma feature, auditar o código existente para evitar duplicação e preservar o que já está correto.
- Nenhuma feature é considerada completa apenas porque compila.

# Regra para agentes de IA

Antes de alterar código para qualquer P3D:

1. Ler esta Constituição.
2. Ler a especificação P3D correspondente e suas dependências.
3. Ler as páginas relevantes do Livro Vivo, principalmente arquitetura, UI/UX, design system, tokens, commands/keymaps, testes e documentação.
4. Auditar a implementação real antes de modificar.
5. Implementar incrementalmente, preservando comportamento já correto.
6. Executar testes e validações após cada etapa significativa.
7. Atualizar tokens, documentação, screenshots e changelog quando aplicável.
8. Encerrar apenas quando os critérios de aceitação da página P3D forem satisfeitos por evidência.

# Status e autoridade

Cada página P3D separa **Specification Status** de **Implementation Status**. A especificação pode estar consolidada mesmo quando a implementação estiver incompleta. Em conflitos, decisões mais recentes e explicitamente normativas do Livro Vivo prevalecem; a implementação real deve ser auditada e divergências registradas, nunca ocultadas.

# Escopo detalhado da Implementation Bible

O caderno agora cobre **P3D-001 a P3D-155**, incluindo as funcionalidades derivadas das observações do backlog e o roadmap pós-GA aprovado. As especificações estão organizadas por epics A–O. Cada página P3D funciona como prompt canônico para a LLM: ela deve ser lida junto da Constituição, do protocolo de implementação e das dependências indicadas.

P3D-016 foi preservada como registro histórico, mas sua UX pública foi **absorvida por P3D-015**, que unifica Object / Vertex / Edge / Face. Os itens P3D-131–143 formalizam feedback modal, Paint avançado, rig/animação, materiais avançados, AI e bridge futura com game engine. Os itens **P3D-144–155** formalizam o pós-GA: Asset Validator, collisions, sockets, LODs, palettes, texture atlas, vertex colors, batch processing, portable packages, Tool Presets, Command Recipes/Macros e Live Asset Link.

A planilha continua sendo o índice operacional resumido; **esta Implementation Bible é a especificação principal**.

# Auditoria final e execução por waves

A revisão final identificou lacunas transversais que agora estão formalizadas nas páginas **10–15**. Nenhuma P3D P0/P1 deve ser tratada como implementation-ready apenas por existir no caderno: aplicar os gates `SPEC READY` e `IMPLEMENTATION READY`. Para execução multi-feature, usar **16 — Master Prompt de Implementação por Gauntlet Waves**; ele define a ordem global, quality gates e condições de saída até GA e pós-GA.

[00 — Constituição do Projeto e Diretiva para Agentes](00%20%E2%80%94%20Constitui%C3%A7%C3%A3o%20do%20Projeto%20e%20Diretiva%20para%20Agent%203da9bb7d023f81b38d2df54976ff9bcc.md)

[01 — Protocolo de Especificação P3D e Prompt para LLM](01%20%E2%80%94%20Protocolo%20de%20Especifica%C3%A7%C3%A3o%20P3D%20e%20Prompt%20para%20%203da9bb7d023f81f5b9c9cd63ad798790.md)

[02 — Gauntlet Loop, Quality Gates e Definition of Done](02%20%E2%80%94%20Gauntlet%20Loop,%20Quality%20Gates%20e%20Definition%20of%20%203da9bb7d023f8126b6f5e61ba4cc397f.md)

[03 — Invariantes de UI/UX, Design System e Acessibilidade](03%20%E2%80%94%20Invariantes%20de%20UI%20UX,%20Design%20System%20e%20Acessib%203da9bb7d023f818e87f1d7e412c5edb9.md)

[04 — Invariantes de Arquitetura, Modularidade e Core Agnóstico à UI](04%20%E2%80%94%20Invariantes%20de%20Arquitetura,%20Modularidade%20e%20Co%203da9bb7d023f81fbacc1e5c1ca6b360c.md)

[05 — Política de Documentação, Screenshots e Changelog](05%20%E2%80%94%20Pol%C3%ADtica%20de%20Documenta%C3%A7%C3%A3o,%20Screenshots%20e%20Chang%203da9bb7d023f8154ba5bfbdf59996f70.md)

[06 — Governança do Backlog, Status e Prioridades](06%20%E2%80%94%20Governan%C3%A7a%20do%20Backlog,%20Status%20e%20Prioridades%203da9bb7d023f81d5b6d7c1bc4d81816f.md)

[07 — Roadmap, Adendos e Critérios para Novos Módulos](07%20%E2%80%94%20Roadmap,%20Adendos%20e%20Crit%C3%A9rios%20para%20Novos%20M%C3%B3dul%203da9bb7d023f81829ef4dc0563c4f9a7.md)

[A — Project & Files](A%20%E2%80%94%20Project%20&%20Files%203da9bb7d023f81848075edd44ab90aa3.md)

[B — Viewport & Navigation](B%20%E2%80%94%20Viewport%20&%20Navigation%203da9bb7d023f81a5b519e3f8a052a12f.md)

[C — Reference Workflow](C%20%E2%80%94%20Reference%20Workflow%203da9bb7d023f81b6ae74f0e9c36f2046.md)

[D — Selection, Transform & Modeling](D%20%E2%80%94%20Selection,%20Transform%20&%20Modeling%203da9bb7d023f8174b7f0d37eef29b408.md)

[E — Assets, Scene & Inspector](E%20%E2%80%94%20Assets,%20Scene%20&%20Inspector%203da9bb7d023f81d69d18eb7df7707747.md)

[F — Surface, Materials, Paint & UV](F%20%E2%80%94%20Surface,%20Materials,%20Paint%20&%20UV%203da9bb7d023f8146b501ee6b47679bb8.md)

[G — Animation & Rigging](G%20%E2%80%94%20Animation%20&%20Rigging%203da9bb7d023f810fba3acb384c93425b.md)

[H — Import & Export](H%20%E2%80%94%20Import%20&%20Export%203da9bb7d023f81e5b2b6fd472352c10a.md)

[I — UI Shell & Professional UX](I%20%E2%80%94%20UI%20Shell%20&%20Professional%20UX%203da9bb7d023f81b19998e297a70e1894.md)

[J — Customization, i18n & Keymaps](J%20%E2%80%94%20Customization,%20i18n%20&%20Keymaps%203da9bb7d023f812ab8acf9c5b999e501.md)

[K — Architecture & Modularity](K%20%E2%80%94%20Architecture%20&%20Modularity%203da9bb7d023f81859596fdbd8f231efb.md)

[L — Plugins, Automation & AI](L%20%E2%80%94%20Plugins,%20Automation%20&%20AI%203da9bb7d023f816a8cb3ce97e375b91d.md)

[M — Help, Documentation & QA](M%20%E2%80%94%20Help,%20Documentation%20&%20QA%203da9bb7d023f81ada715c17df9979df4.md)

[N — Performance & Project Philosophy](N%20%E2%80%94%20Performance%20&%20Project%20Philosophy%203da9bb7d023f81e1b3e7e4210bd64053.md)

[O — Future Product Integration](O%20%E2%80%94%20Future%20Product%20Integration%203da9bb7d023f8147a105fd530dae3e86.md)

[08 — Roadmap Pós-GA Aprovado](08%20%E2%80%94%20Roadmap%20P%C3%B3s-GA%20Aprovado%203da9bb7d023f81cf93b0fbc587395db5.md)

[09 — Shared 3D Foundation, Map Editor e Game Engine](09%20%E2%80%94%20Shared%203D%20Foundation,%20Map%20Editor%20e%20Game%20Engin%203da9bb7d023f81ec8ce1f837860e0920.md)

[10 — Convenções Espaciais, Unidades e Coordenadas](10%20%E2%80%94%20Conven%C3%A7%C3%B5es%20Espaciais,%20Unidades%20e%20Coordenadas%203da9bb7d023f81d8bfe3f4c671a6a0c7.md)

[11 — Contrato de Mesh, Selection, Tools e Undo](11%20%E2%80%94%20Contrato%20de%20Mesh,%20Selection,%20Tools%20e%20Undo%203da9bb7d023f81b3bc84de1a30708e34.md)

[12 — Contrato de Materiais, Texturas, UV e Color Pipeline](12%20%E2%80%94%20Contrato%20de%20Materiais,%20Texturas,%20UV%20e%20Color%20P%203da9bb7d023f81d5bbdae4ef29debe13.md)

[13 — Jobs, Concorrência, Diagnósticos, Segurança e Trust Boundaries](13%20%E2%80%94%20Jobs,%20Concorr%C3%AAncia,%20Diagn%C3%B3sticos,%20Seguran%C3%A7a%20e%203da9bb7d023f817e9a93e7ba16b483fe.md)

[14 — Release, Compatibilidade, Distribuição e Critérios de GA](14%20%E2%80%94%20Release,%20Compatibilidade,%20Distribui%C3%A7%C3%A3o%20e%20Crit%203da9bb7d023f8181b495c9306e7a6d8c.md)

[15 — Auditoria Final de Lacunas e Readiness Matrix](15%20%E2%80%94%20Auditoria%20Final%20de%20Lacunas%20e%20Readiness%20Matrix%203da9bb7d023f81de9526f4746ffb74d8.md)

[16 — Master Prompt de Implementação por Gauntlet Waves](16%20%E2%80%94%20Master%20Prompt%20de%20Implementa%C3%A7%C3%A3o%20por%20Gauntlet%20W%203da9bb7d023f8159898bdea9663eeef5.md)