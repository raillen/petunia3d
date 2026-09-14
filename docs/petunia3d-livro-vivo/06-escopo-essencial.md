# 06 — Escopo Essencial: o que entra e o que fica fora

## Regra de produto

Uma funcionalidade entra no núcleo somente se reduzir significativamente uma destas fricções:

1. criar a silhueta;
2. transformar silhueta em volume;
3. ajustar forma low-poly;
4. unir partes;
5. texturizar/UV;
6. validar para game engine;
7. recuperar/entender o que o usuário fez.

## Modelagem essencial

- Primitivas paramétricas low-poly: cube, plane, cylinder, cone, sphere/icosphere e capsule simples.
- Draw/Profile: polyline, polygon, rectangle, circle/arc segmentado e formas básicas.
- Extrude e Push/Pull com preferência por operações topológicas locais.
- Move, Rotate, Scale.
- Mirror como operação central e fácil de manter ativa.
- Bevel/Chamfer **Core V1 com 1 segmento**. Multi-segment bevel é evolução posterior/Advanced e não faz parte da baseline inicial.
- Cut/Slice simples; `Cut` pode recorrer a Difference quando uma operação local não resolver o caso.
- Inset de face.
- Duplicate.
- Combine com **Keep Parts, Join, Fuse e Connect**. `Group/Assembly` não é uma ação pública do Combine V1.
- `Fuse` representa Union quando volumes precisam ser realmente fundidos.
- Weld/Merge, Bridge e Dissolve básicos.
- Edição de **Point/Edge/Face faz parte obrigatória do Core V1** como escape hatch avançado; o fluxo inicial não exige que o iniciante a use.
- Revolve de profile com segmentos radiais explícitos.

**Loft não entra no produto-base atual.** Simple Sweep está **fechado como Official Extension/V1.x** conforme capítulo 14, limitado a `closed polygon Profile + open polyline Path`, framing simples por parallel transport e baixa densidade geométrica explícita.

## Referências e câmera

- Front, Side, Top e Perspective, Ortho, Isometric Game.
- Ortográfica automática durante tracing.
- Perspective automática ao orbitar.
- Reference Sets com Front/Side/Back/Top.
- Opacity, lock, x-ray e alinhamento de imagens.
- V1 usa alinhamento manual previsível das referências; **Landmark Alignment fica para V1.x**, junto da evolução de calibração/multi-view.
- Smart Snap contextual.
- Silhouette comparison.

## UV e textura essenciais

- Auto UV orientado a low-poly.
- **Project From Reference/View**, especialmente útil para assets desenhados sobre referência. `Reference` é a origem preferida quando existir uma ReferenceView conhecida; `View` funciona como projeção livre/custom.
- UV packing simples e previsível.
- Pixel-density / texel-density coerente.
- UV checker e alerta de stretching.
- Texture painting integrado, focado em pencil, fill, eraser, color picker e formas; brushes complexos ficam fora do core.
- **Palette simples + recent colors entram no Paint V1**, com import/export de paleta.
- **Pixel Grid entra no Paint V1** como overlay contextual, aparecendo quando o zoom justificar e podendo ser desligado.
- Textura atualizada ao vivo na viewport.

## Game-readiness essencial

- Authoring com triangles/quads/n-gons e triangulação determinística no background.
- Show Triangulation + Flip Diagonal.
- Face orientation.
- Detecção de faces duplicadas, degenerate faces, non-manifold edges e vertices soltos.
- Contagem de vertices/faces/triangles sempre disponível de forma discreta.
- Export inicial prioritário: glTF/GLB e OBJ; FBX pode ficar posterior por complexidade/licenciamento/ecossistema.

## Explicitamente fora do core inicial

- Sculpting completo.
- NURBS/B-Rep/CAD kernel.
- Remesh avançado.
- Geometry Nodes (provavelmente na v2).
- Simulation, particles, fluids, cloth (provavelmente na v2).
- Compositor.
- Render engine completo (provavelmente na v3).
- Procedural materials avançados.
- Retopology suite profissional.
- Modifiers extensos no estilo Blender.
- Rigging/animation completos na primeira fase.
- Loft como operação oficial obrigatória.
- Boolean arbitrário como paradigma central de modelagem.

Capacidades avançadas que não justificam presença permanente no core podem ser oferecidas por **Official Extensions** ou **Community Plugins** através da Extension API.

## Filosofia

O Petunia3D deve ser poderoso por **composição de poucas operações previsíveis**, não por quantidade de comandos.

A extensibilidade faz parte da arquitetura V1 para impedir que crescimento funcional obrigue o core e a interface principal a crescerem indefinidamente. Uma feature só entra no produto-base quando resolve uma necessidade frequente, não é substituível por composição simples das ferramentas existentes e pode ser apresentada sem aumentar excessivamente a carga cognitiva.
