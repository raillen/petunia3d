# 04 — Combine, Fuse, Weld e Personagens

# Um único botão "juntar" não é suficiente

Em 3D existem intenções diferentes quando o usuário diz que quer unir objetos. O Petunia3D deve traduzi-las para opções simples e previsíveis.

| Ação | O que faz | Uso recomendado |
| --- | --- | --- |
| Keep Parts | Mantém objetos independentes dentro da composição sem alterar sua geometria. | Props compostos, roupas, acessórios e peças que devem continuar independentes. |
| Join | Um MeshObject contendo várias ilhas desconectadas. | Exportar várias partes como um único asset sem alterar sua forma. |
| Fuse | Fusão geométrica real. Semanticamente corresponde a Union quando volumes precisam virar uma única superfície externa. | Hard-surface, objetos rígidos e formas interpenetrantes. O termo Boolean permanece detalhe técnico, não requisito conceitual para o usuário. |
| Connect | Cria continuidade topológica controlada; Bridge/Weld/Stitch são mecanismos internos/avançados. | Pescoço, ombro, quadril e regiões que precisam deformar. |

# Exemplo: corpo criado em peças

Cabeça, tronco, braços e pernas podem continuar separados durante quase toda a criação. Isso torna mover, escalar, espelhar e substituir partes muito mais fácil.

## Para personagem rígido ou estilo segmentado

**Join** pode ser suficiente. Não há necessidade de transformar tudo em uma superfície contínua apenas por princípio.

## Para personagem animado com deformação nos joints

Nas regiões que dobram, como ombro, cotovelo, quadril e joelho, o ideal é criar continuidade com **Bridge/Weld** ou conectar loops explicitamente. Boolean Union sozinho pode produzir uma topologia ruim para deformação.

## Para props e hard-surface

**Fuse** é muito útil quando volumes interpenetrantes precisam realmente virar uma superfície única. Internamente, Fuse pode delegar a um BooleanProvider robusto; para o usuário, porém, a intenção continua sendo apenas "fundir formas".

Boolean não deve ser usado quando uma operação topológica local simples resolve o caso. Extrude sobre uma região já delimitada e Push/Pull que não atravessa outra parte do objeto devem editar a topology diretamente.

# UX V1: Combine

Ao selecionar múltiplas formas, `Combine` oferece exatamente quatro intenções em linguagem simples:

```
Keep Parts | Join | Fuse | Connect
```

- **Keep Parts** — preservar partes independentes na composição.
- **Join** — mesmo MeshObject, mantendo ilhas desconectadas.
- **Fuse** — fundir volumes sobrepostos em uma única superfície; Union é detalhe técnico interno.
- **Connect** — criar continuidade topológica entre boundaries compatíveis.

Um preview mostra o resultado antes da confirmação.

`Group / Assembly` não é nome de ação do Combine V1. Pode permanecer como conceito estrutural interno e só deve virar feature pública própria se uma necessidade real de agrupamento hierárquico justificar isso.

# Política normativa de Fuse e Cut

- **Fuse = Union** quando uma fusão volumétrica verdadeira for necessária.
- **Cut = Difference quando necessário**; recessos simples podem ser resolvidos por edição topológica local sem CSG.
- Through-cut ou cutters que atravessam/intersectam geometria arbitrária podem delegar a BooleanProvider.
- Intersect não faz parte do fluxo principal da V1; pode entrar posteriormente em Advanced/extensão.
- BooleanProvider é substituível e fica atrás de interface própria; o Geometry Core não depende de uma implementação específica.
- Resultado boolean recebe apenas cleanup seguro: degenerates, weld de pontos coincidentes e dissolução coplanar quando comprovadamente segura.
- **Não fazer remesh automático** nem retopology automática como consequência de Fuse/Cut.
- Preview e Undo são obrigatórios.

# Smart Fuse como evolução

Uma ferramenta futura pode fazer Fuse + cleanup local + tentativa opcional de simplificação. Ela nunca deve esconder mudanças de topologia: mostrar preview, contagem de faces/triangles, warnings de manifold e permitir voltar ao original.

# Política normativa de Connect / Weld / Bridge

**Connect é a ação principal exposta ao usuário.** Bridge e Weld são mecanismos topológicos usados por Connect e permanecem disponíveis em edição avançada, mas não devem ser conceitos obrigatórios para modelagem básica.

## Intenção de UX

O usuário escolhe duas regiões abertas — preferencialmente duas faces-cap ou dois boundaries — e aciona **Connect**. O Petunia identifica os contornos, mostra preview e cria a faixa de geometria entre eles. O usuário não precisa preparar manualmente a mesma quantidade de points nos dois lados.

Fluxo principal:

```
select part A + part B
        ↓
     Connect
        ↓
detect boundaries
        ↓
auto align correspondence
        ↓
preview bridge
        ↓
commit transaction
```

## Quando as duas extremidades ainda possuem faces-cap

Se o usuário selecionar duas faces que representam as tampas das regiões a conectar, Connect pode removê-las dentro da mesma transaction e usar seus loops como boundaries. Isso torna casos como braço ↔ torso, pescoço ↔ cabeça e duas seções de tubo muito mais diretos.

