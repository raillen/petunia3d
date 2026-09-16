# Auditoria de Conformidade Documental — `docs/` vs Livro Vivo

> **Data:** 2026-09-16 · **Autoridade:** `docs/bible/` (Petunia3D — Livro Vivo, 249 páginas)
> **Classificação:** auditoria de documentação (não é uma segunda autoridade documental)
> **Estado:** sessões 0–8 executadas · divergências de **código** registradas na seção 4

## Método

1. Inventário mecânico completo do caderno (249 páginas) e de `docs/` (496 `.md` fora de `node_modules`).
2. Comparação página-a-página entre o espelho antigo e o caderno.
3. Leitura integral das páginas normativas de documentação: `00`, `01`, `02`, `05`, `06`, `12`, `13`, `36`, `M` e o hub de Readiness.
4. Amostragem auditada dos epics A–O e do catálogo P3D, com foco em P3D-084–125.
5. Verificação cruzada com o código (`Cargo.toml`, `crates/xtask`, `.github/workflows`, `assets/`).

Status (ch. 00/01): `COMPLIANT`, `PARTIALLY_COMPLIANT`, `FUNCTIONAL_BUT_DIFFERENT`, `RUDIMENTARY`, `STUB`, `BROKEN`, `DUPLICATED`, `MISSING`, `OBSOLETE`.

## 1. Resultado

| Métrica | Antes | Depois |
| --- | --- | --- |
| Local canônico do caderno | dois (raiz + espelho divergente) | **um** (`docs/bible/`) |
| Páginas canônicas | 229 (espelho incompleto) | **249** + índice de status |
| Capítulos presentes | 00–16 · 01–36 | 00–16 · **01–44** |
| Specs P3D presentes | 001–155 | **001–168** |
| Links internos quebrados | 1 arquivo + 2 caminhos mortos em 9 arquivos | 0 (406 links verificados) |
| Gates de documento na CI | 0 | **2** (`bible-check`, `docs-generate --check`) |
| Termos de usuário conformes | não | sim (gate bloqueia regressão) |
| Site público | sem regra | **congelado e verificado por hash** |

## 2. Sessões executadas

### Sessão 0 — Fonte única e completude

| Item | Status final | Evidência |
| --- | --- | --- |
| Caderno duplicado em dois locais | `COMPLIANT` | migração para `docs/bible/`; origem removida |
| Espelho sem a consolidação de 2026-09-15 | `COMPLIANT` | páginas substituídas pelo texto canônico |
| Capítulos 37–44 | `COMPLIANT` | `foundations/37-…` a `foundations/44-…` |
| Specs P3D-156–168 | `COMPLIANT` | `specs/p3d-156-…` a `specs/p3d-168-…` |
| Raiz do Livro Vivo + hub de Readiness | `COMPLIANT` | `index.md` e `especificacoes-p3d-readiness-gauntlet-waves.md` |
| Links internos do caderno | `COMPLIANT` | 406 links reescritos para caminhos relativos |
| Status de implementação do espelho antigo | preservado | `docs/bible/status/p3d-implementation-status.md` (rotulado como operacional, não autoridade) |
| `PRUMO.md`, `contracts/bindings.json`, ADRs | `COMPLIANT` | caminhos canônicos corrigidos |
| `constitution/00–07` (linhagem histórica) | `COMPLIANT` | preservados na íntegra; o hub declara que não formam autoridade paralela |

### Sessão 1 — Governança e política

`docs/contributing/documentation-policy.md` reescrito: fonte única, hierarquia de
autoridade (32 → 34 → 27–31/35/36 → 09 → 21 → 12 → 14–20 → 22–26 → especializados → 07),
marcadores de status, delta check obrigatório, vocabulário, screenshots reais, regra do
site congelado e comandos de verificação. Status: `COMPLIANT`.

### Sessão 2 — Produto e escopo

| Item | Status final |
| --- | --- |
| Contagem de crates (13/14/17) | `COMPLIANT` — **19 membros**, com a lista real em `scope.md`, `ARCHITECTURE.md` e `developers/architecture.md` |
| Inspirações do produto (MoI, Plasticity, Blockbench, Blender) | `COMPLIANT` — `Wings3D` removido como referência estrutural |
| Temas nativos anunciados | `COMPLIANT` na documentação — Dark oficial + High Contrast; extras registrados como divergência |
| Pacotes de ícones | `COMPLIANT` — Tabler, Iconoir, Phosphor, Lucide + Petunia Custom |
| Perfis de atalhos | `COMPLIANT` — os 8 presets nomeados |
| Fluxo canônico `Reference → Draw/Create → Shape → Paint/Project → Check → Export` | `COMPLIANT` |
| Escopo V1 (Capsule, Round Edge 1 segmento, Loft fora, Simple Sweep V1.x, FBX fora) | `COMPLIANT` |
| Nós/procedurais ("sem nós") | `COMPLIANT` — separado em simples (P3D-113/164, pós-V1) vs `Out of Scope` (Geometry Nodes completo) |
| Renderer/stack declarada | `COMPLIANT` na documentação — egui-wgpu/wgpu como baseline; GL como compatibilidade |

