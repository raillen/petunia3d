# 07 — Roadmap, Adendos e Critérios para Novos Módulos

# Adendos consolidados

- Possíveis módulos futuros devem reforçar criação/entrega de assets sem transformar o produto em DCC universal.
- Integração com futura game engine deve ocorrer por bridge/API/formato, mantendo Petunia independente.
- Efeitos de materiais entram primeiro como perfis/parâmetros simples antes de nodes completos.

# Critério de entrada de módulo

Perguntar: resolve problema real do workflow low-poly? pode ser desacoplado? aumenta carga cognitiva do fluxo principal? exige runtime/dependência pesada? pode viver como módulo/plugin?

# Direção

Candidatos coerentes incluem collision helpers, LOD simples, atlas/palette tools, rig/animation, export profiles e bridges para engines. Cena complexa, render cinematográfico e sculpt/remesh continuam fora do centro do produto.

[ADENDO-01 — Possíveis futuros módulos sem bloat](../addenda/adendo-01-possiveis-futuros-modulos-sem-bloat.md)

[ADENDO-02 — Caminho para uma game engine](../addenda/adendo-02-caminho-para-uma-game-engine.md)

[ADENDO-03 — Texturas com efeitos e shaders](../addenda/adendo-03-texturas-com-efeitos-e-shaders.md)

# Roadmap pós-GA aprovado

O planejamento pós-GA foi formalizado em [08 — Roadmap Pós-GA Aprovado](08-roadmap-pos-ga-aprovado.md). Ele inclui GA Hardening, Retro Asset Toolkit, Surface & Lookdev, Animation Toolkit, Extensibility & AI e o futuro ecossistema com produtos separados.

# Reuso tecnológico entre produtos

A estratégia para reaproveitar infraestrutura consolidada do Petunia3D em um Map Editor e uma Game Engine independentes foi formalizada em [09 — Shared 3D Foundation, Map Editor e Game Engine](09-shared-3d-foundation-map-editor-e-game-engin.md). A regra é compartilhar somente módulos neutros e comprovadamente reutilizáveis, nunca um AppState/UX único entre produtos.