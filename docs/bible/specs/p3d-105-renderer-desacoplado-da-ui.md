# P3D-105 — Renderer desacoplado da UI

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 1
- **Status Canônico**: `COMPLIANT (Renderer Boundary WGPU / GL)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **precisa auditoria de renderer/egui integration** · Prioridade: P0.

</aside>

## Objetivo

Separar Scene/Viewport renderer da composição egui.

## Auditoria

Mapear ownership de wgpu device/queue/surface, render targets, camera, picking, resize, egui renderer e tool overlays. Verificar se é possível render offscreen sem abrir egui.

## Contrato

Renderer recebe scene/camera/material/overlay descriptors neutros. Frontend fornece viewport size/native target/integration adapter.

## Dependências

P3D-004, P3D-102–104, P3D-131.

## Testes / DoD

Offscreen/smoke render quando viável, no egui types em render core e viewport continua funcional após boundary.