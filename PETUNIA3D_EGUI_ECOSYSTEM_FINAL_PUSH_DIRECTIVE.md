# Petunia3D — Egui Ecosystem Final Push v2
## Auditoria de implementação, evidências, arquitetura normativa, refatoração UI/UX e diretiva exaustiva para code agents

**Status:** DIRETIVA ARQUITETURAL TEMPORARIAMENTE NORMATIVA — V2 CONSOLIDADA  
**Data:** 2026-09-16  
**Revisão:** compatibilidade upstream, policy de Git pin/fork mínimo, promoção de egui_table e ecossistema hello_egui 0.36  
**Escopo:** `petunia-ui`, shell, layout, componentes, design system, UX de painéis, tooling de UI e documentação correlata  
**Objetivo:** provar, de forma controlada e mensurável, se **egui + ecossistema especializado + Petunia Components** consegue atingir a experiência visual/funcional desejada sem os hacks de layout, fragilidade de personalização e custo de manutenção observados na implementação atual.

---

# 0. Regra de autoridade desta diretiva

Durante esta iniciativa, este documento deve ser tratado como a **diretiva de intervenção** para o code agent.

Ele **não substitui** permanentemente o Livro Vivo, mas:

1. identifica conflitos concretos entre código e documentação;
2. define a direção que deve ser aplicada para resolver esses conflitos;
3. ordena a atualização das fontes documentais canônicas;
4. impede que um code agent continue seguindo regras antigas incompatíveis com a nova filosofia;
5. deixa explícito que a tentativa atual não é “continuar polindo egui cru”, e sim **mudar a arquitetura interna da UI mantendo egui como runtime**.

Após a conclusão e validação das waves, a documentação canônica deve incorporar estas decisões e esta diretiva poderá ser movida para histórico/auditoria.

Nenhum code agent está autorizado a interpretar “preservar a stack egui” como “preservar a arquitetura de layout atual”.

---

# 1. Resumo executivo

A implementação atual do Petunia3D contém bons fundamentos — renderer desacoplado, componentes próprios, tokens, árvore especializada, Command Architecture, regiões de UI e testes — mas a camada de apresentação sofre de uma contradição arquitetural:

> O projeto instalou e documentou bibliotecas auxiliares para resolver limitações do egui, mas a política de implementação continuou privilegiando `egui::Ui` built-in e cálculos manuais mesmo nos pontos onde as bibliotecas especializadas seriam mais adequadas.

O resultado observado no código é previsível:

- `ui.horizontal(...)` amplamente espalhado;
- `available_width()` usado diretamente em componentes de produto;
- breakpoints numéricos manuais;
- cálculo manual de largura de campos, tabs e clusters;
- painting customizado e `allocate_exact_size` para elementos que deveriam ser componentes estruturados;
- lógica responsiva duplicada;
- múltiplos pontos de definição de spacing/tamanho;
- widgets e painéis com regras próprias em vez de uma fundação de layout comum;
- dependências “integradas” apenas por pilotos/adapters isolados, sem adoção substancial em product paths;
- documentação mutuamente contraditória sobre `egui_tiles`, Taffy e preferência por built-ins.

A nova filosofia é:

> **Raw egui para composição simples. Bibliotecas especializadas para problemas especializados. Petunia Components e Adapters como única superfície permitida para product UI.**

A stack de UI alvo desta tentativa passa a ser, conceitualmente:

```text
                          PETUNIA UI PRODUCT CODE
                                   │
                           Petunia Components
                                   │
            ┌──────────────────────┼──────────────────────┐
            │                      │                      │
         LAYOUT                  STYLE                BEHAVIOR
            │                      │                      │
     PetuniaLayout API      Petunia Tokens       Petunia Interaction
            │                      │                      │
      egui_taffy             Twill adapter          egui_dnd
      egui_tiles             Theme system           egui_animation
            │                      │                 egui_inbox
            └──────────────────────┼──────────────────────┘
                                   │
                              SPECIALIZED
                                   │
                 egui_ltreeview / transform-gizmo
                 egui_extras / file-dialog / iconflow
                 future compatible table/list/node helpers
                                   │
                                  egui
                                   │
                              egui-wgpu/wgpu
```

---

# 2. Escopo e não-escopo

## 2.1 Escopo obrigatório

Esta iniciativa deve:

- corrigir o mau uso de layout responsivo em egui;
- promover `egui_taffy` de piloto para fundação de layout complexo;
- restaurar/alinha `egui_tiles` como engine de macro-layout controlado, salvo bloqueador técnico comprovado;
- preservar a proibição de docking irrestrito;
- aprofundar o uso de Petunia Components;
- criar adapters explícitos para bibliotecas auxiliares;
- reduzir drasticamente cálculos de largura/posição no código de produto;
- centralizar spacing/radius/density/motion;
- restaurar interações naturais de drag-and-drop onde setas/botões foram usados apenas para evitar dependências;
- preparar virtualização de coleções grandes;
- melhorar forms, feedback, animação, overlays e estados assíncronos quando houver biblioteca compatível;
- criar guardrails de arquitetura para code agents;
- criar component gallery executável;
- ampliar testes de UI, screenshots e regressão;
- atualizar documentação do repositório e Livro Vivo;
- remover contradições documentais;
- medir performance antes/depois;
- definir um gate claro para decidir entre continuar em egui ou migrar para Slint/iced.

## 2.2 Não-escopo

Esta iniciativa **não** deve:

- reescrever Geometry Core;
- alterar Commands/Algorithms;
- mudar wgpu;
- migrar para Slint/iced/web durante estas waves;
- introduzir docking livre estilo Blender;
- permitir plugins com acesso cru a `egui::Ui`;
- duplicar estado de domínio na UI;
- substituir o design system Petunia por um UI kit de terceiros;
- espalhar tipos de crates auxiliares por Core/Application;
- adicionar bibliotecas incompatíveis que puxem uma segunda família de `egui`;
- adicionar dependências só porque “parecem úteis”.

---

# 3. Base de evidência

A auditoria foi feita sobre o repositório `raillen/petunia3d`, branch padrão consultada em 2026-09-16.

Resultados de code search referenciam, em vários casos, o snapshot de commit:

```text
77afa0189e5d5f1c54582f8976bcdd336549d606
```

Os caminhos e padrões devem ser novamente verificados pelo code agent com `rg`, `cargo tree`, `cargo check` e inspeção do branch efetivamente usado antes de editar.

A evidência abaixo não é uma inferência genérica sobre egui; ela aponta ocorrências concretas do Petunia.

---

# 4. Evidência concreta: Taffy foi instalado, mas deliberadamente subutilizado

## 4.1 Dependência existe

No workspace:

```toml
egui_taffy = "0.14"
```

Em `crates/ui/Cargo.toml`:

```toml
egui_taffy = { workspace = true }
```

Logo, a biblioteca já faz parte da stack compilável.

`egui_taffy 0.14` é compatível com `egui 0.36` e oferece CSS Block, Flexbox e Grid através do Taffy.

Referência upstream:

- https://docs.rs/egui_taffy/latest/egui_taffy/
- https://github.com/PPakalns/egui_taffy

## 4.2 Uso real encontrado

Code search por:

```text
egui_taffy::
```

encontra essencialmente:

```text
crates/ui/src/flex_layout.rs
crates/xtask/src/main.rs
```

Ou seja, a biblioteca que deveria atacar layout complexo não está difundida por adapters/componentes de produto.

## 4.3 O próprio arquivo limita a adoção

`crates/ui/src/flex_layout.rs` declara:

```rust
//! Advanced responsive sublayout pilot (`egui_taffy`, P1-15).
//!
//! BASELINE CANDIDATE, not a migration...
//! ... Native `ui.horizontal` remains the default everywhere else;
//! broader adoption needs an ADR proving complexity/performance wins.
```

Isso explica por que o agente não usa a dependência.

O problema não é apenas “o modelo esqueceu a crate”.

**A política atual diz explicitamente para ele não adotá-la amplamente.**

## 4.4 O piloto é excessivamente pequeno

A implementação relevante resume-se a:

```rust
pub fn flex_key_value_row(...) -> egui::Response {
    egui_taffy::tui(ui, id)
        .reserve_available_width()
        .style(taffy::Style {
            flex_direction: taffy::FlexDirection::Row,
            justify_content: Some(taffy::JustifyContent::SpaceBetween),
            align_items: Some(taffy::AlignItems::Center),
            ...
        })
        ...
}
```

Isto prova compatibilidade e funcionamento, mas não prova que o Petunia testou uma arquitetura Taffy-first nas áreas problemáticas.

### Conclusão

**Status atual: dependência integrada tecnicamente, mas não integrada arquiteturalmente.**

A definição anterior de “integrado” foi insuficiente para UI.

---

# 5. Evidência concreta: Inspector implementa responsividade manual que deveria pertencer à fundação de layout

Arquivo:

```text
crates/ui/src/inspector_widgets.rs
```

## 5.1 Breakpoints manuais

`Vector3Field::show` usa:

```rust
let avail = ui.available_width().max(0.0);

let row: Option<(f32, f32, f32)> = if avail >= 300.0 {
    Some((14.0, 6.0, 40.0))
} else if avail >= 190.0 {
    Some((12.0, 4.0, 32.0))
} else {
    None
};
```

Isso codifica diretamente em um widget:

- breakpoint 300;
- breakpoint 190;
- label width 14;
- gap 6;
- minimum field 40;
- label width 12;
- gap 4;
- minimum field 32.

Depois calcula:

```rust
let field_w =
    ((avail - 3.0 * label_w - 2.0 * gap) / 3.0).max(min_field);
```

e monta:

```rust
ui.horizontal(|ui| { ... });
```

Para narrow layout, faz outro:

```rust
ui.horizontal(|ui| {
    ...
    let field_w = ui.available_width().max(40.0);
    ...
});
```

### Por que isso é um problema

Esta lógica mistura:

1. decisão semântica do componente;
2. breakpoint;
3. matemática de layout;
4. spacing;
5. sizing;
6. comportamento responsivo.

Tudo dentro do mesmo widget.

Qualquer mudança em:

- densidade;
- fonte;
- escala da UI;
- largura do painel;
- label;
- locale;
- iconografia;
- tamanho mínimo de campo;

pode exigir retocar fórmulas.

### Substituição obrigatória

`Vector3Field` deve se tornar um componente baseado em um `PetuniaResponsiveGrid`/Taffy.

O componente deve declarar **intenção**, não calcular pixels:

