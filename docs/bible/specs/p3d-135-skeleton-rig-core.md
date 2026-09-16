# P3D-135 — Skeleton & Rig Core

<aside>
🧩

Novo subsistema fundamental de animação · Prioridade: P3.

</aside>

## Objetivo

Núcleo mínimo de skeleton/rig independente da UI.

## Modelo mínimo

`SkeletonId`, `BoneId/JointId`, hierarchy parent/children, local/rest transform, bind pose, skin weights e constraints mínimas somente quando justificadas.

## Arquitetura

Skinning/render é adapter sobre os dados do rig; timeline e rig widgets não possuem a verdade do skeleton. IDs estáveis permitem retargeting, animation library e APIs.

## Segurança/robustez

Evitar ciclos na hierarquia, weights inválidos e bones órfãos; erros estruturados.

## Testes / DoD

Criar hierarchy, reparent válido/inválido, bind pose, weights normalizados, save/load, undo e aplicação headless de transforms.