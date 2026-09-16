# Escopo do Projeto — Petunia3D

Contrato de escopo usado para impedir *feature creep*. Itens podem mudar por decisão
explícita de produto, mas **não** migram silenciosamente entre as categorias abaixo.

Autoridade: [06 — Escopo Essencial](../bible/foundations/06-escopo-essencial.md) e
[12 — Baseline Funcional, Roadmap e Contrato de Escopo](../bible/foundations/12-baseline-funcional-roadmap-escopo.md);
o roadmap pós-GA (P3D-144 a P3D-168) vive no
[hub de Especificações P3D](../bible/especificacoes-p3d-readiness-gauntlet-waves.md).

## Marcadores de status

| Marcador | Significado |
| :--- | :--- |
| `Core V1` | Requisito obrigatório da experiência principal. |
| `Official Extension` | Mantida oficialmente, isolada do core quando isso reduz acoplamento. |
| `V1.x` | Planejada após a primeira baseline; não bloqueia o MVP. |
| `Community Plugin` | Extensão opcional; não gera requisito de instalação padrão. |
| `Experimental` | Pesquisa/avaliação; gera roadmap, não task obrigatória. |
| `Out of Scope` | Fora do baseline; introduzir exige decisão explícita de escopo. |

## Regra de produto

Uma funcionalidade entra no núcleo somente se reduzir significativamente uma destas
fricções: criar a silhueta · transformar silhueta em volume · ajustar forma low-poly ·
unir partes · texturizar/UV · validar para game engine · recuperar/entender o que o
usuário fez.

## Core V1 — experiência obrigatória

### Referências e câmera
- Reference Views e Reference Sets (Front/Side/Back/Top).
- Ortográfica automática durante tracing; perspectiva automática ao orbitar.
- Opacity, lock, x-ray e centerline; alinhamento manual previsível.
- **Work planes conhecidos no espaço 3D** — um profile é uma forma plana no 3D, não um
  desenho 2D que "adivinha" volume.
- Landmark Alignment fica para `V1.x`.

### Criação
- Profiles poligonais: polyline/polygon, rectangle, circle/arc segmentados.
- Primitivas low-poly básicas, **incluindo `Capsule`** (`Star` não faz parte do Core V1).
- Dois caminhos igualmente válidos: `Profile → Extrude/Shape` e `Primitive → Direct Edit`.

### Shape
- Move / Rotate / Scale.
- Extrude e Push/Pull (preferindo operações topológicas locais).
- Mirror; Duplicate; Inset básico; Cut/Slice simples.
- **Round Edge (Bevel/Chamfer) com 1 segmento** — multi-segmento é avançado/posterior.
- Edição de `Point` / `Edge` / `Face` como **escape hatch avançado**; o fluxo inicial
  não exige que o iniciante a use.
- Dissolve/Weld/Bridge/Fill mínimos para reparo e conexão.
- **Revolve 360°** com segmentos radiais explícitos.
- **Combine** com `Keep Parts`, `Join`, `Fuse` e `Connect`. `Group/Assembly` não é
  ação pública do Combine V1; Weld/Stitch/Bridge são mecanismos técnicos usados por Connect.

### Booleans
Boolean não é paradigma exposto. As ações de usuário são **Fuse** (Union quando a fusão
é real) e **Cut** (Difference quando a operação local não resolve). `Intersect` não é
necessário como ação principal da V1, não há remesh automático e o cleanup é apenas o
seguro (degenerates, welds coincidentes, dissolução coplanar). Preview e Undo são
obrigatórios.

### Topologia e representação
- Authoring com tri/quad/n-gon; `PetuniaMesh` próprio em half-edge como representação
  principal, com handles generacionais tipados.
- Triangulation cache determinística; **Show Triangulation** e **Flip Diagonal**.
- Flat/Smooth + sharp edges; **Flat é default**, Smooth é opção secundária.
- Contagem discreta de pontos/faces/triângulos sempre disponível.

