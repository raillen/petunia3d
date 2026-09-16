# P3D-084 — Design System tokenizado

<aside>
🧩

Estado: **implementação precisa auditoria completa** · Prioridade: P0.

</aside>

## Objetivo

Toda aparência da UI controlada por tokens semânticos e componentes Petunia, não por hardcodes espalhados.

## Auditoria

Buscar cores RGB/Color32, spacing, radii, strokes, font sizes e estados definidos diretamente em widgets. Mapear wrappers já existentes e duplicações.

## Contrato

Tokens nomeiam intenção (`surface.panel`, `text.muted`, `border.focus`) e não tonalidade (`gray700`). Themes parciais usam fallback seguro.

## Integrações

P3D-085–089, P3D-077–079.

## Testes / DoD

Theme switch runtime, fallback, contraste básico, nenhum novo hardcode visual em componentes públicos e docs de tokens geradas.