# 20 — Testes, Conformance e Qualidade Técnica

<aside>
🧪

A arquitetura deve ser testável sem a interface, mas a baseline Rust/egui agora também define testes próprios para UI. Geometry, document, renderer, export, plugins, MCP e interface possuem suites independentes e complementares.

</aside>

# Pirâmide de testes

## Unit

Testar matemática, IDs, serialization helpers, profile processing, topology primitives e pequenas regras de materials/UV.

## Command tests

Cada command mutável deve verificar:

- resultado esperado;
- invariantes após commit;
- Undo restaura estado anterior;
- Redo restaura resultado;
- erro/Cancel não altera documento;
- IDs/remaps retornados são válidos.

## Geometry fixtures

Manter coleção pequena de meshes legíveis e intencionais:

- triangle/quad/n-gon;
- concave polygon;
- open boundary;
- cube/cylinder;
- different-count bridge loops;
- non-manifold inputs;
- near-degenerate cases;
- mirrored geometry;
- UV seams/sharp edges.

# Property/invariant testing

Usar **proptest** para gerar inputs e sequências simples de edits. Em debug/tests, validar invariantes da half-edge mesh:

- twin consistency;
- loop closure;
- next/previous coherence;
- face/edge/vertex ownership;
- no references to deleted `slotmap` keys;
- finite coordinates / no NaN/Inf;
- valid winding metadata;
- triangulation map para `FaceId` válido;
- IDs tipados e únicos dentro do domínio.

Propriedades importantes incluem `Connect → Undo → estado semanticamente igual`, `valid Profile → Extrude → invariants` e `save → load → semantic equality`.

# Fuzzing

Usar **cargo-fuzz** e priorizar parsers/geometry boundaries:

- `.petunia` ZIP/manifest/JSON loader;
- image decode adapters;
- OBJ/glTF import adapters;
- polygon triangulation;
- Profile validation;
- Connect matching;
- Cut/Slice inputs;
- provider adapter conversions;
- plugin manifest;
- MCP schema/input boundaries.

Fuzz tests nunca dependem da UI.

# Provider contract tests

`BooleanProvider`, `UvUnwrapProvider`, `Importer`, `Exporter` e futuros providers possuem suites de contrato independentes.

A mesma suíte deve validar todas as implementações substituíveis. Trocar o vendor não autoriza mudar silenciosamente semântica, invariantes ou error codes.

Exemplo Boolean:

```
input A/B
→ provider
→ output is structurally valid
→ material/property mapping obeys contract
→ no NaN/Inf
→ deterministic enough for fixture
```

Trocar Manifold/xatlas exige passar os mesmos contratos.

# Golden tests

Usar golden files para:

- canonical project serialization;
- project migrations;
- triangulation de fixtures;
- export glTF/OBJ;
- selected validator diagnostics;
- MCP schemas/tool catalog;
- Lua API descriptors.

Goldens só mudam com revisão explícita.

# Round-trip tests

- save `.petunia` → load → semantic equality;
- import supported format → internal → export quando aplicável;
- project migration antiga → atual → save → reload;
- Undo/Redo multiple commands → final hash/state esperado.

# Crash/recovery tests

Simular interrupção durante save e confirmar que:

- arquivo anterior permanece válido;
- temp file incompleto não substitui original;
- recovery snapshot pode ser detectado;
- opening invalid recovery nunca destrói o save normal.

# Plugin tests

- capability denial;
- one Lua state per plugin;
- blocked filesystem/system APIs;
- transaction rollback em erro Lua;
- unload remove commands/subscriptions;
- API version mismatch produz diagnóstico claro;
- memory/execution budget não corrompe host.

# MCP tests

- catalog determinístico;
- input schema validation;
- permission denial;
- mutations geram transaction/Undo;
- stale IDs retornam erro estruturado;
- batch atomicity;
- bridge local IPC não expõe endpoint de rede por padrão;
- app closed/not connected retorna erro explícito.

# Export validation tests

Comparar output com glTF validators quando disponível no CI e carregar fixtures em pelo menos um parser independente. OBJ recebe parser round-trip de teste.

# CI

Baseline Rust:

```
cargo fmt --check
→ cargo clippy
→ unit/command/property tests
→ provider contract tests
→ project round-trip/migrations
→ export goldens
→ plugin/MCP tests
→ UI component/accessibility/visual tests
→ dependency/license/security checks
→ debug/release build
```

Matrix inicial:

- Linux x86_64;
- Windows x86_64.

macOS entra quando plataforma virar target suportado obrigatório.

