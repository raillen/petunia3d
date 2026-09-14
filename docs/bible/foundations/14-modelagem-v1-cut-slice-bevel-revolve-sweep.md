# 14 — Modelagem V1: Cut, Slice, Bevel, Revolve e Simple Sweep

> Este capítulo congela as operações de modelagem que ainda estavam pendentes. A regra é manter poucas operações, comportamento determinístico e fallback seguro. Capacidades CAD/generalistas ficam fora.

# Cut e Slice são conceitos diferentes

**Cut** remove volume ou cria recessos. **Slice** apenas divide a geometria por um plano; não remove material por padrão.

## Cut superficial / recess

Quando um Profile é desenhado sobre uma face planar e empurrado para dentro sem atravessar outra parte do objeto:

```plain text
profile on face
→ split local region
→ extrude inward
→ create side walls
```

Isso é operação topológica local e não usa Boolean.

## Through Cut

Se o corte atravessa outra superfície, cruza geometria arbitrária ou usa um cutter volumétrico independente, `Cut` delega a `BooleanProvider.difference`.

Regras:

- preview antes do commit;
- cutter temporário por padrão;
- opção interna para preservar cutter existe, mas não é requisito da UX básica;
- nenhuma retopology/remesh automática;
- cleanup apenas seguro após o provider.

## Slice

Slice usa um **plano infinito** e insere a interseção na mesh.

- atua somente nos objetos/partes selecionados;
- divide edges/faces cruzados pelo plano;
- mantém ambos os lados por padrão;
- pode gerar duas regiões separáveis depois, mas `Separate` é operação distinta;
- uma linha desenhada em vista ortográfica pode definir o plano usando a direção da câmera como normal auxiliar;
- Knife 3D arbitrário, cortes livres sobre superfície e sistemas de múltiplos planos ficam fora da V1.

# Bevel / Chamfer / Round Edge

A operação de usuário é conceitualmente **Round Edge**, com `Bevel/Chamfer` como nome técnico.

## Escopo V1

O core suporta **1 segmento** de bevel/chamfer. Isso cobre o caso low-poly predominante e reduz muito a complexidade de corner handling.

Regras:

- funciona em edges manifold com duas faces adjacentes válidas;
- largura limitada para impedir auto-intersection local evidente;
- miter/corners usam uma única política determinística simples;
- preview obrigatório;
- operação falha preservando o modelo quando a região não pode ser resolvida com segurança;
- border bevel complexo, bevel em non-manifold e múltiplos perfis de miter não fazem parte do core V1.

## Evolução

Múltiplos segmentos podem entrar em V1.x como extensão oficial se houver necessidade real. O modelo de dados não deve impedir `segments > 1`, mas o algoritmo base não precisa implementá-los agora.

# Revolve

Revolve entra no **Core V1** porque tem alto valor e implementação controlável.

Contrato:

- entrada principal: Profile poligonal + eixo;
- rotação completa de 360° na V1;
- número radial de segmentos explícito;
- gera faces laterais quads quando possível e triangles nos polos/fechamentos necessários;
- caps somente quando geometricamente necessários;
- axis pode vir de eixo do Work Plane ou linha explicitamente escolhida;
- resultado pode permanecer procedural até `Make Editable`.

Parcial revolve, pitch/helical revolve e perfis variáveis ficam fora da V1.

# Simple Sweep

**Simple Sweep é Official Extension / V1.x**, não Core V1.

Contrato deliberadamente limitado:

```plain text
closed polygon Profile
+
open polyline Path
→ swept low-poly mesh
```

## Geometria

- profile inicialmente fechado e poligonal;
- path inicialmente aberto e piecewise-linear;
- usar **parallel transport frames** para minimizar twist acumulado;
- orientação inicial vem do Work Plane do Profile;
- uma seção é gerada em cada point relevante do path;
- conectar seções com quads; triangles apenas quando necessários por degeneração/polo;
- caps no início/fim habilitados pelo provider quando o profile é fechado.

## Deliberadamente fora

- múltiplos profiles;
- loft/correspondence;
- taper/scale ao longo do path;
- twist artístico por keyframes;
- path branching;
- rail duplo;
- automatic self-intersection repair;
- smoothing de curva que inventa segmentos.

Se um path muito fechado provocar auto-intersection evidente, avisar/falhar em vez de remesh.

# Inset

Inset permanece Core V1, porém limitado a faces planas simples e regiões sem ambiguidade topológica. Casos complexos podem falhar de modo seguro. Não implementar um solver geral de inset para qualquer n-gon patológico antes de necessidade comprovada.

# Fill e Dissolve

`Fill` fecha um boundary simples usando triangulação determinística; quando o loop for planar e adequado, o authoring pode manter n-gon. `Dissolve` remove edge/point apenas quando a topologia resultante continua válida. Ambos são ferramentas de reparo, não sistemas automáticos de retopology.

# Regra de implementação

Operações simples manipulam diretamente a authoring mesh. Providers externos entram apenas quando a natureza do problema realmente exige algoritmo especializado. Toda operação é transacional, produz invalidação localizada de caches e retorna IDs/remaps úteis para seleção e Undo.
