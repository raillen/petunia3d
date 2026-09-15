# Petunia3D — Auditoria Profunda de Arquitetura, Modularidade e Desacoplamento da Interface

> **Diagnóstico Arquitetural do Código Real, Catálogo de Acoplamentos e Plano Diretor de Desacoplamento (Gauntlet Loop)**  
> **Data:** 2026-09-13  
> **Status:** AUDITORIA CONCLUÍDA — PROIBIDA QUALQUER REFATORAÇÃO NESTA ETAPA

---

## 1. Sumário Executivo

Esta auditoria foi realizada com base **estritamente no código-fonte real** dos 15 crates do workspace do Petunia3D, ignorando afirmações teóricas da documentação que não encontram respaldo na implementação.

O objetivo futuro do projeto é transformar o Petunia3D em um **núcleo de modelagem 3D soberano e agnóstico de interface gráfica**, permitindo que o frontend atual em `egui` seja apenas uma das possíveis implementações visuais — viabilizando a substituição futura por frontends nativos em Rust (`Slint`, `Iced`, `Floem`, `GPUI`) ou em outras linguagens (`C++`, `C#`, `Go`, `Python`, `Web`) via FFI ou IPC.

### Resumo das Respostas Fundamentais

| Pergunta Crítica | Diagnóstico Real | Justificativa Sintética |
| :--- | :---: | :--- |
| **Se apagássemos hoje o egui, o app compilaria?** | **NÃO** | `petunia_core`, `petunia_config` e todos os 4 crates `module-*` dependem diretamente do `egui` em seus manifestos e tipos. |
| **A UI pode ser substituída hoje?** | **NÃO** | Mais de 1.000 mutações de estado e regras de negócio de ferramentas vivem diretamente em callbacks de widgets egui. |
| **O core pode rodar headless hoje?** | **PARCIALMENTE** | `petunia_mesh` e `petunia_project` são 100% headless, mas não existe uma struct de sessão/editor desacoplada de janela e egui. |
| **Uma UI em outra linguagem pode usar o core hoje?** | **NÃO** | Ausência total de C-ABI, FFI, DTOs neutros ou dispatcher de comandos por identificadores estáveis. |
| **Ferramentas são modulares e plugáveis?** | **PARCIALMENTE** | A trait `Tool` é apenas casca de UI; adicionar uma nova ferramenta completa exige alterar de 12 a 15 arquivos em 6 crates. |

---

## 2. Pontuação Arquitetural Global (Scorecard)

| Métrica Arquitetural | Nota | Veredito |
| :--- | :---: | :--- |
| **UI Independence** | **2.5 / 10** | Acoplamento severo: egui vaza para `core`, `config`, `modules` e `app`. |
| **Core Isolation** | **4.0 / 10** | `mesh` e `project` são isolados, mas `core` é contaminado por widgets e tipos egui. |
| **Application Boundary** | **2.0 / 10** | Não existe camada Application distinta; UI fala diretamente com `AppState`. |
| **Tool Modularity** | **3.5 / 10** | Trait `Tool` não executa; lógica real de ferramentas está dispersa no viewport egui. |
| **Command Architecture** | **2.0 / 10** | Não existem Commands semânticos; `commands` é apenas um buffer de snapshots de undo. |
| **Renderer Isolation** | **7.5 / 10** | Ponto forte: `render-wgpu` e `render-gl` não dependem de egui e aceitam renderização offscreen. |
| **State Ownership** | **3.0 / 10** | `AppState` é um God Object com 60+ campos misturando domínio, widgets e GPU. |
| **Dependency Direction** | **3.5 / 10** | Violação da inversão de dependência: módulos de domínio dependem de tipos de apresentação. |
| **Testability** | **6.5 / 10** | Bons testes unitários de mesh e core, mas 65% dos testes de interação dependem do egui. |
| **Headless Capability** | **5.0 / 10** | Operações de malha e projeto rodam sem GPU/UI, mas falta orquestrador de sessão headless. |
| **Cross-language Readiness** | **1.5 / 10** | Nenhum suporte a FFI, ponte C, DTOs serializáveis ou contratos de IPC. |
| **Plugin/Extension Readiness** | **3.0 / 10** | Adicionar recursos exige edição em cascata em enums fechados e múltiplos arquivos. |
| **Module Cohesion** | **4.5 / 10** | Módulos contêm algoritmos de domínio e widgets egui no mesmo arquivo. |
| **Replaceability** | **2.5 / 10** | Trocar egui hoje exigiria reescrever metade da base de código do editor. |
| **Maintainability** | **5.5 / 10** | Código limpo e bem formatado, mas com atrito alto de modificação estrutural. |
| **MÉDIA GERAL** | **3.8 / 10** | **ARQUITETURA FORTEMENTE ACOPLADA AO FRONTEND EGUI** |

---

