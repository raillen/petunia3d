# Seu Primeiro Modelo (Tutorial de 15 Minutos)

Neste tutorial prático, você criará um **caixote estilizado low-poly** do zero no Petunia3D em apenas 15 minutos, aprendendo as ferramentas fundamentais de modelagem.

---

## Passo 1: O Cubo Inicial
1. Ao iniciar o Petunia3D, você verá um cubo padrão centralizado no viewport.
2. Certifique-se de que ele está selecionado (contorno destacado).
3. Pressione `Tab` para alternar da seleção de **Object** para os componentes da malha.
4. Observe que na barra da viewport ficam disponíveis os domínios de seleção: **Point** (`1`), **Edge** (`2`) e **Face** (`3`).

---

## Passo 2: Inserir as Faces Internas (Inset)
1. Pressione a tecla `3` para ativar o modo de seleção de **Faces**.
2. Clique na face superior do cubo para selecioná-la.
3. Pressione a tecla `I` (ferramenta **Inset / Inserção**).
4. Mova o cursor do mouse ligeiramente para dentro para criar uma borda de madeira moldada (aproximadamente `0.15m`) e clique com o botão esquerdo (`LMB`) para confirmar.
5. Repita o processo nas 4 faces laterais do cubo:
   - Segure `Shift` e clique em cada uma das 4 faces para selecioná-las em conjunto;
   - Pressione `I` e recue para criar a moldura das laterais.

---

## Passo 3: Extrusão para Dentro (Extrude)
1. Com as faces recuadas selecionadas, vamos rebaixar a madeira interior para dar profundidade ao caixote.
2. Pressione a tecla `E` (ferramenta **Extrusão**).
3. Pressione `Z` no teclado para **travar o movimento exclusivamente no eixo vertical**.
   - Observe a **linha guia azul brilhante** atravessando a tela inteira;
   - Observe o badge no HUD flutuante: `[ EIXO Z ]`.
4. Mova o mouse levemente para baixo para afundar a face em `-0.1m` e clique com o botão esquerdo (`LMB`) para confirmar.

---

## Passo 4: Adicionando Detalhes com Loop Cut
1. Pressione `Ctrl+R` para ativar a ferramenta **Loop Cut (Corte em Anel)**.
2. Passe o mouse sobre uma das arestas verticais da moldura: uma linha-guia amarela indicará o plano de corte ao redor do modelo.
3. Clique uma vez com o botão esquerdo (`LMB`) para criar o anel de corte e deslize o mouse para posicioná-lo.
4. Clique novamente para confirmar o corte na posição desejada.

---

## Passo 5: Suavizando Cantos com Round Edge (Bevel)
1. Ative o domínio de seleção **Edge** (`2`).
2. Selecione as arestas dos cantos externos do caixote segurando `Shift`.
3. Pressione `Ctrl+B` (ferramenta **Round Edge / Bevel**).
4. Mova o mouse suavemente para chanfrar os cantos vivos com um acabamento facetado característico do estilo low-poly.
5. Clique com `LMB` para confirmar.

---

## Passo 6: Retorno ao Modo Objeto e Salvamento
1. Pressione `Tab` para voltar à seleção de **Object**.
2. No Outliner (painel superior direito), clique duas vezes sobre o nome do modelo e renomeie-o para `Caixote_Madeira`.
3. Pressione `Ctrl+S` para salvar seu projeto com o nome `meu_primeiro_caixote.petunia`.
4. Parabéns! Você concluiu seu primeiro modelo 3D totalmente funcional e pronto para exportação!
