# V1 Primitives — Wave 0 Audit (2026-09-15, rev `c5e1cd2`)

Verificado no código, não em claims. Legenda: OK / PARCIAL / AUSENTE / QUEBRADO.

| Primitive | Geometry | Params | Preview | Undo | Save/Load | Export | UI | i18n | Tests | Action |
|---|---|---|---|---|---|---|---|---|---|---|
| Cube | OK (`cube(size)`) | PARCIAL (só `size`; falta W/H/D) | OK (sessão Wave 8) | OK (1 txn) | OK | OK | OK (shelf/outliner/Add+) | OK nomes | PARCIAL | P2: builder `box()` + sessão W/H/D |
| Plane | OK (`plane(size)`) | PARCIAL (quadrado fixo) | OK | OK | OK | OK | OK | OK nomes | PARCIAL | P2: W×D (escala, feito na sessão); default 1×1 |
| Wedge | AUSENTE | AUSENTE | — | — | — | — | AUSENTE | AUSENTE | AUSENTE | P3: builder + sessão + menu |
| Cylinder | OK (`cylinder`, sempre tampado) | PARCIAL (sem caps, sides fixo 16!) | OK | OK | OK | OK | OK | OK nomes | PARCIAL | P3: gerador radial + caps + default 8 |
| Cone | OK (`cone`, sempre tampado) | PARCIAL (sem top_radius/caps, 16!) | OK via sessão | OK | OK | OK | OK (outliner/Add+, sem shelf) | OK nomes | PARCIAL | P3: frustum + default 8 |
| Circle/Disc | AUSENTE | AUSENTE | — | — | — | — | AUSENTE | AUSENTE | AUSENTE | P3: builder + fill None/Disc |
| Torus | AUSENTE | AUSENTE | — | — | — | — | AUSENTE | AUSENTE | AUSENTE | P4: builder 12/6 |
| UV Sphere | OK (`sphere_low`) | PARCIAL (default 16/12, denso!) | OK | OK | OK | OK | OK | OK nomes | PARCIAL | P4: default 12/6 + soldar polos |
| Icosphere | AUSENTE | AUSENTE | — | — | — | — | AUSENTE | AUSENTE | AUSENTE | P4: builder subdiv 0–3 |
| Capsule | OK (`capsule`, perfil fixo 6 anéis) | PARCIAL (sem radial/caps params, 16!) | OK via sessão | OK | OK | OK | OK (outliner, sem shelf) | OK nomes | PARCIAL | P4: `capsule_profile` + default radial 8 |

## Defeitos estruturais encontrados

1. **Defaults densos** (`generate_mesh`): Sphere 16/12, Cylinder/Cone/Capsule 16 sides — contra §5. `Mesh::sphere_low` duplica vértices dos polos (seg cópias coincidentes).
2. **Cylinder/Cone duplicados**: anéis laterais + tampas reimplementados; sem gerador radial comum (§9).
3. **Sem tampas opcionais**: cylinder/cone sempre tampados; sem `cap_top/cap_bottom`.
4. **Três caminhos de criação**: shelf/outliner via sessão (Wave 8), menu Add da viewport via `AddPrimitiveCmd` direto (**fura a sessão** — §65/§82), `generate_mesh` com defaults próprios divergentes da sessão.
5. **Cube sem W/H/D** (§11); Plane sem W×D no builder (sessão escala, builder não).
6. **Sem validador de primitivas** (§28/§57): só `Mesh::validate` (reparo de import).
7. **Sem Wedge/Circle/Torus/Icosphere** em geometria, UI, i18n e testes.
8. **Sessão Wave 8 cobre 6 espécies com defaults próprios** (Cube 2.0, Cyl 12, Plane 2×2) — divergem do §5; sem Reset nem estatísticas (§22/23/25).
9. **Sem menu agrupado** (§36): Add+ lista 5 itens planos; shelf tem 4 atalhos; Cone/Capsule só no outliner/Add+.

## Decisões de arquitetura (ledger)

