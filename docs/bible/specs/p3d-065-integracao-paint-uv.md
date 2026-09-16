# P3D-065 — Integração Paint ↔ UV

<aside>
🧩

Estado: **a implementar após fundações** · Prioridade: P1.

</aside>

## Objetivo

Alternar Paint e UV preservando contexto e permitindo entender imediatamente qual material/texture/channel está sendo editado.

## Contrato

Mesmo MaterialId/TextureResource/UVSet; workspace switch não duplica bitmap nem selection. Quando útil, seleção de faces/ilhas pode ser transferida por uma representação explícita.

## Dependências

P3D-050, P3D-055, P3D-063–064.

## Testes / DoD

Alternância repetida, undo atravessando workspaces, save/load e atualizações refletidas no Material Preview.