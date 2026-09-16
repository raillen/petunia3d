# P3D-157 — Modifier Stack & Visual Deformer UX

<aside>
🧱

Estado: **pós-V1 aprovado** · Prioridade: alta após Decals/Surface foundation. Implementar uma stack pequena, previsível e didática de modificadores não destrutivos. A regra central é: **o viewport deve explicar visualmente a deformação antes de o usuário precisar entender parâmetros numéricos**.

</aside>

# Objetivo

Adicionar edição geométrica não destrutiva suficiente para workflows low-poly recorrentes sem reproduzir o catálogo do Blender. A V1 pós-GA inicial deve cobrir Mirror Live, Array, Thickness e Simple Deform; Surface Conform e Path modifiers entram depois das fundações correspondentes.

# Contrato de dados

```rust
pub struct ModifierStack {
    pub modifiers: Vec<ModifierInstance>,
}

pub struct ModifierInstance {
    pub id: ModifierId,
    pub enabled: bool,
    pub kind: ModifierKind,
    pub parameters: ModifierParameters,
}
```

Requisitos: ordem determinística, enable/disable, reorder, duplicate, apply/bake, remove, serialization versionada, cache por revision, Undo/Redo e diagnostics explícitos.

# Contrato visual obrigatório

Todo modifier ativo que altera forma ou distribuição deve possuir representação visual contextual no viewport. O overlay deve usar tokens do design system, funcionar em alto contraste e nunca depender apenas de cor.

## Deformer Cage / Capture Box

Para Bend, Twist, Taper e Stretch, mostrar uma caixa/cage ajustada aos bounds da geometria afetada. A cage representa **onde** a deformação atua.

Elementos mínimos:

- caixa de influência;
- eixo/spine central;
- origem/pivot da deformação;
- planos Start/End editáveis;
- indicação de eixo X/Y/Z/local;
- preview ghost opcional da forma original;
- handles diretos;
- label contextual com nome e valor principal.

## Bend

Mostrar:

- cage original;
- spine central;
- arco-guia indicando trajetória esperada;
- handle de curvature/angle;
- Start/End bounds;
- origem do arco.

Arrastar o arco altera `angle`; mover Start/End altera limits; mover/rotacionar a cage altera origin/orientation quando permitido.

## Twist

Mostrar:

- cage;
- eixo central;
- anéis/handles de rotação nas extremidades;
- indicação helicoidal simplificada da progressão do twist;
- Start Angle e End Angle quando modo avançado estiver aberto.

O usuário deve perceber visualmente "esta ponta gira em relação àquela".

## Stretch / Squash

Mostrar:

- cage;
- handles nas duas extremidades;
- setas longitudinais;
- preview de mudança de seção transversal quando `Preserve Volume` estiver ativo;
- linha de comprimento original opcional.

## Taper

Mostrar:

- cage;
- frame/plano de escala no início e fim;
- handle de largura/altura no extremo afetado;
- midpoint apenas em Advanced se necessário.

## Array

Mostrar cópias fantasma antes do commit/aplicar; handle de spacing entre primeira e segunda cópia; contador próximo à última instância; radial array mostra arco e eixo central.

## Thickness

Mostrar shell/contorno fantasma e setas de normal indicando direção do offset. `Center`, `Inside` e `Outside` devem ser visualmente distinguíveis sem depender apenas do nome.

## Mirror

Mostrar plano de espelho, normal do plano e preview fantasma do lado gerado. Seam/merge region deve ser indicada quando `Merge Center` estiver ativo.

# UX de parâmetros

Painel compacto:

```
DEFORM
Mode      Bend
Axis      Z
Amount    35°
Limits    0.00 — 1.00
Origin    Object

[✓] Show Cage
[ ] Preserve Volume

[ Apply Now ] [ Keep Live ]
```

Campos numéricos continuam scrubbable e editáveis por teclado. Manipulação de viewport e Inspector devem escrever os mesmos parâmetros/Commands.

# Simple vs Advanced

Simple mostra mode, axis, amount, cage e handles essenciais. Advanced revela limits exatos, custom origin, locks, preserve volume, influence mask e diagnostics.

# Influence / seleção parcial

V1 pode atuar no objeto inteiro. Evolução permite Selection Set/Mask/Vertex Group equivalente, mas sem criar Weight Paint obrigatório. Quando houver influência parcial, o viewport deve mostrar a região capturada/influência de forma legível.

# Avaliação e performance

- avaliar stack sobre snapshot imutável da base mesh;
- cachear saída de cada etapa por revision + parameter hash;
- invalidar somente a partir do modifier alterado;
- viewport pode usar preview simplificado durante drag se profiling justificar;
- export usa resultado avaliado ou bake explícito conforme profile.

# Erros previsíveis

Thickness em non-manifold, Surface Conform sem target, Array com merge inválido e deformers com capture length zero devem gerar diagnóstico, não geometria silenciosamente corrompida.

# Acessibilidade

Handles precisam de target mínimo conforme design system, foco por teclado, descrição textual, valores numéricos equivalentes e modo `Show Guides Always / When Selected / Never`. Não usar somente cor para diferenciar eixos ou bounds.

# Referências técnicas

- [Blender Simple Deform](https://docs.blender.org/manual/id/5.2/modeling/modifiers/deform/simple_deform.html): Twist/Bend/Taper/Stretch, Axis, Origin e Limits.
- [Houdini Bend](https://www.sidefx.com/docs/houdini/nodes/sop/bend): capture region/spine e handles distintos para Bend, Twist, Length Scale e Taper; principal referência para nossa cage didática simplificada.
- [Maya nonlinear deformers](https://help.autodesk.com/cloudhelp/2026/ENU/Maya-CharacterAnimation/files/GUID-8DF6AA06-2848-4CA8-AB57-313057EA14B1.htm): deformers matemáticos simples.
- [Maya bend handle](https://help.autodesk.com/cloudhelp/2022/ENU/Maya-CharacterAnimation/files/GUID-557246DD-CF0F-4BDB-BDD0-47947A97BC94.htm): handles para controlar extent/curvature visualmente.
- [Maya lattice](https://help.autodesk.com/cloudhelp/2026/ENU/Maya-CharacterAnimation/files/GUID-8F27AC88-6892-4EAE-A1CB-C43FADF95A64.htm): referência para cage espacial explícita envolvendo o objeto.

# Testes / DoD

- reorder altera resultado de forma determinística;
- Undo/Redo de parâmetros e reorder;
- save/load conserva stack;
- cage corresponde matematicamente à capture region;
- viewport handle e Inspector produzem o mesmo estado;
- Apply gera geometria equivalente ao preview;
- export recebe resultado correto;
- golden tests para Bend/Twist/Taper/Stretch/Array/Mirror/Thickness;
- UI tests para focus, keyboard, drag e accessibility labels.

# Non-goals

Sem remesh, cloth/physics, volume/voxel, arbitrary Geometry Nodes modifier, modifier scripting genérico ou catálogo extenso.