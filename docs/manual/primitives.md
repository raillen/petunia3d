# Primitivas & Criação Paramétrica

O Petunia3D começa todo asset a partir de um vocabulário pequeno de formas
(**dez primitivas V1**). Inserir uma primitiva abre uma **sessão de criação**:
o cartão Last Operation permite ajustar parâmetros com regeneração ao vivo.
Confirmar grava **uma** transação de undo; `Esc` cancela e remove sem vestígios.

---

## 1. As dez primitivas (três famílias)

**BÁSICAS** — Cube (caixa W/H/D), Plane, Wedge (rampa).

**REDONDAS** — Cylinder (tampas opcionais), Cone (raio do topo vira frustum),
Circle (anel ou disco), Torus.

**ORGÂNICAS** — UV Sphere (loops previsíveis), Icosphere (subdivisão 0–3),
Capsule (corpo + calotas).

Menu **Add ▸**: `BASIC / ROUND / ORGANIC`. Atalhos rápidos de Cube, Sphere,
Cylinder e Plane vivem na shelf do workspace Model.

---

## 2. Defaults low-poly

Cube 1×1×1 · Plane 1×1 · Cylinder/Cone 8 lados · Circle 12 vértices ·
UV Sphere 12×6 · Icosphere subdiv 1 · Capsule 8 radiais · Torus 12×6.
Densidade maior é sempre escolha deliberada (limites protegem o hardware).

---

## 3. Cartão Last Operation

Após inserir: ajuste tamanho, raios, lados, anéis, tampas e preenchimento com
pré-visualização imediata; **Reset** volta aos defaults; a linha de estatísticas
mostra verts/faces/tris. `Enter` confirma, `Esc` cancela, clique na viewport
confirma. Qualquer outra edição converte a primitiva em malha comum.

---

## 4. Após confirmar

A primitiva é uma malha Petunia normal: seleção, transform, Edit Mode,
materiais, pintura, UV, duplicação, save/load e exportação OBJ/GLB funcionam
sem restrições. Sessões não são persistidas (só a malha final salva).