```text
large:
[X field] [Y field] [Z field]

compact:
[X field] [Y field] [Z field]

narrow:
X [field]
Y [field]
Z [field]
```

A decisão de quando o grid quebra deve viver em um adapter/policy reutilizável.

Não deve existir `3.0 * label_w` em product widget.

---

# 6. Evidência concreta: abas contextuais calculam largura manualmente

Ainda em:

```text
crates/ui/src/inspector_widgets.rs
```

`context_tabs` usa:

```rust
let gap = 4.0;

let tab_w =
    ((ui.available_width()
        - gap * (tabs.len().saturating_sub(1)) as f32)
        / tabs.len() as f32)
    .max(0.0);
```

Depois:

```rust
ui.horizontal(|ui| {
    ui.spacing_mut().item_spacing = vec2(gap, 0.0);

    for (...) {
        let (rect, resp) =
            ui.allocate_exact_size(vec2(tab_w, h), Sense::click());
        ...
    }
});
```

### Problema

Um segmented control está sendo implementado com:

- divisão manual de largura;
- painter manual;
- layout manual;
- foco manual;
- estado visual manual.

Não é necessariamente errado desenhar um componente customizado em egui, mas **a matemática de layout deve ser separada do drawing**.

### Alvo

Criar:

```text
PetuniaSegmentedControl
└── layout: PetuniaTaffyEqualColumns
└── visuals: Petunia tokens
└── semantics: selectable items
```

`context_tabs` passa a fornecer apenas itens/estado.

---

# 7. Evidência concreta: Viewport toolbar faz engine de layout na mão

Arquivo:

```text
crates/ui/src/viewport_bar.rs
```

Existe uma função chamada:

```rust
fn measured_widths(...) -> [(BarCluster, f32); 5]
```

Ela codifica somas como:

```rust
let domain = 28.0 + 24.0 * 3.0 + 3.0 * 2.0;
```

Menus:

```rust
8.0 + text_w(ui, l, 12.0) + 4.0 + 16.0
```

Transform:

```rust
14.0 + 3.0 + 62.0 + 2.0
+ 14.0 + 3.0 + 88.0 + ...
```

Display:

```rust
24.0 + 22.0 + ...
```

Depois o código implementa a própria política de overflow:

```rust
let avail_w = ui.available_width();
...
if used + width_of(cluster) + 16.0 > avail_w {
    hidden.push(cluster);
}
```

E implementa responsividade vertical com:

```rust
if ui.available_height() > 40.0 {
    // duas linhas
}
```

O comentário do próprio código diz:

```text
"padrões CSS traduzidos p/ egui built-in"
```

### Diagnóstico

Este é um dos exemplos mais fortes da auditoria.

O projeto já reconheceu que está reproduzindo conceitos de CSS/flex/responsividade, mas os traduziu para aritmética manual.

Isso é exatamente o tipo de problema que `egui_taffy` existe para resolver.

### Refatoração obrigatória

Criar:

```text
PetuniaResponsiveToolbar
├ clusters
├ priority
├ min-content intent
├ wrap policy
└ overflow policy
```

Taffy deve cuidar de:

- gap;
- row layout;
- alignment;
- wrapping;
- distribuição.

A política Petunia ainda decide **quais clusters podem ir ao overflow**, porque isso é semântica de produto.

Mas o código não deve somar dezenas de constantes de width em `viewport_bar.rs`.

A lógica correta é:

```text
Toolbar model
  ↓
semantic priority
  ↓
layout adapter
  ↓
measured/available container
  ↓
visible clusters + overflow
```

Nunca:

```text
product widget
  ↓
hardcoded width algebra
```

---

# 8. Evidência concreta: toolbar lateral usa breakpoints e colunas manuais

Arquivo:

```text
crates/ui/src/toolbar.rs
```

Exemplos:

```rust
let compact = ui.available_width() < 90.0;
```

e:

```rust
let two_cols =
    state.ui.toolbar_columns.clamp(1, 2) == 2
    && ui.available_width() >= 100.0;
```

Quando duas colunas:

```rust
for pair in group.chunks(2) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(3.0, 3.0);
        ...
    });
}
```

### Diagnóstico

Isto é um grid manual implementado como chunks + rows horizontais.

### Alvo

Usar `PetuniaToolGrid`, implementado sobre Taffy Grid.

A toolbar informa:

```text
preferred_columns
min_item_width
gap
entries
```

O adapter resolve:

- 1 coluna;
- 2 colunas;
- compact mode;
- width.

---

# 9. Evidência concreta: Reference Manager possui breakpoint manual de 560px

Arquivo:

```text
crates/ui/src/reference_manager.rs
```

Code search encontra:

```rust
if ui.available_width() < 560.0 {
    ...
}
```

com mudança manual de composição.

### Alvo

Header/actions devem usar Taffy Flex Wrap:

```text
title/description
actions
```

Sem `if width < 560`.

---

# 10. Evidência concreta: uso generalizado de `ui.horizontal`

A busca por:

```text
ui.horizontal(
```

encontra ocorrências em numerosos product paths, incluindo:

```text
asset_library_drawer.rs
animation_ui.rs
reference_manager.rs
modeling_tool_properties.rs
settings_modal.rs
toolbar.rs
asset_browser.rs
recovery_dialog.rs
lib.rs
tool_fields.rs
inspector_widgets.rs
primitive_card.rs
outliner.rs
paint_ui.rs
timeline.rs
...
```

`ui.horizontal` não é proibido por si só.

O problema é a combinação:

```text
ui.horizontal
+ available_width
+ add_space
+ spacing_mut
+ exact_size
+ manual breakpoints
+ manual painter
```

em componentes responsivos de produto.

### Nova regra

`ui.horizontal` continua permitido para **microcomposição local previsível**.

Exemplos aceitáveis:

```text
icon + label
checkbox + text
two adjacent fixed icon buttons
```

Não é aceitável como engine principal para:

```text
responsive property grids
header three-zone layout
toolbars com overflow
multi-column asset layout
dock/panel macro-layout
adaptive modal forms
```

---

# 11. Evidência concreta: Twill existe, mas ainda é prova de conceito

Arquivo:

```text
crates/ui/src/twill_bridge.rs
```

O próprio comentário diz:

```text
Pilot scope: toolbar button corner radius.
Broader adoption ... is future work.
```

O adapter hoje cobre essencialmente:

- radius;
- spacing;
- CSS proof;
- teste de cor.

Code search por:

```text
twill::
```

retorna essencialmente:

```text
twill_bridge.rs
xtask ownership checks
```

### Diagnóstico

Twill não está errado.

A integração está incompleta para o objetivo declarado.

### Nova função de Twill

Twill deve continuar **sem seu backend egui incompatível**.

Ele será usado como fundação type-safe de:

```text
Spacing scale
Radius scale
Color semantic construction where useful
Style variants
Density mapping
Motion-duration tokens if applicable
```

Mas Petunia continua dono da semântica.

Fluxo:

```text
.petunia-theme / ThemeConfig
       ↓
validated Petunia semantic tokens
       ↓
Twill/Petunia typed values
       ↓
PetuniaStyleAdapter
       ↓
egui Style + Petunia Components
```

---

# 12. Evidência concreta: egui_inbox parece estar encapsulado sem adoção ampla

Existe:

```text
crates/ui/src/inbox_bridge.rs
```

com:

```rust
pub struct UiBridge<T> {
    inbox: egui_inbox::UiInbox<T>,
}
```

Mas code search por `UiBridge<` não encontrou product paths fora do próprio bridge; os outros resultados são documentação/auditoria.

### Interpretação

Isto deve ser confirmado com `rg`, mas o estado encontrado sugere novamente:

> dependency adapter exists != product architecture actually uses it.

### Alvo

Jobs reais que terminam em UI devem convergir para um bridge padronizado:

```text
worker/service
    ↓
typed result
    ↓
UiBridge<T>
    ↓
main UI frame
    ↓
small view-state invalidation
```

Não criar polling ad-hoc.

---

# 13. Evidência documental: fontes canônicas se contradizem sobre egui_tiles

Este conflito é crítico para code agents.

## Fonte A — Livro Vivo / capítulo 36

A baseline final registrada define:

```text
egui_tiles é BASELINE, atrás de PetuniaLayoutAdapter
```

e mantém:

```text
sem docking irrestrito
```

## Fonte B — `docs/manual/interface.md`

Diz que a interface:

```text
"adota ... egui_tiles"
```

## Fonte C — `crates/ui/src/flex_layout.rs`

Diz:

```text
"egui_tiles stays the controlled macro-layout engine"
```

## Fonte D — `docs/developers/ui-architecture.md`

Diz explicitamente:

```text
"sem egui_tiles"
```

e afirma que o shell usa Panels diretamente.

## Fonte E — `docs/developers/ui-component-map.md`

Diz:

```text
egui_tiles (removido na convergência —
sistema de dock próprio de ~120 linhas atende...)
```

### Efeito

Um agente pode legitimamente escolher qualquer uma destas interpretações:

1. usar egui_tiles;
2. não usar egui_tiles;
3. continuar custom dock;
4. tratar egui_tiles apenas como histórico.

Isso é inaceitável.

### Decisão desta diretiva

**`egui_tiles` volta a ser a escolha normativa de macro-layout controlado para a tentativa final com egui**, atrás de `PetuniaLayoutAdapter`.

Não significa docking livre.

O usuário não recebe acesso à árvore inteira.

Petunia continua dono de:

```text
Left
Center
Right
Bottom
allowed plugin slots
min/max dimensions
persisted workspace layouts
```

`egui_tiles` fornece:

```text
split/layout engine
resize
tabs controladas
tile tree
layout mechanics
```

Se houver bloqueador técnico concreto, o agente deve:

1. produzir benchmark/reprodução;
2. documentar;
3. propor ADR;
4. **não trocar silenciosamente por egui_dock/custom layout**.

---

# 14. Por que egui_tiles em vez de egui_dock como padrão

Ambos possuem versões compatíveis com egui 0.36 em 2026.

## egui_tiles 0.17.1

- egui 0.36;
- horizontal;
- vertical;
- grid;
- tabs;
- drag/drop;
- resize;
- `Behavior` customizável.

Referência:

https://docs.rs/crate/egui_tiles/latest

## egui_dock 0.21.1

- egui 0.36;
- docking maduro;
- tabs;
- split;
- undock;
- window surfaces;
- muita customização.

Referência:

