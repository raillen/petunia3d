# Scripts e Ferramentas Utilitárias — Petunia3D (`scripts/`)

Este diretório contém scripts de suporte, automação e ferramentas de desenvolvimento do Petunia3D.

---

## 1. Scripts Disponíveis

### `extract_svg_elements.py`
Extrai, recorta e cataloga elementos atômicos e componentes de SVGs exportados do Figma (`docs/image-references/`).

#### Funcionalidades:
- **Extração de Raster**: Decodifica imagens PNG/JPEG embutidas em base64 em `<image>` e gera tanto o arquivo `.png` quanto um wrapper `.svg`.
- **Extração de Ícones Vetoriais**: Localiza grupos com `clip-path`, calcula o bounding box a partir do `<clipPath>` e salva cada ícone com seu `viewBox` exato.
- **Componentes e Variantes**: Identifica frames de componentes do Figma (retângulos roxos `#8A38F5` ou caixas delimitadoras de componentes) e isola o conjunto completo de estados.
- **Botões e Controles**: Isola botões e controles individuais (incluindo máscaras de borda e ícones sobrepostos).
- **Catálogos Automáticos**: Gera `index.json` e `CATALOG.md` em cada subdiretório, além de um `README.md` mestre.

#### Como Executar:

```bash
# Executar em todos os SVGs de docs/image-references
python3 scripts/extract_svg_elements.py

# Especificar diretórios customizados
python3 scripts/extract_svg_elements.py --input-dir docs/image-references --output-dir docs/image-references/extracted

# Executar apenas em um arquivo específico
python3 scripts/extract_svg_elements.py --file docs/image-references/toolbar.svg
```
