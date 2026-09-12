# 12 — Baseline Funcional, Roadmap e Contrato de Escopo

> Este capítulo é o contrato de escopo usado para impedir feature creep. Itens podem mudar por decisão explícita de produto, mas não devem migrar silenciosamente entre Core, V1.x e Experimental.

# Modelo mental do produto

O usuário deve aprender poucos conceitos:

```plain text
REFERENCE
    ↓
DRAW / CREATE
    ↓
SHAPE
    ↓
PAINT / PROJECT
    ↓
CHECK
    ↓
EXPORT
```

Topologia, triangulação e UV continuam acessíveis, porém não são pré-requisitos para o primeiro asset.

# Core V1 — experiência obrigatória

## Criação e referência

- Reference Views e Reference Sets.
- Front/Side/Top ortográficos e Perspective por orbit.
- Opacity, lock, x-ray e centerline.
- Work planes conhecidos no espaço 3D.
- Profiles poligonais: polyline/polygon, rectangle, circle/arc segmentados.
- Primitivas low-poly básicas, incluindo **Capsule** no Core V1. `Star` não faz parte do Core V1.

## Shape

- Move / Rotate / Scale.
- Extrude.
- Push/Pull local.
- Mirror.
- Bevel/Chamfer simples, priorizando 1 segmento.
- Inset básico.
- Cut/Slice simples.
- Duplicate.
- Point/Edge/Face edit quando necessário.
- Dissolve/Weld/Bridge/Fill mínimos para reparo e conexão.
- Combine com intenções claras: Keep Parts, Join, Fuse e Connect.

## Topology e render representation

- Authoring com tri/quad/n-gon.
- `PetuniaMesh` próprio em half-edge como representação authoring principal, com handles generacionais tipados.
- Triangulation cache determinística.
- Show Triangulation e Flip Diagonal.
- Flat/Smooth + Sharp edges; Flat permanece default e Smooth é opção secundária V1.
- Unlit suportado na V1 como opção secundária de visualização/material.

## UV e textura

- UV representation nativa.
- Auto UV simples orientado a low-poly.
- Project From Reference/View.
- UV packing básico.
- Texel/pixel density coerente.
- Checker/stretch warnings.
- Paint on Model com conjunto pequeno de ferramentas.
- Palette simples + recent colors + import/export de paleta.
- Pixel Grid contextual no Paint.
- Editor 2D de textura **opcional, fechado por padrão**, compartilhando a mesma textura/Undo do Paint 3D.

## Viewport

- Textured.
- Solid.
- Wireframe.
- Silhouette/Reference.
- Overlays em vez de multiplicação de modos.

## Robustez

- Undo/Redo transacional.
- Auto recovery/autosave.
- Validação básica de topology.
- Contagem discreta de vertices/faces/triangles.

## Export

- Prioridade inicial para GLB/glTF e OBJ.
- Triangulação previsível no pipeline de export.

# V1 / módulo oficial: Boolean mínimo

Boolean não é paradigma de modelagem. Ações de usuário são **Fuse** e **Cut**.

- `Fuse` corresponde semanticamente a Union.
- `Cut` usa Difference quando a operação local não é suficiente.
- Extrude e Push/Pull simples preferem operações topológicas locais.
- Through-cut e interseção arbitrária podem delegar a BooleanProvider robusto.
- Intersect não é necessário como ação principal da V1; pode aparecer posteriormente em Advanced.
- Não executar remesh automático após boolean.
- Fazer apenas cleanup seguro: degenerates, welds coincidentes e dissolução coplanar quando comprovadamente segura.
- Preview e Undo obrigatórios.

# Simple Sweep

**Aprovado como direção**, porém deliberadamente simplificado.

Objetivo: criar cabos, tubos, galhos, chifres, corrimões e formas semelhantes a partir de `Profile + Path`, com baixa densidade geométrica explícita.

Não precisa reproduzir sweep CAD completo. O contrato detalhado **já está fechado no capítulo 14**: `closed polygon Profile + open polyline Path`, framing por parallel transport, densidade geométrica explícita e casos de erro previsíveis. Permanece **Official Extension/V1.x**, não requisito do Core V1.

# Loft

**Fora do escopo atual.** O ganho não compensa correspondence entre profiles, vertex matching, twist, resampling e demais casos de borda. Pode ser plugin futuro sem compromisso de entrar no produto oficial.

# Array

Não é requisito central da primeira baseline. Pode ser adicionado como operação simples ou extensão após os workflows principais estarem sólidos. Evitar tratá-lo como indispensável apenas por existir em DCCs tradicionais.

# Photo Projection

Parte importante da identidade:

- Reference, Alignment e Projection reutilizam a mesma imagem/dados.
- Project From Reference gera UV sem exigir editor UV.
- Photo Projection pode permanecer não destrutiva no authoring e ser baked no export.
- Decal é conceito separado de projeção principal.
- Múltiplas referências e best-view projection ficam para evolução após a versão simples estar confiável.

# Official Extensions / V1.x

Candidatos:

- Simple Sweep.
- Landmark Alignment/calibração de múltiplas referências.
- Star como conveniência de Profile, se houver demanda oficial; também pode existir via plugin.
- Decals.
- Multi-view photo projection.
- Advanced UV providers.
- Intersect.
- Exporters adicionais.
- Game-engine specific validators/export helpers.

# Experimental / pesquisa