https://docs.rs/crate/egui_dock/latest

## Decisão Petunia

O Petunia não quer um sistema livre de docking.

Ele quer um **layout graph controlado**, que pode conter mais de dois filhos e grid/flexibilidade estrutural.

Portanto:

```text
egui_tiles
→ BASELINE macro layout
```

e:

```text
egui_dock
→ NÃO USAR simultaneamente
→ alternativa somente via ADR se egui_tiles falhar
```

Nunca manter dois engines de docking no mesmo product path.

---

# 15. Evidência documental: política anterior incentivava o comportamento errado

`docs/modernization/08-phase-e-p1-integration-wave.md` registra para `egui_taffy`:

```text
Use for selected complex sublayouts.
Compare complexity and performance against native egui layout before broad adoption.
```

A política parecia prudente, mas produziu um efeito colateral:

> o agente permaneceu usando built-ins em quase tudo e nunca houve broad adoption suficiente para testar se Taffy resolveria os problemas reais.

`docs/developers/ui-component-map.md` é ainda mais explícito ao afirmar:

```text
egui_css/egui_taffy/egui_flex
→ built-ins são suficientes para os padrões atuais
```

### Nova regra

Substituir:

```text
built-in first; helper only exceptionally
```

por:

```text
simple local composition
→ native egui

complex/responsive layout
→ Petunia Layout API / Taffy

macro split/tab/resizing
→ PetuniaLayoutAdapter / egui_tiles

tree
→ PetuniaTreeAdapter / egui_ltreeview

reordering
→ PetuniaDragAdapter / egui_dnd

async-to-UI
→ PetuniaInboxAdapter / egui_inbox
```

---

# 16. Evidência de processo: Phase E foi declarada 100% completa apesar de pilots

`docs/audits/stack-modernization/40-p1-integration-gauntlet.md` declara:

```text
Phase E ... 100% COMPLETE and verified.
```

Mas a mesma tabela classifica:

```text
egui_taffy
→ "Flex pilot for dense sublayouts"

twill
→ pilot
```

Logo, “100% complete” significava:

```text
dependency resolved
adapter exists
smoke test exists
```

e não:

```text
dependency solves its target architectural problem in real product UI
```

### Mudança obrigatória da definição de integração UI

Para uma dependência de UI ser considerada **PRODUCT-INTEGRATED**, ela deve:

1. estar na versão compatível;
2. estar isolada em adapter;
3. ter testes;
4. estar em pelo menos um product path relevante;
5. substituir o padrão problemático que justificou sua adoção;
6. possuir screenshots/interaction tests;
7. possuir performance comparison;
8. possuir documentação de when-to-use;
9. possuir guardrail que impeça regressão para a implementação antiga.

Um piloto deixa de contar como “integração completa”.

---

# 17. Filosofia normativa nova

## 17.1 Regra principal

> **Use egui como runtime e primitive layer; não como solução manual para cada problema de layout.**

## 17.2 Matriz obrigatória

| Problema | Solução normativa |
|---|---|
| micro row simples | egui built-in |
| micro column simples | egui built-in |
| layout complexo/responsivo | `PetuniaTaffyLayout` |
| flex/wrap | Taffy |
| grid | Taffy |
| macro shell | `PetuniaLayoutAdapter` + `egui_tiles` |
| árvore Parts/Scene | `PetuniaTreeAdapter` + `egui_ltreeview` |
| reorder/drag list | `PetuniaDragAdapter` + `egui_dnd` |
| animation | `PetuniaMotion` + `egui_animation` |
| async → UI | `PetuniaInboxAdapter` + `egui_inbox` |
| file dialog | existing file-dialog/native adapter |
| icons | `IconRegistry` / iconflow / Petunia Domain |
| transforms 3D | transform-gizmo adapter |
| theme values | Petunia tokens + Twill adapter |
| forms/validation | Petunia Form adapter, if compatible library passes gate |
| large virtual collection | virtualized adapter once 0.36-compatible dependency passes gate |
| node graph | Petunia graph adapter + compatible egui_snarl in future |
| tables | compatible table adapter only after egui 0.36 gate |

---

# 18. Regra de compatibilidade, atualização upstream e forks

A política anterior era conservadora demais: uma crate publicada no `crates.io` com `egui 0.34/0.35` era tratada quase automaticamente como bloqueada.

A nova política é mais precisa:

> **Compatibilidade deve ser avaliada na seguinte ordem: release crates.io → upstream atual → revision pin → fork mínimo Petunia → implementação local/defer.**

A idade do release publicado não é, sozinha, evidência de incompatibilidade real.

## 18.1 Invariante absoluta: uma família egui no shipping graph

**NÃO adicionar nenhuma crate que introduza segunda versão de egui no produto.**

Antes e depois de qualquer alteração de dependência:

```bash
cargo tree -d
cargo tree -i egui
cargo tree -i eframe
cargo tree -i egui-wgpu
```

O shipping graph deve convergir para:

```text
egui        0.36.x
eframe      0.36.x
egui-wgpu   0.36.x
egui_extras 0.36.x
egui_kittest 0.36.x (dev/test)
```

Nenhum helper pode justificar:

```text
egui 0.36
+
egui 0.35
```

ou:

```text
egui 0.36
+
egui 0.34
```

no produto.

## 18.2 Fluxo normativo de resolução

Para cada biblioteca desejada:

```text
1. Há release crates.io compatível com egui 0.36?
        │
        ├─ SIM → usar release normal
        │
        └─ NÃO
             ↓
2. Upstream main/tag/branch já migrou para egui 0.36?
        │
        ├─ SIM → fixar revision SHA
        │
        └─ NÃO
             ↓
3. O port é pequeno e semanticamente seguro?
        │
        ├─ SIM → fork mínimo Petunia
        │
        └─ NÃO
             ↓
4. Existe backend equivalente já compatível?
        │
        ├─ SIM → usar atrás do mesmo adapter
        │
        └─ NÃO → deferir ou implementar contrato local mínimo
```

Nunca usar `branch = "main"` em shipping dependency.

Use:

```toml
rev = "<SHA_EXATO_TESTADO>"
```

para builds reproduzíveis.

## 18.3 Compatibilidade confirmada — baseline atual

### `egui_taffy`

```text
egui_taffy 0.14
→ egui 0.36
→ production candidate
```

Fonte:
https://github.com/PPakalns/egui_taffy

### `egui_tiles`

```text
egui_tiles 0.17.1
→ egui 0.36
→ baseline de macro-layout controlado
```

Fonte:
https://github.com/rerun-io/egui_tiles

### `egui_table`

**CORREÇÃO DA V1:** não está bloqueado.

O upstream atual declara:

```toml
version = "0.10.0"

egui = "0.36.0"
eframe = "0.36.0"
egui_kittest = "0.36.0"
```

Logo:

```text
egui_table 0.10
→ AVAILABLE NOW
→ usar release/upstream compatível
```

Fonte:
https://github.com/rerun-io/egui_table/blob/main/Cargo.toml

### ecossistema `hello_egui`

O workspace upstream atual declara:

```text
hello_egui          0.13.0
egui_dnd            0.17.0
egui_animation      0.13.0
egui_flex           0.8.0
egui_form           0.10.0
egui_inbox          0.13.0
egui_suspense       0.13.0
egui_virtual_list   0.12.0
egui_infinite_scroll 0.12.0
egui_router         0.9.0
egui_thumbhash      0.12.0

workspace egui        0.36.0
workspace eframe      0.36.0
workspace egui_extras 0.36.0
workspace egui_kittest 0.36.0
```

Fonte:
https://github.com/lucasmerlin/hello_egui/blob/main/Cargo.toml

Isso significa que, mesmo quando o crates.io estiver atrasado, o Petunia pode usar uma **revision pinada do monorepo upstream**.

### `egui-notify`

O release oficial atual encontrado é:

```toml
egui-notify = "0.22.0"
egui = "0.34"
eframe = "0.34" # dev dependency
```

Fonte:
https://docs.rs/crate/egui-notify/latest/source/Cargo.toml.orig

Portanto:

```text
egui-notify 0.22
→ NOT DIRECTLY COMPATIBLE
→ APPROVED FOR MINIMAL PETUNIA PORT/FORK
```

Não adicionar a versão oficial como shipping dependency enquanto ela puxar egui 0.34.

---

# 19. Estratégia oficial para dependencies Git e forks Petunia

## 19.1 Revision pin upstream

Quando upstream já possui o port 0.36 mas não publicou release adequada:

```toml
[workspace.dependencies]
egui_suspense = {
    git = "https://github.com/lucasmerlin/hello_egui",
    rev = "<TESTED_SHA>",
    package = "egui_suspense"
}
```

Aplicar a mesma revision ao conjunto Hello Egui usado pelo Petunia.

Não usar SHAs diferentes do mesmo monorepo sem motivo técnico.

Exemplo conceitual:

```text
hello_egui revision ABC
├ egui_suspense
├ egui_virtual_list
├ egui_form
├ egui_dnd
├ egui_animation
└ egui_inbox
```

Isto garante que os crates foram desenvolvidos contra o mesmo conjunto de versões.

## 19.2 Fork mínimo

Quando upstream não suporta 0.36, mas o port é pequeno:

```text
upstream
   ↓
Petunia fork
   ↓
version manifest bump
   ↓
API compatibility patches
   ↓
tests
   ↓
[patch.crates-io] or git revision
```

O fork deve preservar:

- nome original da crate quando possível;
- API pública;
- licença;
- histórico upstream;
- diff mínimo.

## 19.3 Exemplo — `egui-notify`

Primeiro port esperado:

```toml
# upstream
egui = "0.34"

# Petunia fork
egui = "0.36"
```

e dev dependency:

```toml
eframe = "0.36"
```

Depois:

```bash
cargo check
cargo test
cargo clippy --all-targets -- -D warnings
cargo tree -d
```

Apenas corrigir API de egui se o compilador indicar necessidade.

**Não reescrever a biblioteca preventivamente.**

## 19.4 Uso de `[patch.crates-io]`

Se o fork preservar `name/version` upstream:

```toml
[patch.crates-io]
egui-notify = {
    git = "https://github.com/raillen/egui-notify",
    rev = "<TESTED_SHA>"
}
```

Vantagens:

- callers continuam usando `egui_notify`;
- remoção futura do fork é trivial;
- não espalha source overrides;
- upgrade upstream é simples.

## 19.5 Arquivo obrigatório para forks

Cada fork/patch deve possuir um registro Petunia:

