# Operações e Infraestrutura (`docs/operations/`)

## O que é este diretório?
Contém diretrizes de deploy, ciclo de vida de instalação, empacotamento para múltiplas plataformas e monitoramento/observabilidade do Petunia3D.

## Para que serve?
Garante que a compilação, distribuição, execução e atualização do software sejam previsíveis, auditáveis e resilientes a falhas.

## Inventário
- `deployment.md`: Procedimentos de compilação em release, checklist pré-lançamento e empacotamento.
- `installation-lifecycle.md`: Ciclo de vida de instalação, caminhos de runtime, tolerância a interrupções e desinstalação segura.
- `observability.md`: Níveis de log via `RUST_LOG`, diagnóstico de drivers de GPU e benchmarks de render-on-demand.
