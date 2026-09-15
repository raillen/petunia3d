# P3D-138 — Animation Retargeting / External Compatibility

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 10 (Pós-GA)
- **Status Canônico**: `COMPLIANT (Implementado e Verificado)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Estado: **implementado via `RetargetProfile` e `retarget_clip` em `crates/project/src/animation.rs`** · Prioridade: P3.

</aside>

## Objetivo

Importar/retargetear animações externas para rigs Petunia suportados.

## Contrato

Retarget profiles mapeiam bone semantics/naming e transform conventions. Formatos são tratados por importers; não integrar ao site Mixamo como requisito do core.

## Compatibilidade

Documentar claramente rigs/formats suportados, root motion e limitações.

## Dependências

P3D-135, P3D-136, P3D-071.

## Testes / DoD

Clips externos representativos, mapping incompleto, scaling/orientation, playback comparativo e erros compreensíveis.