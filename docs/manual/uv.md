# Mapeamento UV

O workspace **UV** permite projetar e desembrulhar a malha tridimensional no espaço bidimensional da textura.

---

## 1. Métodos de Desembrulhamento (Unwrap)

- **Smart UV Project (`U -> Smart Project`)**: Analisa as quebras de ângulo da malha e divide automaticamente as ilhas de faces com proporções uniformes.
- **Projeção Ortogonal da Vista (`U -> Project from View`)**: Projeta as faces exatamente conforme a perspectiva ou vista ortográfica da câmera ativa.
- **Cube Projection (`U -> Cube Projection`)**: Mapeia as faces nas 6 direções canônicas de um cubo, muito útil para caixas, edifícios e blocos.

---

## 2. Editor de Coordenadas UV

Na divisão esquerda da tela durante o workspace UV:
- É possível mover (`G`), rotacionar (`R`) e escalar (`S`) as ilhas UV;
- Suporte a empacotar ilhas (*Pack Islands*) para maximizar o uso da área de textura de 0 a 1.
