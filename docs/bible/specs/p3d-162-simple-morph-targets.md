# P3D-162 — Simple Morph Targets

<aside>
🙂

Estado: **pós-V1 aprovado**. Morph Targets simples servem personagens, damage states e variações de props sem mudar topologia.

</aside>

# Regra principal

Morph só é válido enquanto a base mantém a mesma correspondência de vértices. Operações topológicas ficam bloqueadas durante `Morph Edit`.

# Modelo

Armazenar deltas por vertex contra uma base revision e weight normalizado.

```
Final = Base + Σ(delta_i × weight_i)
```

# UX

`Add Morph` → nome → `Morph Edit` → mover componentes permitidos → `Finish` → slider. Mirror Morph e duplicate entram cedo.

# Usos

Body Height, Jaw Width, Nose Size, smile/blink; porta amassada; carro danificado; monstro transformado; objeto inflado/achatado.

# Compatibilidade

Pode combinar com Decal Set/Material Variant via Parametric Asset Properties. glTF morph targets continuam caminho preferencial quando suportado.

# Invalidations

Topology edit marca morphs incompatíveis antes de commit; oferecer `Apply Morphs and Continue`, `Cancel topology change` ou fluxo de reconstrução explícito — nunca descartar silenciosamente.

# Testes / DoD

Multiple weights, negative/extended weights se suportados, serialization, mirror, export glTF, animation e topology guard.