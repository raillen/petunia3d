# Animation — pós-V1 (fora da baseline V1)

> ⚠️ **Este não é um workspace V1.** A UI Baseline Final V1 congela apenas
> `MODEL / PAINT / UV`. Rig/animação completos estão explicitamente fora do core
> inicial (`Out of Scope`, capítulo 12) e são catalogados como pós-V1 em
> [P3D-066](../bible/specs/p3d-066-animation-workspace.md),
> [P3D-135](../bible/specs/p3d-135-skeleton-rig-core.md),
> [P3D-136](../bible/specs/p3d-136-rig-presets-humanoid-quadruped-e-mult.md) e
> [P3D-067](../bible/specs/p3d-067-animacao-simples.md).

## Status no build atual — divergência resolvida

**Resolvido em 2026-09-16.** O build de V1 **não expõe** mais um quarto workspace: a
variação `Workspace::Animate` passou a ser compilada apenas com a feature
`animation-workspace`, desligada por padrão.

```bash
cargo build                         # MODEL / PAINT / UV (V1 congelada)
cargo build --features animation-workspace   # + ANIMATE (P3D-066, pós-V1)
```

Consequências:

- a barra de pílulas do header é derivada de `Workspace::all()` e nunca mostra um
  workspace fora do escopo congelado;
- o módulo de animação e sua timeline continuam **compiláveis e testáveis** atrás da
  feature — nada foi removido, apenas retirado do caminho de usuário da V1;
- `Workspace::COUNT` dimensiona a memória de layout por workspace, então ligar a
  feature não quebra índices nem persistência.

O módulo de dados (`crates/project/src/animation.rs`) permanece no formato de
projeto: esqueletos e clipes já são serializáveis, o que é justamente o pré-requisito
de P3D-135/136/139.

## Conteúdo pós-V1 previsto

Quando promovido, o recorte previsto é: workspace com viewport superior e timeline
inferior (transporte, régua de quadros e marcadores) mais inspeção de keyframes de
posição/rotação/escala. Nada disso é requisito da V1 e nenhum item desta página
representa compromisso de entrega.