### UV e textura
- `UV0` único; UV representation nativa; seams e chart boundaries.
- **Auto UV** orientada a low-poly; **Project From Reference/View**; packing básico.
- Texel/pixel density coerente; checker e avisos de stretch.
- **Paint on Model** (canal Albedo) com conjunto pequeno de ferramentas.
- Palette simples + recent colors com import/export; **Pixel Grid** contextual.
- Editor 2D de textura **opcional, fechado por padrão**, compartilhando textura e Undo
  do Paint 3D.

### Viewport
Modos-base `Wireframe / Solid / Textured / Silhouette`; overlays como composição, não
multiplicação de modos. `Unlit` suportado na V1 como opção secundária.

### Robustez e export
- Undo/Redo transacional; auto recovery/autosave; validação básica de topologia.
- Export prioritário **GLB/glTF 2.0**; **OBJ secundário**; **FBX fora da V1**.
- Triangulação previsível no pipeline de export.

## Official Extension / V1.x

Simple Sweep (`closed polygon Profile + open polyline Path`, parallel transport,
densidade explícita) · Landmark Alignment e calibração multi-referência · `Star` como
conveniência de Profile · Decals · Multi-view photo projection · Advanced UV providers ·
`Intersect` · Exporters adicionais · Validators/helpers específicos de engine.

## Experimental / pesquisa

Inflate Profile (sketch-based modeling) · Guides sobre superfície · Trace Depth e
múltiplas silhuetas · Smart Fuse com cleanup sofisticado · Best-view face assignment ·
Procedural generators especializados.

## Out of Scope (fora do produto-base)

NURBS/B-Rep e CAD kernel · Sculpting suite · Geometry Nodes completo · Remesh/retopology
profissional · Physics, fluids, cloth e particles · Compositor e render engine completo ·
Shader graph avançado · **Loft como requisito oficial** · Simulações e ferramentas de
cena que não contribuam para criação de assets low-poly.

> **Nós e procedimentos:** existem nós simples de textura/recipes catalogados
> (P3D-113 e P3D-164, pós-V1) e um **Modifier Stack simples** (P3D-157, pós-V1). O que
> está fora é **Geometry Nodes completo / shader graph avançado**, não a existência de
> automação procedural simples.

## Extensibilidade e fronteiras arquiteturais

- Extensibilidade faz parte da arquitetura V1 para impedir que o crescimento funcional
  obrigue core e UI principal a crescerem indefinidamente.
- Superfície pública de terceiros: **Lua** + Petunia UI Extension API (sem acesso cru a
  egui/wgpu); MCP como adapter sobre a Application API.
- Core e domínio **não dependem** de egui, widgets, cores, ícones ou teclas físicas;
  strings usam `TextId`, ícones `IconId`, aparência `ThemeToken`, ações `CommandId`.
- O workspace declara **19 membros** em `Cargo.toml` (`core`, `mesh`, `commands`,
  `config`, `project`, `plugins`, `mcp`, `render`, `render-gl`, `render-wgpu`,
  `module-model`, `module-paint`, `module-uv`, `module-assets`, `ui`, `app`, `cli`,
  `ffi`, `xtask`). Contagens antigas de 13/14/17 estavam desatualizadas; a autoridade é
  o [capítulo 28](../bible/foundations/28-arquitetura-rust-cargo-crates.md).

## Restrições de compatibilidade

- **Stack final:** Rust 2024 + egui + eframe + **egui-wgpu + wgpu** + Geometry Core
  próprio, com o viewport integrado por `egui-wgpu` (capítulo 27 e 36).
- Requisito transversal: **performance em máquinas modestas** (o alvo é uso confortável
  em laptops de 4 GB de RAM, não uma estação de trabalho de alto padrão).
- Formato `.petunia` versionado, com save atômico, recovery separado e compatibilidade
  entre versões menores registrada por versão semântica.
- Tema oficial da V1 é **Dark** (+ High Contrast de acessibilidade); temas extras entram
  como `.petunia-theme` declarativo.
