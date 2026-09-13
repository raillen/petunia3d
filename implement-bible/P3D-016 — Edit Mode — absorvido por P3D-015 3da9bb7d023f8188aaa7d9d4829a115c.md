# P3D-016 — Edit Mode — absorvido por P3D-015

<aside>
🧩

Status: **absorvido por P3D-015**. Não implementar como UX pública separada.

</aside>

## Missão

Auditar o código existente de Edit Mode e migrar somente responsabilidades internas realmente necessárias para o modelo unificado Object/Vertex/Edge/Face.

## Regras

- Não apagar lógica útil antes de cobri-la por testes.
- Remover/ocultar apenas a separação pública redundante.
- Tools/topology state podem continuar usando contexto interno desde que não obriguem o usuário a alternar modos artificiais.

## Critério de encerramento

P3D-015 oferece toda a funcionalidade necessária, `Tab` funciona como definido, nenhuma tool perde contexto e não existem dois sistemas paralelos de seleção.