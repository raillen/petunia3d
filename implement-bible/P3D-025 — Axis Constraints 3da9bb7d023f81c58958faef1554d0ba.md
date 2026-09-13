# P3D-025 — Axis Constraints

<aside>
🧩

Estado: **funcional em um eixo; ampliar com cuidado** · Prioridade: P1.

</aside>

## Objetivo

Restringir Move/Rotate/Scale/Extrude a eixos e, quando útil, planos.

## Decisão a implementar após auditoria

Avaliar XY/XZ/YZ ou semântica de excluir um eixo. Só adotar se reduzir passos sem criar keymap confuso.

## Arquitetura

Constraint é parâmetro semântico de tool/transform, não leitura de tecla. Keymap/modal input resolve X/Y/Z/combinações para o descriptor.

## Dependências

P3D-021–023, P3D-029, P3D-090.

## Testes / DoD

Eixo único, plano, local/global, cancelamento e feedback visual coerente.