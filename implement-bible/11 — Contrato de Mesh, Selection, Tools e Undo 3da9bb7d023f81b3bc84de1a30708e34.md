# 11 — Contrato de Mesh, Selection, Tools e Undo

<aside>
🧰

Contrato compartilhado para P3D-015–041, P3D-083, P3D-100–101 e P3D-131. Evita que cada ferramenta implemente seleção, preview, confirmação e undo de forma diferente.

</aside>

# Selection Domain

A UX pública usa `Object / Vertex / Edge / Face`. `Tab` alterna Object ↔ último domínio de componente. O estado interno pode ter subestados, mas não deve criar uma segunda verdade pública.

# Selection semantics

Congelar comportamento para replace/add/subtract/toggle, marquee, hidden/locked, X-Ray, seleção via Outliner e seleção sincronizada. Selection é editor/application state; widgets apenas consultam/solicitam mudanças.

# Topologia e invariantes

Auditar o mesh model atual e registrar invariantes: referências válidas, winding, faces degeneradas, edges órfãs, non-manifold quando permitido, normals/tangents e IDs/handles. Toda operação topológica deve validar invariantes após commit em debug/tests.

# Tool lifecycle canônico

`Inactive → Armed/Activated → Preview/Modal → Confirmed | Cancelled` quando aplicável. Activate/deactivate/cancel não deixam estado fantasma. Troca de tool encerra/cancela explicitamente a operação anterior conforme contrato.

# Preview vs Commit

Preview é temporário e não deve gerar dezenas de entradas de Undo. Confirm gera uma transação semântica. Cancel restaura exatamente o estado anterior.

# Modal Tool Feedback

P3D-131 fornece guideline/line, delta, axis constraint, snap, numeric input e hints de status de forma reutilizável. A tool descreve feedback; renderer/frontend apresenta sem acoplar algoritmo ao egui.

# Axis, Snap e Numeric Input

X/Y/Z e planos aprovados devem usar a mesma infraestrutura entre Move/Rotate/Scale/Extrude/Inset/Bevel quando semanticamente compatível. Keymaps resolvem teclas; tools consomem intent semântico.

# Undo/Redo

Gestos contínuos são uma transação. Operações topológicas, paint strokes, property drags e transform drags devem declarar boundary de transaction. Redo não depende de widget ou mouse original. Branch após Undo invalida redo de forma previsível.

# Error/invalid-state policy

Operação inválida não corrompe mesh e não entra no history. Se algo puder ser parcialmente aplicado, usar validação/preflight ou transaction rollback.

# Tests obrigatórios

Headless tests por operation; property/invariant tests onde úteis; cancel/confirm; undo/redo; multi-selection; locked/hidden; invalid topology; sequences longas; deterministic replay quando aplicável.

# DoD

Uma nova modeling tool pode reutilizar selection/modal/undo infrastructure sem criar um segundo lifecycle e sem alterar módulos não relacionados.