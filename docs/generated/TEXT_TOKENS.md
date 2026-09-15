---
title: Tokens de Tradução e Localização
description: Catálogo canônico de chaves de internacionalização TextId (P3D-119)
---

<!--
  ARQUIVO GERADO AUTOMATICAMENTE — NÃO EDITE MANUALMENTE!
  Gerado deterministicamente por `cargo xtask docs` (P3D-119).
  Para atualizar execute: cargo run -p xtask -- docs
-->

# Catálogo Canônico de Chaves de Localização (`TextId`)

> **Single Source of Truth (P3D-088, P3D-119)**
> A UI do Petunia3D é 100% internacionalizada. Nenhuma string do usuário é hardcoded; todas as mensagens passam pelo motor `I18n` com fallback seguro em inglês.

Total de chaves de localização cadastradas: **280**.

| Chave (`TextId`) | Inglês (`en.toml`) | Português (`pt-BR.toml`) |
| :--- | :--- | :--- |
| `actions.amount` | Amount | Qtd |
| `actions.apply_scale` | Apply scale | Aplicar escala |
| `actions.bevel` | Bevel | Bevel |
| `actions.connect` | Bridge Faces | Conectar Faces |
| `actions.cursor_to_origin` | Cursor to World Origin | Cursor para origem global |
| `actions.delete` | Delete | Apagar |
| `actions.deselect` | None | Nada |
| `actions.dissolve` | Dissolve Selected | Dissolver Seleção |
| `actions.distance` | Distance | Distância |
| `actions.duplicate` | Duplicate | Duplicar |
| `actions.extrude` | Extrude | Extrudar |
| `actions.factor` | Factor | Fator |
| `actions.flip_normals` | Flip Normals | Inverter Normais |
| `actions.inset` | Inset | Inset |
| `actions.invert` | Invert (Ctrl+I) | Inverter (Ctrl+I) |
| `actions.merge_center` | Merge center | Fundir no centro |
| `actions.mirror` | Mirror | Espelhar |
| `actions.move` | Move | Mover |
| `actions.pushpull` | Push/Pull | Push/Pull |
| `actions.recalculate_normals` | Recalc Normals | Recalcular Normais |
| `actions.scale` | Scale | Escala |
| `actions.select_all` | All | Tudo |
| `actions.select_linked` | Linked (L) | Conectados (L) |
| `actions.slice` | Slice Plane | Fatiar Plano |
| `actions.subdivide` | Subdivide | Subdividir |
| `actions.triangulate` | Triangulate | Triangular |
| `actions.weld_eps` | Weld | Solda |
| `animate.auto_rig` | Auto-Rig | Auto-Rig |
| `animate.first_frame` | First frame | Primeiro frame |
| `animate.frame` | Frame | Frame |
| `animate.humanoid` | Humanoid | Humanoide |
| `animate.last_frame` | Last frame | Último frame |
| `animate.pause` | Pause | Pausar |
| `animate.play` | Play | Reproduzir |
| `animate.tip_first` | Jump to First Frame · Shift+Left | Ir ao Primeiro Frame · Shift+Left |
| `animate.tip_last` | Jump to Last Frame · Shift+Right | Ir ao Último Frame · Shift+Right |
| `animate.tip_next` | Step 1 Frame Forward · Right | Avançar 1 Frame · Right |
| `animate.tip_play` | Play / Pause Animation · Space | Reproduzir / Pausar Animação · Space |
| `animate.tip_prev` | Step 1 Frame Backward · Left | Voltar 1 Frame · Left |
| `app.title` | Petunia3D | Petunia3D |
| `camera.back` | Back | Traseira |
| `camera.bottom` | Bottom | Inferior |
| `camera.frame_hint` | Frame the selection, or the active object when nothing is selected (F / Numpad .). | Enquadrar a seleção ou o objeto ativo quando nada estiver selecionado (F / Numpad .). |
| `camera.free` | Free | Livre |
| `camera.front` | Front | Frente |
| `camera.height_hint` | World height at the view center. Drag the value or double-click to type an exact size. | Altura do enquadramento no centro da vista, em metros. Arraste o valor ou clique duas vezes para digitar uma medida exata. |
| `camera.iso_ne` | Isometric NE | Isométrico NE |
| `camera.iso_nw` | Isometric NW | Isométrico NW |
| `camera.iso_se` | Isometric SE | Isométrico SE |
| `camera.iso_sw` | Isometric SW | Isométrico SW |
| `camera.isometric` | Isometric | Isométrico |
| `camera.left` | Left | Esquerda |
| `camera.orthographic` | Orthographic | Ortográfica |
| `camera.perspective` | Perspective | Perspectiva |
| `camera.projection` | Projection | Projeção |
| `camera.projection_hint` | Change projection while keeping the same framing at the view center. Orbit works in both modes. | Alterar a projeção mantendo o enquadramento no centro da vista. É possível orbitar nos dois modos. |
| `camera.reset` | Reset view | Redefinir vista |
| `camera.reset_hint` | Restore the origin and default zoom while keeping this view and projection (Home). | Voltar à origem e ao zoom inicial mantendo esta vista e projeção (Home). |
| `camera.right` | Right | Direita |
| `camera.top` | Top | Superior |
| `camera.view` | View | Vista |
| `camera.views_hint` | Axis-aligned views use orthographic projection. Ctrl selects the opposite side. | Vistas alinhadas aos eixos usam projeção ortográfica. Ctrl seleciona o lado oposto. |
| `camera.visible_height` | Visible height | Altura visível |
| `command_palette.no_results` | No commands found | Nenhum comando encontrado |
| `command_palette.placeholder` | Type a command or search… | Digite um comando ou busque… |
| `edit.redo` | Redo | Refazer |
| `edit.undo` | Undo | Desfazer |
| `export.empty` | nothing selected | nada selecionado |
| `export.format` | Format | Formato |
| `export.go` | Export… | Exportar… |
| `export.report` | Report | Relatório |
| `export.title` | Export | Exportar |
| `file.import_obj` | Import OBJ… | Importar OBJ… |
| `file.new` | New project | Novo projeto |
| `file.open_project` | Open project… | Abrir projeto… |
| `file.quit` | Quit | Sair |
| `file.save` | Save | Salvar |
| `file.save_as` | Save as… | Salvar como… |
| `help.body` | MMB orbit • Shift+MMB pan • wheel zoom • Tab mode • Del delete • Home reset • H help • Ctrl+Z/Y undo | MMB orbita • Shift+MMB pan • scroll zoom • Tab modo • Del apaga • Home reseta • H ajuda • Ctrl+Z/Y desfaz |
| `hints.bevel` | Ctrl+B: interactive bevel of one supported edge. | Ctrl+B: bevel interativo de uma aresta suportada. |
| `hints.connect` | B: bridge two loops or faces. | B: conecta (bridge) dois loops ou faces. |
| `hints.dissolve` | X: dissolve selected edges/vertices cleanly. | X: dissolve arestas/vértices sem deixar buracos. |
| `hints.draw_profile` | Shift+P: click in ortho view to add points. Click near 1st to close. | Shift+P: clique na vista ortográfica p/ pontos. Perto do 1º fecha. |
| `hints.extrude` | E: extrude selected faces. | E: extruda as faces selecionadas. |
| `hints.inset` | I: inset selected faces. | I: inset nas faces selecionadas. |
| `hints.merge` | M: merge selected into center. | M: funde a seleção no centro. |
| `hints.mirror` | Ctrl+M: mirror + weld. | Ctrl+M: espelha + solda. |
| `hints.paint` | B: click the mesh to paint vertex colors. Alt+click: pick. | B: clique na malha p/ pintar. Alt+clique: conta-gotas. |
| `hints.primitives` | A: add low-poly primitive. | A: adiciona primitiva low-poly. |
| `hints.pushpull` | P: push/pull along normals. | P: empurra/puxa ao longo das normais. |
| `hints.select` | Click to select. 1/2/3: vertex/edge/face. | Clique p/ selecionar. 1/2/3: vértice/aresta/face. |
| `hints.slice` | Shift+K: drag a cutting plane; Enter applies, Esc cancels. | Shift+K: arraste o plano de corte; Enter aplica, Esc cancela. |
| `hints.subdivide` | W: subdivide (loop cut). Triangulate below. | W: subdivide (loop cut). Triangular abaixo. |
| `hints.transform` | G/R/S: move, rotate, scale with mouse. Enter applies; Esc cancels. | G/R/S: mover, rotacionar, escalar com mouse. Enter aplica; Esc cancela. |
| `keymap.capture` | capture… | capturar… |
| `keymap.unsupported_key` | Key not supported by the keymap | Tecla não suportada pelo keymap |
| `menu.command_palette` | Command Palette | Paleta de Comandos |
| `menu.edit` | Edit | Editar |
| `menu.file` | File | Arquivo |
| `menu.help` | Help | Ajuda |
| `menu.preferences` | Preferences | Preferências |
| `menu.recent_projects` | Recent Projects | Projetos Recentes |
| `menu.view` | View | Exibir |
| `menu.window` | Window | Janela |
| `modes.edge` | Edge | Aresta |
| `modes.edit` | Edit | Edição |
| `modes.face` | Face | Face |
| `modes.object` | Object | Objeto |
| `modes.paint` | Texture Paint | Pintura |
| `modes.vertex` | Vertex | Vértice |
| `paint.brush` | Brush | Pincel |
| `paint.canvas` | Albedo canvas | Canvas albedo |
| `paint.canvas_hint` | Drag to paint. Ctrl+drag erases. Wheel over UV scales it. | Arraste p/ pintar. Ctrl+arraste apaga. Scroll no UV escala. |
| `paint.clear` | Clear | Limpar |
| `paint.color` | Color | Cor |
| `paint.eraser` | Eraser (hold Ctrl) | Borracha (segure Ctrl) |
| `paint.eraser_hint` | Hold Ctrl while painting to erase | Segure Ctrl pintando p/ apagar |
| `paint.fill` | Fill | Preencher |
| `paint.fill_sel` | Fill sel | Preencher sel |
| `paint.new_canvas` | New 256² | Novo 256² |
| `paint.pick` | Pick? | Pegar? |
| `paint.pick_hint` | Alt+click the mesh to pick a color | Alt+clique na malha p/ pegar cor |
| `paint.picked` | color picked | cor capturada |
| `paint.radius` | Radius | Raio |
| `paint.size` | Size px | Tam px |
| `paint.strength` | Strength | Força |
| `paint.vertex` | Vertex paint | Pintura vértice |
| `prims.cancel` | Cancel | Cancelar |
| `prims.capsule` | Capsule | Cápsula |
| `prims.cone` | Cone (8) | Cone (8) |
| `prims.confirm` | Confirm | Confirmar |
| `prims.confirm_hint` | Enter confirms · Esc cancels | Enter confirma · Esc cancela |
| `prims.cube` | Cube | Cubo |
| `prims.cylinder` | Cylinder (8) | Cilindro (8) |
| `prims.height` | Height | Altura |
| `prims.plane` | Plane | Plano |
| `prims.radius` | Radius | Raio |
| `prims.reopen` | Last operation… | Última operação… |
| `prims.rings` | Rings | Anéis |
| `prims.segments` | Segments | Segmentos |
| `prims.sides` | Sides | Lados |
| `prims.size` | Size | Tamanho |
| `prims.sphere` | Sphere (low) | Esfera (low) |
| `prims.width` | Width | Largura |
| `profile.clear` | Clear | Limpar |
| `profile.close` | Close | Fechar |
| `profile.closed` | profile closed | perfil fechado |
| `profile.depth` | Depth | Profundidade |
| `profile.gen_extrude` | Gen Extrude | Gerar Extrude |
| `profile.gen_revolve` | Gen Revolve | Gerar Revolve |
| `profile.generated` | profile mesh generated | malha do perfil gerada |
| `profile.need_closed` | close the profile first (click near 1st point) | feche o perfil antes (clique perto do 1º ponto) |
| `profile.need_points` | draw at least 2 points first | desenhe ao menos 2 pontos |
| `profile.points` | points | pontos |
| `profile.segments` | Revolve segs | Segs revolve |
| `profile.snap` | Snap 0.25 | Snap 0.25 |
| `profile.tris` | tris | tris |
| `profile.undo_pt` | Undo pt | Desfaz pt |
| `props.faces` | tris | tris |
| `props.name` | Name | Nome |
| `props.verts` | verts | verts |
| `refs.add_custom` | Add Unassigned… | Adicionar Avulsa… |
| `refs.add_custom_tooltip` | Load a reference image without pinning to a canonical slot | Carregar uma imagem de referência sem fixá-la a um slot canônico |
| `refs.align_view` | Align 3D camera to this reference angle | Alinhar câmera 3D com este ângulo de referência |
| `refs.back` | Back | Trás |
| `refs.bottom` | Bottom | Fundo |
| `refs.clear_all` | Clear All | Limpar Todas |
| `refs.clear_all_tooltip` | Remove all reference images from scene | Remover todas as imagens de referência da cena |
| `refs.click_to_load` | Click to load | Clique para carregar |
| `refs.fine_tune` | Fine tuning | Ajuste fino |
| `refs.front` | Front | Frente |
| `refs.left` | Left | Esquerda |
| `refs.load` | Load image… | Carregar imagem… |
| `refs.loaded` | reference(s) loaded | referência(s) carregada(s) |
| `refs.lock` | Lock | Bloquear |
| `refs.manage` | Reference Manager… | Gerenciador de Referências… |
| `refs.manager_desc` | Configure independent reference images for the 6 canonical orthographic slots. | Configure imagens de referência independentes para os 6 slots ortográficos canônicos. |
| `refs.manager_title` | Reference Set Manager | Gerenciador de Conjunto de Referências |
| `refs.no_image` | No image bound to this view | Nenhuma imagem vinculada a esta vista |
| `refs.offset` | Offset | Offset |
| `refs.opacity` | Opacity | Opacidade |
| `refs.remove` | Remove reference image | Remover imagem de referência |
| `refs.replace` | Click to replace | Clique para substituir |
| `refs.reset_default` | Reset to default | Redefinir padrão |
| `refs.right` | Right | Direita |
| `refs.rotation` | Rotation | Rotação |
| `refs.side` | Side | Lado |
| `refs.size` | Size | Tamanho |
| `refs.top` | Top | Topo |
| `refs.visible` | Visible | Visível |
| `refs.xray` | X-Ray / Overlay | Raio-X / Sobrepor |
| `settings.appearance` | Appearance | Aparência |
| `settings.export_glb` | GLB export format (off exports OBJ) | Exportar em GLB (desligado exporta OBJ) |
| `settings.export_glb_hint` | Default format for the export dialog | Formato padrão do diálogo de exportação |
| `settings.icons` | Icons | Ícones |
| `settings.import_export` | Import / Export | Importar / Exportar |
| `settings.interface` | Interface | Interface |
| `settings.keymap` | Keymap | Atalhos |
| `settings.language` | Language | Idioma |
| `settings.reset_all_layouts` | Reset All UI Layouts | Redefinir Todos os Layouts |
| `settings.reset_workspace` | Reset Current Workspace Layout | Redefinir Layout do Workspace Atual |
| `settings.show_shelf` | Contextual shelf over the viewport | Barra contextual sobre a viewport |
| `settings.title` | Settings | Configurações |
| `shading.smooth` | Smooth | Suave |
| `shading.solid` | Flat | Plano |
| `shading.textured` | Textured | Textura |
| `shading.unlit` | Unlit | Sem Luz |
| `shading.wire` | Wireframe | Arame |
| `tools.add_primitive` | Add Primitive | Adicionar primitiva |
| `tools.annotate` | Annotate | Anotar |
| `tools.bevel` | Bevel | Bevel |
| `tools.connect` | Connect | Conectar |
| `tools.cursor_3d` | 3D Cursor | Cursor 3D |
| `tools.dissolve` | Dissolve | Dissolver |
| `tools.draw_profile` | Profile | Perfil |
| `tools.eraser` | Eraser | Borracha |
| `tools.extrude` | Extrude | Extrudar |
| `tools.extrude_individual` | Extrude Individual | Extrusão individual |
| `tools.flip_diagonal` | Flip Diagonal | Inverter diagonal |
| `tools.inset` | Inset | Inset |
| `tools.knife` | Knife | Faca |
| `tools.loop_cut` | Loop Cut | Corte em loop |
| `tools.measure` | Measure | Medir |
| `tools.merge` | Merge | Fundir |
| `tools.mirror` | Mirror | Espelho |
| `tools.move` | Move | Mover |
| `tools.paint` | Paint | Pintar |
| `tools.picker` | Picker | Conta-gotas |
| `tools.primitives` | Add | Adicionar |
| `tools.pushpull` | Push/Pull | Push/Pull |
| `tools.revolve` | Revolve 360° | Revolução 360° |
| `tools.rotate` | Rotate | Rotacionar |
| `tools.scale` | Scale | Escalar |
| `tools.select` | Select | Seleção |
| `tools.select_box` | Box Select | Seleção em caixa |
| `tools.slice` | Slice | Fatiar |
| `tools.subdivide` | Cut | Cortar |
| `tools.transform` | Transform | Transformar |
| `tools.uv_project` | Project | Projetar |
| `tools.uv_seam` | Seam | Costura |
| `tools.uv_select` | UV Select | Seleção UV |
| `tools.uv_unwrap` | Unwrap | Desdobrar |
| `ui.active_tool` | Active tool | Ferramenta ativa |
| `ui.assets` | Asset Library | Assets |
| `ui.at_3d_cursor` | at 3D Cursor | no Cursor 3D |
| `ui.close` | Close | Fechar |
| `ui.collapse` | Collapse section | Recolher painel |
| `ui.dock_split_hint` | Drag to resize panels | Arraste para redimensionar os painéis |
| `ui.duplicate` | Duplicate | Duplicar |
| `ui.expand` | Expand section | Expandir painel |
| `ui.floating_inspector` | Floating Inspector | Inspector Flutuante |
| `ui.help` | Help | Ajuda |
| `ui.language` | Language | Idioma |
| `ui.lock` | Lock | Bloquear |
| `ui.mesh` | Mesh | Malha |
| `ui.more` | More… | Mais… |
| `ui.no_tool` | Tool disabled in tools.toml | Ferramenta desligada no tools.toml |
| `ui.outliner` | Outliner | Outliner |
| `ui.properties` | Properties | Propriedades |
| `ui.redock` | Re-dock the Properties panel into the sidebar | Reancorar o Painel de Propriedades na barra lateral |
| `ui.refs` | Reference images | Imagens de referência |
| `ui.rename` | Rename | Renomear |
| `ui.search` | Search | Buscar |
| `ui.tools` | Tools | Ferramentas |
| `ui.tools_menu` | Tools… | Ferramentas… |
| `ui.visible` | Visible | Visível |
| `uv.faces` | Faces | Faces |
| `uv.hint` | Click: select face. Drag: move UVs. Wheel: scale. | Clique: seleciona face. Arraste: move UVs. Scroll: escala. |
| `uv.preview_3d` | 3D preview | Prévia 3D |
| `uv.reproject` | Planar | Planar |
| `uv.scale` | Scale | Escala |
| `uv.selected` | sel faces | faces sel |
| `uv.title` | UV editor | Editor UV |
| `view.frame` | Frame | Enquadrar |
| `view.frame_all` | Frame All | Enquadrar tudo |
| `ws.animate` | ANIMATE | ANIMATE |
| `ws.export` | EXPORT | EXPORT |
| `ws.model` | MODEL | MODEL |
| `ws.paint` | PAINT | PINTURA |
| `ws.uv` | UV | UV |

