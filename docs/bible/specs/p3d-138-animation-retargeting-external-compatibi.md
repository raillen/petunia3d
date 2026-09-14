# P3D-138 — Animation Retargeting / External Compatibility

::: info STATUS DA ESPECIFICAÇÃO — SINGLE SOURCE OF TRUTH
- **Wave do Gauntlet**: Wave 10 (Pós-GA)
- **Status Canônico**: `ROADMAP PÓS-GA (Animation & Rigging)`
- **Contrato**: Verificado contra a suíte de testes automatizados e o código-fonte canônico.
:::


<aside>
🧩

Novo item; Mixamo é workflow de referência, não dependência proprietária · Prioridade: P3.

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