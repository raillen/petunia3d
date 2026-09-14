# 15 — Auditoria Final de Lacunas e Readiness Matrix

<aside>
🔎

Auditoria documental final da Implementation Bible. O caderno é forte em visão, boundaries e backlog, mas a profundidade ainda é desigual: P3D-001–003 estão próximos de specification-ready; muitas P3D-004+ são contratos compactos que precisam ser aprofundados antes da implementação definitiva.

</aside>

# Resultado geral

A arquitetura conceitual está bem definida: Constituição, Core agnóstico à UI, Commands, Tools, tokens, keymaps, docs e Gauntlet Loop são coerentes. A maior lacuna não é falta de ideias, mas **transformar contratos compactos em especificações implementation-ready antes de cada wave**.

# Lacunas P0 transversais encontradas

1. Convenções espaciais/unidades/axis/color pipeline não estavam congeladas de forma canônica.
2. Mesh operations, selection, modal lifecycle e Undo estavam distribuídos em várias P3Ds sem um contrato compartilhado suficiente.
3. Material/Paint/UV careciam de color-space, resource e layer semantics comuns.
4. Jobs/concurrency/cancellation, diagnostics e trust boundaries estavam fragmentados.
5. Não havia Definition of GA, platform matrix e release/distribution gate explícitos.
6. Não existia uma ordem topológica global de implementação; apenas dependências locais.

# Lacunas P1 de especificação

- P3D-004–014: precisam congelar performance baselines, spatial conventions e renderer/picking contracts antes do polish definitivo.
- P3D-015–041: precisam explicitar edge cases topológicos, selection semantics, modal transaction e invalid-state behavior por tool.
- P3D-042–049: precisam de contrato de seleção/rename/delete/missing references e state persistence entre Asset Browser/Outliner/Inspector.
- P3D-050–065/132–134/140: são o maior bloco funcional ainda subespecificado; aprofundar channels, color pipeline, layer model, UV V1 e brush engine antes de implementar.
- P3D-066–067/135–139: permanecem roadmap; precisam de skinning limits, interpolation, clip/root motion, constraints e retarget contracts antes de implementação.
- P3D-068–072: falta uma matriz inicial de formatos/capabilities e política de axis/unit/material conversion.
- P3D-073–083: boa direção visual, mas aprofundar focus/modal/drag-drop/input routing e state persistence por window/panel.
- P3D-084–099: aprofundar schema versioning/migration de packs, hot reload/reload manual, fallback e platform-specific modifier display.
- P3D-100–109: precisam ser auditadas no código antes de qualquer grande refactor; a documentação não deve predeterminar a quantidade de crates.
- P3D-110–112/141–142: definir lifecycle/package/capabilities de plugin e transaction/approval model de agentes antes de execução destrutiva.
- P3D-116–125: ampliar test taxonomy, fixtures/goldens/fuzz/failure injection e política de docs versionadas.

# Pós-GA

P3D-144–155 estão corretamente em nível de roadmap/pesquisa, não implementation-ready. Aprofundar somente quando o GA estiver consolidado e a dependency correspondente tiver consumidor real.

# Gate `SPEC READY`

Uma P3D P0/P1 só entra em implementação definitiva quando possui, conforme aplicável: objetivo; escopo e non-goals; auditoria do código; dependências; estado/modelo de dados; boundary Core/Application/UI/Renderer; commands/tokens/keymaps; undo/persistência; erros/invalid states; performance; security; testes; docs; acceptance criteria e rollback/migration risk.

# Gate `IMPLEMENTATION READY`

Além de SPEC READY: baseline tests passam, dependências bloqueadoras da wave estão concluídas ou explicitamente stubbed, architecture checks relevantes existem e o agente consegue apontar os arquivos/módulos reais a modificar.

# Readiness por macroárea

- Project & Files: alta especificação, implementação a auditar.
- Architecture & Modularity: alta prioridade, depende de evidência de código.
- UI Shell/Customization: direção forte, implementação precisa auditoria e polish.
- Viewport/Modeling: funcionalidade parcial existente; edge cases e contracts precisam aprofundamento.
- Materials/Paint/UV: baixa readiness de implementação definitiva; aprofundar primeiro.
- Animation/Rigging: roadmap, não bloquear GA inicial salvo decisão de escopo.
- Plugins/AI/Post-GA: pesquisa/evolução, não bloquear core.

# Próxima ação canônica

Executar o Master Prompt de Gauntlet Waves. Cada wave começa aprofundando as páginas que ainda não estão `SPEC READY`; somente depois altera código.