# Linha do Tempo & Animação — pós-V1

> ⚠️ **Fora da baseline V1.** A UI Baseline Final V1 congela `MODEL / PAINT / UV`, e
> rig/animação completos estão explicitamente fora do core inicial (capítulo 12).
> O escopo pós-V1 está catalogado em
> [P3D-066](../bible/specs/p3d-066-animation-workspace.md),
> [P3D-067](../bible/specs/p3d-067-animacao-simples.md),
> [P3D-135](../bible/specs/p3d-135-skeleton-rig-core.md) e
> [P3D-136](../bible/specs/p3d-136-rig-presets-humanoid-quadruped-e-mult.md).

Esta página não descreve um caminho de usuário da V1 e não representa compromisso de
entrega. O que existe hoje no build é um quarto workspace (`Animate`) além da
baseline — divergência registrada na
auditoria de conformidade (`docs/audits/bible-conformance/`) e pendente de
resolução na trilha de implementação.

## Recorte previsto (quando promovido)

- linha do tempo enxuta com transporte (play/pause, primeiro/último quadro);
- inserção de quadros-chave de posição, rotação e escala;
- inspeção de interpolação no painel Context.

Nada acima é requisito da V1.