### Sessão 3 — Vocabulário e UX tutorial-first

| Item | Status final |
| --- | --- |
| "Modo de Edição" como conceito | `COMPLIANT` — substituído por domínios `Object / Face / Edge / Point` |
| "Vértice/Aresta" como rótulo de interface | `COMPLIANT` nos manuais e tutoriais |
| "Chanfro" | `COMPLIANT` — **Round Edge** (amigável) / Bevel (técnico) |
| Booleans expostos como paradigma | `COMPLIANT` — **Fuse** / **Cut** |
| Profile como desenho 2D | `COMPLIANT` — Work Plane no espaço 3D + dois caminhos válidos |
| Ordem de UV (automação antes do editor manual) | `COMPLIANT` |
| Glifos/emoji como pseudo-ícones | `COMPLIANT` — removidos de `manual/`, `getting-started/`, `tools/` |
| Gate anti-regressão | `bible-check` bloqueia os termos acima e 17 glifos |

### Sessão 4 — UI Baseline Final V1

| Item | Status final |
| --- | --- |
| `ui/README.md` canonizava `Blender.svg` e menus de DCC genérico | `COMPLIANT` — reescrito para o capítulo 36; mockups rebaixados a `FIGMA-CONFIRMED` (evidência, nunca requisito) |
| Workspaces | `COMPLIANT` — `MODEL / PAINT / UV`; `EXPORT` e `Animation` explicitamente fora |
| Medidas do shell | `COMPLIANT` — top bar 40, Parts 248, Context 288, Asset Library 176, header 28, control 28, status ~22, gutter 8, breakpoints |
| Design tokens finais | `COMPLIANT` — tipografia 10–14, radius 4/5/8/10/full, accent `#B58CFF`, motion 80–180 ms + reduced motion |
| Theme Extension API | `COMPLIANT` — `.petunia-theme`, herança, tokens extensíveis/protegidos, Theme Manager |
| Plugin Panels | `COMPLIANT` — Panel Registry, extension slots, Petunia UI Extension API, capabilities, isolamento |
| Input/foco/acessibilidade | `COMPLIANT` — tabela de input baseline, `F6`/`Shift+F6`, AccessKit como contrato |
| `shortcuts/cheatsheet.md` (mapa Blender como canônico) | `COMPLIANT` — agora **gerado** do perfil `petunia-default`; drift impossível |
| Customização (temas/keymaps/ícones) | `COMPLIANT` — 8 presets oficiais, tokens, packs oficiais |
| Referências visuais arquivadas | `COMPLIANT` — rotuladas como não normativas |

### Sessão 5 — Superfícies de documentação e QA

| Item | Status final |
| --- | --- |
| P3D-116 website | `COMPLIANT` (build + deploy existem); publicação congelada por decisão |
| P3D-117 changelog vivo | `COMPLIANT` — sincronizado por xtask; migration notes exigidas na política |
| P3D-118 screenshots reais | `PARTIALLY_COMPLIANT` — regra publicada e mockups rebaixados; **capturas reais ainda pendentes** |
| P3D-119 referências geradas | `COMPLIANT` — 8 arquivos gerados; tabela canônica do keymap default que estava **vazia** foi corrigida |
| P3D-120 docs check | `COMPLIANT` — `docs-check` inclui o caderno; `bible-check` e `docs-generate --check` rodam na CI |
| P3D-121/123/124/125 | `PARTIALLY_COMPLIANT` — cobertura descrita em `development/testing-strategy.md`, execução por feature |
| P3D-122 architecture checks | `COMPLIANT` — `arch-check` existe e está no gate |

### Sessão 6 — Arquitetura e docs de desenvolvedor

| Item | Status final |
| --- | --- |
| Grafo/contagem de crates | `COMPLIANT` — 19 membros declarados, capítulos 28/34 como autoridade |
| `developers/plugins.md` ensinava `Module::draw_ui(&egui::Context)` no core | `COMPLIANT` — reescrito para Lua + capabilities + Petunia UI Extension API; nota explícita da correção |
| `contributing/adding-a-tool.md` (ícone unicode + tecla física no Tool) | `COMPLIANT` — cadeia `Tool → Command → Algorithm → Data`, `IconId`/`TextId`, bind no keymap, checklist de invariantes |
| Tool→Command→Algorithm→Data, transactions, single-writer | `COMPLIANT` — documentado |
| Plugin API / MCP | `COMPLIANT` — capabilities, panel registry, adapter sobre Application API |

### Sessão 7 — Evidências e auditoria

