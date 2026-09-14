# P3D-107 — API cross-language-ready

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 0 / Wave 1
- **Status Canônico**: `COMPLIANT (Arquitetura Spine & Invariantes)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **future-ready; não congelar ABI cedo** · Prioridade: P2.

</aside>

## Objetivo

Preparar caminho para C ABI/IPC/DTOs sem deformar o core Rust.

## Ordem

1. boundaries internas estáveis;
2. headless API;
3. handles/DTOs/events;
4. somente então escolher C ABI, IPC ou híbrido.

## Opções a comparar

Same-process C ABI com opaque handles; processo separado por IPC; híbrido direct Rust + external adapter. Avaliar viewport/GPU sharing separadamente.

## Regras

Não expor referências/lifetimes/structs Rust internas como API pública estável. Não serializar cada frame do viewport por screenshot streaming.

## Dependências

P3D-104, P3D-106, P3D-108–109.

## DoD

Architecture proposal + proof API quando chegar a fase; hoje manter types/IDs compatíveis com futura boundary.