# P3D-115 — Ajuda contextual ?

<aside>
🧩

Estado: **precisa integração com docs** · Prioridade: P1.

</aside>

## Objetivo

Links de ajuda contextuais para manual oficial sem espalhar URLs em widgets.

## Modelo

`DocsTopic/DocsId` ou metadata equivalente resolve URLs estáveis. Vários commands podem apontar para a mesma página temática.

## UX

`?` aparece onde realmente reduz dúvida, não ao lado de todo botão. Abrir documentação online e futuramente permitir docs offline sem mudar domínio.

## Dependências

P3D-116, P3D-119, P3D-100.

## Testes / DoD

Todos DocsTopics resolvem, links quebrados falham CI e nenhum algoritmo core conhece URL.