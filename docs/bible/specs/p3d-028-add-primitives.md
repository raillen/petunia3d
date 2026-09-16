# P3D-028 — Add Primitives

<aside>
🧩

Estado: **parcial; apenas Cube** · Prioridade: P0.

</aside>

## Objetivo

Adicionar primitivas low-poly aprovadas com parâmetros editáveis no momento da criação.

## Requisitos

Cube, Plane, Cylinder, Sphere/Icosphere quando fizer sentido, Cone, Capsule e outras somente se justificadas pelo escopo. Cada primitive deve nascer com parâmetros explícitos como sides/segments/radius/depth antes da confirmação.

## UX

A criação abre Tool Properties contextual; o usuário ajusta parâmetros e confirma/cancela. Não obrigar edição posterior para escolher uma geometria low-poly básica.

## Arquitetura

Primitive generators são independentes da UI, retornam mesh/procedural descriptor testável headless e integram undo. Preservar parâmetros proceduralmente enquanto útil; conversão para mesh editável é explícita.

## Dependências

P3D-100, P3D-101, P3D-131.

## Testes / DoD

Parâmetros mínimos/máximos, criação/cancelamento, IDs estáveis, undo/redo, orientação e performance.