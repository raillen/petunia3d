# P3D-159 — Parametric Asset Properties

<aside>
🎛️

Estado: **fundação pós-V1 aprovada**. Unificar parâmetros provenientes de Morphs, Decal Sets, Material Variants, Modifiers, Generators e visibility sem criar schemas paralelos.

</aside>

# Objetivo

Permitir que um asset exponha propriedades semânticas simples para Inspector, Animation, Presets, export adapters, MCP e futuros game integrations.

# Exemplos

```
character.body_height = 0.55
character.jaw_width = 0.32
character.eyes = "happy"
character.mouth = "smile"
crate.damage = 0.72
crate.label = "hazard"
fence.spacing = 1.5
```

# Tipos V1

Bool, Integer, Float com range/step, Enum, Color, AssetReference e optional string curta quando justificável.

# Binding

Uma propriedade aponta para um target aprovado: Morph weight, Decal Set variant, Material Variant, Modifier parameter, Generator parameter ou visibility. Binding é declarativo e validado.

# Inspector

Agrupar por categorias (`Body`, `Face`, `Surface`, `Damage`, `Generator`). Advanced mostra origem/binding; usuário comum vê apenas controles semânticos.

# Presets

Salvar subconjuntos como Character Preset, Prop Variant ou Generator Preset.

# Animation

Float/Color podem interpolar; Enum/Bool usam step. Nem toda propriedade precisa ser animável.

# Export

Engine adapters podem mapear propriedades reconhecidas; GLB genérico preserva metadata quando solicitado, mas export universal continua podendo bake/flatten.

# Testes / DoD

Serialization, enum migration, range validation, broken binding diagnostics, preset round-trip, animation step/continuous e MCP query/set com transaction.