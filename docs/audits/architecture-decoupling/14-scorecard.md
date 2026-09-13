# 14 — Placar de Maturidade Arquitetural (Architecture Scorecard)

> **Avaliação quantitativa de 0 a 10 para 15 dimensões arquiteturais essenciais, com justificativas técnicas detalhadas baseadas nas evidências da auditoria.**

---

## 1. Tabela Resumo do Scorecard

```text
================================================================================
DIMENSÃO ARQUITETURAL                NOTA      STATUS
================================================================================
1.  UI Independence                  2.5 / 10  Crítica (Acoplamento Severo)
2.  Core Isolation                   4.0 / 10  Deficiente (egui presente no core)
3.  Application Boundary             2.0 / 10  Crítica (Inexistente)
4.  Tool Modularity                  3.5 / 10  Deficiente (Lógica dispersa na UI)
5.  Command Architecture             2.0 / 10  Crítica (Sem commands/dispatcher)
6.  Renderer Isolation               7.5 / 10  Forte (wgpu desacoplado da UI)
7.  State Ownership                  3.0 / 10  Deficiente (God Object AppState)
8.  Dependency Direction             3.5 / 10  Deficiente (Inversão violada)
9.  Testability                      6.5 / 10  Moderada (Testes bons, mas presos à UI)
10. Headless Capability              5.0 / 10  Parcial (Mesh/Project puros, Core preso)
11. Cross-language Readiness         1.5 / 10  Crítica (Sem FFI ou C-ABI)
12. Plugin/Extension Readiness       3.0 / 10  Deficiente (Atrito de 15 arquivos)
13. Module Cohesion                  4.5 / 10  Deficiente (Domínio e UI misturados)
14. Replaceability                   2.5 / 10  Crítica (Impossível trocar egui hoje)
15. Maintainability                  5.5 / 10  Moderada (Código limpo, atrito alto)
================================================================================
MÉDIA ARQUITETURAL GERAL:            3.8 / 10  ARQUITETURA ALTAMENTE ACOPLADA À UI
================================================================================
```

---

## 2. Justificativas Técnicas por Critério

### 1. UI Independence — Nota: 2.5 / 10
* **Justificativa**: O `egui` não está confinado ao crate de apresentação. Está presente nos manifestos de `petunia_core`, `petunia_config` e nos 4 crates `module-*`. Mutações de topologia de malha e gravação de checkpoints de histórico são chamadas diretamente de closures em widgets da UI. Se apagássemos o egui hoje, quase nenhum crate compilaria.

### 2. Core Isolation — Nota: 4.0 / 10
* **Justificativa**: A nota é salva pela pureza do `petunia_mesh` e `petunia_project`, que não contêm contaminações visuais. Porém, o `petunia_core` traz tipos gráficos (`egui::Rect`, `egui::TextureHandle`) e traits com métodos de apresentação (`Module::ui`), falhando na garantia de isolamento do núcleo.

### 3. Application Boundary — Nota: 2.0 / 10
* **Justificativa**: Não existe uma camada intermediária de aplicação ou casos de uso entre a UI e o Core. A UI manipula diretamente o `AppState` sem nenhuma barreira de validação, DTOs ou serviços orquestradores.

### 4. Tool Modularity — Nota: 3.5 / 10
* **Justificativa**: A trait `Tool` é puramente cosmética (declara id, textos e um método `ui`). As ferramentas interativas reais (Move, Rotate, Scale, Extrude, Loop Cut, Knife, Slice, Annotate, Measure) têm suas máquinas de estado codificadas dentro de widgets egui em `crates/ui` e `modal.rs`.

### 5. Command Architecture — Nota: 2.0 / 10
* **Justificativa**: O crate `petunia_commands` é apenas uma pilha genérica de snapshots (`UndoStack<Project>`). Não existem comandos semânticos, identificadores estáveis ou barramento de despacho. A UI decide os rótulos de undo e chama `checkpoint()` de forma descentralizada em mais de 20 lugares.

### 6. Renderer Isolation — Nota: 7.5 / 10
* **Justificativa**: Ponto alto do projeto. O `petunia_render_wgpu` é uma implementação pura em `wgpu` v25, sem nenhuma dependência de egui, e já é plenamente capaz de renderizar cenas offscreen ou em texturas compartilhadas. A única perda de pontos deve-se ao entrelaçamento do loop de redraw da janela com a UI no `petunia_app`.

### 7. State Ownership — Nota: 3.0 / 10
* **Justificativa**: O `AppState` é um God Object com 60+ campos públicos misturando domínio, parâmetros voláteis de ferramentas, estado transitório de widgets, contadores de GPU e flags de janela. Evita nota zero pelo fato de não utilizar variáveis globais estáticas (`static mut`).

### 8. Dependency Direction — Nota: 3.5 / 10
* **Justificativa**: A regra de dependência esperada (`UI -> Application -> Core`) está invertida em múltiplos pontos (`Core -> egui`, `Config -> egui`, `Modules -> egui`), criando acoplamentos circulares conceituais.

### 9. Testability — Nota: 6.5 / 10
* **Justificativa**: O projeto conta com 198 testes automatizados aprovados e sem flakes. Porém, 88 desses testes (44%) estão em `petunia_ui`, simulando eventos de mouse no egui para testar comportamentos de domínio que deveriam ser testados em nível de sessão headless.

### 10. Headless Capability — Nota: 5.0 / 10
* **Justificativa**: Algoritmos de malha e serialização de arquivos rodam 100% sem interface gráfica. Contudo, não é possível instanciar um editor headless unificado devido à dependência do `egui` no `petunia_core` e ao confinamento do `Core` no `petunia_app`.

### 11. Cross-language Readiness — Nota: 1.5 / 10
* **Justificativa**: O sistema não possui FFI, bindings C, tipos de dados de ABI estável, DTOs serializáveis nem contratos de comunicação entre processos.

### 12. Plugin / Extension Readiness — Nota: 3.0 / 10
* **Justificativa**: A extensão do editor é rígida. Para adicionar uma nova ferramenta completa com paridade aos recursos existentes, é necessário editar de 12 a 15 arquivos espalhados por 6 crates.

### 13. Module Cohesion — Nota: 4.5 / 10
* **Justificativa**: Crates como `module-model` e `module-paint` possuem baixa coesão interna, agrupando no mesmo arquivo regras de edição de vértices e desenho de sliders egui.

### 14. Replaceability — Nota: 2.5 / 10
* **Justificativa**: Substituir o frontend atual por outro toolkit gráfico (como Qt ou Slint) hoje exigiria a reescrita ou extração manual de mais de 1.000 pontos de mutação e todas as sessões de ferramentas interativas.

### 15. Maintainability — Nota: 5.5 / 10
* **Justificativa**: O código é limpo, bem documentado, formata 100% sob `cargo fmt` e compila sem nenhum aviso sob `cargo clippy -D warnings`. Contudo, o atrito arquitetural para alterações estruturais é elevado.
