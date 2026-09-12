# Retomada — Gauntlet Petunia3D

Atualizado: 2026-09-12. Rodada 2 em andamento.

## Pedido e continuidade

Continuar o Gauntlet com foco em câmera ortográfica, ícones vetoriais, UI legível
em janelas estreitas e campos numéricos preenchíveis para ferramentas. Preservar
prévia, cancelamento exato e undo único. O usuário autorizou um commit do estado
validado e pediu este documento antes de esgotar sua cota do ChatGPT.

A API desta sessão não expõe percentual restante da cota do ChatGPT. Portanto,
não é possível detectar precisamente 3%; este documento é mantido preventivamente.

## Último estado comprovado antes desta rodada

128 testes, fmt, Clippy estrito e smoke passaram. Captura GL real em Intel HD4000
validou o viewport central. Evidências: `.prumo/history/premium/`.

## Rodada atual — responsabilidades

- `core/camera`, `ui/camera_controls`: projeção ortográfica explícita, seis vistas,
  preservação de escala, orbit ortográfico e altura exata.
- `ui/icons`, `config/theme`: desenhos vetoriais e tokens de legibilidade.
- `ui/tool_fields`, `core/modal`: campos exatos com prévia transacional.
- `ui/lib`: cabeçalho e rodapé responsivos, toolbar e painel de propriedades.

## Antes de interromper ou continuar

1. Ler `PROJECT_STATE.md`, este documento e `docs/GAUNTLET.md`.
2. Inspecionar `git status` e o último commit; não reverter trabalho existente.
3. Rodar fmt, testes workspace, Clippy e smoke após alterações.
4. Comparar capturas GL em janela estreita e ampla; validar câmera e campos.
5. Atualizar este documento com resultados, pendências e commit realizado.

O repositório começou sem commits: todos os arquivos do projeto estavam untracked.
O commit solicitado deve conter o projeto e a documentação, excluindo `target/`
e caches/runtime ignorados. Não há autorização de push remoto.

## Pendências premium anteriores

Ver `docs/development/premium-interaction-plan.md`: bevel multiaresta/arredondado,
inset métrico/côncavo, picking entre assets, pintura de texturaUV no viewport,
caminho dourado por artista e desempenho de release ainda não comprovados.