`cargo xtask verify` deve agrupar a verificação local equivalente para humanos e agentes.

# Testes de interoperabilidade entre funcionalidades

Além de suites isoladas, manter workflows canônicos que provem composição entre features independentes:

```
Primitive → Extrude → Bevel → Mirror → Cut → Auto UV → Paint → Save → Reload → Export
```

```
Reference → Profile → Extrude → Project From Reference → Paint → Export
```

Uma Tool não é considerada interoperável porque outra Tool consegue chamá-la; a integração deve ocorrer via Commands/Algorithms compartilhados conforme capítulo 34.

# Architecture conformance tests

Sempre que possível, CI/`xtask` deve verificar invariantes arquiteturais mecanicamente:

- `petunia-geometry` não depende de egui, eframe, Lua ou MCP;
- core/domain não depende de wgpu;
- renderer não recebe `&mut Document`;
- optional feature ausente retorna capability error;
- public adapters não expõem vendor types/slotmap keys;
- crates seguras permanecem `#![forbid(unsafe_code)]`;
- tooling/commands canônicos existem para build/test/verify.

Regras não verificáveis estaticamente entram em architecture tests/review checklist: Tool não chama Tool; `.clone()` não é usado como fuga automática de borrowing; traits existem apenas para boundaries reais; long-lived relationships usam IDs.

# Regression rule

Todo bug geométrico, corrupção de save, crash de provider, incompatibilidade de import/export ou falha de Undo deve ganhar fixture/test antes ou junto da correção.

# UI/egui tests

A interface agora possui critérios explícitos:

- component states;
- keyboard navigation;
- focus order/visibility;
- accessibility roles/names/states;
- screenshots/visual regression para componentes congelados;
- flows reais com input automatizado;
- múltiplos UI scales/HiDPI quando a infraestrutura permitir;
- labels longos/i18n.

Usar `egui_kittest` para semantic/interaction/snapshot tests e, em builds de desenvolvimento, `egui_inspection`/`egui_mcp` para inspecionar AccessKit tree, enviar input e capturar screenshots. A UI não está pronta apenas porque compila.

# Relação com a arquitetura Rust-safe

Aplicar integralmente [34 — Arquitetura Modular Explícita, Rust Safety e Representação em Código](34-arquitetura-modular-rust-safety.md). Segurança de memória do compilador é somente uma camada; correctness inclui invariants, property tests, fuzzing, conformance, integration, interoperability, Miri onde aplicável e testes visuais/accessibility.

# Regra final

O framework deve gerar tarefas de implementação acompanhadas de testes de contrato e invariantes. **Uma operação geométrica não está pronta só porque produz uma forma visualmente plausível.**

# Consolidação — Gauntlet Loop e score por evidência

O processo canônico de implementação/validação absorvido do antigo caderno é:

`AUDIT → TARGET → SAFETY TESTS → IMPLEMENT/REFACTOR → BUILD → TEST → VISUAL/BEHAVIOR REVIEW → ARCHITECTURE CHECK → PERFORMANCE CHECK → DOCS CHECK → SCORE → FIX → REPEAT`.

## Regras

- Antes de mudança significativa, reconciliar especificação e implementação real; não assumir que “existente” significa correto nem que “diferente” significa errado.
- Criar/fortalecer safety tests antes de refactor destrutivo.
- Executar `cargo fmt --check`, `cargo check`, testes relevantes e `cargo clippy` quando viável, além dos validators específicos do domínio afetado.
- Mudanças de UI exigem revisão de comportamento real, foco, keyboard navigation, resize/DPI, idiomas longos e screenshots reais.
- Hot paths exigem baseline e delta mensurável; otimização sem medição não eleva a qualidade.
- Save/import/autosave/jobs e outras fronteiras críticas devem usar failure injection quando aplicável.
- Nenhum novo hardcode público de texto, ícone, cor ou shortcut pode ser introduzido quando existir o sistema semântico correspondente.

## Scorecard

Pontuar de 0–10, sem inflação e com evidência, pelo menos: **Functional Correctness, Architecture/Decoupling, Data Integrity, Tests, UX/Accessibility, Performance, Failure Handling/Security e Documentation**. Registrar baseline e delta.

## Condição de saída

Uma feature/wave termina quando seus acceptance criteria estão satisfeitos e não restam P0/P1 dentro do escopo, salvo limitação explicitamente aceita e documentada com impacto, risco e próxima ação. Se uma etapa quebrar comportamento já correto, integridade do projeto ou boundary arquitetural, reverter o menor delta necessário e redimensionar a mudança.