- Geometria fica em `crates/mesh/src/primitives.rs` (casa canônica; sem nova árvore `mesh/primitives/`).
- `PrimitiveKind` estendido (Wedge, Circle, Torus, Icosphere); `generate_mesh` = defaults §5.
- `PrimitiveDescriptor` (sessão) estendido com os 10 + `Reset` = `default_for(kind)`.
- Frustum radial comum: `radial_frustum(bottom_r, top_r, height, sides, cap_bottom, cap_top)`; `cylinder`/`cone` delegam (assinaturas preservadas).
- Menu Add+ vira grupos BASIC/ROUND/ORGANIC e roteia pela sessão (caminho único).
- Polos soldados em `sphere_low`/`capsule_profile` (sem vértices coincidentes).
- Limites (§44): sides 3–32, rings 2–24, ico subdiv 0–3, torus segs 3–64/3–32, raios > 0 (clamp + testes).

## Implementação (Waves P1–P11, 2026-09-15)

- **P1 fundação**: `radial_frustum` comum (cylinder/cone delegam, windings originais preservados); `primitive_audit` (finitude, índices, degeneradas exatas, bounds, outward, coincidentes); `cube`/`plane` com clamp anti-degeneração.
- **P2**: `box_dim` W/H/D, defaults `generate_mesh` low-poly (Sphere 12/6, Cyl/Cone 8, Capsule 8, Plane 1×1).
- **P3**: `wedge` (6v/5f, windings verificados), caps opcionais no frustum, `circle` (anel ngono / leque soldado).
- **P4**: polos soldados em `sphere_low`, `icosphere` 0–3 com cache de aresta (12/42/162/642v), `capsule_profile` com calotas cosseno parametrizadas (fusão de anel com corpo zero), `torus` sem costura duplicada.
- **P5–P6**: `PrimitiveDescriptor` com 10 espécies + sessão existente reutilizada (Reset, estatísticas verts/faces/tris no cartão, 1 transação).
- **P7**: menu Add+ agrupado + `primitive_menu_groups` testado (10 exatas); shelf/outliner roteiam pela sessão; viewport Add direto eliminado.
- **P8**: ~45 chaves `prims.*` en/pt-BR + `TextId`, tooltips §41, dropdowns Cap/Fill com largura local.
- **P9**: regeneração escopada (1 asset, sem checkpoint, fingerprint existente invalida só o necessário); sem validadores caros por frame.
- **P10**: `primitive_pipeline_tests.rs` — 10 espécies × save/load/OBJ/GLB + cancel-nunca-salva.
- **P11**: `docs/manual/primitives.md`, CHANGELOG, gates abaixo.

## Pós-gauntlet: unificação do caminho de criação (modelagem)

- `AddPrimitiveCmd::execute` e `PrimitivesTool::add_primitive` roteiam pela sessão (`begin_primitive`); `is_destructive()=false` no comando (checkpoint único, sem duplicar com o dispatcher).
- `model_primitives_list` com as 10 espécies; botão Extrude Individual no form; `model.revolve` + `model.add_{wedge,circle,torus,icosphere}` registrados (paleta/docs via `docs-generate`).
- Restante inventariado (fora deste slice): Merge by distance (só `merge_center` existe; `weld(eps)` pronto no mesh), Bevel multi-segmento sem UI (spec P3D-032 fixa 1 segmento — expor exige adendo), Quick Symmetry e Origin/Pivot Presets (candidatos Era 1, inexistentes), Sweep sem UI (extensão V1.x futura por design).

## Achados corrigidos no caminho

- Windings do frustum/cápsula/esfera derivados à mão e verificados pelo audit (3 inversões pegas antes de commitar qualquer geometria).
- `loop_cut` teste de anel dependia da indexação intercalada antiga (seed atualizada).
- `cube(0)`/`plane(0)` geravam lixo degenerado (clamp nos builders).
- Capsule corpo-zero alocava anel duplicado (push condicional).
- Tolerância de degeneração passou a exata (malhas minúsculas válidas não acusam).