```text
docs/dependencies/forks/egui-notify.md
```

com:

```text
Upstream URL
Upstream version/tag/commit
Petunia base commit
Reason for fork
Exact diff summary
License
Security review
Compatibility target
Tests
Removal condition
Last upstream sync
Owner
```

---

# 20. Política de saída de forks

Fork é **ponte temporária**, não nova feature Petunia.

Todo fork deve possuir `EXIT_CONDITION`.

Exemplo:

```text
Remove egui-notify fork when:
- upstream publishes version compatible with egui 0.36.x;
- Petunia adapter tests pass against upstream;
- no Petunia-only API has leaked into callers.
```

O adapter é o mecanismo que torna isso possível:

```text
product code
   ↓
PetuniaToastHost
   ↓
egui-notify fork
```

Depois:

```text
product code
   ↓
PetuniaToastHost
   ↓
official egui-notify
```

Sem alterações nos callers.

---

# 21. Classificação atualizada das bibliotecas auxiliares

## 21.1 P0 — baseline

### `egui_taffy 0.14`

Layout complexo/responsivo.

### `egui_tiles 0.17.1`

Macro-layout controlado.

### `egui_dnd 0.17`

Drag/reorder.

Preferir release compatível; se usar Hello Egui git, pin da mesma revision do conjunto.

### `egui_animation 0.13`

Motion foundation.

### Twill core

Typed design foundation, sem backend egui incompatível.

## 21.2 P1 — adotar em product paths

### `egui_table 0.10`

Agora promovido.

Usos:

- Asset Validator;
- geometry statistics;
- diagnostics;
- LOD;
- batch processing;
- material/UV reports.

Sempre atrás de:

```text
PetuniaTable
```

### `egui_suspense 0.13` upstream

Agora promovido para candidato real.

Usos:

- thumbnail loading;
- asset scanning;
- plugin load state;
- async import;
- retry/error views.

Sempre atrás de:

```text
PetuniaAsyncView
```

### `egui_virtual_list 0.12` upstream

Agora promovido.

Usos:

- Asset Library;
- command palette;
- large result sets;
- Parts quando apropriado.

Sempre atrás de:

```text
PetuniaVirtualCollection
```

### `egui_form 0.10` upstream

Promovido para avaliação imediata.

Usos:

- Settings;
- project/export forms;
- primitive settings;
- plugin settings;
- theme editor.

Sempre atrás de:

```text
PetuniaForm
```

### `egui_inbox 0.13`

Aprofundar integração real.

## 21.3 P1-patched

### `egui-notify 0.22`

Port/fork mínimo aprovado.

Usar atrás de:

```text
PetuniaToastHost
```

Não permitir `Toasts` em arbitrary product files.

## 21.4 P2

- `egui_infinite_scroll`;
- `egui_router`;
- `egui_thumbhash`;
- `egui-snarl`;
- specialized graph/plot libraries.

Entram somente quando uma feature do produto justificar.

---

# 22. Arquitetura obrigatória do código

O code agent deve convergir para a estrutura lógica abaixo.

Não é obrigatório mover todos os arquivos de uma vez; aplicar incrementalmente.

```text
crates/ui/src/
│
├ foundation/
│  ├ mod.rs
│  ├ theme.rs
│  ├ density.rs
│  ├ spacing.rs
│  ├ radius.rs
│  ├ typography.rs
│  └ motion.rs
│
├ adapters/
│  ├ mod.rs
│  ├ taffy_layout.rs
│  ├ tile_layout.rs
│  ├ tree.rs
│  ├ drag_drop.rs
│  ├ inbox.rs
│  ├ icons.rs
│  ├ file_dialog.rs
│  ├ gizmo.rs
│  ├ form.rs              // only when compatible
│  ├ virtual_collection.rs
│  └ table.rs
│
├ components/
│  ├ mod.rs
│  ├ button.rs
│  ├ icon_button.rs
│  ├ menu.rs
│  ├ popup.rs
│  ├ modal.rs
│  ├ panel.rs
│  ├ section.rs
│  ├ property_grid.rs
│  ├ property_row.rs
│  ├ number_field.rs
│  ├ vector_field.rs
│  ├ segmented_control.rs
│  ├ toolbar.rs
│  ├ search.rs
│  ├ tree.rs
│  ├ asset_grid.rs
│  ├ drag_list.rs
│  ├ inline_message.rs
│  ├ progress.rs
│  └ empty_state.rs
│
├ shell/
│  ├ mod.rs
│  ├ layout.rs
│  ├ header.rs
│  ├ left_region.rs
│  ├ center_region.rs
│  ├ right_region.rs
│  ├ bottom_region.rs
│  └ status.rs
│
├ panels/
│  ├ parts/
│  ├ context/
│  ├ assets/
│  ├ settings/
│  └ ...
│
└ devtools/
   ├ component_gallery.rs
   ├ layout_inspector.rs
   └ ...
```

## 22.1 Regra de dependência

```text
panels/screens
      ↓
Petunia Components
      ↓
Petunia Adapters/Foundation
      ↓
egui/helper crates
```

Proibido:

```text
random_panel.rs
    ↓
egui_taffy directly
```

Proibido:

```text
properties_panel.rs
    ↓
egui_dnd directly
```

Proibido:

```text
settings_modal.rs
    ↓
twill directly
```

Somente adapters/foundation conhecem detalhes das crates, salvo componentes especializados explicitamente aprovados.

---

# 23. API de layout Petunia

Criar uma API pequena para evitar que product code conheça Taffy.

Exemplo conceitual:

```rust
pub enum PetuniaLayoutMode {
    Row,
    Column,
    Grid,
}

pub struct PetuniaGap {
    pub horizontal: f32,
    pub vertical: f32,
}

pub struct PetuniaResponsiveLayout {
    pub mode: PetuniaLayoutMode,
    pub gap: PetuniaGap,
    pub align: PetuniaAlign,
    pub justify: PetuniaJustify,
    pub wrap: bool,
}
```

Implementação:

```text
PetuniaResponsiveLayout
       ↓
Taffy Style
       ↓
egui_taffy::tui(...)
```

O product code não cria `taffy::Style` arbitrário.

---

# 24. `PetuniaPropertyGrid`

Este componente é obrigatório porque o painel Context é uma das principais fontes de fragilidade.

API conceitual:

```rust
property_grid(ui, state, "transform-grid", |grid| {
    grid.row("Position", |ui| {
        vector3_field(...);
    });

    grid.row("Rotation", |ui| {
        vector3_field(...);
    });

    grid.row("Scale", |ui| {
        vector3_field(...);
    });
});
```

Internamente:

```text
wide:
[label] [control................]

narrow:
[label..........................]
[control........................]
```

Ou, para `Vector3Field`:

```text
wide:
X [value] Y [value] Z [value]

narrow:
X [value.................]
Y [value.................]
Z [value.................]
```

### Proibido no caller

```rust
if ui.available_width() >= 300.0
```

### Permitido

Uma policy interna do adapter pode possuir thresholds baseados em tokens/measurements, mas eles existem em um lugar único, testado e documentado.

---

# 25. Top Bar

A Top Bar deve deixar de depender de hacks de centralização.

Implementar como grid/flex de três zonas:

```text
| LEFT                    | CENTER          | RIGHT            |
| menus/project           | MODEL PAINT UV  | global actions   |
```

Regras:

- CENTER permanece geometricamente centralizado quando possível;
- LEFT/RIGHT não empurram as pills de forma assimétrica sem fallback;
- em largura insuficiente, ações de menor prioridade vão para overflow;
- não usar `add_space` repetido para fingir centralização.

---

# 26. Viewport Toolbar

Separar em modelo semântico:

```rust
struct ToolbarCluster {
    id: ToolbarClusterId,
    priority: ToolbarPriority,
    overflowable: bool,
    content: ...
}
```

Clusters:

```text
Domain
Menus
Transform
Snap/Proportional
Display
```

Taffy cuida de alinhamento/wrap.

Petunia cuida de prioridade.

Os valores de min-size devem vir de componentes/tokens, não de somas duplicadas.

O sistema pode fazer measurement em uma fase controlada, mas a medição deve viver no adapter.

Remover a função `measured_widths` do product module ao final da wave.

---

# 27. Right Context / Inspector — refatoração UX autorizada

Esta diretiva autoriza refatoração visual e estrutural **desde que preserve a filosofia selection-centric**.

Objetivo:

- remover “caixas dentro de caixas”;
- reduzir sensação espremida;
- labels alinhadas;
- controle com largura previsível;
- disclosure consistente;
- tabs contextuais estáveis;
- rolagem previsível;
- sem botões espremidos;
- forms adaptativos.

Estrutura alvo:

```text
CONTEXT
────────────────────
Object Name       ○
Mesh · 8v · 12t

[ Object | Modify | Material ]

TRANSFORM
Position
  X [          ]
  Y [          ]
  Z [          ]

Rotation
  ...

GEOMETRY
...

DISPLAY
...
```

Em largura confortável, `Vector3Field` pode ficar horizontal.

Em largura estreita, deve empilhar.

Nenhum field pode ficar menor que o mínimo de acessibilidade/legibilidade.

---

# 28. Parts / Scene Tree

Continuar usando `egui_ltreeview`.

Mas reintroduzir interação natural onde apropriado.

Se reordenação/hierarquia editável for suportada:

```text
egui_ltreeview
+
PetuniaDragAdapter
+
egui_dnd
```

O drag não altera domínio diretamente.

Fluxo:

```text
drag UI
  ↓
drop intent
  ↓
MovePartCommand / ReparentCommand
  ↓
Application
```

---

# 29. Modifier Stack e listas ordenáveis

Não usar ↑/↓ como interação principal quando drag for seguro.

Manter acessibilidade por teclado/botões auxiliares, mas principal:

```text
grab handle
drag
drop indicator
animated reposition
commit command
```

Usar:

```text
PetuniaDragList
→ egui_dnd
→ egui_animation
```

Undo:

- uma operação por drop;
- não criar checkpoint por frame de drag.

---

# 30. Motion

Criar tokens:

```text
motion.hover = 90ms
motion.menu = 100ms
motion.collapse = 140ms
motion.structural = 180ms max
```

Respeitar:

```text
reduced_motion
```

`PetuniaMotion` é a única API para animações comuns.

Não espalhar durations literais.

Permitido:

- collapse;
- position/reorder;
- opacity/appearance curta;
- panel transition curta.

Proibido:

