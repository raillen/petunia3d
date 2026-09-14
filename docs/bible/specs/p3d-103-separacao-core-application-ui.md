# P3D-103 — Separação Core / Application / UI

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 1
- **Status Canônico**: `COMPLIANT (Separação Core / Application / UI)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **boundary precisa ser comprovada pelo código** · Prioridade: P0.

</aside>

## Objetivo

Estabelecer ownership e direção de dependências claros sem arquitetura enterprise excessiva.

## Fronteira desejada

UI/frontend apresenta e coleta input; Application coordena use cases/commands/state; Core executa domínio/geometry; Infrastructure/Renderer lidam com mundo externo e GPU.

## Auditoria

Mapear crates/modules, imports, state holders, cycles e callbacks UI com business/mesh/filesystem logic.

## Regras

Não criar trait/repository para tudo. Criar boundaries somente onde substituição/teste/ownership justificam.

## Dependências

P3D-100–102, P3D-105, P3D-109.

## Testes / DoD

Dependency rules automatizadas, estado de UI separado de project/editor state e APIs internas idiomáticas Rust.