Este documento é a **Implementation-vs-Spec Gap Matrix** de documentação. O scorecard de
8 eixos do capítulo 02 permanece em `docs/GAUNTLET.md` (a ser reconciliado quando a
próxima wave for validada). `docs/modernization/*` e os arquivos
`PETUNIA3D_*_GAUNTLET.md` são **evidência histórica**, não autoridade paralela.

### Sessão 8 — Gates automatizados

| Gate | Implementação |
| --- | --- |
| Completude do caderno | `bible-check` valida 00–16, 01–44, P3D-001–168, A–O, adendos e páginas de entrada |
| Integridade de links | 406 links internos verificados; falha com lista dos quebrados |
| Caminhos mortos | falha em referências a pastas de caderno extintas |
| Vocabulário de usuário | falha em termos técnicos primários e glifos usados como ícone |
| Site congelado | `frozen-site.lock.json` (814 arquivos) com detecção de qualquer alteração |
| Drift de referências | `docs-generate --check` (catálogos + cheatsheet + changelog) |
| CI | `.github/workflows/bible.yml` (independente do deploy congelado) |

## 3. 🧊 Dívida do site congelado (a resolver no descongelamento)

O site público está congelado por decisão explícita (`AGENTS.md` §1) e **não** foi
alterado. Itens conhecidos, para quando o descongelamento acontecer:

| # | Pendência | Ação no descongelamento |
| --- | --- | --- |
| F1 | `docs/index.md` (hero) declara "inspirada no Blender e Wings3D", "4 temas nativos (Dark, Light, Capuccino, Tokyo Nights)", "5 pacotes de ícones" e um fluxo por "Modo de Edição" | reescrever conforme capítulo 36 e `product/vision.md` |
| F2 | `.vitepress/bibleSidebar.ts` termina em P3D-155 | regenerar a navegação do caderno (249 páginas) |
| F3 | Capturas reais inexistentes; `image-references/` contém mockups | gerar screenshots da implementação (P3D-118) |
| F4 | `docs/image-references/extracted/*` promove elementos do mockup a catálogo | rebaixar a evidência ou substituir por arte própria |
| F5 | Site não versiona a documentação | definir versionamento e busca |

Modificar qualquer arquivo da lista congelada **quebra o `bible-check`** por design.
Para descongelar: decisão explícita + `cargo run -p xtask -- bible-lock`.

## 4. Divergências de **código** registradas (fora do escopo documental)

Estas não foram corrigidas nesta rodada porque exigem mudança de implementação, não de
documentação. A documentação descreve o **contrato** e registra a divergência.

| # | Divergência | Contrato | Impacto | Próxima ação |
| --- | --- | --- | --- | --- |
| C1 | `Workspace::Animate` existe no build (4 workspaces + pill) | cap. 36: `MODEL / PAINT / UV` | usuário vê uma superfície fora da V1 | remover da V1 **ou** reclassificar por decisão explícita |
| C2 | Temas extras embarcam (Light, Capuccino, Tokyo Nights) | cap. 36: Dark oficial + High Contrast; demais via `.petunia-theme` | paletas específicas expostas como baseline | reclassificar como pacotes de exemplo/usar a Theme Extension API |
| C3 | Pack `future-dark` de ícones | P3D-087/088: Tabler, Iconoir, Phosphor, Lucide + Petunia Custom | pack extra sem contrato | remover ou promover com decisão |
| C4 | Rótulo técnico `select.domain_vertex` / "Vertex Domain" | cap. 13: **Point** na linguagem de usuário | `TextId` do comando mostra termo técnico na UI | ajustar o `TextId` (IDs técnicos permanecem estáveis) |
| C5 | `EditMode` rígido como porta de entrada | P3D-015: modelo unificado e contextual | fluxo ainda exige alternância de modo | convergir para domínios contextuais |
| C6 | `assets/keybinds/petunia.toml` duplica o default de `assets/keymaps/petunia-default.toml` | ch. 00: consolidar `DUPLICATED` | duas verdades de keymap (a documentação já aponta a canônica) | transformar em exemplo ou remover |
| C7 | Pipeline OpenGL-first no código | cap. 27/36: egui-wgpu/wgpu é o baseline | divergência de renderer | convergir o viewport para o adapter |
| C8 | `docs/GAUNTLET.md` ainda usa a contagem antiga de crates e cita OpenGL como padrão | cap. 27/28 | evidência histórica com números velhos | reconciliar quando a próxima wave for validada |

## 5. Como verificar

```bash
cargo run -p xtask -- bible-check           # caderno, links, vocabulário, site congelado
cargo run -p xtask -- docs-generate --check # drift de catálogos, cheatsheet e changelog
cargo run -p xtask -- docs-check            # integridade + mapa de UI + build do site
```

Falha em `bible-check` significa: ou o caderno ficou incompleto, ou um link quebrou, ou
alguém trabalhou no site congelado.
