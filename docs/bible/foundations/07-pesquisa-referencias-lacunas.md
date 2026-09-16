# 07 — Pesquisa: referências, lacunas e ideias acadêmicas

## Produtos estudados

### Blockbench

Pontos fortes: baixo atrito, ferramentas low-poly, background/reference images, modos Textured/Solid/Wireframe, pintura 3D/UV integrada e Auto UV. Lacunas observadas na comunidade: confusão de iniciantes com UV, necessidade de melhor controle de smooth/sharp shading, problemas de workflow quando se sai do estilo Minecraft e pedidos de visualização de stretching/UV.

### picoCAD / picoCAD 2

Confirma o valor de uma ferramenta deliberadamente limitada: editor focado no essencial, wireframe/solid/textured, textura integrada, paletas e estética retro. A principal lição é tratar restrições como parte da UX, não como limitação acidental.

### Crocotile 3D

Mostra um fluxo 2D→3D muito natural usando tiles, com Draw/Edit modes, orthographic/perspective, UV, paint e wireframe. Reforça que o Petunia pode tratar desenho como ferramenta de construção, não como feature lateral.

### Asset Forge

Mostra o valor de montagem rápida por blocos, snapping/placement contextual, mirror, groups e export direto para engines. Isso inspira uma pequena biblioteca de primitivas/prefabs sem transformar o Petunia em kitbasher.

### Dust3D

É particularmente relevante: sketch-to-mesh em canvas 2D e automação de UV/rigging. Reforça a tese de que operações tediosas podem ser automatizadas, mantendo edição posterior.

## Lacunas recorrentes identificadas

- Blender é percebido como poderoso, porém excessivo para usuários que só querem criar assets low-poly.
- Blockbench é percebido como muito acessível, mas usuários de low-poly genérico eventualmente encontram limites fora do paradigma Minecraft/pixel-perfect.
- UV e textura são um ponto de abandono importante para iniciantes.
- Smooth/flat/sharp shading precisa ser previsível, especialmente para assets que serão exportados para engines.
- Triangulação invisível pode causar diferenças de shading entre editor e game engine; por isso precisa ser determinística e inspecionável.

## Pesquisa acadêmica aplicável

### Teddy — SIGGRAPH 1999

A partir de uma silhueta 2D, o sistema infere uma superfície 3D plausível. Para Petunia, a lição não é implementar o algoritmo inteiro na V1, mas considerar futuramente um **Inflate Profile** opcional para formas orgânicas low-poly.

### Sketch-based Modeling Surveys — 2009

A literatura identifica o principal problema do sketch-to-3D: ambiguidade. Decisão para Petunia: evitar inferência mágica no core. O usuário informa contexto explicitamente — plano ativo, face ativa, operação Draw/Profile/Push — e assim obtemos previsibilidade e implementação muito mais simples.

### FiberMesh — SIGGRAPH 2007

Curvas permanecem como handles manipuláveis sobre a superfície. Inspiração futura: linhas desenhadas pelo usuário podem permanecer editáveis como guias de shape/deform, sem exigir NURBS.

### Sketch2CAD — 2020

Interpreta strokes no contexto do modelo parcial. Lição: contextualidade reduz ambiguidade. Podemos aplicar isso deterministicamente: um círculo sobre uma face selecionada significa region/cut; num plano vazio significa profile; arrastar perpendicularmente significa depth.

### GA-Sketching / Sketch2Mesh

Pesquisas recentes mostram valor de refinamento iterativo por múltiplas vistas. Para Petunia, isso valida o workflow Front→Side→Perspective, mas IA/deep learning deve ser pesquisa futura, não dependência do editor básico.

## Pesquisa específica: Booleans em ferramentas comparáveis

A comparação entre ferramentas reforça que Boolean não precisa ser o paradigma central do Petunia:

- **picoCAD** privilegia um conjunto mínimo de operações e não trata booleans como eixo principal do workflow.
- **Asset Forge** enfatiza composição/montagem de blocos; unir conceitualmente um asset não exige sempre uma fusão CSG.
- **Blockbench** possui booleans em ferramentas adicionais/experimentais, o que reforça que mesh editing low-poly funciona sem torná-los fundamento universal.
- **Dust3D** usa boolean/união para combinar partes em determinados workflows, mas a própria evolução do projeto evidencia dificuldades típicas com inputs non-manifold e junções problemáticas.
- **MoI, Plasticity e Shapr3D** tornam booleans muito mais centrais porque operam sobre kernels/sólidos CAD adequados ao problema; copiar essa arquitetura seria custo desnecessário para Petunia.

A lição de UX do Shapr3D permanece valiosa: operações contextuais podem mapear intenção para Add/Union ou Cut/Subtract sem exigir que o iniciante escolha operadores CSG técnicos.

## Estratégia técnica para Boolean

Preferir uma abordagem híbrida:

```
caso simples
→ operação local sobre topology authoring

interseção volumétrica real
→ BooleanProvider especializado
```

Providers candidatos devem ser avaliados por robustez, simplicidade de integração, licença e capacidade de preservar atributos. Um candidato inicial importante é **Manifold**, isolado atrás de contrato próprio. libigl/CGAL podem ser avaliados como alternativas se houver necessidade comprovada de operações exatas ou casos que o provider primário não resolva.

A existência do provider não autoriza dependência arquitetural direta: Fuse/Cut falam com `BooleanService/BooleanProvider`, nunca com uma biblioteca externa em toda a aplicação.

## Referências de implementação e UX

- Shapr3D — sketch/profile, adaptive UI, Extrude contextual, Add/Subtract e decals.
- Plasticity — UX de direct modeling e booleans robustos, sem copiar o kernel CAD.
- MoI — simplicidade de profile → volume.
- Blockbench — low-poly acessível, pintura/UV integrada e ecossistema de plugins.
- picoCAD — disciplina de escopo e limitações intencionais.
- Dust3D — sketch-driven low-poly e lições sobre união de partes.
- Manifold — referência para backend boolean robusto de triangle meshes.

## Diretriz de implementação

Preferir operações determinísticas, meshes poligonais e feedback visual imediato. IA pode futuramente oferecer sugestões, nunca substituir a representação editável nem esconder como a geometria foi criada. Problemas geometricamente difíceis podem usar providers externos, desde que contratos internos preservem simplicidade, testabilidade e possibilidade de substituição.