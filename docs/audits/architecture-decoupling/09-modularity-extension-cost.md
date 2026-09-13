# 09 — Modularidade e Custo de Extensão (Modularity & Extension Cost)

> **Matriz quantitativa de atrito para adição de novos recursos, análise do Princípio Aberto/Fechado (OCP) e catálogo de hotspots de acoplamento.**

---

## 1. Matriz Quantitativa de Custo de Extensão

Para mensurar a modularidade real do Petunia3D, calculou-se a quantidade de arquivos e crates que precisam ser modificados para cada tipo de extensão:

| Tarefa de Extensão | Dificuldade Atual | Arquivos Modificados | Crates Tocados | Risco Arquitetural | Dificuldade Desejada |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **Adicionar Nova Ferramenta** | **MUITO ALTA** | **12 a 15** | `module-model`, `config`, `ui`, `app`, `core` | **ALTO** (Cascata de enums e matches) | **BAIXA** (1 arquivo de tool + registro) |
| **Adicionar Novo Comando** | **ALTA** | **5 a 7** | `app`, `ui` (múltiplos painéis), `config` | **MÉDIO** (Duplicação de regras e undo) | **BAIXA** (1 struct de comando + handler) |
| **Adicionar Nova Primitiva 3D** | **MÉDIA** | **4** | `mesh`, `module-model`, `ui` (bar e outliner) | **BAIXO** (Algoritmo isolado em mesh) | **BAIXA** (1 gerador geométrico + registro) |
| **Adicionar Novo Exportador** | **BAIXA** | **3** | `project`, `ui` (export panel) | **BAIXO** (Exportadores são desacoplados) | **BAIXA** (1 módulo de exportação) |
| **Adicionar Novo Workspace** | **ALTA** | **6** | `core`, `ui` (header, bar, shelf, tiles), `app` | **MÉDIO** (Quebra de layout de janelas) | **MÉDIA** (Configuração declarativa de painéis) |
| **Substituir o Frontend (egui)**| **CRÍTICA** | **25+** | Quase todos os crates do workspace | **MÁXIMO** (Lógica de negócio embutida na UI) | **BAIXA** (Frontend consome Application API) |

---

## 2. Catálogo dos Hotspots de Acoplamento

Identificou-se os cinco arquivos com maior concentração de responsabilidades, dependentes e volume de chamadas cruzadas:

### 1. `crates/ui/src/outliner.rs` (71 KB, ~1.500 LOC)
* **Responsabilidades Múltiplas**:
  * Renderiza a árvore com `egui_ltreeview`;
  * Contém 179 acessos diretos a `state.*`;
  * Executa mutações de domínio: duplicação, exclusão e renomeação de objetos, anotações, medições e imagens de referência;
  * Gerencia menus de contexto RMB para 5 tipos de entidades;
  * Instancia primitivas na cena;
  * Emite checkpoints de undo manualmente.

### 2. `crates/ui/src/viewport_bar.rs` (34 KB, ~800 LOC)
* **Responsabilidades Múltiplas**:
  * 157 acessos diretos a `state.*`;
  * Orquestra 7 clusters funcionais: modo de interação, modos de seleção, travamento de eixos, projeção de câmera, snapping, overlays e esferas de sombreamento;
  * Constrói menus suspensos (`PetuniaMenuItem`) que executam mutações diretas em `AppState`.

### 3. `crates/ui/src/properties_panel.rs` (35 KB, ~850 LOC)
* **Responsabilidades Múltiplas**:
  * 133 acessos diretos a `state.*`;
  * Inspetor detalhado de Objetos, Malhas, Anotações, Medições e Materiais;
  * Realiza mutações contínuas de Transform (Location, Rotation, Scale);
  * Implementa lógica de exclusão destrutiva de malha (`mesh.delete_selected()`).

### 4. `crates/app/src/lib.rs` (68 KB, 1.855 LOC)
* **Responsabilidades Múltiplas**:
  * Inicialização da biblioteca de janela `winit`;
  * Inicialização de contexto e superfícies gráficas (WebGPU / OpenGL glow);
  * Loop de eventos principal;
  * Despacho manual de teclado físico para strings de ação (`Core::on_key`);
  * Orquestração de renderização 3D e passe de UI do egui;
  * Gerenciamento de redimensionamento e encerramento.

### 5. `crates/core/src/state.rs` (25 KB, ~750 LOC)
* **Responsabilidades Múltiplas**:
  * God Object acumulando 60+ campos de cinco camadas arquiteturais diferentes;
  * Contém tipos visuais diretos do egui (`Rect`, `TextureHandle`).

---

## 3. Avaliação do Princípio Aberto/Fechado (Open/Closed Principle)

O código atual **viola o Princípio Aberto/Fechado** de forma consistente:
* Para adicionar uma nova ferramenta ou modo de seleção, o sistema exige a alteração de enums fechados (`EditMode`, `ModalKind`, `PetuniaIcon`), casamentos em match statements espalhados (`on_key`, `viewport_bar`, `contextual_shelf`), e edição direta de dezenas de arquivos existentes.
* Não existe um modelo orientado a registro ou injeção onde um novo recurso possa ser introduzido simplesmente declarando um struct e registrando-o em tempo de compilação ou inicialização sem tocar no código alheio.
