# Temas de Exemplo (`assets/theme-examples/`)

Packs de tema **não oficiais**, mantidos como exemplo de declaração, estudo de
paleta e teste de contraste. Nada aqui é embutido no produto nem carregado por
padrão.

| Pack | Caráter |
| --- | --- |
| [`petunia-light/`](petunia-light/) | Paleta clara técnica |
| [`petunia-capuccino/`](petunia-capuccino/) | Tons terrosos quentes |
| [`petunia-tokyo-nights/`](petunia-tokyo-nights/) | Paleta escura de acento frio |

## Por que estão fora de `assets/themes/`

O capítulo 36 congela a V1 com **dois** temas oficiais: `petunia-dark` (completo,
default) e `petunia-high-contrast` (acessibilidade). Qualquer outro tema é
declaração externa do usuário, carregada de
`<diretório-de-usuário>/themes/<id>/` — nunca embutida no editor.

## Como usar um pack daqui

1. Copie a pasta para o diretório de temas do usuário:
   `<config-do-usuário>/petunia3d/themes/<id>/`;
2. Mantenha `manifest.toml` e `theme.toml` juntos;
3. O tema aparece no seletor de temas sem recompilar o editor.

Um pack parcial é válido: tokens ausentes herdam o default (`petunia-dark`).

## Contrato

Valem as mesmas regras dos temas oficiais: puramente declarativo, sem código,
confinado ao próprio pack, e incapaz de alterar layout estrutural, densidade do
shell ou o documento `.petunia`. Ver [`../themes/README.md`](../themes/README.md).
