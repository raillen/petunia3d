# Workspace: UV

O workspace **UV** divide a tela entre o viewport 3D e o editor 2D de coordenadas de
textura, com seleção sincronizada nos dois sentidos.

## Regiões do shell

- **Split 3D + 2D** com default aproximado **`55/45`** (view 3D à esquerda, editor UV à
  direita), redimensionável em divisor autorizado; double-click no divisor restaura a
  medida default.
- **Editor 2D**: malha desembrulhada no quadrado `0.0..=1.0`, com transladar,
  rotacionar, escalar e empacotar ilhas.
- **Viewport 3D**: selecionar faces na malha mostra imediatamente as ilhas
  correspondentes (e vice-versa).
- **Context** (direita): parâmetros de projeção, packing e densidade.

## Fluxo recomendado

A automação vem primeiro:

1. **Auto UV** orientada a low-poly — UV gerada pelos generators quando conhecida,
   projeção quando há reference/view, e provider de unwrap substituível (`xatlas`) para
   malha genérica.
2. **Project From Reference** (origem preferida quando existe uma ReferenceView
   conhecida) ou **Project From View** (projeção livre/custom).
3. **Packing simples e previsível**, sem overlaps automáticos.
4. **Texel/pixel density** coerente e **checker + aviso de stretch**.
5. **Paint on Model** direto no workspace PAINT.

O editor manual existe para **controle e correção** — não é pré-requisito para pintar
nem para exportar.

## Contrato da V1

- `UV0` único; seams e chart boundaries suportados.
- Provider de unwrap e de projeção ficam atrás de interfaces próprias
  (`UvUnwrapProvider`), substituíveis sem tocar no Geometry Core.
- Projeção a partir de referência reaproveita a mesma imagem/dados da ReferenceView;
  Photo Projection pode permanecer não destrutiva no authoring e ser *baked* no export.
- Toda operação de UV é transacional e entra no Undo.

## Fora do escopo da V1

Providers avançados de unwrap, múltiplas referências com best-view projection,
atlas automático e edição UV com ferramentas de alto nível são candidatos de
`V1.x`/extensão (P3D-149, P3D-063, P3D-064, P3D-065).
