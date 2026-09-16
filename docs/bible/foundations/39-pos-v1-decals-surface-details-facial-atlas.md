# 39 — Pós-V1: Decals, Surface Details e Facial Atlases

<aside>
🎭

**Decals passam a ser prioridade alta pós-V1** no Petunia3D. O objetivo é oferecer uma camada simples, game-ready e reutilizável de detalhes de superfície para personagens, roupas, props, veículos, arquitetura e objetos, integrando UV, Paint, Material Variants, animação simples e exportação.

</aside>

# Visão

Decals não devem ser tratados apenas como "adesivos". No Petunia eles formam uma abstração de **Surface Detail** que pode representar:

- cicatrizes, tatuagens e maquiagem;
- olhos, íris, pupilas, sobrancelhas e bocas estilizadas;
- logos, patches, bolsos falsos, costuras, estampas e desgaste em roupas;
- etiquetas, números, placas, sinais, adesivos e serial numbers em props;
- rachaduras, sujeira, ferrugem, manchas, marcas de impacto e damage variants;
- detalhes técnicos simples que não justificam geometria adicional.

# Princípio de produto

O usuário deve poder **arrastar um decal da biblioteca para a superfície, posicioná-lo visualmente e continuar trabalhando** sem precisar compreender shaders complexos, UV math ou nós.

A implementação interna pode usar UV, projeção, mesh decal ou baking conforme o caso, mas a UX deve apresentar uma única ideia coerente: `Place Surface Detail`.

# Tipos de Decal

## UV Decal

Decal ancorado a uma região UV do objeto. É o modo mais previsível para detalhes que devem acompanhar deformation/skinning e para assets que serão baked.

## Surface / Mesh Decal

Pequena geometria que acompanha a superfície. Útil quando o detalhe precisa envolver bordas ou manter UV próprio. Deve possuir offset mínimo e ferramentas contra z-fighting.

## Projected Decal

Projeção local por volume/plano. Excelente para placement rápido, porém precisa de fallback de bake para engines/plataformas onde decal runtime não é desejado.

## Baked Decal

O Petunia compõe o decal diretamente na textura/atlas final do asset. Este é o fallback mais portável para exportação estática.

# Decal Library

Cada decal deve poder ser salvo como asset reutilizável com:

```rust
pub struct DecalAsset {
    pub id: DecalAssetId,
    pub name: String,
    pub category: DecalCategory,
    pub texture: TextureAssetId,
    pub normal: Option<TextureAssetId>,
    pub roughness: Option<TextureAssetId>,
    pub default_size: Vec2,
    pub pivot: Vec2,
    pub tags: Vec<String>,
}
```

Categorias iniciais sugeridas:

- Face / Eyes
- Face / Brows
- Face / Mouth
- Skin / Scars
- Skin / Tattoos
- Clothes / Logos
- Clothes / Seams
- Props / Labels
- Props / Damage
- Environment / Dirt
- Environment / Cracks

# Parametric Decal Sets

Um **Decal Set** reúne variantes semanticamente equivalentes.

Exemplo:

```
Mouth
├── Neutral
├── Smile
├── Frown
├── Open
├── O
└── Teeth
```

Ou:

```
Eyes
├── Open
├── Half
├── Closed
├── Happy
└── Angry
```

O modelo não precisa conhecer arquivos individuais. Ele expõe uma propriedade:

```
mouth = "Smile"
eyes = "Half"
```

Isso cria um conceito muito útil para games: **model variables / appearance parameters**.

# Decal Atlas

Para reduzir draw calls e facilitar animação, vários decals relacionados devem poder ser empacotados em um atlas.

```
Face Atlas
┌────────┬────────┬────────┐
│ eye 01 │ eye 02 │ eye 03 │
├────────┼────────┼────────┤
│ brow01 │ brow02 │ brow03 │
├────────┼────────┼────────┤
│ mouth1 │ mouth2 │ mouth3 │
└────────┴────────┴────────┘
```

O Decal Set armazena a região UV/atlas de cada frame/variant.

# Animação simples

O módulo de animação poderá oferecer tracks específicos:

