# P3D-033 — Knife / Cut

::: tip STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 5
- **Status Canônico**: `COMPLIANT (Selection Domain, Transform & Modeling Core)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado reportado: **implementado e funcional** · Trabalho: validar/regredir · Prioridade: P1.

</aside>

## Objetivo

Preservar corte manual controlado enquanto se comprova robustez topológica e integração com o modelo unificado.

## Auditoria

Validar lifecycle da tool, snapping opcional, hit-testing, preview, confirm/cancel, seleção resultante e undo.

## Arquitetura

Cálculo de interseção/corte não depende do painter egui; overlay da linha de corte é presentation descriptor.

## Dependências

P3D-015, P3D-040, P3D-041, P3D-123.

## Testes / DoD

Cortes válidos/fora da mesh, cruzamento de múltiplas faces, cancel e topologia consistente.