- looping decorativo;
- motion que atrasa input;
- transform de viewport por animação UI;
- animation state duplicando domínio.

---

# 31. Macro-layout e egui_tiles

## 31.1 Modelo

```rust
enum PetuniaPane {
    Parts,
    Viewport,
    Context,
    AssetLibrary,
    Plugin(PluginPanelId),
}
```

`PetuniaLayoutAdapter` traduz:

```text
Petunia workspace layout DTO
       ↓
egui_tiles tree
```

## 31.2 Restrições

Core panes:

```text
Viewport
Parts
Context
AssetLibrary
```

não podem ser arbitrariamente destruídos por UI do usuário.

Plugin panels só entram em slots permitidos.

Não expor drag de pane global se não estiver autorizado.

## 31.3 Persistência

Não serializar `egui_tiles::Tree` diretamente como contrato durável.

Persistir DTO Petunia:

```text
workspace
left_width
right_width
bottom_height
visible_panels
plugin panel slots
tab selection
```

Gerar a árvore runtime a partir disso.

Assim podemos trocar backend no futuro.

---

# 32. Asset Library

Objetivo:

- grid responsivo;
- thumbnail sizing;
- busca/filtro;
- grande volume;
- não renderizar todos os itens;
- async loading desacoplado da árvore visível.

A V2 permite usar o ecossistema upstream egui 0.36:

```text
PetuniaAssetGrid
      ↓
PetuniaVirtualCollection
      ↓
egui_virtual_list 0.12
      ↓
egui 0.36
```

Quando a versão usada ainda não estiver publicada em crates.io:

- usar revision pinada de `hello_egui`;
- mesma revision usada por `egui_suspense/form/dnd` se esses também estiverem via git;
- registrar SHA no dependency ledger.

Taffy resolve geometria/layout.

Virtual list resolve quantidade de elementos.

Suspense/Inbox resolvem loading/job delivery.

Arquitetura:

```text
Asset repository
      ↓
filtered/sorted lightweight view model
      ↓
virtual window
      ↓
visible rows/cards
      ↓
thumbnail async request
      ↓
UiBridge / PetuniaAsyncView
```

Não:

```text
25,000 assets
→ instantiate 25,000 cards
```

Benchmark mínimo:

```text
100
1,000
10,000
25,000 metadata entries
```

---

# 33. Settings, Forms e validação

Criar:

```text
PetuniaSettingsLayout
├ navigation
└ PetuniaForm
```

Layout com Taffy Grid/Flex.

A V2 promove `egui_form 0.10` do workspace Hello Egui atual para candidato real, desde que:

1. revision usada esteja no workspace `egui 0.36`;
2. `cargo tree -d` permaneça limpo;
3. adapter Petunia esconda tipos da crate.

`PetuniaForm` padroniza:

- label;
- help text;
- required;
- error;
- disabled;
- validation;
- focus-on-error;
- keyboard navigation;
- semantic grouping.

Fluxo:

```text
Domain/ViewModel validation data
       ↓
PetuniaForm
       ↓
egui_form adapter
       ↓
visual errors/focus
```

Não permitir que regras de domínio sejam movidas para a crate de forms.

---

---

# 34. Popups, menus e modais

Criar APIs únicas:

```text
PetuniaPopup
PetuniaMenu
PetuniaModal
PetuniaContextMenu
```

Product code não deve usar livremente:

```text
egui::Window
egui::Popup
Area
raw layers
```

exceto em módulos especializados/whitelisted.

Motivo:

- sizing consistente;
- z-order consistente;
- focus;
- escape;
- click-outside;
- max-size;
- scaling;
- accessibility;
- motion.

Isso reduz bugs de menu/modal que já apareceram no Petunia.

---

# 35. Design tokens

A meta é zero hardcode visual fora de foundation/adapters/component internals, salvo valores de domínio inevitáveis.

Escalas mínimas:

```text
spacing
radius
font size
control height
icon size
panel metric
motion duration
border
semantic colors
density
```

Exemplo:

```rust
tokens::spacing::XS
tokens::spacing::SM
tokens::spacing::MD

tokens::control::HEIGHT
tokens::control::MIN_HIT_SIZE

tokens::motion::COLLAPSE
```

Twill pode fornecer type-safe scale; Petunia fornece significado.

---

# 36. Proibições de código após a migração

Nas áreas migradas, o code agent não deve introduzir novamente:

```rust
if ui.available_width() < 560.0
```

nem:

```rust
let width = (available - x * y - gap) / count;
```

nem:

```rust
ui.spacing_mut().item_spacing = vec2(3.0, 3.0);
```

fora de components/foundation.

Também evitar em product code:

```rust
ui.allocate_exact_size(...)
ui.painter().rect_filled(...)
ui.add_space(4.0)
```

quando o objetivo for um componente padrão já existente.

### Exceções legítimas

- viewport;
- UV canvas;
- timeline ruler;
- gizmo;
- custom data visualization;
- low-level component implementation.

A exceção deve estar em módulo especializado, não em cada panel.

---

# 37. `ui-guard` obrigatório

Adicionar ou estender:

```text
cargo xtask ui-guard
```

Objetivo: evitar regressões arquiteturais.

O guard deve procurar, fora de whitelists:

```text
egui_taffy::        // só adapters
egui_tiles::        // só adapter
egui_dnd::          // só adapter
twill::             // foundation/adapter
Color32::from_      // product paths suspeitos
spacing_mut()       // product paths suspeitos
allocate_exact_size // product paths suspeitos
available_width()   // product responsive layout suspeito
egui::Window        // fora modal/devtools
egui::Popup         // fora popup/menu adapters
```

Não precisa começar como erro para todos os padrões.

Fases:

```text
Wave 1 → report
Wave 2 → warning CI
Wave 3 → error para arquivos migrados
```

---

# 38. Component Gallery obrigatório

Criar executável/example:

```bash
cargo run -p petunia_ui --example component_gallery
```

Deve mostrar:

- buttons;
- icon buttons;
- workspace pills;
- segmented controls;
- inputs;
- numeric fields;
- Vector3Field;
- sliders;
- property rows;
- sections;
- tabs;
- toolbar clusters;
- menus;
- context menus;
- popup;
- modal;
- tree;
- drag list;
- asset cards;
- empty states;
- inline messages;
- progress;
- async/loading;
- Taffy row/flex/grid/wrap;
- dark;
- high contrast;
- density presets;
- scaling presets.

O code agent deve ser instruído a:

1. procurar componente existente;
2. modificar component gallery;
3. só depois usar no produto.

Não criar visual isolado em um panel sem primeiro existir no gallery quando for reutilizável.

---

# 39. Testes de UI obrigatórios

## Resoluções

```text
1024×768
1280×720
1366×768
1920×1080
2560×1440
```

## Scaling

```text
100%
125%
150%
175%
200%
```

## Cenários

```text
Model empty
Model object selected
Face selected
Edge selected
Point selected
Paint
UV
Settings
Reference Manager
Asset Library expanded
Asset Library collapsed
Right Context min width
Right Context max width
menu open
popover open
modal open
plugin panel slot
long pt-BR strings
pseudo locale
high contrast
```

## Invariantes

- nada sobrepõe Status Bar;
- nada sai da janela;
- não há botões inacessíveis;
- não há texto interativo < baseline;
- viewport mínimo preservado;
- keyboard focus visível;
- menus não ultrapassam bounds sem scroll/flip;
- resize contínuo não explode layout;
- no panic;
- no invalid IDs.

---

# 40. Visual regression

Adicionar screenshots canônicos com `egui_kittest`/harness já existente.

Não snapshotar ruído instável.

Snapshotar:

- component gallery;
- shell principal;
- Context;
- toolbar;
- modais principais.

Mudança visual intencional deve atualizar snapshot junto com justificativa.

---

# 41. Performance gates

Não escolher helper por intuição.

Medir antes/depois.

Cenários:

```text
idle shell
resize right panel continuously
open/close menu repeatedly
open/close modal repeatedly
scroll Parts
scroll Asset Library
drag modifier list
switch workspaces
selection changes updating Context
theme switch
125/150/200% scaling
```

Métricas:

- CPU frame time;
- UI frame time;
- allocations where measurable;
- memory RSS;
- input latency qualitativa + instrumentation;
- repaint frequency.

Regra inicial:

> Nenhuma wave de UI pode introduzir regressão consistente >10% no median frame cost de seu cenário alvo sem justificativa e ADR.

Para pequenas diferenças dentro de ruído, não otimizar prematuramente.

---

# 42. Wave 0 — congelar e documentar baseline

Antes de refatorar:

1. rodar todos os testes;
2. capturar screenshots;
3. medir perf;
4. gerar lista de arquivos com:
   - `ui.horizontal`;
   - `available_width`;
   - `spacing_mut`;
   - `allocate_exact_size`;
   - raw popup/window;
5. gerar `docs/audits/ui-ecosystem-final-push/00-baseline.md`.

Este arquivo deve conter contagens.

---

# 43. Wave 1 — corrigir documentação e autoridade

**Esta wave ocorre antes da expansão do código.**

Atualizar no mínimo:

```text
docs/developers/ui-architecture.md
docs/developers/ui-component-map.md
docs/manual/interface.md
docs/modernization/06-p0-to-p3-dependency-policy.md
docs/modernization/08-phase-e-p1-integration-wave.md
docs/modernization/full-gauntlet.md
docs/audits/stack-modernization/40-p1-integration-gauntlet.md
docs/bible/foundations/... capítulos de UI relevantes
```

E o Livro Vivo/Notion:

```text
35 — egui, Petunia Components, UI Adapters e Tooling
36 — UI Baseline Final V1, Temas e Plugin Panels
```

## Alteração conceitual obrigatória

Remover/reformular frases equivalentes a:

```text
built-ins são suficientes
Taffy somente piloto
Taffy somente se ADR provar
custom dock substitui tiles definitivamente
```

Substituir por:

```text
native egui para micro-layout simples;
specialized adapters para problemas complexos;
egui_taffy baseline para responsive sublayouts;
egui_tiles baseline para controlled macro layout;
no unrestricted docking;
third-party types confined to adapters;
```

## Audit histórico

Não apagar história falsa ou reescrever datas.

No `40-p1-integration-gauntlet.md`, adicionar nota:

```text
Historical result: dependency-level integration complete.
Product-architecture adoption of Taffy/Twill was intentionally pilot-scoped
and is superseded by the Egui Ecosystem Final Push directive.
```

---

