# P3D-077 — Menus profissionais

<aside>
🧩

Estado: **dropdowns/submenus ainda crus, próximos do egui default** · Prioridade: P0.

</aside>

## Objetivo

Sistema único de menus/dropdowns/submenus/context menus com linguagem visual Petunia.

## Componentes

`PetuniaMenuButton`, `PetuniaPopupMenu`, `PetuniaMenuItem`, `PetuniaSubmenu`, `PetuniaSeparator`, toggle/radio items e selector dropdowns distintos de command menus.

## Layout de item

Icon column + label + shortcut secundário + submenu arrow. Shortcuts vêm do keymap atual; labels usam TextId.

## UX

Padding/row height/radius/surface/hover/open state via ThemeTokens; submenu reposiciona quando não cabe. Menus superiores e right-click compartilham infraestrutura.

## Dependências

P3D-084, P3D-088–090.

## Testes / DoD

Keyboard navigation, submenu, viewport bounds, disabled/toggle/radio, idioma longo e 1366×768.