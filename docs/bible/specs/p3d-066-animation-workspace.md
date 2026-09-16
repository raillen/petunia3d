# P3D-066 — Animation Workspace

<aside>
🧩

Escopo expandido; não tratar como feature monolítica · Prioridade: P3.

</aside>

## Objetivo

Workspace de animação low-poly simples, construído sobre subsistemas separados e reutilizáveis.

## Decomposição obrigatória

- P3D-135 Skeleton & Rig Core.
- P3D-136 Rig Presets.
- P3D-137 Auto-Rig.
- P3D-138 Retargeting/compatibilidade externa.
- P3D-139 Animation Asset Library.

## UX

Timeline e painéis de rig/animação aparecem principalmente neste workspace, não permanentemente nos workspaces de Model/Paint/UV. Interface deve priorizar clips/keyframes e edição simples, não dezenas de graph editors.

## Arquitetura

Workspace é frontend; skeleton/clip/weights vivem fora da UI e são serializáveis/testáveis.

## Testes / DoD

Trocar workspace sem perder contexto, rigs/clips básicos visíveis/editáveis e nenhum acoplamento do core a timeline/widget.