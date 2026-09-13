# 14 — Release, Compatibilidade, Distribuição e Critérios de GA

<aside>
🚢

O projeto tinha critérios de feature, mas faltava um contrato claro para afirmar que o aplicativo como produto está pronto. Esta página define o gate de release sem inventar suporte de plataforma não confirmado.

</aside>

# Matriz de plataformas

Antes de GA, declarar oficialmente quais sistemas/arquiteturas são suportados, testados, experimentais ou fora de escopo. File paths, key modifiers, DPI, native dialogs e packaging precisam de testes por plataforma suportada.

# Versionamento

Definir versão do aplicativo, `project format version`, schema versions de customization/plugin/bridge e política de migração. App version e file format não são a mesma coisa.

# Compatibilidade de projeto

Documentar versões que abrem diretamente, versões migráveis, backup antes de migration destrutiva e comportamento ao abrir arquivo de versão futura.

# Distribuição

Definir artefatos por plataforma, portable/install quando aplicável, localização de user-data/cache/logs e política de update. Assinatura/notarização entra somente onde a plataforma/distribuição exigir.

# Release pipeline

Release build reproduzível o suficiente, test suite, architecture checks, docs build, changelog/release notes, package smoke test e hashes/artifacts. Nunca publicar build que depende de assets de desenvolvimento não empacotados.

# Privacy

Sem login obrigatório. Documentar network access real, update checks, external docs links, MCP/plugins e qualquer futura telemetria. Conversão para comportamento online nunca pode ocorrer silenciosamente.

# GA Hardening

Antes de novas features grandes, provar o fluxo: `criar/abrir → modelar → pintar/UV quando no GA → organizar → salvar → fechar → reabrir → exportar` sem P0/P1 conhecidos dentro do escopo GA.

# Performance Gate

Registrar hardware/perfis de baseline reais antes de congelar números. Definir budgets mensuráveis para startup, idle memory, simple-scene frame time, operações representativas e grandes bibliotecas; otimizações são medidas contra baseline.

# Crash/recovery Gate

Save atômico, autosave/recovery, arquivo corrompido e falhas de disco/permission possuem comportamento testado.

# Accessibility/UX Gate

Layouts suportados não apresentam overlap crítico; keyboard focus e menus fundamentais funcionam; idiomas suportados não quebram UI; ferramentas essenciais são descobríveis.

# Definition of GA

GA só pode ser declarado por checklist de produto, não por contagem de P3Ds concluídas. Itens pós-GA permanecem explicitamente fora do gate.