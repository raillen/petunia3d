# Criando um Prop Low-Poly

Fluxo shape-first completo: silhueta → malha → detalhe → cor → export.

1. **Silhueta**: ative **Draw Profile** (`Shift+P`) e desenhe o contorno sobre a vista ortográfica (`O` alterna projeção). Feche clicando perto do 1º ponto.
2. **Volume**: ajuste **Depth** e clique **Gen Extrude** (ou **Gen Revolve** para peças redondas como garrafas).
3. **Detalhe**: `Tab` → Face, selecione áreas e use `E` (extrude), `Ctrl+B` (bevel) e `W` (subdivide) com parcimônia — low-poly lê melhor com menos faces.
4. **Cor**: workspace **PAINT**, pincel `B`, conta-gotas `Alt+clique`.
5. **UV** (se for texturizar): workspace **UV**, projeção planar, ajuste as ilhas.
6. **Export**: OBJ para intercâmbio, GLB com PBR para game-ready.

Dica: espelhe metade com `Ctrl+M` + solda em vez de modelar tudo duas vezes.