- Inflate Profile inspirado em sketch-based modeling.
- Guides sobre superfície.
- Trace Depth / múltiplas silhuetas como auxílio de volume.
- Smart Fuse com cleanup mais sofisticado.
- Best-view face assignment para múltiplas fotos.
- Procedural generators especializados.

# Fora do produto-base

- NURBS/B-Rep e CAD kernel.
- Sculpting suite.
- Geometry Nodes completo.
- Remesh/retopology profissional.
- Physics, fluids, cloth e particles.
- Compositor e render engine completo.
- Shader graph avançado.
- Loft como requisito oficial.
- Simulações e ferramentas de cena que não contribuam diretamente para criação de assets low-poly.

# Critério para promover uma feature

Uma feature só sobe para o produto principal quando:

1. resolve uma necessidade frequente do workflow low-poly;
2. não existe composição simples das ferramentas atuais que resolva bem o problema;
3. a UX pode ser explicada com poucos conceitos;
4. a implementação e manutenção são compatíveis com o tamanho do projeto;
5. sua presença permanente na UI não piora a experiência do iniciante.

# Connect / Weld / Bridge — decisão fechada

**Connect** é a ação principal de usuário para criar continuidade topológica entre duas regiões. `Bridge` e `Weld` são mecanismos internos/avançados, não conceitos obrigatórios para o fluxo básico.

- Connect detecta boundaries a partir de duas faces-cap ou loops abertos selecionados.
- Mesma contagem de points: correspondência 1:1, favorecendo quads.
- Contagens diferentes: gerar faixa mista de quads + triangles; não exigir subdivisão manual, LCM, remesh ou quad-only.
- Correspondência automática escolhe orientação/offset de menor custo; correção manual simples via preview pode ajustar o match inicial.
- Sem cuts/interpolation/smoothness avançados no Connect V1.
- Weld une apenas points explicitamente envolvidos ou dentro de tolerância contextual pequena; nunca executar merge-by-distance global silencioso.
- Stitch é comportamento contextual de Connect/Weld, não ferramenta principal separada.
- Operação sempre transacional, com preview, validação e rollback.
- Advanced bridge/surface blending e retopology ficam para extensão futura.

# Fechamento técnico — 2026-09-10

As decisões técnicas não ligadas diretamente à UI/UX/interface/acessibilidade foram congeladas. Os antigos itens pendentes agora possuem contratos especializados nas páginas 14–21.

## Decisões finais incorporadas ao baseline

- Cut superficial = operação topológica local; Through Cut = Difference via provider quando necessário.
- Slice = plano de corte que divide geometry sem remover volume por padrão.
- Bevel/Chamfer Core V1 = **1 segmento**; multi-segment é evolução.
- Revolve 360° entra no Core V1.
- Simple Sweep fica Official Extension/V1.x com `closed Profile + open polyline Path` e parallel-transport framing.
- Normals são derivados; Flat/Smooth + Sharp edges; sem custom split normals V1.
- Auto UV: UV gerada pelos generators quando conhecida, projection quando há reference/view e xatlas para mesh genérica.
- UV0 único na V1; seams e chart boundaries suportados; packing sem overlaps automáticos.
- Paint on Model V1 prioriza Base Color raster; um stroke = uma transaction.
- Material interno segue glTF metallic-roughness + Unlit; Height/Displacement não bloqueia V1.
- `.petunia` = ZIP versionado + JSON authoring data + images embutidas.
- Undo usa snapshots das regiões/objetos afetados; pintura usa tile diffs.
- Save é atômico; recovery usa snapshots completos separados.
- GLB/glTF 2.0 é export principal; OBJ secundário; FBX fora da V1.
- **Rust 2024** é a language baseline final do Application/Geometry Core, conforme ADR 32 e capítulos 27–36; o Geometry Core continua próprio e desacoplado de bibliotecas de terceiros.
- `manifold-rust` é o provider baseline para Boolean 3D arbitrário; xatlas permanece fallback de Auto UV atrás de `UvUnwrapProvider`; tangents usam a crate Rust `mikktspace` somente quando normal maps realmente exigirem.
- GLB/glTF usa a stack Rust `gltf` + `gltf-json` atrás de `Importer`/`Exporter`; OBJ permanece secundário, com writer próprio pequeno para export e `tobj` como parser baseline de import, sempre atrás do `Importer` trait e fixtures de conformance.
- Lua 5.4 é runtime público de plugins, hospedado por `mlua`.
- `petunia-mcp` é implementado em Rust sobre `rmcp`; Tokio fica isolado no serviço MCP e requests entram no Application Core por channel tipado, sem acesso mutável direto ao `Document`.
- Document mutável usa single-writer; jobs pesados operam em snapshots imutáveis com revision checks.

# UI Baseline Final

A plataforma tecnológica e a experiência-base de UI estão congeladas: **Rust + egui + eframe + egui-wgpu + wgpu**, com Petunia Components e os contratos finais do capítulo 36.

O capítulo 36 é a autoridade para composição do shell, layout/medidas, gestures, design tokens finais, shortcuts UX, focus/keyboard navigation, accessibility semantics, viewport adapter, Theme Extensions e Plugin Panels. Pequenas calibrações posteriores são `TUNING`; mudanças estruturais exigem decisão explícita/ADR quando aplicável.

A documentação gerada pelo framework deve tratar decisões explicitamente marcadas como Core/V1 e UI Baseline Final como normativas, e itens Experimental como não comprometidos.
