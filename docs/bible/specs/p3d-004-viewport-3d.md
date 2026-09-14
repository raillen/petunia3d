# P3D-004 — Viewport 3D

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 4
- **Status Canônico**: `COMPLIANT (Viewport, Navigation & Reference Workflow)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado inicial: **funcional, validar performance** · Trabalho: refino/validação · Prioridade: P1.

</aside>

## Objetivo

Preservar o viewport 3D já funcional e elevar sua robustez/performance em máquinas modestas.

## Decisões

- Não reescrever renderer só por polish.
- Medir frame time, memória, resize e custo de overlays/shading antes de otimizar.
- Settings pode expor qualidade/antialiasing apenas se houver suporte real e fallback seguro.

## Auditoria obrigatória

Mapear renderer, camera, picking, surface resize, texture targets, integração egui-wgpu e qualquer alocação por frame. Confirmar se lógica de viewport vaza para widgets.

## Contrato de implementação

Câmera/picking usam tipos neutros; egui apenas hospeda a view. Nenhum decode/asset parse por frame. Preservar comportamento atual enquanto são introduzidos testes e métricas.

## Integrações

P3D-105, P3D-125 e P3D-126. Avaliar TextId/IconId/ThemeToken para Settings e controls.

## Testes / DoD

- viewport renderiza e redimensiona corretamente;
- navegação/seleção não regressam;
- baseline de performance registrada;
- opções de qualidade não quebram PCs modestos;
- docs/screenshots atualizados quando UI mudar.