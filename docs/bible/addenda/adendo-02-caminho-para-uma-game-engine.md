# ADENDO-02 — Caminho para uma game engine

<aside>
🧩

Este adendo foi formalizado como **P3D-143 — Game Engine Integration / Bridge**.

</aside>

## Direção

Uma futura engine pode focar games retro/low-poly e facilidade semelhante a ferramentas como GameGuru Max, mas com maior liberdade. Ela deve permanecer **produto independente**, compartilhando formats/APIs/bridges com Petunia.

## Regra anti-bloat

Não mover ECS/game runtime/physics/map editor para o core do Petunia. O Petunia produz assets; a engine consome assets.

## Próxima etapa futura

Pesquisar contrato de asset bridge, live-reload opcional, material/animation mapping e collision/LOD metadata conforme P3D-143.