## Loops com a mesma quantidade de points

Usar correspondência 1:1 e gerar quads sempre que a orientação permitir. O sistema escolhe automaticamente o offset inicial que minimiza twist/distância. Se houver mais de uma solução semelhante, preview deve tornar a escolha visível.

## Loops com quantidades diferentes de points

**Não exigir subdivisão manual e não igualar contagens usando LCM ou remesh.** O Petunia aceita topology mista, portanto Connect pode gerar uma faixa composta por quads e triangles.

Estratégia recomendada:

- ordenar os dois boundary loops;
- determinar orientação e ponto inicial de menor custo;
- percorrer ambos monotonicamente;
- avançar um lado gera triangle; avançar ambos gera quad quando válido;
- escolher a sequência que minimiza custo geométrico simples, favorecendo arestas curtas, baixa distorção e ausência de cruzamentos;
- validar orientação, degenerates e self-intersection local antes do commit.

A implementação pode usar um algoritmo de correspondência/shortest-path dinâmico simples em uma grade `n × m`. Como o foco é low-poly, as contagens de points são pequenas e previsíveis; isso prioriza clareza e robustez sobre otimizações sofisticadas.

## Twist

O Petunia tenta resolver twist automaticamente. Não expor um slider permanente de `Twist` na UI básica. Se a solução automática estiver errada, oferecer uma correção contextual simples, como **Rotate Match** / arrastar o marcador de correspondência inicial no preview. Opções técnicas detalhadas podem existir em Advanced.

## Intermediate cuts

Connect V1 cria apenas a superfície necessária entre as duas extremidades. Não gerar cortes intermediários automaticamente. Se o usuário quiser mais resolução, ele pode aumentar a geometria antes/depois ou usar ferramentas futuras. Isso evita que Connect vire o Bridge Edge Loops completo de um DCC generalista.

## Weld

`Weld` significa unir points que devem ocupar uma única posição/topological identity.

- Auto-weld só ocorre para points explicitamente envolvidos na operação ou dentro de tolerância muito pequena e contextual.
- Nunca fazer Merge by Distance global silencioso.
- Edição avançada pode oferecer `Weld Points` para seleção explícita.
- Default de posição: midpoint/centro da seleção; quando houver um point ativo explícito, Advanced pode permitir `To Active`.
- UV seams, sharp edges e atributos de vertex não devem ser descartados silenciosamente; conflitos geram regra determinística ou warning.

## Stitch

`Stitch` não precisa ser uma ferramenta independente na V1. É uma variante de Connect/Weld para boundaries já sobrepostos ou muito próximos. A UI pode simplesmente reconhecer o caso e apresentar **Connect**; internamente a operação reduz distância, cria correspondência e faz weld quando seguro.

## Objetos separados

Connect pode operar entre dois MeshObjects. Durante o commit, eles tornam-se um único MeshObject apenas quando a conexão topológica realmente é criada. Antes do commit, os objetos permanecem intactos para preview/rollback.

## Segurança e validação

Connect é sempre transacional e deve validar:

- boundaries válidos e não ambíguos;
- orientação consistente;
- nenhuma face degenerada nova;
- nenhuma conexão cruzada evidente;
- manifold local quando o input permitir;
- preservação previsível de material/UV/sharp flags;
- triangulation cache invalidada e regenerada após commit.

Se não houver solução segura, **falhar com explicação curta e preservar o modelo**, em vez de tentar uma reconstrução agressiva.

## O que deliberadamente não entra

- Blend Surface/Smoothness/Profile Shape do Bridge avançado.
- geração automática de muitos loops intermediários;
- retopology/remesh;
- correspondência semântica baseada em IA;
- deformação/smoothing automática da junção;
- obrigação de topology quad-only.

Essas capacidades podem ser extensões futuras se surgir necessidade real.

## Referências e rationale

O Blender demonstra que Bridge Edge Loops pode conectar loops com contagens diferentes, mas também expõe Twist, Cuts, Interpolation, Smoothness e Profile Shape — flexibilidade excessiva para a proposta do Petunia. O Wings 3D mantém Bridge mais simples, porém exige o mesmo número de edges/vertices nas duas faces. O Crocotile reforça operações low-poly pequenas e explícitas de merge/weld. Petunia escolhe um meio-termo: UX simples como Wings/Crocotile, aceitando contagens diferentes graças a triangles/quads mistos.

- [Blender — Bridge Edge Loops](https://docs.blender.org/manual/en/5.3/modeling/meshes/editing/edge/bridge_edge_loops.html)
- [Wings 3D — Face Bridge](https://www.wings3d.com/documentation/user-manual-table-of-contents/command-menus/menu-face/)
- [Crocotile 3D — Documentation](https://www.crocotile3d.com/howto.html)

# Regra para o Petunia3D

Não exigir fusão real para exportar um personagem ou asset. Um modelo de game pode conter múltiplos submeshes/ilhas. **Fundir só quando isso melhora shading, deformação, edição ou requisitos do pipeline.** Quando continuidade real for necessária, preferir **Connect**, deixando Bridge/Weld como detalhes técnicos e ferramentas avançadas.