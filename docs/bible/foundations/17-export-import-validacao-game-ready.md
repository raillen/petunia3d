# 17 — Export, Import, Validação e Pipeline Game-Ready

> Exportação é uma compilação do documento de authoring para um formato de game. O processo nunca precisa destruir ou simplificar silenciosamente o projeto original.

# Formatos V1

- **GLB/glTF 2.0**: formato principal.
- **OBJ/MTL**: compatibilidade simples/secundária.
- FBX fica fora da V1; só entra se uma necessidade real justificar dependência adicional.

# Bibliotecas

Na baseline Rust, **GLB/glTF 2.0 usa `gltf` + `gltf-json`** atrás dos contratos `Importer`/`Exporter`. O exporter recebe um `ExportModel` próprio do Petunia, já triangulado e validado, para que tipos glTF nunca contaminem o authoring model.

OBJ permanece secundário. A V1 usa um **writer próprio pequeno** para export e **`tobj` como parser baseline de import**, atrás do `Importer` trait e das mesmas fixtures de conformance. Essa dependência é substituível: seus tipos não vazam para `Document`, `PetuniaMesh` ou `ExportModel`. `cgltf` e `tiny_obj_c` deixam de ser dependências normativas da baseline atual e permanecem apenas como referências históricas/alternativas técnicas.

Todas as bibliotecas de interchange continuam atrás de contratos próprios; trocar parser/serializer não pode alterar semântica de `Document`, `PetuniaMesh` ou `ExportModel`.

# Export pipeline

```plain text
Document snapshot
→ resolve procedural generators em cópia temporária
→ validate
→ resolve materials/textures
→ bake projections/decals necessários
→ compute normals
→ triangulate deterministically
→ compute tangents only if needed
→ coordinate/unit conversion
→ serialize exporter
→ post-export validation
```

O documento aberto não é convertido destrutivamente para triangles apenas para exportar.

# Triangulação

- triangles/quads/n-gons são válidos no authoring;
- quad com `locked_triangulation` respeita diagonal escolhida;
- quads sem lock usam regra determinística;
- n-gons usam triangulação determinística após projeção local adequada;
- resultado de triangulação é cache/artefato derivado;
- export e viewport devem compartilhar a mesma política para impedir diferenças visuais inesperadas.

# Normals / Sharp

- winding define face normal;
- `Flat` trata boundaries como sharp para cálculo de vertex normals;
- `Smooth` compartilha normals entre faces conectadas salvo Edge marcada Sharp;
- `Smooth by Angle` é command de conveniência que escreve flags Sharp segundo dihedral threshold; não é modifier mágico permanente;
- custom split normals editáveis não entram na V1.

# Game Ready Validator

Validator produz **Errors, Warnings e Info**, nunca uma caixa binária misteriosa.

## Erros estruturais

- índices/IDs inválidos;
- face com menos de 3 points;
- zero-area/degenerate faces relevantes;
- loops corrompidos;
- topology impossível de serializar;
- textura referenciada ausente.

## Warnings

- non-manifold boundary quando o formato/workflow espera solid;
- duplicate/coplanar faces suspeitas;
- loose vertices/edges;
- normals/winding inconsistentes;
- UV ausente onde existe texture;
- UV stretch alto;
- overlap UV não explicitamente marcado como intencional;
- material/texture settings que não mapeiam para o exporter escolhido;
- triangle budget acima de um target definido pelo projeto.

## Info

- vertex/edge/face/triangle counts;
- material count;
- texture sizes;
- texel density range;
- number of mesh islands.

# Auto-fix seguro

Pode corrigir automaticamente apenas operações com semântica clara:

- remover unused vertices;
- remover faces zero-area comprovadamente degeneradas;
- recalcular derived normals;
- orientar winding quando uma componente fechada e orientável permitir decisão inequívoca;
- regenerar caches;
- repack UV somente após ação explícita.

Nunca auto-remesh, fechar holes arbitrariamente, fundir objetos, reduzir polígonos ou alterar silhouette sem ordem explícita.

# Boolean validation

Fuse/Cut recebem validation própria antes/depois do provider. Na baseline Rust, o provider oficial inicial é **`manifold-rust`**, mantido atrás de `BooleanProvider`. Não existe requisito de C FFI para Boolean na arquitetura vigente. O fluxo é `triangulated snapshot → manifold-rust → triangle result → reconstruct PetuniaMesh → safe cleanup → validate`; nenhuma etapa executa remesh automático ou expõe tipos do provider para o restante da aplicação.

# Import

Import sempre cria representação authoring válida ou falha com diagnóstico.

- glTF/GLB: importar meshes, materials e textures relevantes ao escopo; animation/skin podem ser ignorados/avisados enquanto não forem suportados;
- OBJ: positions, faces, UV, normals/material groups quando disponíveis;
- n-gons importados são validados e mantidos quando seguros ou triangulados deterministically se necessário.

# Export batch

A arquitetura suporta export de um ou vários assets porque o exporter recebe uma lista explícita de Object/Asset IDs. Batch é composição do mesmo pipeline, não implementação paralela.

# Reprodutibilidade

Com o mesmo document snapshot + mesmas opções + mesma versão de providers, o output geométrico deve ser determinístico sempre que as bibliotecas permitirem. Registrar versões de providers no diagnóstico/export metadata quando útil.

# Regra final

O validator protege o usuário contra problemas de pipeline, mas não decide estética. Warnings devem ser acionáveis e nenhuma correção destrutiva acontece silenciosamente.
