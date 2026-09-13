# P3D-002 — Autosave e Recuperação

<aside>
🛟

**Specification Status:** consolidada. **Implementation Status:** deve ser auditado no código. P3D-002 protege o trabalho do usuário sem transformar autosave em substituto silencioso do Save explícito.

</aside>

# Objetivo

Proteger projetos contra crash, queda de energia, falha da aplicação e encerramentos inesperados através de snapshots consistentes, retenção controlada e fluxo explícito de recuperação.

# Referências canônicas relacionadas

- [16 — Documento, Formato de Projeto, Undo, Autosave e Recovery](https://app.notion.com/p/16-Documento-Formato-de-Projeto-Undo-Autosave-e-Recovery-3d79bb7d023f812e80d2ebd9fe12f7a5?pvs=21)
- [09 — Arquitetura, Princípios de Decisão e Governança Técnica](https://app.notion.com/p/09-Arquitetura-Princ-pios-de-Decis-o-e-Governan-a-T-cnica-3d79bb7d023f81f084cee41d2e6f2979?pvs=21)

# Auditoria obrigatória

Antes de implementar, localizar qualquer autosave, backup, session lock, dirty flag, timer, background worker e recovery já existente. Registrar comportamento real e lacunas. Não criar um segundo mecanismo paralelo.

# Princípio central

Autosave não deve sobrescrever cegamente o único arquivo principal. O contrato é:

```
explicit Save → projeto oficial
Autosave → snapshot de recuperação
```

Autosave não limpa o dirty state do projeto.

# Estrutura sugerida

Adaptar ao formato real do projeto; uma organização aceitável é:

```
MyProject/
└── .petunia/
    ├── autosave/
    │   ├── autosave-001.petunia
    │   ├── autosave-002.petunia
    │   └── ...
    └── recovery/
```

Caches e recovery não devem contaminar o conteúdo lógico do projeto nem ser tratados como assets do usuário.

# Configurações

Em `Settings → Autosave & Recovery`, oferecer conforme suporte real:

- Enable Autosave;
- Interval;
- Keep N autosaves;
- Autosave only when project changed.

Presets razoáveis podem incluir 1, 2, 5, 10 e 15 minutos; o default deve ser justificado e documentado.

# Trigger

Autosave só cria novo snapshot quando o projeto está dirty ou quando houver motivo explícito. Não criar arquivos repetidos sem mudança.

# Snapshot consistente

Nunca serializar estado no meio de uma mutação topológica ou transação incompleta. Gerar snapshot consistente do documento. Se serialização em background for usada, definir claramente ownership/snapshotting para evitar race/data corruption.

# Não bloquear o viewport

Quando projetos crescerem, evitar freeze perceptível. Avaliar background serialization, cópia/snapshot imutável ou worker thread, sem introduzir complexidade desnecessária para projetos low-poly pequenos.

# Retenção

Manter apenas os N snapshots configurados. Ao exceder, remover o mais antigo com segurança. Nunca confundir snapshots com o save manual.

# Status UI

Status bar pode apresentar estados discretos como `Autosaving…`, `Autosaved 2m ago` ou warning de falha. Tudo via `TextId` e ThemeToken.

# Falhas de autosave

Falha transitória não deve interromper a sessão nem apagar dados existentes. Registrar erro, mostrar warning não bloqueante e escalar destaque somente se falhas persistirem ou não houver espaço/permissão.

# Detecção de encerramento não limpo

Implementar estratégia robusta de sessão (por exemplo marker/lock controlado) para distinguir clean shutdown de encerramento inesperado. Não assumir simplesmente que existe autosave = houve crash.

# Startup Recovery

Fluxo conceitual:

```
startup
→ previous session unclean?
→ recoverable snapshot exists?
→ present Recovery Dialog
```

Nunca recuperar silenciosamente por cima do projeto principal.

# Recovery Dialog

Deve apresentar informações suficientes para decisão:

```
Recover Project
Project: Forest Adventure
Last normal save: 22:14
Recovery snapshot: 22:21
[Recover] [Open Saved Version] [Discard Recovery]
```

Podem ser exibidos timestamp, tamanho e futuramente thumbnail quando isso for barato/útil.

# Múltiplas recuperações

Se existirem vários projetos recuperáveis, apresentar lista organizada ou Recovery Manager em vez de cadeia de dialogs confusa.

# Recovery Manager

Avaliar `File → Recover Projects` quando houver snapshots recuperáveis persistentes. Deve permitir revisar/descartar sem misturar com Open Recent.

# Recuperar não sobrescreve automaticamente

Ao recuperar, carregar o snapshot como estado dirty/recovered. O usuário decide `Save` ou `Save As`. Preservar o último save normal até uma ação explícita.

# Clean shutdown

Ao fechar normalmente, atualizar/remover markers de sessão somente depois de concluir etapas necessárias. Testar ordem para não registrar falsos crashes.

# Backup do save manual

Quando a estratégia de persistência justificar, manter temporariamente o último arquivo válido durante atomic replace. Não acumular backups ilimitados.

# Storage edge cases

Testar projeto em local read-only, sem espaço, share de rede/removível quando suportado e arquivo removido durante operação. Erros devem ser estruturados e recuperáveis.

# AutosaveService

Autosave é application concern, não regra de widget. Conceitualmente:

```
AutosaveService
← clock/tick neutro
← dirty state
← project snapshot provider
→ AutosaveResult/Event
```

Não acoplar regra de tempo ao frame do egui.

# Concorrência

Se houver worker thread, documentar claramente snapshot ownership, cancelamento no shutdown, ordem de flush e comportamento enquanto Save manual ocorre. Save explícito e Autosave não podem corromper um ao outro.

# Events/results

Eventos/notifications úteis podem incluir `AutosaveStarted`, `AutosaveCompleted`, `AutosaveFailed`, `RecoveryAvailable`, mas evitar EventBus global indiscriminado.

# i18n, ícones e menus

Textos de recovery, warnings e Settings usam `TextId`. Ícones usam `IconId`. Configurações usam ThemeTokens. Não hardcodar intervalos em labels quando forem data-driven.

# Testes obrigatórios

- autosave apenas quando dirty;
- autosave não limpa dirty state;
- retenção de N snapshots;
- snapshot consistente;
- falha durante write/rename;
- autosave e Save explícito concorrentes ou próximos;
- clean shutdown versus crash marker;
- recovery detection;
- Recover/Open Saved/Discard;
- snapshot corrompido;
- projeto read-only/sem espaço;
- testes headless sem egui.

# Testes de resiliência

Quando viável, simular interrupção durante serialize, temp write e replace. Verificar que existe sempre pelo menos um estado válido recuperável.

# Documentação e screenshots

Criar/atualizar `Autosave & Recovery` no website/manual. Capturar Recovery Dialog e Settings de Autosave quando existirem visualmente.

# Critérios de aceitação

- [ ]  Autosave não sobrescreve cegamente o save principal.
- [ ]  Só cria snapshot quando necessário.
- [ ]  Não limpa dirty state.
- [ ]  Retenção funciona e nunca remove o save manual.
- [ ]  Recovery detecta sessão não limpa de forma confiável.
- [ ]  Usuário decide Recover/Open Saved/Discard.
- [ ]  Recovery nunca sobrescreve silenciosamente o projeto oficial.
- [ ]  Falhas são recuperáveis e não causam crash da UI.
- [ ]  AutosaveService é independente do egui/frame de UI.
- [ ]  Testes de falha e headless passam.
- [ ]  Documentação foi atualizada.

# Gauntlet Loop específico

Baseline de persistência → implementar snapshot mínimo → falhas de I/O → retention → recovery marker → Recovery UX → concorrência/performance → arquitetura Core/UI → docs → repetir até não restarem gaps P0/P1 no escopo.