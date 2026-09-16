# P3D-167 — Asset States & Variant Composition

<aside>
🎚️

Estado: **futuro pós-V1; composição, não nova engine**. Implementar somente após Morph, Decal Sets, Material Variants e Parametric Properties estarem sólidos.

</aside>

# Objetivo

Salvar combinações semânticas de propriedades existentes como estados de asset.

# Exemplo

```
State: Damaged
morph.damage = 0.65
material = "rusty"
decals = [scratch_02, dirt_01]
handle.visible = false
```

# Casos

Clean/Damaged/Abandoned; shirt styles; vehicle trims; face expressions compostas; prop variants.

# Arquitetura

State armazena overrides de P3D-159; não duplica Material/Morph/Decal systems.

# Animation

Troca de state pode ser step event apenas quando target adapter suportar; não transformar em state machine de gameplay.

# Boundary

Não há logic transitions, triggers, conditions ou runtime behavior. Isso pertence à game engine.

# Testes / DoD

Override precedence, missing property diagnostics, preset migration, export selected state e flatten.