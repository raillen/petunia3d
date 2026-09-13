# P3D-001 — Sistema de Projetos

<aside>
📁

**Specification Status:** consolidada para implementação incremental. **Implementation Status:** deve ser auditado no código antes de qualquer alteração. Esta página define o contrato funcional e arquitetural de P3D-001.

</aside>

# Objetivo

Criar um sistema de projetos confiável, versionável e independente da UI, capaz de criar, abrir, salvar, salvar como, fechar e reabrir projetos recentes sem acoplar regras de persistência ao egui.

# Referências canônicas relacionadas

- [16 — Documento, Formato de Projeto, Undo, Autosave e Recovery](https://app.notion.com/p/16-Documento-Formato-de-Projeto-Undo-Autosave-e-Recovery-3d79bb7d023f812e80d2ebd9fe12f7a5?pvs=21)
- [09 — Arquitetura, Princípios de Decisão e Governança Técnica](https://app.notion.com/p/09-Arquitetura-Princ-pios-de-Decis-o-e-Governan-a-T-cnica-3d79bb7d023f81f084cee41d2e6f2979?pvs=21)
- [28 — Arquitetura Rust, Cargo Workspace e Fronteiras entre Crates](https://app.notion.com/p/28-Arquitetura-Rust-Cargo-Workspace-e-Fronteiras-entre-Crates-3d89bb7d023f812093add994fa489fda?pvs=21)
- [23 — Macroarquitetura da Interface Petunia3D](https://app.notion.com/p/23-Macroarquitetura-da-Interface-Petunia3D-3d79bb7d023f8148ad6cff403996f066?pvs=21)

# Auditoria obrigatória antes da implementação

Localizar e documentar a implementação real de New Project, Open, Save, Save As, Close, Recent Projects, dirty state, paths, serialização, loaders, dialogs e integração com assets. Não duplicar sistemas já corretos. Não assumir que a documentação representa o estado atual.

# Modelo conceitual do projeto

Um projeto representa o contexto completo de trabalho e deve poder referenciar, conforme a implementação vigente:

- metadata e versão do formato;
- estado persistente de cena/editor que realmente pertença ao documento;
- modelos/assets;
- materiais e texturas;
- imagens de referência;
- metadata da biblioteca de modelos;
- configurações específicas do projeto.

Não persistir como documento estados puramente visuais como hover, popup aberto, scroll temporário ou largura de painel, salvo quando deliberadamente classificados como sessão/preferência.

# Estrutura de armazenamento

Avaliar a implementação existente e, se necessário, aproximar-se de uma estrutura organizada como:

```
MyProject/
├── project.petunia
├── assets/
│   ├── models/
│   ├── textures/
│   ├── materials/
│   └── references/
├── thumbnails/
├── metadata/
└── .petunia/
    ├── autosave/
    ├── recovery/
    └── cache/
```

A estrutura final deve ser justificada tecnicamente e permanecer portável.

# Formato central

`project.petunia` deve possuir versão explícita de formato e metadata suficiente para migração futura. O formato pode ser TOML, JSON, RON ou representação equivalente já adotada, desde que seja robusto, versionável e testável.

## Metadata mínima conceitual

- `project_id` estável;
- `name`;
- `format_version`;
- `created_at`;
- `modified_at`;
- versão do Petunia que salvou o documento quando útil;
- referências relativas a assets e metadata necessária.

# IDs estáveis

Não usar filename, display name ou índice de `Vec` como identidade de longo prazo quando a entidade precisa sobreviver a rename, undo, relink ou API externa. Auditar necessidade de `ProjectId`, `AssetId`, `ObjectId`, `MaterialId`, `TextureId` e `ReferenceImageId`.

# Paths

Preferir paths relativos ao projeto. Arquivos externos devem seguir política explícita: referenciar externamente, copiar para o projeto ou perguntar ao usuário. Não decidir silenciosamente.

# Dirty State

O projeto precisa representar mudanças não salvas semanticamente.

## Normalmente deixam dirty

- edição de mesh;
- transform de objeto;
- material persistente;
- referência adicionada/removida;
- asset metadata do projeto;
- configurações específicas do projeto.

## Não deixam dirty

- orbit/zoom do viewport quando não persistentes;
- hover;
- popup aberto;
- busca temporária na biblioteca;
- geometria/tamanho de janela do usuário;
- estados de UI classificados como preferência/sessão.

# Save

Fluxo desejado:

```
request Save
→ ProjectService
→ validate snapshot
→ serialize
→ safe temporary write
→ flush/validate
→ atomic replace quando suportado
→ mark clean
→ ProjectSaved event/result
```

Falha de save não pode destruir o último arquivo válido e não deve limpar o dirty state.

# Save As

Deve criar/transferir corretamente a estrutura necessária, atualizar o destino ativo e preservar IDs estáveis. Não tratar como simples rename de um único arquivo se o projeto possui diretório/assets associados.

# New Project

`File → New Project` abre um fluxo próprio com pelo menos nome e local. Pode incluir `Create folder automatically` e template `Empty Project`. Validar nomes inválidos conforme a plataforma. Se houver conceito de Untitled Project, documentar claramente seu ciclo.

# Open Project

`File → Open Project…` deve abrir o arquivo/pasta canônico do projeto, validar versão e falhar de modo recuperável para arquivo inválido, corrompido ou de versão futura não suportada.

# Recent Projects

Histórico é preferência da aplicação, não dado do documento. Manter path, display name e last-opened quando útil. Projeto ausente não pode quebrar o menu; marcar indisponível ou permitir remoção.

# Close Project

Projeto dirty deve exigir decisão semântica: Save, Don't Save ou Cancel. O core/application expõe a necessidade da decisão; o frontend escolhe a apresentação.

# Barra de menus principal

A barra principal deve usar o sistema canônico de menus, `TextId`, `IconId`, `CommandId` e keymap atual.

## File

```
File
├── New Project
├── Open Project...
├── Open Recent ›
├── ─────────────
├── Save
├── Save As...
├── ─────────────
├── Import ›
├── Export ›
├── ─────────────
├── Project Model Library
├── Project Settings... (quando existir conteúdo específico do projeto)
├── ─────────────
├── Close Project
└── Exit
```

## Edit

```
Edit
├── Undo
├── Redo
├── ─────────────
├── Cut
├── Copy
├── Paste
├── Duplicate
├── Delete
├── ─────────────
├── Select All
├── Select None
├── Invert Selection
├── ─────────────
└── Settings...
```

## View

O menu global View não deve duplicar integralmente o `View` contextual do viewport.

```
View
├── Panels ›
│   ├── Asset Browser
│   ├── Outliner
│   ├── Properties
│   └── Status Bar
├── ─────────────
├── Command Palette
├── ─────────────
├── Reset Layout
└── Fullscreen
```

## Window

```
Window
├── Project Model Library
├── Asset Browser
├── Outliner
├── Properties
├── ─────────────
├── Maximize Viewport
├── Reset Workspace Layout
└── Close Floating Windows
```

Somente mostrar ações realmente implementadas.

## Help

```
Help
├── Documentation
├── Getting Started
├── Keyboard Shortcuts
├── Troubleshooting
├── ─────────────
├── Report an Issue
├── ─────────────
└── About Petunia3D
```

`About` deve exibir logo, nome, versão/build, website/GitHub e licença, sem excesso.

# Settings — separação obrigatória

Settings é configuração da aplicação. Project Settings é configuração do documento. Não misturar theme/language/keymap com dados do projeto.

## Settings Window

Deve ser uma janela/painel próprio, redimensionável, pesquisável quando viável e lembrar a última categoria como preferência do usuário.

## Categorias recomendadas

- General;
- Interface;
- Appearance;
- Icons;
- Language;
- Input → Keymap / Mouse / Navigation;
- Viewport;
- Files & Projects;
- Autosave & Recovery;
- Performance;
- Advanced.

Criar apenas categorias com conteúdo real.

### General

Startup behavior, limite de recent projects e confirmações gerais realmente implementadas.

### Interface

UI scale, tooltips, ajuda contextual e animações quando suportadas.

### Appearance

Theme e Accent via ThemeToken/theme packs.

### Icons

Pack selecionável: Tabler, Iconoir, Phosphor, Lucide e packs customizados, com preview.

### Language

Translation pack, native name, locale e coverage quando disponível.

### Input

Sistema canônico de keymaps; nenhuma tecla hardcoded na feature.

### Viewport

Sensibilidade e preferências globais de navegação/grid quando pertencerem às preferências do usuário.

### Files & Projects

Local padrão, recent-project limit e políticas globais de arquivo.

### Autosave & Recovery

Configurações definidas em P3D-002.

### Performance

Caches/presets globais realmente suportados.

# Arquitetura e desacoplamento

Possível separação conceitual, adaptada ao código real:

```
Core
→ dados do projeto, IDs, estruturas persistentes
Application
→ ProjectService, comandos, validação, orquestração
Infrastructure
→ filesystem/serializer quando apropriado
UI
→ menus, dialogs, file picker, settings window
```

`ProjectService` não deve abrir file picker nem depender de egui.

# Error model

Preferir erros estruturados (`PermissionDenied`, `InvalidFormat`, `UnsupportedVersion`, `MissingAsset` etc.) e deixar a UI traduzi-los via `TextId`. Não misturar mensagem localizada com regra de domínio.

# Versionamento e migração

`format_version` é obrigatório. Versões antigas suportadas devem migrar de forma explícita/testada. Versão futura incompatível deve falhar com diagnóstico claro, não tentar carregar cegamente.

# Commands, tokens e documentação

Toda ação exposta deve avaliar `CommandId`, `TextId`, `IconId`, keymap e DocsTopic. Menus nunca hardcodam shortcuts; usam o keymap atual.

# Testes obrigatórios

- new project;
- save e save as;
- open;
- close dirty;
- recent projects;
- arquivo inválido/corrompido;
- versão futura;
- path ausente/read-only;
- falha durante atomic save;
- separação de dirty state versus UI state;
- testes headless do ProjectService sem egui.

# Documentação e screenshots

Atualizar website/manual para Create Project, Open, Save, Recent Projects, Project Settings e Settings. Capturar New Project Dialog, File Menu e Settings quando a UI mudar.

# Critérios de aceitação

- [ ]  Create/Open/Save/Save As funcionam e têm cobertura de teste.
- [ ]  Dirty state é semântico e confiável.
- [ ]  Close protege alterações não salvas.
- [ ]  Recent Projects é persistente e robusto a paths ausentes.
- [ ]  Projeto possui versionamento de formato.
- [ ]  Falhas de save não destroem o último arquivo válido.
- [ ]  Core/application não dependem de egui para persistir projeto.
- [ ]  Menus usam Commands/TextIds/IconIds/keymap atual.
- [ ]  Settings e Project Settings estão conceitualmente separados.
- [ ]  Documentação e screenshots afetados foram atualizados.

# Gauntlet Loop específico

Auditar → implementar menor slice → testes de persistência → testes de falha → auditoria de boundary Core/UI → revisão de UX/menus → docs → reavaliar. Não inflar qualidade; corrigir gaps até cumprir os critérios acima.