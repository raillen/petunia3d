---
title: Tokens de Ícones Semânticos
description: Catálogo canônico de identificadores de ícones IconId (P3D-119)
---

<!--
  ARQUIVO GERADO AUTOMATICAMENTE — NÃO EDITE MANUALMENTE!
  Gerado deterministicamente por `cargo xtask docs` (P3D-119).
  Para atualizar execute: cargo run -p xtask -- docs
-->

# Catálogo Canônico de Tokens de Ícones (`IconId`)

> **Single Source of Truth (P3D-086, P3D-119)**
> Ícones no Petunia3D são estritamente endereçados por tokens semânticos (`IconId`), permitindo substituição de pacotes gráficos em tempo de execução sem afetar a lógica.

Total de tokens declarados em `PetuniaIcon`: **87**.

| Token Enum | Identificador Textual (`IconId`) | Grupo Semântico |
| :--- | :--- | :--- |
| `PetuniaIcon::PropCollection` | `prop_collection` | Abas de Propriedades (Assets Figma 1..15) |
| `PetuniaIcon::PropData` | `prop_data` | Abas de Propriedades (Assets Figma 1..15) |
| `PetuniaIcon::PropMaterial` | `prop_material` | Abas de Propriedades (Assets Figma 1..15) |
| `PetuniaIcon::PropModifiers` | `prop_modifiers` | Abas de Propriedades (Assets Figma 1..15) |
| `PetuniaIcon::PropObject` | `prop_object` | Abas de Propriedades (Assets Figma 1..15) |
| `PetuniaIcon::PropOutput` | `prop_output` | Abas de Propriedades (Assets Figma 1..15) |
| `PetuniaIcon::PropRender` | `prop_render` | Abas de Propriedades (Assets Figma 1..15) |
| `PetuniaIcon::PropScene` | `prop_scene` | Abas de Propriedades (Assets Figma 1..15) |
| `PetuniaIcon::PropTool` | `prop_tool` | Abas de Propriedades (Assets Figma 1..15) |
| `PetuniaIcon::PropViewLayer` | `prop_view_layer` | Abas de Propriedades (Assets Figma 1..15) |
| `PetuniaIcon::PropWorld` | `prop_world` | Abas de Propriedades (Assets Figma 1..15) |
| `PetuniaIcon::PropertyTab` | `property_tab` | Abas de Propriedades (Assets Figma 1..15) |
| `PetuniaIcon::Collection` | `collection` | Auxiliares de Viewport e Cena |
| `PetuniaIcon::Delete` | `delete` | Auxiliares de Viewport e Cena |
| `PetuniaIcon::Duplicate` | `duplicate` | Auxiliares de Viewport e Cena |
| `PetuniaIcon::ObjectMesh` | `object_mesh` | Auxiliares de Viewport e Cena |
| `PetuniaIcon::OrientationGlobal` | `orientation_global` | Auxiliares de Viewport e Cena |
| `PetuniaIcon::Overlays` | `overlays` | Auxiliares de Viewport e Cena |
| `PetuniaIcon::PivotMedian` | `pivot_median` | Auxiliares de Viewport e Cena |
| `PetuniaIcon::ProportionalEditing` | `proportional_editing` | Auxiliares de Viewport e Cena |
| `PetuniaIcon::ReferenceImage` | `reference_image` | Auxiliares de Viewport e Cena |
| `PetuniaIcon::SnapMagnet` | `snap_magnet` | Auxiliares de Viewport e Cena |
| `PetuniaIcon::ViewOrthographic` | `view_orthographic` | Auxiliares de Viewport e Cena |
| `PetuniaIcon::ViewPerspective` | `view_perspective` | Auxiliares de Viewport e Cena |
| `PetuniaIcon::XRay` | `x_ray` | Auxiliares de Viewport e Cena |
| `PetuniaIcon::Custom` | `custom` | Customizado por ID |
| `PetuniaIcon::Bevel` | `bevel` | Ferramentas de Modelagem Poligonal |
| `PetuniaIcon::DrawProfile` | `draw_profile` | Ferramentas de Modelagem Poligonal |
| `PetuniaIcon::Extrude` | `extrude` | Ferramentas de Modelagem Poligonal |
| `PetuniaIcon::Inset` | `inset` | Ferramentas de Modelagem Poligonal |
| `PetuniaIcon::Knife` | `knife` | Ferramentas de Modelagem Poligonal |
| `PetuniaIcon::LoopCut` | `loop_cut` | Ferramentas de Modelagem Poligonal |
| `PetuniaIcon::PushPull` | `push_pull` | Ferramentas de Modelagem Poligonal |
| `PetuniaIcon::Slice` | `slice` | Ferramentas de Modelagem Poligonal |
| `PetuniaIcon::Subdivide` | `subdivide` | Ferramentas de Modelagem Poligonal |
| `PetuniaIcon::PaintBrush` | `paint_brush` | Ferramentas de Pintura (Paint) |
| `PetuniaIcon::PaintEraser` | `paint_eraser` | Ferramentas de Pintura (Paint) |
| `PetuniaIcon::PaintFill` | `paint_fill` | Ferramentas de Pintura (Paint) |
| `PetuniaIcon::PaintLine` | `paint_line` | Ferramentas de Pintura (Paint) |
| `PetuniaIcon::PaintPicker` | `paint_picker` | Ferramentas de Pintura (Paint) |
| `PetuniaIcon::PaintRect` | `paint_rect` | Ferramentas de Pintura (Paint) |
| `PetuniaIcon::ShadingMaterial` | `shading_material` | Modos de Sombreamento |
| `PetuniaIcon::ShadingRendered` | `shading_rendered` | Modos de Sombreamento |
| `PetuniaIcon::ShadingSolid` | `shading_solid` | Modos de Sombreamento |
| `PetuniaIcon::ShadingWireframe` | `shading_wireframe` | Modos de Sombreamento |
| `PetuniaIcon::ModeEdit` | `mode_edit` | Modos e Alvos de Seleção |
| `PetuniaIcon::ModeObject` | `mode_object` | Modos e Alvos de Seleção |
| `PetuniaIcon::SelectEdge` | `select_edge` | Modos e Alvos de Seleção |
| `PetuniaIcon::SelectFace` | `select_face` | Modos e Alvos de Seleção |
| `PetuniaIcon::SelectVertex` | `select_vertex` | Modos e Alvos de Seleção |
| `PetuniaIcon::AddPrimitive` | `add_primitive` | Toolbar / Ferramentas |
| `PetuniaIcon::Annotate` | `annotate` | Toolbar / Ferramentas |
| `PetuniaIcon::Cursor3D` | `cursor3_d` | Toolbar / Ferramentas |
| `PetuniaIcon::Measure` | `measure` | Toolbar / Ferramentas |
| `PetuniaIcon::Move` | `move` | Toolbar / Ferramentas |
| `PetuniaIcon::Rotate` | `rotate` | Toolbar / Ferramentas |
| `PetuniaIcon::Scale` | `scale` | Toolbar / Ferramentas |
| `PetuniaIcon::SelectBox` | `select_box` | Toolbar / Ferramentas |
| `PetuniaIcon::Transform` | `transform` | Toolbar / Ferramentas |
| `PetuniaIcon::ChevronDown` | `chevron_down` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::ChevronLeft` | `chevron_left` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::ChevronRight` | `chevron_right` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::ChevronUp` | `chevron_up` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Close` | `close` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Eye` | `eye` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::EyeHidden` | `eye_hidden` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::File` | `file` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Filter` | `filter` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Folder` | `folder` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::JumpEnd` | `jump_end` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::JumpStart` | `jump_start` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Lock` | `lock` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Maximize` | `maximize` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Minimize` | `minimize` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::MoreVert` | `more_vert` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Pause` | `pause` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Pin` | `pin` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Play` | `play` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Plus` | `plus` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Redo` | `redo` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Search` | `search` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Settings` | `settings` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::StepBackward` | `step_backward` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::StepForward` | `step_forward` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Trash` | `trash` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Undo` | `undo` | Utilitários (Phosphor / Vetoriais) |
| `PetuniaIcon::Unlock` | `unlock` | Utilitários (Phosphor / Vetoriais) |

