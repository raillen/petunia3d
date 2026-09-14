# 15 — UV, Textura, Projeção e Materiais — Arquitetura Fechada

> O objetivo técnico do sistema de UV/textura é permitir que a maioria dos usuários pinte ou projete uma imagem sem precisar entender unwrap manual. O editor UV existe como correção/controle, não como rito obrigatório.

# Uma única UV principal na V1

A V1 trabalha com **UV0** como conjunto principal. Múltiplos UV sets, UDIM e workflows de lightmap dedicados ficam fora do core inicial.

# Estratégia Auto UV

Não implementar um algoritmo universal próprio. O sistema escolhe a rota mais simples conforme a origem da geometria.

## 1. UV gerada junto com a criação

Primitivas e generators conhecidos produzem UV previsível no momento da criação:

- Box: projeções por face organizadas em ilhas;
- Cylinder/Cone: side strip + caps;
- Sphere/Icosphere: layout próprio simples;
- Extruded Profile: caps projetadas no Work Plane + strip lateral;
- Revolve: unwrap radial/longitudinal determinístico.

## 2. Project From Reference/View

Quando a imagem já é a referência de modelagem, reutilizar sua transformação para gerar UV planar diretamente nas faces escolhidas.

## 3. Generic Auto UV

Meshes arbitrariamente editadas usam um `UVProvider` externo. O provider oficial inicial será **xatlas**, isolado atrás de interface própria. xatlas gera charts e packing e foi criado para produzir coordenadas UV únicas adequadas a texture painting. [xatlas](https://github.com/jpcy/xatlas)

A aplicação nunca depende de tipos xatlas fora do adapter.

# Seams

Cada Edge pode possuir flag `uv_seam` por UV set.

Regras:

- Auto UV pode derivar seams/charts sem exigir edição manual;
- seams manuais são constraints para o provider, implementadas preparando/splitting adjacency na entrada quando necessário;
- boundaries de material e sharp edges podem influenciar charting, mas não devem obrigatoriamente ser seams;
- após unwrap, boundaries de charts ficam disponíveis como informação derivada;
- `Cut UV` e `Stitch UV` são operações técnicas suportadas, independentemente de como serão apresentadas na interface.

# Packing

O packing padrão prioriza previsibilidade e pintura:

- ilhas únicas, sem overlap automático;
- rotação permitida em passos compatíveis com preservação visual; para pixel workflow, preferir 90°;
- padding configurável em texels;
- suporte a target texel density;
- repack não altera geometria;
- islands explicitamente espelhadas podem compartilhar UV somente quando o usuário/workflow pedir; Auto UV genérico não cria overlap silencioso.

# Pixel / Texel Density

A arquitetura armazena resolução de textura e pode calcular texels-per-unit por island/face.

Suportar:

- normalização automática de density entre islands;
- snap de UV vertices/edges ao pixel grid;
- relatório de density inconsistente;
- modo de density alvo por asset/material.

Não forçar power-of-two: é recomendação/preset futuro, não requisito estrutural.

# Stretch detection

Calcular diferença entre área/forma 3D e UV para produzir warning de stretch. Não tentar corrigir toda distorção automaticamente; o sistema pode sugerir reproject/unwrap/repack.

# Paint on Model

A pintura 3D é uma projeção de brush hits para UV0 por raycast/barycentric interpolation.

## Ferramentas técnicas V1

- hard pixel pencil;
- round brush simples com hardness/size;
- eraser;
- fill sobre textura/região válida;
- color picker;
- line/rectangle como raster operations simples;
- **Palette simples** com cores salvas + recent colors e import/export de paleta;
- **Pixel Grid contextual**, visível em zoom suficiente e desativável.

Um brush stroke inteiro é **uma transaction de Undo**, não uma entrada por sample.

## Editor 2D de textura V1

O Paint workspace também possui um **editor 2D de textura opcional, fechado por padrão**. Ele não é um segundo sistema de pintura: mostra e edita a mesma `TextureBitmap` usada pelo Paint on Model.

Regras:

- Paint 3D e Paint 2D compartilham a mesma textura, palette, Pixel Grid e Undo/Redo;
- o editor 2D oferece o mesmo núcleo simples de Pencil, Brush, Eraser, Fill, Picker e Line/Rectangle;
- nenhuma layer stack complexa, filtros avançados, selection suite ou comportamento de Photoshop entra na V1;
- abrir/fechar o painel 2D altera apenas apresentação, nunca o documento ou a textura;
- mudanças devem aparecer ao vivo na viewport 3D.

Symmetry painting fica V1.x; Mirror geometry continua podendo compartilhar UV explicitamente em workflows futuros.

# Modelo de textura

Para manter implementação pequena, V1 não terá compositor procedural nem layer stack complexo.

`TextureDocument` suporta fontes ordenadas simples:

```plain text
RasterPaint
PhotoProjection
Decal (V1.x)
```

Composição V1 usa alpha normal. Blend modes complexos ficam fora.

A principal textura editável é raster e lossless; pintura/bake usam PNG.

# Photo Projection

`PhotoProjection` é metadado paramétrico associado a uma Reference/View + face set.

- calcula UV derivada no CPU;
- reutiliza a imagem original como source durante authoring;
- continua ajustável enquanto não for baked;
- ao iniciar pintura incompatível com a projeção dinâmica, o sistema pode fazer bake automático para textura raster dentro de uma transaction;
- múltiplas projeções simultâneas e blending entre views são V1.x.

# Decal

Decal é V1.x. É uma imagem localizada com transform/projection próprios e é baked no export ou quando necessário para edição raster. Não criar sistema de materiais decal em runtime como requisito de export.

# Materiais

O modelo interno segue de perto **glTF PBR metallic-roughness** por interoperabilidade:

- Base Color factor/texture;
- Metallic factor/texture;
- Roughness factor/texture;
- Normal texture;
- Emissive factor/texture;
- Alpha mode/cutoff;
- Unlit flag.

`MaterialId` é atribuído por face; um asset pode ter vários materiais, embora workflows simples possam usar apenas um.

## Canais de pintura

Core V1: pintura direta principalmente em **Base Color**.

V1.x/Advanced: Roughness, Metallic e Normal via UV/2D editor.

Height/Displacement pode existir como canal authoring opcional posterior, mas **não é requisito do renderer/export glTF V1**; não adicionar displacement/parallax apenas para justificar o canal.

# Color space

- Base Color e Emissive: sRGB.
- Normal, Roughness, Metallic e dados técnicos: linear.
- photos/references: tratadas como sRGB salvo metadata explícita.

# Normals e tangents

Normals são derivados da geometria/sharp flags. Não armazenar custom split normals editáveis na V1.

Tangents são gerados somente quando necessários por normal mapping, usando a crate Rust **`mikktspace`** ou implementação comprovadamente compatível atrás de adapter simples. Essa dependência é **condicional**: só entra quando normal mapping exigir tangents e não faz parte do caminho mínimo de Base Color/Unlit. [MikkTSpace](https://github.com/mmikk/MikkTSpace)

# Regra final

A regra é: gerar UV cedo quando a operação já conhece sua parametrização; usar projection quando a referência fornece a resposta; recorrer ao provider automático somente para meshes arbitrárias. Isso reduz implementação e torna o resultado mais previsível.
