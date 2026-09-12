# Documentação Canônica do Projeto Petunia3D (`docs/`)

## O que é este diretório?
A pasta `docs/` é a fonte canônica da verdade para todas as especificações de engenharia, produto, arquitetura, testes, manuais e governança do Petunia3D.

## Para que serve?
Implementa o princípio de **Documentação Canônica Viva**: o repositório é autossuficiente e todo o conhecimento técnico essencial reside diretamente no código e em arquivos Markdown padronizados e versionados.

## Roteador Central
Consulte [`PRUMO.md`](PRUMO.md) como ponto de entrada principal para navegação guiada por intenção (usuário, desenvolvedor, operador, agente).

## Inventário Completo de Documentação
- [`PRUMO.md`](PRUMO.md): Roteador central de intenção (mapa mestre de navegação).
- [`ARCHITECTURE.md`](ARCHITECTURE.md): Grafo de crates, regras de dependência, eventos e render-on-demand.
- [`GAUNTLET.md`](GAUNTLET.md): Histórico de validações independentes, estabilidade, benchmarks e evidências.
- `contracts/`: Contratos formais e mapeamento semântico de documentação (`bindings.json`).
- `architecture/`: Arquitetura do sistema, boundaries, contratos de Clean Code e ADRs.
- `product/`: Visão de produto, proposta de valor, público-alvo e limites de escopo.
- [`development/premium-interaction-plan.md`](development/premium-interaction-plan.md): plano e evidências da rodada premium.
- `development/`: Padrões de código, diretrizes de Rust e estratégia de testes com quality gates.
- `operations/`: Procedimentos de deploy, ciclo de vida de instalação, observabilidade e telemetria.
- `reference/`: Referência da CLI, variáveis de ambiente, códigos de saída e parâmetros.
- `manual/`: Manuais de instalação passo a passo e guia prático de uso/atalhos do viewport.
- `ui/`: Design system, tokens, especificação dos 4 workspaces e fluxos de tela.
- `security/`: Modelagem de ameaças (STRIDE), contratos de segurança e trust boundaries de I/O.
- `governance/`: Políticas de branches, commits convencionais e regras de PR.
- `petunia3d-livro-vivo/`: Coleção abrangente de 36 capítulos com especificações técnicas e de design.
