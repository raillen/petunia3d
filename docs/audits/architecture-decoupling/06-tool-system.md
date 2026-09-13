# 06 — Sistema e Ciclo de Vida de Ferramentas (Tool System Report)

> **Auditoria da trait `Tool`, rastreamento do ciclo de vida das ferramentas representativas e cálculo do custo de extensão.**

---

## 1. A Trait `Tool` Atual (`crates/module-model/src/lib.rs`)

O contrato que define uma ferramenta de modelagem no Petunia3D é:

```rust
pub trait Tool {
    fn id(&self) -> &'static str;
    fn label_key(&self) -> &'static str;
    fn hint_key(&self) -> &'static str;
    fn icon(&self) -> &'static str;
    fn shortcut(&self) -> &'static str { "" }
    fn ui(&self, _ctx: &egui::Context, _ui: &mut egui::Ui, _state: &mut AppState) {}
    fn on_activate(&self, _state: &mut AppState) {}
}
```

### Análise Crítica do Contrato:
1. **Ausência de Execução**: A trait não possui nenhum método de execução topológica (`execute()`, `apply()`, `commit()`).
2. **Ausência de Ciclo de Vida de Interação**: Não existem conceitos de `begin()`, `update()`, `confirm()`, `cancel()`.
3. **Ausência de Entrada de Dados**: A ferramenta não recebe eventos de ponteiro, raios de picking, coordenadas de tela nem gestos do usuário.
4. **Acoplamento Mandatório com egui**: O único método funcional relevante é `ui()`, que obriga qualquer implementação de ferramenta a depender do `egui::Context` e `egui::Ui` para desenhar sliders na barra lateral.

---

## 2. Onde as Ferramentas Reais Estão Implementadas

Como a trait `Tool` é apenas uma casca de metadados para a barra lateral, as ferramentas interativas reais foram implementadas **fora do sistema de ferramentas**, dispersas pela camada de interface e pelo núcleo:

| Ferramenta | Onde Vive a Máquina de Estado Interativa? | Onde Vive o Algoritmo Geométrico? | Onde é Desenhado o Feedback Visual? |
| :--- | :--- | :--- | :--- |
| **Select (Vértice / Aresta / Face)** | `crates/ui/src/viewport_interaction.rs` | `crates/mesh/src/lib.rs` | `egui::Painter` em `viewport_interaction.rs` |
| **Move / Rotate / Scale** | `crates/ui/src/modal_viewport.rs` | `crates/core/src/modal.rs` | Linhas-guia e HUD em `modal_viewport.rs` |
| **Transform Gizmo (3D)** | `crates/ui/src/transform_gizmo_integration.rs` | `crates/core/src/state.rs` | `transform-gizmo-egui` |
| **Extrude** | Dividida: `module-model/src/extrude.rs` (sliders) e `modal_viewport.rs` (mouse) | `crates/mesh/src/ops.rs` | `modal_viewport.rs` |
| **Loop Cut** | `crates/ui/src/cutting.rs` (`CutSession`) | `crates/mesh/src/loop_cut.rs` | `egui::Painter` em `cutting.rs` |
| **Knife** | `crates/ui/src/cutting.rs` (`CutSession`) | `crates/mesh/src/knife.rs` | `egui::Painter` em `cutting.rs` |
| **Slice Plane** | `crates/ui/src/cutting.rs` (`CutSession`) | `crates/mesh/src/ops.rs` | `egui::Painter` em `cutting.rs` |
| **Draw Profile** | `crates/module-model/src/draw_profile.rs` | `crates/mesh/src/ops.rs` | `viewport_interaction.rs` |
| **Paint Brush** | `crates/ui/src/viewport_interaction.rs` | `crates/core/src/state.rs:paint_at` | Círculo projetado em `viewport_interaction.rs` |
| **Measure (Régua 3D)** | `crates/ui/src/measurement.rs` | `crates/project/src/lib.rs` | `egui::Painter` em `measurement.rs` |
| **Annotate (Lápis 3D)** | `crates/ui/src/annotation.rs` | `crates/project/src/lib.rs` | `egui::Painter` em `annotation.rs` |

### Diagnóstico de Fragmentação:
Não existe uma abstração uniforme de "Ferramenta". Ferramentas baseadas em corte usam `cutting.rs`; ferramentas baseadas em transformação usam `modal.rs`; medição e anotação usam módulos visuais próprios em `crates/ui`; e ferramentas simples usam callbacks avulsos na barra lateral de `module-model`.

---

## 3. Experimento Prático: Custo de Extensão de uma Nova Ferramenta

Para responder à pergunta da auditoria:
> **"Para adicionar hoje `Tool::MyNewTool`, quantos lugares precisam ser modificados?"**

Rastreou-se o fluxo completo necessário para introduzir uma ferramenta fictícia com paridade funcional ao restante do editor:

1. `crates/module-model/src/my_new_tool.rs` [NOVO]: Implementar a struct e a trait `Tool`.
2. `crates/module-model/src/lib.rs`: Adicionar módulo, exportar e registrar em `ToolRegistry::with_defaults()`.
3. `assets/tools.toml`: Adicionar a flag de ativação (`my_new_tool = true`).
4. `assets/locales/en.toml`: Adicionar chaves de tradução (`tools.my_new_tool`, `hints.my_new_tool`).
5. `assets/locales/pt-BR.toml`: Adicionar traduções em português.
6. `crates/ui/src/icon_registry.rs`: Adicionar variante ao enum `PetuniaIcon` e mapeamento de string.
7. `crates/ui/src/icons.rs`: Implementar o desenho vetorial do ícone procedural via egui Painter.
8. `crates/ui/src/toolbar.rs`: Adicionar botão e tratamento de clique na barra de ferramentas.
9. `crates/ui/src/viewport_bar.rs`: Adicionar item correspondente nos menus dropdown superiores.
10. `crates/ui/src/contextual_shelf.rs`: Adicionar suporte na shelf flutuante inferior.
11. `crates/config/src/keybinds.rs`: Mapear atalho padrão e ação na tabela de keybinds.
12. `crates/app/src/lib.rs`: Adicionar match arm em `Core::on_key` para a ação string da ferramenta.
13. `crates/core/src/state.rs`: Adicionar parâmetros de estado voláteis em `AppState`.
14. `crates/core/src/modal.rs` (se interativa): Adicionar variante a `ModalKind` e lógica de pré-visualização.
15. `crates/ui/src/viewport_interaction.rs` ou módulo de sessão: Implementar o loop de eventos de mouse e feedback gráfico.

### Total de Arquivos Modificados para 1 Nova Ferramenta: **15 arquivos em 6 crates diferentes**.

Esse alto custo de extensão decorre do acoplamento rígido entre o enum de ícones, os layouts de UI, o despachador de atalhos e o estado do núcleo.

---

## 4. Direção Recomendada para a Futura Camada de Ferramentas

Em uma arquitetura desacoplada:
1. Uma ferramenta deve ser um objeto de sessão que recebe eventos de entrada agnósticos de biblioteca (`PointerEvent { world_ray, screen_point, modifiers, button }`);
2. A ferramenta gera descritores neutros de feedback visual (`OverlayMesh`, `GuideLine`, `GizmoDescriptor`), sem desenhar diretamente via `egui::Painter`;
3. Ao confirmar, a ferramenta emite um `Command` semântico para o `CommandDispatcher`, que cuida de gravar o checkpoint de undo e aplicar a modificação topológica.
