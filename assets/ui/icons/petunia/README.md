# Tema padrão Petunia — v0.1

Arte curada do pack de pesquisa `petunia-icons-v0.1` (Iconoir como fonte
primária; Tabler, Heroicons e Lucide como reservas aprovadas). Este diretório é
a fonte canônica do pacote **petunia**, o padrão do produto.

## Como funciona

- `crates/ui/src/icon_registry.rs` embute cada SVG (`include_str!`) no módulo
  `petunia_theme::ICONS` e o rasteriza como máscara branca (tingível).
- O tema é consultado **depois** do glifo de pacote genérico e **antes** da
  arte vetorial/PNG antiga: pacote "petunia" → tema; outros pacotes → glifo do
  pacote (o desenho muda, o significado nunca).
- `mapping.csv` e `manifest.json` preservam a rastreabilidade (id Petunia,
  ícone upstream, licença, status). `licenses/` traz os textos integrais.
- Candidatos não embutidos (`candidate_not_bundled`) e itens marcados
  `custom_required` (Push/Pull, Weld, Dissolve, Low-Poly Hair) continuam na
  arte vetorial Petunia — ver `crates/ui/src/icons.rs`.

## Cobertura atual do tema

| PetuniaIcon | Upstream | Fonte |
| --- | --- | --- |
| `mode_object` | box-3d-center | Iconoir |
| `select_vertex` | select-point-3d | Iconoir |
| `select_edge` | select-edge-3d | Iconoir |
| `select_face` | select-face-3d | Iconoir |
| `extrude` | extrude | Iconoir |
| `bevel` | fillet-3d | Iconoir |
| `knife` | cube-cut-with-curve | Iconoir |
| `rotate` | rotate-3d | Tabler |
| `orientation_global` | gizmo | Tabler |
| `xray` | xray-view | Iconoir |
| `view_perspective` | perspective-view | Iconoir |
| `view_orthographic` | orthogonal-view | Iconoir |
| `paint_brush` | paint-brush | Heroicons |

## Próximo passo (pós-v0.1)

Normalizar peso óptico/alinhamento dos fallbacks para stroke 1.5 e revisar os
candidatos não embutidos antes do atlas final.