```
Face / Mouth
0f   Neutral
5f   Open
8f   O
12f  Open
16f  Neutral
```

Propriedades inicialmente animáveis:

- `variant_index`;
- opacity;
- tint/color;
- local offset;
- scale;
- rotation.

O caso principal é **step animation** para troca de olhos/boca, com interpolação apenas para propriedades contínuas.

# Relação com Morph Targets

Decals e morphs se complementam:

- Decal: detalhe visual barato, ótimo para olhos/boca estilizados.
- Morph: alteração real da silhueta/volume.

Uma expressão pode combinar:

```
Smile Morph 0.35
+
Mouth Decal = Smile_02
+
Eye Decal = Happy
```

# Exportação e portabilidade

## glTF / GLB genérico

- Morph Targets possuem representação e animação padrão no glTF.
- `KHR_materials_variants` pode representar variantes estáticas de materiais.
- `KHR_texture_transform` representa offset/rotation/scale UV, mas não deve ser assumido como um canal de animação genérico.
- `extras` pode transportar metadata específica do Petunia para Decal Sets, parâmetros e mappings.

Portanto, **Decal Animation não deve depender apenas do glTF core**.

## Estratégia de exportação

1. **Static universal:** bake decals no UV/atlas final.
2. **Configurable asset:** exportar variantes + metadata quando o formato/engine suportar.
3. **Engine adapter:** Unity/Godot/Unreal exporters podem converter tracks/variables Petunia para material parameters, atlas offsets ou recursos nativos da engine.
4. **Petunia metadata:** preservar semanticamente `eyes`, `mouth`, `damage_state`, `logo_variant` etc. em metadata quando solicitado.

# UX proposta

```
DECALS
────────────────────────
Library   [ Face ▼ ]

[eye_open] [eye_half] [eye_closed]
[mouth_a ] [mouth_o  ] [mouth_smile]

Mode
● Surface Detail
○ Project
○ UV

[✓] Follow Surface
[✓] Mirror
[ ] Bake on Export

Scale      1.00
Rotation   0°
Opacity    1.00
Tint       White

[ Create Set ]
[ Add to Atlas ]
[ Bake ]
```

# Dificuldade estimada

| Recurso | Dificuldade | Valor |
| --- | --- | --- |
| Static decal + alpha | 3/10 | Muito alto |
| Decal Library | 3/10 | Muito alto |
| UV anchored decal | 4/10 | Muito alto |
| Surface / mesh decal | 5/10 | Alto |
| Projected decal | 5/10 | Alto |
| Decal Set / variants | 4/10 | Muito alto |
| Atlas Builder integration | 4/10 | Muito alto |
| Step animation de variant | 5/10 | Muito alto |
| BaseColor bake | 5/10 | Muito alto |
| Normal/Roughness decals | 6/10 | Alto |
| Engine-specific animated export | 6–7/10 | Alto |

# Prioridade proposta

## Decals M0 — primeiro pós-V1

- static decals;
- Decal Library;
- placement visual;
- transform/tint/opacity;
- UV or Surface attachment;
- Bake to BaseColor;
- integração com Undo/Redo.

## Decals M1

- Decal Sets;
- Atlas integration;
- variables/semantic names;
- Mirror;
- presets.

## Decals M2

- animation track `variant_index`;
- eyes/mouth authoring preset;
- export metadata;
- adapters por engine.

## Decals M3

- normal/roughness decals;
- projection avançada;
- layered sorting;
- baking multi-channel.

# Riscos e guardrails

- evitar dezenas de planos transparentes sobrepostos por padrão;
- exibir overdraw/budget warning em assets com muitos decals alpha;
- evitar z-fighting com offset/controlado;
- preservar sorting determinístico;
- oferecer bake como caminho recomendado para plataformas modestes;
- não transformar o sistema em um compositor 2D completo.

# Referências externas de pesquisa

