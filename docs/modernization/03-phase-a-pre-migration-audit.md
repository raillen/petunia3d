> [← Voltar ao Índice de Modernização](/modernization/) | [Documento Integral Monolítico](/modernization/full-gauntlet)

# 4. Phase A — Pre-migration audit

Before changing code, produce `docs/audits/stack-modernization/00-baseline.md` containing:

## 4.1 Repository baseline

Capture:

```bash
git status --short
git branch --show-current
git rev-parse HEAD
rustc --version --verbose
cargo --version
cargo metadata --format-version 1
cargo tree --workspace
cargo tree -d
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features
```

Record:

- every workspace crate;
- every `edition`;
- current direct dependency versions;
- duplicate major/minor lines of egui/wgpu/winit;
- current features;
- current renderer ownership;
- current host lifecycle;
- current AccessKit integration;
- current file dialog paths;
- current UI test paths;
- current plugin/MCP state;
- current import/export state;
- current geometry/UV state;
- current CI jobs.

## 4.2 Baseline screenshots and behavior

Capture at least:

- initial workspace;
- Model workspace;
- Edit mode;
- Outliner;
- Properties;
- Settings;
- Asset Browser;
- file open/save flow;
- viewport shading modes;
- transform gizmo;
- reference-image flow.

Use the project’s existing golden references where applicable.

## 4.3 Baseline performance

Record representative current measurements before migration:

- cold startup;
- first frame;
- steady idle CPU;
- representative viewport frame time;
- large Outliner interaction;
- representative mesh operation;
- save/load fixture;
- import/export fixture where available.

Do not invent numbers. If a measurement cannot be produced, state why.

## 4.4 Baseline score

Score all eight Gauntlet dimensions with evidence.

Only then start modernization.

---
