> [← Voltar ao Índice de Modernização](/modernization/) | [Documento Integral Monolítico](/modernization/full-gauntlet)

# 2. Canonical Gauntlet Loop

Every wave and every major dependency group must execute:

```text
AUDIT
  ↓
TARGET
  ↓
SAFETY TESTS
  ↓
IMPLEMENT / REFACTOR
  ↓
BUILD
  ↓
TEST
  ↓
VISUAL / BEHAVIOR REVIEW
  ↓
ARCHITECTURE CHECK
  ↓
PERFORMANCE CHECK
  ↓
DOCS CHECK
  ↓
SCORE
  ↓
FIX
  ↓
REPEAT
```

## 2.1 Scoring

Score **0–10 without inflation** in these dimensions:

1. Functional Correctness
2. Architecture / Decoupling
3. Data Integrity
4. Tests
5. UX / Accessibility
6. Performance
7. Failure Handling / Security
8. Documentation

A score may rise only when supported by evidence:

- passing tests;
- real behavior;
- screenshots;
- dependency graph checks;
- architecture checks;
- benchmarks;
- fuzz/property results;
- diagnostics;
- reproducible commands.

“Implemented”, “compiled once”, or “agent says it looks correct” is not evidence.

## 2.2 Permanent gates

At minimum, for every meaningful wave:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Also run when introduced/applicable:

```bash
cargo deny check
cargo test --workspace --doc
cargo test -p <affected-crate>
cargo bench --no-run
cargo fuzz build
```

For feature-gated combinations, test the combinations deliberately rather than relying only on `--all-features`.

UI-affecting waves additionally require:

- `egui_kittest` flows;
- focus/keyboard interaction tests;
- accessibility inspection;
- screenshot/visual comparison;
- 1366×768 and 1920×1080 layout checks;
- DPI/scale checks;
- long-string locale checks;
- input routing checks;
- no shortcut leakage through focused text fields/modals.

## 2.3 Stop condition

A wave stops only when:

- its acceptance criteria are satisfied;
- no P0/P1 defects remain inside that wave’s scope;
- all regressions introduced by the wave are fixed;
- architecture rules pass;
- docs/changelog are synchronized;
- remaining limitations are explicitly accepted with impact/risk/next action.

If a wave breaks previously correct behavior, rollback the smallest offending change and reduce the migration step. Avoid big-bang rewrites.

---

# 3. Agent operating rules

1. **Do not begin by editing dependencies blindly.**
2. Audit the repository, current branch, manifests, feature flags, current test state, renderer paths, UI adapters, CI, docs, and current dependency graph.
3. Record a baseline report before edits.
4. Do not discard working implementations merely because a new crate exists.
5. If a requested library overlaps an existing working solution, integrate it at the correct boundary, feature-gate it, or use it for the intended future/dev path; do not create two uncontrolled implementations.
6. If a library is incompatible with egui 0.36/wgpu 30:
   - first check its current stable release;
   - then maintained upstream git if a release is imminent;
   - only then consider a minimal fork/patch;
   - keep the patch behind an adapter;
   - preserve license notices;
   - document the reason and upstream issue/commit;
   - do not downgrade the whole Petunia stack to satisfy one optional library.
7. P2 and P3 dependencies must be **properly integrated according to their adoption class**, not dumped into default production dependencies.
8. P3/FUTURE integration means:
   - real adapter/provider module exists;
   - feature is default-off;
   - it compiles and has a smoke/integration test or executable dev path;
   - documentation explains activation and future owner;
   - it does not block V1/GA;
   - no dead default production UI is resurrected.
9. DEV TOOL dependencies must not be enabled in distribution builds by default.
10. Security-sensitive services (`egui_mcp`, inspection endpoints, plugin host, MCP server) default to disabled/restricted and never bind broadly without explicit configuration.
11. Keep commits small and wave-scoped if repository workflow permits.
12. Do not hide failures by disabling tests, broad `allow` lints, removing assertions, or weakening acceptance criteria.
13. Do not artificially raise Gauntlet scores.
14. Continue fixing until the wave meets its exit gate.

---
