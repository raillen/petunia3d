# 15 — Alternativas de Arquitetura-Alvo (Target Architecture Options)

> **Comparativo estruturado de alternativas arquiteturais, análise de prós e contras e recomendação pragmática para o Petunia3D.**

---

## 1. Comparativo de Três Estratégias de Arquitetura

Para orientar a evolução do projeto sem incorrer em dogmatismo ("Clean Architecture religiosa"), foram avaliadas três alternativas técnicas viáveis:

```text
┌─────────────────────────────────────────────────────────────────────────────────┐
│                     OPÇÃO A: Módulo Único / Crates Consolidados                 │
│  Reduzir crates; usar visibilidade Rust (pub(crate)) e módulos internos estritos. │
├─────────────────────────────────────────────────────────────────────────────────┤
│                     OPÇÃO B: Crates Estratificados Pragmáticos (Recomendada)    │
│  Manter limites físicos onde o compilador proíbe imports proibidos (Core -> UI).│
├─────────────────────────────────────────────────────────────────────────────────┤
│                     OPÇÃO C: DDD / Clean Architecture Corporativa Pesada        │
│  Interfaces para cada struct, DTOs intermediários em cada camada, Repositories. │
└─────────────────────────────────────────────────────────────────────────────────┘
```

| Critério Técnico | Opção A: Consolidação em Poucos Crates | Opção B: Crates Estratificados Pragmáticos | Opção C: DDD / Clean Architecture Pesada |
| :--- | :--- | :--- | :--- |
| **Garantia de Não-Regressão** | **Baixa**: Fácil reintroduzir `use egui` acidentalmente em submódulos | **Máxima**: Compilador bloqueia fisicamente dependências proibidas | **Alta**: Camadas abstratas bloqueiam vazamentos |
| **Desempenho em Tempo de Execução** | **Zero-Cost**: Chamadas diretas sem dynamic dispatch | **Zero-Cost**: Traits estáticas e chamadas diretas | **Penalidade**: Múltiplas alocações e cópias de DTOs |
| **Tempo de Compilação (Build)** | Médio (recompilação de crate maior a cada alteração) | **Rápido**: Crates de domínio (`mesh`, `project`) raramente recompilam | Lento (muitas macros e abstrações) |
| **Ergonomia Idiomática Rust** | Alta | **Altíssima** | Baixa (parece Java/C# transposto para Rust) |
| **Complexidade de Migração** | Alta (fundir arquivos e reorganizar) | **Gradual (Strangler)**: Extração incremental passo a passo | Altíssima (reescrita quase total) |

---

## 2. Recomendação Pragmática: Opção B (Estratificação Limpa)

A **Opção B** é a escolha recomendada porque aproveita a divisão de crates que o Petunia3D já possui, corrigindo apenas a direção das dependências e estabelecendo a camada de aplicação que hoje está ausente:

```mermaid
flowchart TD
    subgraph PRESENTATION ["Camada de Apresentação / Frontends (Substituíveis)"]
        UI_EGUI["petunia_ui_egui\n(Frontend Atual egui)"]
        UI_CLI["petunia_cli\n(Frontend Headless CLI / Prova de Desacoplamento)"]
        UI_FFI["petunia_ffi\n(Futura C-ABI para C++, C#, Go, Python)"]
    end

    subgraph APPLICATION ["Camada de Aplicação (Soberana e Neutra)"]
        APP_API["petunia_application\n(CommandDispatcher, EditorSession, Queries, DTOs, ProjectService)"]
    end

    subgraph CORE_ENGINE ["Núcleo do Editor (Agnóstico de UI)"]
        CORE["petunia_core\n(Selection, Camera, ModalOp, Picking, EventBus)\n*ZERO EGUI*"]
        CONFIG["petunia_config\n(Keybinds, Tokens de Tema puros, i18n)\n*ZERO EGUI*"]
        CMD["petunia_commands\n(Command Trait, CommandId, UndoStack)"]
    end

    subgraph DOMAIN ["Domínio Geométrico (100% Puro)"]
        MESH["petunia_mesh\n(B-Rep Half-Edge, Operadores Topológicos)"]
        PROJ["petunia_project\n(Project, Asset, Canvas, Serialization, Export)"]
    end

    subgraph RENDERER ["Infraestrutura de Renderização 3D"]
        RND["petunia_render\n(Shading, Stats, Caps)"]
        RND_W["petunia_render_wgpu\n(WebGPU Pipeline)"]
        RND_G["petunia_render_gl\n(OpenGL Pipeline)"]
    end

    %% Relações estritas de cima para baixo
    UI_EGUI --> APP_API
    UI_CLI --> APP_API
    UI_FFI --> APP_API

    APP_API --> CORE
    APP_API --> CONFIG
    APP_API --> CMD
    APP_API --> MESH
    APP_API --> PROJ

    CORE --> MESH
    CORE --> PROJ
    CORE --> CMD
    CORE --> CONFIG
    CORE --> RND

    RND_W --> PROJ
    RND_W --> MESH
    RND_W --> RND

    RND_G --> PROJ
    RND_G --> MESH
    RND_G --> RND
```

---

## 3. Benefícios Estratégicos da Arquitetura Proposta

1. **Compilação do Core 100% Livre de UI**:
   * `petunia_core` passa a ter como dependências apenas `glam`, `serde`, `uuid` e os crates de domínio (`mesh`, `project`, `commands`, `config`).
   * Um teste unitário ou script de automação pode instanciar uma sessão completa e operar sobre a geometria em microssegundos.
2. **Fronteira Única de Entrada de Mutações**:
   * Toda e qualquer alteração de estado entra por `dispatcher.dispatch(Command)`, garantindo que o histórico de undo/redo, o dirty flag da GPU e os eventos de notificação sejam disparados de forma determinística e consistente.
3. **Substituição Trivial de Frontend**:
   * Construir uma nova UI em `Slint`, `Iced` ou `Qt` exigirá apenas conectar os widgets aos comandos e queries expostos por `petunia_application`, sem necessidade de conhecer a matemática interna de malha ou loops de modal.
