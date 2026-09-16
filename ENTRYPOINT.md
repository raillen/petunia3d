# Prumo entrypoint

0. Read `AGENTS.md` first. It is normative. **The public documentation site is
   FROZEN until the end of the project** (see `AGENTS.md` §1) and the canonical
   notebook lives in `docs/bible/` (Livro Vivo).
1. Read `prumo.json`, `PROJECT_STATE.md` and `docs/PRUMO.md`.
2. Read the active Goal and its dependencies.
3. Start with the minimum sufficient context; do not read the entire repository.
4. Prefer Context Packs/task maps, document sections, symbols and related tests.
5. Expand context only when evidence is insufficient; delegation depth is bounded by `prumo.json`.
6. Never weaken acceptance criteria silently.
7. Keep code, tests and canonical docs synchronized through a Documentation Delta.
   Canonical documentation means `docs/bible/` — never a parallel copy.
8. Keep intermediate output compact and do not persist task-specific context files.
9. Before completion, record evidence/project intelligence and remove temporary context.
