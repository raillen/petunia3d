# Manual de Uso e Navegação — Petunia3D

> **Vocabulário:** na interface, componentes da malha são **Point**, **Edge** e **Face**;
> o arredondamento de quinas é **Round Edge**. `Vertex`, `Bevel` e `Chamfer` são os
> termos técnicos equivalentes (código, comandos internos e changelog).

## Viewport

`MMB` orbita; `Shift+MMB` faz pan no plano da tela; a roda ajusta o zoom.
`F` enquadra suavemente a seleção (ou a malha ativa, sem seleção).
`Home` restaura a câmera. `Numpad 1/3/7` muda para Front/Right/Top,
`Numpad 9` inverte a vista e `Numpad .` enquadra a seleção.

`1/2/3` escolhe o domínio `Point`/`Edge`/`Face` e passa a operar nos componentes
da malha ativa. `0` escolhe o objeto inteiro. Clique substitui a seleção;
`Shift+clique` alterna componentes. O contorno de destaque antecipa o alvo.
Wireframe (`Z`) permite picking através de faces da malha ativa. A tolerância é 8
pixels para `Point` e 5 para `Edge`; clique longe do componente não tem fallback
para outro alvo.

## Modelagem direta

| Atalho | Interação |
|---|---|
| `G`, `R`, `S` | Mover, rotacionar, escalar com o mouse |
| `E` | Extrudar região de faces pela normal média |
| `I` | Inset proporcional, fator entre 0 e 0,95 |
| `P` | Transladar seleção pela normal média (Push/Pull) |
| `Ctrl+B` | **Round Edge** (bevel) de uma aresta, com cantos simples |
| `Ctrl+R` | Prévia de loop de quads; roda muda cortes; clique inicia slide; segundo clique aplica |
| `K` | Faca: clique em duas arestas da mesma face; repita segmentos e confirme |
| `Shift+K` | Arraste um plano de corte; conserva o lado positivo e fecha a tampa |
| `Shift+P` | Perfil desenhado na vista ortográfica |
| `Delete` | Remover seleção |
| `X` | Dissolver (pela ferramenta contextual) |
| `Ctrl+Z`, `Ctrl+Shift+Z` | Desfazer e refazer |

As ferramentas modais exibem guia e HUD. `Enter` ou `LMB` confirma;
`Esc` ou `RMB` cancela com restauração do estado anterior. Um arrasto pelo
gizmo confirma ao soltar o botão. No slide de loop cut, `RMB` centraliza e
confirma; `Esc` desfaz todo o corte. No fatiamento, solte o arrasto para
inspecionar a prévia e confirme com `Enter` ou outro clique.

Durante transformações, `X/Y/Z` restringe ao eixo; `Shift+X/Y/Z` restringe
ao plano perpendicular. Repetir a restrição libera o movimento. Digite um
número exato, inclusive negativo ou decimal; `Backspace` corrige a entrada.
`Ctrl` ativa incrementos de 0,1 (15° na rotação). Valores rejeitados mantêm a
última prévia válida e impedem a confirmação até serem corrigidos.

O gizmo lembra o último modo `G/R/S`: setas e planos para mover, anéis para
rotacionar e pontas quadradas para escalar. Os painéis ficam desativados durante
as transações. Clicar na toolbar apenas arma a ferramenta; mova o cursor para
o viewport para iniciar a sessão. Merge/Mirror/Subdivide continuam com aplicação
explícita em seus painéis.

## Paint, UV e exportação

No workspace PAINT, clique/arraste pinta a superfície em **Paint on Model**
(canal Albedo). Cada traço gera um único undo. `Esc` cancela o traço e aguarda um
novo clique. `Alt+clique` ou `G` captura a cor do ponto mais próximo na face sob o
cursor. `F` ajusta o raio arrastando; clique/`Enter` confirma e `Esc` restaura o
raio. O círculo acompanha o plano da face atingida; não é uma projeção curva sobre
múltiplas superfícies. O canvas 2D e as operações de UV permanecem nos respectivos
workspaces — o editor 2D de textura é painel opcional, fechado por padrão.
A exportação oferece `.glb` (principal) e `.obj` (secundário); FBX está fora da V1.

## Limites conhecidos desta rodada

Round Edge suporta uma aresta de superfície fechada, com extremidades
trivalentes; configurações não suportadas são rejeitadas. Ainda não há segmentos
arredondados no caminho principal nem Round Edge simultâneo de várias arestas —
multi-segmento é recurso avançado, além do Core V1. Inset é proporcional, não uma
distância em metros, e não garante ausência de auto-interseção em faces côncavas.
Loop cut aceita faixas/anéis de quads orientáveis e rejeita ramificações ou
polígonos não quadrangulares no percurso. A faca exige cliques nas arestas
intermediárias ao atravessar várias faces. Fatiamento com tampas contendo buracos
não está validado. O picking de componentes considera a malha ativa; oclusão por
outros assets ainda requer extensão.

## Personalização

Os atalhos pertencem a um **perfil de keymap** em `assets/keymaps/` — o preset
default é `assets/keymaps/petunia-default.toml`. Cada bind aponta para um
`CommandId`, e o remapping completo (busca, captura de tecla, detecção de conflito,
reset e import/export JSON) fica em `Settings → Keymap`.

> Consulte o [catálogo gerado de perfis e atalhos](../generated/KEYBINDS.md) — ele é
> derivado do código e nunca diverge do que o aplicativo realmente usa.
