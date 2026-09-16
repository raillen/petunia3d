# Pacotes de Ícones

Troque em **Settings → Icons** sem reiniciar. O pacote muda o chrome
(utilitários, menus, viewport); ferramentas de domínio mantêm arte própria
para não perder significado.

## Pacotes oficiais

O contrato define quatro linguagens visuais oficiais mais a arte própria do Petunia:

- **Petunia Custom Icons**: conceitos 3D sem candidato adequado (Extrude, Inset, Round
  Edge, Loop Cut, shading, pivot, proportional editing, entre outros);
- **Tabler**, **Iconoir**, **Phosphor**, **Lucide**: packs genéricos, selecionáveis.

Todos resolvem pelo mesmo `IconId`: trocar de pack **não** muda o significado nem o
contrato da UI. Vende-se apenas os assets usados, com versões fixadas e licenças em
`THIRD_PARTY_NOTICES`.

## Fallbacks

Todo ícone resolve por cadeia: asset do pack ativo → desenho vetorial Petunia →
diagnóstico de ícone ausente. Nenhum controle fica invisível e nenhum placeholder de
quadrado/círculo/emoji é aceito como ícone.

> **Divergência registrada:** o build atual expõe um pack `future-dark` adicional. Ele é
> `FUNCTIONAL_BUT_DIFFERENT` em relação ao conjunto oficial e está anotado na
> auditoria de conformidade (`docs/audits/bible-conformance/`).

Pacotes extras podem ser descobertos via manifests (o cache atualiza pelo
file watcher); detalhes de criação em [Criando um Pacote de Ícones](../tutorials/custom-icon-pack.md).
