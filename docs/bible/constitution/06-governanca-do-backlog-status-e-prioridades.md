# 06 — Governança do Backlog, Status e Prioridades

# Papel do backlog

A planilha é o índice operacional; a Implementation Bible é a especificação canônica; o repositório é a verdade da implementação; o website é a documentação pública.

# Campos principais

- **Status:** maturidade da especificação/execução.
- **Estado real:** avaliação inicial da implementação, sempre confirmada por auditoria de código.
- **Tipo de trabalho:** auditar, corrigir, implementar, redesenhar, pesquisar ou governar.
- **Prioridade:** P0 bloqueia fundações; P1 core/importante; P2 evolução/polish; P3 futuro; RESEARCH pesquisa; RULE invariante.
- **Epic:** agrupamento de dependências e sequência.

# Readiness states recomendados

Além de Status/Estado real, usar mentalmente ou na planilha quando útil: **IDEA/ROADMAP**, **SPEC DRAFT**, **SPEC READY**, **AUDITED**, **IMPLEMENTATION READY**, **IN PROGRESS**, **VALIDATION**, **DONE**, **ABSORBED**, **RULE**. `DONE` exige evidência, não apenas código presente.

# Regras

Não renumerar IDs existentes. Recursos absorvidos permanecem registrados para histórico. Novos recursos recebem IDs novos. Observações do usuário nunca são apagadas; avaliações da IA ficam em colunas próprias. P0/P1 não entram em implementação definitiva sem `SPEC READY`; grandes refactors exigem `AUDITED` + baseline de testes; itens pós-GA não são puxados para o GA sem decisão explícita.

# Dependency governance

Dependências locais das P3Ds não substituem o roadmap global. Para execução multi-feature, usar as waves de `16 — Master Prompt de Implementação por Gauntlet Waves` e só avançar quando blockers da wave anterior estiverem resolvidos ou explicitamente aceitos.