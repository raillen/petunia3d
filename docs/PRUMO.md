# Prumo — Petunia3D (Intent Router)

Roteador central de intenção e mapa canônico de navegação do projeto **Petunia3D** para humanos e agentes autônomos.

---

## Estado Operacional Canônico

- [Estado Atual do Projeto](../PROJECT_STATE.md) — Fase ativa, metas e ordem de recuperação
- [`prumo.json`](../prumo.json) — Manifesto canônico do projeto e perfis tecnológicos
- [Contratos de Documentação (M5)](contracts/bindings.json) — Mapeamento formal de conformidade de documentação

---

## 1. Quero Usar o Produto (Usuário / Artista 3D)

- [Visão Geral e Introdução](../README.md) — O que é o Petunia3D e como começar
- [Manual de Uso e Navegação](manual/usage.md) — Controles de viewport, modos de seleção e atalhos
- [Workspaces e Design System](ui/README.md) — Os 4 workspaces (MODEL, PAINT, UV, EXPORT) e fluxos
- [Guia de Instalação e Requisitos](manual/installation.md) — Instalação via Cargo, requisitos de GPU e binários
- [Referência de Atalhos](../../assets/keybinds/petunia.toml) — Arquivo TOML configurável de atalhos de teclado

---

## 2. Quero Desenvolver e Contribuir (Engenheiro de Software)

- [Padrões de Código e Engenharia](development/coding-standards.md) — Diretrizes de Rust, `rustfmt` e `clippy`
- [Estratégia de Testes e Conformance](development/testing-strategy.md) — Pirâmide de testes, gates e validações
- [Plano e evidências de interação premium](development/premium-interaction-plan.md) — auditoria atual e critérios pendentes
- [Relatório de Evidências Gauntlet](GAUNTLET.md) — Histórico de rodadas de validação, benchmarks e métricas
- [Governança do Repositório](governance/repository-governance.md) — Políticas de branch, commits convencionais e PRs
- [Contrato de Segurança e Modelo de Confiança](security/security-contract.md) — Limites de confiança e sanitização de I/O
- [Modelagem de Ameaças (STRIDE)](security/threat-model.md) — Análise de riscos para modelador desktop

---

## 3. Arquitetura e Decisões de Engenharia

- [Topologia e Grafo de Crates](ARCHITECTURE.md) — Arquitetura de 14 crates acíclicos e render-on-demand
- [Visão Geral de Arquitetura](architecture/overview.md) — Camadas de domínio, aplicação e adaptadores
- [Contrato de Clean Code](architecture/clean-code-contract.md) — Princípios de separação de responsabilidades
- [Registros de Decisões Arquiteturais (ADRs)](architecture/adr/README.md):
  - [ADR 001: Linha de Base Arquitetural](architecture/adr/001-architecture-baseline.md)
  - [ADR 032: Migração de Odin para Rust](petunia3d-livro-vivo/32-adr-odin-para-rust.md)
- [Petunia3D Livro Vivo](petunia3d-livro-vivo/README.md) — 36 capítulos de especificações profundas:
  - [Workflow Shape-First](petunia3d-livro-vivo/02-workflow-modelagem-shape-first.md)
  - [Geometria e Topologia](petunia3d-livro-vivo/03-geometry-core-faces-topologia.md)
  - [Operações de Fusão e Weld](petunia3d-livro-vivo/04-combine-fuse-weld-personagens.md)
  - [Viewport e Shading](petunia3d-livro-vivo/05-viewport-shading-modos-visualizacao.md)
  - [Arquitetura Modular Rust & Safety](petunia3d-livro-vivo/34-arquitetura-modular-rust-safety.md)
  - [Design System & Tokens](petunia3d-livro-vivo/24-design-system-tokens-estados.md)
  - [Integração MCP para Agentes](petunia3d-livro-vivo/11-mcp-api-automacao-agentes.md)

---

## 4. Quero Operar e Suportar (Operador / Release)

- [Ciclo de Vida de Instalação](operations/installation-lifecycle.md) — Procedimentos de atualização, rollback e desinstalação
- [Guia de Deploy e Empacotamento](operations/deployment.md) — Compilação release, validação de dependências e distribuição
- [Observabilidade e Diagnósticos](operations/observability.md) — Logs `RUST_LOG`, diagnóstico de drivers e benchmarks
- [Referência da Linha de Comando (CLI)](reference/cli.md) — Variáveis `PETUNIA_BACKEND`, flags e códigos de saída

---

## 5. Sou um Agente de IA (Protocolo Lean Progressive Context)

1. Leia [`ENTRYPOINT.md`](../ENTRYPOINT.md), [`PROJECT_STATE.md`](../PROJECT_STATE.md) e este `docs/PRUMO.md`.
2. Identifique a meta ativa em `.ai/goals/`.
3. Carregue **apenas o contexto mínimo suficiente** para a tarefa específica.
4. Respeite as barreiras arquiteturais: `core` não conhece crates de módulos concretos; `render-gl` e `render-wgpu` são os únicos que tocam GPU.
5. Nunca enfraqueça os critérios de aceitação de testes nem suprima erros em silêncio.
6. Mantenha sincronizados código, testes e documentação através de deltas.
7. Registre evidências e telemetria antes de finalizar a execução.

---

## Metas e Inteligência

- **Metas do Projeto**: Localizadas em `.ai/goals/<fase>/` (ex: `.ai/goals/P00/`).
- **Inteligência Durável**: Métricas operacionais e consumo em `.prumo/history/project-intelligence.json`.
