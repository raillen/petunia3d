# 32 — ADR: Migração da Baseline Odin para Rust

<aside>
📝

**Architecture Decision Record — 2026-09-10.** Esta decisão substitui a stack anterior baseada em Odin como baseline de implementação do Petunia3D. As decisões funcionais do produto não mudam; muda a plataforma usada para implementá-las.

</aside>

# Status

**Adotada como FINAL BASELINE.** O vertical slice do capítulo 31 permanece obrigatório como teste de conformance/integração; ele não funciona mais como gate rotineiro de escolha da stack.

# Contexto anterior

A baseline anterior escolhia Odin para Application/Geometry Core, C/C++ providers para Manifold/xatlas/cgltf, Lua 5.4 para plugins e um sidecar Go para MCP. A escolha priorizava linguagem pequena, FFI C simples, controle de sistemas e aprendizado.

# Motivo da reabertura

A interface tornou-se o maior risco concreto do projeto. O Petunia exige:

- alta fidelidade ao design system definido no Figma;
- componentes desktop reais, não assets estáticos;
- accessibility semantics;
- keyboard/focus/HiDPI;
- viewport GPU integrado;
- workflow verificável por agentes de IA.

A pesquisa comparativa posterior colocou C++/Qt e uma stack Rust-native como finalistas. Rust ganhou relevância porque seu ecossistema atual cobre hoje grande parte das peças que anteriormente obrigariam C/C++/Go separados e porque egui/eframe/egui-wgpu oferecem uma camada desktop permissiva diretamente alinhada ao renderer wgpu.

# Decisão

Adotar como baseline de implementação:

```
Rust 2024
+ egui
+ eframe
+ egui-wgpu
+ wgpu
+ PetuniaMesh/Geometry Core próprio em Rust
+ manifold-rust
+ geo
+ xatlas fallback isolado
+ Serde/ZIP/glTF
+ mlua/Lua 5.4
+ rmcp
```

# Razões principais

## Homogeneidade

A aplicação passa a ter uma linguagem de implementação dominante. MCP deixa de exigir sidecar Go; Boolean pode usar port Rust; serialization, tooling, tests e build ficam integrados ao Cargo.

## Segurança

Ownership, borrowing, tipos, Result/Option e Send/Sync transformam parte importante da disciplina arquitetural em verificação do compilador.

## Desenvolvimento com IA

`cargo check/test/clippy` oferece feedback rápido e determinístico. egui possui `egui_kittest`, AccessKit e tooling de inspection/MCP para semantic tests, input e screenshots. Isso ajuda a atacar o problema histórico de agentes entregarem uma interface funcional porém distante da referência.

## Renderer

wgpu fornece backend moderno cross-platform sem exigir engine completa nem Vulkan cru.

## Extensibilidade

`mlua` e `rmcp` permitem manter Lua e MCP dentro da mesma plataforma Rust, preservando Application API comum.

# O que NÃO muda

Continuam válidos:

- shape-first/direct-mesh;
- half-edge authoring model;
- transactions/Undo;
- single-writer document;
- immutable worker snapshots/revision check;
- local operations before Boolean;
- Manifold semantics para Fuse/Cut complexo;
- xatlas como generic Auto UV fallback;
- `.petunia` ZIP + JSON;
- Lua como API pública de plugins;
- UI/Plugins/MCP sobre Command/Application API;
- Figma como visual truth, Notion como behavior truth, code como implementation truth.

# Consequências positivas

- menos linguagens no executável/ecossistema principal;
- Cargo como build/dependency/test hub;
- MCP Rust nativo;
- menor superfície manual de memory safety;
- melhor ergonomia de serialization;
- renderer independente e moderno;
- possibilidade de UI testing/inspection assistido por IA;
- boundaries entre crates testáveis.

# Consequências/risco

- egui é immediate-mode e exige disciplina para evitar estado de apresentação espalhado ou custo de CPU desnecessário;
- o Petunia precisa construir/possuir seu Design System e componentes recorrentes em Rust;
- crates auxiliares do ecossistema possuem ritmos de atualização diferentes e devem ficar atrás de adapters;
- xatlas permanece implementação C++ por baixo do wrapper;
- ecossistema de geometry processing acadêmico ainda é menor que C++;
- alguns algoritmos podem futuramente exigir provider nativo externo.

# Mitigação

- vertical slice de conformance obrigatório após o congelamento da baseline;
- qualquer bloqueador estrutural capaz de reabrir a stack exige evidência reproduzível + novo ADR;
- `petunia-ui` como adapter substituível;
- `UvUnwrapProvider` para xatlas;
- providers não vazam tipos externos;
- contract tests;
- pinning de versões;
- Geometry Core próprio preserva independência.

# Alternativa externa de contingência

A baseline atual está congelada. **C++23 + Qt Quick + QRhi** permanece apenas como referência externa de contingência. Só pode voltar à avaliação se um bloqueador estrutural reproduzível demonstrar que egui/eframe impede requisitos normativos de interface/acessibilidade/desktop; qualquer mudança exige novo ADR e não reabre Geometry/Core automaticamente.

# Regra documental

Qualquer página anterior que afirme “Odin é a linguagem principal”, “MCP usa sidecar Go” ou “UI toolkit está totalmente aberto” deve ser interpretada como **superseded por este ADR e pelos capítulos 27–31** quando houver conflito.

# Regra final

**A mudança de linguagem não autoriza mudar o produto. A stack serve ao Petunia; o Petunia não existe para justificar a stack.**