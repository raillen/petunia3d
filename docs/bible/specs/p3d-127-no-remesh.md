# P3D-127 — No Remesh

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Transversal
- **Status Canônico**: `COMPLIANT (Invariantes de Filosofia do Projeto)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

**Fora de escopo / decisão normativa** · Prioridade: RULE.

</aside>

## Decisão

Remesh não faz parte do escopo principal do Petunia3D. O editor é low-poly/direct-mesh e deve preservar topologia previsível.

## Implicações

Paint/Height não introduz remesh silencioso; sculpt workflows não são requisito; operações de modelagem devem resolver necessidades por mesh direta/generators/booleans limitados quando aprovados.

## Regra para agentes

Não adicionar dependência ou pipeline de remesh como “atalho técnico” sem nova decisão explícita de produto.