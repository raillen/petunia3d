# Workspace: PAINT

O workspace **PAINT** pinta o modelo em 3D. A prioridade da V1 é **Paint on Model**
sobre `Base Color`, com UV gerada por automação — não um editor de imagem.

## Regiões do shell

- **Centro — Viewport 3D dominante**, em shading `Textured`, mostrando a pintura ao vivo.
- **Direita — Context**: brush, material e textura (raio, opacidade, cor, mapa ativo).
- **Esquerda — Parts**: seleção do objeto/parte a pintar.
- **Embaixo — Asset Library**: paletas e assets.
- **Painel 2D de textura**: opcional e **fechado por padrão**. Quando aberto, compartilha
  a mesma `TextureBitmap`, a mesma paleta, o mesmo Pixel Grid e o mesmo Undo/Redo do
  Paint 3D. Ele não se transforma em editor de imagem generalista.

## Fluxo recomendado

```
Auto UV → Project From Reference/View → packing → texel density → avisos de stretch → Paint on Model
```

O editor UV manual existe para **controle e correção**, não como rito obrigatório
antes de pintar. Se o asset nasceu sobre uma referência, `Project From Reference`
resolve a maior parte dos casos sem abrir o workspace UV.

## Ferramentas do Paint V1

| Recurso | Escopo |
| :--- | :--- |
| Pincel (*pencil*) | traço contínuo sobre a superfície com pressão/raio ajustáveis |
| Preenchimento (*fill*) | preenche região/face com tolerância |
| Borracha (*eraser*) | remove pintura do alvo |
| Conta-gotas | captura cor da superfície ou da paleta |
| Formas simples | primitivas de pintura além do traço livre |
| Paleta simples + recent colors | com **import/export de paleta** |
| Pixel Grid | overlay contextual, aparece quando o zoom justificar e pode ser desligado |

Brushes complexos (procedurais, com máscaras avançadas, efeitos em pilha) **não** são
core — ficam para módulos/extensões (P3D-134, P3D-165).

## Contrato de dados

- Um **stroke = uma transaction**: Undo/Redo da pintura usa *diffs* de tile.
- Material interno segue **glTF metallic-roughness** + `Unlit`; Height/Displacement
  não bloqueia a V1.
- `UV0` único na V1, com seams e chart boundaries suportados e packing sem overlaps
  automáticos.
- Trocar de tema **nunca** altera o documento; trocar de textura/mapa acontece via
  Command e entra no Undo.

## Fora do escopo do Paint V1

Pintura procedural em nós, camadas de efeito ilimitadas, decals projetados como
recurso principal, atlas automático como fluxo obrigatório e edição de imagem
genérica. Itens pós-V1 estão catalogados em
[P3D-132](../bible/specs/p3d-132-paint-masks-face-selection-isolation.md),
[P3D-133](../bible/specs/p3d-133-decal-projection-layers.md),
[P3D-134](../bible/specs/p3d-134-paint-effect-stack.md),
[P3D-149](../bible/specs/p3d-149-texture-atlas-builder.md) e
[P3D-165](../bible/specs/p3d-165-surface-paint-toolbox.md).
