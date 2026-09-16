# Arte de ícones arquivada (`assets/legacy-icon-art`)

Material de arte **não registrado** como pacote de ícones do produto.

## Por que isso existe

Os pacotes de ícones oficiais são `petunia`, `tabler`, `iconoir`, `phosphor` e
`lucide` — e vivem em `assets/icons/<id>/` com `manifest.toml` + `icons.toml`. O
carregador percorre `assets/icons/*` e só registra diretórios que possuem
`manifest.toml`.

`future-dark/` é um acervo de PNGs (pack "Petunia UI icons", artes 3D e recortes
por categoria) que **nunca** foi registrado como pacote: não tem manifest, não
aparece no seletor de ícones e não faz parte de nenhuma lista oficial do Livro Vivo
(Tabler, Iconoir, Phosphor, Lucide + Petunia Custom Icons — P3D-087/P3D-088).

O material foi movido para fora de `assets/icons/` para não poluir o diretório de
packs com arte que o runtime não usa. Ele continua no repositório porque é insumo
visual reaproveitável.

## Como promover algo daqui a pacote oficial

1. Selecionar apenas os ícones efetivamente usados (nada de placeholders de
   quadrado/círculo/emoji).
2. Criar `assets/icons/<id>/manifest.toml` (id, nome, versão, autor, licença) e
   `icons.toml` mapeando `IconId` → asset.
3. Garantir que o pack respeite tamanho óptico, stroke e hitbox do sistema, e que
   trocar de pack não mude o significado do `IconId`.
4. Registrar licenças/origem em `THIRD_PARTY_NOTICES`.
5. Atualizar documentação de PACOTES OFICIAIS **e** pedir decisão explícita, porque
   a lista de packs é contrato do produto (P3D-087).
