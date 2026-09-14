> [← Voltar ao Índice de Modernização](/modernization/) | [Documento Integral Monolítico](/modernization/full-gauntlet)

# 17. Performance rules

Petunia must remain friendly to modest PCs.

Never accept a new library solely because it simplifies code if it measurably harms important hot paths without justification.

For each performance-relevant dependency:

- record baseline;
- record post-integration result;
- record fixture/hardware/software context;
- compare memory where meaningful;
- compare startup where meaningful;
- compare frame time where meaningful.

Particularly benchmark:

- `manifold-rust`;
- `geo` operations used interactively;
- `rayon` job thresholds;
- `egui_taffy`;
- `egui_table`;
- `rstar`;
- project ZIP save/load;
- xatlas unwrap.

Avoid parallelizing tiny workloads where Rayon overhead dominates.

---

# 18. Dependency quality rules

Every direct dependency must have:

- purpose;
- owner;
- compatible license;
- current version rationale;
- maintenance assessment;
- feature selection;
- default-feature review where important;
- security/trust impact;
- removal/replacement path.

Use `default-features = false` only when you understand and test the resulting feature set.

Do not minimize features blindly and accidentally remove Linux/Windows support, image codecs, accessibility, or renderer functionality.

Run:

```bash
cargo tree -e features
cargo tree -d
cargo deny check
```

Resolve unjustified duplicates.

---