- [DECALmachine — mesh decals, projection, atlas, trim sheets e baking](https://machin3.io/DECALmachine/docs/)
- [Unreal Engine — Decal Materials](https://dev.epicgames.com/documentation/unreal-engine/decal-materials-in-unreal-engine)
- [Unreal Engine — Mesh Decals](https://dev.epicgames.com/documentation/unreal-engine/using-mesh-decals-in-unreal-engine)
- [Khronos — KHR_materials_variants](https://github.com/KhronosGroup/glTF/tree/main/extensions/2.0/Khronos/KHR_materials_variants)
- [Khronos — KHR_texture_transform](https://github.com/KhronosGroup/glTF/tree/main/extensions/2.0/Khronos/KHR_texture_transform)
- [glTF 2.0 — Morph Targets e Animation](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html)

# Decisão preliminar

**Decals / Surface Details devem ser promovidos a recurso de alta prioridade pós-V1**, antes de sistemas de character authoring mais pesados. A mesma base deve atender personagens, roupas, props e ambientes.

# Decisão de UX — Decal Tool dentro do Surface/Paint

A pesquisa comparativa reforça que **decal não deve ser reduzido a um brush**. O Petunia deve manter uma ferramenta própria de `Place / Edit Decal` com manipulador direto na viewport, enquanto a organização, layers, library, channels e bake permanecem no ecossistema Surface/Paint.

## Modelo de interação

```
Paint / Surface Workspace
├── Brush
├── Eraser
├── Fill
├── Decal              ← ferramenta própria
├── Projection
└── Surface Detail Layers
```

Fluxo principal:

1. Usuário arrasta um Decal Asset da biblioteca para a viewport.
2. O Petunia raycasta a superfície sob o cursor e cria um `DecalInstance` ancorado nela.
3. Um **Surface Manipulator** aparece imediatamente.
4. Arrastar o centro desliza o decal sobre a superfície.
5. Handles simples controlam Scale e Rotation.
6. Inspector contextual controla opacity, tint, mirror, projection mode e channel participation.
7. A instância aparece como uma `Decal Layer`/Surface Detail no stack.
8. O usuário pode continuar manipulando o decal diretamente em 3D ou ajustar sua região UV na visão 2D quando necessário.

## Surface Manipulator

O gizmo de decal deve ser diferente do transform gizmo genérico. O modo principal é **surface-relative**, não XYZ global.

```
        Rotate
          ↻
     ┌──────────┐
     │  decal   │
     └──────────┘
      ↔ Scale

Drag center = Slide on Surface
```

Capacidades:

- Slide on Surface por raycast;
- Tangent-space translation;
- Rotation em torno da surface normal;
- Uniform scale por padrão;
- optional X/Y scale avançado;
- Mirror X/Y;
- offset mínimo contra z-fighting;
- snap opcional a centerline/UV/grid quando fizer sentido;
- reset transform;
- duplicate;
- delete;
- preview before commit.

## Dois níveis de UX

### Simple

- Drag & drop;
- slide;
- rotate;
- scale;
- opacity;
- tint;
- mirror;
- bake/export.

### Advanced

- projection type;
- UV-region editing;
- warp points;
- channel masks;
- depth/falloff;
- sorting/order;
- multi-channel decals.

O modo avançado fica escondido por padrão.

## Relação com Paint

Brush e Decal compartilham:

- TextureResources;
- Layers;
- Masks;
- Channels;
- Undo;
- Color pipeline;
- UV information;
- export/bake.

Mas **não compartilham a mesma interação**. Brush escreve strokes; Decal mantém uma entidade/instância editável até Bake/Flatten.

## Referências de UX pesquisadas

- Adobe Substance 3D Painter permite drag-and-drop de SVG/recursos diretamente na viewport, criando camada/projeção apropriada para decals.
- Planar/Warp Projection possuem manipuladores próprios de translation, rotation, scale e **Surface Manipulator** que desliza a projeção sobre a superfície do modelo.
- A viewport do Painter mantém toolbar contextual por ferramenta, reduzindo a necessidade de expor permanentemente todos os parâmetros.

## Decisão

**Decal Tool oficial na viewport + Surface/Paint como workspace e data model.** Não criar um workspace exclusivo `Decal`; não esconder a manipulação de decals dentro do brush; não exigir UV Editor para placement básico.

# Decisão de UX — Decal Tool dentro do Surface/Paint

A pesquisa comparativa reforça que **decal não deve ser reduzido a um brush**. O Petunia deve manter uma ferramenta própria de `Place / Edit Decal` com manipulador direto na viewport, enquanto a organização, layers, library, channels e bake permanecem no ecossistema Surface/Paint.

## Modelo de interação

```
Paint / Surface Workspace
├── Brush
├── Eraser
├── Fill
├── Decal              ← ferramenta própria
├── Projection
└── Surface Detail Layers
```

Fluxo principal:

1. Usuário arrasta um Decal Asset da biblioteca para a viewport.
2. O Petunia raycasta a superfície sob o cursor e cria um `DecalInstance` ancorado nela.
3. Um **Surface Manipulator** aparece imediatamente.
4. Arrastar o centro desliza o decal sobre a superfície.
5. Handles simples controlam Scale e Rotation.
6. Inspector contextual controla opacity, tint, mirror, projection mode e channel participation.
7. A instância aparece como uma `Decal Layer`/Surface Detail no stack.
8. O usuário pode continuar manipulando o decal diretamente em 3D ou ajustar sua região UV na visão 2D quando necessário.

## Surface Manipulator

O gizmo de decal deve ser diferente do transform gizmo genérico. O modo principal é **surface-relative**, não XYZ global.

```
        Rotate
          ↻
     ┌──────────┐
     │  decal   │
     └──────────┘
      ↔ Scale

Drag center = Slide on Surface
```

Capacidades:

- Slide on Surface por raycast;
- Tangent-space translation;
- Rotation em torno da surface normal;
- Uniform scale por padrão;
- optional X/Y scale avançado;
- Mirror X/Y;
- offset mínimo contra z-fighting;
- snap opcional a centerline/UV/grid quando fizer sentido;
- reset transform;
- duplicate;
- delete;
- preview before commit.

## Dois níveis de UX

### Simple

- Drag & drop;
- slide;
- rotate;
- scale;
- opacity;
- tint;
- mirror;
- bake/export.

### Advanced

- projection type;
- UV-region editing;
- warp points;
- channel masks;
- depth/falloff;
- sorting/order;
- multi-channel decals.

O modo avançado fica escondido por padrão.

## Relação com Paint

Brush e Decal compartilham TextureResources, Layers, Masks, Channels, Undo, Color pipeline, UV information e export/bake. Mas **não compartilham a mesma interação**: Brush escreve strokes; Decal mantém uma entidade/instância editável até Bake/Flatten.

## Decisão

**Decal Tool oficial na viewport + Surface/Paint como workspace e data model.** Não criar um workspace exclusivo `Decal`; não esconder a manipulação de decals dentro do brush; não exigir UV Editor para placement básico.

# Nota de pesquisa externa

A decisão acima foi validada contra workflows atuais de texturing 3D: projeções planas/warp usam manipuladores contextuais na viewport, incluindo mover sobre a superfície, rotação e escala, enquanto assets podem ser arrastados diretamente para a malha e convertidos em layers/projections. O Petunia adota a ideia de **manipulação direta + layer editável**, mas mantém o escopo low-poly e game-ready.

# Guardrail de produto

Decals continuam sendo **asset authoring**, não world/level authoring. O Petunia edita decals pertencentes ao asset selecionado; não oferece pintura de decals espalhados por uma cena de game, gerenciamento de decal actors do level, streaming, gameplay triggers ou ferramentas de composição de mapas.

# Relação com Animation e export

O Decal Tool não altera a estratégia já definida: placement e edição permanecem no Surface/Paint; Animation apenas referencia propriedades animáveis da instância (`variant_index`, opacity, tint, local offset, scale, rotation). Bake continua sendo o fallback universal de export.

# Implementation note

Representar a instância de decal separadamente do raster final permite mover/rotacionar/escalar sem recompor permanentemente a textura a cada interação. A composição/bake pode ser cacheada e atualizada após commit, preservando Undo previsível.