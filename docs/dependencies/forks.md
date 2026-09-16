# Fork Registry — forks e pins Petunia

<aside>
🍴

**Arquivo obrigatório para forks e pins de Git** (§19.5 e §20 da *Egui Ecosystem
Final Push Directive*). Um fork ou `git =` só pode entrar no manifesto depois de
existir aqui.

</aside>

# Estado atual

**Nenhum fork. Nenhum pin de Git. Nenhum `[patch.crates-io]` de UI.**

Todas as dependências de UI vêm de releases do `crates.io` — ver
[`ui-ecosystem-lock.md`](./ui-ecosystem-lock.md). Isso é o cenário mais barato de
manter e o preferido: só saímos dele quando a ordem normativa de compatibilidade
(§18) chega ao passo 3 ou 4.

# Quando usar cada mecanismo

| Mecanismo | Quando | Custo |
| --- | --- | --- |
| release do `crates.io` | default | nenhum |
| **revision pin** (`git =`, `rev = SHA`) | upstream já suporta `egui 0.36`, mas sem release novo | acompanhar manualmente; sem patch próprio |
| **fork mínimo** | upstream precisa de delta real para suportar a baseline | paga rebase a cada release do upstream |
| `[patch.crates-io]` | fork precisa substituir uma dependência transitiva | afeta o grafo inteiro; só com ADR |

**Regra:** tentar pin antes de fork, e fork antes de reimplementar.

# Política de saída (§20)

Todo fork declara, no momento em que é criado:

1. **motivo**: qual comportamento a baseline exige e o upstream não entrega;
2. **delta**: a menor lista possível de commits/arquivos alterados;
3. **condição de saída**: o que precisa acontecer no upstream para voltarmos ao
   release (versão mínima, issue/PR de referência);
4. **revisão**: com que frequência o fork é rebaseado;
5. **fallback**: o que acontece se o fork ficar sem manutenção.

Um fork sem condição de saída mensurável não pode ser adicionado.

# Registro

<!-- Preencher uma linha por fork/pin. Exemplo comentado abaixo. -->

| pacote | mecanismo | revision/SHA | motivo | delta | saída para o upstream | revisão |
| --- | --- | --- | --- | --- | --- | --- |
| _(nenhum)_ | — | — | — | — | — | — |

<!--
Exemplo de registro futuro (mantido comentado até existir de fato):

| `egui-notify` | fork Petunia | `petunia/egui-notify@<sha>` | release publicado fixa `egui 0.34`; bump é de 1 linha | `Cargo.toml` (bump de `egui`) | upstream lançar 0.22.x com `egui 0.36` | rebase a cada release do `egui` |

Regra do exemplo: só vira fork **depois** de uma branch de teste provar que o bump
de `egui` é pequeno (§44, ação 4).
-->

# Proibições

- fork **privado sem delta documentado** — se o delta é "usar a versão que eu
  gosto", não é fork, é preferência;
- fork criado para contornar o gate de compatibilidade em vez de passar por ele;
- `[patch.crates-io]` para uma dependência transitiva sem ADR;
- fork que puxa uma segunda família `egui` (viola a invariante absoluta do §18.1).
