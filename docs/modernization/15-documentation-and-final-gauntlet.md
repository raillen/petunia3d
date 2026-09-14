> [← Voltar ao Índice de Modernização](/modernization/) | [Documento Integral Monolítico](/modernization/full-gauntlet)

# 19. Documentation changes required

Update the canonical documentation only after implementation evidence exists.

At minimum update:

```text
README / developer setup
PROJECT_STATE.md
CHANGELOG.md
canonical stack documentation
dependency/adapters documentation
architecture diagrams if changed
feature flags
plugin/MCP security docs
project/package format docs
import/export docs
UI devtools docs
ThemeToken schema and generated token reference
IconId mapping/reference and icon-pack licensing/source metadata
TextId/i18n key reference and locale coverage report
PetuniaNumberField interaction specification
Properties/Tool Properties UX specification
benchmark/fuzz instructions
```

Generate `docs/audits/stack-modernization/FINAL_REPORT.md` with:

1. before/after stack;
2. exact versions;
3. dependencies added;
4. dependencies removed;
5. feature flags;
6. architectural changes;
7. migration notes;
8. test results;
9. benchmark results;
10. fuzz/property results;
11. security findings;
12. accessibility findings;
13. screenshots;
14. known limitations;
15. final scorecard;
16. unresolved issues ranked P0/P1/P2/P3.

---

# 20. Required final dependency audit

At the end, generate a machine-verifiable table:

| Dependency | Version | Owner crate | Class | Default enabled? | Adapter/provider | Tests | License | Status |
|---|---|---|---|---|---|---|---|---|

Every P0→P3 dependency from this directive must appear.

Allowed status values:

```text
INTEGRATED
INTEGRATED_DEV_ONLY
INTEGRATED_OPTIONAL
INTEGRATED_FUTURE_DEFAULT_OFF
PATCHED_UPSTREAM_COMPAT
BLOCKED_EXTERNAL
```

`BLOCKED_EXTERNAL` is allowed only when:

- the agent attempted reasonable compatible integration;
- the blocker is external and evidenced;
- forcing it would violate architecture/security/build correctness;
- an ADR records the exact blocker;
- a next action is specified.

The goal remains zero blocked dependencies.

---

# 21. Final combined Gauntlet

After all waves, run a clean combined validation from repository root.

At minimum:

```bash
cargo clean
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo test --workspace --doc
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
cargo bench --no-run
cargo fuzz build
cargo tree -d
cargo xtask ui-audit
```

Then run the project-specific:

- UI interaction suite;
- token/theme hardcode audit;
- TextId/i18n coverage audit;
- icon SVG/iconflow mapping audit;
- scrubbable numeric-field interaction tests;
- theme × locale × icon-pack visual matrix;
- visual regression;
- renderer smoke tests;
- import/export fixtures;
- project save/load/recovery;
- Lua capability tests;
- MCP tests;
- headless CLI tests;
- architecture dependency checks;
- representative benchmarks.

Repeat:

```text
SCORE → FIX → RETEST → RESCORE
```

until:

- no in-scope P0/P1 defect remains;
- no failing required test remains;
- no unexplained visual regression remains;
- no forbidden dependency edge remains;
- no required P0→P3 item is silently absent;
- default production build contains no unintended dev/future service;
- documentation matches real behavior.

---

# 22. Final score requirements

Do not manufacture a 10/10.

A 10/10 in a dimension requires evidence that no known meaningful gap remains in the scoped modernization.

Final report must show:

| Dimension | Baseline | Final | Evidence |
|---|---:|---:|---|
| Functional Correctness | x/10 | x/10 | tests/behavior |
| Architecture / Decoupling | x/10 | x/10 | graph checks |
| Data Integrity | x/10 | x/10 | invariants/fixtures |
| Tests | x/10 | x/10 | suites/property/fuzz |
| UX / Accessibility | x/10 | x/10 | kittest/inspection/screens |
| Performance | x/10 | x/10 | benchmarks |
| Failure Handling / Security | x/10 | x/10 | injection/capability tests |
| Documentation | x/10 | x/10 | docs checks |

If any score is below the project’s accepted completion threshold, continue the Gauntlet Loop.

---

# 23. Mandatory agent completion report

