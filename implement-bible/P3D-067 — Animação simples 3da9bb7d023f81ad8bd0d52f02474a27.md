# P3D-067 — Animação simples

<aside>
🧩

Definição original genérica; precisa ser fechada sobre o novo Rig Core · Prioridade: P3.

</aside>

## Objetivo

Fornecer keyframes/clips mínimos para transformações de bones/rigs e playback previsível.

## Escopo inicial sugerido

- timeline simples;
- keyframe add/delete/move;
- playback/loop;
- clip start/end;
- transform channels necessários ao rig;
- interpolation limitada e clara.

## Não objetivos iniciais

Graph editor complexo, simulation, constraints avançadas, NLA sofisticado.

## Dependências

P3D-135 e P3D-066.

## Testes / DoD

Create/edit/play clip, keyframes, interpolation suportada, undo, save/load e determinismo.