# 07 — Roadmap, Adendos e Critérios para Novos Módulos

# Adendos consolidados

- Possíveis módulos futuros devem reforçar criação/entrega de assets sem transformar o produto em DCC universal.
- Integração com futura game engine deve ocorrer por bridge/API/formato, mantendo Petunia independente.
- Efeitos de materiais entram primeiro como perfis/parâmetros simples antes de nodes completos.

# Critério de entrada de módulo

Perguntar: resolve problema real do workflow low-poly? pode ser desacoplado? aumenta carga cognitiva do fluxo principal? exige runtime/dependência pesada? pode viver como módulo/plugin?

# Direção

Candidatos coerentes incluem collision helpers, LOD simples, atlas/palette tools, rig/animation, export profiles e bridges para engines. Cena complexa, render cinematográfico e sculpt/remesh continuam fora do centro do produto.

[ADENDO-01 — Possíveis futuros módulos sem bloat](ADENDO-01%20%E2%80%94%20Poss%C3%ADveis%20futuros%20m%C3%B3dulos%20sem%20bloat%203da9bb7d023f811eb9ceffee4f2ef578.md)

[ADENDO-02 — Caminho para uma game engine](ADENDO-02%20%E2%80%94%20Caminho%20para%20uma%20game%20engine%203da9bb7d023f8142974ecb89e9befb7d.md)

[ADENDO-03 — Texturas com efeitos e shaders](ADENDO-03%20%E2%80%94%20Texturas%20com%20efeitos%20e%20shaders%203da9bb7d023f816e8d50c6bd5cecc8a1.md)

# Roadmap pós-GA aprovado

O planejamento pós-GA foi formalizado em [08 — Roadmap Pós-GA Aprovado](08%20%E2%80%94%20Roadmap%20P%C3%B3s-GA%20Aprovado%203da9bb7d023f81cf93b0fbc587395db5.md). Ele inclui GA Hardening, Retro Asset Toolkit, Surface & Lookdev, Animation Toolkit, Extensibility & AI e o futuro ecossistema com produtos separados.

# Reuso tecnológico entre produtos

A estratégia para reaproveitar infraestrutura consolidada do Petunia3D em um Map Editor e uma Game Engine independentes foi formalizada em [09 — Shared 3D Foundation, Map Editor e Game Engine](09%20%E2%80%94%20Shared%203D%20Foundation,%20Map%20Editor%20e%20Game%20Engin%203da9bb7d023f81ec8ce1f837860e0920.md). A regra é compartilhar somente módulos neutros e comprovadamente reutilizáveis, nunca um AppState/UX único entre produtos.