# 44. Wave 2 — fundação/adapters e dependency modernization

Criar/refatorar:

```text
foundation/
adapters/
component gallery
ui-guard
dependency ledger
fork registry
```

Antes de migrar painéis.

## Dependency actions nesta wave

1. confirmar:
   - `egui_taffy 0.14`;
   - `egui_tiles 0.17.1`;
   - `egui_table 0.10`;
2. escolher um SHA testado de `hello_egui` quando forem necessários crates ainda não publicados no crates.io;
3. alinhar `egui_dnd`, `egui_animation`, `egui_suspense`, `egui_virtual_list`, `egui_form`, `egui_inbox` à mesma revision quando aplicável;
4. criar fork mínimo de `egui-notify` somente depois de provar por branch de teste que o bump de egui é pequeno;
5. registrar todos os overrides.

`cargo tree -d` não pode mostrar segunda família egui.

Criar:

```text
docs/dependencies/ui-ecosystem-lock.md
```

contendo:

```text
crate
source
version
revision
egui version
why
adapter
shipping?
upstream exit condition
```

---

# 45. Wave 3 — Context/Inspector

É a primeira product wave porque é uma das áreas com maior dor.

Migrar:

- property rows;
- vector fields;
- tabs;
- section header sizing;
- responsive controls.

Eliminar fórmulas manuais observadas na auditoria.

Acceptance:

- sem `avail >= 300` / `190` em `Vector3Field`;
- sem cálculo manual `tab_w`;
- layout passa todos os tamanhos/scales;
- screenshot comparado;
- behavior/undo preservados.

---

# 46. Wave 4 — Viewport Toolbar

Migrar cluster layout.

Acceptance:

- remover/encapsular `measured_widths`;
- sem somas manuais de pixels no product module;
- overflow semantics preservadas;
- Domain/Menus/Display permanecem acessíveis conforme política;
- long locale tested;
- 1/2 row behavior estável.

---

# 47. Wave 5 — Top Bar e shell

Implementar Top Bar em grid/flex.

Reintroduzir `PetuniaLayoutAdapter` com egui_tiles.

Migrar:

- Left;
- Center;
- Right;
- Bottom.

Preservar:

- viewport-first;
- no unrestricted docking;
- min viewport;
- workspace-specific persistence.

---

# 48. Wave 6 — toolbar lateral, Settings e Reference Manager

Remover breakpoints manuais.

Usar Taffy Grid/Flex.

Settings ganha `PetuniaForm` contract.

Reference Manager usa flex-wrap.

---

# 49. Wave 7 — DnD e motion

Integrar:

```text
egui_dnd
egui_animation
```

Aplicar primeiro a um path real e valioso:

```text
toolbar customization
ou modifier list
```

Depois expandir.

Não implementar em tudo ao mesmo tempo.

---

# 50. Wave 8 — Asset Library e coleções grandes

Implementar:

```text
PetuniaAssetGrid
PetuniaVirtualCollection
PetuniaThumbnailState
```

Backend preferido:

```text
egui_virtual_list 0.12
via compatible release or pinned hello_egui revision
```

Benchmark:

- 100;
- 1,000;
- 10,000;
- 25,000 asset metadata entries.

Thumbnails lazy.

O benchmark deve separar:

```text
metadata list cost
layout cost
thumbnail decode cost
GPU upload cost
```

Nunca atribuir custo de thumbnail decoding à virtual-list crate.

---

# 51. Wave 9 — tables, feedback, forms e async

Esta wave deixa de ser uma wave de “esperar compatibilidade” e passa a ser uma wave de **integração controlada**.

## Table

```text
PetuniaTable
→ egui_table 0.10
```

Primeiro product path:

```text
Asset Validator ou Geometry Statistics
```

## Forms

```text
PetuniaForm
→ egui_form 0.10 (compatible release / pinned Hello Egui)
```

Primeiro:

```text
Settings
```

## Suspense

```text
PetuniaAsyncView
→ egui_suspense 0.13
```

Primeiro:

```text
thumbnail/asset loading
```

## Notifications

```text
PetuniaToastHost
→ patched egui-notify 0.22
```

Somente depois de:

```text
fork compile
tests
clippy
single-egui-tree check
```

Se o port de `egui-notify` se provar maior que o esperado, manter `PetuniaToastHost` com backend local temporário, sem bloquear o restante da wave.

---

# 52. Wave 10 — cleanup

Remover:

- bridges piloto substituídos;
- APIs antigas;
- helpers duplicados;
- dead code;
- docs antigas;
- magic constants migrados;
- custom dock anterior quando egui_tiles estiver validado;
- arrows-only reordering onde DnD substituiu.

---

# 53. Wave 11 — Gauntlet final

Rodar:

```text
AUDIT
TARGET
SAFETY TESTS
IMPLEMENT/REFACTOR
BUILD
TEST
VISUAL/BEHAVIOR REVIEW
ARCHITECTURE CHECK
PERFORMANCE CHECK
DOCS CHECK
SCORE
FIX
REPEAT
```

Cada categoria abaixo recebe nota 1–10:

```text
layout robustness
responsive behavior
visual consistency
accessibility
component reuse
helper-library adoption
agent guardrails
performance
documentation consistency
code maintainability
```

Nenhuma pode ficar abaixo de 9/10 para encerrar a wave final.

---

# 54. Critério para manter egui

Egui permanece baseline se, depois desta intervenção:

1. Context pode ser alterado sem quebrar shell;
2. toolbar responde a width/locale sem matemática frágil;
3. resize é estável;
4. component changes propagam globalmente;
5. não há proliferation de hardcoded widths;
6. custom styling é centralizado;
7. code agents usam components/adapters por padrão;
8. visual regression impede que correções quebrem outras regiões;
9. performance permanece dentro dos gates;
10. novas telas não exigem reinventar layout.

---

# 55. Critério objetivo para abandonar egui

Após o Final Push, migrar para Slint/iced se continuarem presentes **dois ou mais** destes pontos:

- layout responsivo ainda exige matemática manual em product panels;
- popups/modais continuam estruturalmente frágeis;
- customização visual simples exige alterações em muitos arquivos;
- painel Context não atinge qualidade desejada sem painter custom excessivo;
- agent frequentemente quebra shell apesar dos guardrails;
- Taffy/tiles introduzem custo/bugs maiores que o benefício;
- acessibilidade fica difícil de preservar;
- scaling/locale continuam quebrando layout;
- manutenção de components é mais custosa que um spike Slint/iced equivalente.

A decisão deve ser feita com screenshots, profiling e código do spike, não preferência abstrata.

---

# 56. Diretivas explícitas para o code agent

## O agente DEVE

- ler esta diretiva antes de alterar UI;
- ler chapters 35/36 atualizados;
- verificar versão real do branch;
- rodar `cargo tree -d`;
- usar adapters;
- procurar Petunia Component antes de criar widget;
- criar/atualizar component gallery;
- escrever teste;
- executar UI scenarios;
- atualizar documentação junto com implementação;
- medir antes/depois;
- preservar Commands e Domain boundaries.

## O agente NÃO DEVE

- adicionar `if width < N` em product panels;
- criar nova engine de docking;
- usar `egui_dock` e `egui_tiles` simultaneamente;
- usar helper crate diretamente em qualquer arquivo arbitrário;
- duplicar egui version;
- reescrever renderer;
- introduzir `Arc<Mutex<AppState>>`;
- mover domínio para UI;
- criar hardcoded color/spacing;
- usar painter para controles padrões sem justificativa;
- substituir UX selection-centric por um Properties genérico;
- inventar APIs de crates sem consultar docs atuais;
- declarar wave concluída apenas porque compila.

---

# 57. Definition of Done por biblioteca

## egui_taffy

DONE somente quando:

- adapter existe;
- Context usa;
- toolbar usa;
- Settings/Reference Manager têm path real;
- manual width formulas diminuíram;
- tests/screenshots passam;
- benchmarks passam.

## egui_tiles

DONE somente quando:

- shell real usa;
- workspace persistence funciona;
- core panes protegidos;
- plugin slots respeitados;
- resize testado;
- docking livre desabilitado;
- old custom dock removido.

## egui_dnd

DONE somente quando:

- pelo menos um reorder real usa;
- keyboard fallback existe;
- undo ocorre uma vez por drop;
- animation feedback correto;
- dependency usa egui 0.36.

## egui_animation

DONE somente quando:

- motion tokens existem;
- reduced motion existe;
- collapse/reorder real usa;
- nenhum loop decorativo foi adicionado.

## egui_table

DONE somente quando:

- versão 0.10/egui 0.36 está no graph;
- `PetuniaTable` adapter existe;
- um product path real usa;
- large-row fixture existe;
- sticky/header/resize behavior testado;
- product code não importa `egui_table` diretamente.

## egui_virtual_list

DONE somente quando:

- source/revision 0.36 é pinado;
- `PetuniaVirtualCollection` existe;
- 10k+ fixture passa;
- keyboard/focus não quebra;
- overscan é medido;
- product callers não conhecem a crate.

## egui_suspense

DONE somente quando:

- revision/release 0.36 está pinada;
- `PetuniaAsyncView` existe;
- loading/error/success/retry testados;
- cancel/destruction não deixa stale state;
- async crate não possui domínio.

## egui_form

DONE somente quando:

- adapter existe;
- Settings usa;
- validation/focus behavior testado;
- domain validation não foi movida para UI;
- dependency graph continua single-egui.

## egui-notify fork

DONE somente quando:

- fork documentado;
- base upstream identificada;
- egui/eframe ajustados para 0.36;
- code changes são mínimos;
- `cargo check/test/clippy` passam;
- toast screenshots passam;
- `[patch.crates-io]` usa SHA fixo;
- `PetuniaToastHost` esconde a crate;
- EXIT_CONDITION documentada.

## Twill

DONE somente quando:

- foundation usa scale;
- tokens hardcoded convergem;
- backend incompatível não é ativado;
- callers não importam Twill diretamente.

---

# 58. Atualizações específicas de documentação

## `docs/developers/ui-architecture.md`

Substituir:

```text
sem egui_tiles
```

por descrição do controlled tile adapter.

Adicionar:

- Taffy complex layout policy;
- helper ownership;
- forbidden raw patterns;
- product-code dependency rules.

## `docs/developers/ui-component-map.md`

Remover conclusão:

```text
egui_tiles removed
egui_dnd unnecessary
Taffy only occasional
built-ins sufficient
```

Substituir pela matriz normativa desta diretiva.

