# 09 — Shared 3D Foundation, Map Editor e Game Engine

<aside>
🧱

Estratégia aprovada: reutilizar partes neutras e consolidadas da tecnologia do Petunia3D para acelerar uma futura Game Engine e um Map Editor separados, sem transformar os três em um único aplicativo ou criar dependências circulares entre produtos.

</aside>

# Princípio

Petunia3D, Map Editor e Game Engine podem compartilhar uma **3D Foundation** comum. O que deve ser compartilhado é infraestrutura genérica e estável; o que é comportamento de produto permanece em cada aplicativo.

# O que pode ser extraído/compartilhado

- matemática 2D/3D, transforms, rays, bounds e IDs estáveis;
- representação neutra de mesh e operações reutilizáveis realmente genéricas;
- asset IDs, metadata, serialization helpers e versionamento;
- image/texture loading, cache e thumbnail infrastructure quando neutra;
- material data model e shader descriptors portáveis;
- import/export adapters ou contratos comuns;
- GPU abstractions e renderer core quando não conhecerem egui nem regras do editor;
- camera math e viewport projection sem dependência de widgets;
- picking/raycasting genérico;
- gizmo math separado de sua UI;
- async jobs/task infrastructure;
- filesystem/path abstractions;
- logging, diagnostics e profiling;
- command/event primitives genéricos quando realmente compartilháveis;
- collision data, sockets, LOD metadata, animation clips e rigs conforme amadurecerem;
- Asset Bridge e formatos de interchange.

# O que NÃO deve ser compartilhado como core comum

- painéis egui, menus, Outliner, Properties e estado visual do Petunia;
- regras específicas de UX/modelagem do Petunia;
- ECS/gameplay runtime dentro do Petunia;
- physics world/runtime da engine dentro do editor de assets;
- map editing dentro do Petunia;
- AI panel ou workflows específicos de um produto se não forem genéricos;
- um `AppState` gigante compartilhado entre produtos.

# Arquitetura desejada

```
shared-3d-foundation
├── math / geometry
├── asset-schema
├── mesh-data
├── material-schema
├── render-core
├── io / import-export contracts
├── jobs / diagnostics
└── bridge-protocol

petunia3d
├── modeling
├── paint / uv
├── asset authoring
└── Petunia UI

map-editor
├── world/map authoring
├── placement
├── terrain/tile/world tools
└── Map Editor UI

game-engine
├── ECS
├── runtime
├── physics
├── scripting/gameplay
├── scene runtime
└── Engine Editor UI
```

# Regra de dependência

Produtos podem depender da foundation. A foundation não depende de nenhum produto. Petunia não depende da engine; engine não depende da UI do Petunia; Map Editor não depende da aplicação Petunia. Integração ocorre por formatos/APIs/bridge.

# Estratégia de extração

Não criar um mega-framework compartilhado antecipadamente. Primeiro terminar e auditar Petunia. Quando um módulo estiver estável e houver pelo menos **dois consumidores reais**, extrair somente a parte genérica. Aplicar a regra: `duplicação pequena primeiro → boundary comprovada → extração compartilhada`.

# Benefício para a futura engine

Petunia já poderá maturar renderer, asset schemas, materials, import/export, picking, cameras, geometry, serialization, diagnostics e caching. Esses componentes podem reduzir significativamente o trabalho inicial da engine se forem mantidos UI-agnostic e sem regras exclusivas de modelagem.

# Benefício para o Map Editor

O Map Editor poderá reutilizar render-core, camera/navigation, picking, asset browser backend, thumbnails, materials, collision metadata, sockets, import/export e bridge. Entretanto, placement, map chunks, navigation data, terrain/tile workflows e world hierarchy serão responsabilidades próprias.

# Bridge entre produtos

Petunia produz assets; Map Editor os posiciona/organiza; Game Engine executa. O contrato deve transportar mesh/material/texture/animation/collision/socket/LOD metadata de forma versionada. Inicialmente file-based/export profiles; posteriormente file watching/IPC e Live Asset Link.

# Decisão sobre monorepo

Não está decidido. Avaliar mais tarde `workspace Rust/monorepo` versus repositórios separados com crates versionadas. A escolha deve considerar velocidade de desenvolvimento, releases independentes, estabilidade da API e quantidade real de código compartilhado.

# Critério de sucesso

Ser capaz de evoluir ou substituir a UI de qualquer produto sem quebrar os demais; corrigir uma biblioteca compartilhada sem importar conceitos de produto; e permitir que Petunia continue sendo útil independentemente da existência da futura engine.