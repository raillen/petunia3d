# 01 — Visão, UX e Referências

O usuário deve conseguir começar por uma imagem de referência e sentir que está **desenhando o modelo**, em vez de precisar aprender topologia antes de conseguir criar uma forma reconhecível.

## Câmera contextual

<table>
<tr><td>Contexto</td><td>Projeção padrão</td><td>Comportamento</td></tr>
<tr><td>Front / Side / Top</td><td>Ortográfica</td><td>Traçado e alinhamento sem distorção de perspectiva.</td></tr>
<tr><td>Reference / Trace</td><td>Ortográfica</td><td>Câmera bloqueada ao plano da referência enquanto desenha.</td></tr>
<tr><td>Orbit</td><td>Perspectiva</td><td>Inspeção natural do volume.</td></tr>
<tr><td>Draw on Face</td><td>Ortográfica à face</td><td>Face temporariamente tratada como uma folha 2D.</td></tr>
</table>

## Reference Sets

Um projeto pode associar imagens às vistas Front, Side, Back e Top. A troca de vista ativa automaticamente a referência correspondente.

### Recursos

- Opacity da referência.
- Lock de posição e escala.
- Overlay / X-Ray.
- Centerline visual.
- Silhouette Mode.
- Difference/Comparison overlay como evolução futura.
- Na V1, múltiplas referências usam alinhamento manual previsível de posição/escala/rotação. **Landmark Alignment** (Head, Hip, Feet etc.) fica para V1.x como evolução de calibração multi-view.

## Linguagem espacial amigável

O modo iniciante pode apresentar **Width / Height / Depth** em vez de exigir X/Y/Z o tempo todo. O modo técnico continua oferecendo os eixos convencionais.

## Seleção contextual

Evitar obrigar o iniciante a compreender imediatamente uma separação rígida entre Object Mode e Edit Mode. A interface pode expor pills discretas: **Object · Face · Edge · Point**, além de inferência contextual por clique/duplo clique.

## Smart Snap

Por padrão, snapping contextual detecta Vertex, Edge, Midpoint, Center, Grid, centerline da referência e interseções de profiles. Configuração avançada permanece disponível.

## Filosofia de nomenclatura

Priorizar termos diretos como Draw, Cut, Push/Pull, Round Edge e Split. Tooltips podem apresentar o termo técnico equivalente, como Bevel, para facilitar transferência de conhecimento para outros softwares.

# Direção de interface documentada

A fase de UI/UX passa a ter documentação especializada. Esta página continua definindo a intenção de uso e referências; detalhes visuais e de componentes devem ser lidos nos capítulos seguintes:

- [22 — Referência de Interface: Análise do Figma Blender UI Redesign](22-referencia-interface-figma.md) — observações confirmadas da referência Figma e limites da análise.
- [23 — Macroarquitetura da Interface Petunia3D](23-macroarquitetura-interface.md) — aplicação dos princípios ao Petunia: viewport-first, Parts, Context, workspaces e painéis.
- [24 — Design System Visual: Tokens, Hierarquia e Estados](24-design-system-tokens-estados.md) — tokens e regras de consistência visual.
- [25 — Biblioteca de Componentes e Contratos de Interação](25-biblioteca-componentes-interacao.md) — componentes reutilizáveis e critérios de aceite.
- [26 — Figma → Implementação, Assets e Fundamentos de Acessibilidade](26-figma-implementacao-acessibilidade.md) — handoff, assets, controles reais, scaling e fundamentos de acessibilidade.

## Regra de precedência

A referência visual nunca deve reintroduzir conceitos de Blender que contrariem a filosofia de simplicidade do Petunia. Em especial, não usar `Object/Edit Mode`, taxonomia de Properties de Render/World/ViewLayer ou Timeline no workspace Model apenas porque existem no concept analisado.