## 3. Os 5 Maiores Riscos Arquiteturais Identificados

1. **`AppState` como God Object (`crates/core/src/state.rs`)**:
   Uma única estrutura acumula mais de 60 campos misturando persistência (`Project`), histórico de undo, seleção, parâmetros voláteis de ferramentas (`extrude_dist`, `inset_factor`), estado transitório de widgets (`outliner_search`, `properties_tab`, `show_settings`), telemetria e identificadores de temas/ícones, além de conter diretamente tipos do egui (`egui::Rect`, `egui::TextureHandle`).
2. **Ausência de Camada de Comandos e Dispatcher Semântico**:
   O crate `petunia_commands` é apenas um wrapper genérico de `UndoStack<Project>`. Não existem `CommandId`, `CommandDispatcher` ou objetos de comando. Mutações são executadas de forma ad-hoc por dezenas de callbacks de UI que manipulam malhas e gravam checkpoints manualmente.
3. **Trait `Tool` Ilusória e Lógica Espalhada (`crates/module-model`)**:
   A interface `Tool` só fornece metadados textuais e um método `ui(&self, &egui::Context, &mut egui::Ui, &mut AppState)`. As ferramentas interativas reais (Move, Rotate, Scale, Extrude, Inset, Bevel, Loop Cut, Knife, Slice, Annotation, Measure) têm seus loops e máquinas de estado implementados dentro de widgets egui em `crates/ui`.
4. **Vazamento de Dependências de UI no Núcleo e Módulos**:
   `crates/core`, `crates/config`, `crates/module-model`, `crates/module-paint`, `crates/module-uv` e `crates/module-assets` trazem `egui = { workspace = true }` em seus `Cargo.toml`, impedindo a compilação do núcleo sem a biblioteca gráfica.
5. **Acoplamento de I/O e Diálogos de Sistema na Camada de Apresentação**:
   Carregamento de projeto, salvamento, importação de OBJ e exportação são iniciados dentro de funções da UI que abrem `rfd::FileDialog` ou `egui-file-dialog` e realizam a mutação direta de múltiplos campos de `AppState` sem um serviço de aplicação intermediário.

---

## 4. Estrutura do Relatório de Auditoria

A documentação detalhada desta auditoria está dividida nos seguintes relatórios técnicos:

* [`01-current-architecture.md`](./01-current-architecture.md): Mapeamento físico de crates, camadas reais e diagramas de fluxo de execução.
* [`02-dependency-analysis.md`](./02-dependency-analysis.md): Grafo de dependências real, violações de direção e análise de ciclos.
* [`03-ui-coupling.md`](./03-ui-coupling.md): Inventário de imports e tipos egui vazando para o domínio e regras de negócio na UI.
* [`04-state-ownership.md`](./04-state-ownership.md): Anatomia do God State, segregação necessária e ciclo de vida de dados.
* [`05-command-system.md`](./05-command-system.md): Avaliação do sistema de comandos atual vs requisitos de desacoplamento.
* [`06-tool-system.md`](./06-tool-system.md): Ciclo de vida real de ferramentas, máquinas de estado e rastreio de adição de ferramentas.
* [`07-renderer-boundary.md`](./07-renderer-boundary.md): Isolamento dos backends wgpu e OpenGL e capacidade de renderização offscreen.
* [`08-project-assets-io.md`](./08-project-assets-io.md): Fronteiras de I/O de arquivos, serialização, assets e desacoplamento de file pickers.
* [`09-modularity-extension-cost.md`](./09-modularity-extension-cost.md): Matriz quantitativa de custo de extensão e hotspots de acoplamento.
* [`10-headless-readiness.md`](./10-headless-readiness.md): Diagnóstico de prontidão para execução e testes headless.
* [`11-cross-language-ui.md`](./11-cross-language-ui.md): Análise técnica de viabilidade para frontends em C++, C#, Go, Python e Web.
* [`12-testing-gaps.md`](./12-testing-gaps.md): Distribuição da suíte de testes e lacunas de cobertura sem UI.
* [`13-findings.md`](./13-findings.md): Catálogo padronizado de achados de auditoria (P0 a P3).
* [`14-scorecard.md`](./14-scorecard.md): Scorecard detalhado com critérios de evidência técnica.
* [`15-target-architecture-options.md`](./15-target-architecture-options.md): Comparativo de arquiteturas-alvo e recomendação pragmática.
* [`16-remediation-plan.md`](./16-remediation-plan.md): Plano Diretor estruturado em Gauntlet Loops incrementais (G0 a G10).

---

## 5. Regra Estrita de Execução

> [!CAUTION]
> **NENHUMA REFATORAÇÃO DEVE SER INICIADA NESTA ETAPA.**  
> Esta auditoria e o plano correspondente devem ser revisados e validados pelo usuário antes de qualquer alteração estrutural no código de produção.
