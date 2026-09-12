# 08 — Photo Projection, Trace & Project e Lições do Shapr3D

O Petunia3D deve tratar **imagem de referência**, **profile/shape** e **projeção de textura** como conceitos separados, porém integrados em um único fluxo simples.

## O que o desenho do Petunia realmente é

O desenho não é um rabisco 2D ambíguo a partir do qual o software tenta inferir todo o 3D. Ele é um **profile/sketch preso a um plano conhecido dentro do espaço 3D**.

Fluxo-base:

```plain text
Reference View / Work Plane
        ↓
Profile (polygon/polyline/circle/arc; Bézier opcional depois)
        ↓
Extrude / Push-Pull / Cut / Revolve
        ↓
Generated Mesh
        ↓
Point / Edge / Face edit quando necessário
```

Isso aproxima o conceito de sketches do Shapr3D e de curves/splines do Blender, mas com uma diferença: o Petunia gera mesh low-poly previsível desde o início, sem usar NURBS/B-Rep como representação principal.

## Photo Projection / Project from Reference

Adicionar uma ferramenta de primeira classe para aplicar a própria foto de referência sobre a geometria.

Fluxo ideal:

```plain text
Import Photo
   ↓
Trace silhouette / shapes
   ↓
Extrude / model depth
   ↓
Project from Reference
   ↓
Photo becomes UV projection on selected faces
```

A referência usada para modelar já conhece imagem, enquadramento, proporção e plano/câmera. Esses mesmos dados devem ser reutilizados para criar UVs sem exigir que o usuário abra um editor UV.

### Modos de projeção planejados

- **From Reference/View** — projeção ortográfica ou de câmera a partir da imagem ativa.
- **Selected Faces** — limita a projeção às faces escolhidas.
- **Front / Side / Top / Back** — projeções por reference set.
- **Planar** — para fachadas, placas, paredes e props planos.
- **Cylindrical** — futura conveniência para latas, braços simples, postes etc.
- **Decal** — imagem não destrutiva posicionada sobre uma parte da superfície.

## Trace & Project

Feature de identidade proposta: ao importar uma imagem e modelar diretamente sobre ela, o Petunia pode oferecer **Trace & Project**.

Exemplo de prédio:

```plain text
photo.jpg
  ↓
trace facade silhouette
  ↓
extrude building depth
  ↓
Project Photo
  ↓
front facade receives the photograph automatically
```

A projeção frontal é excelente para faces quase paralelas ao plano da foto. Faces que recuam em profundidade podem sofrer stretching; o programa deve avisar e permitir adicionar Side/Top/Custom projection em vez de esconder o problema.

## Multi-view photo texturing

O Reference Set pode evoluir para também dirigir textura:

```plain text
Front photo → front-facing faces
Side photo  → side-facing faces
Top photo   → upper faces
```

O Petunia poderá atribuir cada face à melhor projeção de forma explícita e previsível, mantendo controle artístico e evitando reconstrução fotogramétrica pesada.

## Foto como textura retrô

Fotos reais podem carregar iluminação, sombra e highlights já capturados. Para estética PS1/N64/PS2 isso é desejável. Portanto Textured View deve funcionar bem com **Unlit** ou iluminação reduzida, evitando dupla iluminação visual.

## Referência do Shapr3D

O Shapr3D é uma forte referência de UX, não de geometry core.

Adaptar:

- imagem importada com opacity para tracing;
- sketch/profile como base de criação;
- extrude por manipulação direta;
- operação contextual: novo volume, adicionar, subtrair conforme direção/contexto;
- UI adaptativa por seleção;
- projeção de sketches/edges sobre faces;
- decals com scale/rotate/opacity e conveniências de wrap;
- history simples combinada com edição direta.

Não adaptar como núcleo:

- Parasolid/B-Rep;
- precisão/manufacturing CAD como objetivo primário;
- sistema pesado de constraints e dimensions;
- dependência de NURBS/solids matemáticos.

## Regra de UX

**A imagem deve poder cumprir três papéis sem configuração duplicada:**

1. referência para desenhar;
2. guia para alinhar a geometria;
3. fonte para projetar a textura.

O usuário não deve precisar recriar câmera, material, Image Texture, UV Project e associação manual como em workflows genéricos de DCC.

## Implementação recomendada

Representar uma `ReferenceView` com dados de imagem e projeção. A projeção UV reutiliza a transformação da referência.

```plain text
ReferenceView
├── image
├── projectionType (orthographic / perspective)
├── transform / camera
├── aspectRatio
└── crop

PhotoProjection
├── referenceViewId
├── targetFaces
├── projectionTransform
├── blend / opacity
└── bakedToUV
```

Durante authoring, PhotoProjection pode ser não destrutiva. No export, ela pode ser consolidada/baked em UV + textura compatível com o formato de destino.

## Escopo inicial recomendado

V1: Project from Reference em faces selecionadas + ortho references + bake para UV.

V1.x: Decals e múltiplas projections por objeto.

Depois: blending entre views, cylindrical projection e ferramentas de correção de seams/stretch.
