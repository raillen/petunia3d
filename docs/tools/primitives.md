# Adicionar Primitivas (Add Primitive)

Crie formas-base low-poly com um atalho: `A` abre o menu **Add** com três famílias, dez espécies. Cada criação abre uma sessão com cartão **Last Operation** (regeneração ao vivo, transação única de undo; `Esc` cancela).

## Famílias

**BASIC** — `Cube`, `Plane`, `Wedge`.
**ROUND** — `Cylinder`, `Cone`, `Circle`, `Torus`.
**ORGANIC** — `Sphere`, `Icosphere`, `Capsule`.

## Filosofia low-poly

Os padrões já nascem com poucas faces (ex. cilindro com 8 lados, esfera simplificada): a forma legível vem primeiro, o detalhe vem depois com Extrude, Bevel e Subdivide.

## Dicas

- A primitiva nasce no **3D Cursor** e já selecionada.
- Icosphere aceita níveis de subdivisão 0–3 (cada nível multiplica as faces).
- Torus usa raio maior/menor; Circle pode ser só contorno ou disco preenchido.
- Tudo desfaz com um único `Ctrl+Z`.