## `docs/manual/interface.md`

Manter a descrição do comportamento de usuário.

Não expor detalhes desnecessários de implementação, mas alinhar que layout é estável/controlado.

## `docs/modernization/08...`

Promover Taffy de selected pilot para complex-layout baseline.

## `docs/modernization/06...`

Atualizar definition of integrated para UI dependencies.

## `docs/audits/...40...`

Marcar pilot scope histórico e nova wave de product adoption.

## Livro Vivo capítulo 35

Adicionar:

```text
Petunia UI Helper Architecture
Adapters
Taffy-first complex layouts
Tiles controlled macro layout
DnD/motion contracts
compatibility gate
```

## Livro Vivo capítulo 36

Preservar UX baseline.

Reafirmar:

- viewport-first;
- Parts/Context/Assets;
- controlled layout;
- no unrestricted docking.

Adicionar que detalhes de implementação usam specialized helpers e não raw egui por padrão em layout complexo.

---

# 59. Mudanças permitidas na UI/UX

A iniciativa não é só “trocar código por Taffy”.

Ela pode corrigir UI/UX que exista apenas por limitação técnica antiga.

Exemplos autorizados:

- substituir ↑/↓ por drag + keyboard fallback;
- trocar rows espremidas por stacked responsive controls;
- mover secondary actions para overflow;
- reformular header de panel para respeitar space;
- aumentar hit areas;
- reorganizar property rows;
- reduzir nested cards;
- melhorar popup positioning;
- ajustar panel minimum widths;
- aplicar motion curta;
- melhorar states loading/error/empty.

Não alterar:

- nomenclatura pública central sem decisão;
- workspaces;
- selection domains;
- shell conceptual;
- tool semantics.

---

# 60. Não fazer big-bang rewrite

Apesar de profunda, a mudança deve ser incremental.

Sempre:

```text
old product path
  ↓
new adapter/component
  ↓
parity test
  ↓
switch caller
  ↓
remove old helper
```

Não criar uma segunda UI paralela inteira.

---

# 61. Exemplo de refatoração — Vector3Field

## Antes

```rust
let avail = ui.available_width();

if avail >= 300.0 {
    ...
} else if avail >= 190.0 {
    ...
} else {
    ...
}

let field_w = ...
ui.horizontal(...)
```

## Depois — chamada

```rust
PetuniaVectorField::new(...)
    .show(ui, state);
```

## Interno

```text
PetuniaVectorField
  ↓
PetuniaResponsiveFieldLayout
  ↓
PetuniaTaffyAdapter
```

A largura dos children deriva do layout.

O componente só declara:

```text
axis label
numeric control
min usable field size
preferred compact policy
```

---

# 62. Exemplo de refatoração — Toolbar

## Antes

```text
measure text
sum buttons
sum gaps
sum badges
compare with available width
hide cluster
```

## Depois

```text
ToolbarModel
├ Domain priority=mandatory
├ Menus priority=mandatory
├ Transform priority=secondary
├ SnapProp priority=secondary
└ Display priority=mandatory
        ↓
PetuniaResponsiveToolbar
        ↓
Taffy layout + Petunia overflow policy
```

---

# 63. Exemplo de refatoração — Right Dock

## Antes

```text
Panel + custom split math + custom state
```

## Depois

```text
PetuniaWorkspaceLayoutDto
       ↓
PetuniaLayoutAdapter
       ↓
egui_tiles
       ↓
render pane by PetuniaPane
```

Nada em domain conhece `TileId`.

---

# 64. Exemplo de refatoração — reorder

## Antes

```text
[↑] [↓]
```

## Depois

```text
drag handle
    ↓
PetuniaDragAdapter
    ↓
visual reorder
    ↓
drop
    ↓
typed command
```

Keyboard:

```text
Alt+Up / Alt+Down
```

ou comandos acessíveis equivalentes.

---

# 65. Verificação estática pós-wave

Ao final de cada wave:

```bash
rg 'available_width\(\)' crates/ui/src
rg 'ui\.horizontal\(' crates/ui/src
rg 'spacing_mut\(\)' crates/ui/src
rg 'allocate_exact_size' crates/ui/src
rg 'egui_taffy::' crates/ui/src
rg 'egui_tiles::' crates/ui/src
rg 'egui_dnd::' crates/ui/src
rg 'twill::' crates/ui/src
```

O objetivo não é zero de todos.

O objetivo é:

```text
raw complexity migrando para adapters/components
product panels ficando declarativos
```

Registrar delta de contagem na auditoria.

---

# 66. Build gates

Obrigatórios:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo xtask arch-check
cargo xtask ui-check
cargo xtask docs-check
cargo xtask ui-guard
```

Quando features opcionais compatíveis forem usadas, incluir matrix específica.

---

# 67. Dependency gate e source governance

Após qualquer crate, Git pin ou fork:

```bash
cargo tree -d
cargo tree -i egui
cargo tree -i eframe
cargo tree -i egui-wgpu
```

Falhar a wave se o shipping graph contiver duas famílias egui.

## Git dependencies

Proibido:

```toml
branch = "main"
```

em produto.

Obrigatório:

```toml
rev = "<SHA>"
```

## Forks

Fork deve possuir:

```text
docs/dependencies/forks/<crate>.md
```

e:

```text
upstream
base revision
patch revision
diff scope
license
tests
exit condition
```

## Hello Egui

Se mais de uma crate do monorepo for usada via git, todas devem vir, preferencialmente, do mesmo SHA.

## Crates.io upgrades

A cada atualização de lockfile relevante:

1. verificar se release oficial substitui pin/fork;
2. testar adapter;
3. remover override quando possível.

Exceção dev-only de segunda família egui exige ADR e não pode entrar no release graph.

---

# 68. Segurança e plugin boundary

Nada desta iniciativa muda:

```text
Plugin
  ↓
Petunia UI Extension API
  ↓
Petunia Components
```

Plugins continuam sem:

```text
&mut egui::Ui
egui::Context
wgpu::Device
raw painter
raw layout tree
```

Se Petunia Components mudarem de backend, plugins não quebram semanticamente.

---

# 69. Acessibilidade

Todo novo adapter/component deve preservar:

- widget roles;
- labels;
- focus;
- keyboard activation;
- focus ring;
- minimum hit areas;
- reduced motion;
- high contrast.

DnD nunca pode ser única forma de reorder.

Tooltips não substituem accessible names.

---

# 70. i18n

Layouts precisam ser testados com:

- English;
- pt-BR;
- pseudo-locale expandida.

Proibido usar largura assumindo tamanho do English.

Taffy deve reduzir a necessidade de heurística por texto.

---

# 71. Performance philosophy

A nova filosofia **não é usar helper crate em tudo**.

É:

```text
use abstraction where it removes repeated manual layout machinery
```

Um simples:

```rust
ui.horizontal(|ui| {
    icon;
    label;
});
```

não precisa de Taffy.

Um responsive property grid precisa.

---

# 72. Critério de sucesso arquitetural

O melhor indicador não é quantidade de crates usadas.

É a diferença entre:

## Antes

```text
"mude o tamanho do Context"
→ 8 arquivos quebram
```

## Depois

```text
"mude o tamanho do Context"
→ token/layout policy muda
→ components recompõem
→ snapshots mostram diferenças esperadas
```

---

# 73. Checklist final do code agent

Antes de declarar esta iniciativa completa, responder **sim** a tudo:

- [ ] documentação não se contradiz sobre `egui_tiles`;
- [ ] Taffy está em product paths críticos;
- [ ] Inspector não contém breakpoints manuais antigos;
- [ ] viewport toolbar não soma dezenas de widths manualmente;
- [ ] Top Bar usa layout estruturado;
- [ ] macro shell está atrás de `PetuniaLayoutAdapter`;
- [ ] no unrestricted docking;
- [ ] drag/reorder usa adapter;
- [ ] motion usa tokens;
- [ ] Twill não vaza;
- [ ] nenhum helper crate vaza para Core;
- [ ] não há segunda família egui;
- [ ] component gallery existe;
- [ ] ui-guard existe;
- [ ] visual regression existe;
- [ ] resolution/scaling matrix passa;
- [ ] i18n long strings passa;
- [ ] high contrast passa;
- [ ] performance gates passam;
- [ ] docs-check passa;
- [ ] Notion/Livro Vivo atualizado;
- [ ] histórico preservado com notas de supersession;
- [ ] final report compara baseline e resultado.

---

# 74. Relatório final obrigatório

Criar:

```text
docs/audits/ui-ecosystem-final-push/99-final-report.md
```

Conteúdo:

1. baseline;
2. problems found;
3. code evidence;
4. docs conflicts;
5. dependencies added/removed;
6. adapters created;
7. components migrated;
8. raw-layout count before/after;
9. screenshots;
10. performance before/after;
11. accessibility;
12. i18n;
13. tests;
14. known issues;
15. scorecard;
16. recommendation:

```text
KEEP EGUI
ou
MIGRATE — SLINT SPIKE
ou
MIGRATE — ICED SPIKE
```

A recomendação deve ser sustentada por evidência.

---

# 75. Evidências principais resumidas

| Evidência | Local | Diagnóstico |
|---|---|---|
| Taffy instalado | `Cargo.toml`, `crates/ui/Cargo.toml` | capability disponível |
| Taffy praticamente só em piloto | `flex_layout.rs` | subutilização |
| política diz native horizontal default | `flex_layout.rs` | agente é incentivado a evitar Taffy |
| Vector3 manual | `inspector_widgets.rs` | breakpoints/matemática no widget |
| Tabs manuais | `inspector_widgets.rs` | largura dividida manualmente |
| Viewport widths manuais | `viewport_bar.rs` | mini layout engine próprio |
| Toolbar chunks/widths | `toolbar.rs` | grid manual |
| Reference 560px branch | `reference_manager.rs` | responsive branch manual |
| ui.horizontal espalhado | diversos arquivos | estrutura repetida |
| Twill piloto | `twill_bridge.rs` | design foundation incompleta |
| Inbox bridge sem usos detectados | `inbox_bridge.rs` | adapter não significa product adoption |
| Docs dizem `egui_tiles` baseline | Livro Vivo / manual | uma autoridade |
| Dev architecture diz sem tiles | `docs/developers/ui-architecture.md` | contradição |
| Component map diz tiles removido | `ui-component-map.md` | contradição |
| flex_layout diz tiles continua macro engine | `flex_layout.rs` | contradição |
| Phase E “100% complete” | audit 40 | integração medida por presença/piloto |

---

# 76. Fontes técnicas externas consultadas — revisão V2

## egui

https://github.com/emilk/egui

## egui_taffy

https://github.com/PPakalns/egui_taffy

Estado verificado:
- `0.14`;
- `egui 0.36`;
- Block/Flexbox/Grid.

## egui_tiles

https://github.com/rerun-io/egui_tiles/blob/main/Cargo.toml

Estado verificado:
- `0.17.1`;
- `egui 0.36.0`;
- `egui_kittest 0.36.0`;
- tiling/layout, tabs, drag e resizing.

## egui_table

https://github.com/rerun-io/egui_table/blob/main/Cargo.toml

**Correção da V1.**

Estado verificado:
- workspace `0.10.0`;
- `egui 0.36.0`;
- `eframe 0.36.0`;
- `egui_kittest 0.36.0`.

Portanto não deve mais ser tratado como bloqueado.

## Hello Egui

https://github.com/lucasmerlin/hello_egui/blob/main/Cargo.toml

Estado verificado do workspace:

```text
hello_egui          0.13.0
egui_dnd            0.17.0
egui_animation      0.13.0
egui_form           0.10.0
egui_inbox          0.13.0
egui_suspense       0.13.0
egui_virtual_list   0.12.0
egui_infinite_scroll 0.12.0
egui_thumbhash      0.12.0

