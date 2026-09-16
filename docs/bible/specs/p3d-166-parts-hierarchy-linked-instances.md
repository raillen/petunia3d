# P3D-166 — Parts Hierarchy & Linked Instances

<aside>
🧩

Estado: **pós-V1 aprovado para especificação**. Asset assembly precisa parent/child e duplicação vinculada sem evoluir para scene graph de level.

</aside>

# Parts Hierarchy

Parent Part / Child Part com local transform relativo, preserve world transform on reparent, regras explícitas de visibility/lock e export hierarchy quando suportado.

# Usos

porta + maçaneta; arma + carregador + mira; veículo + rodas; personagem + acessórios; máquinas articuláveis.

# Linked Instance

`Duplicate Linked` referencia a mesma source geometry/material até `Make Unique`. Transform permanece por instância.

# Usos

parafusos, rodas, dentes, botões, peças repetidas e modular props.

# Boundary

Hierarchy existe apenas dentro do asset/documento de authoring. Placement de props no mundo, level hierarchy, streaming ou spawn system ficam fora do Petunia.

# Export

Profiles podem preservar nodes/instances quando formato/engine suportar ou flatten/bake via P3D-160.

# Testes / DoD

Reparent, cycles rejected, transforms, Make Unique, source edit propagation, delete source behavior, save/load e export flatten/preserve.