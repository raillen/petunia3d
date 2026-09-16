# P3D-082 — Context Menus

<aside>
🧩

Estado: **parcial; precisa unificação** · Prioridade: P1.

</aside>

## Objetivo

Right-click menus realmente contextuais a Object/Vertex/Edge/Face e outros locais.

## Regra

Usar o mesmo sistema visual de P3D-077 e os mesmos CommandIds de menu/toolbar/keybind. Não duplicar callbacks.

## Conteúdo

Somente commands relevantes ao contexto; mais específico ganha. Text input/popup/modal tool não pode abrir context menu indevido.

## Dependências

P3D-015, P3D-077, P3D-100.

## Testes / DoD

Cada Selection Domain, Outliner, Asset Browser e estado sem seleção; submenu/shortcut e dismissal.