egui                0.36.0
eframe              0.36.0
egui_extras         0.36.0
egui_kittest        0.36.0
```

Conclusão:
- upstream já porta vários helpers para 0.36;
- se crates.io estiver atrasado, pin de revision é permitido.

## egui-notify

Manifest do release:
https://docs.rs/crate/egui-notify/latest/source/Cargo.toml.orig

Estado verificado:
- `egui-notify 0.22.0`;
- `egui = 0.34`;
- `eframe = 0.34` em dev.

Conclusão:
- não usar release diretamente;
- port mínimo/fork Petunia aprovado;
- revisar upstream periodicamente.

## egui_dock

https://docs.rs/crate/egui_dock/latest

Alternativa ao tiles apenas por ADR; não rodar dois macro-layout engines.

## Twill

Manter core atrás do Petunia adapter; não ativar backend incompatível que duplique egui.

---

---

# 77. Fontes internas obrigatórias a reconciliar

```text
Cargo.toml
crates/ui/Cargo.toml
crates/ui/src/flex_layout.rs
crates/ui/src/twill_bridge.rs
crates/ui/src/inbox_bridge.rs
crates/ui/src/inspector_widgets.rs
crates/ui/src/viewport_bar.rs
crates/ui/src/toolbar.rs
crates/ui/src/reference_manager.rs
crates/ui/src/lib.rs
docs/developers/ui-architecture.md
docs/developers/ui-component-map.md
docs/manual/interface.md
docs/modernization/06-p0-to-p3-dependency-policy.md
docs/modernization/08-phase-e-p1-integration-wave.md
docs/modernization/full-gauntlet.md
docs/audits/stack-modernization/40-p1-integration-gauntlet.md
Livro Vivo 35
Livro Vivo 36
```

---

# 78. Comando inicial sugerido ao code agent

O agente deve iniciar assim, conceitualmente:

```text
Read PETUNIA3D_EGUI_ECOSYSTEM_FINAL_PUSH_DIRECTIVE.md completely.

Do not start by changing visuals.

First:
1. audit current branch against every evidence item;
2. report divergences from this document;
3. reconcile documentation authority;
4. establish baseline screenshots/performance;
5. implement adapters/foundation;
6. migrate one product path at a time;
7. run the Gauntlet after every wave.

Treat the directive as normative for this intervention.
Do not use raw egui layout for complex responsive product UI when a Petunia
layout/component adapter exists.
Do not add any dependency that introduces another egui major/minor family.
Do not change the Geometry Core or renderer architecture.
Do not declare completion based only on compilation or pilot integration.
```

---

# 79. Matriz de sourcing de dependências UI

| Crate | Target | Source preferida | Fallback | Petunia boundary |
|---|---:|---|---|---|
| egui_taffy | 0.14 | crates.io | upstream SHA | `PetuniaTaffyLayout` |
| egui_tiles | 0.17.1 | crates.io | upstream SHA | `PetuniaLayoutAdapter` |
| egui_table | 0.10 | crates.io/upstream | pinned SHA | `PetuniaTable` |
| egui_dnd | 0.17 | crates.io ou same Hello Egui SHA | pinned SHA | `PetuniaDragAdapter` |
| egui_animation | 0.13 | crates.io ou same Hello Egui SHA | pinned SHA | `PetuniaMotion` |
| egui_inbox | 0.13 | crates.io | same Hello Egui SHA | `PetuniaInboxAdapter` |
| egui_suspense | 0.13 | release quando disponível | Hello Egui SHA | `PetuniaAsyncView` |
| egui_virtual_list | 0.12 | release quando disponível | Hello Egui SHA | `PetuniaVirtualCollection` |
| egui_form | 0.10 | release quando disponível | Hello Egui SHA | `PetuniaForm` |
| egui-notify | 0.22 port | upstream future | **Petunia fork** | `PetuniaToastHost` |

---

# 80. Política de monorepo Hello Egui

Se o Petunia usar crates do `hello_egui` por git:

```text
NÃO:
suspense @ SHA-A
virtual_list @ SHA-B
form @ SHA-C
```

Preferir:

```text
HELLO_EGUI_REV = SHA-X

suspense     @ SHA-X
virtual_list @ SHA-X
form         @ SHA-X
dnd          @ SHA-X
animation    @ SHA-X
```

Motivo:

- mesmos dependency constraints;
- mesmo egui;
- mesmo ciclo de testes;
- upgrade coordenado;
- troubleshooting simplificado.

Registrar SHA único em:

```text
docs/dependencies/ui-ecosystem-lock.md
```

---

# 81. Política de fork mínimo — regra “manifest first”

Antes de alterar source code de uma crate:

1. fork;
2. alterar apenas `Cargo.toml` para egui 0.36;
3. compilar;
4. deixar o compilador indicar incompatibilidades;
5. fazer menor patch possível.

Isto evita “modernização preventiva” desnecessária.

Exemplo:

```text
egui-notify upstream
  Cargo.toml: egui 0.34
       ↓
fork
  Cargo.toml: egui 0.36
       ↓
cargo check
       ↓
0 errors?
  → source code permanece intacto
```

Se houver erros:

```text
fix exact API changes only
```

---

# 82. Política de manutenção upstream

Para cada pin/fork, manter:

```text
last_checked_upstream
upstream_latest_version
upstream_supports_egui_036
removal_ready
```

O objetivo é reduzir permanentemente código mantido pelo Petunia.

Não criar features novas no fork, a menos que sejam upstreamáveis e acompanhadas de issue/PR upstream.

---

# 83. Testes especiais para patched dependencies

Todo patched dependency deve ter:

```text
compile test
minimal API smoke test
Petunia adapter test
screenshot or behavior test where visual
dependency graph assertion
```

No caso de `egui-notify`:

```text
info toast
warning toast
error toast
multiple toast stacking
expiration
manual close
high DPI
high contrast
theme switch
```

---

# 84. CI — single-egui invariant

Adicionar um gate automatizado que falhe se shipping dependencies apresentarem mais de uma versão de:

```text
egui
ecolor
emath
epaint
eframe
egui-wgpu
```

O script deve distinguir dev-only tooling quando explicitamente whitelisted.

Objetivo:

> tornar impossível um code agent “resolver” incompatibilidade simplesmente aceitando duas árvores egui.

---

# 85. Atualização da definição de “dependency blocked”

A partir desta V2, uma dependency só pode ser marcada como **BLOCKED** depois de:

1. verificar crates.io;
2. verificar upstream main/tags;
3. tentar pin seguro se upstream já estiver portado;
4. estimar port mínimo se não estiver;
5. avaliar substituto compatível.

Estados válidos:

```text
AVAILABLE_RELEASE
AVAILABLE_PINNED_UPSTREAM
PATCHABLE_MINIMAL
LOCAL_ADAPTER_TEMPORARY
DEFERRED_COMPLEX_PORT
REJECTED_ARCHITECTURAL
```

Usar simplesmente:

```text
BLOCKED
```

sem esta investigação é proibido na documentação de stack.

---

# 86. Correções explícitas em relação à V1 desta diretiva

Esta V2 supersede as afirmações da V1 que classificavam:

```text
egui_table
egui_suspense
egui_virtual_list
```

como indisponíveis/bloqueados de forma geral.

O estado correto em 2026-09-16 é:

```text
egui_table 0.10
→ AVAILABLE_RELEASE / egui 0.36

egui_suspense 0.13 upstream
→ AVAILABLE_PINNED_UPSTREAM / egui 0.36

egui_virtual_list 0.12 upstream
→ AVAILABLE_PINNED_UPSTREAM / egui 0.36

egui_form 0.10 upstream
→ AVAILABLE_PINNED_UPSTREAM / egui 0.36

egui-notify 0.22
→ PATCHABLE_MINIMAL / upstream release still egui 0.34
```

Nenhum code agent deve voltar às decisões V1 antigas.

---

# 87. Decisão final desta diretiva

A conclusão desta auditoria não é:

> “egui está condenado”.

Também não é:

> “instalar mais crates resolve automaticamente”.

A conclusão é:

> **A arquitetura atual não usufrui suficientemente do ecossistema que o projeto já escolheu, e parte da própria documentação instrui o code agent a continuar reproduzindo manualmente problemas de layout que bibliotecas especializadas podem resolver.**

A última tentativa com egui só é válida se for uma tentativa **arquitetural**:

```text
egui runtime
+
specialized layout engines
+
controlled adapters
+
Petunia Components
+
visual tests
+
agent guardrails
```

Se isso ainda não entregar a liberdade visual, estabilidade e facilidade de manutenção necessárias, teremos evidência forte e objetiva para migrar para Slint ou iced.

Até esse teste existir, migrar agora significaria abandonar egui **antes de testar a configuração que realmente deveria ter sido usada**.

A V2 acrescenta um princípio final:

> **O Petunia não deve confundir atraso de publicação no crates.io com incapacidade técnica do ecossistema.**

Se upstream já migrou, pinamos uma revision. Se o port é trivial, mantemos um fork mínimo e removível. O critério continua sendo uma única família egui, boundaries Petunia e evidência por testes.
