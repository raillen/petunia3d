# P3D-009 — Grid 3D

<aside>
🧩

Estado inicial: **parcial / configuração incompleta** · Prioridade: P1.

</aside>

## Objetivo

Grid 3D configurável para orientação e modelagem sem comprometer performance.

## Requisitos

- toggle on/off;
- tamanho/spacing configurável;
- cor e opacidade configuráveis por settings/theme conforme escopo;
- integração clara com snapping sem confundir visual grid com regra de snap.

## Arquitetura

Renderer recebe descriptor de grid; UI apenas edita settings. Cores padrão usam ThemeToken; preferência do usuário pode ser persistida em Application Settings, não necessariamente no projeto.

## Dependências

P3D-010, P3D-040, P3D-084.

## Testes / DoD

Grid desativado realmente não renderiza; mudanças refletem sem recrear recursos caros desnecessariamente; DPI/zoom e ortho/perspective permanecem legíveis.