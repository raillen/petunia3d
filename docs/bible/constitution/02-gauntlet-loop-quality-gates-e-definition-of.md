# 02 — Gauntlet Loop, Quality Gates e Definition of Done

# Loop canônico

`AUDIT → TARGET → SAFETY TESTS → IMPLEMENT/REFACTOR → BUILD → TEST → VISUAL/BEHAVIOR REVIEW → ARCHITECTURE CHECK → PERFORMANCE CHECK → DOCS CHECK → SCORE → FIX → REPEAT`.

# Notas

Avaliar de 0–10 sem inflação. Uma nota só sobe quando há evidência: testes, comportamento, screenshots, dependency checks ou métricas.

# Quality Gates permanentes

- `cargo fmt --check`;
- `cargo check`;
- unit/integration tests relevantes;
- `cargo clippy` quando viável;
- architecture checks;
- topology/data invariant checks quando aplicáveis;
- failure injection para save/import/autosave/jobs quando aplicável;
- UI interaction/regression tests para fluxos afetados;
- performance baseline/delta em hot paths;
- docs/generated references atualizados;
- screenshots reais quando UI mudar;
- nenhum novo hardcode de texto, ícone, cor ou shortcut em UI pública.

# Taxonomia de testes

Preferir a menor combinação que prove o comportamento: unit para algoritmos puros; integration para boundaries; golden/fixtures para formatos; property/invariant tests para mesh/serialization quando úteis; UI tests para interação/focus; snapshots apenas onde estáveis; benchmarks para regressão; fuzzing/parsers quando custo/risco justificar.

# Scorecard mínimo

Pontuar 0–10 com evidência em **Functional Correctness, Architecture/Decoupling, Data Integrity, Tests, UX/Accessibility, Performance, Failure Handling/Security e Documentation**. Registrar baseline e delta; não elevar nota por conclusão administrativa.

# Condições de parada

O loop para quando critérios de aceitação da feature/wave estão satisfeitos e não restam P0/P1 dentro do escopo, ou quando uma limitação é explicitamente aceita/documentada com impacto, owner/next action e risco.

# Rollback

Se uma etapa quebra funcionalidade já correta, projeto salvo, performance crítica ou boundaries arquiteturais, reverter a menor mudança necessária e redimensionar a etapa. Evitar big-bang rewrites.