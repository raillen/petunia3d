# P3D-129 — Sem login obrigatório

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Transversal
- **Status Canônico**: `COMPLIANT (Invariantes de Filosofia do Projeto)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

**Invariante de produto** · Prioridade: RULE.

</aside>

## Decisão

Funcionamento normal do Petunia3D é local e não exige conta, autenticação ou cloud service.

## Implicações

Project files, themes, translations, icon packs, keymaps e plugins funcionam offline. Futuras integrações online/AI podem exigir credenciais próprias, mas são opcionais e isoladas.

## Privacidade

Não introduzir telemetry/login por dependência transitiva sem decisão explícita e documentação.