When finished, return a concise but evidence-rich summary containing:

1. final Rust/Edition version;
2. final egui/eframe/wgpu versions;
3. renderer status;
4. every P0 library and integration status;
5. every P1 library and integration status;
6. every P2 library and integration status;
7. every P3 library and integration status;
8. new feature flags;
9. dependencies removed/replaced;
10. architecture changes;
11. test command results;
12. benchmark deltas;
13. security/fuzz/property-test results;
14. UI/accessibility results;
15. theme-token coverage and built-in theme validation;
16. full i18n/TextId coverage including en + pt-BR + pseudo-locale test;
17. iconflow/SVG migration status and remaining justified raster assets;
18. numeric-field and Properties/Tool Properties UX results;
19. final scorecard;
20. exact known limitations;
21. final commit SHA(s), if commits are part of the workflow.

Do not say “done” until the evidence exists.

---

# 24. Source-of-truth references the agent must read before implementation

Repository:

- `Cargo.toml`
- all workspace `Cargo.toml` files
- `PROJECT_STATE.md`
- `CHANGELOG.md`
- `docs/petunia3d-livro-vivo/27-stack-rust-canonica.md`
- `docs/petunia3d-livro-vivo/35-egui-components-adapters-tooling.md`
- current architecture audits
- current implementation plans
- relevant P3D specification mirrors in the repository

Canonical Implementation Bible / Notion, when available:

- **02 — Gauntlet Loop, Quality Gates e Definition of Done**
- **03 — Invariantes de UI/UX, Design System e Acessibilidade**
- **04 — Invariantes de Arquitetura, Modularidade e Core Agnóstico à UI**
- **05 — Política de Documentação, Screenshots e Changelog**
- **10 — Convenções Espaciais, Unidades e Coordenadas**
- **11 — Contrato de Mesh, Selection, Tools e Undo**
- **12 — Contrato de Materiais, Texturas, UV e Color Pipeline**
- **13 — Jobs, Concorrência, Diagnósticos, Segurança e Trust Boundaries**
- **14 — Release, Compatibilidade, Distribuição e Critérios de GA**
- **15 — Auditoria Final de Lacunas e Readiness Matrix**
- **16 — Master Prompt de Implementação por Gauntlet Waves**

Additional implementation references for this modernization:

- `iconflow`: https://github.com/FerrisMind/iconflow
- `egui_extras` SVG loader: https://docs.rs/egui_extras/latest/egui_extras/loaders/fn.install_image_loaders.html
- `egui::DragValue`: https://docs.rs/egui/latest/egui/widgets/struct.DragValue.html
- `twill`: https://docs.rs/crate/twill/latest
- theme architecture references only: `egui_elements`, `egui-elegance`, `egui_sauge`, `egui_colors`

When documentation and implementation disagree:

1. identify the conflict;
2. prefer newer explicit normative decisions;
3. verify behavior/tests;
4. do not silently rewrite history;
5. record the discrepancy and resolution.

---

# 25. Final command to the coding agent

Execute this modernization as a sequence of small, evidence-backed waves.

**Do not stop at dependency installation.**
A dependency counts only when it is correctly owned, isolated, exercised, tested, documented, and compatible with Petunia’s architecture.

**Do not sacrifice Petunia’s philosophy to satisfy a library.**
The library adapts to Petunia; Petunia does not reorganize its domain around library APIs.

**Do not turn optional/future dependencies into baseline bloat.**
Integrate P2/P3 completely at their proper boundary and lifecycle, using default-off features/dev tooling where their canonical adoption class requires it.

**Do not inflate Gauntlet scores.**
Fix defects and repeat until evidence supports the result.

Proceed continuously through:

```text
AUDIT
→ RUST 1.98.1 / EDITION 2024
→ EGUI 0.36.2 / EFRAME 0.36.2 / WGPU 30.0.1
→ P0
→ P1
→ P2
→ P3
→ UI/THEME/ICON/I18N/UX REMEDIATION
→ CONSOLIDATION
→ FULL GAUNTLET
→ FINAL REPORT
```

Preserve working behavior, improve architecture, and leave the repository in a cleaner, more testable, more observable, more secure, and easier-to-evolve state than before the migration.
