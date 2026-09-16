# Temas Visuais

Aparência no Petunia3D é 100% token semântico (`ThemeToken`): superfícies, bordas,
texto, accent, seleção, estado, sombra, radius, tipografia e motion. Trocar de tema
**nunca** altera o documento `.petunia`.

## Baseline oficial

- **Dark** é o tema completo oficial da V1.
- **High Contrast** é a variação oficial de acessibilidade.
- Light pode surgir depois sem mudar arquitetura, justamente porque todo estilo usa
  tokens semânticos.
- **Accent baseline:** violeta floral em torno de `#B58CFF`. Accent significa
  `selected`, `active`, `current` e `focus emphasis`; warning/error/success têm famílias
  semânticas próprias. Ajustes de luminância/contraste são *tuning* permitido.

> **Divergência registrada no build atual:** ele embarca temas extras (Light, Capuccino,
> Tokyo Nights) com paletas específicas. Eles são `FUNCTIONAL_BUT_DIFFERENT` em relação
> ao capítulo 36 e estão registrados na
> auditoria de conformidade (`docs/audits/bible-conformance/`). O contrato válido
> é o conjunto de tokens semânticos + a Theme Extension API abaixo — não as paletas
> individuais de cada tema embarcado.

## Theme Extension API (`.petunia-theme`)

Não é preciso escrever plugin nem código executável para criar um tema:

```
my-theme.petunia-theme
├ theme.toml
├ tokens.json
├ preview.png        opcional
├ fonts/             opcional, sujeito a validação
└ icons/             opcional, somente overrides theme-safe suportados
```

O pacote é um **ZIP versionado e puramente declarativo** — ele **não executa código**
durante a resolução de tokens.

```toml
id = "community.midnight_petunia"
name = "Midnight Petunia"
version = "1.0.0"
petunia_theme_api_version = "1"
extends = "petunia.dark"
```

### Herança

Temas herdam de uma base (`petunia.dark`, `petunia.high_contrast` ou outro tema quando a
dependência está disponível); a resolução é em cadeia, com detecção de ciclos e de
dependências ausentes.

### Tokens extensíveis

`surface.*`, `border.*`, `text.*`, `accent.*`, `selection.*`, `state.*`, `shadow.*`,
`radius.*` (dentro de ranges suportados), motion (dentro dos limites de acessibilidade),
família/peso tipográfico quando o recurso de fonte é válido, e mapeamento de ícone
somente em slots genericamente tematizáveis.

### Tokens protegidos

Um tema **não pode** reduzir ou quebrar: minimum hit areas, semântica de foco,
accessible names/roles, comportamento de teclado, topologia do shell, tamanho mínimo do
viewport, contraste mínimo do High Contrast, indicadores de segurança e semântica de
ação destrutiva. Spacing e métricas estruturais só variam dentro de ranges validados.

## Theme Manager

Em **Settings → Appearance**: theme picker com preview, *Create Theme from Current*,
duplicate/rename, edição de tokens semânticos, import/export `.petunia-theme`, reset de
token, avisos de contraste/acessibilidade e live preview/hot reload quando seguro.

## Tema e plugins

Um Community Plugin pode embutir um `.petunia-theme` opcional ou registrar um theme
package declarativo — mas o pacote de tema continua sendo uma superfície **separada** do
código do plugin. Preferência: tema puro → `.petunia-theme`; extensão funcional com tema
→ `.petunia-plugin` contendo o theme package.
